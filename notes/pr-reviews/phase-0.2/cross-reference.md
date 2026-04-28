# Cross-Reference: Draft Review Issues for `phase-0.2`

## Review Framing Decision
- **Context snapshot:** Found — Phase Plan present (Goals 1-5, scope boundaries, design notes), Snapshot section says "No snapshot — using authoritative data from file-manifest.json"
- **Prior fix round:** None
- **Framing:** First review
- **Rating rationale:** Approve with Minor Issues. The PR correctly implements 4 of 5 goals with one minor plan-spec deviation (module_tree_error kind string never emitted). The deviation does not break functionality — errors are still captured and surfaced — but it diverges from the explicit kind string prescribed in Goal 1. The errors.clone() in module_tree is a design improvement opportunity, not a correctness issue.

## Per-Issue Cross-Reference

### Issue 1: [Defect] Plan-specified `module_tree_error` kind string never emitted
- **File:** `src/lib.rs`
- **Claim:** The phase plan (Goal 1) explicitly states: "Remove `.unwrap_or_default()` on `module_tree::build_module_tree` in `run()`, capture errors as `ErrorEntry` with `kind = "module_tree_error"`". The implementation (lines 83-86) destructures the tuple return `(m, e)` from `build_module_tree` and extends `collected_errors` with `e` — using the errors' own specific kinds (syn_parse_error, orphaned_module). The `module_tree_error` wrapper kind is never constructed.
- **Per-file analysis Checklist check:** "Change appears within plan scope? Yes" — the per-file analysis does not flag this specific deviation. The analysis correctly notes that error collection was improved, but does not verify the kind string against the plan specification.
- **Manifest fact check:** `src/lib.rs` — `added_functions: ["run"]`, `removed_functions: ["run"]`, `lines_added: 56, lines_removed: 41`. Confirms run() was replaced.
- **Context snapshot check:** Already resolved? No — no prior fix round. The plan document (PHASE_PLAN.md) contains the spec, and the codebase-state.md pre-implementation analysis also flagged this as a requirement.
- **Verdict:** CONSISTENT — The plan prescribes `kind = "module_tree_error"`. This string appears zero times in any Rust source file (confirmed via grep across entire repo). The implementation chose to surface errors with their specific kinds instead, which is arguably a better design but does not match the plan specification.
- **Action:** Included in draft-review.md as [Defect].

### Issue 2: [Improvement] `errors.clone()` in `process_module_info` induces O(depth * errors) clone cost
- **File:** `src/module_tree.rs`, line 252.
- **Claim:** `process_module_info` accepts `errors: &mut Vec<ErrorEntry>` AND returns `(Vec<ModuleInfo>, Vec<ErrorEntry>)`. The dual reporting forces `errors.clone()` at the return point. Since the function is called recursively, this incurs O(depth * errors) clone cost across the full module tree traversal. The clone is necessary for the current function signature, but the signature design (mutable borrow + return of same data) forces this inefficiency.
- **Per-file analysis Checklist check:** "Unnecessary clone/unwrap/expect? Yes — process_module_info calls errors.clone() at its return point. The errors are collected via &mut errors during the function body, then cloned for the return value. This is correct but incurs O(depth * errors) clone cost across recursive module tree traversal. The original errors vector (borrowed) goes out of scope in the caller after the return, so the clone is necessary for the current function signature. Consider returning (Vec<ModuleInfo>, Vec<ErrorEntry>) without the &mut parameter in a future refactor."
- **Manifest fact check:** `src/module_tree.rs` — `lines_added: 146, lines_removed: 59`, `added_functions: ["build_module_tree", ...]`, `removed_functions: ["build_module_tree"]`. Confirms significant refactoring of this module.
- **Context snapshot check:** Already resolved? No — no prior fix round addressed this.
- **Verdict:** CONSISTENT — The per-file analysis explicitly flags the clone, acknowledges it's necessary for the current signature, and suggests a future refactor. This is a design improvement opportunity, not a bug.
- **Action:** Included in draft-review.md as [Improvement].

