# Deferred Items & Architectural Patterns -- Single-Crate Support Phase

> Generated: 2026-05-06 | Phase: single-crate-support
> Synthesized from: `plans/single-crate-support/PLAN.md`, `notes/pr-reviews/{mvp,phase-0.1,phase-0.2}/deferred.md`, `notes/architecture-current.md`

---

## 1. Key Architectural Patterns from the Plan

### Auto-detect, not flag-gated

The plan chooses auto-detection over a new CLI flag (`--single-crate`). This keeps the CLI surface clean but introduces an ambiguity: what happens if a Cargo.toml has both `[workspace]` AND is also a valid single crate? The plan handles this correctly -- workspace mode takes priority, single-crate is only a fallback from `WorkspaceRootNotFound`. No change to CLI struct necessary.

### Two separate discovery functions

`find_workspace_root` and `find_crate_root` are separate, not a combined function. Rationale: a combined function that checks for either `[workspace]` or `[package]` would stop at a workspace member's Cargo.toml (which has `[package]` but no `[workspace]`), yielding the member directory instead of the workspace root. This is the correct design for a multi-crate workspace but is also correct for single-crate: scanning for `[package]` finds the crate root regardless.

### Pipeline integration point

The fallback lives in `lib.rs:build_map`, not in `workspace.rs`. This is important: `workspace.rs` remains pure -- `find_workspace_root` only looks for `[workspace]`, `find_crate_root` only looks for `[package]`. The orchestration logic (try A, on specific error fall back to B) stays in the pipeline coordinator. This preserves single-responsibility in the workspace module.

### Silent error exits are a hard fix

Four sites in `main.rs` swallow errors. The plan treats all four as a single work item, not separate tasks. This is correct: fixing only some would leave the user with a partial (and confusing) experience -- single-crate works but failures are still invisible.

### Integration test behavior changes

The existing `test_missing_workspace_section` test expects exit code 1 for a crate without `[workspace]`. After the fix, this same scenario must succeed. The test is renamed to `test_single_crate_without_workspace` and its assertions inverted. This is a deliberate signal: the behavior change is not a regression but a new capability.

---

## 2. Known Failure Modes to Avoid

### F1. Silent error swallowing (being fixed)

The plan directly addresses the four `exit(1)` sites in `main.rs`. Verify during implementation that no additional silent-exit sites exist (e.g., any remaining `.is_err()` patterns that skip error messages). After this fix, every error path should print to stderr.

### F2. Path safety in validate.rs

Plan explicitly flags this in its Implementation Note (lines 139-141). `validate::validate` must not assume `workspace_root` is a parent-of-crates directory. In single-crate mode, `workspace_root == crate_dir`. Read `validate.rs:check_orphan_files` before implementing to confirm it uses `crate_info` paths directly.

### F3. Hardcoded `strip_prefix("src/")` in validate.rs (deferred MVP fix)

From MVP deferred item #1: `determine_parent_file` hardcodes `strip_prefix("src/")`. This fails for workspace members not at the workspace root. Not directly triggered by single-crate mode (single crate IS at workspace root), but if a user later adds the standalone crate to a workspace, the hint breaks. Worth noting but not blocking.

### F4. Unwrapped `unwrap_or_default()` in workspace.rs

From phase-0.1 deferred item #4: workspace.rs defaults to empty vectors when workspace section is missing or malformed. The single-crate fallback in `build_map` only catches `WorkspaceRootNotFound`, not `MissingWorkspaceSection` or `TomlParse`. If a Cargo.toml exists but is malformed, the fallback is not triggered and the old silent behavior may surface. Verify during implementation that the error match is specific enough.

### F5. Missing test for recursive scenarios from fixture

From MVP deferred item #2. Not directly related to this phase, but the `test_single_crate_module_tree` test in Step 7 should at minimum cover modules at depth 2 (e.g., `helpers/sub.rs` declared from `helpers.rs`). This would prevent regression in recursive module resolution.

### F6. `errors.clone()` in recursive `process_module_info` (deferred phase-0.2)

