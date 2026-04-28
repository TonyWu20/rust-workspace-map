# Phase 0.2 — Task Checklist

Generated: 2026-04-28

---

## TASK-1: Add ErrorSeverity enum and ErrorContext struct to schema.rs

Module wiring check:
- pub mod in parent: Yes — `pub mod schema;` declared in `lib.rs` line 17
- pub use re-export: No — neither type is re-exported from `lib.rs` (not needed, they are internal to the schema module)
- Consumer updates co-located: Yes — `schema.rs` itself contains the types

Known failure mode check:
- Missing pub mod risk: Low — `schema.rs` already has `pub mod` in `lib.rs`
- Missing pub use risk: Low — consumers reference `crate::schema::ErrorSeverity` directly
- Stale import risk: Low — no existing code imports these types yet

Before-block check:
- Grep confirmed: Yes — the before-block text ("// -- Internal types") exists at `schema.rs` line 281
- Acceptance commands present: Yes — `cargo check -p rust-workspace-map`

Depends on: None

Notes: The insertion point is clean — placed before `pub struct FileInfo`. `ErrorContext` uses `bon::Builder` and `serde::Serialize` which are already imported indirectly via `schema.rs` derives.

---

## TASK-2: Add MissingWorkspaceSection variant to Error enum in schema.rs

Module wiring check:
- pub mod in parent: Yes — `pub mod schema;` declared in `lib.rs` line 17
- pub use re-export: No — new variant, no re-export needed
- Consumer updates co-located: Yes — only `schema.rs` changes

Known failure mode check:
- Missing pub mod risk: Low
- Missing pub use risk: Low
- Stale import risk: Low — no existing code references this variant

Before-block check:
- Grep confirmed: Yes — before-block ("GlobPattern variant + closing brace") exists at `schema.rs` lines 32-36
- Acceptance commands present: Yes — `cargo check -p rust-workspace-map`

Depends on: TASK-1 (not for wiring; `MissingWorkspaceSection` error message references no new types)

Notes: Straightforward variant addition. The error message string "workspace Cargo.toml is missing the [workspace] section" is clear and consistent with other error messages in this enum.

---

## TASK-3: Expand ErrorEntry struct with severity, kind, context, and cause fields

Module wiring check:
- pub mod in parent: Yes — `pub mod schema;` declared in `lib.rs` line 17
- pub use re-export: No
- Consumer updates co-located: Yes — `schema.rs` changes only

Known failure mode check:
- Missing pub mod risk: Low
- Missing pub use risk: Low
- Stale import risk: Low

Before-block check:
- Grep confirmed: Yes — before-block ("ErrorEntry struct with 3 fields") exists at `schema.rs` lines 302-309
- Acceptance commands present: Yes — `cargo check -p rust-workspace-map`

Depends on: TASK-1 — **BLOCKED without TASK-1** because the new `kind` field references `ErrorSeverity` (defined in TASK-1) and `context` field references `ErrorContext` (also defined in TASK-1)

Notes: The new `kind: String` field has no `skip_serializing_if` annotation, meaning it will always be serialized (even as empty string). This is intentional — callers are expected to always set it. Both `context` and `cause` have proper `skip_serializing_if` annotations. The `#[builder(default)]` on `line: usize` means the default is `0` — this is fine.

---

## TASK-4: Change parse_file to return ParsedFile with optional parse errors instead of Result

Module wiring check:
- pub mod in parent: Yes — `pub mod file_parser;` declared in `lib.rs` line 14
- pub use re-export: No — `ParsedFile` and `SynParseError` are `pub` (no re-export needed; they are used internally)
- Consumer updates co-located: No — `module_tree.rs` and `lib.rs` need updates for the new return type (handled in TASK-5 and TASK-7)

Known failure mode check:
- Missing pub mod risk: Low — `file_parser.rs` already `pub mod`
- Missing pub use risk: Medium — `build_parse_error_entry` is NOT `pub` (no visibility qualifier). It is referenced as `crate::file_parser::build_parse_error_entry` in `module_tree.rs` changes. Since Rust default visibility for non-pub items in the same crate is `crate`-level, this WILL work. But the plan does not explicitly declare this visibility, which could confuse an executor.
- Stale import risk: High — `file_parser.rs` imports `ErrorEntry` and `ErrorSeverity` from `schema.rs` which are defined in TASK-1 and TASK-3. If TASK-1 and TASK-3 are not applied first, the imports will fail.

