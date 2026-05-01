# Deferred Improvements — MVP Fix Review

Items flagged by strategic review as worth doing but out of scope for the current fix direction.

## 1. Fix `determine_parent_file` path calculation

**File**: `src/validate.rs:105-124`
**Issue**: The function hardcodes `strip_prefix("src/")` to isolate the directory portion of an orphan file path. This fails for workspace members not at the workspace root (e.g., `crates/mylib/src/subdir/file.rs` would strip to `crates/mylib/src/subdir/file.rs`, then produce `mylib/src/crates/mylib/src/subdir/mod.rs`). The `_crate_info` parameter is received but unused, suggesting this was anticipated.
**Fix**: Derive the crate-relative `src/` prefix from `crate_info.root` rather than hardcoding `"src/"`.
**Priority**: Low (only affects fix hint, not detection). Medium if users report confusing hints.

## 2. Add integration test for recursive orphan detection

**Issue**: The `bad-orphan` fixture only has `src/forgotten.rs` at depth 1. The recursive walk is the headline feature of this fix but has no targeted test for subdirectory orphan detection (e.g., `src/sub/deep/orphan.rs`).
**Priority**: Medium — without this test, a regression back to non-recursive behavior would not be caught.

## 3. Make `pub mod` suggestion context-aware

**File**: `src/validate.rs:93`
**Issue**: The fix hint always says `pub mod {stem};` but binary crate modules and private internal modules should use `mod {stem};` without `pub`.
**Fix**: Check whether the crate is a library (use `pub mod`) or binary (use `mod`).
**Priority**: Low (cosmetic UX).

## 4. Document `src/tests/` directory pruning trade-off

**Issue**: The `filter_entry` predicate prunes all `tests/` directories. While `/tests/` at the crate root holds integration tests (already outside the walk), `src/tests/` can be a legitimate module directory. Files inside a declared `src/tests/` subtree that are themselves undeclared would not be detected.
**Priority**: Low — acceptable trade-off (test-only code is not public API). Worth documenting if users report false negatives.