Not triggered by this phase (module_tree.rs is unchanged), but worth noting for future performance work. The clone cost is O(depth x errors) and could be high for crates with many errors.

---

## 3. Deferred Improvements to Incorporate Now

The following items from prior deferred.md files are relevant to this phase and should be addressed during implementation, not postponed further.

### D1. Fix `MissingWorkspaceSection` error handling

**Source:** phase-0.1 deferred #4, phase-0.2 architecture note line 142
**Why now:** The single-crate fallback in `build_map` only matches `WorkspaceRootNotFound`. If a Cargo.toml exists but lacks `[workspace]` AND is also not a valid single crate (no `[package]` either), `find_crate_root` will walk past it and potentially find an unrelated ancestor. The error chain should distinguish between "no Cargo.toml found" and "Cargo.toml found but no `[workspace]` or `[package]` section".
**Action:** Ensure the error match in `build_map` catches both `WorkspaceRootNotFound` and `MissingWorkspaceSection` before falling back. Add `MissingWorkspaceSection` to the fallback pattern if applicable.

### D2. Wire empty `errors` vector in output

**Source:** phase-0.1 deferred #1
**Why now:** The plan adds error handling paths. If single-crate discovery fails (e.g., `CrateRootNotFound`), the error should be visible in the JSON output, not just on stderr. Currently `WorkspaceMap.errors` is always empty. Consider populating it with the fallback failure for consistency with the partial-results design.
**Action:** Low-priority -- verify current behavior with `cargo run -- index /tmp/nonexistent` (Step 5 in verification). If the error appears on stderr and exit code is non-zero, this is sufficient. No wiring change needed.

### D3. Remove `unwrap_or_default` in workspace.rs resolution

**Source:** phase-0.1 deferred #4
**Why now:** `enumerate_members` currently uses `.unwrap_or_default()` for member/exclude parsing. In single-crate mode, `enumerate_members` is never called (the fallback creates `vec![crate_dir]` directly). But if the workspace path has a valid `[workspace]` section with malformed members, the fallback is skipped and the old silent behavior persists.
**Action:** Review during implementation to ensure the error path is not silently skipped. If `find_workspace_root` succeeds, we must still handle `enumerate_members` errors properly.

---

## 4. Interaction Matrix: Plan Steps vs. Deferred Items

| Plan Step | Relevant Deferred Items | Risk |
|---|---|---|
| Step 1: Add `CrateRootNotFound` variant | None directly | Low |
| Step 2: Add `find_crate_root` | D1 (MissingWorkspaceSection scenario) | Low -- standalone function, well-encapsulated |
| Step 3: Unit tests for `find_crate_root` | F5 (recursive test missing) | Low -- two tests cover success and failure |
| Step 4: Modify `build_map` in `lib.rs` | **D1**, D2, D3 | **Medium** -- error match pattern must be comprehensive |
| Step 5: Fix silent error exits in `main.rs` | F1 (being fixed) | Low -- straightforward replacement |
| Step 6: Update `test_missing_workspace_section` | None | Low -- test behavior inversion is correct |
| Step 7: Add `test_single_crate_module_tree` | F5 (depth coverage) | Low -- add at least depth-2 modules |

---

## 5. Verification Checklist Additions

Beyond the plan's verification list, add:

1. **Malformed Cargo.toml scenario:** Place a `Cargo.toml` without `[workspace]` or `[package]` in a temp directory, run `cargo run -- index <dir>`. Verify clear error message on stderr, non-zero exit.
2. **Find_workspace_root succeeds with bad members:** Create a workspace with a valid `[workspace]` section but pointing to a non-existent member. Verify `enumerate_members` error surfaces (not swallowed).
3. **Module depth in test:** `test_single_crate_module_tree` should include at least one module at depth >= 2 (e.g., `helpers/sub.rs`).
4. **Clippy on new code:** `cargo clippy -- -D warnings` on the new `find_crate_root` function. The function uses `content.contains("[package]")` -- verify there is no clippy lint about string contains vs. more specific parsing.
