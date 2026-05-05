# Review: Single-Crate Support + Fix Silent Error Handling

**Index**: notes/directions/single-crate-support/directions-index.json
**Reviewed**: 2026-05-06

## Summary

**Passed** — all tasks fully implemented as directed. 162 lines of production + test code across 5 source files. 73 tests pass (54 unit + 19 integration). Clippy is silent. The "do NOT touch" modules have zero diff bytes.

## Per-Task Results

### TASK-single-crate-core-01: Add CrateRootNotFound variant
- **Status**: ✓ Passed
- **Diff validation**: Variant `CrateRootNotFound(PathBuf)` added after `WorkspaceRootNotFound` at schema.rs:57-59. Error message, type, and placement match the guidance exactly.
- **Strategic review**: Properly placed in the Error enum with correct thiserror attribute and PathBuf payload matching the sibling variant pattern.

### TASK-single-crate-core-02: Add find_crate_root discovery function + 2 unit tests
- **Status**: ✓ Passed
- **Diff validation**: Function `find_crate_root` at workspace.rs:34-49 structurally mirrors `find_workspace_root`. Both unit tests match `tdd_interface.test_code` verbatim. Test for happy path (finds `[package]` walking from nested subdir) and error path (empty dir returns `CrateRootNotFound`).
- **Strategic review**: TDD interface satisfied. Signature matches. Expected behavior implemented correctly (ancestor walk, found, not-found, FileRead error mapping). Test coverage adequate for this phase.

### TASK-pipeline-fallback: Add fallback logic in build_map
- **Status**: ✓ Passed
- **Diff validation**: `Error` import added to `use schema` block. Two-line workspace discovery replaced with match expression handling both `WorkspaceRootNotFound` and `MissingWorkspaceSection` fallback triggers. Both fallback paths call `find_crate_root` and produce `(crate_dir.clone(), vec![crate_dir])`. Other errors propagate as fatal.
- **Strategic review**: D1 enhancement (catch `MissingWorkspaceSection` from `enumerate_members` as fallback trigger) incorporated correctly. Orchestration stays in `build_map` — workspace.rs remains pure.

### TASK-silent-exit-fix: Fix 4 silent error exits in main.rs
- **Status**: ✓ Passed
- **Diff validation**: All 4 sites fixed with correct message format (`{e}` for I/O errors, `{e:#}` for anyhow chains). Validation exit (exit 2) and lookup serialization errors left untouched. `cargo check` and `cargo clippy -- -D warnings` pass.
- **Strategic review**: Backward compatible — workspace mode tests (including `test_deeply_nested_modules`) continue to pass.

### TASK-test-existing-update: Rename and invert test_missing_workspace_section
- **Status**: ✓ Passed
- **Diff validation**: Test renamed to `test_single_crate_without_workspace`. Uses `setup_crate` two-param helper. Asserts exit 0, parses JSON, verifies 1 crate named "standalone" with a struct "Standalone" in public items.
- **Strategic review**: Correctly inverts the old test — what was an error condition is now a supported scenario.

### TASK-test-single-crate-module-tree: Add depth >= 2 module tree integration test
- **Status**: ✓ Passed
- **Diff validation**: Creates crate with `pub mod helpers` → `src/helpers/mod.rs: pub mod sub` → `src/helpers/sub.rs: pub fn assist()`. Verifies 3 module paths and the depth-2 module's public item `assist`.
- **Strategic review**: Exercises depth-2 submodule resolution in pure single-crate mode (no `[workspace]` section), matching the known pitfalls requirement.

## Issues Found

None.

## Deferred Items

See `deferred.md` for items flagged for future phases.
