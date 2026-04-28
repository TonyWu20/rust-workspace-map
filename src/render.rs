use crate::schema::WorkspaceMap;
use std::io::Write;

/// Serialize the workspace map to a JSON string with 2-space indentation.
#[must_use]
pub fn render_json(map: &WorkspaceMap) -> serde_json::Result<String> {
    serde_json::to_string_pretty(map)
}

/// Serialize the workspace map to the given writer.
pub fn render_to_writer(map: &WorkspaceMap, writer: impl Write) -> serde_json::Result<()> {
    serde_json::to_writer_pretty(writer, map)
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{
        CrateInfo, CrateType, CrossReferences, DepInfo, ModuleInfo, PackageInfo,
        WorkspaceInfo, WorkspaceMap,
    };

    fn make_minimal_map() -> WorkspaceMap {
        WorkspaceMap::builder()
            .workspace(WorkspaceInfo::builder()
                .root(".".to_string())
                .workspace_name("test".to_string())
                .build())
            .crates(vec![
                CrateInfo::builder()
                    .name("test-crate".to_string())
                    .root(".".to_string())
                    .package(PackageInfo::builder()
                        .name("test-crate".to_string())
                        .version("0.1.0".to_string())
                        .edition("2021".to_string())
                        .crate_type(CrateType::Lib)
                        .build())
                    .modules(vec![ModuleInfo::builder()
                        .path("".to_string())
                        .file("src/lib.rs".to_string())
                        .visibility("pub".to_string())
                        .build()])
                    .deps(DepInfo::default())
                    .build(),
            ])
            .cross_references(CrossReferences::default())
            .workspace_root(std::path::PathBuf::from("."))
            .build()
    }

    #[test]
    fn render_json_produces_valid_json() {
        let map = make_minimal_map();
        let json = render_json(&map).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["workspace"]["root"], ".");
        assert_eq!(parsed["crates"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn render_json_skips_empty_errors() {
        let map = make_minimal_map();
        let json = render_json(&map).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        // errors field should be absent (skip_serializing_if)
        assert!(parsed.get("errors").is_none());
    }

    #[test]
    fn render_to_writer_matches_render_json() {
        let map = make_minimal_map();
        let json = render_json(&map).unwrap();

        let mut buf = Vec::new();
        render_to_writer(&map, &mut buf).unwrap();
        let from_writer = String::from_utf8(buf).unwrap();

        assert_eq!(json, from_writer);
    }
}