Before-block check:
- Grep confirmed: Yes — the full `parse_file` function block matches at `file_parser.rs` lines 1-41
- Acceptance commands present: Yes — `cargo check -p rust-workspace-map`

Depends on: TASK-1, TASK-3 — **BLOCKED without these** because the new code imports `ErrorEntry` and `ErrorSeverity` from `schema.rs`

Notes: `ParsedFile` and `SynParseError` are `pub` in `file_parser.rs` but not re-exported from `lib.rs`. This is fine for internal use. The `build_parse_error_entry` helper function is `pub(crate)` by Rust default (no `pub` keyword). `parse_file` now always succeeds (never returns an `Err`), which is the intended behavior per the plan.

---

## TASK-5: Refactor module_tree.rs: fix path fallbacks, add error collection, change build_module_tree return type

Module wiring check:
- pub mod in parent: Yes — `pub mod module_tree;` declared in `lib.rs` line 15
- pub use re-export: No
- Consumer updates co-located: No — `lib.rs::run()` needs to call the new `(Vec<ModuleInfo>, Vec<ErrorEntry>)` return type (handled in TASK-7)

Known failure mode check:
- Missing pub mod risk: Low
- Missing pub use risk: Low
- Stale import risk: Medium — `module_tree.rs` imports `FileInfo` and `SubmoduleDecl` from `schema.rs` and uses `file_parser::parse_file`. If TASK-4 is not applied first, the `file_parser::parse_file` call will not match the new `ParsedFile` return type.

Before-block check:
- Grep confirmed: Yes — all 9 before-blocks verified against `module_tree.rs` source
- Acceptance commands present: Yes — `cargo check -p rust-workspace-map`

Depends on: TASK-4 — **BLOCKED without TASK-4** because `build_module_tree` and `process_submodule` call `file_parser::parse_file()` which returns `ParsedFile` in TASK-4 but `Result<(...)>` in current source. Also, `module_tree.rs` references `crate::file_parser::build_parse_error_entry` which is only defined in TASK-4.

Notes: This is the most complex task with 9 sequential replace operations on a single file. The operations must be applied in order (or atomically merged) because later before-blocks reference code that is mutated by earlier before-blocks. The path fallback fix (`unwrap_or_else(|| Path::new("."))` -> `unwrap_or(file_path)` or `unwrap_or(crate_root)`) is conservative — using the file itself as fallback when parent() returns None is safer than the original `Path::new(".")` which could resolve to the wrong directory. The `process_module_info` function gains an `errors: &mut Vec<...>` parameter that threads errors through recursion. The `process_module_items` function calls `process_module_info` with `&mut Vec::new()` which starts a fresh error vector per inline module — this may lose cross-module error context.

---

## TASK-6: Update workspace.rs enumerate_members to return MissingWorkspaceSection error

Module wiring check:
- pub mod in parent: Yes — `pub mod workspace;` declared in `lib.rs` line 18
- pub use re-export: No
- Consumer updates co-located: No — `lib.rs::run()` may propagate this error (but the error already propagates via `?` since it returns `Result`)

Known failure mode check:
- Missing pub mod risk: Low
- Missing pub use risk: Low
- Stale import risk: Low — `workspace.rs` already imports `Error` from `schema.rs`; the `MissingWorkspaceSection` variant is added in TASK-2

Before-block check:
- Grep confirmed: Yes — all 3 before-blocks verified against `workspace.rs` source
- Acceptance commands present: Yes — `cargo check -p rust-workspace-map`

Depends on: TASK-2 — **BLOCKED without TASK-2** because `Error::MissingWorkspaceSection` variant is only defined in TASK-2. If applied first, `cargo check` will fail with "variant does not exist".

