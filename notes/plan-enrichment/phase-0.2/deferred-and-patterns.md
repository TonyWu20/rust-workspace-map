## Deferred Improvements

- Empty `errors` vector in `WorkspaceMap::run()` — the field is always empty because no code path populates it; wiring in error collection would improve debugging of misconfigured workspaces
- Silent error swallowing in `build_module_tree` — `unwrap_or_default()` masks parse failures, making it indistinguishable from crates with no public items
- Path fallback to `"."` in `module_tree.rs` — three `.unwrap_or_else(|| Path::new("."))` calls produce incorrect relative paths when `.parent()` returns `None`
- Path fallback to `""` in `workspace.rs` — defaulting to empty vectors/malformed sections silently includes zero members; falling back to `""` for file names bypasses exclude filters
- No unit tests for public API functions — 13 public functions across 5 modules lack unit tests; only integration tests cover the full pipeline

## Known Failure Modes

None found. The only `fix-plan.toml` (`notes/pr-reviews/phase-0.1/fix-plan.toml`) contained no fix tasks — the PR was reviewed and approved with all items deferred to future phases.
