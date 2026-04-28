## Deferred Improvements

- Empty `errors` vector in `WorkspaceMap::run()` — the field is always empty because no code path populates it; wiring in error collection would improve debugging of misconfigured workspaces
- Silent error swallowing in `build_module_tree` — `unwrap_or_default()` masks parse failures, making it indistinguishable from crates with no public items
- Path fallback to `"."` in `module_tree.rs` — three `.unwrap_or_else(|| Path::new("."))` calls produce incorrect relative paths when `.parent()` returns `None`
- Path fallback to `""` in `workspace.rs` — defaulting to empty vectors/malformed sections silently includes zero members; falling back to `""` for file names bypasses exclude filters
- No unit tests for public API functions — 13 public functions across 5 modules lack unit tests; only integration tests cover the full pipeline
- **Standalone crate support** — the binary requires a Cargo workspace (`[workspace]` section) and rejects standalone crate directories with `WorkspaceRootNotFound`. It should also accept standalone crates by treating the crate's own directory as a virtual workspace root with a single member, falling back to the crate's `[package]` metadata for workspace-level fields.

### Pipeline integration

- **Fatal errors bypass JSON output** — when `[workspace]` section is missing or root is not found, the binary exits non-zero with no JSON output and error text on stderr. Pipeline agents that always expect parseable JSON must handle this as a special case. A `--errors-as-json` flag (or equivalent) that captures *all* output in JSON form including fatal errors would make it more robustly pipeable.
- **No output format version marker** — if the JSON schema evolves, pipeline agents consuming the output have no way to detect which format version they are looking at. A `"formatVersion": "1.0"` top-level field would allow agents to assert compatibility.
- **No `--quiet` mode** — non-fatal warnings (orphaned modules, missing crate roots, nonexistent members) are written to stderr and interleaved with any JSON output. A `--quiet` flag to suppress stderr noise would let agents capture clean JSON via stdout without mixing in diagnostic text.

### MCP server for agent-native access

Add a second binary target (`src/bin/mcp.rs`) that runs the existing library as an MCP stdio server. Instead of producing one monolithic JSON blob, it exposes granular tools (`list_crates`, `get_crate_info`, `search_symbol`, `get_module_tree`, `resolve_import`, `refresh_workspace`) that return only the slice of data the agent needs — saving context tokens compared to parsing the full workspace map. The `lib.rs` already exports the core functions and serde-annotated schema types, so the MCP server is a thin protocol layer on top of existing logic.

**Key design questions to resolve in the next phase:**
- Separate binary vs `--mcp` flag on the existing binary
- Cached workspace scan vs re-scan on every tool call
- Which tools to expose (the six above, or a different split)
- Error representation: MCP protocol error codes vs structured error entries

## Known Failure Modes

None found. The only `fix-plan.toml` (`notes/pr-reviews/phase-0.1/fix-plan.toml`) contained no fix tasks — the PR was reviewed and approved with all items deferred to future phases.