Notes: The first change introduces a `match parsed.get("workspace")` that explicitly checks for `None` and returns `Err(Error::MissingWorkspaceSection)`. The second and third changes are purely doc additions (````# Errors` sections) to `find_workspace_root` and `enumerate_members`.

---

## TASK-7: Refactor lib.rs run(): parallel error collection, error entry construction, remove unwrap_or_default

Module wiring check:
- pub mod in parent: N/A — this modifies `lib.rs` (the crate root, not a module file)
- pub use re-export: Changes imports to add `ErrorContext`, `ErrorSeverity` to the use block — these types come from `schema.rs` (TASK-1, TASK-3)
- Consumer updates co-located: N/A — `lib.rs` IS the consumer of all other modules

Known failure mode check:
- Missing pub mod risk: N/A
- Missing pub use risk: Low — imports added to the existing `use schema::{...}` block
- Stale import risk: High — the before-block is very large (the entire `par_iter` closure block). If any prior task (TASK-4, TASK-5) changes the source before TASK-7 is applied, the before-block will NOT match.

Before-block check:
- Grep confirmed: Yes — the large before-block matches the current `lib.rs` lines 39-113
- Acceptance commands present: Yes — `cargo check -p rust-workspace-map`

Depends on: TASK-4, TASK-5, TASK-6 — **BLOCKED without these**. Specifically:
  - TASK-4: `parse_file` return type changed to `ParsedFile`
  - TASK-5: `build_module_tree` return type changed to `(Vec<ModuleInfo>, Vec<ErrorEntry>)`
  - TASK-6: `enumerate_members` may return `Err(MissingWorkspaceSection)` — already handled via `?`

Notes: This is the most critical integration task. The entire `par_iter` block is replaced with a `map` that returns `(Option<CrateInfo>, Vec<ErrorEntry>)` tuples, which are then collected and processed in a post-loop. The `errors: Vec<ErrorEntry>` variable is renamed to `crate_errors` and wired into the final `WorkspaceMap`. The `unwrap_or_default()` on line 79 is removed in favor of destructuring the `(modules, errors)` tuple from `build_module_tree`. If TASK-5 is not applied first, the `build_module_tree` call with the new return type will not compile.

---

## TASK-8: Remove all crate-level clippy allow attributes and fix individual lint violations

Module wiring check:
- pub mod in parent: N/A — modifies `lib.rs`, `file_parser.rs`, `module_tree.rs`, `render.rs`, `cross_refs.rs`
- pub use re-export: N/A — no new exports
- Consumer updates co-located: N/A — purely cosmetic/lint fixes

Known failure mode check:
- Missing pub mod risk: N/A
- Missing pub use risk: N/A
- Stale import risk: High — 11 replace operations across 5 files. Any prior task that modifies these files before TASK-8 will cause before-block mismatches.

Before-block check:
- Grep confirmed: Yes — all before-blocks verified against current source. Note: some "before" and "after" blocks are identical (e.g., `lib.rs` `workspace_name` block at line 959 and `relativize_path` ending at line 1073). These appear to be no-op placeholders. The actual changes are: removing crate-level `#![allow(...)]` lines, adding `#[must_use]` to extractors, and adding `# Errors` doc sections.
- Acceptance commands present: Yes — `cargo clippy -p rust-workspace-map -- -D warnings`

Depends on: TASK-7 — **Should follow TASK-7** because TASK-7 adds `# Errors` doc section to `run()` which is referenced in TASK-8's before/after. Also, TASK-8's clippy fixes should be applied after the structural changes from TASK-7 so clippy can validate the new code.

Notes: Some before/after blocks are identical (no-op). The `workspace_name` block and `relativize_path` block appear to be no-op placeholders — the actual change is the `#![warn(clippy::pedantic)]` block replacement. The `#[must_use]` annotations are added to all public extraction functions and the `resolve_module_path` function.

---

## TASK-9: Add unit tests for file_parser, module_tree, workspace, cargo_info, cross_refs, and render

Module wiring check:
- pub mod in parent: N/A — tests are `#[cfg(test)]` modules appended to existing source files
- pub use re-export: N/A
- Consumer updates co-located: N/A

Known failure mode check:
- Missing pub mod risk: Low
- Missing pub use risk: Low
- Stale import risk: Medium — tests reference `ErrorSeverity` (from TASK-1), `ParsedFile` (from TASK-4), `SynParseError` (from TASK-4), `ErrorContext::builder()` (from TASK-1), and `ErrorContext::builder().module_path(...)` (from TASK-1). These must exist in the compilation unit.

Before-block check:
- Grep confirmed: Yes — all 6 before-blocks verified against current source files
- Acceptance commands present: Yes — `cargo test -p rust-workspace-map`

Depends on: TASK-1, TASK-4 — **BLOCKED without these** because tests import `ErrorSeverity` and reference `ParsedFile`/`SynParseError` types. Also depends on TASK-5 indirectly because `build_module_tree` return type changes are tested.

Notes: Tests use `tempfile` crate which is NOT declared in `Cargo.toml`. **Missing `[dev-dependencies]` section with `tempfile = "..."` must be added before this task can compile.** The `parse_source` helper function writes temp files, which is correct for unit tests. The `build_module_tree_returns_empty_for_nonexistent` test expects `modules.is_empty()` and `!errors.is_empty()` — this is a reasonable expectation for a non-existent file (parse error from `std::fs::read_to_string`).

---

## TASK-10: Expand integration tests from 3 to 10 covering error cases, workspace patterns, and output

Module wiring check:
- pub mod in parent: N/A — integration test file
- pub use re-export: N/A
- Consumer updates co-located: N/A

Known failure mode check:
- Missing pub mod risk: N/A
- Missing pub use risk: N/A
- Stale import risk: Medium — depends on all structural changes (TASK-4 through TASK-8) being applied because the binary behavior changes (new error entries, new error kinds like `syn_parse_error`, `orphaned_module`, `toml_parse_error`, `missing_crate_roots`).

Before-block check:
- Grep confirmed: Yes — the before-block (the `test_missing_path_exits_nonzero` function end) matches at `integration_test.rs` lines 116-128
- Acceptance commands present: Yes — `cargo test -p rust-workspace-map --test integration_test`

Depends on: TASK-4, TASK-5, TASK-6, TASK-7 — **BLOCKED without these** because integration tests assert on error entry structure (`severity`, `kind` fields) and module tree behavior that only exist after the structural changes.

Notes: Tests use `tempfile` crate which is NOT declared in `Cargo.toml`. **Same missing `[dev-dependencies]` issue as TASK-9.** The new tests cover: parse failure error entries, missing workspace section, glob member patterns, workspace exclude, deeply nested modules (3+ levels), re-export chains, and output via `-o` flag. The `test_output_via_flag` test references `tests/fixtures/sample-workspace` which exists. Helper functions (`run_binary`, `parse_output`, `write_cargo_toml`, `setup_crate`) are defined inline in the after-block.

---

# Dependency Graph Summary

```
TASK-1 ──┐
TASK-2 ──┤
TASK-3 ──┤         TASK-6 ──┐
  │        │                  │
  └──► TASK-4 ──► TASK-5 ────┤
                          ▼
                       TASK-7 ──► TASK-8
                          │
                          ▼
                       TASK-9
                          │
                          ▼
                       TASK-10
```

Dependency notes:
- TASK-3 depends on TASK-1 because `ErrorEntry` new fields reference `ErrorSeverity` and `ErrorContext` types.
- TASK-4 depends on TASK-1, TASK-3 because it imports `ErrorEntry` and `ErrorSeverity`.
- TASK-5 depends on TASK-4 because it calls `file_parser::parse_file()` (new return type) and `crate::file_parser::build_parse_error_entry` (new function).
- TASK-6 depends on TASK-2 because it returns `Error::MissingWorkspaceSection` (new variant).
- TASK-7 depends on TASK-4, TASK-5, TASK-6 because it calls `parse_file()` (new type), `build_module_tree()` (new return type), and `enumerate_members()` (new error variant).
- TASK-8 should follow TASK-7 so clippy validates the new code.
- TASK-9 depends on TASK-1, TASK-4 because tests reference those new types.
- TASK-10 depends on TASK-4 through TASK-7 because integration tests assert on new error kinds and module tree structure.

# Overall Assessment

Ready to Implement: Yes, but with the following action items:

1. **Add `[dev-dependencies]` section** with `tempfile = "*"` (or a specific version). This is required for TASK-9 and TASK-10 tests to compile.

2. **Execute order matters**: Tasks must be executed in dependency order (TASK-1/2/3 first, then 4/6, then 5/7, then 8/9, then 10). Parallel execution of independent tasks (e.g., TASK-1 + TASK-2, or TASK-4 + TASK-6 after their deps) is possible but should be done carefully.

3. **High-risk transition points**:
   - TASK-4 -> TASK-5: `parse_file` return type changes from `Result<(...)>` to `ParsedFile`
   - TASK-5 -> TASK-7: `build_module_tree` return type changes from `Result<Vec<ModuleInfo>>` to `(Vec<ModuleInfo>, Vec<ErrorEntry>)`
   - TASK-7: Large single-replace in `lib.rs` that touches the entire parallel processing pipeline

4. **Before-block stability**: TASK-7's before-block is a large multi-line block that is the most fragile. If any prior task modifies `lib.rs::run()` before TASK-7, the before-block will not match.

Wiring issues flagged: 2 (missing `tempfile` dev-dependency, `build_parse_error_entry` visibility not explicitly declared).
Before-block unverified: 0 (all before-blocks match current source).
