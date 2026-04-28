## Memory

### MEMORY.md (index)

- [Tony's role and preferences](user_tony_role.md) — Rust developer building an LLM-oriented workspace map tool, prefers review-first workflow and partial-results error handling
- [Phase 0.2 plan and decisions](project_phase_02.md) — Current phase context, architectural decisions, execution sequence, and goal descriptions
- [Workflow preferences](feedback_workflow.md) — Review-before-coding, partial results over all-or-nothing, pragmatic crate selection, conventional commit style

### user_tony_role.md

Tony is a Rust developer building `rust-workspace-map`, a CLI tool that analyzes Rust workspace structures and emits structured JSON for LLM consumption. The project is in its early phases (currently phase 0.2: "Hardening for Trustworthiness").

Key preferences:
- Prefers thorough architectural review and planning before coding begins
- Favors partial results over all-or-nothing — when some data has errors, still emit what's available rather than aborting (maximizes information for LLM consumers)
- Pragmatic about crate choices — prefers established, well-understood crates (thiserror, anyhow) over flashy alternatives
- Actively maintains project memory for cross-session continuity

### project_phase_02.md

**Phase:** 0.2 — Hardening for Trustworthiness
**Plan status:** Reviewed (2026-04-28)
**Next step:** Enrich into executable TOML tasks

Key architectural decisions made during plan review:
1. **Partial module tree preservation** — a single submodule parse failure collects an `ErrorEntry` and continues with siblings rather than aborting the entire crate.
2. **External `#[cfg(test)]` modules** are parsed for error reporting but their public items are not extracted.
3. **Error kind taxonomy** — 8 initial values defined: `toml_parse_error`, `syn_parse_error`, `missing_crate_roots`, `orphaned_module`, `module_tree_error`, `missing_workspace_section`, `glob_pattern_error`, `member_not_found`.
4. **Error handling crates** — `thiserror` + `anyhow` are sufficient; no additional crates needed.

Execution sequence:
Goal 3 and Goal 1 can run in parallel -> Goal 1 -> Goal 2 -> Goal 4 -> Goal 5

Goals:
1. Error Visibility Pipeline (Medium) — wire structured ErrorEntry end-to-end
2. Unit Test Suite (Large) — >=80% line coverage across 5 core modules
3. Path Safety Fixes (Small) — fix 4 unwrap_or_default path bugs
4. Integration Test Expansion (Medium) — 3->8+ tests covering errors and edge cases
5. Remove All Clippy Suppressions (Small) — 9 crate-level allows eliminated

How to apply:
Use this context when planning or executing work in this project. The phase plan and decisions file are at `plans/phase-0.2/PHASE_PLAN.md` and `notes/plan-reviews/phase-0.2/decisions.md`.

### feedback_workflow.md

**Review before coding.** Architectural review is Gate 1 before any implementation phase begins. Apply `/plan-review` before `/enrich-phase-plan`.

**Why:** Ensures design soundness is validated before time is spent on decomposition and task execution. Caught a critical all-or-nothing ambiguity during this phase's review.

**How to apply:** Never jump to implementation or task decomposition without first doing a plan review. Always flag ambiguous design points during review rather than assuming they'll be resolved during coding.

**Partial results over all-or-nothing.** When some data has errors, emit what's available rather than aborting entirely. This maximizes information for downstream consumers (LLM agents).

**Why:** The tool's primary consumer is an LLM agent — partial data is far more useful than no data. A single parse failure shouldn't hide all successfully parsed data.

**How to apply:** When designing error handling, prefer collecting errors alongside results over propagating via `?` to abort the parent operation. This applies to module tree construction, crate processing, and any batch operation in the project.

**Pragmatic crate selection.** Prefer established, well-understood crates. Don't add new dependencies unless they solve a concrete problem that existing dependencies don't cover.

**Why:** Fewer dependencies means faster compilation, fewer supply-chain risks, and less cognitive overhead.

**How to apply:** When evaluating a new crate, first check if existing dependencies (`thiserror`, `anyhow`, `serde`, `rayon`, `syn`, `toml`, `glob`) can solve the problem. Only add a new crate if it provides clear, non-trivial value over what's already available.

**Commit messages follow conventional commits.** Use `type(scope): description` format.

**How to apply:** Follow the pattern established in git history. For this project: `plan-review(phase-N):`, `plan(phase-N):`, `review(phase-N):`, `feat:`, `fix:`, `test:`, `refactor:`.

---

## Phase Plan

**File:** `plans/phase-0.2/PHASE_PLAN.md`

(Full contents below — this is the reviewed Phase 0.2 plan for "Hardening for Trustworthiness", dated 2026-04-28.)

### Goals

#### Goal 1 — Error Visibility Pipeline (Medium)
Wire error collection end-to-end so every non-fatal error during a run appears as a structured `ErrorEntry` in the output JSON. No errors are silently swallowed.

The `ErrorEntry` type (already defined in schema but never populated) gains rich context fields:
- `pub struct ErrorEntry` with fields: `file`, `line`, `message`, `severity` (ErrorSeverity enum: Error/Warning), `kind` (machine-readable tag), `context` (ErrorContext with crate_name, module_path, line, snippet), `cause`.

Specific changes:
- Change `parse_file()` to propagate `SynParse` errors as `ErrorEntry` with severity `error`, rather than returning empty results. This applies to all parsed files including inline/external `#[cfg(test)]` module bodies. External `#[cfg(test)]` modules are parsed for error reporting but their public items are not extracted.
- Remove `.unwrap_or_default()` on `module_tree::build_module_tree` in `run()`, capture errors as `ErrorEntry` with `kind = "module_tree_error"`
- Convert `cargo_info::parse_cargo_toml` failures to `ErrorEntry` with `kind = "toml_parse_error"`
- Convert `workspace::resolve_crate_roots` empty results to `ErrorEntry` with `kind = "missing_crate_roots"`
- Collect `ErrorEntry` values from parallel crate processing
- Emit `ErrorEntry` for orphaned modules with `kind = "orphaned_module"`
- Emit `ErrorEntry` for missing workspace members via `Error::MemberNotFound`

#### Goal 2 — Unit Test Suite (Large)
Add `#[cfg(test)]` modules with comprehensive unit tests for all public functions across the 5 core modules. Target >=80% line coverage.

#### Goal 3 — Path Safety Fixes (Small)
Fix four path fallback bugs:
1. `module_tree.rs`: `crate_root.parent().unwrap_or_else(|| Path::new("."))`
2. `module_tree.rs`: same in `process_submodule`
3. `module_tree.rs`: same in `process_module_info`
4. `workspace.rs`: `enumerate_members` returns `Err(Error::MissingWorkspaceSection)` when `[workspace]` section or `members` key is absent

#### Goal 4 — Integration Test Expansion (Medium)
Expand from 3 to 8+ integration tests covering error paths, edge cases, and complex workspace structures.

#### Goal 5 — Remove All Clippy Suppressions (Small)
No `#![allow(clippy::*)]` remains. Each suppression removed with specific fix.

### Scope Boundaries

**In scope:** Structured error collection, comprehensive unit tests, path safety fixes, expanded integration tests, clippy cleanup.

**Out of scope:** New output formats, new extraction features, performance optimization, `cargo metadata`, CLI changes, schema-breaking changes.

### Design Notes
- ErrorContext fields: `crate_name: Option<String>`, `module_path: Option<String>`, `line: Option<usize>`, `snippet: Option<String>`
- ErrorSeverity: `error` (data loss, output is partial) / `warning` (run completed fully but something unusual)
- Partial module tree preservation on submodule parse failure
- Error kind taxonomy: `toml_parse_error`, `syn_parse_error`, `missing_crate_roots`, `orphaned_module`, `module_tree_error`, `missing_workspace_section`, `glob_pattern_error`, `member_not_found`

### Deferred Items Absorbed
All 5 deferred improvements from phase 0.1 are absorbed: empty errors vector, silent error swallowing, path fallback bugs, missing unit tests.

---

## Snapshot

No snapshot — using authoritative data from file-manifest.json.
