# Per-File Analysis

## File: Cargo.lock

### Intent

Added ~286 lines of lock entries driven by the new `tempfile` dev-dependency and its transitive dependencies (rustix, bitflags, errno, fastrand, etc.), and upgraded `indexmap` to include `serde`/`serde_core` as dependencies.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — Cargo.lock is auto-generated]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable — not source code]
- Change appears within plan scope? [Yes — TASK-PREP adds tempfile dev-dependency]

### Notes

- `tempfile = "3"` resolved to version `3.27.0` with a full transitive dependency chain (fastrand, getrandom, once_cell, rustix, windows-sys).
- `hashbrown` now appears twice (0.15.5 and 0.17.0) — `wasmparser` pins the older version while the main deps use the newer one.
- `indexmap` gained `serde` and `serde_core` as dependencies, suggesting a version bump that added serde support.

---

## File: Cargo.toml

### Intent

Added `tempfile = "3"` under `[dev-dependencies]` for use in unit and integration tests (TASK-PREP).

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — manifest file]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable — manifest file]
- Change appears within plan scope? [Yes — TASK-PREP]

### Notes

- Minimal change: 3 lines added, 0 removed. Clean and focused.

---

## File: execution_reports/execution_fix-plan_20260428.md

### Intent

An execution report documenting the fix-plan run (TASK-1 failure, rest completed). It is an artifact of the execution pipeline, not source code.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — execution report]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable — execution report]
- Change appears within plan scope? [Yes — execution artifact from fix-plan]

### Notes

- Records `TASK-1` as the only failed task.
- 54 lines of metadata and summary of what ran.

---

## File: execution_reports/execution_phase-0.2_20260428.md

### Intent

An execution report documenting phase-0.2 execution outcomes — TASK-2 and TASK-3 completed; TASK-4 through TASK-10 failed. This is an artifact of the execution pipeline.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — execution report]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable — execution report]
- Change appears within plan scope? [Yes — execution artifact]

### Notes

- 288 lines. Lists all failed tasks with reasons.
- This file is read-only metadata, no code review needed.

---

## File: notes/plan-enrichment/phase-0.2/codebase-state.md

### Intent

A documentation file summarizing codebase state for the plan enrichment pipeline. Contains function signatures and module descriptions used by the enrichment process.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — documentation file]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable — documentation]
- Change appears within plan scope? [Yes — plan enrichment artifact]

### Notes

- 328 lines of structured documentation. No code review needed.

---

## File: notes/plan-enrichment/phase-0.2/deferred-and-patterns.md

### Intent

Documentation summarizing deferred items and coding patterns discovered during plan enrichment.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — documentation]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable — documentation]
- Change appears within plan scope? [Yes — plan enrichment artifact]

### Notes

- 28 lines. No code review needed.

---

## File: notes/plan-enrichment/phase-0.2/draft-elaboration.md

### Intent

A draft elaboration document produced by the plan enrichment pipeline, containing task-level breakdown of the phase plan.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — documentation]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable — documentation]
- Change appears within plan scope? [Yes — plan enrichment artifact]

### Notes

- 563 lines. No code review needed.

---

## File: notes/plan-enrichment/phase-0.2/gather-summary.md

### Intent

A summary of the gather phase of plan enrichment — what was collected from the codebase.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — documentation]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable — documentation]
- Change appears within plan scope? [Yes — plan enrichment artifact]

### Notes

- 27 lines. No code review needed.

---

## File: notes/plan-enrichment/phase-0.2/plan.approved.toml

### Intent

The approved and enriched implementation plan for phase-0.2, transformed from the high-level PHASE_PLAN.md into executable TOML tasks with explicit file changes, acceptance criteria, and dependencies.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — plan document]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable — plan document]
- Change appears within plan scope? [Yes — this is the plan itself]

### Notes

- 2168 lines. Contains all TASK-PREP through TASK-10 with detailed `[[tasks.*.changes]]` entries specifying file paths, before/after text, and acceptance criteria.
- This is the authoritative task definition for the execution pipeline.

---

## File: notes/plan-enrichment/phase-0.2/task-checklist.md

### Intent

A task checklist tracking completion status across all phase-0.2 tasks.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — documentation]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable — documentation]
- Change appears within plan scope? [Yes — plan enrichment artifact]

### Notes

- 274 lines of task status tracking. No code review needed.

---

## File: notes/pr-reviews/phase-0.2/context.md

### Intent

Metadata file providing PR review context: branch name, creation date, and task dependency graph.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — metadata file]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable — metadata]
- Change appears within plan scope? [Yes — PR review artifact]

