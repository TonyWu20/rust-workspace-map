# Phase 0.2: Hardening for Trustworthiness

**Date:** 2026-04-28
**Status:** Reviewed

## Goals

### Goal 1 — Error Visibility Pipeline (Medium)

Wire error collection end-to-end so every non-fatal error during a run appears as a structured `ErrorEntry` in the output JSON. No errors are silently swallowed.

The `ErrorEntry` type (already defined in schema but never populated) gains rich context fields:

```rust
pub struct ErrorEntry {
    pub file: String,
    pub line: usize,
    pub message: String,
    pub severity: ErrorSeverity,        // new: "error" | "warning"
    pub kind: String,                   // new: machine-readable tag, e.g. "syn_parse_error"
    pub context: Option<ErrorContext>,   // new: structured context (see Design Notes)
    pub cause: Option<String>,          // new: underlying error detail
}
```

Specific changes:
- Change `parse_file()` to propagate `SynParse` errors as `ErrorEntry` with severity `error`, rather than returning empty results. This applies to **all** files that are parsed — including inline `#[cfg(test)]` module bodies and external `#[cfg(test)]` module files. External `#[cfg(test)]` modules are parsed for error reporting purposes but their public items are not extracted (they are test-only by nature). No parse failure is silently swallowed, regardless of file role.
- Remove `.unwrap_or_default()` on `module_tree::build_module_tree` in `run()`, capture errors as `ErrorEntry` with `kind = "module_tree_error"`
- Convert `cargo_info::parse_cargo_toml` failures (currently `eprintln!` + `return None`) to `ErrorEntry` with `kind = "toml_parse_error"`
- Convert `workspace::resolve_crate_roots` empty results (currently `eprintln!` + `return None`) to `ErrorEntry` with `kind = "missing_crate_roots"`
- Collect `ErrorEntry` values from parallel crate processing (use `Mutex<Vec<ErrorEntry>>` or `rayon::collect` into a shared vec)
- Emit `ErrorEntry` for orphaned modules (currently `eprintln!` only) with `kind = "orphaned_module"`
- Emit `ErrorEntry` for missing workspace members via `Error::MemberNotFound` (already defined but never constructed), replacing the `eprintln!` at `workspace.rs:76`
- Add `ErrorSeverity` enum to schema (`Error`, `Warning`)
- Add `ErrorContext` struct with `crate_name`, `module_path`, `line`, `snippet` fields

**Why now:** Without this, an LLM agent consuming the JSON cannot distinguish "no public API" from "parsing failed." This is the highest-leverage reliability change.

### Goal 2 — Unit Test Suite (Large)

Add `#[cfg(test)]` modules with comprehensive unit tests for all public functions across the 5 core modules. Target >=80% line coverage.

Modules and functions to cover:
- `workspace.rs`: `find_workspace_root`, `enumerate_members`, `resolve_crate_roots`
- `cargo_info.rs`: `parse_cargo_toml`
- `file_parser.rs`: `parse_file`, `extract_public_items`, `extract_imports`, `extract_re_exports`, `extract_submodules`, `extract_impls`
- `module_tree.rs`: `resolve_module_path`, `build_module_tree`
- `cross_refs.rs`: `compute`
- `render.rs`: `render_json`, `render_to_writer`

Parser/extractor tests use inline Rust strings. Filesystem-dependent tests use `tempfile::tempdir()`.

**Why now:** Without unit tests, every change risks silent regressions caught only by slow integration tests. Unit tests also document each function's contract.

### Goal 3 — Path Safety Fixes (Small)

Fix the four path fallback bugs that produce silently incorrect relative paths:

1. `module_tree.rs`: `crate_root.parent().unwrap_or_else(|| Path::new("."))` — replace with explicit handling
2. `module_tree.rs`: same pattern in `process_submodule` for `file_path.parent()`
3. `module_tree.rs`: same pattern in `process_module_info` for `file_path.parent()`
4. `workspace.rs`: `enumerate_members` currently defaults to an empty member list when the `[workspace]` section or `members` key is absent. Change it to return `Err(Error::MissingWorkspaceSection)` (a new variant), which the caller in `run()` converts to an `ErrorEntry` with `kind = "missing_workspace_section"`.

**Why now:** These are independently fixable bugs that produce incorrect output in edge cases.

### Goal 4 — Integration Test Expansion (Medium)

Expand from 3 to 8+ integration tests covering error paths, edge cases, and complex workspace structures:

- Parse failure workspace → verify `ErrorEntry` with severity appears in JSON
- Missing workspace section → verify appropriate error
- Glob member patterns → verify all matching crates discovered
- Workspace with `exclude` → verify excluded crate absent
- Deeply nested modules (3+ levels) → verify correct paths and hierarchy
- Re-export chains → verify correct re-export tracking
- Output via `-o` flag → verify file content matches stdout content

**Why now:** After Goals 1-3, error behavior and edge cases need explicit regression coverage.

### Goal 5 — Remove All Clippy Suppressions (Small)

**No `#![allow(clippy::*)]` remains in the codebase.** Every suppression is removed:

| Suppression | Disposition |
|-------------|-------------|
| `missing_errors_doc` | Add `# Errors` doc sections to functions returning `Result` |
| `must_use_candidate` | Add `#[must_use]` to pure functions as clippy suggests |
| `doc_markdown` | Fix doc comments to backtick-code identifiers |
| `uninlined_format_args` | Inline all format arguments |
| `redundant_closure` | Replace closures with direct function references |
| `collapsible_if` | Merge nested `if` expressions |
| `needless_pass_by_value` | Change to `&` references where callers don't need ownership |
| `needless_borrow` | Remove unnecessary borrows |
| `redundant_closure_for_method_calls` | Use method directly instead of closure wrapper |

