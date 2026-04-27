#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::redundant_closure_for_method_calls)]

pub mod cargo_info;
pub mod cross_refs;
pub mod file_parser;
pub mod module_tree;
pub mod render;
pub mod schema;
pub mod workspace;

pub use schema::Config;

use anyhow::Context;
use rayon::prelude::*;
use schema::{
    CrateInfo, CrateType, ErrorEntry, ModuleInfo, WorkspaceInfo, WorkspaceMap,
};
use std::path::Path;

/// Run the full workspace mapping pipeline.
///
/// 1. Discover workspace root and member crates.
/// 2. Process each crate in parallel (Cargo.toml parsing + module tree).
/// 3. Compute cross-crate references.
/// 4. Render JSON to stdout or the configured output file.
pub fn run(config: Config) -> anyhow::Result<()> {
    let workspace_root = workspace::find_workspace_root(&config.workspace_path)?;
    let member_dirs = workspace::enumerate_members(&workspace_root)?;

    let errors: Vec<ErrorEntry> = Vec::new();

    let mut crate_infos: Vec<CrateInfo> = member_dirs
        .par_iter()
        .filter_map(|dir| {
            let cargo_toml = dir.join("Cargo.toml");

            let (pkg, deps) = match cargo_info::parse_cargo_toml(&cargo_toml) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!(
                        "warning: failed to parse {}: {}",
                        cargo_toml.display(),
                        e
                    );
                    return None;
                }
            };

            let roots = workspace::resolve_crate_roots(dir);
            if roots.is_empty() {
                eprintln!(
                    "warning: no crate entry points found in {}",
                    dir.display()
                );
                return None;
            }

            let crate_type = if roots.iter().any(|(_, t)| *t == CrateType::Lib)
                && roots.iter().any(|(_, t)| *t == CrateType::Bin)
            {
                CrateType::LibAndBin
            } else {
                roots.first().map_or(CrateType::Lib, |(_, t)| *t)
            };

            let pkg_name = pkg.name.clone();
            let mut modules: Vec<ModuleInfo> = roots
                .iter()
                .flat_map(|(root, _ty)| {
                    module_tree::build_module_tree(root, &pkg_name).unwrap_or_default()
                })
                .collect();

            // Relativize all paths to the workspace root.
            for m in &mut modules {
                m.file = relativize_path(&m.file, &workspace_root);
                for item in &mut m.public_items {
                    item.file = relativize_path(&item.file, &workspace_root);
                }
            }

            let crate_root = roots
                .first()
                .map(|(r, _)| relativize_path(&r.to_string_lossy(), &workspace_root))
                .unwrap_or_default();

            let rebuilt_pkg = schema::PackageInfo::builder()
                .name(pkg.name)
                .version(pkg.version)
                .edition(pkg.edition)
                .crate_type(crate_type)
                .build();

            Some(
                CrateInfo::builder()
                    .name(pkg_name)
                    .root(crate_root)
                    .package(rebuilt_pkg)
                    .modules(modules)
                    .deps(deps)
                    .build(),
            )
        })
        .collect();

    // Deterministic sort by crate name.
    crate_infos.sort_by(|a, b| a.name.cmp(&b.name));

    let cross_refs = cross_refs::compute(&mut crate_infos);

    let workspace_name = workspace_root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    let workspace_info = WorkspaceInfo::builder()
        .root(".".to_string())
        .workspace_name(workspace_name)
        .build();

    let map = WorkspaceMap::builder()
        .workspace(workspace_info)
        .crates(crate_infos)
        .cross_references(cross_refs)
        .errors(errors)
        .workspace_root(workspace_root.clone())
        .build();

    if let Some(ref output_path) = config.output_path {
        let file = std::fs::File::create(output_path)
            .with_context(|| format!("failed to create output file: {}", output_path.display()))?;
        let writer = std::io::BufWriter::new(file);
        render::render_to_writer(&map, writer)?;
    } else {
        let stdout = std::io::stdout();
        render::render_to_writer(&map, stdout.lock())?;
    }

    Ok(())
}

/// Strip the workspace root prefix from a path string, returning a
/// workspace-relative path. If the prefix doesn't match, returns the
/// original string unchanged.
fn relativize_path(path_str: &str, root: &Path) -> String {
    let p = Path::new(path_str);
    match p.strip_prefix(root) {
        Ok(rel) => rel.to_string_lossy().to_string(),
        Err(_) => path_str.to_string(),
    }
}
