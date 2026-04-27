use crate::schema::{CrateType, Error, Result};
use std::path::{Path, PathBuf};

/// Walk up the directory tree from `start_path` to find a `Cargo.toml`
/// containing a `[workspace]` section. Returns the directory containing it.
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

    let members: Vec<String> = parsed
        .get("workspace")
        .and_then(|w| w.get("members"))
        .and_then(|m| m.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

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

/// For a crate directory, determine its entry-point file(s).
/// Returns `(path, CrateType)` pairs — one for `src/lib.rs` (Lib),
/// one for `src/main.rs` (Bin), or empty if neither exists.
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