### Issue 3: [Dropped] Redundant `use std::io::Write;` in integration_test.rs
- **File:** `tests/integration_test.rs`
- **Claim:** The per-file analysis states there is a file-level `use std::io::Write;` and a duplicate inside `write_cargo_toml` function body.
- **Per-file analysis Checklist check:** "Dead code or unused imports? Yes — use std::io::Write; is imported at file level but also duplicated inside the write_cargo_toml function body."
- **Manifest fact check:** `tests/integration_test.rs` — `added_imports: ["std::io::Write"]`. The manifest records the import as added.
- **Context snapshot check:** Not applicable.
- **Verdict:** CONTRADICTION → DROPPED. Ground-truth verification of the actual file shows there is **no file-level** `use std::io::Write;` — the only `use std::io::Write;` appears at line 143, scoped inside the `write_cargo_toml` function body. There is no duplicate. The per-file analysis's claim of a redundant import is incorrect.

### Issue 4: [Dropped] Remaining `unwrap_or_default()` calls in non-path-fallback contexts
- **File:** `src/workspace.rs` (lines 57, 69, 99), `src/file_parser.rs` (lines 213, 404, 528), `src/lib.rs` (lines 100, 138)
- **Claim:** Several `unwrap_or_default()` calls remain in the codebase after Goal 3 (Path Safety Fixes).
- **Per-file analysis Checklist check:** None of these files have "Unnecessary clone/unwrap/expect? Yes" for production code related to these calls. For `lib.rs`, the checklist says "No — no clone/unwrap/expect in production code." The remaining unwrap_or_default calls are on Option<Vec<T>> or Option<String> conversions, not path fallbacks.
- **Manifest fact check:** Files confirmed modified.
- **Context snapshot check:** The plan's Goal 3 targets 4 specific path fallback fixes. All 4 are confirmed fixed via ground-truth verification (no `Path::new(".")` fallbacks remain, MissingWorkspaceSection error is emitted).
- **Verdict:** Plan scope honored → DROPPED. The remaining `unwrap_or_default()` calls are on different types (Vec defaults, String defaults) and are legitimate uses, not the path-fallback bugs targeted by the plan.

### Issue 5: [Dropped] Duplicated `write_cargo_toml` helper across test modules
- **File:** `src/cargo_info.rs`, `src/workspace.rs`, `tests/integration_test.rs`
- **Claim:** Three test modules contain identically-named `write_cargo_toml` helper functions.
- **Per-file analysis Checklist check:** For `src/cargo_info.rs`: "Notes: Test helper write_cargo_toml is duplicated identically in workspace.rs tests and integration_test.rs. This is acceptable code duplication for test isolation." For `src/workspace.rs`: not flagged as issue. For `tests/integration_test.rs`: not flagged as issue.
- **Manifest fact check:** All three files confirm the helper function exists.
- **Context snapshot check:** Not applicable.
- **Verdict:** Intentional design choice → DROPPED. Test isolation is a valid reason for helper duplication. Each test module is self-contained.

### Issue 6: [Dropped] Missing direct unit tests for `ErrorSeverity` and `ErrorContext`
- **File:** `src/schema.rs`
- **Claim:** `ErrorSeverity` enum and `ErrorContext` struct have no dedicated unit tests.
- **Per-file analysis Checklist check:** "New public API: tests present? Not directly — ErrorSeverity and ErrorContext are consumed by other modules' tests (file_parser, module_tree, lib.rs). Integration tests exercise the full serialized output including these types."
- **Manifest fact check:** `src/schema.rs` — `added_functions: ["ErrorSeverity", "ErrorContext"]`, `added_imports: []`. No test functions listed for this file.
- **Context snapshot check:** Not applicable.
- **Verdict:** Acceptable design → DROPPED. These are pure data types (Copy + Clone enum, Builder struct) with no behavior to test. They are exercised through integration tests and consumer tests across file_parser, module_tree, and lib.rs.