### Notes

- 128 lines. No code review needed.

---

## File: notes/pr-reviews/phase-0.2/cross-reference-validation.md

### Intent

Cross-reference validation output produced during the PR review gather phase.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — validation output]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable — validation output]
- Change appears within plan scope? [Yes — PR review artifact]

### Notes

- 57 lines. No code review needed.

---

## File: notes/pr-reviews/phase-0.2/cross-reference.md

### Intent

Cross-reference analysis produced during the PR review gather phase.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — validation output]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable — validation output]
- Change appears within plan scope? [Yes — PR review artifact]

### Notes

- 59 lines. No code review needed.

---

## File: notes/pr-reviews/phase-0.2/deferred.md

### Intent

Deferred improvements identified during the PR review.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — documentation]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable — documentation]
- Change appears within plan scope? [Yes — PR review artifact]

### Notes

- 8 lines. Minimal deferred items list.

---

## File: notes/pr-reviews/phase-0.2/draft-fix-document.md

### Intent

A draft document listing defects and issues found during PR review, for the author to address.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — documentation]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable — documentation]
- Change appears within plan scope? [Yes — PR review artifact]

### Notes

- 9 lines. No code review needed.

---

## File: notes/pr-reviews/phase-0.2/draft-fix-plan.toml

### Intent

A draft fix plan in TOML format specifying what needs to be changed to address review findings.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — fix plan]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable — fix plan]
- Change appears within plan scope? [Yes — PR review artifact]

### Notes

- 27 lines. No code review needed.

---

## File: notes/pr-reviews/phase-0.2/draft-review.md

### Intent

A draft review summary produced during the PR review process.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — documentation]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable — documentation]
- Change appears within plan scope? [Yes — PR review artifact]

### Notes

- 15 lines. No code review needed.

---

## File: notes/pr-reviews/phase-0.2/file-manifest.json

### Intent

A JSON manifest listing all files in the diff with metadata (line counts, function lists, imports). Produced by the gather phase.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — data file]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable — data file]
- Change appears within plan scope? [Yes — PR review artifact]

### Notes

- 791 lines. Structured data file produced by the pipeline.

---

## File: notes/pr-reviews/phase-0.2/fix-plan.toml

### Intent

The authoritative fix plan in TOML format, specifying what changes are needed to address review findings.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — fix plan]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable — fix plan]
- Change appears within plan scope? [Yes — PR review artifact]

### Notes

- 27 lines. No code review needed.

---

## File: notes/pr-reviews/phase-0.2/gather-summary.md

### Intent

Summary of the gather phase outputs for the PR review.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — documentation]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable — documentation]
- Change appears within plan scope? [Yes — PR review artifact]

### Notes

- 34 lines. No code review needed.

---

## File: notes/pr-reviews/phase-0.2/per-file-analysis-template.md

### Intent

The template used to generate the per-file analysis. This is meta-infrastructure for the review pipeline.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — template file]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable — template]
- Change appears within plan scope? [Yes — review pipeline artifact]

### Notes

- 1450 lines. Self-referential — this template was used to produce the analysis you are reading.

---

## File: notes/pr-reviews/phase-0.2/per-file-analysis.md

### Intent

The output of this analysis — a per-file code review with judgment fields filled in based on the raw diff data.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — this is the output being produced]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable — this is the output]
- Change appears within plan scope? [Yes — this is the review of the implementation]

### Notes

- 1448 lines. This is the deliverable being produced by this task.

---

## File: notes/pr-reviews/phase-0.2/raw-diff.md

### Intent

The raw unified diff produced by `git diff` for the entire phase-0.2 branch. This is the primary data source for the review.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — raw data file]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable — raw data]
- Change appears within plan scope? [Yes — raw data for review]

### Notes

- 19027 lines. Contains diffs for all files changed between the base commit and HEAD on phase-0.2.
- The diff covers 37 distinct files including source code, tests, plan documents, and execution artifacts.

---

## File: notes/pr-reviews/phase-0.2/review.md

### Intent

Review summary document produced during the PR review process.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — documentation]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable — documentation]
- Change appears within plan scope? [Yes — PR review artifact]

### Notes

- 35 lines. No code review needed.

---

## File: notes/pr-reviews/phase-0.2/status.md

### Intent

Status tracking for the PR review process, recording completion state of review tasks.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — documentation]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable — documentation]
- Change appears within plan scope? [Yes — PR review artifact]

### Notes

- 53 lines. No code review needed.

---

## File: plans/phase-0.2.toml

### Intent

