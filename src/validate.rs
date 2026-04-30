use crate::schema::{
    CanonicalPath, DiagnosticKind, ErrorEntry, ErrorSeverity, CrateInfo,
};
use std::collections::HashSet;
use std::path::Path;
use walkdir::WalkDir;

/// Run validation checks on the crate set.
///
/// Checks performed:
/// - **Orphan files**: `.rs` files on disk not declared in any module tree
/// - **Dead re-exports**: `pub use` to symbols not found in the symbol index
///
/// Returns a list of `ErrorEntry` findings (severity: Warning).
/// Check whether an import path targets an external crate.
///
/// Returns `true` if the path points to a crate that is not a workspace member
/// and not a top-level module of the current crate. Such re-exports should be
/// skipped during dead-re-export checking because the symbol lives in an
/// external crate's public API.
#[must_use]
fn is_external_crate_re_export(
    import_path: &str,
    crate_names: &HashSet<&str>,
    crate_info: &CrateInfo,
) -> bool {
    // `self::` and `super::` are always internal to the current crate.
    if import_path.starts_with("self::") || import_path.starts_with("super::") {
        return false;
    }

    // `crate::` is always internal to the current crate.
    if import_path.starts_with("crate::") {
        return false;
    }

    // Bare path — check the first `::`-delimited segment.
    let first_seg = import_path.split_once("::").map_or(import_path, |(seg, _)| seg);

    // Single-segment bare path (no `::`) is treated as internal.
    if !import_path.contains("::") {
        return false;
    }

    // Check if the first segment is a top-level module of the current crate.
    let is_top_level_module = crate_info
        .modules
        .iter()
        .any(|m| m.path == format!("{}::{first_seg}", crate_info.name));

    if is_top_level_module {
        return false;
    }

    // Not a top-level module. If it's a workspace member, it's a cross-workspace
    // re-export — let the existing cross-crate check handle it (return true to
    // skip here so the downstream logic doesn't fire, but the cross-crate check
    // below will also skip it).
    if crate_names.contains(first_seg) {
        return true;
    }

    // Neither a top-level module nor a workspace member: definitely external.
    true
}

/// Run validation checks on the crate set.
///
/// Checks performed:
/// - **Orphan files**: `.rs` files on disk not declared in any module tree
/// - **Dead re-exports**: `pub use` to symbols not found in the symbol index
///
/// Returns a list of `ErrorEntry` findings (severity: Warning).
#[must_use]
pub fn validate(
    crates: &[CrateInfo],
    symbols: &std::collections::BTreeMap<CanonicalPath, crate::schema::SymbolEntry>,
    workspace_root: &Path,
) -> Vec<ErrorEntry> {
    let mut findings: Vec<ErrorEntry> = Vec::new();
    let crate_names: std::collections::HashSet<&str> =
        crates.iter().map(|c| c.name.as_str()).collect();

    for crate_info in crates {
        findings.extend(check_orphan_files(crate_info, workspace_root));
        findings.extend(check_dead_reexports(crate_info, symbols, &crate_names));
    }

    findings
}

/// Find `.rs` files on disk that are not declared in the module tree.
fn check_orphan_files(
    crate_info: &CrateInfo,
    workspace_root: &Path,
) -> Vec<ErrorEntry> {
    // crate_info.root is a relativized path to the crate root file (e.g., "src/lib.rs").
    // We need the src/ directory, which is the parent of the root file.
    let crate_root_path = workspace_root.join(&crate_info.root);
    let src_dir = crate_root_path
        .parent()
        .unwrap_or(workspace_root)
        .to_path_buf();
    let crate_name = &crate_info.name;

    // Collect all file paths declared in the module tree.
    let declared_files: HashSet<String> = crate_info
        .modules
        .iter()
        .filter(|m| m.file != "<unresolved>")
        .map(|m| m.file.clone())
        .collect();

    // Walk the src/ directory recursively using walkdir.
    let mut findings = Vec::new();

    for entry in WalkDir::new(&src_dir).into_iter().filter_entry(|e| {
        // Always yield the root; keep files (filtered later); skip hidden, bin/, tests/ dirs.
        e.depth() == 0 || !e.file_type().is_dir() || {
            let name = e.file_name().to_string_lossy();
            !name.starts_with('.') && name != "bin" && name != "tests"
        }
    }) {
        let Ok(entry) = entry else { continue };
        let path = entry.path();

        if !path.is_file() {
            continue;
        }
        if path.extension().is_none_or(|e| e != "rs") {
            continue;
        }
        let file_name = path.file_name().map_or_else(String::new, |f| f.to_string_lossy().into_owned());
        if file_name == "lib.rs" || file_name == "main.rs" || file_name == "mod.rs" {
            continue; // Crate roots and mod.rs — declared implicitly.
        }

        let ws_rel = path.strip_prefix(workspace_root)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();

        if declared_files.contains(&ws_rel) {
            continue;
        }

        let parent_file = determine_parent_file(&ws_rel, crate_name, crate_info);
        let stem = path.file_stem()
            .map_or_else(String::new, |s| s.to_string_lossy().into_owned());
        findings.push(ErrorEntry::builder()
            .file(ws_rel.clone())
            .message(format!(
                "orphan file: '{file_name}' is not declared in the module tree. Add 'pub mod {stem};' to {parent_file}."
            ))
            .severity(ErrorSeverity::Warning)
            .kind(DiagnosticKind::OrphanFile)
            .build());
    }

    findings
}

