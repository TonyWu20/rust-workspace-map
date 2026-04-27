use crate::schema::WorkspaceMap;
use std::io::Write;

/// Serialize the workspace map to a JSON string with 2-space indentation.
pub fn render_json(map: &WorkspaceMap) -> serde_json::Result<String> {
    serde_json::to_string_pretty(map)
}

/// Serialize the workspace map to the given writer.
pub fn render_to_writer(map: &WorkspaceMap, writer: impl Write) -> serde_json::Result<()> {
    serde_json::to_writer_pretty(writer, map)
}
