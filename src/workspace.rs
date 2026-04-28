use crate::schema::{CrateType, Error, Result};
use std::path::{Path, PathBuf};

/// Walk up the directory tree from `start_path` to find a `Cargo.toml`
/// containing a `[workspace]` section. Returns the directory containing it.
///
/// # Errors
///
/// Returns `Error::WorkspaceRootNotFound` if no `Cargo.toml` with a
/// `[workspace]` section is found in any ancestor directory.
pub fn find_workspace_root(start_path: &Path) -> Result<PathBuf> {
    for ancestor in start_path.ancestors() {
        let cargo_toml = ancestor.join("Cargo.toml");
        if cargo_toml.exists() {
            let content = std::fs::read_to_string(&cargo_toml).map_err(|source| Error::FileRead {
                path: cargo_toml.clone(),
                source,
            })?;
            if content.contains("[workspace]") {
                return Ok(ancestor.to_path_buf());
            }
        }
    }
    Err(Error::WorkspaceRootNotFound(start_path.to_path_buf()))
}

/// Parse the workspace `Cargo.toml`, resolve member paths (including glob
/// patterns), apply `exclude` list, and return absolute paths to each member
/// crate directory.
///
/// # Errors
///
/// Returns `Error::MissingWorkspaceSection` if the `Cargo.toml` lacks a
/// `[workspace]` section entirely.
pub fn enumerate_members(root: &Path) -> Result<Vec<PathBuf>> {
    let cargo_toml_path = root.join("Cargo.toml");
    let content = std::fs::read_to_string(&cargo_toml_path).map_err(|source| Error::FileRead {
        path: cargo_toml_path.clone(),
        source,
    })?;

    let parsed: toml::Value = toml::from_str(&content).map_err(|source| Error::TomlParse {
        path: cargo_toml_path.clone(),
        source,
    })?;

    let members: Vec<String> = match parsed.get("workspace") {
        None => return Err(Error::MissingWorkspaceSection),
        Some(workspace) => workspace
            .get("members")
            .and_then(|m| m.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default(),
    };

    let exclude: Vec<String> = parsed
        .get("workspace")
        .and_then(|w| w.get("exclude"))
        .and_then(|e| e.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let mut result = Vec::new();
    for member in &members {
        let has_glob = member.contains('*') || member.contains('?') || member.contains('[');
        if has_glob {
            let pattern = root.join(member).to_string_lossy().to_string();
            let iter = glob::glob(&pattern).map_err(|e| Error::GlobPattern(e.to_string()))?;
            for entry in iter {
                let path = entry.map_err(|e| Error::GlobPattern(e.to_string()))?;
                if path.is_dir() && path.join("Cargo.toml").exists() {
                    result.push(path);
                }
            }
        } else {
            let path = root.join(member);
            if path.is_dir() && path.join("Cargo.toml").exists() {
                result.push(path);
            } else {
                eprintln!(
                    "warning: workspace member {} does not exist",
                    path.display()
                );
            }
        }
    }

    result.retain(|p| {
        let name = p
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        !exclude.contains(&name)
    });

    result.sort();
    result.dedup();
    Ok(result)
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_cargo_toml(dir: &std::path::Path, content: &str) {
        let mut f = std::fs::File::create(dir.join("Cargo.toml")).unwrap();
        f.write_all(content.as_bytes()).unwrap();
    }

    fn setup_crate(dir: &std::path::Path) {
        let src = dir.join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("lib.rs"), "").unwrap();
        let mut f = std::fs::File::create(dir.join("Cargo.toml")).unwrap();
        use std::io::Write;
        writeln!(f, "[package]").unwrap();
        writeln!(f, "name = \"{}\"", dir.file_name().unwrap().to_string_lossy()).unwrap();
        writeln!(f, "version = \"0.1.0\"").unwrap();
        writeln!(f, "edition = \"2021\"").unwrap();
    }

    #[test]
    fn find_workspace_root_finds_cargo_toml() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("subdir").join("nested");
        std::fs::create_dir_all(&path).unwrap();
        write_cargo_toml(tmp.path(), "[workspace]");
        let result = find_workspace_root(&path).unwrap();
        assert_eq!(result, tmp.path());
    }

    #[test]
    fn enumerate_members_returns_members() {
        let tmp = tempfile::tempdir().unwrap();
        write_cargo_toml(tmp.path(), r#"
[workspace]
members = ["crate_a", "crate_b"]
"#);
        setup_crate(tmp.path().join("crate_a").as_path());
        setup_crate(tmp.path().join("crate_b").as_path());
        let members = enumerate_members(tmp.path()).unwrap();
        assert_eq!(members.len(), 2);
    }

    #[test]
    fn enumerate_members_returns_err_for_missing_workspace() {
        let tmp = tempfile::tempdir().unwrap();
        write_cargo_toml(tmp.path(), "[dependencies]\nfoo = \"1\"");
        let result = enumerate_members(tmp.path());
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::MissingWorkspaceSection => {},
            other => panic!("expected MissingWorkspaceSection, got {:?}", other),
        }
    }

    #[test]
    fn enumerate_members_applies_exclude() {
        let tmp = tempfile::tempdir().unwrap();
        write_cargo_toml(tmp.path(), r#"
[workspace]
members = ["a", "b", "c"]
exclude = ["b"]
"#);
        setup_crate(tmp.path().join("a").as_path());
        setup_crate(tmp.path().join("b").as_path());
        setup_crate(tmp.path().join("c").as_path());
        let members = enumerate_members(tmp.path()).unwrap();
        let names: Vec<_> = members.iter().map(|p| p.file_name().unwrap().to_string_lossy()).collect();
        assert!(names.iter().any(|n| *n == "a"));
        assert!(!names.iter().any(|n| *n == "b"));
        assert!(names.iter().any(|n| *n == "c"));
    }

    #[test]
    fn resolve_crate_roots_detects_lib() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join("src")).unwrap();
        std::fs::write(tmp.path().join("src").join("lib.rs"), "").unwrap();
        let roots = resolve_crate_roots(tmp.path());
        assert_eq!(roots.len(), 1);
        assert_eq!(roots[0].1, CrateType::Lib);
    }

    #[test]
    fn resolve_crate_roots_detects_bin() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join("src")).unwrap();
        std::fs::write(tmp.path().join("src").join("main.rs"), "").unwrap();
        let roots = resolve_crate_roots(tmp.path());
        assert_eq!(roots.len(), 1);
        assert_eq!(roots[0].1, CrateType::Bin);
    }
}
/// Returns `(path, CrateType)` pairs — one for `src/lib.rs` (Lib),
/// one for `src/main.rs` (Bin), or empty if neither exists.
#[must_use]
pub fn resolve_crate_roots(crate_dir: &Path) -> Vec<(PathBuf, CrateType)> {
    let mut roots = Vec::new();
    let lib_rs = crate_dir.join("src").join("lib.rs");
    let main_rs = crate_dir.join("src").join("main.rs");
    if lib_rs.exists() {
        roots.push((lib_rs, CrateType::Lib));
    }
    if main_rs.exists() {
        roots.push((main_rs, CrateType::Bin));
    }
    roots
}