/// Determine the parent file for an orphan file's fix hint.
fn determine_parent_file(
    orphan_file: &str,
    crate_name: &str,
    crate_info: &CrateInfo,
) -> String {
    // Derive the src/ directory from crate_info.root (e.g. "src/lib.rs" → "src").
    let src_dir = std::path::Path::new(&crate_info.root)
        .parent()
        .map_or(std::path::Path::new(""), |p| p);
    let src_prefix = format!("{}/", src_dir.display());

    // Strip the src/ prefix, resolve parent directory to a module file.
    let stripped = orphan_file.strip_prefix(&src_prefix).unwrap_or(orphan_file);
    let parent_dir = stripped.rsplit_once('/').map(|(dir, _)| dir);

    match parent_dir {
        Some("") | None => {
            // File is directly in src/ — parent is lib.rs or main.rs.
            format!("{crate_name}/{src_prefix}lib.rs")
        }
        Some(dir) => {
            // File is in a subdirectory — parent module file is dir/lib.rs or dir/mod.rs.
            format!("{crate_name}/{src_prefix}{dir}/mod.rs")
        }
    }
}

/// Find `pub use` re-exports that reference symbols not in the index.
fn check_dead_reexports(
    crate_info: &CrateInfo,
    symbols: &std::collections::BTreeMap<CanonicalPath, crate::schema::SymbolEntry>,
    crate_names: &HashSet<&str>,
) -> Vec<ErrorEntry> {
    let mut findings = Vec::new();
    let my_name = crate_info.name.as_str();

    for module in &crate_info.modules {
        for re_export in &module.re_exports {
            let path = &re_export.import_path;

            // Skip glob re-exports.
            if path.ends_with("::*") {
                continue;
            }

            // Skip re-exports targeting external crates.
            if is_external_crate_re_export(path, crate_names, crate_info) {
                continue;
            }

            // Parse the import path. Resolve 'crate::' prefix.
            let resolved = resolve_import_path(path, my_name, &module.path);

            // If first segment is a workspace member crate name, skip (external re-export).
            let first_seg = resolved.split("::").next().unwrap_or(&resolved);
            if crate_names.contains(first_seg) && first_seg != my_name {
                continue;
            }

            // Build the canonical path from the resolved import path.
            let canonical = CanonicalPath::from(resolved.clone());

            // Look up in the symbols map.
            if !symbols.contains_key(&canonical) {
                findings.push(ErrorEntry::builder()
                    .file(module.file.clone())
                    .line(re_export.line)
                    .message(format!(
                        "dead re-export: '{}' resolves to '{}' which is not found in the symbol index. Remove or fix the 'pub use' statement.",
                        re_export.export_path,
                        re_export.import_path
                    ))
                    .severity(ErrorSeverity::Warning)
                    .kind(DiagnosticKind::DeadReExport)
                    .build());
            }
        }
    }

    findings
}

