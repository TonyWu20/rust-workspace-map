# Deferred Items — Single-Crate Support Phase

Items flagged by strategic review as worth doing but out of scope for this phase.

1. **Proper TOML parsing in discovery functions**: Both `find_workspace_root` and `find_crate_root` use `.contains()` string matching to detect `[workspace]` / `[package]` sections, which can trigger on comments or string literals. The D1 enhancement (`MissingWorkspaceSection` fallback) mitigates the workspace case. For `[package]`, downstream `cargo_info::parse_cargo_toml` handles false positives gracefully. Consider using `toml::from_str` in both functions for section existence verification.

2. **`FileRead` error test coverage**: Neither `find_workspace_root` nor `find_crate_root` has an explicit unit test for the `FileRead` error path. Add a test using permission-denied or similar mechanism for both functions.

3. **`FileRead` on first Cargo.toml stops ancestor walk**: When a Cargo.toml exists but is unreadable, both functions return the error immediately rather than continuing to parent ancestors. Consider skipping unreadable Cargo.toml files (with a diagnostic) to continue the walk.

4. **Custom source paths (`[lib]`/`[[bin]]` `path` field)**: `resolve_crate_roots` only checks `src/lib.rs` and `src/main.rs`. Crates with custom source paths aren't detected. Pre-existing limitation.

5. **`workspace.root` as absolute path**: The `workspace.root` field uses the absolute `PathBuf` string. Consider making it a canonical path or offering a `--relative` flag for portable output. Pre-existing pattern, not introduced by this PR.
