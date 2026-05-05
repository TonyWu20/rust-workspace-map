# Changelog

## 0.2.0 — 2026-05-06

### Added
- Single-crate project support: `rust-workspace-map` now auto-detects non-workspace
  (single-crate) projects and treats them as a workspace of one. No `[workspace]`
  section required. (Closes #4)
- `find_crate_root` discovery function in `workspace.rs` — walks ancestor
  directories looking for `Cargo.toml` with a `[package]` section.
- `CrateRootNotFound` variant to `schema::Error` for signaling single-crate
  discovery failure.
- D1 enhancement: fallback also triggers on `MissingWorkspaceSection` from
  `enumerate_members`, handling false-positive `[workspace]` in comments.
- Integration test `test_single_crate_module_tree` — verifies depth-2 submodule
  resolution in single-crate mode.
- Integration test `test_single_crate_without_workspace` — replaces the old
  `test_missing_workspace_section` to assert success for standalone crates.

### Fixed
- Silent error exits: all 4 `exit(1)` sites in `main.rs` now print descriptive
  error messages to stderr before exiting, using `{e}` for I/O errors and
  `{e:#}` for anyhow error chains.