The authoritative enriched implementation plan for phase-0.2. Contains all 10 tasks (TASK-PREP through TASK-10) with detailed file-level before/after changes, acceptance criteria, and dependency graphs.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — plan document]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable — plan document]
- Change appears within plan scope? [Yes — this is the plan itself]

### Notes

- 2168 lines. Structurally identical to `notes/plan-enrichment/phase-0.2/plan.approved.toml`.
- Defines the task dependency chain: TASK-PREP -> TASK-1 -> TASK-3 -> TASK-2, TASK-4 -> TASK-5 -> TASK-6 -> TASK-7 -> TASK-8 -> TASK-9 -> TASK-10.

---

## File: src/cargo_info.rs

### Intent

Added `#[must_use]` attributes, doc comments with `# Errors` sections to `parse_cargo_toml`, and 3 unit tests covering minimal parsing, default values for missing package fields, and dependency classification (normal, dev, workspace).

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Existing `Error::FileRead` error propagation is meaningful, uses anyhow]
- Dead code or unused imports? [No]
- New public API: tests present? [Yes — 3 tests]
- Change appears within plan scope? [Yes — TASK-2 (unit tests for cargo_info)]

### Notes

- `parse_cargo_toml_parses_minimal` uses `tempfile::tempdir()` which is the new dev-dependency.
- `parse_cargo_toml_uses_defaults_for_missing_package` validates the fallback behavior for missing `[package]`.
- `parse_cargo_toml_distinguishes_deps` exercises the workspace dependency tracking logic — checks both `normal`/`dev` lists and `workspace_members` set.

---

## File: src/cross_refs.rs

### Intent

Added `#[must_use]` to `kind_to_string`, and a test module with `make_crate` helper and 2 tests: one verifying cross-crate import detection, one verifying empty result when no cross-references exist.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — cross_refs is pure computation, no fallible operations]
- Dead code or unused imports? [No]
- New public API: tests present? [Yes — 2 tests]
- Change appears within plan scope? [Yes — TASK-2 (unit tests for cross_refs)]

### Notes

- `make_crate` is a well-structured helper that builds a minimal `CrateInfo` using the builder pattern.
- `compute_finds_cross_crate_import` manually injects an `Import` into a crate's module to test the cross-reference computation.
- `compute_empty_for_no_cross_references` asserts that types are either empty or have no imported_by entries.

---

## File: src/file_parser.rs

### Intent

Major refactoring: `parse_file` changed from `Result<(syn::File, FileInfo)>` to `ParsedFile`, enabling partial results on parse failure. Added `ParsedFile` and `SynParseError` types, `build_parse_error_entry` helper, `extract_re_exports_from_tree` improvements to handle full paths and renames, and 10 unit tests.

### Checklist

- Unnecessary clone/unwrap/expect? [No — `std::fs::remove_file(&tmp).ok()` is an intentional fire-and-forget cleanup in test]
- Error handling: [Significantly improved — parse failures now produce `SynParseError` instead of being silently swallowed. `build_parse_error_entry` converts structured parse errors into `ErrorEntry`]
- Dead code or unused imports? [No]
- New public API: tests present? [Yes — 10 tests]
- Change appears within plan scope? [Yes — TASK-1 (ErrorVisibilityPipeline) and TASK-2 (unit tests)]

### Notes

- `parse_file` now takes a "never fail" approach: both file-read errors and syn parse errors return `ParsedFile` with an optional `parse_error`. This is the core mechanism for Goal 1 (Error Visibility Pipeline).
- `extract_re_exports_from_tree` was improved to build `full_path` for both `Name` and `Rename` variants, correctly handling `pub use crate::foo as bar`.
- `flatten_use_tree` changed `prefix: String` to `prefix: &str` to avoid unnecessary clones in the recursive `Group` case.
- `extract_attrs` replaced nested if-let chains with inline if-let guards (Rust 2024 style).
- `into_public_item` replaced `m.ident.as_ref().map(|i| i.to_string())` with `m.ident.as_ref().map(ToString::to_string)`.
- The `COUNTER` atomic counter in the test module is used to generate unique temporary file names for test isolation.

---

## File: src/lib.rs

### Intent

Removed 9 `#![allow(clippy::*)]` suppressions, changed `run` signature from `run(config: Config)` to `run(config: &Config)`, and rewrote the main pipeline to collect errors via `ErrorEntry` instead of silently swallowing failures or printing to stderr.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Transformed from silent error swallowing (eprintln + return None) to structured `ErrorEntry` collection. This is the core implementation of Goal 1 (Error Visibility Pipeline)]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable — `run` is an integration entry point, covered by integration tests]
- Change appears within plan scope? [Yes — TASK-1 (ErrorVisibilityPipeline), TASK-5 (Remove All Clippy Suppressions)]

