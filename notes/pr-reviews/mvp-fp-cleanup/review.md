# Review: mvp-fp-cleanup — Eliminate DeadReExport False Positives

**Index**: notes/directions/mvp-fp-cleanup/directions-index.json
**Reviewed**: 2026-05-01
**Branch**: mvp-dev

## Summary

**PASS — approved for merge.** All 5 tasks (G1, G2, G3, G4, README) are implemented. Two minor issues found, neither blocking. Fix directions provided for optional cleanup.

## Per-Task Results

### TASK-G1: Prefix-aware external-crate pre-check
- **Status**: ⚠ Minor Issue
- **Diff validation**: `is_external_crate_re_export` exists with correct signature. Three-case dispatch (self::/super:: → false, crate:: → false, bare path → check). Top-level module detection prevents bare-path pitfall. All 5 tests present (1 TDD + 4 additional). Existing cross-crate check preserved.
- **Strategic review**: Code-comment mismatch at the self-reference guard. When `first_seg == crate_info.name`, comment says "fall through to return false" but code falls through to `return true`. False negative for dead self-referencing re-exports. Near-zero real-world impact (Rust compiler rejects self-imports of non-existent types from same crate).

### TASK-G2: derive_attrs field and prefix-decomposition heuristic
- **Status**: ✓ Passed (minor issue)
- **Diff validation**: `SymbolEntry.derive_attrs` field with `#[builder(default)]` and `#[serde(skip_serializing_if = "Vec::is_empty")]`. Plumbing in `indexes.rs` fold closure. `is_derive_companion` with correct decreasing prefix iteration and same-module scoping. Correct wiring in `check_dead_reexports` `!symbols.contains_key` branch. All 4 TDD test cases plus 4 edge-case tests present.
- **Strategic review**: Purely structural heuristic — zero hardcoded derive names, zero suffix lists. Longest-prefix-first minimizes false suppression. Missing edge case test for empty `module_path` parameter (procedural guidance only, no functional impact).

### TASK-G3: Fix determine_parent_file src/ prefix
- **Status**: ✓ Passed
- **Diff validation**: Parameter renamed `_crate_info` → `crate_info`. Uses `Path::new(&crate_info.root).parent()` — no hardcoded `"src/"`, no `.join("src")` anti-pattern. rsplit_once and format! macros preserved. No call site changes needed.
- **Strategic review**: Correct implementation. Handle-less edge case for root paths without a parent directory (not reachable in practice).

### TASK-G4: Deep orphan fixture + recursive test
- **Status**: ✓ Passed
- **Diff validation**: Fixture files correct (lib.rs with `pub mod sub;`, sub/mod.rs with `pub mod legal;` + `helper()`, no `pub mod deep;`, orphan at sub/deep/orphan.rs). Integration test checks exit code 2, both deep and shallow orphan detection.
- **Strategic review**: Correct fixture structure. Existing `test_validate_orphan_file_exits_2` regression-guarded.

### TASK-README: Document derive-companion heuristic
- **Status**: ✓ Passed
- **Diff validation**: Derive-companion heuristic and private-base-type limitation documented under "Validation Rules" section. Conservative recall tradeoff, zero hardcoded derive names, purely structural signal — all covered.
- **Strategic review**: Clear documentation. Tool's conservative posture explicitly stated.

## Fix Verification (2026-04-30)

All 3 fix tasks from `fix-directions.json` have been verified as correctly applied.

| Fix Task | Status | File | Line |
|---|---|---|---|
| FIX-G1-GUARD: Self-reference guard | ✓ Applied | src/validate.rs | 51 (`&& first_seg != crate_info.name`) |
| FIX-G1-TEST: Self-name test case | ✓ Applied | src/validate.rs | 468-493 (`test_is_external_crate_re_export_self_name`) |
| FIX-G2-TEST: Empty module_path test | ✓ Applied | src/validate.rs | 568-572 (`is_derive_companion("FooBuilder", "", &BTreeMap::new())`) |

**Acceptance**: `cargo test` — 70 passed, 0 failed. `cargo clippy -- -D warnings` — clean.

## Issues Found

### Issue 1: Self-reference guard code-comment mismatch (low)
- **File**: `src/validate.rs:48-57`
- **Description**: In `is_external_crate_re_export`, when `first_seg == crate_info.name`, the comment on line 52 says "fall through to return false (internal)" but code falls through to `return true` on line 57. Produces a false negative for `pub use <self_crate>::<Nonexistent>`.
- **Real-world impact**: Near zero — Rust compiler rejects self-imports of non-existent types in same crate.
- **Fix**: Add `if first_seg == crate_info.name { return false; }` before the final `return true`, or fix the comment.

### Issue 2: Missing self-crate-name test (low)
- **File**: `src/validate.rs` — test module
- **Description**: No test covers `first_seg == crate_info.name` scenario. Would have caught Issue 1 during TDD.
- **Fix**: Add test for `is_external_crate_re_export("mycrate::Something", ...)` where `crate_info.name == "mycrate"` and `crate_names` includes `"mycrate"`.

### Issue 3: Missing empty module_path test (cosmetic)
- **File**: `src/validate.rs` — test module
- **Description**: Procedural guidance for G2 specified `is_derive_companion("FooBuilder", "", &symbols) → false` test. Not present.
- **Fix**: Add the missing edge case test. No functional impact.

## Acceptance Criteria Verification

| Command | Expected | Actual |
|---|---|---|
| `cargo test` | All pass | 69 passed, 0 failed |
| `cargo clippy -- -D warnings` | Clean | Clean, 0 warnings |

## Deferred Items

See `deferred.md`.
