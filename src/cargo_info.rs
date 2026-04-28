use crate::schema::{CrateType, DepInfo, Error, PackageInfo, Result};
use std::path::Path;

/// Parse a crate's `Cargo.toml` and return package metadata and dependency lists.
/// The returned `PackageInfo.crate_type` is set to `Lib` by default; the caller
/// overrides it based on `workspace::resolve_crate_roots`.
///
/// # Errors
///
/// Returns an error if the file cannot be read or parsed.
pub fn parse_cargo_toml(path: &Path) -> Result<(PackageInfo, DepInfo)> {
    let content = std::fs::read_to_string(path).map_err(|source| Error::FileRead {
        path: path.to_path_buf(),
        source,
    })?;

    let parsed: toml::Value = toml::from_str(&content).map_err(|source| Error::TomlParse {
        path: path.to_path_buf(),
        source,
    })?;

    let default_package = || {
        PackageInfo::builder()
            .name("unknown".to_string())
            .version("0.0.0".to_string())
            .edition("2021".to_string())
            .crate_type(CrateType::Lib)
            .build()
    };

    let package = parsed
        .get("package")
        .map_or_else(default_package, |p| {
            PackageInfo::builder()
                .name(
                    p.get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown")
                        .to_string(),
                )
                .version(
                    p.get("version")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0.0.0")
                        .to_string(),
                )
                .edition(
                    p.get("edition")
                        .and_then(|v| v.as_str())
                        .unwrap_or("2021")
                        .to_string(),
                )
                .crate_type(CrateType::Lib)
                .build()
        });

    let extract_deps = |section: &str| -> (Vec<String>, Vec<String>) {
        let mut normal = Vec::new();
        let mut workspace_members = Vec::new();
        if let Some(table) = parsed.get(section).and_then(|v| v.as_table()) {
            let mut keys: Vec<&String> = table.keys().collect();
            keys.sort();
            for key in keys {
                if let Some(value) = table.get(key) {
                    let is_workspace_dep = value
                        .as_table()
                        .and_then(|t| t.get("workspace"))
                        .and_then(toml::Value::as_bool)
                        == Some(true);
                    if is_workspace_dep {
                        workspace_members.push(key.clone());
                    } else {
                        normal.push(key.clone());
                    }
                }
            }
        }
        (normal, workspace_members)
    };

    let (normal_deps, mut ws_deps) = extract_deps("dependencies");
    let (dev_deps, ws_dev_deps) = extract_deps("dev-dependencies");
    ws_deps.extend(ws_dev_deps);
    ws_deps.sort();
    ws_deps.dedup();

    let deps = DepInfo::builder()
        .normal(normal_deps)
        .dev(dev_deps)
        .workspace_members(ws_deps)
        .build();

    Ok((package, deps))
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

    #[test]
    fn parse_cargo_toml_parses_minimal() {
        let tmp = tempfile::tempdir().unwrap();
        write_cargo_toml(tmp.path(), r#"
[package]
name = "test-pkg"
version = "1.0.0"
edition = "2021"
"#);
        let (pkg, _deps) = parse_cargo_toml(tmp.path().join("Cargo.toml").as_path()).unwrap();
        assert_eq!(pkg.name, "test-pkg");
        assert_eq!(pkg.version, "1.0.0");
        assert_eq!(pkg.edition, "2021");
    }

    #[test]
    fn parse_cargo_toml_uses_defaults_for_missing_package() {
        let tmp = tempfile::tempdir().unwrap();
        write_cargo_toml(tmp.path(), "");
        let (pkg, _deps) = parse_cargo_toml(tmp.path().join("Cargo.toml").as_path()).unwrap();
        assert_eq!(pkg.name, "unknown");
        assert_eq!(pkg.edition, "2021");
    }

    #[test]
    fn parse_cargo_toml_distinguishes_deps() {
        let tmp = tempfile::tempdir().unwrap();
        write_cargo_toml(tmp.path(), r#"
[package]
name = "test-pkg"
version = "0.1.0"
edition = "2021"

[dependencies]
foo = "1"
bar = { workspace = true }

[dev-dependencies]
baz = "2"
qux = { workspace = true }
"#);
        let (_, deps) = parse_cargo_toml(tmp.path().join("Cargo.toml").as_path()).unwrap();
        assert_eq!(deps.normal, vec!["foo"]);
        assert_eq!(deps.dev, vec!["baz"]);
        assert!(deps.workspace_members.contains(&"bar".to_string()));
        assert!(deps.workspace_members.contains(&"qux".to_string()));
    }
}