### Notes

- The 9 clippy suppressions removed: `missing_errors_doc`, `must_use_candidate`, `doc_markdown`, `uninlined_format_args`, `redundant_closure`, `collapsible_if`, `needless_pass_by_value`, `needless_borrow`, `redundant_closure_for_method_calls`.
- Only `#[allow(clippy::too_many_lines)]` remains on `run()`, which is justified by the complexity of the parallel processing pipeline.
- `run` now takes `&Config` instead of `Config` — this fixes a needless-pass-by-value issue.
- The parallel processing closure now returns `(Option<CrateInfo>, Vec<ErrorEntry>)` tuples, with errors collected into `crate_errors`.
- `module_tree::build_module_tree` is now called directly (not `unwrap_or_default`) — errors are collected into `collected_errors` and tagged with `"module_tree_error"` if no kind is set.
- Added doc comment with `# Errors` section to `run`.

---

## File: src/main.rs

### Intent

Updated the call to `run()` to pass `&config` instead of `config` (moving from owned to borrowed).

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — single-line change]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable]
- Change appears within plan scope? [Yes — follows from lib.rs signature change]

### Notes

- 1 line changed: `rust_workspace_map::run(config)` -> `rust_workspace_map::run(&config)`.

---

## File: src/module_tree.rs

### Intent

Changed `build_module_tree` and all helper functions from returning `Result<T>` to returning `(T, Vec<ErrorEntry>)`. Fixed 3 path fallback bugs (`unwrap_or_else(|| Path::new("."))` -> `unwrap_or(parent)`). Added error propagation through the module tree traversal, orphaned module error emission, and 4 unit tests.

### Checklist

- Unnecessary clone/unwrap/expect? [No — tests use `.ok()` for cleanup which is intentional]
- Error handling: [Significantly improved — orphaned modules now produce `ErrorEntry` with `kind = "orphaned_module"` and severity `Warning` instead of eprintln. `build_module_tree` no longer uses `unwrap_or_default`]
- Dead code or unused imports? [No]
- New public API: tests present? [Yes — 4 tests]
- Change appears within plan scope? [Yes — TASK-1 (ErrorVisibilityPipeline), TASK-3 (Path Safety Fixes), TASK-2 (unit tests)]

### Notes

- `resolve_module_path` gained a doc comment clarifying it returns `None` if neither path exists.
- Path fix: `crate_root.parent().unwrap_or_else(|| Path::new("."))` changed to `crate_root.parent().unwrap_or(crate_root)`. Using the original path as fallback is safer than defaulting to `"."` — a module's parent being the crate root is a valid inference.
- `process_submodule` now returns `(Vec<ModuleInfo>, Vec<ErrorEntry>)` instead of `Result<Vec<ModuleInfo>>`.
- `process_module_info` signature changed: added `errors: &mut Vec<ErrorEntry>` parameter for error accumulation. Added `#[allow(clippy::too_many_arguments)]`.
- `process_module_items` changed to return `(Vec<ModuleInfo>, Vec<ErrorEntry>)`.

---

## File: src/render.rs

### Intent

Added `#[must_use]` attributes, doc comments with `# Errors` sections to `render_json` and `render_to_writer`, and 3 unit tests covering valid JSON output, empty error serialization skipping, and writer/json parity.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — render functions delegate to serde_json and return serde_json::Result]
- Dead code or unused imports? [No]
- New public API: tests present? [Yes — 3 tests]
- Change appears within plan scope? [Yes — TASK-2 (unit tests for render)]

### Notes

- `render_json_produces_valid_json` round-trips through serde_json to verify the output is valid.
- `render_json_skips_empty_errors` validates that `skip_serializing_if` on the errors field works correctly — empty errors vector should not appear in JSON output.
- `render_to_writer_matches_render_json` verifies that `to_writer_pretty` produces identical output to `to_string_pretty`.

---

## File: src/schema.rs

### Intent

Added `ErrorSeverity` enum, `ErrorContext` struct with builder, `MissingWorkspaceSection` error variant, and extended `ErrorEntry` with `severity`, `kind`, `context`, and `cause` fields.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Excellent — `ErrorSeverity` (Error/Warning), `ErrorContext` (crate_name, module_path, line, snippet), `kind` (machine-readable tag), and `cause` (original error message) provide rich structured error reporting. All fields are serde-serializable with skip_serializing_if for Option fields]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable — schema types are consumed by callers, covered by integration tests]
- Change appears within plan scope? [Yes — TASK-1 (ErrorVisibilityPipeline)]

