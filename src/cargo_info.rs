use crate::schema::{CrateType, DepInfo, Error, PackageInfo, Result};
use std::path::Path;

/// Parse a crate's `Cargo.toml` and return package metadata and dependency lists.
/// The returned `PackageInfo.crate_type` is set to `Lib` by default; the caller
/// overrides it based on `workspace::resolve_crate_roots`.
pub fn parse_cargo_toml(path: &Path) -> Result<(PackageInfo, DepInfo)> {
    let content = std::fs::read_to_string(path).map_err(|source| Error::FileRead {
        path: path.to_path_buf(),
        source,
    })?;

    let parsed: toml::Value = toml::from_str(&content).map_err(|source| Error::TomlParse {
        path: path.to_path_buf(),
        source,
    })?;

    let package = parsed
        .get("package")
        .map(|p| {
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
        })
        .unwrap_or_else(|| {
            PackageInfo::builder()
                .name("unknown".to_string())
                .version("0.0.0".to_string())
                .edition("2021".to_string())
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
                        .and_then(|v| v.as_bool())
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