No crate-level suppressions are converted to per-item suppressions. If a lint fires on legitimate code, the code is changed, not silenced.

Also:
- Remove unused error variants (if any remain after Goal 1)
- Standardize error message style across `eprintln!` calls
- Verify `cargo clippy -- -D warnings` passes clean

**Why now:** 9 crate-level suppressions mask real code quality issues. Each fix is mechanical but individually important — they compound to produce cleaner, more idiomatic Rust.

## Scope Boundaries

**In scope:**
- Structured error collection with rich context for LLM consumption
- Comprehensive unit test suite (>=80% line coverage)
- Fix known path fallback bugs
- Expanded integration tests for error paths and edge cases
- Code quality cleanup (clippy, unused code)

**Out of scope:**
- New output formats (TOML, YAML, etc.)
- New extraction features (trait methods, const generics, associated types)
- Performance optimization beyond what's needed for correctness
- `cargo metadata` integration
- CLI changes (output flags, filtering, etc.)
- Schema-breaking changes to `WorkspaceMap` or `CrateInfo`

## Design Notes

### ErrorEntry context structure

The `ErrorContext` struct should contain fields that are meaningful for both humans and LLM agents:

```rust
pub struct ErrorContext {
    pub crate_name: Option<String>,
    pub module_path: Option<String>,
    /// Line number in the source file where the error occurred
    pub line: Option<usize>,
    /// A short source snippet near the error location (if available)
    pub snippet: Option<String>,
}
```

### ErrorSeverity semantics

| Severity | Meaning | Consumer guidance |
|----------|---------|-------------------|
| `error` | Data loss — a crate or module's information is incomplete | Output is partial, do not rely on missing data |
| `warning` | Run completed fully but something unusual happened | Output is complete but may need attention |

### Partial module tree preservation

When a submodule file fails to parse, the module tree builder collects an `ErrorEntry` with `kind = "syn_parse_error"` and continues processing sibling modules rather than aborting the entire crate. This ensures an LLM consumer sees all successfully parsed data even when some files have errors.

Implementation approach: `parse_file` returns errors alongside results rather than propagating via `?` on parse failures. `build_module_tree` similarly collects per-module errors into a `Vec<ErrorEntry>` returned alongside the module tree.

### Error kind taxonomy

The `kind` field on `ErrorEntry` uses the following initial set of machine-readable tags. Consumers should handle unknown `kind` values gracefully, as the set is extensible:

| `kind` value | Meaning |
|---|---|
| `toml_parse_error` | A crate's `Cargo.toml` failed to parse |
| `syn_parse_error` | A Rust source file failed to parse |
| `missing_crate_roots` | No `lib.rs` or `main.rs` found for a crate |
| `orphaned_module` | A `mod` declaration references a file that doesn't exist |
| `module_tree_error` | The module tree builder failed for a crate root |
| `missing_workspace_section` | The workspace `Cargo.toml` has no `[workspace]` section |
| `glob_pattern_error` | A glob pattern in `members` failed to evaluate |
| `member_not_found` | A workspace member path does not exist on disk |

### Error handling crates

The project already uses `thiserror` (for the library `Error` enum with `#[derive(Error)]`) and `anyhow` (for the binary's `Result` type with `.context()`). These are the standard, well-established error handling crates for Rust and no additional crates are needed. The `Error` enum in `schema.rs` gains `MissingWorkspaceSection` as a new variant; existing variants (`SynParse`, `MemberNotFound`, etc.) are retained and populated where they were previously unused.

### Sequence of execution

```
Goal 3 ──┐
         ├──→ Goal 1 → Goal 2 → Goal 4 → Goal 5
```

Goals 3 and 1 are independent and can run in parallel. Goal 1 must precede 2 and 4 (it changes function signatures). Goal 5 is last.

## Deferred Items Absorbed

All 5 deferred improvements from phase 0.1 are absorbed:

| Deferred Item | Absorbed By |
|---------------|-------------|
| Empty errors vector in `run()` | Goal 1 |
| Silent error swallowing in `build_module_tree` | Goal 1 |
| Path fallback to `"."` in `module_tree.rs` | Goal 3 |
| Path fallback to `""` in `workspace.rs` | Goals 1 & 3 |
| No unit tests for public API functions | Goal 2 |

## Open Questions

None resolved. See decisions above.

## Decisions

- `ErrorContext` includes `line: Option<usize>` for source location (resolves Open Question 1)
- `severity` field with `Error`/`Warning` variants is the approach (resolves Open Question 2 — no `recoverable` boolean)
- Every parse failure produces an `error`-severity `ErrorEntry`. With TDD principles, we care about test quality — silent failures in any file undermine trust in the output. (Resolves Open Question 3)
- **Partial results** — a single submodule parse failure collects an `ErrorEntry` and continues with siblings, rather than aborting the entire crate. This maximizes information for LLM consumers. (Resolves architectural gap from review)
- **External `#[cfg(test)]` modules** are parsed for error reporting but their public items are not extracted. This ensures parse failures in test files are visible without injecting test-only items into the API surface.
- **`kind` taxonomy** uses the initial 8-value set defined in Design Notes. Consumers handle unknown values gracefully.
- **Error handling crates** remain `thiserror` + `anyhow`. No additional error handling crates are introduced.
