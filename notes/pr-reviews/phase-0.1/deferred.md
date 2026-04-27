## Deferred Improvements: `phase-0.1` — 2026-04-28

### Empty errors vector in run()

**Source:** Round 1 review
**Rationale:** The `errors` field in `WorkspaceMap` is always an empty array because no code path ever populates it. This was part of the schema design but the mechanism to collect and report workspace-level errors was never wired in. Adding error collection would make the tool more useful for debugging misconfigured workspaces and provide a structured error path distinct from stderr warnings.
**Candidate for:** Phase 0.2 plan
**Precondition:** Need to define which workspace-level errors should be collected (e.g., member parse failures vs. missing crate roots)

### Silent error swallowing in module tree construction

**Source:** Round 1 review
**Rationale:** `build_module_tree` errors are swallowed via `unwrap_or_default()`, making it impossible to distinguish between a crate with no public items and one whose parsing failed. This undermines the reliability of the tool's output for downstream consumers. Should be replaced with proper error propagation, perhaps through the `errors` field or structured stderr reporting.
**Candidate for:** Phase 0.2 plan
**Precondition:** First consumer of the JSON output who needs visibility into per-crate health

### Path fallback to "." in module_tree.rs

**Source:** Round 1 review
**Rationale:** Three `.unwrap_or_else(|| Path::new("."))` calls produce incorrect relative paths when `.parent()` returns None. While practically unreachable for real Rust projects (crate roots are never at "/"), this pattern masks bugs during development and produces confusing output.
**Candidate for:** Phase 0.2 plan
**Precondition:** Before adding support for unusual filesystem layouts or enhanced debugging

### Path fallback to "" in workspace.rs

**Source:** Round 1 review
**Rationale:** Defaulting to empty vectors when the workspace section is missing or malformed silently includes zero members rather than producing an informative error. Falling back to "" for file names bypasses the exclude filter. These defaults are lenient but mask configuration errors.
**Candidate for:** Phase 0.2 plan
**Precondition:** Before extending member/exclude resolution logic

### No unit tests for public API functions

**Source:** Round 1 review
**Rationale:** Thirteen public functions across five modules have no unit tests. Integration tests cover the full pipeline but do not isolate individual function behavior, making it impossible to verify parsing logic, edge cases, or error handling without running the entire tool. This increases regression risk during refactoring.
**Candidate for:** Phase 0.2 plan
**Precondition:** Before adding new features or refactoring existing code
