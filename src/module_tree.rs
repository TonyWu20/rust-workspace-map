use crate::file_parser;
use crate::schema::{FileInfo, ModuleInfo, Result, SubmoduleDecl};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Resolve a `mod name;` declaration to a file path.
/// Tries `{parent_dir}/{mod_name}.rs` first, then `{parent_dir}/{mod_name}/mod.rs`.
pub fn resolve_module_path(parent_dir: &Path, mod_name: &str) -> Option<PathBuf> {
    let rs_file = parent_dir.join(format!("{}.rs", mod_name));
    if rs_file.exists() {
        return Some(rs_file);
    }
    let mod_dir = parent_dir.join(mod_name).join("mod.rs");
    if mod_dir.exists() {
        return Some(mod_dir);
    }
    None
}

/// Build the full module tree for a crate starting from its entry point
/// (e.g., `src/lib.rs`). Returns a flat `Vec<ModuleInfo>` containing the
/// root module and all recursively discovered submodules.
pub fn build_module_tree(crate_root: &Path, crate_name: &str) -> Result<Vec<ModuleInfo>> {
    let mut visited = HashSet::new();
    let parent_dir = crate_root.parent().unwrap_or_else(|| Path::new("."));

    let (ast, file_info) = file_parser::parse_file(crate_root)?;
    visited.insert(crate_root.to_path_buf());

    let root_module = build_module_info(
        crate_name,
        crate_root,
        "pub",
        &file_info.public_items,
        &file_info.imports,
        &file_info.re_exports,
        &file_info.submodules,
    );

    let mut modules = vec![root_module];

    for sub in &file_info.submodules {
        if sub.is_test {
            continue;
        }
        let sub_module_path = format!("{}::{}", crate_name, sub.name);
        let child_modules = process_submodule(
            &sub_module_path,
            &sub.name,
            &ast.items,
            parent_dir,
            crate_root,
            &mut visited,
        )?;
        modules.extend(child_modules);
    }

    Ok(modules)
}

// ── Internal helpers ────────────────────────────────────────────────────

fn build_module_info(
    path: &str,
    file: &Path,
    visibility: &str,
    public_items: &[crate::schema::PublicItem],
    imports: &[crate::schema::Import],
    re_exports: &[crate::schema::ReExport],
    submodules: &[SubmoduleDecl],
) -> ModuleInfo {
    ModuleInfo::builder()
        .path(path.to_string())
        .file(file.to_string_lossy().to_string())
        .visibility(visibility.to_string())
        .public_items(public_items.to_vec())
        .imports(imports.to_vec())
        .re_exports(re_exports.to_vec())
        .submodules(
            submodules
                .iter()
                .map(|s| s.name.clone())
                .collect::<Vec<_>>(),
        )
        .build()
}

fn process_submodule(
    module_path: &str,
    mod_name: &str,
    parent_items: &[syn::Item],
    parent_dir: &Path,
    parent_file: &Path,
    visited: &mut HashSet<PathBuf>,
) -> Result<Vec<ModuleInfo>> {
    // Locate the `mod` item in the parent's AST.
    let mod_item = parent_items.iter().find_map(|item| {
        if let syn::Item::Mod(m) = item {
            if m.ident == mod_name {
                return Some(m);
            }
        }
        None
    });

    let Some(mod_item) = mod_item else {
        eprintln!("warning: orphaned module {}", module_path);
        return Ok(vec![ModuleInfo::builder()
            .path(module_path.to_string())
            .file("<unresolved>".to_string())
            .visibility("private".to_string())
            .build()]);
    };

    let visibility = if matches!(mod_item.vis, syn::Visibility::Public(_)) {
        "pub"
    } else {
        "private"
    };

    if let Some((_, ref inline_items)) = mod_item.content {
        // Inline module: process its body items directly (no file lookup).
        process_module_items(
            module_path,
            parent_file,
            visibility,
            inline_items,
            parent_dir,
            visited,
        )
    } else {
        // External module: resolve file path, parse, and recurse.
        let file_path = resolve_module_path(parent_dir, mod_name);
        let Some(ref file_path) = file_path else {
            eprintln!("warning: orphaned module {}", module_path);
            return Ok(vec![ModuleInfo::builder()
                .path(module_path.to_string())
                .file("<unresolved>".to_string())
                .visibility(visibility.to_string())
                .build()]);
        };

        if visited.contains(file_path.as_path()) {
            return Ok(vec![]); // cycle detected
        }
        visited.insert(file_path.clone());

        let (ast, file_info) = file_parser::parse_file(file_path)?;
        process_module_info(
            module_path,
            file_path,
            visibility,
            &file_info,
            &ast.items,
            &file_path.parent().unwrap_or_else(|| Path::new(".")),
            visited,
        )
    }
}

fn process_module_items(
    module_path: &str,
    file_path: &Path,
    visibility: &str,
    items: &[syn::Item],
    parent_dir: &Path,
    visited: &mut HashSet<PathBuf>,
) -> Result<Vec<ModuleInfo>> {
    let file_info = FileInfo {
        public_items: file_parser::extract_public_items(items),
        imports: file_parser::extract_imports(items),
        re_exports: file_parser::extract_re_exports(items),
        submodules: file_parser::extract_submodules(items),
        impls: file_parser::extract_impls(items),
    };
    process_module_info(module_path, file_path, visibility, &file_info, items, parent_dir, visited)
}

fn process_module_info(
    module_path: &str,
    file_path: &Path,
    visibility: &str,
    file_info: &FileInfo,
    items: &[syn::Item],
    _parent_dir: &Path,
    visited: &mut HashSet<PathBuf>,
) -> Result<Vec<ModuleInfo>> {
    let mut modules = vec![build_module_info(
        module_path,
        file_path,
        visibility,
        &file_info.public_items,
        &file_info.imports,
        &file_info.re_exports,
        &file_info.submodules,
    )];

    for sub in &file_info.submodules {
        if sub.is_test {
            continue;
        }
        let child_path = format!("{}::{}", module_path, sub.name);
        let child_dir = file_path.parent().unwrap_or_else(|| Path::new("."));
        let child_modules = process_submodule(
            &child_path,
            &sub.name,
            items,
            child_dir,
            file_path,
            visited,
        )?;
        modules.extend(child_modules);
    }

    Ok(modules)
}
