use crate::file_parser;
use crate::schema::{ErrorContext, ErrorEntry, ErrorSeverity, FileInfo, ModuleInfo, SubmoduleDecl};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Resolve a `mod name;` declaration to a file path.
/// Tries `{parent_dir}/{mod_name}.rs` first, then `{parent_dir}/{mod_name}/mod.rs`.
///
/// Returns `None` if neither path exists.
#[must_use]
pub fn resolve_module_path(parent_dir: &Path, mod_name: &str) -> Option<PathBuf> {
    let rs_file = parent_dir.join(format!("{mod_name}.rs"));
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
/// (e.g., `src/lib.rs`). Returns a tuple of module info and any errors
/// encountered during submodule parsing (including orphaned module warnings).
#[must_use]
pub fn build_module_tree(
    crate_root: &Path,
    crate_name: &str,
) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {

    let mut visited = HashSet::new();
    let parent_dir = crate_root.parent().unwrap_or(crate_root);

    let parsed = file_parser::parse_file(crate_root);
    let mut errors: Vec<ErrorEntry> = Vec::new();
    if let Some(ref err) = parsed.parse_error {
        errors.push(crate::file_parser::build_parse_error_entry(crate_root, err));
    }
    visited.insert(crate_root.to_path_buf());

    let root_module = build_module_info(
        crate_name,
        crate_root,
        "pub",
        &parsed.file_info.public_items,
        &parsed.file_info.imports,
        &parsed.file_info.re_exports,
        &parsed.file_info.submodules,
    );

    let mut modules = vec![root_module];

    for sub in &parsed.file_info.submodules {
        if sub.is_test {
            continue;
        }
        let sub_module_path = format!("{}::{}", crate_name, sub.name);
        let (child_modules, child_errors) = process_submodule(
            &sub_module_path,
            &sub.name,
            &parsed.ast.items,
            parent_dir,
            crate_root,
            &mut visited,
        );
        errors.extend(child_errors);
        modules.extend(child_modules);
    }

    (modules, errors)
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
) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {
    // Locate the `mod` item in the parent's AST.
    let mod_item = parent_items.iter().find_map(|item| {
        if let syn::Item::Mod(m) = item
            && m.ident == mod_name
        {
            return Some(m);
        }
        None
    });

    let Some(mod_item) = mod_item else {
        let err = ErrorEntry::builder()
            .file(String::new())
            .message(format!("orphaned module: {module_path}"))
            .severity(ErrorSeverity::Warning)
            .kind("orphaned_module".to_string())
            .context(ErrorContext::builder()
                .module_path(module_path.to_string())
                .build())
            .build();
        return (vec![ModuleInfo::builder()
            .path(module_path.to_string())
            .file("<unresolved>".to_string())
            .visibility("private".to_string())
            .build()], vec![err]);
    };

    let visibility = if matches!(mod_item.vis, syn::Visibility::Public(_)) {
        "pub"
    } else {
        "private"
    };

    if let Some((_, ref inline_items)) = mod_item.content {
        // Inline module: process its body items directly (no file lookup).
        let (modules, errs) = process_module_items(
            module_path,
            parent_file,
            visibility,
            inline_items,
            parent_dir,
            visited,
        );
        return (modules, errs);
    }
    // External module: resolve file path, parse, and recurse.
    let file_path = resolve_module_path(parent_dir, mod_name);
    let Some(ref file_path) = file_path else {
        let err = ErrorEntry::builder()
            .file(String::new())
            .message(format!("orphaned module: {module_path}"))
            .severity(ErrorSeverity::Warning)
            .kind("orphaned_module".to_string())
            .context(ErrorContext::builder()
                .module_path(module_path.to_string())
                .build())
            .build();
        return (vec![ModuleInfo::builder()
            .path(module_path.to_string())
            .file("<unresolved>".to_string())
            .visibility(visibility.to_string())
            .build()], vec![err]);
    };

    if visited.contains(file_path.as_path()) {
        return (vec![], vec![]); // cycle detected
    }
    visited.insert(file_path.clone());

    let parsed = file_parser::parse_file(file_path);
    let mut errors: Vec<ErrorEntry> = Vec::new();
    if let Some(ref err) = parsed.parse_error {
        errors.push(crate::file_parser::build_parse_error_entry(file_path, err));
    }
    process_module_info(
        module_path,
        file_path,
        visibility,
        &parsed.file_info,
        &parsed.ast.items,
        file_path.parent().unwrap_or(file_path),
        visited,
        &mut errors,
    )
}

fn process_module_items(
    module_path: &str,
    file_path: &Path,
    visibility: &str,
    items: &[syn::Item],
    parent_dir: &Path,
    visited: &mut HashSet<PathBuf>,
) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {
    let file_info = FileInfo {
        public_items: file_parser::extract_public_items(items),
        imports: file_parser::extract_imports(items),
        re_exports: file_parser::extract_re_exports(items),
        submodules: file_parser::extract_submodules(items),
        impls: file_parser::extract_impls(items),
    };
    process_module_info(module_path, file_path, visibility, &file_info, items, parent_dir, visited, &mut Vec::new())
}

#[allow(clippy::too_many_arguments)]
fn process_module_info(
    module_path: &str,
    file_path: &Path,
    visibility: &str,
    file_info: &FileInfo,
    items: &[syn::Item],
    _parent_dir: &Path,
    visited: &mut HashSet<PathBuf>,
    errors: &mut Vec<ErrorEntry>,
) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {
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
        let child_dir = file_path.parent().unwrap_or(file_path);
        let (child_modules, child_errors) = process_submodule(
            &child_path,
            &sub.name,
            items,
            child_dir,
            file_path,
            visited,
        );
        errors.extend(child_errors);
        modules.extend(child_modules);
    }

    (modules, errors.clone())
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_module_path_finds_rs_file() {
        let tmp = std::env::temp_dir().join("resolve_test");
        let _ = std::fs::create_dir_all(&tmp);
        let mod_file = tmp.join("foo.rs");
        std::fs::write(&mod_file, "").ok();
        let result = resolve_module_path(&tmp, "foo");
        assert_eq!(result, Some(mod_file));
        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn resolve_module_path_finds_mod_rs() {
        let tmp = std::env::temp_dir().join("resolve_test2");
        let _ = std::fs::create_dir_all(&tmp);
        let mod_dir = tmp.join("bar");
        let _ = std::fs::create_dir_all(&mod_dir);
        let mod_rs = mod_dir.join("mod.rs");
        std::fs::write(&mod_rs, "").ok();
        let result = resolve_module_path(&tmp, "bar");
        assert_eq!(result, Some(mod_rs));
        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn resolve_module_path_returns_none_for_missing() {
        let tmp = std::env::temp_dir().join("resolve_test3");
        let _ = std::fs::create_dir_all(&tmp);
        let result = resolve_module_path(&tmp, "nonexistent");
        assert!(result.is_none());
        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn build_module_tree_returns_empty_for_nonexistent() {
        let tmp = std::env::temp_dir().join("bmt_test");
        let _ = std::fs::create_dir_all(&tmp);
        let (modules, errors) = build_module_tree(&tmp, "test");
        assert!(!modules.is_empty());
        assert!(!errors.is_empty());
        std::fs::remove_dir_all(&tmp).ok();
    }
}