### Notes

- `ErrorSeverity` derives `Debug, Clone, Copy, PartialEq, Eq, Serialize` — compact and serializable.
- `ErrorContext` uses `bon::Builder` and `#[serde(rename_all = "camelCase")]` for consistent JSON output.
- `ErrorEntry` extension adds 4 new fields: `severity`, `kind`, `context`, `cause` — all optional except `severity` and `kind`.
- `MissingWorkspaceSection` variant has no associated data, matching the pattern of returning a clear error when the workspace is misconfigured.
- Fixed `FileInfo` doc comment to properly escape backticks: `consumed by \`module_tree\``.

---

## File: src/workspace.rs

### Intent

Added `#[must_use]` to `resolve_crate_roots`, added `# Errors` doc comments, changed `enumerate_members` to return `Error::MissingWorkspaceSection` when `[workspace]` section is absent instead of silently defaulting, and added 6 unit tests.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Improved — `enumerate_members` now errors instead of silently returning empty list when `[workspace]` section is missing]
- Dead code or unused imports? [No]
- New public API: tests present? [Yes — 6 tests]
- Change appears within plan scope? [Yes — TASK-1 (ErrorVisibilityPipeline), TASK-3 (Path Safety Fixes), TASK-2 (unit tests)]

### Notes

- `enumerate_members` changed from a chain of `.and_then().map().unwrap_or_default()` to a `match` that explicitly returns `Err(Error::MissingWorkspaceSection)` when workspace section is absent.
- `resolve_crate_roots` gained `#[must_use]` attribute.
- `setup_crate` test helper duplicates logic from `write_cargo_toml` (both write a Cargo.toml with package fields). This is acceptable since `setup_crate` also creates the src/lib.rs file.
- 6 tests cover: workspace root discovery, member enumeration, missing workspace section error, exclude list application, lib/bin detection.

---

## File: tests/fixtures/sample-workspace/Cargo.lock

### Intent

A new Cargo.lock file for the sample-workspace test fixture, pinning dependency versions for the test workspace that contains `core` and `engine` crates.

### Checklist

- Unnecessary clone/unwrap/expect? [No]
- Error handling: [Not applicable — auto-generated lock file]
- Dead code or unused imports? [No]
- New public API: tests present? [Not applicable — fixture data]
- Change appears within plan scope? [Yes — test fixture needed by integration tests]

### Notes

- 83 lines. Contains `core` (depends on serde), `engine` (depends on core and serde), and pinned versions of proc-macro2, quote, serde, syn, unicode-ident.
- This file is needed by `test_output_via_flag` which runs the binary against the sample workspace.

---

## File: tests/integration_test.rs

### Intent

Expanded from 3 to 10 integration tests. Added 7 new tests covering parse failure error entries, missing workspace section, glob member patterns, workspace exclude, deeply nested modules, re-export chains, and -o flag output. Added test helper functions: `run_binary`, `parse_output`, `write_cargo_toml`, `setup_crate`, `Secret`, `extract_array`.

### Checklist

- Unnecessary clone/unwrap/expect? [No — helper functions use unwrap appropriately for test setup; `run_binary` uses `.expect("failed to execute binary")` which is correct for a test helper that must not fail silently]
- Error handling: [Tests verify that error paths produce the expected structured output — `test_parse_failure_error_entry` checks for `syn_parse_error` kind and `error` severity in the JSON output]
- Dead code or unused imports? [No]
- New public API: tests present? [Yes — 10 integration tests]
- Change appears within plan scope? [Yes — TASK-4 (Integration Test Expansion)]

### Notes

- `parse_output` uses `.unwrap()` on JSON deserialization — this is acceptable in tests since malformed JSON output from the binary would be a real bug worth failing loudly.
- `test_parse_failure_error_entry` creates a 2-crate workspace (one valid, one with invalid Rust) and verifies the error is captured in JSON output.
- `test_glob_member_patterns` creates 3 crates under `crates/*` and verifies all are discovered.
- `test_workspace_with_exclude` verifies that `exclude = ["b"]` removes crate `b` from the member list.
- `test_deeply_nested_modules` creates a multi-level module hierarchy (mod a { mod b { mod c {} } }) and verifies the tree structure.
- `test_reexport_chains` verifies re-export detection for `pub use crate::core::Secret as EngineSecret`.
- `test_output_via_flag` compares `-o` file output with stdout output for equality.
- `Secret` struct is a test-only helper type used in `test_reexport_chains`.
- `extract_array` is a generic helper that navigates JSON values — used across multiple tests.
