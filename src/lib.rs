#![warn(clippy::pedantic)]

pub mod cargo_info;
pub mod cross_refs;
pub mod file_parser;
pub mod indexes;
pub mod lookup;
pub mod module_tree;
pub mod render;
pub mod schema;
pub mod validate;
pub mod workspace;

pub use schema::Config;

use anyhow::Context;
use rayon::prelude::*;
use schema::{
    CrateInfo, CrateType, DiagnosticKind, Error, ErrorEntry, ErrorSeverity, ModuleInfo,
    WorkspaceInfo, WorkspaceMap,
};
use std::path::Path;

/// Build a `WorkspaceMap` from the given config, without rendering.
///
/// This function contains all pipeline logic up to and including
/// `WorkspaceMap` construction — workspace discovery, per-crate processing,
/// cross-refs computation, index derivation, optional validation, and map building.
///
/// # Errors
///
/// Returns an error if the workspace root cannot be found, the workspace
/// Cargo.toml is missing a `[workspace]` section, member crates cannot be
/// parsed, or the JSON output cannot be written.
#[allow(clippy::too_many_lines)]
pub fn build_map(config: &Config) -> anyhow::Result<WorkspaceMap> {
    let (workspace_root, member_dirs) = match workspace::find_workspace_root(&config.workspace_path) {
        Ok(root) => {
            match workspace::enumerate_members(&root) {
                Ok(members) => (root, members),
                Err(Error::MissingWorkspaceSection) => {
                    // [workspace] was a false positive (e.g., in a comment).
                    // Fall back to single-crate discovery.
                    let crate_dir = workspace::find_crate_root(&config.workspace_path)?;
                    (crate_dir.clone(), vec![crate_dir])
                }
                Err(other) => return Err(other.into()),
            }
        }
        Err(Error::WorkspaceRootNotFound(_)) => {
            let crate_dir = workspace::find_crate_root(&config.workspace_path)?;
            (crate_dir.clone(), vec![crate_dir])
        }
        Err(other) => return Err(other.into()),
    };

    let mut crate_errors: Vec<ErrorEntry> = Vec::new();

    let results: Vec<(Option<CrateInfo>, Vec<ErrorEntry>)> = member_dirs
        .par_iter()
        .map(|dir| {
            let cargo_toml = dir.join("Cargo.toml");
            let mut crate_errors = Vec::new();

            let (pkg, deps) = match cargo_info::parse_cargo_toml(&cargo_toml) {
                Ok(v) => v,
                Err(e) => {
                    crate_errors.push(ErrorEntry::builder()
                        .file(cargo_toml.to_string_lossy().to_string())
                        .message(format!("failed to parse Cargo.toml: {e}"))
                        .severity(ErrorSeverity::Error)
                        .kind(DiagnosticKind::TomlParseError)
                        .cause(e.to_string())
                        .build());
                    return (None, crate_errors);
                }
            };

            let roots = workspace::resolve_crate_roots(dir);
            if roots.is_empty() {
                crate_errors.push(ErrorEntry::builder()
                    .file(dir.to_string_lossy().to_string())
                    .message("no crate entry points found".to_string())
                    .severity(ErrorSeverity::Warning)
                    .kind(DiagnosticKind::MissingCrateRoots)
                    .build());
                return (None, crate_errors);
            }

            let crate_type = if roots.iter().any(|(_, t)| *t == CrateType::Lib)
                && roots.iter().any(|(_, t)| *t == CrateType::Bin)
            {
                CrateType::LibAndBin
            } else {
                roots.first().map_or(CrateType::Lib, |(_, t)| *t)
            };

            let pkg_name = pkg.name.clone();
            let mut modules: Vec<ModuleInfo> = Vec::new();
            let mut collected_errors = Vec::new();
            for (root, _ty) in &roots {
                let (m, e) = module_tree::build_module_tree(root, &pkg_name);
                modules.extend(m);
                collected_errors.extend(e);
            }
            for err in collected_errors {
                crate_errors.push(err);
            }

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

            let crate_info = CrateInfo::builder()
                .name(pkg_name)
                .root(crate_root)
                .package(rebuilt_pkg)
                .modules(modules)
                .deps(deps)
                .build();

            (Some(crate_info), crate_errors)
        })
        .collect();

    let mut crate_infos: Vec<CrateInfo> = Vec::new();

    for (info, errs) in results {
        // Extend errors in both branches before checking info.
        // `errs` is moved by `extend` — this is fine because results is consumed by the for loop (move iteration).
        if let Some(ci) = info {
            crate_infos.push(ci);
        }
        crate_errors.extend(errs);
    }

    // Deterministic sort by crate name.
    crate_infos.sort_by(|a, b| a.name.cmp(&b.name));

    let cross_refs = cross_refs::compute(&mut crate_infos);

    // Derive flat indexes from crate info.
    let (symbols, name_index, files) = indexes::derive_from_crates(&crate_infos);

    // Run validation if enabled.
    if config.validate {
        let validate_findings = validate::validate(&crate_infos, &symbols, &workspace_root);
        crate_errors.extend(validate_findings);
    }

    let workspace_name = workspace_root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    let workspace_info = WorkspaceInfo::builder()
        .root(workspace_root.to_string_lossy().to_string())
        .workspace_name(workspace_name)
        .build();

    let map = WorkspaceMap::builder()
        .workspace(workspace_info)
        .crates(crate_infos)
        .cross_references(cross_refs)
        .symbols(symbols)
        .name_index(name_index)
        .files(files)
        .errors(crate_errors)
        .workspace_root(workspace_root.clone())
        .build();

    Ok(map)
}

/// Run the full workspace mapping pipeline.
///
/// 1. Discover workspace root and member crates.
/// 2. Process each crate in parallel (Cargo.toml parsing + module tree).
/// 3. Compute cross-crate references.
/// 4. Render JSON to stdout or the configured output file.
///
/// # Errors
///
/// Returns an error if the workspace root cannot be found, the workspace
/// Cargo.toml is missing a `[workspace]` section, member crates cannot be
/// parsed, or the JSON output cannot be written.
pub fn run(config: &Config) -> anyhow::Result<()> {
    let map = build_map(config)?;

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
