# Review: MVP Fix — Recursive orphan walk + test cleanup

**Directions**: `notes/pr-reviews/mvp/fix-directions.json`
**Reviewed**: 2026-04-30

## Summary

**PASSED** — both fix tasks are correctly implemented. All acceptance criteria pass (`cargo check`, `cargo test`, `cargo clippy -- -D warnings` are clean). No defects found that require a fix-directions.json.

## Per-Task Results

### Fix-1: Recursive orphan walk via walkdir
- **Status**: ✓ Passed
- **Diff validation**: All 11 requirements verified against implementation. Actual code is character-for-character identical to the replacement block in the fix directions (with one cosmetic clippy-driven improvement: `unwrap_or(path)` → `unwrap_or(&path)` via `map_unwrap_or` lint, in commit `0c1bfbb`).
- **Strategic review**: WalkDir integration is idiomatic. `filter_entry` predicate correctly skips hidden, bin/, and tests/ directories. Workspace-relative path handling via `strip_prefix` with `unwrap_or` fallback is safe. Edge cases (symlinks, non-UTF8 paths, empty src_dir, deeply nested dirs) are handled correctly.

### Fix-2: Remove spurious Cargo.toml write in test
- **Status**: ✓ Passed
- **Diff validation**: The `std::fs::write(src.join("Cargo.toml").parent()...)` line is confirmed absent from `test_orphaned_module_serde_regression`. No other test modifications were made by the fix commits.
- **Strategic review**: Test setup is clean — proper tempdir usage, correct Cargo.toml, straightforward assertions.

## Issues Found

**None.** Both implementations are correct and within scope.

## Non-Blocking Observations

1. **Pre-existing `determine_parent_file` bug for non-root workspace members** (src/validate.rs:111). The function hardcodes `strip_prefix("src/")` but workspace-relative paths may include a crate-name prefix (e.g., `crates/mylib/src/subdir/file.rs`). This produces a wrong parent hint for orphan files in subdirectories of non-root crates. Outside the scope of these fix directions — the parent hint was previously never exercised for subdirectory orphans because the old `read_dir` only walked one level deep. Flagged as deferred.

2. **WalkDir produces unsorted findings** — entries are in filesystem order. This is acceptable since diagnostics are non-structural output and existing tests only check for presence, not order.

## Deferred Items

See `deferred.md` for 4 items flagged for future phases.
