use crate::schema::{
    CanonicalPath, DiagnosticKind, ErrorEntry, ErrorSeverity, CrateInfo,
};
use std::collections::HashSet;
use std::path::Path;

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

    // Walk the src/ directory recursively.
    let mut findings = Vec::new();
    let entries = match std::fs::read_dir(&src_dir) {
        Ok(e) => e,
        Err(_) => return findings,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.file_name().map_or(false, |f| f == "lib.rs" || f == "main.rs") {
            continue; // Skip crate roots — they're declared implicitly.
        }
        if !path.extension().map_or(false, |e| e == "rs") {
            continue;
        }
        if path.file_name().map_or(false, |f| f == "mod.rs") {
            continue; // mod.rs files are declared by their parent directory.
        }
        if path.file_name().map_or(false, |f| f == "bin") {
            continue; // Skip src/bin/ directory.
        }

        let ws_rel = path.strip_prefix(workspace_root)
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();

        // Skip if declared in module tree.
        if declared_files.contains(&ws_rel) {
            continue;
        }

        // This file is orphaned. Determine parent file for fix hint.
        let parent_file = determine_parent_file(&ws_rel, crate_name, crate_info);

        let file_name = path
           .file_name()
           .map_or("unknown".to_string(), |f| f.to_string_lossy().into_owned());
       let stem = path
           .file_stem()
           .map_or(String::new(), |s| s.to_string_lossy().into_owned());
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
    _crate_info: &CrateInfo,
) -> String {
    // Strip src/ prefix, resolve parent directory to a module file.
    let stripped = orphan_file.strip_prefix("src/").unwrap_or(orphan_file);
    let parent_dir = stripped.rsplit_once('/').map(|(dir, _)| dir);

    match parent_dir {
        Some("") | None => {
            // File is directly in src/ — parent is lib.rs or main.rs.
            format!("{crate_name}/src/lib.rs")
        }
        Some(dir) => {
            // File is in a subdirectory — parent module file is dir/lib.rs or dir/mod.rs.
            format!("{crate_name}/src/{dir}/mod.rs")
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

/// Resolve an import path by expanding 'crate::', 'self::', 'super::' prefixes.
fn resolve_import_path(path: &str, crate_name: &str, module_path: &str) -> String {
    if let Some(rest) = path.strip_prefix("crate::") {
        format!("{crate_name}::{rest}")
    } else if let Some(rest) = path.strip_prefix("self::") {
        format!("{module_path}::{rest}")
    } else if let Some(rest) = path.strip_prefix("super::") {
        let parent = module_path.rsplit_once("::")
            .map(|(p, _)| p)
            .unwrap_or("");
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