/// Resolve an import path by expanding `crate::`, `self::`, `super::` prefixes.
fn resolve_import_path(path: &str, crate_name: &str, module_path: &str) -> String {
    if let Some(rest) = path.strip_prefix("crate::") {
        format!("{crate_name}::{rest}")
    } else if let Some(rest) = path.strip_prefix("self::") {
        format!("{module_path}::{rest}")
    } else if let Some(rest) = path.strip_prefix("super::") {
        let parent = module_path.rsplit_once("::")
            .map_or("", |(p, _)| p);
        if parent.is_empty() {
            format!("{crate_name}::{rest}")
        } else {
            format!("{parent}::{rest}")
        }
    } else {
        // Bare path — assume it's relative to the crate root.
        format!("{crate_name}::{path}")
    }
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use crate::schema::{CrateInfo, DepInfo, CrateType, ModuleInfo, PackageInfo};

    #[test]
    fn test_is_external_crate_re_export() {
        let crate_info = CrateInfo::builder()
            .name("mycrate".to_string())
            .root("src/lib.rs".to_string())
            .package(
                PackageInfo::builder()
                    .name("mycrate".to_string())
                    .version("0.1.0".to_string())
                    .edition("2021".to_string())
                    .crate_type(CrateType::Lib)
                    .build(),
            )
            .modules(vec![
                ModuleInfo::builder()
                    .path("mycrate".to_string())
                    .file("src/lib.rs".to_string())
                    .visibility("pub".to_string())
                    .build(),
                ModuleInfo::builder()
                    .path("mycrate::utils".to_string())
                    .file("src/utils.rs".to_string())
                    .visibility("pub".to_string())
                    .build(),
            ])
            .deps(DepInfo::default())
            .build();

        let mut crate_names = HashSet::new();
        crate_names.insert("mycrate");

        // Bare path: "serde" is not a workspace member and not a top-level module → external
        assert!(
            is_external_crate_re_export("serde::Serialize", &crate_names, &crate_info),
            "serde::Serialize should be external"
        );

        // Bare path: "utils" IS a top-level module of mycrate → internal
        assert!(
            !is_external_crate_re_export("utils::Helper", &crate_names, &crate_info),
            "utils::Helper should be internal (utils is a top-level module)"
        );

        // crate:: prefix is always internal
        assert!(
            !is_external_crate_re_export("crate::sub::missing", &crate_names, &crate_info),
            "crate:: path should always be internal"
        );

        // self:: prefix is always internal
        assert!(
            !is_external_crate_re_export("self::inner::Type", &crate_names, &crate_info),
            "self:: path should always be internal"
        );
    }

    #[test]
    fn test_is_external_crate_re_export_super_prefix() {
        let crate_info = CrateInfo::builder()
            .name("mycrate".to_string())
            .root("src/lib.rs".to_string())
            .package(
                PackageInfo::builder()
                    .name("mycrate".to_string())
                    .version("0.1.0".to_string())
                    .edition("2021".to_string())
                    .crate_type(CrateType::Lib)
                    .build(),
            )
            .modules(vec![])
            .deps(DepInfo::default())
            .build();

        let crate_names: HashSet<&str> = HashSet::new();

        // super:: prefix is always internal regardless of workspace members.
        assert!(
            !is_external_crate_re_export("super::other::Thing", &crate_names, &crate_info),
            "super:: path should always be internal"
        );
    }

    #[test]
    fn test_is_external_crate_re_export_single_segment() {
        let crate_info = CrateInfo::builder()
            .name("mycrate".to_string())
            .root("src/lib.rs".to_string())
            .package(
                PackageInfo::builder()
                    .name("mycrate".to_string())
                    .version("0.1.0".to_string())
                    .edition("2021".to_string())
                    .crate_type(CrateType::Lib)
                    .build(),
            )
            .modules(vec![])
            .deps(DepInfo::default())
            .build();

        let crate_names: HashSet<&str> = HashSet::new();

        // Single-segment bare path (no ::) is treated as internal.
        assert!(
            !is_external_crate_re_export("Foo", &crate_names, &crate_info),
            "bare single-segment path should be internal"
        );
    }

    #[test]
    fn test_is_external_crate_re_export_cross_workspace() {
        let crate_info = CrateInfo::builder()
            .name("mycrate".to_string())
            .root("src/lib.rs".to_string())
            .package(
                PackageInfo::builder()
                    .name("mycrate".to_string())
                    .version("0.1.0".to_string())
                    .edition("2021".to_string())
                    .crate_type(CrateType::Lib)
                    .build(),
            )
            .modules(vec![])
            .deps(DepInfo::default())
            .build();

        let mut crate_names = HashSet::new();
        crate_names.insert("othercrate");
        crate_names.insert("mycrate");

        // "othercrate" is a workspace member but NOT a top-level module → external
        // (G1 skips it; the existing cross-crate check below handles it).
        assert!(
            is_external_crate_re_export("othercrate::Thing", &crate_names, &crate_info),
            "cross-workspace-member re-export should be external (passes G1)"
        );
    }

    #[test]
    fn test_is_external_crate_re_export_empty_path() {
        let crate_info = CrateInfo::builder()
            .name("mycrate".to_string())
            .root("src/lib.rs".to_string())
            .package(
                PackageInfo::builder()
                    .name("mycrate".to_string())
                    .version("0.1.0".to_string())
                    .edition("2021".to_string())
                    .crate_type(CrateType::Lib)
                    .build(),
            )
            .modules(vec![])
            .deps(DepInfo::default())
            .build();

        let crate_names: HashSet<&str> = HashSet::new();

        // Empty string is not a valid path; treat as internal (defensive).
        assert!(
            !is_external_crate_re_export("", &crate_names, &crate_info),
            "empty path should be treated as internal"
        );
    }

    #[test]
    fn resolve_import_path_crate_prefix() {
        assert_eq!(
            resolve_import_path("crate::foo::bar", "mycrate", "mycrate::sub"),
            "mycrate::foo::bar"
        );
    }

    #[test]
    fn resolve_import_path_self_prefix() {
        assert_eq!(
            resolve_import_path("self::inner", "mycrate", "mycrate::sub"),
            "mycrate::sub::inner"
        );
    }

    #[test]
    fn resolve_import_path_super_prefix() {
        assert_eq!(
            resolve_import_path("super::other", "mycrate", "mycrate::sub::deep"),
            "mycrate::sub::other"
        );
    }

    #[test]
    fn resolve_import_path_super_root() {
        // At crate root, super:: wraps to crate root.
        assert_eq!(
            resolve_import_path("super::helper", "mycrate", "mycrate"),
            "mycrate::helper"
        );
    }

    #[test]
    fn resolve_import_path_bare() {
        assert_eq!(
            resolve_import_path("foo::Bar", "mycrate", "mycrate::sub"),
            "mycrate::foo::Bar"
        );
    }
}
