# Raw Diff: `single-crate-support` -> `main`
Generated: 2026-05-05T21:25:41Z

## Commits
e1af062 chore: gitignore pipeline and claude artifacts
bfabaf2 feat(single-crate-support): TASK-test-existing-update,TASK-test-single-crate-module-tree: add single-crate integration tests
a5052d2 feat(single-crate-support): TASK-pipeline-fallback: add single-crate fallback in build_map
f795887 feat(single-crate-support): TASK-single-crate-core-02 (TDD): add find_crate_root discovery function
4177d62 chore(single-crate-support): gitignore checkpoint files in worktree
01dd9c8 feat(single-crate-support): TASK-silent-exit-fix: add error messages to stderr before silent exits
8610138 feat(single-crate-support): TASK-single-crate-core-01: add CrateRootNotFound variant to Error enum
178ad42 feat(plan): elaborate directions for single-crate-support phase

## Diff Stat
 .gitignore                                         |   2 +
 .../single-crate-support/codebase-state.md         | 110 ++++
 .../single-crate-support/deferred-and-patterns.md  | 105 ++++
 .../single-crate-support/directions-index.json     |  65 +++
 ...directions-single-crate-support-group-core.json | 100 ++++
 ...ctions-single-crate-support-group-fallback.json |  77 +++
 ...ons-single-crate-support-group-silent-exit.json |  62 +++
 ...irections-single-crate-support-group-tests.json |  88 +++
 .../single-crate-support/directions.json           | 207 +++++++
 .../single-crate-support/draft-directions.json     | 207 +++++++
 .../single-crate-support/draft-elaboration.md      | 406 ++++++++++++++
 .../single-crate-support/task-checklist.md         | 183 +++++++
 .../single-crate-support/workspace-map.json        | 600 +++++++++++++++++++++
 src/lib.rs                                         |  23 +-
 src/main.rs                                        |  12 +-
 src/schema.rs                                      |   3 +
 src/workspace.rs                                   |  47 ++
 tests/integration_test.rs                          |  90 +++-
 18 files changed, 2364 insertions(+), 23 deletions(-)

## Full Diff
diff --git a/.gitignore b/.gitignore
index 73ff9cb..1b1d30b 100644
--- a/.gitignore
+++ b/.gitignore
@@ -20,3 +20,5 @@ target
 #  option (not recommended) you can uncomment the following to ignore the entire idea folder.
 #.idea/
 .pipeline-worktrees/
+.exploration_checkpoint.json
+.claude/
diff --git a/notes/directions/single-crate-support/codebase-state.md b/notes/directions/single-crate-support/codebase-state.md
new file mode 100644
index 0000000..9619fff
--- /dev/null
+++ b/notes/directions/single-crate-support/codebase-state.md
@@ -0,0 +1,110 @@
+# Codebase State -- Single-Crate Support Phase
+
+> Generated: 2026-05-06 | Phase: single-crate-support
+
+## 1. File Tree (Affected Areas)
+
+```
+src/
+  schema.rs     (451 lines)  -- Error enum, all schema types
+  workspace.rs  (222 lines)  -- find_workspace_root, enumerate_members, resolve_crate_roots
+  lib.rs        (212 lines)  -- build_map, run, relativize_path (pipeline orchestrator)
+  main.rs       (172 lines)  -- CLI entry point
+  validate.rs   (664 lines)  -- validate, check_orphan_files, check_dead_reexports
+  cargo_info.rs (156 lines)  -- parse_cargo_toml
+  module_tree.rs(307 lines)  -- build_module_tree, resolve_module_path
+  cross_refs.rs(213 lines)   -- compute (cross-crate references)
+  indexes.rs   (287 lines)   -- derive_from_crates
+  render.rs    ( 91 lines)   -- render_json, render_to_writer
+  lookup.rs    (206 lines)   -- lookup_symbol, lookup_file
+tests/
+  integration_test.rs (660 lines) -- all integration tests
+  fixtures/
+    sample-workspace/   -- workspace with [workspace] section
+    bad-orphan/         -- workspace with orphan .rs files
+    bad-dead-reexport/  -- workspace with dead re-exports
+```
+
+## 2. Key Type/Function Signatures
+
+### schema.rs (Error enum, lines 52-83)
+```rust
+pub enum Error {
+    WorkspaceRootNotFound(PathBuf),      // line 54
+    // NEW variant to add after line 55: CrateRootNotFound(PathBuf)
+    FileRead { path: PathBuf, source: std::io::Error },
+    TomlParse { path: PathBuf, source: toml::de::Error },
+    SynParse { path: PathBuf, source: syn::Error },
+    MemberNotFound(PathBuf),
+    GlobPattern(String),
+    MissingWorkspaceSection,
+}
+pub type Result<T> = std::result::Result<T, Error>;
+```
+
+### workspace.rs (discovery functions)
+```rust
+pub fn find_workspace_root(start_path: &Path) -> Result<PathBuf>          // line 11
+pub fn enumerate_members(root: &Path) -> Result<Vec<PathBuf>>             // line 35
+pub fn resolve_crate_roots(crate_dir: &Path) -> Vec<(PathBuf, CrateType)> // line 210
+```
+
+### lib.rs (pipeline)
+```rust
+pub fn build_map(config: &Config) -> anyhow::Result<WorkspaceMap>  // line 36
+pub fn run(config: &Config) -> anyhow::Result<()>                  // line 186
+fn relativize_path(path_str: &str, root: &Path) -> String          // line 205
+```
+
+### main.rs (CLI) -- silent error exits
+Lines 88-90: file write `.is_err()` -> silent `exit(1)`
+Lines 93-95: stdout write `.is_err()` -> silent `exit(1)`
+Lines 102-104: pipeline error `Err(_)` -> silent `exit(1)`
+Lines 165-167: lookup error `Err(_)` -> silent `exit(1)`
+
+## 3. Module Dependency Graph
+
+```
+main.rs
+  └── lib.rs::build_map, lib.rs::run, render::render_to_writer, lookup::lookup_*
+
+lib.rs::build_map
+  ├── workspace::find_workspace_root   (dependency: schema::Error)
+  ├── workspace::enumerate_members     (dependency: schema::Error, glob)
+  ├── cargo_info::parse_cargo_toml     (dependency: schema::*)
+  ├── workspace::resolve_crate_roots
+  ├── module_tree::build_module_tree   (dependency: file_parser::parse_file)
+  ├── cross_refs::compute
+  ├── indexes::derive_from_crates
+  ├── validate::validate               (dependency: check_orphan_files, check_dead_reexports)
+  └── render::render_to_writer
+```
+
+## 4. Existing Test Structure
+
+### Unit tests per module
+- `workspace.rs` (6 tests): `find_workspace_root_finds_cargo_toml`, `enumerate_members_returns_members`, `enumerate_members_returns_err_for_missing_workspace`, `enumerate_members_applies_exclude`, `resolve_crate_roots_detects_lib`, `resolve_crate_roots_detects_bin`
+- `cargo_info.rs` (3 tests), `module_tree.rs` (4 tests), `cross_refs.rs` (2 tests), `indexes.rs` (5 tests), `validate.rs` (13 tests), `render.rs` (3 tests), `lookup.rs` (4 tests)
+
+### Integration tests (18 tests)
+All in `tests/integration_test.rs`. Key ones affected by this phase:
+- `test_missing_workspace_section` (line 199): expects exit 1 for crate without [workspace] — WILL BREAK after fix
+- `test_deeply_nested_modules` (line 284): uses workspace mode with `members = ["."]`
+
+## 5. Validate Path Safety Analysis
+
+### check_orphan_files (validate.rs:90-158)
+`crate_root_path = workspace_root.join(&crate_info.root)`
+
+**In workspace mode:** workspace_root=/ws, crate_info.root="core/src/lib.rs" -> /ws/core/src/lib.rs
+**In single-crate mode:** workspace_root=/crate, crate_info.root="src/lib.rs" -> /crate/src/lib.rs
+
+Both modes produce correct paths. The `strip_prefix(workspace_root)` call at line 135 also works identically. **Safe.**
+
+## 6. Key Observations
+
+1. **lib.rs does NOT import `schema::Error`** (lines 18-21 only import specific types). The plan's Step 4a (add Error to import) is required for the match pattern to compile.
+2. **The plan's D1 enhancement** (catch MissingWorkspaceSection too) is recommended by the architect. The match becomes: `Ok(root)` -> try `enumerate_members`, catch `MissingWorkspaceSection` -> fallback.
+3. **No additional silent-exit sites** beyond the four identified.
+4. **validate.rs path safety is confirmed** — no changes needed.
+5. **Four silent-exit serialization errors in Lookup branch** (lines 123-128, 140-145) already print messages on stderr so are NOT "silent". Not in scope.
diff --git a/notes/directions/single-crate-support/deferred-and-patterns.md b/notes/directions/single-crate-support/deferred-and-patterns.md
new file mode 100644
index 0000000..fff3157
--- /dev/null
+++ b/notes/directions/single-crate-support/deferred-and-patterns.md
@@ -0,0 +1,105 @@
+# Deferred Items & Architectural Patterns -- Single-Crate Support Phase
+
+> Generated: 2026-05-06 | Phase: single-crate-support
+> Synthesized from: `plans/single-crate-support/PLAN.md`, `notes/pr-reviews/{mvp,phase-0.1,phase-0.2}/deferred.md`, `notes/architecture-current.md`
+
+---
+
+## 1. Key Architectural Patterns from the Plan
+
+### Auto-detect, not flag-gated
+
+The plan chooses auto-detection over a new CLI flag (`--single-crate`). This keeps the CLI surface clean but introduces an ambiguity: what happens if a Cargo.toml has both `[workspace]` AND is also a valid single crate? The plan handles this correctly -- workspace mode takes priority, single-crate is only a fallback from `WorkspaceRootNotFound`. No change to CLI struct necessary.
+
+### Two separate discovery functions
+
+`find_workspace_root` and `find_crate_root` are separate, not a combined function. Rationale: a combined function that checks for either `[workspace]` or `[package]` would stop at a workspace member's Cargo.toml (which has `[package]` but no `[workspace]`), yielding the member directory instead of the workspace root. This is the correct design for a multi-crate workspace but is also correct for single-crate: scanning for `[package]` finds the crate root regardless.
+
+### Pipeline integration point
+
+The fallback lives in `lib.rs:build_map`, not in `workspace.rs`. This is important: `workspace.rs` remains pure -- `find_workspace_root` only looks for `[workspace]`, `find_crate_root` only looks for `[package]`. The orchestration logic (try A, on specific error fall back to B) stays in the pipeline coordinator. This preserves single-responsibility in the workspace module.
+
+### Silent error exits are a hard fix
+
+Four sites in `main.rs` swallow errors. The plan treats all four as a single work item, not separate tasks. This is correct: fixing only some would leave the user with a partial (and confusing) experience -- single-crate works but failures are still invisible.
+
+### Integration test behavior changes
+
+The existing `test_missing_workspace_section` test expects exit code 1 for a crate without `[workspace]`. After the fix, this same scenario must succeed. The test is renamed to `test_single_crate_without_workspace` and its assertions inverted. This is a deliberate signal: the behavior change is not a regression but a new capability.
+
+---
+
+## 2. Known Failure Modes to Avoid
+
+### F1. Silent error swallowing (being fixed)
+
+The plan directly addresses the four `exit(1)` sites in `main.rs`. Verify during implementation that no additional silent-exit sites exist (e.g., any remaining `.is_err()` patterns that skip error messages). After this fix, every error path should print to stderr.
+
+### F2. Path safety in validate.rs
+
+Plan explicitly flags this in its Implementation Note (lines 139-141). `validate::validate` must not assume `workspace_root` is a parent-of-crates directory. In single-crate mode, `workspace_root == crate_dir`. Read `validate.rs:check_orphan_files` before implementing to confirm it uses `crate_info` paths directly.
+
+### F3. Hardcoded `strip_prefix("src/")` in validate.rs (deferred MVP fix)
+
+From MVP deferred item #1: `determine_parent_file` hardcodes `strip_prefix("src/")`. This fails for workspace members not at the workspace root. Not directly triggered by single-crate mode (single crate IS at workspace root), but if a user later adds the standalone crate to a workspace, the hint breaks. Worth noting but not blocking.
+
+### F4. Unwrapped `unwrap_or_default()` in workspace.rs
+
+From phase-0.1 deferred item #4: workspace.rs defaults to empty vectors when workspace section is missing or malformed. The single-crate fallback in `build_map` only catches `WorkspaceRootNotFound`, not `MissingWorkspaceSection` or `TomlParse`. If a Cargo.toml exists but is malformed, the fallback is not triggered and the old silent behavior may surface. Verify during implementation that the error match is specific enough.
+
+### F5. Missing test for recursive scenarios from fixture
+
+From MVP deferred item #2. Not directly related to this phase, but the `test_single_crate_module_tree` test in Step 7 should at minimum cover modules at depth 2 (e.g., `helpers/sub.rs` declared from `helpers.rs`). This would prevent regression in recursive module resolution.
+
+### F6. `errors.clone()` in recursive `process_module_info` (deferred phase-0.2)
+
+Not triggered by this phase (module_tree.rs is unchanged), but worth noting for future performance work. The clone cost is O(depth x errors) and could be high for crates with many errors.
+
+---
+
+## 3. Deferred Improvements to Incorporate Now
+
+The following items from prior deferred.md files are relevant to this phase and should be addressed during implementation, not postponed further.
+
+### D1. Fix `MissingWorkspaceSection` error handling
+
+**Source:** phase-0.1 deferred #4, phase-0.2 architecture note line 142
+**Why now:** The single-crate fallback in `build_map` only matches `WorkspaceRootNotFound`. If a Cargo.toml exists but lacks `[workspace]` AND is also not a valid single crate (no `[package]` either), `find_crate_root` will walk past it and potentially find an unrelated ancestor. The error chain should distinguish between "no Cargo.toml found" and "Cargo.toml found but no `[workspace]` or `[package]` section".
+**Action:** Ensure the error match in `build_map` catches both `WorkspaceRootNotFound` and `MissingWorkspaceSection` before falling back. Add `MissingWorkspaceSection` to the fallback pattern if applicable.
+
+### D2. Wire empty `errors` vector in output
+
+**Source:** phase-0.1 deferred #1
+**Why now:** The plan adds error handling paths. If single-crate discovery fails (e.g., `CrateRootNotFound`), the error should be visible in the JSON output, not just on stderr. Currently `WorkspaceMap.errors` is always empty. Consider populating it with the fallback failure for consistency with the partial-results design.
+**Action:** Low-priority -- verify current behavior with `cargo run -- index /tmp/nonexistent` (Step 5 in verification). If the error appears on stderr and exit code is non-zero, this is sufficient. No wiring change needed.
+
+### D3. Remove `unwrap_or_default` in workspace.rs resolution
+
+**Source:** phase-0.1 deferred #4
+**Why now:** `enumerate_members` currently uses `.unwrap_or_default()` for member/exclude parsing. In single-crate mode, `enumerate_members` is never called (the fallback creates `vec![crate_dir]` directly). But if the workspace path has a valid `[workspace]` section with malformed members, the fallback is skipped and the old silent behavior persists.
+**Action:** Review during implementation to ensure the error path is not silently skipped. If `find_workspace_root` succeeds, we must still handle `enumerate_members` errors properly.
+
+---
+
+## 4. Interaction Matrix: Plan Steps vs. Deferred Items
+
+| Plan Step | Relevant Deferred Items | Risk |
+|---|---|---|
+| Step 1: Add `CrateRootNotFound` variant | None directly | Low |
+| Step 2: Add `find_crate_root` | D1 (MissingWorkspaceSection scenario) | Low -- standalone function, well-encapsulated |
+| Step 3: Unit tests for `find_crate_root` | F5 (recursive test missing) | Low -- two tests cover success and failure |
+| Step 4: Modify `build_map` in `lib.rs` | **D1**, D2, D3 | **Medium** -- error match pattern must be comprehensive |
+| Step 5: Fix silent error exits in `main.rs` | F1 (being fixed) | Low -- straightforward replacement |
+| Step 6: Update `test_missing_workspace_section` | None | Low -- test behavior inversion is correct |
+| Step 7: Add `test_single_crate_module_tree` | F5 (depth coverage) | Low -- add at least depth-2 modules |
+
+---
+
+## 5. Verification Checklist Additions
+
+Beyond the plan's verification list, add:
+
+1. **Malformed Cargo.toml scenario:** Place a `Cargo.toml` without `[workspace]` or `[package]` in a temp directory, run `cargo run -- index <dir>`. Verify clear error message on stderr, non-zero exit.
+2. **Find_workspace_root succeeds with bad members:** Create a workspace with a valid `[workspace]` section but pointing to a non-existent member. Verify `enumerate_members` error surfaces (not swallowed).
+3. **Module depth in test:** `test_single_crate_module_tree` should include at least one module at depth >= 2 (e.g., `helpers/sub.rs`).
+4. **Clippy on new code:** `cargo clippy -- -D warnings` on the new `find_crate_root` function. The function uses `content.contains("[package]")` -- verify there is no clippy lint about string contains vs. more specific parsing.
diff --git a/notes/directions/single-crate-support/directions-index.json b/notes/directions/single-crate-support/directions-index.json
new file mode 100644
index 0000000..82f1bdc
--- /dev/null
+++ b/notes/directions/single-crate-support/directions-index.json
@@ -0,0 +1,65 @@
+{
+  "meta": {
+    "title": "Single-Crate Support + Fix Silent Error Handling",
+    "source_branch": "single-crate-support"
+  },
+  "architecture_notes": [
+    "Auto-detect: try workspace discovery first; on WorkspaceRootNotFound or MissingWorkspaceSection, fall back to single-crate discovery. No new CLI flags.",
+    "Two separate functions: find_workspace_root (looks for [workspace]) and find_crate_root (looks for [package]). A combined function would stop at a workspace member's Cargo.toml.",
+    "D1 enhancement: catch MissingWorkspaceSection from enumerate_members as a fallback trigger, not just WorkspaceRootNotFound. This handles the false positive where Cargo.toml contains '[workspace]' as a comment or string literal.",
+    "validate::validate path safety confirmed \u2014 check_orphan_files joins workspace_root with crate_info.root which is always workspace-relative; safe in both multi-crate and single-crate modes.",
+    "workspace_name in single-crate mode will be the crate directory name (e.g., 'rust-workspace-map') derived from workspace_root.file_name().",
+    "find_crate_root must structurally mirror find_workspace_root: same ancestors() walk, same FileRead error mapping, same .contains() check style, same doc-comment structure with # Errors section.",
+    "Orchestration lives in lib.rs:build_map \u2014 workspace.rs remains pure with single-responsibility functions.",
+    "All changes stay within the single rust-workspace-map crate. No new crates introduced.",
+    "Library code in this phase follows ch12-04 TDD: tests claim interfaces first via tdd_interface, then implementations evolve to meet them. See the tdd-pattern.md reference for the full workflow."
+  ],
+  "known_pitfalls": [
+    "lib.rs line 18-21 does NOT import schema::Error \u2014 must add Error to the use schema::{...} import for the match pattern in TASK-pipeline-fallback.",
+    "relativize_path silently returns original path on strip_prefix failure \u2014 no error propagation, works correctly because crate_info.root paths are always workspace-relative.",
+    "workspace_name is derived from workspace_root.file_name().unwrap_or_default(). In single-crate mode this is the crate directory name, which is reasonable.",
+    "D1 enhancement nests a second match inside the Ok(root) arm: match enumerate_members, catch MissingWorkspaceSection as fallback trigger alongside the outer WorkspaceRootNotFound.",
+    "MissingWorkspaceSection false positive: Cargo.toml with '[workspace]' in a comment or string literal triggers find_workspace_root to return Ok, but enumerate_members fails. The D1 enhancement catches this.",
+    "TASK-test-single-crate-module-tree MUST exercise depth >= 2 submodule resolution (e.g., lib -> helpers/mod.rs -> helpers/sub.rs).",
+    "Do NOT touch these modules: cargo_info.rs, file_parser.rs, module_tree.rs, cross_refs.rs, render.rs, validate.rs, indexes.rs, lookup.rs.",
+    "No new CLI flags \u2014 the feature is auto-detect only. Do not add --single-crate or similar flags.",
+    "Both find_workspace_root and find_crate_root use .contains() (string match) rather than toml parsing. This is consistent with existing code. The D1 MissingWorkspaceSection fallback mitigates the false-positive risk.",
+    "unit tests in workspace.rs use the existing write_cargo_toml and setup_crate helpers (one-param version). Integration tests use the separate setup_crate(dir, lib_content) helper (two-param version)."
+  ],
+  "groups": [
+    {
+      "group_id": "group-core",
+      "task_ids": [
+        "TASK-single-crate-core-01",
+        "TASK-single-crate-core-02"
+      ],
+      "description": "All tasks modify shared schema.rs/workspace.rs files \u2014 the core discovery enhancement. TASK-single-crate-core-01 is a prerequisite for TASK-single-crate-core-02 (the error variant must exist before the function compiles).",
+      "file": "directions-single-crate-support-group-core.json"
+    },
+    {
+      "group_id": "group-fallback",
+      "task_ids": [
+        "TASK-pipeline-fallback"
+      ],
+      "description": "Pipeline fallback logic depends on group-core (needs find_crate_root and CrateRootNotFound).",
+      "file": "directions-single-crate-support-group-fallback.json"
+    },
+    {
+      "group_id": "group-silent-exit",
+      "task_ids": [
+        "TASK-silent-exit-fix"
+      ],
+      "description": "Self-contained fix for silent error exits in main.rs \u2014 no dependencies on other groups. Can run in parallel with group-fallback once group-core completes.",
+      "file": "directions-single-crate-support-group-silent-exit.json"
+    },
+    {
+      "group_id": "group-tests",
+      "task_ids": [
+        "TASK-test-existing-update",
+        "TASK-test-single-crate-module-tree"
+      ],
+      "description": "Both are integration test changes that verify the feature. They depend on all production code changes being in place. Can run in parallel with each other.",
+      "file": "directions-single-crate-support-group-tests.json"
+    }
+  ]
+}
diff --git a/notes/directions/single-crate-support/directions-single-crate-support-group-core.json b/notes/directions/single-crate-support/directions-single-crate-support-group-core.json
new file mode 100644
index 0000000..b89ad4b
--- /dev/null
+++ b/notes/directions/single-crate-support/directions-single-crate-support-group-core.json
@@ -0,0 +1,100 @@
+{
+  "meta": {
+    "title": "Single-Crate Support + Fix Silent Error Handling",
+    "source_branch": "single-crate-support"
+  },
+  "architecture_notes": [
+    "Auto-detect: try workspace discovery first; on WorkspaceRootNotFound or MissingWorkspaceSection, fall back to single-crate discovery. No new CLI flags.",
+    "Two separate functions: find_workspace_root (looks for [workspace]) and find_crate_root (looks for [package]). A combined function would stop at a workspace member's Cargo.toml.",
+    "D1 enhancement: catch MissingWorkspaceSection from enumerate_members as a fallback trigger, not just WorkspaceRootNotFound. This handles the false positive where Cargo.toml contains '[workspace]' as a comment or string literal.",
+    "validate::validate path safety confirmed \u2014 check_orphan_files joins workspace_root with crate_info.root which is always workspace-relative; safe in both multi-crate and single-crate modes.",
+    "workspace_name in single-crate mode will be the crate directory name (e.g., 'rust-workspace-map') derived from workspace_root.file_name().",
+    "find_crate_root must structurally mirror find_workspace_root: same ancestors() walk, same FileRead error mapping, same .contains() check style, same doc-comment structure with # Errors section.",
+    "Orchestration lives in lib.rs:build_map \u2014 workspace.rs remains pure with single-responsibility functions.",
+    "All changes stay within the single rust-workspace-map crate. No new crates introduced.",
+    "Library code in this phase follows ch12-04 TDD: tests claim interfaces first via tdd_interface, then implementations evolve to meet them. See the tdd-pattern.md reference for the full workflow."
+  ],
+  "known_pitfalls": [
+    "lib.rs line 18-21 does NOT import schema::Error \u2014 must add Error to the use schema::{...} import for the match pattern in TASK-pipeline-fallback.",
+    "relativize_path silently returns original path on strip_prefix failure \u2014 no error propagation, works correctly because crate_info.root paths are always workspace-relative.",
+    "workspace_name is derived from workspace_root.file_name().unwrap_or_default(). In single-crate mode this is the crate directory name, which is reasonable.",
+    "D1 enhancement nests a second match inside the Ok(root) arm: match enumerate_members, catch MissingWorkspaceSection as fallback trigger alongside the outer WorkspaceRootNotFound.",
+    "MissingWorkspaceSection false positive: Cargo.toml with '[workspace]' in a comment or string literal triggers find_workspace_root to return Ok, but enumerate_members fails. The D1 enhancement catches this.",
+    "TASK-test-single-crate-module-tree MUST exercise depth >= 2 submodule resolution (e.g., lib -> helpers/mod.rs -> helpers/sub.rs).",
+    "Do NOT touch these modules: cargo_info.rs, file_parser.rs, module_tree.rs, cross_refs.rs, render.rs, validate.rs, indexes.rs, lookup.rs.",
+    "No new CLI flags \u2014 the feature is auto-detect only. Do not add --single-crate or similar flags.",
+    "Both find_workspace_root and find_crate_root use .contains() (string match) rather than toml parsing. This is consistent with existing code. The D1 MissingWorkspaceSection fallback mitigates the false-positive risk.",
+    "unit tests in workspace.rs use the existing write_cargo_toml and setup_crate helpers (one-param version). Integration tests use the separate setup_crate(dir, lib_content) helper (two-param version)."
+  ],
+  "task_groups": [
+    {
+      "group_id": "group-core",
+      "reason": "All tasks modify shared schema.rs/workspace.rs files \u2014 the core discovery enhancement. TASK-single-crate-core-01 is a prerequisite for TASK-single-crate-core-02 (the error variant must exist before the function compiles).",
+      "tasks": [
+        "TASK-single-crate-core-01",
+        "TASK-single-crate-core-02"
+      ],
+      "depends_on_groups": []
+    }
+  ],
+  "tasks": [
+    {
+      "id": "TASK-single-crate-core-01",
+      "kind": "direct",
+      "description": "Add CrateRootNotFound variant to schema::Error enum so find_crate_root can signal discovery failure.",
+      "files_in_scope": [
+        "src/schema.rs"
+      ],
+      "changes": [
+        {
+          "path": "src/schema.rs",
+          "action": "modify",
+          "guidance": "Add a new error variant to the Error enum. Insert it after the WorkspaceRootNotFound variant (line 55), before the FileRead variant (line 57):\n\n```rust\n#[error(\"no Cargo.toml with [package] section found starting from {0}\")]\nCrateRootNotFound(PathBuf),\n```\n\nThis variant carries a PathBuf (the start_path that was searched), matching the pattern of WorkspaceRootNotFound. The thiserror attribute format is consistent with other variants. The variant is automatically reachable \u2014 schema::Error is already imported by workspace.rs via `use crate::schema::{CrateType, Error, Result}`."
+        }
+      ],
+      "wiring_checklist": [],
+      "type_reference": {
+        "CrateRootNotFound": "#[error(\"no Cargo.toml with [package] section found starting from {0}\")] CrateRootNotFound(PathBuf)"
+      },
+      "acceptance": [
+        "cargo check --workspace"
+      ],
+      "depends_on": []
+    },
+    {
+      "id": "TASK-single-crate-core-02",
+      "kind": "lib-tdd",
+      "description": "Add find_crate_root discovery function + 2 unit tests via test-driven development.",
+      "files_in_scope": [
+        "src/workspace.rs"
+      ],
+      "changes": [
+        {
+          "path": "src/workspace.rs",
+          "action": "modify",
+          "guidance": "Implement find_crate_root following the TDD cycle. The test (tdd_interface.test_code) defines the contract \u2014 write it first, confirm failure, then implement.\n\nImplementation approach: Walk up the directory tree using start_path.ancestors(). For each ancestor, check if Cargo.toml exists. If it does, read the file contents. Map I/O errors to Error::FileRead { path, source }. Check if the content contains the string \"[package]\". If found, return Ok(ancestor.to_path_buf()). If the loop completes without finding a [package] section, return Err(Error::CrateRootNotFound(start_path.to_path_buf())).\n\nPlacement: Insert after find_workspace_root (after line 25), before enumerate_members (line 35). Use the same doc-comment structure as find_workspace_root: /// description, /// # Errors section. The function must be pub.\n\nFor the unit tests: Add both test functions inside the existing #[cfg(test)] mod tests block, before its closing } at line 206. The test module already has write_cargo_toml and setup_crate (one-param) helpers via `use super::*`. No new imports needed.\n\nEdge cases handled: empty directory tree (ancestors() always includes start_path itself), Cargo.toml without [package] (continues walking), I/O error reading Cargo.toml (returns FileRead error)."
+        }
+      ],
+      "tdd_interface": {
+        "test_file": "src/workspace.rs",
+        "test_module": "tests",
+        "test_fn_name": "find_crate_root_finds_package_section",
+        "test_code": "#[test]\nfn find_crate_root_finds_package_section() {\n    let tmp = tempfile::tempdir().unwrap();\n    setup_crate(tmp.path());\n    // Walk from a nested subdirectory inside src\n    let nested = tmp.path().join(\"src\").join(\"subdir\");\n    std::fs::create_dir_all(&nested).unwrap();\n    let result = find_crate_root(&nested).unwrap();\n    assert_eq!(result, tmp.path());\n}\n\n#[test]\nfn find_crate_root_returns_err_for_no_package() {\n    let tmp = tempfile::tempdir().unwrap();\n    // No Cargo.toml at all \u2014 ancestors() walks up and finds nothing\n    let result = find_crate_root(tmp.path());\n    assert!(result.is_err());\n    match result.unwrap_err() {\n        Error::CrateRootNotFound(_) => {},\n        other => panic!(\"expected CrateRootNotFound, got {:?}\", other),\n    }\n}",
+        "signature": "pub fn find_crate_root(start_path: &Path) -> Result<PathBuf>",
+        "expected_behavior": "find_crate_root walks up from start_path through ancestors, returns the first directory whose Cargo.toml contains a [package] section. Returns CrateRootNotFound error if no Cargo.toml with [package] is found in any ancestor (including start_path itself). Returns FileRead error if a Cargo.toml exists but cannot be read."
+      },
+      "wiring_checklist": [],
+      "type_reference": {
+        "find_crate_root": "pub fn find_crate_root(start_path: &Path) -> Result<PathBuf>"
+      },
+      "acceptance": [
+        "cargo test find_crate_root_finds_package_section",
+        "cargo test find_crate_root_returns_err_for_no_package",
+        "cargo check --workspace"
+      ],
+      "depends_on": [
+        "TASK-single-crate-core-01"
+      ]
+    }
+  ]
+}
diff --git a/notes/directions/single-crate-support/directions-single-crate-support-group-fallback.json b/notes/directions/single-crate-support/directions-single-crate-support-group-fallback.json
new file mode 100644
index 0000000..f418a8e
--- /dev/null
+++ b/notes/directions/single-crate-support/directions-single-crate-support-group-fallback.json
@@ -0,0 +1,77 @@
+{
+  "meta": {
+    "title": "Single-Crate Support + Fix Silent Error Handling",
+    "source_branch": "single-crate-support"
+  },
+  "architecture_notes": [
+    "Auto-detect: try workspace discovery first; on WorkspaceRootNotFound or MissingWorkspaceSection, fall back to single-crate discovery. No new CLI flags.",
+    "Two separate functions: find_workspace_root (looks for [workspace]) and find_crate_root (looks for [package]). A combined function would stop at a workspace member's Cargo.toml.",
+    "D1 enhancement: catch MissingWorkspaceSection from enumerate_members as a fallback trigger, not just WorkspaceRootNotFound. This handles the false positive where Cargo.toml contains '[workspace]' as a comment or string literal.",
+    "validate::validate path safety confirmed \u2014 check_orphan_files joins workspace_root with crate_info.root which is always workspace-relative; safe in both multi-crate and single-crate modes.",
+    "workspace_name in single-crate mode will be the crate directory name (e.g., 'rust-workspace-map') derived from workspace_root.file_name().",
+    "find_crate_root must structurally mirror find_workspace_root: same ancestors() walk, same FileRead error mapping, same .contains() check style, same doc-comment structure with # Errors section.",
+    "Orchestration lives in lib.rs:build_map \u2014 workspace.rs remains pure with single-responsibility functions.",
+    "All changes stay within the single rust-workspace-map crate. No new crates introduced.",
+    "Library code in this phase follows ch12-04 TDD: tests claim interfaces first via tdd_interface, then implementations evolve to meet them. See the tdd-pattern.md reference for the full workflow."
+  ],
+  "known_pitfalls": [
+    "lib.rs line 18-21 does NOT import schema::Error \u2014 must add Error to the use schema::{...} import for the match pattern in TASK-pipeline-fallback.",
+    "relativize_path silently returns original path on strip_prefix failure \u2014 no error propagation, works correctly because crate_info.root paths are always workspace-relative.",
+    "workspace_name is derived from workspace_root.file_name().unwrap_or_default(). In single-crate mode this is the crate directory name, which is reasonable.",
+    "D1 enhancement nests a second match inside the Ok(root) arm: match enumerate_members, catch MissingWorkspaceSection as fallback trigger alongside the outer WorkspaceRootNotFound.",
+    "MissingWorkspaceSection false positive: Cargo.toml with '[workspace]' in a comment or string literal triggers find_workspace_root to return Ok, but enumerate_members fails. The D1 enhancement catches this.",
+    "TASK-test-single-crate-module-tree MUST exercise depth >= 2 submodule resolution (e.g., lib -> helpers/mod.rs -> helpers/sub.rs).",
+    "Do NOT touch these modules: cargo_info.rs, file_parser.rs, module_tree.rs, cross_refs.rs, render.rs, validate.rs, indexes.rs, lookup.rs.",
+    "No new CLI flags \u2014 the feature is auto-detect only. Do not add --single-crate or similar flags.",
+    "Both find_workspace_root and find_crate_root use .contains() (string match) rather than toml parsing. This is consistent with existing code. The D1 MissingWorkspaceSection fallback mitigates the false-positive risk.",
+    "unit tests in workspace.rs use the existing write_cargo_toml and setup_crate helpers (one-param version). Integration tests use the separate setup_crate(dir, lib_content) helper (two-param version)."
+  ],
+  "task_groups": [
+    {
+      "group_id": "group-fallback",
+      "reason": "Pipeline fallback logic depends on group-core (needs find_crate_root and CrateRootNotFound).",
+      "tasks": [
+        "TASK-pipeline-fallback"
+      ],
+      "depends_on_groups": [
+        "group-core"
+      ]
+    }
+  ],
+  "tasks": [
+    {
+      "id": "TASK-pipeline-fallback",
+      "kind": "direct",
+      "description": "Modify build_map in lib.rs to add fallback logic: try workspace discovery first, fall back to single-crate on WorkspaceRootNotFound or MissingWorkspaceSection.",
+      "files_in_scope": [
+        "src/lib.rs"
+      ],
+      "changes": [
+        {
+          "path": "src/lib.rs",
+          "action": "modify",
+          "guidance": "Two changes to src/lib.rs:\n\n**(1) Add Error import**: On line 18-21, the `use schema::{...}` block currently imports CrateInfo, CrateType, DiagnosticKind, ErrorEntry, ErrorSeverity, ModuleInfo, WorkspaceInfo, WorkspaceMap. Add `Error` to this import. The block should include `Error` alongside the existing types.\n\n**(2) Replace workspace discovery lines (37-38) with fallback match**: Replace the two lines:\n```rust\nlet workspace_root = workspace::find_workspace_root(&config.workspace_path)?;\nlet member_dirs = workspace::enumerate_members(&workspace_root)?;\n```\n\nWith a match expression that tries workspace discovery first, then falls back. The structure:\n```rust\nlet (workspace_root, member_dirs) = match workspace::find_workspace_root(&config.workspace_path) {\n    Ok(root) => {\n        match workspace::enumerate_members(&root) {\n            Ok(members) => (root, members),\n            Err(Error::MissingWorkspaceSection) => {\n                // [workspace] was a false positive (e.g., in a comment).\n                // Fall back to single-crate discovery.\n                let crate_dir = workspace::find_crate_root(&config.workspace_path)?;\n                (crate_dir.clone(), vec![crate_dir])\n            }\n            Err(other) => return Err(other.into()),\n        }\n    }\n    Err(Error::WorkspaceRootNotFound(_)) => {\n        let crate_dir = workspace::find_crate_root(&config.workspace_path)?;\n        (crate_dir.clone(), vec![crate_dir])\n    }\n    Err(other) => return Err(other.into()),\n};\n```\n\nKey points:\n- The inner match on enumerate_members handles the D1 false-positive case where Cargo.toml contains \"[workspace]\" as a literal string but no actual workspace section.\n- The fallback path is identical for both MissingWorkspaceSection and WorkspaceRootNotFound: find_crate_root then create a single-element vec.\n- Other errors (TomlParse, FileRead, etc.) propagate as fatal.\n- workspace_root is set to crate_dir in fallback mode; member_dirs is vec![crate_dir].\n- The rest of build_map (lines 40-171) works unchanged \u2014 it uses workspace_root and member_dirs identically regardless of discovery path.\n\nThe function signature and return type do NOT change."
+        }
+      ],
+      "wiring_checklist": [
+        {
+          "kind": "fn_call",
+          "file": "src/lib.rs",
+          "detail": "build_map calls workspace::find_crate_root in two fallback branches (MissingWorkspaceSection and WorkspaceRootNotFound)"
+        },
+        {
+          "kind": "type_annotation",
+          "file": "src/lib.rs",
+          "detail": "schema::Error is imported for the match patterns on Error::WorkspaceRootNotFound and Error::MissingWorkspaceSection"
+        }
+      ],
+      "acceptance": [
+        "cargo check --workspace",
+        "cargo clippy -- -D warnings"
+      ],
+      "depends_on": [
+        "TASK-single-crate-core-02"
+      ]
+    }
+  ]
+}
diff --git a/notes/directions/single-crate-support/directions-single-crate-support-group-silent-exit.json b/notes/directions/single-crate-support/directions-single-crate-support-group-silent-exit.json
new file mode 100644
index 0000000..b96587a
--- /dev/null
+++ b/notes/directions/single-crate-support/directions-single-crate-support-group-silent-exit.json
@@ -0,0 +1,62 @@
+{
+  "meta": {
+    "title": "Single-Crate Support + Fix Silent Error Handling",
+    "source_branch": "single-crate-support"
+  },
+  "architecture_notes": [
+    "Auto-detect: try workspace discovery first; on WorkspaceRootNotFound or MissingWorkspaceSection, fall back to single-crate discovery. No new CLI flags.",
+    "Two separate functions: find_workspace_root (looks for [workspace]) and find_crate_root (looks for [package]). A combined function would stop at a workspace member's Cargo.toml.",
+    "D1 enhancement: catch MissingWorkspaceSection from enumerate_members as a fallback trigger, not just WorkspaceRootNotFound. This handles the false positive where Cargo.toml contains '[workspace]' as a comment or string literal.",
+    "validate::validate path safety confirmed \u2014 check_orphan_files joins workspace_root with crate_info.root which is always workspace-relative; safe in both multi-crate and single-crate modes.",
+    "workspace_name in single-crate mode will be the crate directory name (e.g., 'rust-workspace-map') derived from workspace_root.file_name().",
+    "find_crate_root must structurally mirror find_workspace_root: same ancestors() walk, same FileRead error mapping, same .contains() check style, same doc-comment structure with # Errors section.",
+    "Orchestration lives in lib.rs:build_map \u2014 workspace.rs remains pure with single-responsibility functions.",
+    "All changes stay within the single rust-workspace-map crate. No new crates introduced.",
+    "Library code in this phase follows ch12-04 TDD: tests claim interfaces first via tdd_interface, then implementations evolve to meet them. See the tdd-pattern.md reference for the full workflow."
+  ],
+  "known_pitfalls": [
+    "lib.rs line 18-21 does NOT import schema::Error \u2014 must add Error to the use schema::{...} import for the match pattern in TASK-pipeline-fallback.",
+    "relativize_path silently returns original path on strip_prefix failure \u2014 no error propagation, works correctly because crate_info.root paths are always workspace-relative.",
+    "workspace_name is derived from workspace_root.file_name().unwrap_or_default(). In single-crate mode this is the crate directory name, which is reasonable.",
+    "D1 enhancement nests a second match inside the Ok(root) arm: match enumerate_members, catch MissingWorkspaceSection as fallback trigger alongside the outer WorkspaceRootNotFound.",
+    "MissingWorkspaceSection false positive: Cargo.toml with '[workspace]' in a comment or string literal triggers find_workspace_root to return Ok, but enumerate_members fails. The D1 enhancement catches this.",
+    "TASK-test-single-crate-module-tree MUST exercise depth >= 2 submodule resolution (e.g., lib -> helpers/mod.rs -> helpers/sub.rs).",
+    "Do NOT touch these modules: cargo_info.rs, file_parser.rs, module_tree.rs, cross_refs.rs, render.rs, validate.rs, indexes.rs, lookup.rs.",
+    "No new CLI flags \u2014 the feature is auto-detect only. Do not add --single-crate or similar flags.",
+    "Both find_workspace_root and find_crate_root use .contains() (string match) rather than toml parsing. This is consistent with existing code. The D1 MissingWorkspaceSection fallback mitigates the false-positive risk.",
+    "unit tests in workspace.rs use the existing write_cargo_toml and setup_crate helpers (one-param version). Integration tests use the separate setup_crate(dir, lib_content) helper (two-param version)."
+  ],
+  "task_groups": [
+    {
+      "group_id": "group-silent-exit",
+      "reason": "Self-contained fix for silent error exits in main.rs \u2014 no dependencies on other groups. Can run in parallel with group-fallback once group-core completes.",
+      "tasks": [
+        "TASK-silent-exit-fix"
+      ],
+      "depends_on_groups": []
+    }
+  ],
+  "tasks": [
+    {
+      "id": "TASK-silent-exit-fix",
+      "kind": "direct",
+      "description": "Fix 4 silent error exit sites in main.rs to print error messages to stderr before exiting.",
+      "files_in_scope": [
+        "src/main.rs"
+      ],
+      "changes": [
+        {
+          "path": "src/main.rs",
+          "action": "modify",
+          "guidance": "Fix four silent error exit sites in main.rs where errors are discarded without printing to stderr:\n\n**(1) File write error (lines 88-90)**: Change:\n```rust\nif rust_workspace_map::render::render_to_writer(&map, writer).is_err() {\n    std::process::exit(1);\n}\n```\nTo:\n```rust\nif let Err(e) = rust_workspace_map::render::render_to_writer(&map, writer) {\n    eprintln!(\"error writing JSON to file: {e}\");\n    std::process::exit(1);\n}\n```\nUse plain `{e}` for simple I/O errors.\n\n**(2) Stdout write error (lines 93-95)**: Same pattern, change to `if let Err(e)` with `eprintln!(\"error writing JSON to stdout: {e}\")`.\n\n**(3) Pipeline error (lines 102-104)**: Change `Err(_)` to `Err(e)` and add `eprintln!(\"error: {e:#}\")` before `exit(1)`. Use `{e:#}` (alternate format) for anyhow error chains \u2014 this produces multi-line output with context.\n\n**(4) Lookup pipeline error (lines 165-167)**: Same pattern as (3) \u2014 `Err(e) => { eprintln!(\"error: {e:#}\"); std::process::exit(1); }`.\n\n**Do NOT change**:\n- Validation exit (lines 98-100): exit code 2 is intentional for validation findings; stderr is intentionally silent.\n- Lookup serialization errors (lines 123-128, 140-145): these already print messages to stderr before exiting."
+        }
+      ],
+      "wiring_checklist": [],
+      "acceptance": [
+        "cargo check --workspace",
+        "cargo clippy -- -D warnings"
+      ],
+      "depends_on": []
+    }
+  ]
+}
diff --git a/notes/directions/single-crate-support/directions-single-crate-support-group-tests.json b/notes/directions/single-crate-support/directions-single-crate-support-group-tests.json
new file mode 100644
index 0000000..70105d2
--- /dev/null
+++ b/notes/directions/single-crate-support/directions-single-crate-support-group-tests.json
@@ -0,0 +1,88 @@
+{
+  "meta": {
+    "title": "Single-Crate Support + Fix Silent Error Handling",
+    "source_branch": "single-crate-support"
+  },
+  "architecture_notes": [
+    "Auto-detect: try workspace discovery first; on WorkspaceRootNotFound or MissingWorkspaceSection, fall back to single-crate discovery. No new CLI flags.",
+    "Two separate functions: find_workspace_root (looks for [workspace]) and find_crate_root (looks for [package]). A combined function would stop at a workspace member's Cargo.toml.",
+    "D1 enhancement: catch MissingWorkspaceSection from enumerate_members as a fallback trigger, not just WorkspaceRootNotFound. This handles the false positive where Cargo.toml contains '[workspace]' as a comment or string literal.",
+    "validate::validate path safety confirmed \u2014 check_orphan_files joins workspace_root with crate_info.root which is always workspace-relative; safe in both multi-crate and single-crate modes.",
+    "workspace_name in single-crate mode will be the crate directory name (e.g., 'rust-workspace-map') derived from workspace_root.file_name().",
+    "find_crate_root must structurally mirror find_workspace_root: same ancestors() walk, same FileRead error mapping, same .contains() check style, same doc-comment structure with # Errors section.",
+    "Orchestration lives in lib.rs:build_map \u2014 workspace.rs remains pure with single-responsibility functions.",
+    "All changes stay within the single rust-workspace-map crate. No new crates introduced.",
+    "Library code in this phase follows ch12-04 TDD: tests claim interfaces first via tdd_interface, then implementations evolve to meet them. See the tdd-pattern.md reference for the full workflow."
+  ],
+  "known_pitfalls": [
+    "lib.rs line 18-21 does NOT import schema::Error \u2014 must add Error to the use schema::{...} import for the match pattern in TASK-pipeline-fallback.",
+    "relativize_path silently returns original path on strip_prefix failure \u2014 no error propagation, works correctly because crate_info.root paths are always workspace-relative.",
+    "workspace_name is derived from workspace_root.file_name().unwrap_or_default(). In single-crate mode this is the crate directory name, which is reasonable.",
+    "D1 enhancement nests a second match inside the Ok(root) arm: match enumerate_members, catch MissingWorkspaceSection as fallback trigger alongside the outer WorkspaceRootNotFound.",
+    "MissingWorkspaceSection false positive: Cargo.toml with '[workspace]' in a comment or string literal triggers find_workspace_root to return Ok, but enumerate_members fails. The D1 enhancement catches this.",
+    "TASK-test-single-crate-module-tree MUST exercise depth >= 2 submodule resolution (e.g., lib -> helpers/mod.rs -> helpers/sub.rs).",
+    "Do NOT touch these modules: cargo_info.rs, file_parser.rs, module_tree.rs, cross_refs.rs, render.rs, validate.rs, indexes.rs, lookup.rs.",
+    "No new CLI flags \u2014 the feature is auto-detect only. Do not add --single-crate or similar flags.",
+    "Both find_workspace_root and find_crate_root use .contains() (string match) rather than toml parsing. This is consistent with existing code. The D1 MissingWorkspaceSection fallback mitigates the false-positive risk.",
+    "unit tests in workspace.rs use the existing write_cargo_toml and setup_crate helpers (one-param version). Integration tests use the separate setup_crate(dir, lib_content) helper (two-param version)."
+  ],
+  "task_groups": [
+    {
+      "group_id": "group-tests",
+      "reason": "Both are integration test changes that verify the feature. They depend on all production code changes being in place. Can run in parallel with each other.",
+      "tasks": [
+        "TASK-test-existing-update",
+        "TASK-test-single-crate-module-tree"
+      ],
+      "depends_on_groups": [
+        "group-fallback"
+      ]
+    }
+  ],
+  "tasks": [
+    {
+      "id": "TASK-test-existing-update",
+      "kind": "direct",
+      "description": "Rename and invert test_missing_workspace_section to test_single_crate_without_workspace \u2014 the existing test expects non-zero exit for a standalone crate, which is now a supported scenario.",
+      "files_in_scope": [
+        "tests/integration_test.rs"
+      ],
+      "changes": [
+        {
+          "path": "tests/integration_test.rs",
+          "action": "modify",
+          "guidance": "Rename `test_missing_workspace_section` (line 200) to `test_single_crate_without_workspace` and invert the test logic to match the new single-crate support behavior.\n\nReplace the test body (lines 199-223) with:\n\n1. Create a temp directory using `tempfile::tempdir().unwrap()`. Create a subdirectory `let crate_dir = root.join(\"standalone\");`.\n2. Use `setup_crate(&crate_dir, \"pub struct Standalone { pub x: i32 }\")` \u2014 the existing two-param helper writes both a [package]-only Cargo.toml (with name=\"standalone\") and src/lib.rs for that subdirectory.\n3. Run the index command via `run_index(crate_dir.to_str().unwrap())`.\n4. Assert exit 0: `assert!(output.status.success(), ...)`.\n5. Parse the JSON: `let json = parse_output(&output)`.\n6. Extract crates: `let crates = extract_array(&json, \"crates\")`.\n7. Assert exactly one crate (len == 1).\n8. Assert the crate name is \"standalone\": `crates[0][\"name\"] == \"standalone\"`.\n9. Assert the crate has at least one module and that \"Standalone\" appears in public items \u2014 iterate over modules' publicItems and find a struct named \"Standalone\".\n\nThe test name change from `test_missing_workspace_section` to `test_single_crate_without_workspace` signals the behavior change: what was an error condition is now a supported scenario."
+        }
+      ],
+      "wiring_checklist": [],
+      "acceptance": [
+        "cargo test --test integration_test test_single_crate_without_workspace"
+      ],
+      "depends_on": [
+        "TASK-pipeline-fallback"
+      ]
+    },
+    {
+      "id": "TASK-test-single-crate-module-tree",
+      "kind": "direct",
+      "description": "Add test_single_crate_module_tree integration test with depth >= 2 submodule resolution in single-crate mode.",
+      "files_in_scope": [
+        "tests/integration_test.rs"
+      ],
+      "changes": [
+        {
+          "path": "tests/integration_test.rs",
+          "action": "modify",
+          "guidance": "Add a new integration test `test_single_crate_module_tree` that verifies module tree construction works in single-crate mode at depth >= 2.\n\nImplementation:\n\n1. Create a temp directory and a subdirectory with a known name: `let tmp = tempfile::tempdir().unwrap(); let root = tmp.path().join(\"single-crate\"); std::fs::create_dir_all(&root).unwrap();`\n2. Use `setup_crate(&root, \"pub mod helpers;\")` to create a standalone (non-workspace) crate with lib.rs declaring a submodule.\n3. Create `src/helpers/mod.rs` with `pub mod sub;` (a module at depth 1 declaring depth 2).\n4. Create `src/helpers/sub.rs` with `pub fn assist() {}` \u2014 this is the depth-2 module with a public item.\n5. Run the index command: `let output = run_index(root.to_str().unwrap());`\n6. Assert exit 0: `assert!(output.status.success(), ...)`\n7. Parse JSON: `let json = parse_output(&output);`\n8. Extract crates array and assert length 1.\n9. Assert the crate name is `\"single-crate\"` (derived from the dir basename by setup_crate).\n10. Collect module paths from the crate and assert all three expected paths exist:\n    - `\"single-crate\"` (root module)\n    - `\"single-crate::helpers\"` (depth 1)\n    - `\"single-crate::helpers::sub\"` (depth 2)\n11. Find the `helpers::sub` module and verify it has the `assist` function in its publicItems array (kind = \"fn\", name = \"assist\").\n\nUse the existing helpers: `setup_crate(dir, lib_content)`, `run_index(path)`, `parse_output(output)`, `extract_array(val, key)`. Follow the pattern established by `test_deeply_nested_modules` (lines 284-329).\n\nThe maximum depth for this test (2) exercises recursive module resolution in single-crate mode \u2014 the key risk flagged by the architectural review. The existing `test_deeply_nested_modules` goes to depth 4 but uses workspace mode with `members = [\".\"]` \u2014 this new test confirms the same depth works in pure single-crate mode without any `[workspace]` section."
+        }
+      ],
+      "wiring_checklist": [],
+      "acceptance": [
+        "cargo test --test integration_test test_single_crate_module_tree"
+      ],
+      "depends_on": [
+        "TASK-pipeline-fallback"
+      ]
+    }
+  ]
+}
diff --git a/notes/directions/single-crate-support/directions.json b/notes/directions/single-crate-support/directions.json
new file mode 100644
index 0000000..0135dbe
--- /dev/null
+++ b/notes/directions/single-crate-support/directions.json
@@ -0,0 +1,207 @@
+{
+  "meta": {
+    "title": "Single-Crate Support + Fix Silent Error Handling",
+    "source_branch": "single-crate-support"
+  },
+  "architecture_notes": [
+    "Auto-detect: try workspace discovery first; on WorkspaceRootNotFound or MissingWorkspaceSection, fall back to single-crate discovery. No new CLI flags.",
+    "Two separate functions: find_workspace_root (looks for [workspace]) and find_crate_root (looks for [package]). A combined function would stop at a workspace member's Cargo.toml.",
+    "D1 enhancement: catch MissingWorkspaceSection from enumerate_members as a fallback trigger, not just WorkspaceRootNotFound. This handles the false positive where Cargo.toml contains '[workspace]' as a comment or string literal.",
+    "validate::validate path safety confirmed — check_orphan_files joins workspace_root with crate_info.root which is always workspace-relative; safe in both multi-crate and single-crate modes.",
+    "workspace_name in single-crate mode will be the crate directory name (e.g., 'rust-workspace-map') derived from workspace_root.file_name().",
+    "find_crate_root must structurally mirror find_workspace_root: same ancestors() walk, same FileRead error mapping, same .contains() check style, same doc-comment structure with # Errors section.",
+    "Orchestration lives in lib.rs:build_map — workspace.rs remains pure with single-responsibility functions.",
+    "All changes stay within the single rust-workspace-map crate. No new crates introduced.",
+    "Library code in this phase follows ch12-04 TDD: tests claim interfaces first via tdd_interface, then implementations evolve to meet them. See the tdd-pattern.md reference for the full workflow."
+
+  ],
+  "known_pitfalls": [
+    "lib.rs line 18-21 does NOT import schema::Error — must add Error to the use schema::{...} import for the match pattern in TASK-pipeline-fallback.",
+    "relativize_path silently returns original path on strip_prefix failure — no error propagation, works correctly because crate_info.root paths are always workspace-relative.",
+    "workspace_name is derived from workspace_root.file_name().unwrap_or_default(). In single-crate mode this is the crate directory name, which is reasonable.",
+    "D1 enhancement nests a second match inside the Ok(root) arm: match enumerate_members, catch MissingWorkspaceSection as fallback trigger alongside the outer WorkspaceRootNotFound.",
+    "MissingWorkspaceSection false positive: Cargo.toml with '[workspace]' in a comment or string literal triggers find_workspace_root to return Ok, but enumerate_members fails. The D1 enhancement catches this.",
+    "TASK-test-single-crate-module-tree MUST exercise depth >= 2 submodule resolution (e.g., lib -> helpers/mod.rs -> helpers/sub.rs).",
+    "Do NOT touch these modules: cargo_info.rs, file_parser.rs, module_tree.rs, cross_refs.rs, render.rs, validate.rs, indexes.rs, lookup.rs.",
+    "No new CLI flags — the feature is auto-detect only. Do not add --single-crate or similar flags.",
+    "Both find_workspace_root and find_crate_root use .contains() (string match) rather than toml parsing. This is consistent with existing code. The D1 MissingWorkspaceSection fallback mitigates the false-positive risk.",
+    "unit tests in workspace.rs use the existing write_cargo_toml and setup_crate helpers (one-param version). Integration tests use the separate setup_crate(dir, lib_content) helper (two-param version)."
+  ],
+  "task_groups": [
+    {
+      "group_id": "group-core",
+      "reason": "All tasks modify shared schema.rs/workspace.rs files — the core discovery enhancement. TASK-single-crate-core-01 is a prerequisite for TASK-single-crate-core-02 (the error variant must exist before the function compiles).",
+      "tasks": ["TASK-single-crate-core-01", "TASK-single-crate-core-02"],
+      "depends_on_groups": []
+    },
+    {
+      "group_id": "group-fallback",
+      "reason": "Pipeline fallback logic depends on group-core (needs find_crate_root and CrateRootNotFound).",
+      "tasks": ["TASK-pipeline-fallback"],
+      "depends_on_groups": ["group-core"]
+    },
+    {
+      "group_id": "group-silent-exit",
+      "reason": "Self-contained fix for silent error exits in main.rs — no dependencies on other groups. Can run in parallel with group-fallback once group-core completes.",
+      "tasks": ["TASK-silent-exit-fix"],
+      "depends_on_groups": []
+    },
+    {
+      "group_id": "group-tests",
+      "reason": "Both are integration test changes that verify the feature. They depend on all production code changes being in place. Can run in parallel with each other.",
+      "tasks": ["TASK-test-existing-update", "TASK-test-single-crate-module-tree"],
+      "depends_on_groups": ["group-fallback"]
+    }
+  ],
+  "tasks": [
+    {
+      "id": "TASK-single-crate-core-01",
+      "kind": "direct",
+      "description": "Add CrateRootNotFound variant to schema::Error enum so find_crate_root can signal discovery failure.",
+      "files_in_scope": [
+        "src/schema.rs"
+      ],
+      "changes": [
+        {
+          "path": "src/schema.rs",
+          "action": "modify",
+          "guidance": "Add a new error variant to the Error enum. Insert it after the WorkspaceRootNotFound variant (line 55), before the FileRead variant (line 57):\n\n```rust\n#[error(\"no Cargo.toml with [package] section found starting from {0}\")]\nCrateRootNotFound(PathBuf),\n```\n\nThis variant carries a PathBuf (the start_path that was searched), matching the pattern of WorkspaceRootNotFound. The thiserror attribute format is consistent with other variants. The variant is automatically reachable — schema::Error is already imported by workspace.rs via `use crate::schema::{CrateType, Error, Result}`."
+        }
+      ],
+      "wiring_checklist": [],
+      "type_reference": {
+        "CrateRootNotFound": "#[error(\"no Cargo.toml with [package] section found starting from {0}\")] CrateRootNotFound(PathBuf)"
+      },
+      "acceptance": [
+        "cargo check --workspace"
+      ],
+      "depends_on": []
+    },
+    {
+      "id": "TASK-single-crate-core-02",
+      "kind": "lib-tdd",
+      "description": "Add find_crate_root discovery function + 2 unit tests via test-driven development.",
+      "files_in_scope": [
+        "src/workspace.rs"
+      ],
+      "changes": [
+        {
+          "path": "src/workspace.rs",
+          "action": "modify",
+          "guidance": "Implement find_crate_root following the TDD cycle. The test (tdd_interface.test_code) defines the contract — write it first, confirm failure, then implement.\n\nImplementation approach: Walk up the directory tree using start_path.ancestors(). For each ancestor, check if Cargo.toml exists. If it does, read the file contents. Map I/O errors to Error::FileRead { path, source }. Check if the content contains the string \"[package]\". If found, return Ok(ancestor.to_path_buf()). If the loop completes without finding a [package] section, return Err(Error::CrateRootNotFound(start_path.to_path_buf())).\n\nPlacement: Insert after find_workspace_root (after line 25), before enumerate_members (line 35). Use the same doc-comment structure as find_workspace_root: /// description, /// # Errors section. The function must be pub.\n\nFor the unit tests: Add both test functions inside the existing #[cfg(test)] mod tests block, before its closing } at line 206. The test module already has write_cargo_toml and setup_crate (one-param) helpers via `use super::*`. No new imports needed.\n\nEdge cases handled: empty directory tree (ancestors() always includes start_path itself), Cargo.toml without [package] (continues walking), I/O error reading Cargo.toml (returns FileRead error)."
+        }
+      ],
+      "tdd_interface": {
+        "test_file": "src/workspace.rs",
+        "test_module": "tests",
+        "test_fn_name": "find_crate_root_finds_package_section",
+        "test_code": "#[test]\nfn find_crate_root_finds_package_section() {\n    let tmp = tempfile::tempdir().unwrap();\n    setup_crate(tmp.path());\n    // Walk from a nested subdirectory inside src\n    let nested = tmp.path().join(\"src\").join(\"subdir\");\n    std::fs::create_dir_all(&nested).unwrap();\n    let result = find_crate_root(&nested).unwrap();\n    assert_eq!(result, tmp.path());\n}\n\n#[test]\nfn find_crate_root_returns_err_for_no_package() {\n    let tmp = tempfile::tempdir().unwrap();\n    // No Cargo.toml at all — ancestors() walks up and finds nothing\n    let result = find_crate_root(tmp.path());\n    assert!(result.is_err());\n    match result.unwrap_err() {\n        Error::CrateRootNotFound(_) => {},\n        other => panic!(\"expected CrateRootNotFound, got {:?}\", other),\n    }\n}",
+        "signature": "pub fn find_crate_root(start_path: &Path) -> Result<PathBuf>",
+        "expected_behavior": "find_crate_root walks up from start_path through ancestors, returns the first directory whose Cargo.toml contains a [package] section. Returns CrateRootNotFound error if no Cargo.toml with [package] is found in any ancestor (including start_path itself). Returns FileRead error if a Cargo.toml exists but cannot be read."
+      },
+      "wiring_checklist": [],
+      "type_reference": {
+        "find_crate_root": "pub fn find_crate_root(start_path: &Path) -> Result<PathBuf>"
+      },
+      "acceptance": [
+        "cargo test find_crate_root_finds_package_section",
+        "cargo test find_crate_root_returns_err_for_no_package",
+        "cargo check --workspace"
+      ],
+      "depends_on": ["TASK-single-crate-core-01"]
+    },
+    {
+      "id": "TASK-pipeline-fallback",
+      "kind": "direct",
+      "description": "Modify build_map in lib.rs to add fallback logic: try workspace discovery first, fall back to single-crate on WorkspaceRootNotFound or MissingWorkspaceSection.",
+      "files_in_scope": [
+        "src/lib.rs"
+      ],
+      "changes": [
+        {
+          "path": "src/lib.rs",
+          "action": "modify",
+          "guidance": "Two changes to src/lib.rs:\n\n**(1) Add Error import**: On line 18-21, the `use schema::{...}` block currently imports CrateInfo, CrateType, DiagnosticKind, ErrorEntry, ErrorSeverity, ModuleInfo, WorkspaceInfo, WorkspaceMap. Add `Error` to this import. The block should include `Error` alongside the existing types.\n\n**(2) Replace workspace discovery lines (37-38) with fallback match**: Replace the two lines:\n```rust\nlet workspace_root = workspace::find_workspace_root(&config.workspace_path)?;\nlet member_dirs = workspace::enumerate_members(&workspace_root)?;\n```\n\nWith a match expression that tries workspace discovery first, then falls back. The structure:\n```rust\nlet (workspace_root, member_dirs) = match workspace::find_workspace_root(&config.workspace_path) {\n    Ok(root) => {\n        match workspace::enumerate_members(&root) {\n            Ok(members) => (root, members),\n            Err(Error::MissingWorkspaceSection) => {\n                // [workspace] was a false positive (e.g., in a comment).\n                // Fall back to single-crate discovery.\n                let crate_dir = workspace::find_crate_root(&config.workspace_path)?;\n                (crate_dir.clone(), vec![crate_dir])\n            }\n            Err(other) => return Err(other.into()),\n        }\n    }\n    Err(Error::WorkspaceRootNotFound(_)) => {\n        let crate_dir = workspace::find_crate_root(&config.workspace_path)?;\n        (crate_dir.clone(), vec![crate_dir])\n    }\n    Err(other) => return Err(other.into()),\n};\n```\n\nKey points:\n- The inner match on enumerate_members handles the D1 false-positive case where Cargo.toml contains \"[workspace]\" as a literal string but no actual workspace section.\n- The fallback path is identical for both MissingWorkspaceSection and WorkspaceRootNotFound: find_crate_root then create a single-element vec.\n- Other errors (TomlParse, FileRead, etc.) propagate as fatal.\n- workspace_root is set to crate_dir in fallback mode; member_dirs is vec![crate_dir].\n- The rest of build_map (lines 40-171) works unchanged — it uses workspace_root and member_dirs identically regardless of discovery path.\n\nThe function signature and return type do NOT change."
+        }
+      ],
+      "wiring_checklist": [
+        {
+          "kind": "fn_call",
+          "file": "src/lib.rs",
+          "detail": "build_map calls workspace::find_crate_root in two fallback branches (MissingWorkspaceSection and WorkspaceRootNotFound)"
+        },
+        {
+          "kind": "type_annotation",
+          "file": "src/lib.rs",
+          "detail": "schema::Error is imported for the match patterns on Error::WorkspaceRootNotFound and Error::MissingWorkspaceSection"
+        }
+      ],
+      "acceptance": [
+        "cargo check --workspace",
+        "cargo clippy -- -D warnings"
+      ],
+      "depends_on": ["TASK-single-crate-core-02"]
+    },
+    {
+      "id": "TASK-silent-exit-fix",
+      "kind": "direct",
+      "description": "Fix 4 silent error exit sites in main.rs to print error messages to stderr before exiting.",
+      "files_in_scope": [
+        "src/main.rs"
+      ],
+      "changes": [
+        {
+          "path": "src/main.rs",
+          "action": "modify",
+          "guidance": "Fix four silent error exit sites in main.rs where errors are discarded without printing to stderr:\n\n**(1) File write error (lines 88-90)**: Change:\n```rust\nif rust_workspace_map::render::render_to_writer(&map, writer).is_err() {\n    std::process::exit(1);\n}\n```\nTo:\n```rust\nif let Err(e) = rust_workspace_map::render::render_to_writer(&map, writer) {\n    eprintln!(\"error writing JSON to file: {e}\");\n    std::process::exit(1);\n}\n```\nUse plain `{e}` for simple I/O errors.\n\n**(2) Stdout write error (lines 93-95)**: Same pattern, change to `if let Err(e)` with `eprintln!(\"error writing JSON to stdout: {e}\")`.\n\n**(3) Pipeline error (lines 102-104)**: Change `Err(_)` to `Err(e)` and add `eprintln!(\"error: {e:#}\")` before `exit(1)`. Use `{e:#}` (alternate format) for anyhow error chains — this produces multi-line output with context.\n\n**(4) Lookup pipeline error (lines 165-167)**: Same pattern as (3) — `Err(e) => { eprintln!(\"error: {e:#}\"); std::process::exit(1); }`.\n\n**Do NOT change**:\n- Validation exit (lines 98-100): exit code 2 is intentional for validation findings; stderr is intentionally silent.\n- Lookup serialization errors (lines 123-128, 140-145): these already print messages to stderr before exiting."
+        }
+      ],
+      "wiring_checklist": [],
+      "acceptance": [
+        "cargo check --workspace",
+        "cargo clippy -- -D warnings"
+      ],
+      "depends_on": []
+    },
+    {
+      "id": "TASK-test-existing-update",
+      "kind": "direct",
+      "description": "Rename and invert test_missing_workspace_section to test_single_crate_without_workspace — the existing test expects non-zero exit for a standalone crate, which is now a supported scenario.",
+      "files_in_scope": [
+        "tests/integration_test.rs"
+      ],
+      "changes": [
+        {
+          "path": "tests/integration_test.rs",
+          "action": "modify",
+          "guidance": "Rename `test_missing_workspace_section` (line 200) to `test_single_crate_without_workspace` and invert the test logic to match the new single-crate support behavior.\n\nReplace the test body (lines 199-223) with:\n\n1. Create a temp directory using `tempfile::tempdir().unwrap()`. Create a subdirectory `let crate_dir = root.join(\"standalone\");`.\n2. Use `setup_crate(&crate_dir, \"pub struct Standalone { pub x: i32 }\")` — the existing two-param helper writes both a [package]-only Cargo.toml (with name=\"standalone\") and src/lib.rs for that subdirectory.\n3. Run the index command via `run_index(crate_dir.to_str().unwrap())`.\n4. Assert exit 0: `assert!(output.status.success(), ...)`.\n5. Parse the JSON: `let json = parse_output(&output)`.\n6. Extract crates: `let crates = extract_array(&json, \"crates\")`.\n7. Assert exactly one crate (len == 1).\n8. Assert the crate name is \"standalone\": `crates[0][\"name\"] == \"standalone\"`.\n9. Assert the crate has at least one module and that \"Standalone\" appears in public items — iterate over modules' publicItems and find a struct named \"Standalone\".\n\nThe test name change from `test_missing_workspace_section` to `test_single_crate_without_workspace` signals the behavior change: what was an error condition is now a supported scenario."
+        }
+      ],
+      "wiring_checklist": [],
+      "acceptance": [
+        "cargo test --test integration_test test_single_crate_without_workspace"
+      ],
+      "depends_on": ["TASK-pipeline-fallback"]
+    },
+    {
+      "id": "TASK-test-single-crate-module-tree",
+      "kind": "direct",
+      "description": "Add test_single_crate_module_tree integration test with depth >= 2 submodule resolution in single-crate mode.",
+      "files_in_scope": [
+        "tests/integration_test.rs"
+      ],
+      "changes": [
+        {
+          "path": "tests/integration_test.rs",
+          "action": "modify",
+          "guidance": "Add a new integration test `test_single_crate_module_tree` that verifies module tree construction works in single-crate mode at depth >= 2.\n\nImplementation:\n\n1. Create a temp directory and a subdirectory with a known name: `let tmp = tempfile::tempdir().unwrap(); let root = tmp.path().join(\"single-crate\"); std::fs::create_dir_all(&root).unwrap();`\n2. Use `setup_crate(&root, \"pub mod helpers;\")` to create a standalone (non-workspace) crate with lib.rs declaring a submodule.\n3. Create `src/helpers/mod.rs` with `pub mod sub;` (a module at depth 1 declaring depth 2).\n4. Create `src/helpers/sub.rs` with `pub fn assist() {}` — this is the depth-2 module with a public item.\n5. Run the index command: `let output = run_index(root.to_str().unwrap());`\n6. Assert exit 0: `assert!(output.status.success(), ...)`\n7. Parse JSON: `let json = parse_output(&output);`\n8. Extract crates array and assert length 1.\n9. Assert the crate name is `\"single-crate\"` (derived from the dir basename by setup_crate).\n10. Collect module paths from the crate and assert all three expected paths exist:\n    - `\"single-crate\"` (root module)\n    - `\"single-crate::helpers\"` (depth 1)\n    - `\"single-crate::helpers::sub\"` (depth 2)\n11. Find the `helpers::sub` module and verify it has the `assist` function in its publicItems array (kind = \"fn\", name = \"assist\").\n\nUse the existing helpers: `setup_crate(dir, lib_content)`, `run_index(path)`, `parse_output(output)`, `extract_array(val, key)`. Follow the pattern established by `test_deeply_nested_modules` (lines 284-329).\n\nThe maximum depth for this test (2) exercises recursive module resolution in single-crate mode — the key risk flagged by the architectural review. The existing `test_deeply_nested_modules` goes to depth 4 but uses workspace mode with `members = [\".\"]` — this new test confirms the same depth works in pure single-crate mode without any `[workspace]` section."
+        }
+      ],
+      "wiring_checklist": [],
+      "acceptance": [
+        "cargo test --test integration_test test_single_crate_module_tree"
+      ],
+      "depends_on": ["TASK-pipeline-fallback"]
+    }
+  ]
+}
diff --git a/notes/directions/single-crate-support/draft-directions.json b/notes/directions/single-crate-support/draft-directions.json
new file mode 100644
index 0000000..0135dbe
--- /dev/null
+++ b/notes/directions/single-crate-support/draft-directions.json
@@ -0,0 +1,207 @@
+{
+  "meta": {
+    "title": "Single-Crate Support + Fix Silent Error Handling",
+    "source_branch": "single-crate-support"
+  },
+  "architecture_notes": [
+    "Auto-detect: try workspace discovery first; on WorkspaceRootNotFound or MissingWorkspaceSection, fall back to single-crate discovery. No new CLI flags.",
+    "Two separate functions: find_workspace_root (looks for [workspace]) and find_crate_root (looks for [package]). A combined function would stop at a workspace member's Cargo.toml.",
+    "D1 enhancement: catch MissingWorkspaceSection from enumerate_members as a fallback trigger, not just WorkspaceRootNotFound. This handles the false positive where Cargo.toml contains '[workspace]' as a comment or string literal.",
+    "validate::validate path safety confirmed — check_orphan_files joins workspace_root with crate_info.root which is always workspace-relative; safe in both multi-crate and single-crate modes.",
+    "workspace_name in single-crate mode will be the crate directory name (e.g., 'rust-workspace-map') derived from workspace_root.file_name().",
+    "find_crate_root must structurally mirror find_workspace_root: same ancestors() walk, same FileRead error mapping, same .contains() check style, same doc-comment structure with # Errors section.",
+    "Orchestration lives in lib.rs:build_map — workspace.rs remains pure with single-responsibility functions.",
+    "All changes stay within the single rust-workspace-map crate. No new crates introduced.",
+    "Library code in this phase follows ch12-04 TDD: tests claim interfaces first via tdd_interface, then implementations evolve to meet them. See the tdd-pattern.md reference for the full workflow."
+
+  ],
+  "known_pitfalls": [
+    "lib.rs line 18-21 does NOT import schema::Error — must add Error to the use schema::{...} import for the match pattern in TASK-pipeline-fallback.",
+    "relativize_path silently returns original path on strip_prefix failure — no error propagation, works correctly because crate_info.root paths are always workspace-relative.",
+    "workspace_name is derived from workspace_root.file_name().unwrap_or_default(). In single-crate mode this is the crate directory name, which is reasonable.",
+    "D1 enhancement nests a second match inside the Ok(root) arm: match enumerate_members, catch MissingWorkspaceSection as fallback trigger alongside the outer WorkspaceRootNotFound.",
+    "MissingWorkspaceSection false positive: Cargo.toml with '[workspace]' in a comment or string literal triggers find_workspace_root to return Ok, but enumerate_members fails. The D1 enhancement catches this.",
+    "TASK-test-single-crate-module-tree MUST exercise depth >= 2 submodule resolution (e.g., lib -> helpers/mod.rs -> helpers/sub.rs).",
+    "Do NOT touch these modules: cargo_info.rs, file_parser.rs, module_tree.rs, cross_refs.rs, render.rs, validate.rs, indexes.rs, lookup.rs.",
+    "No new CLI flags — the feature is auto-detect only. Do not add --single-crate or similar flags.",
+    "Both find_workspace_root and find_crate_root use .contains() (string match) rather than toml parsing. This is consistent with existing code. The D1 MissingWorkspaceSection fallback mitigates the false-positive risk.",
+    "unit tests in workspace.rs use the existing write_cargo_toml and setup_crate helpers (one-param version). Integration tests use the separate setup_crate(dir, lib_content) helper (two-param version)."
+  ],
+  "task_groups": [
+    {
+      "group_id": "group-core",
+      "reason": "All tasks modify shared schema.rs/workspace.rs files — the core discovery enhancement. TASK-single-crate-core-01 is a prerequisite for TASK-single-crate-core-02 (the error variant must exist before the function compiles).",
+      "tasks": ["TASK-single-crate-core-01", "TASK-single-crate-core-02"],
+      "depends_on_groups": []
+    },
+    {
+      "group_id": "group-fallback",
+      "reason": "Pipeline fallback logic depends on group-core (needs find_crate_root and CrateRootNotFound).",
+      "tasks": ["TASK-pipeline-fallback"],
+      "depends_on_groups": ["group-core"]
+    },
+    {
+      "group_id": "group-silent-exit",
+      "reason": "Self-contained fix for silent error exits in main.rs — no dependencies on other groups. Can run in parallel with group-fallback once group-core completes.",
+      "tasks": ["TASK-silent-exit-fix"],
+      "depends_on_groups": []
+    },
+    {
+      "group_id": "group-tests",
+      "reason": "Both are integration test changes that verify the feature. They depend on all production code changes being in place. Can run in parallel with each other.",
+      "tasks": ["TASK-test-existing-update", "TASK-test-single-crate-module-tree"],
+      "depends_on_groups": ["group-fallback"]
+    }
+  ],
+  "tasks": [
+    {
+      "id": "TASK-single-crate-core-01",
+      "kind": "direct",
+      "description": "Add CrateRootNotFound variant to schema::Error enum so find_crate_root can signal discovery failure.",
+      "files_in_scope": [
+        "src/schema.rs"
+      ],
+      "changes": [
+        {
+          "path": "src/schema.rs",
+          "action": "modify",
+          "guidance": "Add a new error variant to the Error enum. Insert it after the WorkspaceRootNotFound variant (line 55), before the FileRead variant (line 57):\n\n```rust\n#[error(\"no Cargo.toml with [package] section found starting from {0}\")]\nCrateRootNotFound(PathBuf),\n```\n\nThis variant carries a PathBuf (the start_path that was searched), matching the pattern of WorkspaceRootNotFound. The thiserror attribute format is consistent with other variants. The variant is automatically reachable — schema::Error is already imported by workspace.rs via `use crate::schema::{CrateType, Error, Result}`."
+        }
+      ],
+      "wiring_checklist": [],
+      "type_reference": {
+        "CrateRootNotFound": "#[error(\"no Cargo.toml with [package] section found starting from {0}\")] CrateRootNotFound(PathBuf)"
+      },
+      "acceptance": [
+        "cargo check --workspace"
+      ],
+      "depends_on": []
+    },
+    {
+      "id": "TASK-single-crate-core-02",
+      "kind": "lib-tdd",
+      "description": "Add find_crate_root discovery function + 2 unit tests via test-driven development.",
+      "files_in_scope": [
+        "src/workspace.rs"
+      ],
+      "changes": [
+        {
+          "path": "src/workspace.rs",
+          "action": "modify",
+          "guidance": "Implement find_crate_root following the TDD cycle. The test (tdd_interface.test_code) defines the contract — write it first, confirm failure, then implement.\n\nImplementation approach: Walk up the directory tree using start_path.ancestors(). For each ancestor, check if Cargo.toml exists. If it does, read the file contents. Map I/O errors to Error::FileRead { path, source }. Check if the content contains the string \"[package]\". If found, return Ok(ancestor.to_path_buf()). If the loop completes without finding a [package] section, return Err(Error::CrateRootNotFound(start_path.to_path_buf())).\n\nPlacement: Insert after find_workspace_root (after line 25), before enumerate_members (line 35). Use the same doc-comment structure as find_workspace_root: /// description, /// # Errors section. The function must be pub.\n\nFor the unit tests: Add both test functions inside the existing #[cfg(test)] mod tests block, before its closing } at line 206. The test module already has write_cargo_toml and setup_crate (one-param) helpers via `use super::*`. No new imports needed.\n\nEdge cases handled: empty directory tree (ancestors() always includes start_path itself), Cargo.toml without [package] (continues walking), I/O error reading Cargo.toml (returns FileRead error)."
+        }
+      ],
+      "tdd_interface": {
+        "test_file": "src/workspace.rs",
+        "test_module": "tests",
+        "test_fn_name": "find_crate_root_finds_package_section",
+        "test_code": "#[test]\nfn find_crate_root_finds_package_section() {\n    let tmp = tempfile::tempdir().unwrap();\n    setup_crate(tmp.path());\n    // Walk from a nested subdirectory inside src\n    let nested = tmp.path().join(\"src\").join(\"subdir\");\n    std::fs::create_dir_all(&nested).unwrap();\n    let result = find_crate_root(&nested).unwrap();\n    assert_eq!(result, tmp.path());\n}\n\n#[test]\nfn find_crate_root_returns_err_for_no_package() {\n    let tmp = tempfile::tempdir().unwrap();\n    // No Cargo.toml at all — ancestors() walks up and finds nothing\n    let result = find_crate_root(tmp.path());\n    assert!(result.is_err());\n    match result.unwrap_err() {\n        Error::CrateRootNotFound(_) => {},\n        other => panic!(\"expected CrateRootNotFound, got {:?}\", other),\n    }\n}",
+        "signature": "pub fn find_crate_root(start_path: &Path) -> Result<PathBuf>",
+        "expected_behavior": "find_crate_root walks up from start_path through ancestors, returns the first directory whose Cargo.toml contains a [package] section. Returns CrateRootNotFound error if no Cargo.toml with [package] is found in any ancestor (including start_path itself). Returns FileRead error if a Cargo.toml exists but cannot be read."
+      },
+      "wiring_checklist": [],
+      "type_reference": {
+        "find_crate_root": "pub fn find_crate_root(start_path: &Path) -> Result<PathBuf>"
+      },
+      "acceptance": [
+        "cargo test find_crate_root_finds_package_section",
+        "cargo test find_crate_root_returns_err_for_no_package",
+        "cargo check --workspace"
+      ],
+      "depends_on": ["TASK-single-crate-core-01"]
+    },
+    {
+      "id": "TASK-pipeline-fallback",
+      "kind": "direct",
+      "description": "Modify build_map in lib.rs to add fallback logic: try workspace discovery first, fall back to single-crate on WorkspaceRootNotFound or MissingWorkspaceSection.",
+      "files_in_scope": [
+        "src/lib.rs"
+      ],
+      "changes": [
+        {
+          "path": "src/lib.rs",
+          "action": "modify",
+          "guidance": "Two changes to src/lib.rs:\n\n**(1) Add Error import**: On line 18-21, the `use schema::{...}` block currently imports CrateInfo, CrateType, DiagnosticKind, ErrorEntry, ErrorSeverity, ModuleInfo, WorkspaceInfo, WorkspaceMap. Add `Error` to this import. The block should include `Error` alongside the existing types.\n\n**(2) Replace workspace discovery lines (37-38) with fallback match**: Replace the two lines:\n```rust\nlet workspace_root = workspace::find_workspace_root(&config.workspace_path)?;\nlet member_dirs = workspace::enumerate_members(&workspace_root)?;\n```\n\nWith a match expression that tries workspace discovery first, then falls back. The structure:\n```rust\nlet (workspace_root, member_dirs) = match workspace::find_workspace_root(&config.workspace_path) {\n    Ok(root) => {\n        match workspace::enumerate_members(&root) {\n            Ok(members) => (root, members),\n            Err(Error::MissingWorkspaceSection) => {\n                // [workspace] was a false positive (e.g., in a comment).\n                // Fall back to single-crate discovery.\n                let crate_dir = workspace::find_crate_root(&config.workspace_path)?;\n                (crate_dir.clone(), vec![crate_dir])\n            }\n            Err(other) => return Err(other.into()),\n        }\n    }\n    Err(Error::WorkspaceRootNotFound(_)) => {\n        let crate_dir = workspace::find_crate_root(&config.workspace_path)?;\n        (crate_dir.clone(), vec![crate_dir])\n    }\n    Err(other) => return Err(other.into()),\n};\n```\n\nKey points:\n- The inner match on enumerate_members handles the D1 false-positive case where Cargo.toml contains \"[workspace]\" as a literal string but no actual workspace section.\n- The fallback path is identical for both MissingWorkspaceSection and WorkspaceRootNotFound: find_crate_root then create a single-element vec.\n- Other errors (TomlParse, FileRead, etc.) propagate as fatal.\n- workspace_root is set to crate_dir in fallback mode; member_dirs is vec![crate_dir].\n- The rest of build_map (lines 40-171) works unchanged — it uses workspace_root and member_dirs identically regardless of discovery path.\n\nThe function signature and return type do NOT change."
+        }
+      ],
+      "wiring_checklist": [
+        {
+          "kind": "fn_call",
+          "file": "src/lib.rs",
+          "detail": "build_map calls workspace::find_crate_root in two fallback branches (MissingWorkspaceSection and WorkspaceRootNotFound)"
+        },
+        {
+          "kind": "type_annotation",
+          "file": "src/lib.rs",
+          "detail": "schema::Error is imported for the match patterns on Error::WorkspaceRootNotFound and Error::MissingWorkspaceSection"
+        }
+      ],
+      "acceptance": [
+        "cargo check --workspace",
+        "cargo clippy -- -D warnings"
+      ],
+      "depends_on": ["TASK-single-crate-core-02"]
+    },
+    {
+      "id": "TASK-silent-exit-fix",
+      "kind": "direct",
+      "description": "Fix 4 silent error exit sites in main.rs to print error messages to stderr before exiting.",
+      "files_in_scope": [
+        "src/main.rs"
+      ],
+      "changes": [
+        {
+          "path": "src/main.rs",
+          "action": "modify",
+          "guidance": "Fix four silent error exit sites in main.rs where errors are discarded without printing to stderr:\n\n**(1) File write error (lines 88-90)**: Change:\n```rust\nif rust_workspace_map::render::render_to_writer(&map, writer).is_err() {\n    std::process::exit(1);\n}\n```\nTo:\n```rust\nif let Err(e) = rust_workspace_map::render::render_to_writer(&map, writer) {\n    eprintln!(\"error writing JSON to file: {e}\");\n    std::process::exit(1);\n}\n```\nUse plain `{e}` for simple I/O errors.\n\n**(2) Stdout write error (lines 93-95)**: Same pattern, change to `if let Err(e)` with `eprintln!(\"error writing JSON to stdout: {e}\")`.\n\n**(3) Pipeline error (lines 102-104)**: Change `Err(_)` to `Err(e)` and add `eprintln!(\"error: {e:#}\")` before `exit(1)`. Use `{e:#}` (alternate format) for anyhow error chains — this produces multi-line output with context.\n\n**(4) Lookup pipeline error (lines 165-167)**: Same pattern as (3) — `Err(e) => { eprintln!(\"error: {e:#}\"); std::process::exit(1); }`.\n\n**Do NOT change**:\n- Validation exit (lines 98-100): exit code 2 is intentional for validation findings; stderr is intentionally silent.\n- Lookup serialization errors (lines 123-128, 140-145): these already print messages to stderr before exiting."
+        }
+      ],
+      "wiring_checklist": [],
+      "acceptance": [
+        "cargo check --workspace",
+        "cargo clippy -- -D warnings"
+      ],
+      "depends_on": []
+    },
+    {
+      "id": "TASK-test-existing-update",
+      "kind": "direct",
+      "description": "Rename and invert test_missing_workspace_section to test_single_crate_without_workspace — the existing test expects non-zero exit for a standalone crate, which is now a supported scenario.",
+      "files_in_scope": [
+        "tests/integration_test.rs"
+      ],
+      "changes": [
+        {
+          "path": "tests/integration_test.rs",
+          "action": "modify",
+          "guidance": "Rename `test_missing_workspace_section` (line 200) to `test_single_crate_without_workspace` and invert the test logic to match the new single-crate support behavior.\n\nReplace the test body (lines 199-223) with:\n\n1. Create a temp directory using `tempfile::tempdir().unwrap()`. Create a subdirectory `let crate_dir = root.join(\"standalone\");`.\n2. Use `setup_crate(&crate_dir, \"pub struct Standalone { pub x: i32 }\")` — the existing two-param helper writes both a [package]-only Cargo.toml (with name=\"standalone\") and src/lib.rs for that subdirectory.\n3. Run the index command via `run_index(crate_dir.to_str().unwrap())`.\n4. Assert exit 0: `assert!(output.status.success(), ...)`.\n5. Parse the JSON: `let json = parse_output(&output)`.\n6. Extract crates: `let crates = extract_array(&json, \"crates\")`.\n7. Assert exactly one crate (len == 1).\n8. Assert the crate name is \"standalone\": `crates[0][\"name\"] == \"standalone\"`.\n9. Assert the crate has at least one module and that \"Standalone\" appears in public items — iterate over modules' publicItems and find a struct named \"Standalone\".\n\nThe test name change from `test_missing_workspace_section` to `test_single_crate_without_workspace` signals the behavior change: what was an error condition is now a supported scenario."
+        }
+      ],
+      "wiring_checklist": [],
+      "acceptance": [
+        "cargo test --test integration_test test_single_crate_without_workspace"
+      ],
+      "depends_on": ["TASK-pipeline-fallback"]
+    },
+    {
+      "id": "TASK-test-single-crate-module-tree",
+      "kind": "direct",
+      "description": "Add test_single_crate_module_tree integration test with depth >= 2 submodule resolution in single-crate mode.",
+      "files_in_scope": [
+        "tests/integration_test.rs"
+      ],
+      "changes": [
+        {
+          "path": "tests/integration_test.rs",
+          "action": "modify",
+          "guidance": "Add a new integration test `test_single_crate_module_tree` that verifies module tree construction works in single-crate mode at depth >= 2.\n\nImplementation:\n\n1. Create a temp directory and a subdirectory with a known name: `let tmp = tempfile::tempdir().unwrap(); let root = tmp.path().join(\"single-crate\"); std::fs::create_dir_all(&root).unwrap();`\n2. Use `setup_crate(&root, \"pub mod helpers;\")` to create a standalone (non-workspace) crate with lib.rs declaring a submodule.\n3. Create `src/helpers/mod.rs` with `pub mod sub;` (a module at depth 1 declaring depth 2).\n4. Create `src/helpers/sub.rs` with `pub fn assist() {}` — this is the depth-2 module with a public item.\n5. Run the index command: `let output = run_index(root.to_str().unwrap());`\n6. Assert exit 0: `assert!(output.status.success(), ...)`\n7. Parse JSON: `let json = parse_output(&output);`\n8. Extract crates array and assert length 1.\n9. Assert the crate name is `\"single-crate\"` (derived from the dir basename by setup_crate).\n10. Collect module paths from the crate and assert all three expected paths exist:\n    - `\"single-crate\"` (root module)\n    - `\"single-crate::helpers\"` (depth 1)\n    - `\"single-crate::helpers::sub\"` (depth 2)\n11. Find the `helpers::sub` module and verify it has the `assist` function in its publicItems array (kind = \"fn\", name = \"assist\").\n\nUse the existing helpers: `setup_crate(dir, lib_content)`, `run_index(path)`, `parse_output(output)`, `extract_array(val, key)`. Follow the pattern established by `test_deeply_nested_modules` (lines 284-329).\n\nThe maximum depth for this test (2) exercises recursive module resolution in single-crate mode — the key risk flagged by the architectural review. The existing `test_deeply_nested_modules` goes to depth 4 but uses workspace mode with `members = [\".\"]` — this new test confirms the same depth works in pure single-crate mode without any `[workspace]` section."
+        }
+      ],
+      "wiring_checklist": [],
+      "acceptance": [
+        "cargo test --test integration_test test_single_crate_module_tree"
+      ],
+      "depends_on": ["TASK-pipeline-fallback"]
+    }
+  ]
+}
diff --git a/notes/directions/single-crate-support/draft-elaboration.md b/notes/directions/single-crate-support/draft-elaboration.md
new file mode 100644
index 0000000..1ae9cec
--- /dev/null
+++ b/notes/directions/single-crate-support/draft-elaboration.md
@@ -0,0 +1,406 @@
+# Draft Elaboration -- Single-Crate Support Phase
+
+> Generated: 2026-05-06 | Phase: single-crate-support
+> Plan reviewed: commit `5961d63`
+> Inputs: `plans/single-crate-support/PLAN.md`, `notes/directions/single-crate-support/deferred-and-patterns.md`, `notes/directions/single-crate-support/workspace-map.json`, `notes/architecture-current.md`
+
+---
+
+## 0. Architecture Review Summary
+
+The plan is architecturally sound. The auto-detect design (no new CLI flags), two separate discovery functions, and orchestration in `build_map` all follow existing patterns correctly. Two adjustments are recommended:
+
+1. **D1 (incorporate now)**: Extend the `build_map` error match to also catch `MissingWorkspaceSection` from `enumerate_members` as a fallback trigger.
+2. **validate.rs path safety confirmed**: The plan's Implementation Note concern is resolved -- `check_orphan_files` is safe in single-crate mode.
+
+No structural changes to the plan are needed. The 7-step breakdown is correct; task groupings are described in Section 6.
+
+---
+
+## 1. Design Decisions Per Goal
+
+### Goal A: Single-crate auto-detection (Steps 1-4)
+
+**Decision A1 -- Auto-detect, no new CLI flag**
+
+The plan correctly chooses auto-detection over a `--single-crate` flag. The CLI surface stays clean. The disambiguation rule is: workspace mode takes priority; single-crate is only a fallback when `WorkspaceRootNotFound` is returned.
+
+No change to `Config` (schema.rs:89-100) or `Cli` (main.rs:36-39).
+
+**Decision A2 -- Two separate functions, not one combined**
+
+`find_workspace_root` checks for `[workspace]`; `find_crate_root` checks for `[package]`. A combined function that checks for either would stop at a workspace member's `Cargo.toml` (which has `[package]` but no `[workspace]`), returning the member directory instead of the workspace root. The separate-function design is correct for both multi-crate and single-crate scenarios.
+
+**Decision A3 -- Orchestration lives in `build_map`, not `workspace.rs`**
+
+`workspace.rs` remains pure: each function does one thing. The orchestration (try A, on specific error fall back to B) stays in `lib.rs:build_map`. This preserves single-responsibility.
+
+**Decision A4 -- D1: Catch `MissingWorkspaceSection` as fallback trigger (RECOMMENDED ADDITION)**
+
+The plan's Step 4 match only catches `WorkspaceRootNotFound`:
+
+```rust
+Err(Error::WorkspaceRootNotFound(_)) => {
+    let crate_dir = workspace::find_crate_root(&config.workspace_path)?;
+    ...
+}
+```
+
+But `find_workspace_root` can produce a false positive: a Cargo.toml containing `[workspace]` in a comment or string literal (e.g., `# This project used to be a [workspace]`). In this case, `find_workspace_root` returns `Ok(root)`, but `enumerate_members` fails with `MissingWorkspaceSection` (the TOML has no actual `[workspace]` section).
+
+**Recommendation**: Extend the match to also catch `MissingWorkspaceSection` from `enumerate_members`:
+
+```rust
+let (workspace_root, member_dirs) = match workspace::find_workspace_root(&config.workspace_path) {
+    Ok(root) => {
+        match workspace::enumerate_members(&root) {
+            Ok(members) => (root, members),
+            Err(Error::MissingWorkspaceSection) => {
+                // [workspace] was a false positive (e.g., in a comment).
+                // Fall back to single-crate discovery.
+                let crate_dir = workspace::find_crate_root(&config.workspace_path)?;
+                (crate_dir.clone(), vec![crate_dir])
+            }
+            Err(other) => return Err(other.into()),
+        }
+    }
+    Err(Error::WorkspaceRootNotFound(_)) => {
+        let crate_dir = workspace::find_crate_root(&config.workspace_path)?;
+        (crate_dir.clone(), vec![crate_dir])
+    }
+    Err(other) => return Err(other.into()),
+};
+```
+
+This is a small change with minimal risk: the fallback path is identical to the `WorkspaceRootNotFound` case.
+
+### Goal B: Fix silent error exits (Step 5)
+
+**Decision B1 -- Fix all four sites at once**
+
+The plan treats all four silent-exit sites as a single work item. This is correct: fixing only some would leave the user with a partial experience (single-crate works but some failures remain invisible).
+
+**Decision B2 -- Use `{e:#}` for pipeline/render errors**
+
+The plan uses `{e:#}` (alternate format) for the pipeline and render error sites (lines 102-104 and 165-167). This produces a multi-line error chain with `anyhow` context, which is desirable for `build_map` failures. The JSON write errors (lines 88-95) use plain `{e}` since they are simple I/O errors.
+
+**Decision B3 -- No additional silent-exit sites**
+
+A scan of `main.rs` confirms the four sites identified in the plan are the only ones. No other `.is_err()` or `Err(_)` patterns exist that swallow errors without printing.
+
+Validation exit (lines 98-100) uses `process::exit(2)` with no stderr message -- this is intentional: exit code 2 signals validation findings were present, and the findings themselves are in the JSON output on stdout.
+
+### Goal C: Test updates (Steps 6-7)
+
+**Decision C1 -- Invert `test_missing_workspace_section`**
+
+The existing test expects a non-zero exit for a standalone crate without `[workspace]`. After the fix, this scenario must succeed. Renaming to `test_single_crate_without_workspace` and inverting the assertion is the correct signal: the behavior change is a new capability, not a regression.
+
+**Decision C2 -- Depth >= 2 in new module tree test**
+
+The deferred analysis (F5) correctly flags that `test_single_crate_module_tree` should cover modules at depth >= 2. The existing `test_deeply_nested_modules` goes 4 levels deep but uses `members = ["."]` (workspace mode). The new test should exercise at minimum `lib.rs` -> `helpers.rs` -> `helpers/sub.rs` (depth-2 submodule resolution in single-crate mode).
+
+---
+
+## 2. Crate Boundary Decisions
+
+This is a single-crate tool (`rust-workspace-map`). All changes stay within this crate. No new crates are introduced.
+
+| Change | Module | Rationale |
+|--------|--------|-----------|
+| `CrateRootNotFound` variant | `schema.rs` | Follows existing pattern: all library errors in `schema::Error` |
+| `find_crate_root` function | `workspace.rs` | Follows existing pattern: discovery functions live in `workspace` module |
+| `find_crate_root` unit tests (2) | `workspace.rs` (inline `#[cfg(test)]`) | Follows existing pattern: 5 unit tests already inline |
+| Fallback logic | `lib.rs:build_map` | Follows existing pattern: pipeline orchestration in `build_map` |
+| Silent exit fixes (4 sites) | `main.rs` | CLI presentation layer |
+| Integration test update | `tests/integration_test.rs` | Follows existing pattern: `Command`-based integration tests |
+| New integration test | `tests/integration_test.rs` | Same as above |
+
+**Modules NOT modified and why:**
+
+| Module | Reason unchanged |
+|--------|-----------------|
+| `cargo_info.rs` | Parses per-crate Cargo.toml; unaffected by workspace vs. single-crate distinction |
+| `file_parser.rs` | AST extraction; operates on individual `.rs` files |
+| `module_tree.rs` | Operates on a single crate root path; `build_map` passes the same paths regardless of mode |
+| `cross_refs.rs` | Operates on `&mut [CrateInfo]`; in single-crate mode the slice has one element; degenerate case handled correctly |
+| `render.rs` | Thin serde wrappers; unaware of workspace structure |
+| `validate.rs` | Path safety confirmed (see Section 5); no changes needed |
+| `indexes.rs` | Operates on `&[CrateInfo]`; single-crate case handled correctly |
+| `lookup.rs` | Operates on `WorkspaceMap`; same structure regardless of mode |
+
+---
+
+## 3. Pattern Requirements
+
+### 3.1 `find_crate_root` must mirror `find_workspace_root`
+
+**Structural mirror** (workspace.rs:11-25 -> new function):
+
+```rust
+/// Walk up the directory tree from `start_path` to find a `Cargo.toml`
+/// containing a `[package]` section. Returns the directory containing it.
+///
+/// This is the fallback path when no workspace root is found — it treats
+/// a single crate as a "workspace of one".
+///
+/// # Errors
+///
+/// Returns `Error::CrateRootNotFound` if no `Cargo.toml` with a
+/// `[package]` section is found in any ancestor directory.
+pub fn find_crate_root(start_path: &Path) -> Result<PathBuf> {
+    for ancestor in start_path.ancestors() {
+        let cargo_toml = ancestor.join("Cargo.toml");
+        if cargo_toml.exists() {
+            let content = std::fs::read_to_string(&cargo_toml)
+                .map_err(|source| Error::FileRead {
+                    path: cargo_toml.clone(),
+                    source,
+                })?;
+            if content.contains("[package]") {
+                return Ok(ancestor.to_path_buf());
+            }
+        }
+    }
+    Err(Error::CrateRootNotFound(start_path.to_path_buf()))
+}
+```
+
+Checklist:
+- [x] Same signature: `pub fn find_crate_root(start_path: &Path) -> Result<PathBuf>`
+- [x] Same `ancestors()` walk pattern
+- [x] Same `FileRead` error mapping
+- [x] Same `.contains()` check style (string match, same as workspace)
+- [x] Same doc comment structure (`# Errors` section)
+- [x] Uses `schema::Error` (already imported via `use crate::schema::{CrateType, Error, Result}`)
+
+**Why not use TOML parsing?** Both `find_workspace_root` and `find_crate_root` use `content.contains("[...]")` -- a simple string match. Using `toml::from_str` would be more robust (handles comments, string literals) but adds a dependency on the `toml` crate for a simple existence check. The string-match approach is consistent with the existing code and avoids pulling in TOML parsing for discovery. The `MissingWorkspaceSection` fallback in `build_map` (Decision A4) mitigates the false-positive risk.
+
+### 3.2 Unit tests must follow existing `#[cfg(test)] mod tests` pattern
+
+Two tests, following the structure of existing workspace tests:
+
+1. **`find_crate_root_finds_package_section`** -- creates a temp directory with `Cargo.toml` containing `[package]`, places a `lib.rs` in `src/`, walks from a nested subdirectory to find the crate root. Modeled after `find_workspace_root_finds_cargo_toml` (workspace.rs:134-142).
+
+2. **`find_crate_root_returns_err_for_no_package`** -- creates a temp directory with no `Cargo.toml`, verifies `CrateRootNotFound`. Modeled after `enumerate_members_returns_err_for_missing_workspace` (workspace.rs:157-167).
+
+Both tests use the existing helper `write_cargo_toml` and `setup_crate` functions already in the test module (workspace.rs:117-132).
+
+### 3.3 Integration tests must follow `Command`-based pattern
+
+All 10 existing integration tests and both new tests use `std::process::Command` to invoke the compiled binary. This is the established pattern.
+
+### 3.4 Error handling pattern
+
+`find_crate_root` returns `schema::Result<PathBuf>`. The plan's Step 4 code converts to `anyhow::Error` via `?` in the `build_map` context. This matches the existing pattern: `schema::Error` variants are converted to `anyhow::Error` at the pipeline boundary.
+
+### 3.5 New error variant following existing conventions
+
+```rust
+#[error("no Cargo.toml with [package] section found starting from {0}")]
+CrateRootNotFound(PathBuf),
+```
+
+Position: after `WorkspaceRootNotFound` (schema.rs:55), before `FileRead` (schema.rs:57). The variant carries a `PathBuf` (the start path), matching `WorkspaceRootNotFound`'s pattern. The `thiserror` format is consistent with the other variants.
+
+---
+
+## 4. Type Signatures for Key New Types
+
+### 4.1 New error variant
+
+```rust
+// schema.rs, in enum Error
+#[error("no Cargo.toml with [package] section found starting from {0}")]
+CrateRootNotFound(PathBuf),
+```
+
+### 4.2 New discovery function
+
+```rust
+// workspace.rs
+pub fn find_crate_root(start_path: &Path) -> Result<PathBuf>;
+```
+
+Where `Result<T>` is `schema::Result<T>` (aliased to `std::result::Result<T, schema::Error>`).
+
+### 4.3 Modified `build_map` signature (unchanged, but semantics change)
+
+```rust
+// lib.rs -- signature unchanged, behavior extended
+pub fn build_map(config: &Config) -> anyhow::Result<WorkspaceMap>;
+```
+
+The function now accepts both workspace-root and single-crate `config.workspace_path` values. The `WorkspaceMap.workspace_root` field will be the crate directory in single-crate mode (not a parent-of-crates directory). All downstream consumers (`validate`, `indexes`, `lookup`, `render`) are agnostic to this value.
+
+---
+
+## 5. Known Pitfalls and Constraints
+
+### 5.1 Path safety in validate.rs -- CONFIRMED SAFE
+
+The plan's Implementation Note (lines 139-141) flags a concern about `validate::validate` assuming `workspace_root` is a parent-of-crates directory. Analysis of `check_orphan_files` (validate.rs:90-158) confirms this is safe:
+
+**In multi-crate workspace mode:**
+- `workspace_root` = `/path/to/workspace/`
+- `crate_info.root` = `"core/src/lib.rs"` (workspace-relative)
+- `workspace_root.join("core/src/lib.rs")` = `/path/to/workspace/core/src/lib.rs` -- correct
+
+**In single-crate mode:**
+- `workspace_root` = `/path/to/crate/` (same as crate_dir)
+- `crate_info.root` = `"src/lib.rs"` (workspace-relative, and workspace_root == crate_dir)
+- `workspace_root.join("src/lib.rs")` = `/path/to/crate/src/lib.rs` -- correct
+- `.parent()` = `/path/to/crate/src/` -- correct (finds the src directory)
+- `path.strip_prefix(workspace_root)` on orphan files produces `"src/orphan_file.rs"` -- matches how `declared_files` stores module paths (via `relativize_path`)
+
+The invariant holds: `crate_info.root` is always workspace-relative, and `workspace_root` is always the prefix that was stripped. In single-crate mode, the prefix is the crate directory itself, so `crate_info.root` values are paths like `"src/lib.rs"` rather than `"crate_name/src/lib.rs"`.
+
+**No changes needed to validate.rs.**
+
+### 5.2 F3 (hardcoded `strip_prefix("src/")`) -- ALREADY FIXED
+
+The deferred item F3 referenced a prior MVP issue where `determine_parent_file` hardcoded `strip_prefix("src/")`. This was fixed in the MVP cleanup phase (commit `39ef75d`). The current code (validate.rs:161-186) derives the src directory from `crate_info.root`:
+
+```rust
+let src_dir = std::path::Path::new(&crate_info.root)
+    .parent()
+    .map_or(std::path::Path::new(""), |p| p);
+let src_prefix = format!("{}/", src_dir.display());
+```
+
+This is correct for both workspace and single-crate modes.
+
+### 5.3 D3 (`unwrap_or_default` in workspace.rs) -- NOT TRIGGERED IN SINGLE-CRATE MODE
+
+`enumerate_members` uses `.unwrap_or_default()` on lines 57 and 69 for missing/malformed `members` and `exclude` arrays. In single-crate mode, `enumerate_members` is never called -- the fallback creates `vec![crate_dir]` directly. This is an existing issue specific to workspace mode and is not exacerbated by this phase.
+
+### 5.4 `unwrap_or_default()` in `workspace_name` derivation
+
+In `lib.rs:build_map` (line 150-153), the workspace name is derived from the last component of `workspace_root`:
+
+```rust
+let workspace_name = workspace_root
+    .file_name()
+    .map(|n| n.to_string_lossy().to_string())
+    .unwrap_or_default();
+```
+
+In single-crate mode, `workspace_root` is the crate directory, so `workspace_name` will be the crate directory name (e.g., `"rust-workspace-map"`). This is a reasonable default and consistent with how Cargo names workspaces by their directory.
+
+### 5.5 Cross-references in single-crate mode
+
+`cross_refs::compute(&mut crate_infos)` (lib.rs:139) receives a slice with one element. The function initializes `TypeRef` entries for all public items, scans imports, and attempts to match the first import segment against crate names. With only one crate, all cross-crate imports will be self-referential and correctly skipped (the heuristic already checks `other_name != crate_name`). The `CrossReferences` output will be empty, which is correct.
+
+### 5.6 Clippy on `content.contains("[package]")`
+
+The plan uses the same `.contains()` pattern as the existing `find_workspace_root`. Clippy has no lint for this specifically; the project already uses `#[warn(clippy::pedantic)]` at the crate level (lib.rs:1) and no suppression is needed.
+
+### 5.7 Error printing at render sites -- `main.rs` vs `run()`
+
+The plan's Step 5 fixes the two render error sites (lines 88-90 and 93-95) as:
+
+```rust
+if let Err(e) = ... {
+    eprintln!("error writing JSON to file: {e}");
+    exit(1);
+}
+```
+
+These sites bypass `lib.rs::run()` and call `render::render_to_writer` directly. This is the existing architecture (main.rs handles output routing, not `run()`). The fix is consistent: `main.rs` is responsible for CLI presentation, including error messages.
+
+### 5.8 No `ErrorEntry` wiring for `CrateRootNotFound`
+
+The deferred item D2 suggests wiring errors into `WorkspaceMap.errors`. If `find_crate_root` fails (returns `CrateRootNotFound`), the error propagates as a fatal `anyhow::Error` via `?` in `build_map`. This is consistent with the existing architecture: pre-pipeline failures (workspace root not found, missing workspace section) are fatal, not collected in `errors`. Per-crate failures (toml parse, syn parse, missing crate roots) are soft. D2 is correctly deferred as low-priority.
+
+---
+
+## 6. Suggested Task Grouping Rationale
+
+The plan's 7 steps naturally group into three task groups:
+
+### Group 1: Schema + Discovery (Steps 1-3) -- Library, TDD
+
+**Goal tag: `G1-single-crate-core`**
+
+| Step | Description | Approach |
+|------|-------------|----------|
+| 1 | Add `CrateRootNotFound` to `schema::Error` | `lib-tdd` (pure type addition, compile-checked) |
+| 2 | Add `find_crate_root` to `workspace.rs` | `lib-tdd` (pure discovery logic, testable in isolation) |
+| 3 | Add 2 unit tests for `find_crate_root` | `lib-tdd` (tests written before/during Step 2) |
+
+**Rationale for grouping**: These three steps form a cohesive unit. Step 1 is a prerequisite for Step 2 (the function returns the new error variant). Step 3 validates Step 2. All three can be verified with `cargo test` without touching any other module. No integration test changes needed yet.
+
+**TDD sequence**:
+1. Add `CrateRootNotFound` variant (compiles but unused)
+2. Write the two unit tests (they fail -- function doesn't exist yet)
+3. Implement `find_crate_root` (tests pass)
+
+### Group 2: Pipeline Integration + Error Visibility (Steps 4-5) -- Plumbing, Direct
+
+**Goal tags: `G2-pipeline-fallback`, `G3-silent-exit-fix`**
+
+| Step | Description | Approach |
+|------|-------------|----------|
+| 4 | Modify `build_map` fallback logic (+ D1 enhancement) | `direct` (wires existing functions; test via integration tests) |
+| 5 | Fix 4 silent error exits in `main.rs` | `direct` (presentation-layer change; test via manual run or integration tests) |
+
+**Rationale for grouping**: Steps 4 and 5 together make the feature "work end-to-end." Step 4 alone would make single-crate mode functional but leave failures invisible. Step 5 alone would improve error visibility but not add single-crate support. Together they produce a testable, user-facing improvement.
+
+Note: Step 4 is classified as `direct` (not `lib-tdd`) because the orchestration logic in `build_map` calls existing functions (`find_workspace_root`, `enumerate_members`, `find_crate_root`) which are already unit-tested. The integration is verified by the new/modified integration tests in Group 3.
+
+### Group 3: Integration Test Updates (Steps 6-7) -- Verification, Direct
+
+**Goal tags: `G4-test-existing-update`, `G5-test-single-crate-module-tree`**
+
+| Step | Description | Approach |
+|------|-------------|----------|
+| 6 | Update `test_missing_workspace_section` | `direct` (test behavior inversion, no new production code) |
+| 7 | Add `test_single_crate_module_tree` | `direct` (new test, depth >= 2) |
+
+**Rationale for grouping**: Both steps are integration test changes that depend on Groups 1 and 2 being complete. They can be implemented and verified together after the production code changes are in place.
+
+**Step 7 must include depth >= 2 per F5**: The test should create a module tree of `lib.rs` -> `helpers.rs` -> `helpers/sub.rs` (at minimum). This exercises recursive module resolution in single-crate mode, which is the risk flagged by F5.
+
+---
+
+## 7. Verification Summary
+
+### Per-group verification
+
+| Group | Verification |
+|-------|-------------|
+| G1 | `cargo test --lib workspace` (new unit tests pass) |
+| G2 | `cargo check`, `cargo clippy -- -D warnings` |
+| G3 | `cargo test --test integration_test` (all 19 tests pass) |
+
+### End-to-end verification
+
+| # | Command | Expected |
+|---|---------|----------|
+| 1 | `cargo run -- index .` | Valid JSON, one crate (`rust-workspace-map`), all modules present |
+| 2 | `cargo run -- index . \| jq '.crates[0].name'` | `"rust-workspace-map"` |
+| 3 | `cargo run -- index /tmp/nonexistent` | Non-zero exit, clear error on stderr |
+| 4 | `cargo run -- lookup . --symbol <public_item>` | Valid JSON (confirm lookup works in single-crate mode) |
+| 5 | `cargo test` | All 30 unit + 19 integration tests pass |
+| 6 | `cargo clippy -- -D warnings` | Clean |
+
+### Additional verification (from deferred-and-patterns.md)
+
+| # | Scenario | Expected |
+|---|----------|----------|
+| 7 | Malformed Cargo.toml (no `[workspace]` or `[package]`) in temp dir, run `index` on it | Clear error on stderr, non-zero exit |
+| 8 | Workspace with valid `[workspace]` but pointing to non-existent member | `enumerate_members` error surfaces (not swallowed) |
+
+---
+
+## 8. Interaction Matrix: Plan Steps vs. Deferred Items
+
+| Plan Step | Deferred Items | Risk | Disposition |
+|-----------|---------------|------|-------------|
+| Step 1: `CrateRootNotFound` | None | Low | Proceed as planned |
+| Step 2: `find_crate_root` | D1 (MissingWorkspaceSection) | Low | Proceed as planned; D1 handled in Step 4 |
+| Step 3: Unit tests | F5 (depth coverage) | Low | Proceed as planned |
+| Step 4: `build_map` | **D1**, D2, D3 | **Medium** | **Extend match per Decision A4** |
+| Step 5: Silent exits | F1 (being fixed) | Low | Proceed as planned |
+| Step 6: Test rename | None | Low | Proceed as planned |
+| Step 7: New test | F5 (depth >= 2) | Low | **Ensure depth >= 2 per Decision C2** |
diff --git a/notes/directions/single-crate-support/task-checklist.md b/notes/directions/single-crate-support/task-checklist.md
new file mode 100644
index 0000000..710f6b8
--- /dev/null
+++ b/notes/directions/single-crate-support/task-checklist.md
@@ -0,0 +1,183 @@
+# Task Checklist -- Single-Crate Support + Fix Silent Error Handling
+
+> Generated: 2026-05-06 | Review of `draft-directions.json`
+> Source: `notes/directions/single-crate-support/draft-directions.json`
+> Codebase: `notes/directions/single-crate-support/codebase-state.md`
+
+---
+
+## Overall Assessment
+
+**Ready to Implement with minor corrections and observations noted below.** No task is blocked; no task is ambiguous to the point of being unimplementable. One line-number error in the placement guidance for unit tests (TASK-single-crate-core-02) needs attention before coding.
+
+---
+
+## TASK-single-crate-core-01: Add CrateRootNotFound variant
+
+| Criterion | Verdict | Details |
+|-----------|---------|---------|
+| Goal clear? | CLEAR | Add a single variant to the Error enum. |
+| Files in scope correct? | CLEAR | `src/schema.rs` is the only file. The Error enum lives there (lines 52-83). |
+| Implementation detail sufficient? | CLEAR | Exact variant code provided with field type, thiserror attribute, and precise insertion point (after line 55, before line 57). Verified against the source: line 55 ends `WorkspaceRootNotFound(PathBuf),`, line 57 begins `FileRead {`. Correct. |
+| New types/interfaces defined clearly? | CLEAR | The variant `CrateRootNotFound(PathBuf)` is fully specified. |
+| Dependencies explicit? | CLEAR | No dependencies on other tasks. |
+| Verification sufficient? | CLEAR | `cargo check --workspace` is appropriate — the variant only needs to compile. |
+| Wiring checklist | CLEAR | Empty (trivial task). |
+
+### Finding
+
+- The `known_pitfalls` entry about lib.rs line 18-21 not importing `Error` is correct — I verified `src/lib.rs` lines 18-21 import `CrateInfo, CrateType, DiagnosticKind, ErrorEntry, ErrorSeverity, ModuleInfo, WorkspaceInfo, WorkspaceMap` with no `Error`.
+
+---
+
+## TASK-single-crate-core-02: Add find_crate_root + 2 unit tests (lib-tdd)
+
+| Criterion | Verdict | Details |
+|-----------|---------|---------|
+| Goal clear? | CLEAR | Implement `find_crate_root` following the provided TDD test code. |
+| Files in scope correct? | CLEAR | `src/workspace.rs` — the function belongs in the discovery module alongside `find_workspace_root`. Verified that `Error` and `Result` are already imported (line 1: `use crate::schema::{CrateType, Error, Result}`). |
+| Implementation detail sufficient? | CLEAR | Guidance provides the algorithm (ancestors walk, FileRead error mapping, `.contains("[package]")` check, CrateRootNotFound fallback), placement instructions (after `find_workspace_root`, before `enumerate_members`), doc-comment requirements, and `pub` visibility. |
+| New types/interfaces defined clearly? | CLEAR | Signature is provided: `pub fn find_crate_root(start_path: &Path) -> Result<PathBuf>`. |
+| Dependencies explicit? | CLEAR | Depends on TASK-single-crate-core-01 (CrateRootNotFound variant must exist first). |
+| Wiring checklist | CLEAR | Empty (function definition only, no wiring needed). |
+
+### CRITICAL: Line-number error in unit test placement
+
+The guidance says:
+
+> "Add both test functions to the existing #[cfg(test)] mod tests block (after line 206, before resolve_crate_roots)."
+
+This is **incorrect**. Line 206 is the closing `}` of the `#[cfg(test)] mod tests` block. Lines 207-209 are:
+```
+/// Returns `(path, CrateType)` pairs — one for `src/lib.rs` (Lib),
+/// one for `src/main.rs` (Bin), or empty if neither exists.
+#[must_use]
+```
+
+"After line 206" places the tests **outside** the test module and inside the doc comment for `resolve_crate_roots`. The correct instruction should be: **inside the `mod tests` block, before the closing `}` at line 206** (i.e., insert before line 206, not after).
+
+This is a minor issue — the intent is clear and any implementer would realize the error. But it should be corrected before automated tooling relies on the line numbers.
+
+### TDD Interface Review -- PASS
+
+| Check | Verdict | Details |
+|-------|---------|---------|
+| Test code specific and falsifiable? | CLEAR | `find_crate_root_finds_package_section` walks from a nested subdirectory and asserts the result equals the temp root. `find_crate_root_returns_err_for_no_package` asserts `Error::CrateRootNotFound(_)`. Both assert concrete behavior. |
+| Signature matches test_code? | CLEAR | `pub fn find_crate_root(start_path: &Path) -> Result<PathBuf>` — the test code calls `find_crate_root(&nested)` and `find_crate_root(tmp.path())`, both passing `&Path`. |
+| Expected behavior documented? | CLEAR | Describes ancestor walk, `[package]` detection, CrateRootNotFound and FileRead error conditions. |
+
+One note on the test: `find_crate_root_returns_err_for_no_package` creates a temp dir with no Cargo.toml at all. Walk from `tmp.path()` up ancestors depends on no ancestor having a `[package]` Cargo.toml (e.g., inside the system temp directory). This is the same risk as the existing `find_workspace_root_finds_cargo_toml` test and is acceptable as a pattern. Not a flag.
+
+---
+
+## TASK-pipeline-fallback: Modify build_map for fallback logic
+
+| Criterion | Verdict | Details |
+|-----------|---------|---------|
+| Goal clear? | CLEAR | Replace unconditional `?` propagation with a match that falls back to single-crate on WorkspaceRootNotFound or MissingWorkspaceSection. |
+| Files in scope correct? | CLEAR | `src/lib.rs` — the `build_map` function. Verified lines 37-38 contain the two lines to replace. |
+| Implementation detail sufficient? | CLEAR | Full match expression code provided with inline comments. Guidance explains the nested match, error propagation, and identical fallback path. |
+| New types/interfaces defined clearly? | CLEAR | No new types/interfaces. Reuses existing `Error`, `WorkspaceRootNotFound`, `MissingWorkspaceSection`, and `find_crate_root`. |
+| Dependencies explicit? | CLEAR | Depends on both TASK-single-crate-core-01 (Error::CrateRootNotFound) and TASK-single-crate-core-02 (find_crate_root function exists). |
+| Verification sufficient? | CLEAR | `cargo check --workspace` and `cargo clippy -- -D warnings`. Appropriate for a logic change that doesn't expose new test surface. |
+
+### Wiring checklist review
+
+| Item | Verdict | Details |
+|------|---------|---------|
+| fn_call: `build_map calls workspace::find_crate_root` | CLEAR | The code shows two `workspace::find_crate_root(...)` calls in the match arms. `workspace` module is already imported implicitly via `pub mod workspace;` at line 12. |
+| type_annotation: `schema::Error` imported for match patterns | CLEAR | The guidance explicitly says to add `Error` to the `use schema::{...}` block. I verified that `Error` is missing from the current import (lines 18-21). |
+
+### Finding
+
+- The match expression uses `Err(other) => return Err(other.into())` for non-fallback errors. The `into()` converts `schema::Error` into `anyhow::Error`. This is correct because `build_map` returns `anyhow::Result<WorkspaceMap>`.
+
+---
+
+## TASK-silent-exit-fix: Fix 4 silent error exits in main.rs
+
+| Criterion | Verdict | Details |
+|-----------|---------|---------|
+| Goal clear? | CLEAR | Print error messages to stderr before `exit(1)` at four specific sites. |
+| Files in scope correct? | CLEAR | `src/main.rs` only. I verified all four sites exist at the claimed locations. |
+| Implementation detail sufficient? | CLEAR | Exact before/after code provided for each of the four sites, including the correct format string (`{e}` for I/O errors vs `{e:#}` for anyhow errors). Guidance also correctly marks what NOT to change (validation exit code 2, lookup serialization errors). |
+| New types/interfaces defined clearly? | CLEAR | No new types. |
+| Dependencies explicit? | CLEAR | No dependencies on other tasks. |
+| Verification sufficient? | CLEAR | `cargo check --workspace` and `cargo clippy -- -D warnings`. Correct — no test changes, pure refactor of existing error handling. |
+| Wiring checklist | CLEAR | Empty (self-contained changes to main.rs). |
+
+### Findings
+
+- The guidance correctly identifies four sites. I verified all four:
+  - Lines 88-90: file write `.is_err()` -> silent `exit(1)` (file write)
+  - Lines 93-95: stdout write `.is_err()` -> silent `exit(1)` (stdout write)
+  - Lines 102-104: pipeline `Err(_)` -> silent `exit(1)` (pipeline error)
+  - Lines 165-167: lookup `Err(_)` -> silent `exit(1)` (lookup error)
+
+---
+
+## TASK-test-existing-update: Rename and invert test_missing_workspace_section
+
+| Criterion | Verdict | Details |
+|-----------|---------|---------|
+| Goal clear? | CLEAR | Rename an existing test and invert its expectations (exit 1 -> exit 0 with assertions). |
+| Files in scope correct? | CLEAR | `tests/integration_test.rs`. The test `test_missing_workspace_section` is at lines 199-223. Verified. |
+| Implementation detail sufficient? | CLEAR | Nine-step sequence provided for the new test body. All helper functions (`setup_crate`, `run_index`, `parse_output`, `extract_array`) exist and signatures match usage. The test name is given. |
+| New types/interfaces defined clearly? | CLEAR | No new types. Reuses existing helpers. |
+| Dependencies explicit? | CLEAR | Depends on TASK-pipeline-fallback (the fallback must be in place for the test to pass). |
+| Verification sufficient? | CLEAR | `cargo test --test integration_test test_single_crate_without_workspace` — single test invocation. |
+| Wiring checklist | CLEAR | Empty (test change only). |
+
+### Finding
+
+- The guidance writes `setup_crate(root, "pub struct Standalone { pub x: i32 }")` then asserts `crates[0]["name"] == "standalone"`. The `setup_crate` helper derives the crate name from `dir.file_name().unwrap()`. Since the test creates a temporary directory with a random name (not "standalone"), the assertion `crates[0]["name"] == "standalone"` will FAIL. The temp directory name is a random hex string from `tempfile::tempdir()`.
+
+  This is a **bug in the guidance**. The test should either:
+  - (a) Check that the crate name matches the **directory basename** (i.e., `root.file_name().unwrap()`), or
+  - (b) Use `write_cargo_toml` to create a Cargo.toml with `name = "standalone"` explicitly.
+
+  The guidance itself says "Assert the crate name matches the directory name: `crates[0]["name"] == "standalone"` (directory basename, from setup_crate's naming)" — but the directory basename is NOT `"standalone"`, it's the tempdir's random name. The implementer will need to fix this: either capture the dir name and assert against it, or use a known directory name like `root.join("standalone")`.
+
+---
+
+## TASK-test-single-crate-module-tree: Add depth >= 2 submodule test
+
+| Criterion | Verdict | Details |
+|-----------|---------|---------|
+| Goal clear? | CLEAR | Add an integration test for single-crate mode with module depth >= 2. |
+| Files in scope correct? | CLEAR | `tests/integration_test.rs`. |
+| Implementation detail sufficient? | CLEAR | 11-step sequence provided. All helper functions confirmed to exist. The test structure mirrors the existing `test_deeply_nested_modules` (lines 284-329), which is a good reference. |
+| New types/interfaces defined clearly? | CLEAR | No new types. |
+| Dependencies explicit? | CLEAR | Depends on TASK-pipeline-fallback. |
+| Verification sufficient? | CLEAR | `cargo test --test integration_test test_single_crate_module_tree` — single test invocation. |
+| Wiring checklist | CLEAR | Empty (test change only). |
+
+### Finding
+
+- The guidance has the same "standalone" naming issue as the previous task, but less critically: `setup_crate(root, "pub mod helpers;")` will create a crate named by the tempdir's basename. The test asserts module paths like `"crate::helpers"` and `"crate::helpers::sub"` where `"crate"` should be replaced by the actual directory basename. The existing `test_deeply_nested_modules` avoids this by using `write_cargo_toml` with an explicit `name = "nested"` and then asserting against `"nested"`. The implementer should follow that pattern and NOT rely on `setup_crate` if they want a known crate name. Alternatively, the test should assert module paths relative to the actual dir name. This is not a hard blocker but will cause a test failure if followed literally.
+
+---
+
+## Task Grouping Review
+
+| Group | Dependencies | Issues |
+|-------|-------------|--------|
+| group-core | None | Line number error in TASK-single-crate-core-02 for test placement. Blocking for automated tool-based implementation; not blocking for a human. |
+| group-plumbing | group-core | Sound. TASK-pipeline-fallback and TASK-silent-exit-fix are independent. |
+| group-tests | group-plumbing | Sound. Both tests can run in parallel. |
+
+---
+
+## Summary of Issues
+
+1.  **TASK-single-crate-core-02 -- line number error (minor):** "after line 206, before resolve_crate_roots" places tests outside the `#[cfg(test)]` module. Correct insertion point is **before line 206**, not after it. Human implementers will catch this; automated code gen will not.
+
+2.  **TASK-test-existing-update -- crate name assertion is wrong (medium):** The test guidance asserts `crates[0]["name"] == "standalone"` but `setup_crate(root, ...)` creates a crate named by the tempdir's random basename. This will fail at runtime. The assertion should match the actual directory basename or the test should use a subdirectory with a known name.
+
+3.  **TASK-test-single-crate-module-tree -- implicit crate name issue (minor):** The test uses `setup_crate(root, ...)` and later asserts module paths like `"crate::helpers"`. The crate name will be the tempdir's random basename, not `"crate"`. The implementer needs to either use a known directory name or derive the expected paths dynamically. Follow the pattern of `test_deeply_nested_modules` which uses `write_cargo_toml` with an explicit name.
+
+---
+
+## Verdict
+
+**Ready to Implement with minor corrections.** The two test assertions that hardcode crate names ("standalone" and "crate") need to be fixed in the guidance or caught during implementation. The unit test line-number error is non-blocking for a human but should be corrected for tool-assisted implementation.
diff --git a/notes/directions/single-crate-support/workspace-map.json b/notes/directions/single-crate-support/workspace-map.json
new file mode 100644
index 0000000..620aba5
--- /dev/null
+++ b/notes/directions/single-crate-support/workspace-map.json
@@ -0,0 +1,600 @@
+{
+  "files": {
+    "src/lib.rs": {
+      "modulePath": "rust_workspace_map",
+      "parentModuleFile": null,
+      "isCrateRoot": true
+    },
+    "src/schema.rs": {
+      "modulePath": "rust_workspace_map::schema",
+      "parentModuleFile": "src/lib.rs",
+      "isCrateRoot": false
+    },
+    "src/workspace.rs": {
+      "modulePath": "rust_workspace_map::workspace",
+      "parentModuleFile": "src/lib.rs",
+      "isCrateRoot": false
+    },
+    "src/cargo_info.rs": {
+      "modulePath": "rust_workspace_map::cargo_info",
+      "parentModuleFile": "src/lib.rs",
+      "isCrateRoot": false
+    },
+    "src/file_parser.rs": {
+      "modulePath": "rust_workspace_map::file_parser",
+      "parentModuleFile": "src/lib.rs",
+      "isCrateRoot": false
+    },
+    "src/module_tree.rs": {
+      "modulePath": "rust_workspace_map::module_tree",
+      "parentModuleFile": "src/lib.rs",
+      "isCrateRoot": false
+    },
+    "src/cross_refs.rs": {
+      "modulePath": "rust_workspace_map::cross_refs",
+      "parentModuleFile": "src/lib.rs",
+      "isCrateRoot": false
+    },
+    "src/render.rs": {
+      "modulePath": "rust_workspace_map::render",
+      "parentModuleFile": "src/lib.rs",
+      "isCrateRoot": false
+    },
+    "src/validate.rs": {
+      "modulePath": "rust_workspace_map::validate",
+      "parentModuleFile": "src/lib.rs",
+      "isCrateRoot": false
+    },
+    "src/indexes.rs": {
+      "modulePath": "rust_workspace_map::indexes",
+      "parentModuleFile": "src/lib.rs",
+      "isCrateRoot": false
+    },
+    "src/lookup.rs": {
+      "modulePath": "rust_workspace_map::lookup",
+      "parentModuleFile": "src/lib.rs",
+      "isCrateRoot": false
+    },
+    "src/main.rs": {
+      "modulePath": "rust_workspace_map::main",
+      "parentModuleFile": null,
+      "isCrateRoot": false
+    }
+  },
+  "symbols": {
+    "rust_workspace_map::build_map": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map",
+      "file": "src/lib.rs",
+      "line": 36,
+      "kind": "fn"
+    },
+    "rust_workspace_map::run": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map",
+      "file": "src/lib.rs",
+      "line": 186,
+      "kind": "fn"
+    },
+    "rust_workspace_map::relativize_path": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map",
+      "file": "src/lib.rs",
+      "line": 205,
+      "kind": "fn"
+    },
+    "rust_workspace_map::schema::CanonicalPath": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 6,
+      "kind": "struct",
+      "deriveAttrs": ["serde::Deserialize", "serde::Serialize"]
+    },
+    "rust_workspace_map::schema::WorkspaceRelativePath": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 28,
+      "kind": "struct",
+      "deriveAttrs": ["serde::Deserialize", "serde::Serialize"]
+    },
+    "rust_workspace_map::schema::Error": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 52,
+      "kind": "enum",
+      "deriveAttrs": ["thiserror::Error"]
+    },
+    "rust_workspace_map::schema::Config": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 89,
+      "kind": "struct",
+      "deriveAttrs": ["bon::Builder"]
+    },
+    "rust_workspace_map::schema::CrateType": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 104,
+      "kind": "enum",
+      "deriveAttrs": ["serde::Serialize"]
+    },
+    "rust_workspace_map::schema::WorkspaceMap": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 115,
+      "kind": "struct",
+      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
+    },
+    "rust_workspace_map::schema::WorkspaceInfo": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 141,
+      "kind": "struct",
+      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
+    },
+    "rust_workspace_map::schema::CrateInfo": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 150,
+      "kind": "struct",
+      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
+    },
+    "rust_workspace_map::schema::PackageInfo": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 164,
+      "kind": "struct",
+      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
+    },
+    "rust_workspace_map::schema::DepInfo": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 175,
+      "kind": "struct",
+      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
+    },
+    "rust_workspace_map::schema::ModuleInfo": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 193,
+      "kind": "struct",
+      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
+    },
+    "rust_workspace_map::schema::PublicItem": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 219,
+      "kind": "struct",
+      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
+    },
+    "rust_workspace_map::schema::ItemKind": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 243,
+      "kind": "enum",
+      "deriveAttrs": ["serde::Serialize"]
+    },
+    "rust_workspace_map::schema::ItemAttrs": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 254,
+      "kind": "struct",
+      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
+    },
+    "rust_workspace_map::schema::ImplInfo": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 268,
+      "kind": "struct",
+      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
+    },
+    "rust_workspace_map::schema::ImplItem": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 277,
+      "kind": "struct",
+      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
+    },
+    "rust_workspace_map::schema::ImplItemKind": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 285,
+      "kind": "enum",
+      "deriveAttrs": ["serde::Serialize"]
+    },
+    "rust_workspace_map::schema::Import": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 295,
+      "kind": "struct",
+      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
+    },
+    "rust_workspace_map::schema::ReExport": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 302,
+      "kind": "struct",
+      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
+    },
+    "rust_workspace_map::schema::CrossCrateImport": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 312,
+      "kind": "struct",
+      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
+    },
+    "rust_workspace_map::schema::CrossReferences": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 321,
+      "kind": "struct",
+      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
+    },
+    "rust_workspace_map::schema::TypeRef": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 328,
+      "kind": "struct",
+      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
+    },
+    "rust_workspace_map::schema::DiagnosticKind": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 345,
+      "kind": "enum",
+      "deriveAttrs": ["serde::Serialize"]
+    },
+    "rust_workspace_map::schema::ErrorSeverity": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 362,
+      "kind": "enum",
+      "deriveAttrs": ["serde::Serialize"]
+    },
+    "rust_workspace_map::schema::ErrorContext": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 373,
+      "kind": "struct",
+      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
+    },
+    "rust_workspace_map::schema::FileInfo": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 394,
+      "kind": "struct",
+      "deriveAttrs": []
+    },
+    "rust_workspace_map::schema::SubmoduleDecl": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 403,
+      "kind": "struct",
+      "deriveAttrs": ["bon::Builder"]
+    },
+    "rust_workspace_map::schema::SymbolEntry": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 412,
+      "kind": "struct",
+      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
+    },
+    "rust_workspace_map::schema::FileEntry": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 426,
+      "kind": "struct",
+      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
+    },
+    "rust_workspace_map::schema::ErrorEntry": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::schema",
+      "file": "src/schema.rs",
+      "line": 437,
+      "kind": "struct",
+      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
+    },
+    "rust_workspace_map::workspace::find_workspace_root": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::workspace",
+      "file": "src/workspace.rs",
+      "line": 11,
+      "kind": "fn"
+    },
+    "rust_workspace_map::workspace::enumerate_members": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::workspace",
+      "file": "src/workspace.rs",
+      "line": 35,
+      "kind": "fn"
+    },
+    "rust_workspace_map::workspace::resolve_crate_roots": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::workspace",
+      "file": "src/workspace.rs",
+      "line": 210,
+      "kind": "fn"
+    },
+    "rust_workspace_map::cargo_info::parse_cargo_toml": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::cargo_info",
+      "file": "src/cargo_info.rs",
+      "line": 11,
+      "kind": "fn"
+    },
+    "rust_workspace_map::file_parser::parse_file": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::file_parser",
+      "file": "src/file_parser.rs",
+      "line": 35,
+      "kind": "fn"
+    },
+    "rust_workspace_map::file_parser::ParsedFile": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::file_parser",
+      "file": "src/file_parser.rs",
+      "line": 15,
+      "kind": "struct"
+    },
+    "rust_workspace_map::file_parser::SynParseError": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::file_parser",
+      "file": "src/file_parser.rs",
+      "line": 22,
+      "kind": "struct"
+    },
+    "rust_workspace_map::file_parser::extract_public_items": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::file_parser",
+      "file": "src/file_parser.rs",
+      "line": 104,
+      "kind": "fn"
+    },
+    "rust_workspace_map::file_parser::extract_imports": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::file_parser",
+      "file": "src/file_parser.rs",
+      "line": 119,
+      "kind": "fn"
+    },
+    "rust_workspace_map::file_parser::extract_re_exports": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::file_parser",
+      "file": "src/file_parser.rs",
+      "line": 139,
+      "kind": "fn"
+    },
+    "rust_workspace_map::file_parser::extract_submodules": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::file_parser",
+      "file": "src/file_parser.rs",
+      "line": 168,
+      "kind": "fn"
+    },
+    "rust_workspace_map::file_parser::extract_impls": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::file_parser",
+      "file": "src/file_parser.rs",
+      "line": 202,
+      "kind": "fn"
+    },
+    "rust_workspace_map::module_tree::resolve_module_path": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::module_tree",
+      "file": "src/module_tree.rs",
+      "line": 11,
+      "kind": "fn"
+    },
+    "rust_workspace_map::module_tree::build_module_tree": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::module_tree",
+      "file": "src/module_tree.rs",
+      "line": 27,
+      "kind": "fn"
+    },
+    "rust_workspace_map::cross_refs::compute": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::cross_refs",
+      "file": "src/cross_refs.rs",
+      "line": 12,
+      "kind": "fn"
+    },
+    "rust_workspace_map::render::render_json": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::render",
+      "file": "src/render.rs",
+      "line": 9,
+      "kind": "fn"
+    },
+    "rust_workspace_map::render::render_to_writer": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::render",
+      "file": "src/render.rs",
+      "line": 18,
+      "kind": "fn"
+    },
+    "rust_workspace_map::validate::validate": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::validate",
+      "file": "src/validate.rs",
+      "line": 72,
+      "kind": "fn"
+    },
+    "rust_workspace_map::validate::is_external_crate_re_export": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::validate",
+      "file": "src/validate.rs",
+      "line": 15,
+      "kind": "fn"
+    },
+    "rust_workspace_map::validate::check_orphan_files": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::validate",
+      "file": "src/validate.rs",
+      "line": 90,
+      "kind": "fn"
+    },
+    "rust_workspace_map::validate::check_dead_reexports": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::validate",
+      "file": "src/validate.rs",
+      "line": 189,
+      "kind": "fn"
+    },
+    "rust_workspace_map::validate::is_derive_companion": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::validate",
+      "file": "src/validate.rs",
+      "line": 254,
+      "kind": "fn"
+    },
+    "rust_workspace_map::validate::resolve_import_path": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::validate",
+      "file": "src/validate.rs",
+      "line": 278,
+      "kind": "fn"
+    },
+    "rust_workspace_map::indexes::derive_from_crates": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::indexes",
+      "file": "src/indexes.rs",
+      "line": 28,
+      "kind": "fn"
+    },
+    "rust_workspace_map::lookup::lookup_symbol": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::lookup",
+      "file": "src/lookup.rs",
+      "line": 47,
+      "kind": "fn"
+    },
+    "rust_workspace_map::lookup::lookup_file": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::lookup",
+      "file": "src/lookup.rs",
+      "line": 88,
+      "kind": "fn"
+    },
+    "rust_workspace_map::lookup::SymbolLookupResult": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::lookup",
+      "file": "src/lookup.rs",
+      "line": 7,
+      "kind": "enum",
+      "deriveAttrs": ["serde::Serialize"]
+    },
+    "rust_workspace_map::lookup::DisambiguationHint": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::lookup",
+      "file": "src/lookup.rs",
+      "line": 22,
+      "kind": "struct",
+      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
+    },
+    "rust_workspace_map::lookup::FileLookupResult": {
+      "crateName": "rust-workspace-map",
+      "module": "rust_workspace_map::lookup",
+      "file": "src/lookup.rs",
+      "line": 33,
+      "kind": "struct",
+      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
+    }
+  },
+  "nameIndex": {
+    "CanonicalPath": ["rust_workspace_map::schema::CanonicalPath"],
+    "WorkspaceRelativePath": ["rust_workspace_map::schema::WorkspaceRelativePath"],
+    "Error": ["rust_workspace_map::schema::Error"],
+    "Config": ["rust_workspace_map::schema::Config"],
+    "CrateType": ["rust_workspace_map::schema::CrateType"],
+    "WorkspaceMap": ["rust_workspace_map::schema::WorkspaceMap"],
+    "WorkspaceInfo": ["rust_workspace_map::schema::WorkspaceInfo"],
+    "CrateInfo": ["rust_workspace_map::schema::CrateInfo"],
+    "PackageInfo": ["rust_workspace_map::schema::PackageInfo"],
+    "DepInfo": ["rust_workspace_map::schema::DepInfo"],
+    "ModuleInfo": ["rust_workspace_map::schema::ModuleInfo"],
+    "PublicItem": ["rust_workspace_map::schema::PublicItem"],
+    "ItemKind": ["rust_workspace_map::schema::ItemKind"],
+    "ItemAttrs": ["rust_workspace_map::schema::ItemAttrs"],
+    "ImplInfo": ["rust_workspace_map::schema::ImplInfo"],
+    "ImplItem": ["rust_workspace_map::schema::ImplItem"],
+    "ImplItemKind": ["rust_workspace_map::schema::ImplItemKind"],
+    "Import": ["rust_workspace_map::schema::Import"],
+    "ReExport": ["rust_workspace_map::schema::ReExport"],
+    "CrossCrateImport": ["rust_workspace_map::schema::CrossCrateImport"],
+    "CrossReferences": ["rust_workspace_map::schema::CrossReferences"],
+    "TypeRef": ["rust_workspace_map::schema::TypeRef"],
+    "DiagnosticKind": ["rust_workspace_map::schema::DiagnosticKind"],
+    "ErrorSeverity": ["rust_workspace_map::schema::ErrorSeverity"],
+    "ErrorContext": ["rust_workspace_map::schema::ErrorContext"],
+    "FileInfo": ["rust_workspace_map::schema::FileInfo"],
+    "SubmoduleDecl": ["rust_workspace_map::schema::SubmoduleDecl"],
+    "SymbolEntry": ["rust_workspace_map::schema::SymbolEntry"],
+    "FileEntry": ["rust_workspace_map::schema::FileEntry"],
+    "ErrorEntry": ["rust_workspace_map::schema::ErrorEntry"],
+    "find_workspace_root": ["rust_workspace_map::workspace::find_workspace_root"],
+    "enumerate_members": ["rust_workspace_map::workspace::enumerate_members"],
+    "resolve_crate_roots": ["rust_workspace_map::workspace::resolve_crate_roots"],
+    "parse_cargo_toml": ["rust_workspace_map::cargo_info::parse_cargo_toml"],
+    "parse_file": ["rust_workspace_map::file_parser::parse_file"],
+    "ParsedFile": ["rust_workspace_map::file_parser::ParsedFile"],
+    "SynParseError": ["rust_workspace_map::file_parser::SynParseError"],
+    "extract_public_items": ["rust_workspace_map::file_parser::extract_public_items"],
+    "extract_imports": ["rust_workspace_map::file_parser::extract_imports"],
+    "extract_re_exports": ["rust_workspace_map::file_parser::extract_re_exports"],
+    "extract_submodules": ["rust_workspace_map::file_parser::extract_submodules"],
+    "extract_impls": ["rust_workspace_map::file_parser::extract_impls"],
+    "resolve_module_path": ["rust_workspace_map::module_tree::resolve_module_path"],
+    "build_module_tree": ["rust_workspace_map::module_tree::build_module_tree"],
+    "compute": ["rust_workspace_map::cross_refs::compute"],
+    "render_json": ["rust_workspace_map::render::render_json"],
+    "render_to_writer": ["rust_workspace_map::render::render_to_writer"],
+    "validate": ["rust_workspace_map::validate::validate"],
+    "is_external_crate_re_export": ["rust_workspace_map::validate::is_external_crate_re_export"],
+    "check_orphan_files": ["rust_workspace_map::validate::check_orphan_files"],
+    "check_dead_reexports": ["rust_workspace_map::validate::check_dead_reexports"],
+    "is_derive_companion": ["rust_workspace_map::validate::is_derive_companion"],
+    "resolve_import_path": ["rust_workspace_map::validate::resolve_import_path"],
+    "derive_from_crates": ["rust_workspace_map::indexes::derive_from_crates"],
+    "lookup_symbol": ["rust_workspace_map::lookup::lookup_symbol"],
+    "lookup_file": ["rust_workspace_map::lookup::lookup_file"],
+    "SymbolLookupResult": ["rust_workspace_map::lookup::SymbolLookupResult"],
+    "DisambiguationHint": ["rust_workspace_map::lookup::DisambiguationHint"],
+    "FileLookupResult": ["rust_workspace_map::lookup::FileLookupResult"],
+    "build_map": ["rust_workspace_map::build_map"],
+    "run": ["rust_workspace_map::run"],
+    "relativize_path": ["rust_workspace_map::relativize_path"]
+  },
+  "crossReferences": {
+    "types": {}
+  }
+}
diff --git a/src/lib.rs b/src/lib.rs
index 0e1dfa7..42224d7 100644
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -16,7 +16,7 @@ pub use schema::Config;
 use anyhow::Context;
 use rayon::prelude::*;
 use schema::{
-    CrateInfo, CrateType, DiagnosticKind, ErrorEntry, ErrorSeverity, ModuleInfo,
+    CrateInfo, CrateType, DiagnosticKind, Error, ErrorEntry, ErrorSeverity, ModuleInfo,
     WorkspaceInfo, WorkspaceMap,
 };
 use std::path::Path;
@@ -34,8 +34,25 @@ use std::path::Path;
 /// parsed, or the JSON output cannot be written.
 #[allow(clippy::too_many_lines)]
 pub fn build_map(config: &Config) -> anyhow::Result<WorkspaceMap> {
-    let workspace_root = workspace::find_workspace_root(&config.workspace_path)?;
-    let member_dirs = workspace::enumerate_members(&workspace_root)?;
+    let (workspace_root, member_dirs) = match workspace::find_workspace_root(&config.workspace_path) {
+        Ok(root) => {
+            match workspace::enumerate_members(&root) {
+                Ok(members) => (root, members),
+                Err(Error::MissingWorkspaceSection) => {
+                    // [workspace] was a false positive (e.g., in a comment).
+                    // Fall back to single-crate discovery.
+                    let crate_dir = workspace::find_crate_root(&config.workspace_path)?;
+                    (crate_dir.clone(), vec![crate_dir])
+                }
+                Err(other) => return Err(other.into()),
+            }
+        }
+        Err(Error::WorkspaceRootNotFound(_)) => {
+            let crate_dir = workspace::find_crate_root(&config.workspace_path)?;
+            (crate_dir.clone(), vec![crate_dir])
+        }
+        Err(other) => return Err(other.into()),
+    };
 
     let mut crate_errors: Vec<ErrorEntry> = Vec::new();
 
diff --git a/src/main.rs b/src/main.rs
index 7cd18b1..6d475e9 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -85,12 +85,14 @@ fn main() {
                             }
                         };
                         let writer = std::io::BufWriter::new(file);
-                        if rust_workspace_map::render::render_to_writer(&map, writer).is_err() {
+                        if let Err(e) = rust_workspace_map::render::render_to_writer(&map, writer) {
+                            eprintln!("error writing JSON to file: {e}");
                             std::process::exit(1);
                         }
                     } else {
                         let stdout = std::io::stdout();
-                        if rust_workspace_map::render::render_to_writer(&map, stdout.lock()).is_err() {
+                        if let Err(e) = rust_workspace_map::render::render_to_writer(&map, stdout.lock()) {
+                            eprintln!("error writing JSON to stdout: {e}");
                             std::process::exit(1);
                         }
                     }
@@ -99,7 +101,8 @@ fn main() {
                         std::process::exit(2);
                     }
                 }
-                Err(_) => {
+                Err(e) => {
+                    eprintln!("error: {e:#}");
                     std::process::exit(1);
                 }
             }
@@ -162,7 +165,8 @@ fn main() {
                         }
                     }
                 }
-                Err(_) => {
+                Err(e) => {
+                    eprintln!("error: {e:#}");
                     std::process::exit(1);
                 }
             }
diff --git a/src/schema.rs b/src/schema.rs
index aacdf82..4482ee4 100644
--- a/src/schema.rs
+++ b/src/schema.rs
@@ -54,6 +54,9 @@ pub enum Error {
     #[error("no workspace root found starting from {0}")]
     WorkspaceRootNotFound(PathBuf),
 
+    #[error("no Cargo.toml with [package] section found starting from {0}")]
+    CrateRootNotFound(PathBuf),
+
     #[error("failed to read file {path}: {source}")]
     FileRead {
         path: PathBuf,
diff --git a/src/workspace.rs b/src/workspace.rs
index 48b0331..4172f05 100644
--- a/src/workspace.rs
+++ b/src/workspace.rs
@@ -24,6 +24,30 @@ pub fn find_workspace_root(start_path: &Path) -> Result<PathBuf> {
     Err(Error::WorkspaceRootNotFound(start_path.to_path_buf()))
 }
 
+/// Walk up the directory tree from `start_path` to find a `Cargo.toml`
+/// containing a `[package]` section. Returns the directory containing it.
+///
+/// # Errors
+///
+/// Returns `Error::CrateRootNotFound` if no `Cargo.toml` with a `[package]`
+/// section is found in any ancestor directory.
+pub fn find_crate_root(start_path: &Path) -> Result<PathBuf> {
+    for ancestor in start_path.ancestors() {
+        let cargo_toml = ancestor.join("Cargo.toml");
+        if cargo_toml.exists() {
+            let content =
+                std::fs::read_to_string(&cargo_toml).map_err(|source| Error::FileRead {
+                    path: cargo_toml.clone(),
+                    source,
+                })?;
+            if content.contains("[package]") {
+                return Ok(ancestor.to_path_buf());
+            }
+        }
+    }
+    Err(Error::CrateRootNotFound(start_path.to_path_buf()))
+}
+
 /// Parse the workspace `Cargo.toml`, resolve member paths (including glob
 /// patterns), apply `exclude` list, and return absolute paths to each member
 /// crate directory.
@@ -203,6 +227,29 @@ exclude = ["b"]
         assert_eq!(roots.len(), 1);
         assert_eq!(roots[0].1, CrateType::Bin);
     }
+
+    #[test]
+    fn find_crate_root_finds_package_section() {
+        let tmp = tempfile::tempdir().unwrap();
+        setup_crate(tmp.path());
+        // Walk from a nested subdirectory inside src
+        let nested = tmp.path().join("src").join("subdir");
+        std::fs::create_dir_all(&nested).unwrap();
+        let result = find_crate_root(&nested).unwrap();
+        assert_eq!(result, tmp.path());
+    }
+
+    #[test]
+    fn find_crate_root_returns_err_for_no_package() {
+        let tmp = tempfile::tempdir().unwrap();
+        // No Cargo.toml at all — ancestors() walks up and finds nothing
+        let result = find_crate_root(tmp.path());
+        assert!(result.is_err());
+        match result.unwrap_err() {
+            Error::CrateRootNotFound(_) => {},
+            other => panic!("expected CrateRootNotFound, got {:?}", other),
+        }
+    }
 }
 /// Returns `(path, CrateType)` pairs — one for `src/lib.rs` (Lib),
 /// one for `src/main.rs` (Bin), or empty if neither exists.
diff --git a/tests/integration_test.rs b/tests/integration_test.rs
index 274eea4..5f5c729 100644
--- a/tests/integration_test.rs
+++ b/tests/integration_test.rs
@@ -197,29 +197,87 @@ members = ["good_crate", "bad_crate"]
 }
 
 #[test]
-fn test_missing_workspace_section() {
+fn test_single_crate_without_workspace() {
     let tmp = tempfile::tempdir().unwrap();
     let root = tmp.path();
+    let crate_dir = root.join("standalone");
 
-    // Cargo.toml without [workspace] section
-    write_cargo_toml(root, r#"
-[package]
-name = "standalone"
-version = "0.1.0"
-edition = "2021"
-"#);
+    setup_crate(&crate_dir, "pub struct Standalone { pub x: i32 }");
 
-    let output = Command::new(&binary_path())
-        .arg("index")
-        .arg(root.to_str().unwrap())
-        .output();
+    let output = run_index(crate_dir.to_str().unwrap());
+    assert!(output.status.success(), "standalone crate should succeed: {}", String::from_utf8_lossy(&output.stderr));
+
+    let json = parse_output(&output);
+    let crates = extract_array(&json, "crates");
+    assert_eq!(crates.len(), 1, "should have exactly one crate");
+
+    assert_eq!(crates[0]["name"], "standalone");
+
+    let has_standalone = crates[0]["modules"]
+        .as_array()
+        .unwrap()
+        .iter()
+        .flat_map(|m| m["publicItems"].as_array().unwrap())
+        .any(|item| item["name"] == "Standalone" && item["kind"] == "struct");
+    assert!(has_standalone, "should find pub struct Standalone");
+}
+
+#[test]
+fn test_single_crate_module_tree() {
+    let tmp = tempfile::tempdir().unwrap();
+    let root = tmp.path().join("single-crate");
+    std::fs::create_dir_all(&root).unwrap();
+
+    setup_crate(&root, "pub mod helpers;");
+
+    // Create depth-1 module
+    let helpers_dir = root.join("src").join("helpers");
+    std::fs::create_dir_all(&helpers_dir).unwrap();
+    std::fs::write(helpers_dir.join("mod.rs"), "pub mod sub;").unwrap();
+
+    // Create depth-2 module with a public item
+    std::fs::write(helpers_dir.join("sub.rs"), "pub fn assist() {}").unwrap();
+
+    let output = run_index(root.to_str().unwrap());
+    assert!(output.status.success(), "single-crate module tree should succeed: {}", String::from_utf8_lossy(&output.stderr));
+
+    let json = parse_output(&output);
+    let crates = extract_array(&json, "crates");
+    assert_eq!(crates.len(), 1);
+
+    let crate_info = &crates[0];
+    assert_eq!(crate_info["name"], "single-crate");
+
+    let modules = extract_array(crate_info, "modules");
+    let module_paths: Vec<&str> = modules
+        .iter()
+        .map(|m| m["path"].as_str().unwrap())
+        .collect();
 
-    // Should exit non-zero because workspace is missing
-    let output = output.expect("failed to execute binary");
     assert!(
-        !output.status.success(),
-        "should exit non-zero for missing workspace section"
+        module_paths.iter().any(|p| *p == "single-crate"),
+        "should have root module"
     );
+    assert!(
+        module_paths.iter().any(|p| *p == "single-crate::helpers"),
+        "should have depth-1 module"
+    );
+    assert!(
+        module_paths.iter().any(|p| *p == "single-crate::helpers::sub"),
+        "should have depth-2 module"
+    );
+
+    // Verify the depth-2 module has the `assist` function
+    let sub_module = modules
+        .iter()
+        .find(|m| m["path"].as_str().unwrap() == "single-crate::helpers::sub")
+        .unwrap();
+    let has_assist = sub_module["publicItems"]
+        .as_array()
+        .unwrap()
+        .iter()
+        .any(|item| item["name"] == "assist" && item["kind"] == "fn");
+    assert!(has_assist, "helpers::sub should have pub fn assist");
 }
 
 #[test]

## File: .gitignore
# Generated by Cargo
# will have compiled files and executables
debug
target

# These are backup files generated by rustfmt
**/*.rs.bk

# MSVC Windows builds of rustc generate these, which store debugging information
*.pdb

# Generated by cargo mutants
# Contains mutation testing data
**/mutants.out*/

# RustRover
#  JetBrains specific template is maintained in a separate JetBrains.gitignore that can
#  be found at https://github.com/github/gitignore/blob/main/Global/JetBrains.gitignore
#  and can be added to the global gitignore or merged into this file.  For a more nuclear
#  option (not recommended) you can uncomment the following to ignore the entire idea folder.
#.idea/
.pipeline-worktrees/
.exploration_checkpoint.json
.claude/
## File: notes/directions/single-crate-support/codebase-state.md
# Codebase State -- Single-Crate Support Phase

> Generated: 2026-05-06 | Phase: single-crate-support

## 1. File Tree (Affected Areas)

```
src/
  schema.rs     (451 lines)  -- Error enum, all schema types
  workspace.rs  (222 lines)  -- find_workspace_root, enumerate_members, resolve_crate_roots
  lib.rs        (212 lines)  -- build_map, run, relativize_path (pipeline orchestrator)
  main.rs       (172 lines)  -- CLI entry point
  validate.rs   (664 lines)  -- validate, check_orphan_files, check_dead_reexports
  cargo_info.rs (156 lines)  -- parse_cargo_toml
  module_tree.rs(307 lines)  -- build_module_tree, resolve_module_path
  cross_refs.rs(213 lines)   -- compute (cross-crate references)
  indexes.rs   (287 lines)   -- derive_from_crates
  render.rs    ( 91 lines)   -- render_json, render_to_writer
  lookup.rs    (206 lines)   -- lookup_symbol, lookup_file
tests/
  integration_test.rs (660 lines) -- all integration tests
  fixtures/
    sample-workspace/   -- workspace with [workspace] section
    bad-orphan/         -- workspace with orphan .rs files
    bad-dead-reexport/  -- workspace with dead re-exports
```

## 2. Key Type/Function Signatures

### schema.rs (Error enum, lines 52-83)
```rust
pub enum Error {
    WorkspaceRootNotFound(PathBuf),      // line 54
    // NEW variant to add after line 55: CrateRootNotFound(PathBuf)
    FileRead { path: PathBuf, source: std::io::Error },
    TomlParse { path: PathBuf, source: toml::de::Error },
    SynParse { path: PathBuf, source: syn::Error },
    MemberNotFound(PathBuf),
    GlobPattern(String),
    MissingWorkspaceSection,
}
pub type Result<T> = std::result::Result<T, Error>;
```

### workspace.rs (discovery functions)
```rust
pub fn find_workspace_root(start_path: &Path) -> Result<PathBuf>          // line 11
pub fn enumerate_members(root: &Path) -> Result<Vec<PathBuf>>             // line 35
pub fn resolve_crate_roots(crate_dir: &Path) -> Vec<(PathBuf, CrateType)> // line 210
```

### lib.rs (pipeline)
```rust
pub fn build_map(config: &Config) -> anyhow::Result<WorkspaceMap>  // line 36
pub fn run(config: &Config) -> anyhow::Result<()>                  // line 186
fn relativize_path(path_str: &str, root: &Path) -> String          // line 205
```

### main.rs (CLI) -- silent error exits
Lines 88-90: file write `.is_err()` -> silent `exit(1)`
Lines 93-95: stdout write `.is_err()` -> silent `exit(1)`
Lines 102-104: pipeline error `Err(_)` -> silent `exit(1)`
Lines 165-167: lookup error `Err(_)` -> silent `exit(1)`

## 3. Module Dependency Graph

```
main.rs
  └── lib.rs::build_map, lib.rs::run, render::render_to_writer, lookup::lookup_*

lib.rs::build_map
  ├── workspace::find_workspace_root   (dependency: schema::Error)
  ├── workspace::enumerate_members     (dependency: schema::Error, glob)
  ├── cargo_info::parse_cargo_toml     (dependency: schema::*)
  ├── workspace::resolve_crate_roots
  ├── module_tree::build_module_tree   (dependency: file_parser::parse_file)
  ├── cross_refs::compute
  ├── indexes::derive_from_crates
  ├── validate::validate               (dependency: check_orphan_files, check_dead_reexports)
  └── render::render_to_writer
```

## 4. Existing Test Structure

### Unit tests per module
- `workspace.rs` (6 tests): `find_workspace_root_finds_cargo_toml`, `enumerate_members_returns_members`, `enumerate_members_returns_err_for_missing_workspace`, `enumerate_members_applies_exclude`, `resolve_crate_roots_detects_lib`, `resolve_crate_roots_detects_bin`
- `cargo_info.rs` (3 tests), `module_tree.rs` (4 tests), `cross_refs.rs` (2 tests), `indexes.rs` (5 tests), `validate.rs` (13 tests), `render.rs` (3 tests), `lookup.rs` (4 tests)

### Integration tests (18 tests)
All in `tests/integration_test.rs`. Key ones affected by this phase:
- `test_missing_workspace_section` (line 199): expects exit 1 for crate without [workspace] — WILL BREAK after fix
- `test_deeply_nested_modules` (line 284): uses workspace mode with `members = ["."]`

## 5. Validate Path Safety Analysis

### check_orphan_files (validate.rs:90-158)
`crate_root_path = workspace_root.join(&crate_info.root)`

**In workspace mode:** workspace_root=/ws, crate_info.root="core/src/lib.rs" -> /ws/core/src/lib.rs
**In single-crate mode:** workspace_root=/crate, crate_info.root="src/lib.rs" -> /crate/src/lib.rs

Both modes produce correct paths. The `strip_prefix(workspace_root)` call at line 135 also works identically. **Safe.**

## 6. Key Observations

1. **lib.rs does NOT import `schema::Error`** (lines 18-21 only import specific types). The plan's Step 4a (add Error to import) is required for the match pattern to compile.
2. **The plan's D1 enhancement** (catch MissingWorkspaceSection too) is recommended by the architect. The match becomes: `Ok(root)` -> try `enumerate_members`, catch `MissingWorkspaceSection` -> fallback.
3. **No additional silent-exit sites** beyond the four identified.
4. **validate.rs path safety is confirmed** — no changes needed.
5. **Four silent-exit serialization errors in Lookup branch** (lines 123-128, 140-145) already print messages on stderr so are NOT "silent". Not in scope.
## File: notes/directions/single-crate-support/deferred-and-patterns.md
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
## File: notes/directions/single-crate-support/directions-index.json
{
  "meta": {
    "title": "Single-Crate Support + Fix Silent Error Handling",
    "source_branch": "single-crate-support"
  },
  "architecture_notes": [
    "Auto-detect: try workspace discovery first; on WorkspaceRootNotFound or MissingWorkspaceSection, fall back to single-crate discovery. No new CLI flags.",
    "Two separate functions: find_workspace_root (looks for [workspace]) and find_crate_root (looks for [package]). A combined function would stop at a workspace member's Cargo.toml.",
    "D1 enhancement: catch MissingWorkspaceSection from enumerate_members as a fallback trigger, not just WorkspaceRootNotFound. This handles the false positive where Cargo.toml contains '[workspace]' as a comment or string literal.",
    "validate::validate path safety confirmed \u2014 check_orphan_files joins workspace_root with crate_info.root which is always workspace-relative; safe in both multi-crate and single-crate modes.",
    "workspace_name in single-crate mode will be the crate directory name (e.g., 'rust-workspace-map') derived from workspace_root.file_name().",
    "find_crate_root must structurally mirror find_workspace_root: same ancestors() walk, same FileRead error mapping, same .contains() check style, same doc-comment structure with # Errors section.",
    "Orchestration lives in lib.rs:build_map \u2014 workspace.rs remains pure with single-responsibility functions.",
    "All changes stay within the single rust-workspace-map crate. No new crates introduced.",
    "Library code in this phase follows ch12-04 TDD: tests claim interfaces first via tdd_interface, then implementations evolve to meet them. See the tdd-pattern.md reference for the full workflow."
  ],
  "known_pitfalls": [
    "lib.rs line 18-21 does NOT import schema::Error \u2014 must add Error to the use schema::{...} import for the match pattern in TASK-pipeline-fallback.",
    "relativize_path silently returns original path on strip_prefix failure \u2014 no error propagation, works correctly because crate_info.root paths are always workspace-relative.",
    "workspace_name is derived from workspace_root.file_name().unwrap_or_default(). In single-crate mode this is the crate directory name, which is reasonable.",
    "D1 enhancement nests a second match inside the Ok(root) arm: match enumerate_members, catch MissingWorkspaceSection as fallback trigger alongside the outer WorkspaceRootNotFound.",
    "MissingWorkspaceSection false positive: Cargo.toml with '[workspace]' in a comment or string literal triggers find_workspace_root to return Ok, but enumerate_members fails. The D1 enhancement catches this.",
    "TASK-test-single-crate-module-tree MUST exercise depth >= 2 submodule resolution (e.g., lib -> helpers/mod.rs -> helpers/sub.rs).",
    "Do NOT touch these modules: cargo_info.rs, file_parser.rs, module_tree.rs, cross_refs.rs, render.rs, validate.rs, indexes.rs, lookup.rs.",
    "No new CLI flags \u2014 the feature is auto-detect only. Do not add --single-crate or similar flags.",
    "Both find_workspace_root and find_crate_root use .contains() (string match) rather than toml parsing. This is consistent with existing code. The D1 MissingWorkspaceSection fallback mitigates the false-positive risk.",
    "unit tests in workspace.rs use the existing write_cargo_toml and setup_crate helpers (one-param version). Integration tests use the separate setup_crate(dir, lib_content) helper (two-param version)."
  ],
  "groups": [
    {
      "group_id": "group-core",
      "task_ids": [
        "TASK-single-crate-core-01",
        "TASK-single-crate-core-02"
      ],
      "description": "All tasks modify shared schema.rs/workspace.rs files \u2014 the core discovery enhancement. TASK-single-crate-core-01 is a prerequisite for TASK-single-crate-core-02 (the error variant must exist before the function compiles).",
      "file": "directions-single-crate-support-group-core.json"
    },
    {
      "group_id": "group-fallback",
      "task_ids": [
        "TASK-pipeline-fallback"
      ],
      "description": "Pipeline fallback logic depends on group-core (needs find_crate_root and CrateRootNotFound).",
      "file": "directions-single-crate-support-group-fallback.json"
    },
    {
      "group_id": "group-silent-exit",
      "task_ids": [
        "TASK-silent-exit-fix"
      ],
      "description": "Self-contained fix for silent error exits in main.rs \u2014 no dependencies on other groups. Can run in parallel with group-fallback once group-core completes.",
      "file": "directions-single-crate-support-group-silent-exit.json"
    },
    {
      "group_id": "group-tests",
      "task_ids": [
        "TASK-test-existing-update",
        "TASK-test-single-crate-module-tree"
      ],
      "description": "Both are integration test changes that verify the feature. They depend on all production code changes being in place. Can run in parallel with each other.",
      "file": "directions-single-crate-support-group-tests.json"
    }
  ]
}
## File: notes/directions/single-crate-support/directions-single-crate-support-group-core.json
{
  "meta": {
    "title": "Single-Crate Support + Fix Silent Error Handling",
    "source_branch": "single-crate-support"
  },
  "architecture_notes": [
    "Auto-detect: try workspace discovery first; on WorkspaceRootNotFound or MissingWorkspaceSection, fall back to single-crate discovery. No new CLI flags.",
    "Two separate functions: find_workspace_root (looks for [workspace]) and find_crate_root (looks for [package]). A combined function would stop at a workspace member's Cargo.toml.",
    "D1 enhancement: catch MissingWorkspaceSection from enumerate_members as a fallback trigger, not just WorkspaceRootNotFound. This handles the false positive where Cargo.toml contains '[workspace]' as a comment or string literal.",
    "validate::validate path safety confirmed \u2014 check_orphan_files joins workspace_root with crate_info.root which is always workspace-relative; safe in both multi-crate and single-crate modes.",
    "workspace_name in single-crate mode will be the crate directory name (e.g., 'rust-workspace-map') derived from workspace_root.file_name().",
    "find_crate_root must structurally mirror find_workspace_root: same ancestors() walk, same FileRead error mapping, same .contains() check style, same doc-comment structure with # Errors section.",
    "Orchestration lives in lib.rs:build_map \u2014 workspace.rs remains pure with single-responsibility functions.",
    "All changes stay within the single rust-workspace-map crate. No new crates introduced.",
    "Library code in this phase follows ch12-04 TDD: tests claim interfaces first via tdd_interface, then implementations evolve to meet them. See the tdd-pattern.md reference for the full workflow."
  ],
  "known_pitfalls": [
    "lib.rs line 18-21 does NOT import schema::Error \u2014 must add Error to the use schema::{...} import for the match pattern in TASK-pipeline-fallback.",
    "relativize_path silently returns original path on strip_prefix failure \u2014 no error propagation, works correctly because crate_info.root paths are always workspace-relative.",
    "workspace_name is derived from workspace_root.file_name().unwrap_or_default(). In single-crate mode this is the crate directory name, which is reasonable.",
    "D1 enhancement nests a second match inside the Ok(root) arm: match enumerate_members, catch MissingWorkspaceSection as fallback trigger alongside the outer WorkspaceRootNotFound.",
    "MissingWorkspaceSection false positive: Cargo.toml with '[workspace]' in a comment or string literal triggers find_workspace_root to return Ok, but enumerate_members fails. The D1 enhancement catches this.",
    "TASK-test-single-crate-module-tree MUST exercise depth >= 2 submodule resolution (e.g., lib -> helpers/mod.rs -> helpers/sub.rs).",
    "Do NOT touch these modules: cargo_info.rs, file_parser.rs, module_tree.rs, cross_refs.rs, render.rs, validate.rs, indexes.rs, lookup.rs.",
    "No new CLI flags \u2014 the feature is auto-detect only. Do not add --single-crate or similar flags.",
    "Both find_workspace_root and find_crate_root use .contains() (string match) rather than toml parsing. This is consistent with existing code. The D1 MissingWorkspaceSection fallback mitigates the false-positive risk.",
    "unit tests in workspace.rs use the existing write_cargo_toml and setup_crate helpers (one-param version). Integration tests use the separate setup_crate(dir, lib_content) helper (two-param version)."
  ],
  "task_groups": [
    {
      "group_id": "group-core",
      "reason": "All tasks modify shared schema.rs/workspace.rs files \u2014 the core discovery enhancement. TASK-single-crate-core-01 is a prerequisite for TASK-single-crate-core-02 (the error variant must exist before the function compiles).",
      "tasks": [
        "TASK-single-crate-core-01",
        "TASK-single-crate-core-02"
      ],
      "depends_on_groups": []
    }
  ],
  "tasks": [
    {
      "id": "TASK-single-crate-core-01",
      "kind": "direct",
      "description": "Add CrateRootNotFound variant to schema::Error enum so find_crate_root can signal discovery failure.",
      "files_in_scope": [
        "src/schema.rs"
      ],
      "changes": [
        {
          "path": "src/schema.rs",
          "action": "modify",
          "guidance": "Add a new error variant to the Error enum. Insert it after the WorkspaceRootNotFound variant (line 55), before the FileRead variant (line 57):\n\n```rust\n#[error(\"no Cargo.toml with [package] section found starting from {0}\")]\nCrateRootNotFound(PathBuf),\n```\n\nThis variant carries a PathBuf (the start_path that was searched), matching the pattern of WorkspaceRootNotFound. The thiserror attribute format is consistent with other variants. The variant is automatically reachable \u2014 schema::Error is already imported by workspace.rs via `use crate::schema::{CrateType, Error, Result}`."
        }
      ],
      "wiring_checklist": [],
      "type_reference": {
        "CrateRootNotFound": "#[error(\"no Cargo.toml with [package] section found starting from {0}\")] CrateRootNotFound(PathBuf)"
      },
      "acceptance": [
        "cargo check --workspace"
      ],
      "depends_on": []
    },
    {
      "id": "TASK-single-crate-core-02",
      "kind": "lib-tdd",
      "description": "Add find_crate_root discovery function + 2 unit tests via test-driven development.",
      "files_in_scope": [
        "src/workspace.rs"
      ],
      "changes": [
        {
          "path": "src/workspace.rs",
          "action": "modify",
          "guidance": "Implement find_crate_root following the TDD cycle. The test (tdd_interface.test_code) defines the contract \u2014 write it first, confirm failure, then implement.\n\nImplementation approach: Walk up the directory tree using start_path.ancestors(). For each ancestor, check if Cargo.toml exists. If it does, read the file contents. Map I/O errors to Error::FileRead { path, source }. Check if the content contains the string \"[package]\". If found, return Ok(ancestor.to_path_buf()). If the loop completes without finding a [package] section, return Err(Error::CrateRootNotFound(start_path.to_path_buf())).\n\nPlacement: Insert after find_workspace_root (after line 25), before enumerate_members (line 35). Use the same doc-comment structure as find_workspace_root: /// description, /// # Errors section. The function must be pub.\n\nFor the unit tests: Add both test functions inside the existing #[cfg(test)] mod tests block, before its closing } at line 206. The test module already has write_cargo_toml and setup_crate (one-param) helpers via `use super::*`. No new imports needed.\n\nEdge cases handled: empty directory tree (ancestors() always includes start_path itself), Cargo.toml without [package] (continues walking), I/O error reading Cargo.toml (returns FileRead error)."
        }
      ],
      "tdd_interface": {
        "test_file": "src/workspace.rs",
        "test_module": "tests",
        "test_fn_name": "find_crate_root_finds_package_section",
        "test_code": "#[test]\nfn find_crate_root_finds_package_section() {\n    let tmp = tempfile::tempdir().unwrap();\n    setup_crate(tmp.path());\n    // Walk from a nested subdirectory inside src\n    let nested = tmp.path().join(\"src\").join(\"subdir\");\n    std::fs::create_dir_all(&nested).unwrap();\n    let result = find_crate_root(&nested).unwrap();\n    assert_eq!(result, tmp.path());\n}\n\n#[test]\nfn find_crate_root_returns_err_for_no_package() {\n    let tmp = tempfile::tempdir().unwrap();\n    // No Cargo.toml at all \u2014 ancestors() walks up and finds nothing\n    let result = find_crate_root(tmp.path());\n    assert!(result.is_err());\n    match result.unwrap_err() {\n        Error::CrateRootNotFound(_) => {},\n        other => panic!(\"expected CrateRootNotFound, got {:?}\", other),\n    }\n}",
        "signature": "pub fn find_crate_root(start_path: &Path) -> Result<PathBuf>",
        "expected_behavior": "find_crate_root walks up from start_path through ancestors, returns the first directory whose Cargo.toml contains a [package] section. Returns CrateRootNotFound error if no Cargo.toml with [package] is found in any ancestor (including start_path itself). Returns FileRead error if a Cargo.toml exists but cannot be read."
      },
      "wiring_checklist": [],
      "type_reference": {
        "find_crate_root": "pub fn find_crate_root(start_path: &Path) -> Result<PathBuf>"
      },
      "acceptance": [
        "cargo test find_crate_root_finds_package_section",
        "cargo test find_crate_root_returns_err_for_no_package",
        "cargo check --workspace"
      ],
      "depends_on": [
        "TASK-single-crate-core-01"
      ]
    }
  ]
}
## File: notes/directions/single-crate-support/directions-single-crate-support-group-fallback.json
{
  "meta": {
    "title": "Single-Crate Support + Fix Silent Error Handling",
    "source_branch": "single-crate-support"
  },
  "architecture_notes": [
    "Auto-detect: try workspace discovery first; on WorkspaceRootNotFound or MissingWorkspaceSection, fall back to single-crate discovery. No new CLI flags.",
    "Two separate functions: find_workspace_root (looks for [workspace]) and find_crate_root (looks for [package]). A combined function would stop at a workspace member's Cargo.toml.",
    "D1 enhancement: catch MissingWorkspaceSection from enumerate_members as a fallback trigger, not just WorkspaceRootNotFound. This handles the false positive where Cargo.toml contains '[workspace]' as a comment or string literal.",
    "validate::validate path safety confirmed \u2014 check_orphan_files joins workspace_root with crate_info.root which is always workspace-relative; safe in both multi-crate and single-crate modes.",
    "workspace_name in single-crate mode will be the crate directory name (e.g., 'rust-workspace-map') derived from workspace_root.file_name().",
    "find_crate_root must structurally mirror find_workspace_root: same ancestors() walk, same FileRead error mapping, same .contains() check style, same doc-comment structure with # Errors section.",
    "Orchestration lives in lib.rs:build_map \u2014 workspace.rs remains pure with single-responsibility functions.",
    "All changes stay within the single rust-workspace-map crate. No new crates introduced.",
    "Library code in this phase follows ch12-04 TDD: tests claim interfaces first via tdd_interface, then implementations evolve to meet them. See the tdd-pattern.md reference for the full workflow."
  ],
  "known_pitfalls": [
    "lib.rs line 18-21 does NOT import schema::Error \u2014 must add Error to the use schema::{...} import for the match pattern in TASK-pipeline-fallback.",
    "relativize_path silently returns original path on strip_prefix failure \u2014 no error propagation, works correctly because crate_info.root paths are always workspace-relative.",
    "workspace_name is derived from workspace_root.file_name().unwrap_or_default(). In single-crate mode this is the crate directory name, which is reasonable.",
    "D1 enhancement nests a second match inside the Ok(root) arm: match enumerate_members, catch MissingWorkspaceSection as fallback trigger alongside the outer WorkspaceRootNotFound.",
    "MissingWorkspaceSection false positive: Cargo.toml with '[workspace]' in a comment or string literal triggers find_workspace_root to return Ok, but enumerate_members fails. The D1 enhancement catches this.",
    "TASK-test-single-crate-module-tree MUST exercise depth >= 2 submodule resolution (e.g., lib -> helpers/mod.rs -> helpers/sub.rs).",
    "Do NOT touch these modules: cargo_info.rs, file_parser.rs, module_tree.rs, cross_refs.rs, render.rs, validate.rs, indexes.rs, lookup.rs.",
    "No new CLI flags \u2014 the feature is auto-detect only. Do not add --single-crate or similar flags.",
    "Both find_workspace_root and find_crate_root use .contains() (string match) rather than toml parsing. This is consistent with existing code. The D1 MissingWorkspaceSection fallback mitigates the false-positive risk.",
    "unit tests in workspace.rs use the existing write_cargo_toml and setup_crate helpers (one-param version). Integration tests use the separate setup_crate(dir, lib_content) helper (two-param version)."
  ],
  "task_groups": [
    {
      "group_id": "group-fallback",
      "reason": "Pipeline fallback logic depends on group-core (needs find_crate_root and CrateRootNotFound).",
      "tasks": [
        "TASK-pipeline-fallback"
      ],
      "depends_on_groups": [
        "group-core"
      ]
    }
  ],
  "tasks": [
    {
      "id": "TASK-pipeline-fallback",
      "kind": "direct",
      "description": "Modify build_map in lib.rs to add fallback logic: try workspace discovery first, fall back to single-crate on WorkspaceRootNotFound or MissingWorkspaceSection.",
      "files_in_scope": [
        "src/lib.rs"
      ],
      "changes": [
        {
          "path": "src/lib.rs",
          "action": "modify",
          "guidance": "Two changes to src/lib.rs:\n\n**(1) Add Error import**: On line 18-21, the `use schema::{...}` block currently imports CrateInfo, CrateType, DiagnosticKind, ErrorEntry, ErrorSeverity, ModuleInfo, WorkspaceInfo, WorkspaceMap. Add `Error` to this import. The block should include `Error` alongside the existing types.\n\n**(2) Replace workspace discovery lines (37-38) with fallback match**: Replace the two lines:\n```rust\nlet workspace_root = workspace::find_workspace_root(&config.workspace_path)?;\nlet member_dirs = workspace::enumerate_members(&workspace_root)?;\n```\n\nWith a match expression that tries workspace discovery first, then falls back. The structure:\n```rust\nlet (workspace_root, member_dirs) = match workspace::find_workspace_root(&config.workspace_path) {\n    Ok(root) => {\n        match workspace::enumerate_members(&root) {\n            Ok(members) => (root, members),\n            Err(Error::MissingWorkspaceSection) => {\n                // [workspace] was a false positive (e.g., in a comment).\n                // Fall back to single-crate discovery.\n                let crate_dir = workspace::find_crate_root(&config.workspace_path)?;\n                (crate_dir.clone(), vec![crate_dir])\n            }\n            Err(other) => return Err(other.into()),\n        }\n    }\n    Err(Error::WorkspaceRootNotFound(_)) => {\n        let crate_dir = workspace::find_crate_root(&config.workspace_path)?;\n        (crate_dir.clone(), vec![crate_dir])\n    }\n    Err(other) => return Err(other.into()),\n};\n```\n\nKey points:\n- The inner match on enumerate_members handles the D1 false-positive case where Cargo.toml contains \"[workspace]\" as a literal string but no actual workspace section.\n- The fallback path is identical for both MissingWorkspaceSection and WorkspaceRootNotFound: find_crate_root then create a single-element vec.\n- Other errors (TomlParse, FileRead, etc.) propagate as fatal.\n- workspace_root is set to crate_dir in fallback mode; member_dirs is vec![crate_dir].\n- The rest of build_map (lines 40-171) works unchanged \u2014 it uses workspace_root and member_dirs identically regardless of discovery path.\n\nThe function signature and return type do NOT change."
        }
      ],
      "wiring_checklist": [
        {
          "kind": "fn_call",
          "file": "src/lib.rs",
          "detail": "build_map calls workspace::find_crate_root in two fallback branches (MissingWorkspaceSection and WorkspaceRootNotFound)"
        },
        {
          "kind": "type_annotation",
          "file": "src/lib.rs",
          "detail": "schema::Error is imported for the match patterns on Error::WorkspaceRootNotFound and Error::MissingWorkspaceSection"
        }
      ],
      "acceptance": [
        "cargo check --workspace",
        "cargo clippy -- -D warnings"
      ],
      "depends_on": [
        "TASK-single-crate-core-02"
      ]
    }
  ]
}
## File: notes/directions/single-crate-support/directions-single-crate-support-group-silent-exit.json
{
  "meta": {
    "title": "Single-Crate Support + Fix Silent Error Handling",
    "source_branch": "single-crate-support"
  },
  "architecture_notes": [
    "Auto-detect: try workspace discovery first; on WorkspaceRootNotFound or MissingWorkspaceSection, fall back to single-crate discovery. No new CLI flags.",
    "Two separate functions: find_workspace_root (looks for [workspace]) and find_crate_root (looks for [package]). A combined function would stop at a workspace member's Cargo.toml.",
    "D1 enhancement: catch MissingWorkspaceSection from enumerate_members as a fallback trigger, not just WorkspaceRootNotFound. This handles the false positive where Cargo.toml contains '[workspace]' as a comment or string literal.",
    "validate::validate path safety confirmed \u2014 check_orphan_files joins workspace_root with crate_info.root which is always workspace-relative; safe in both multi-crate and single-crate modes.",
    "workspace_name in single-crate mode will be the crate directory name (e.g., 'rust-workspace-map') derived from workspace_root.file_name().",
    "find_crate_root must structurally mirror find_workspace_root: same ancestors() walk, same FileRead error mapping, same .contains() check style, same doc-comment structure with # Errors section.",
    "Orchestration lives in lib.rs:build_map \u2014 workspace.rs remains pure with single-responsibility functions.",
    "All changes stay within the single rust-workspace-map crate. No new crates introduced.",
    "Library code in this phase follows ch12-04 TDD: tests claim interfaces first via tdd_interface, then implementations evolve to meet them. See the tdd-pattern.md reference for the full workflow."
  ],
  "known_pitfalls": [
    "lib.rs line 18-21 does NOT import schema::Error \u2014 must add Error to the use schema::{...} import for the match pattern in TASK-pipeline-fallback.",
    "relativize_path silently returns original path on strip_prefix failure \u2014 no error propagation, works correctly because crate_info.root paths are always workspace-relative.",
    "workspace_name is derived from workspace_root.file_name().unwrap_or_default(). In single-crate mode this is the crate directory name, which is reasonable.",
    "D1 enhancement nests a second match inside the Ok(root) arm: match enumerate_members, catch MissingWorkspaceSection as fallback trigger alongside the outer WorkspaceRootNotFound.",
    "MissingWorkspaceSection false positive: Cargo.toml with '[workspace]' in a comment or string literal triggers find_workspace_root to return Ok, but enumerate_members fails. The D1 enhancement catches this.",
    "TASK-test-single-crate-module-tree MUST exercise depth >= 2 submodule resolution (e.g., lib -> helpers/mod.rs -> helpers/sub.rs).",
    "Do NOT touch these modules: cargo_info.rs, file_parser.rs, module_tree.rs, cross_refs.rs, render.rs, validate.rs, indexes.rs, lookup.rs.",
    "No new CLI flags \u2014 the feature is auto-detect only. Do not add --single-crate or similar flags.",
    "Both find_workspace_root and find_crate_root use .contains() (string match) rather than toml parsing. This is consistent with existing code. The D1 MissingWorkspaceSection fallback mitigates the false-positive risk.",
    "unit tests in workspace.rs use the existing write_cargo_toml and setup_crate helpers (one-param version). Integration tests use the separate setup_crate(dir, lib_content) helper (two-param version)."
  ],
  "task_groups": [
    {
      "group_id": "group-silent-exit",
      "reason": "Self-contained fix for silent error exits in main.rs \u2014 no dependencies on other groups. Can run in parallel with group-fallback once group-core completes.",
      "tasks": [
        "TASK-silent-exit-fix"
      ],
      "depends_on_groups": []
    }
  ],
  "tasks": [
    {
      "id": "TASK-silent-exit-fix",
      "kind": "direct",
      "description": "Fix 4 silent error exit sites in main.rs to print error messages to stderr before exiting.",
      "files_in_scope": [
        "src/main.rs"
      ],
      "changes": [
        {
          "path": "src/main.rs",
          "action": "modify",
          "guidance": "Fix four silent error exit sites in main.rs where errors are discarded without printing to stderr:\n\n**(1) File write error (lines 88-90)**: Change:\n```rust\nif rust_workspace_map::render::render_to_writer(&map, writer).is_err() {\n    std::process::exit(1);\n}\n```\nTo:\n```rust\nif let Err(e) = rust_workspace_map::render::render_to_writer(&map, writer) {\n    eprintln!(\"error writing JSON to file: {e}\");\n    std::process::exit(1);\n}\n```\nUse plain `{e}` for simple I/O errors.\n\n**(2) Stdout write error (lines 93-95)**: Same pattern, change to `if let Err(e)` with `eprintln!(\"error writing JSON to stdout: {e}\")`.\n\n**(3) Pipeline error (lines 102-104)**: Change `Err(_)` to `Err(e)` and add `eprintln!(\"error: {e:#}\")` before `exit(1)`. Use `{e:#}` (alternate format) for anyhow error chains \u2014 this produces multi-line output with context.\n\n**(4) Lookup pipeline error (lines 165-167)**: Same pattern as (3) \u2014 `Err(e) => { eprintln!(\"error: {e:#}\"); std::process::exit(1); }`.\n\n**Do NOT change**:\n- Validation exit (lines 98-100): exit code 2 is intentional for validation findings; stderr is intentionally silent.\n- Lookup serialization errors (lines 123-128, 140-145): these already print messages to stderr before exiting."
        }
      ],
      "wiring_checklist": [],
      "acceptance": [
        "cargo check --workspace",
        "cargo clippy -- -D warnings"
      ],
      "depends_on": []
    }
  ]
}
## File: notes/directions/single-crate-support/directions-single-crate-support-group-tests.json
{
  "meta": {
    "title": "Single-Crate Support + Fix Silent Error Handling",
    "source_branch": "single-crate-support"
  },
  "architecture_notes": [
    "Auto-detect: try workspace discovery first; on WorkspaceRootNotFound or MissingWorkspaceSection, fall back to single-crate discovery. No new CLI flags.",
    "Two separate functions: find_workspace_root (looks for [workspace]) and find_crate_root (looks for [package]). A combined function would stop at a workspace member's Cargo.toml.",
    "D1 enhancement: catch MissingWorkspaceSection from enumerate_members as a fallback trigger, not just WorkspaceRootNotFound. This handles the false positive where Cargo.toml contains '[workspace]' as a comment or string literal.",
    "validate::validate path safety confirmed \u2014 check_orphan_files joins workspace_root with crate_info.root which is always workspace-relative; safe in both multi-crate and single-crate modes.",
    "workspace_name in single-crate mode will be the crate directory name (e.g., 'rust-workspace-map') derived from workspace_root.file_name().",
    "find_crate_root must structurally mirror find_workspace_root: same ancestors() walk, same FileRead error mapping, same .contains() check style, same doc-comment structure with # Errors section.",
    "Orchestration lives in lib.rs:build_map \u2014 workspace.rs remains pure with single-responsibility functions.",
    "All changes stay within the single rust-workspace-map crate. No new crates introduced.",
    "Library code in this phase follows ch12-04 TDD: tests claim interfaces first via tdd_interface, then implementations evolve to meet them. See the tdd-pattern.md reference for the full workflow."
  ],
  "known_pitfalls": [
    "lib.rs line 18-21 does NOT import schema::Error \u2014 must add Error to the use schema::{...} import for the match pattern in TASK-pipeline-fallback.",
    "relativize_path silently returns original path on strip_prefix failure \u2014 no error propagation, works correctly because crate_info.root paths are always workspace-relative.",
    "workspace_name is derived from workspace_root.file_name().unwrap_or_default(). In single-crate mode this is the crate directory name, which is reasonable.",
    "D1 enhancement nests a second match inside the Ok(root) arm: match enumerate_members, catch MissingWorkspaceSection as fallback trigger alongside the outer WorkspaceRootNotFound.",
    "MissingWorkspaceSection false positive: Cargo.toml with '[workspace]' in a comment or string literal triggers find_workspace_root to return Ok, but enumerate_members fails. The D1 enhancement catches this.",
    "TASK-test-single-crate-module-tree MUST exercise depth >= 2 submodule resolution (e.g., lib -> helpers/mod.rs -> helpers/sub.rs).",
    "Do NOT touch these modules: cargo_info.rs, file_parser.rs, module_tree.rs, cross_refs.rs, render.rs, validate.rs, indexes.rs, lookup.rs.",
    "No new CLI flags \u2014 the feature is auto-detect only. Do not add --single-crate or similar flags.",
    "Both find_workspace_root and find_crate_root use .contains() (string match) rather than toml parsing. This is consistent with existing code. The D1 MissingWorkspaceSection fallback mitigates the false-positive risk.",
    "unit tests in workspace.rs use the existing write_cargo_toml and setup_crate helpers (one-param version). Integration tests use the separate setup_crate(dir, lib_content) helper (two-param version)."
  ],
  "task_groups": [
    {
      "group_id": "group-tests",
      "reason": "Both are integration test changes that verify the feature. They depend on all production code changes being in place. Can run in parallel with each other.",
      "tasks": [
        "TASK-test-existing-update",
        "TASK-test-single-crate-module-tree"
      ],
      "depends_on_groups": [
        "group-fallback"
      ]
    }
  ],
  "tasks": [
    {
      "id": "TASK-test-existing-update",
      "kind": "direct",
      "description": "Rename and invert test_missing_workspace_section to test_single_crate_without_workspace \u2014 the existing test expects non-zero exit for a standalone crate, which is now a supported scenario.",
      "files_in_scope": [
        "tests/integration_test.rs"
      ],
      "changes": [
        {
          "path": "tests/integration_test.rs",
          "action": "modify",
          "guidance": "Rename `test_missing_workspace_section` (line 200) to `test_single_crate_without_workspace` and invert the test logic to match the new single-crate support behavior.\n\nReplace the test body (lines 199-223) with:\n\n1. Create a temp directory using `tempfile::tempdir().unwrap()`. Create a subdirectory `let crate_dir = root.join(\"standalone\");`.\n2. Use `setup_crate(&crate_dir, \"pub struct Standalone { pub x: i32 }\")` \u2014 the existing two-param helper writes both a [package]-only Cargo.toml (with name=\"standalone\") and src/lib.rs for that subdirectory.\n3. Run the index command via `run_index(crate_dir.to_str().unwrap())`.\n4. Assert exit 0: `assert!(output.status.success(), ...)`.\n5. Parse the JSON: `let json = parse_output(&output)`.\n6. Extract crates: `let crates = extract_array(&json, \"crates\")`.\n7. Assert exactly one crate (len == 1).\n8. Assert the crate name is \"standalone\": `crates[0][\"name\"] == \"standalone\"`.\n9. Assert the crate has at least one module and that \"Standalone\" appears in public items \u2014 iterate over modules' publicItems and find a struct named \"Standalone\".\n\nThe test name change from `test_missing_workspace_section` to `test_single_crate_without_workspace` signals the behavior change: what was an error condition is now a supported scenario."
        }
      ],
      "wiring_checklist": [],
      "acceptance": [
        "cargo test --test integration_test test_single_crate_without_workspace"
      ],
      "depends_on": [
        "TASK-pipeline-fallback"
      ]
    },
    {
      "id": "TASK-test-single-crate-module-tree",
      "kind": "direct",
      "description": "Add test_single_crate_module_tree integration test with depth >= 2 submodule resolution in single-crate mode.",
      "files_in_scope": [
        "tests/integration_test.rs"
      ],
      "changes": [
        {
          "path": "tests/integration_test.rs",
          "action": "modify",
          "guidance": "Add a new integration test `test_single_crate_module_tree` that verifies module tree construction works in single-crate mode at depth >= 2.\n\nImplementation:\n\n1. Create a temp directory and a subdirectory with a known name: `let tmp = tempfile::tempdir().unwrap(); let root = tmp.path().join(\"single-crate\"); std::fs::create_dir_all(&root).unwrap();`\n2. Use `setup_crate(&root, \"pub mod helpers;\")` to create a standalone (non-workspace) crate with lib.rs declaring a submodule.\n3. Create `src/helpers/mod.rs` with `pub mod sub;` (a module at depth 1 declaring depth 2).\n4. Create `src/helpers/sub.rs` with `pub fn assist() {}` \u2014 this is the depth-2 module with a public item.\n5. Run the index command: `let output = run_index(root.to_str().unwrap());`\n6. Assert exit 0: `assert!(output.status.success(), ...)`\n7. Parse JSON: `let json = parse_output(&output);`\n8. Extract crates array and assert length 1.\n9. Assert the crate name is `\"single-crate\"` (derived from the dir basename by setup_crate).\n10. Collect module paths from the crate and assert all three expected paths exist:\n    - `\"single-crate\"` (root module)\n    - `\"single-crate::helpers\"` (depth 1)\n    - `\"single-crate::helpers::sub\"` (depth 2)\n11. Find the `helpers::sub` module and verify it has the `assist` function in its publicItems array (kind = \"fn\", name = \"assist\").\n\nUse the existing helpers: `setup_crate(dir, lib_content)`, `run_index(path)`, `parse_output(output)`, `extract_array(val, key)`. Follow the pattern established by `test_deeply_nested_modules` (lines 284-329).\n\nThe maximum depth for this test (2) exercises recursive module resolution in single-crate mode \u2014 the key risk flagged by the architectural review. The existing `test_deeply_nested_modules` goes to depth 4 but uses workspace mode with `members = [\".\"]` \u2014 this new test confirms the same depth works in pure single-crate mode without any `[workspace]` section."
        }
      ],
      "wiring_checklist": [],
      "acceptance": [
        "cargo test --test integration_test test_single_crate_module_tree"
      ],
      "depends_on": [
        "TASK-pipeline-fallback"
      ]
    }
  ]
}
## File: notes/directions/single-crate-support/directions.json
{
  "meta": {
    "title": "Single-Crate Support + Fix Silent Error Handling",
    "source_branch": "single-crate-support"
  },
  "architecture_notes": [
    "Auto-detect: try workspace discovery first; on WorkspaceRootNotFound or MissingWorkspaceSection, fall back to single-crate discovery. No new CLI flags.",
    "Two separate functions: find_workspace_root (looks for [workspace]) and find_crate_root (looks for [package]). A combined function would stop at a workspace member's Cargo.toml.",
    "D1 enhancement: catch MissingWorkspaceSection from enumerate_members as a fallback trigger, not just WorkspaceRootNotFound. This handles the false positive where Cargo.toml contains '[workspace]' as a comment or string literal.",
    "validate::validate path safety confirmed — check_orphan_files joins workspace_root with crate_info.root which is always workspace-relative; safe in both multi-crate and single-crate modes.",
    "workspace_name in single-crate mode will be the crate directory name (e.g., 'rust-workspace-map') derived from workspace_root.file_name().",
    "find_crate_root must structurally mirror find_workspace_root: same ancestors() walk, same FileRead error mapping, same .contains() check style, same doc-comment structure with # Errors section.",
    "Orchestration lives in lib.rs:build_map — workspace.rs remains pure with single-responsibility functions.",
    "All changes stay within the single rust-workspace-map crate. No new crates introduced.",
    "Library code in this phase follows ch12-04 TDD: tests claim interfaces first via tdd_interface, then implementations evolve to meet them. See the tdd-pattern.md reference for the full workflow."

  ],
  "known_pitfalls": [
    "lib.rs line 18-21 does NOT import schema::Error — must add Error to the use schema::{...} import for the match pattern in TASK-pipeline-fallback.",
    "relativize_path silently returns original path on strip_prefix failure — no error propagation, works correctly because crate_info.root paths are always workspace-relative.",
    "workspace_name is derived from workspace_root.file_name().unwrap_or_default(). In single-crate mode this is the crate directory name, which is reasonable.",
    "D1 enhancement nests a second match inside the Ok(root) arm: match enumerate_members, catch MissingWorkspaceSection as fallback trigger alongside the outer WorkspaceRootNotFound.",
    "MissingWorkspaceSection false positive: Cargo.toml with '[workspace]' in a comment or string literal triggers find_workspace_root to return Ok, but enumerate_members fails. The D1 enhancement catches this.",
    "TASK-test-single-crate-module-tree MUST exercise depth >= 2 submodule resolution (e.g., lib -> helpers/mod.rs -> helpers/sub.rs).",
    "Do NOT touch these modules: cargo_info.rs, file_parser.rs, module_tree.rs, cross_refs.rs, render.rs, validate.rs, indexes.rs, lookup.rs.",
    "No new CLI flags — the feature is auto-detect only. Do not add --single-crate or similar flags.",
    "Both find_workspace_root and find_crate_root use .contains() (string match) rather than toml parsing. This is consistent with existing code. The D1 MissingWorkspaceSection fallback mitigates the false-positive risk.",
    "unit tests in workspace.rs use the existing write_cargo_toml and setup_crate helpers (one-param version). Integration tests use the separate setup_crate(dir, lib_content) helper (two-param version)."
  ],
  "task_groups": [
    {
      "group_id": "group-core",
      "reason": "All tasks modify shared schema.rs/workspace.rs files — the core discovery enhancement. TASK-single-crate-core-01 is a prerequisite for TASK-single-crate-core-02 (the error variant must exist before the function compiles).",
      "tasks": ["TASK-single-crate-core-01", "TASK-single-crate-core-02"],
      "depends_on_groups": []
    },
    {
      "group_id": "group-fallback",
      "reason": "Pipeline fallback logic depends on group-core (needs find_crate_root and CrateRootNotFound).",
      "tasks": ["TASK-pipeline-fallback"],
      "depends_on_groups": ["group-core"]
    },
    {
      "group_id": "group-silent-exit",
      "reason": "Self-contained fix for silent error exits in main.rs — no dependencies on other groups. Can run in parallel with group-fallback once group-core completes.",
      "tasks": ["TASK-silent-exit-fix"],
      "depends_on_groups": []
    },
    {
      "group_id": "group-tests",
      "reason": "Both are integration test changes that verify the feature. They depend on all production code changes being in place. Can run in parallel with each other.",
      "tasks": ["TASK-test-existing-update", "TASK-test-single-crate-module-tree"],
      "depends_on_groups": ["group-fallback"]
    }
  ],
  "tasks": [
    {
      "id": "TASK-single-crate-core-01",
      "kind": "direct",
      "description": "Add CrateRootNotFound variant to schema::Error enum so find_crate_root can signal discovery failure.",
      "files_in_scope": [
        "src/schema.rs"
      ],
      "changes": [
        {
          "path": "src/schema.rs",
          "action": "modify",
          "guidance": "Add a new error variant to the Error enum. Insert it after the WorkspaceRootNotFound variant (line 55), before the FileRead variant (line 57):\n\n```rust\n#[error(\"no Cargo.toml with [package] section found starting from {0}\")]\nCrateRootNotFound(PathBuf),\n```\n\nThis variant carries a PathBuf (the start_path that was searched), matching the pattern of WorkspaceRootNotFound. The thiserror attribute format is consistent with other variants. The variant is automatically reachable — schema::Error is already imported by workspace.rs via `use crate::schema::{CrateType, Error, Result}`."
        }
      ],
      "wiring_checklist": [],
      "type_reference": {
        "CrateRootNotFound": "#[error(\"no Cargo.toml with [package] section found starting from {0}\")] CrateRootNotFound(PathBuf)"
      },
      "acceptance": [
        "cargo check --workspace"
      ],
      "depends_on": []
    },
    {
      "id": "TASK-single-crate-core-02",
      "kind": "lib-tdd",
      "description": "Add find_crate_root discovery function + 2 unit tests via test-driven development.",
      "files_in_scope": [
        "src/workspace.rs"
      ],
      "changes": [
        {
          "path": "src/workspace.rs",
          "action": "modify",
          "guidance": "Implement find_crate_root following the TDD cycle. The test (tdd_interface.test_code) defines the contract — write it first, confirm failure, then implement.\n\nImplementation approach: Walk up the directory tree using start_path.ancestors(). For each ancestor, check if Cargo.toml exists. If it does, read the file contents. Map I/O errors to Error::FileRead { path, source }. Check if the content contains the string \"[package]\". If found, return Ok(ancestor.to_path_buf()). If the loop completes without finding a [package] section, return Err(Error::CrateRootNotFound(start_path.to_path_buf())).\n\nPlacement: Insert after find_workspace_root (after line 25), before enumerate_members (line 35). Use the same doc-comment structure as find_workspace_root: /// description, /// # Errors section. The function must be pub.\n\nFor the unit tests: Add both test functions inside the existing #[cfg(test)] mod tests block, before its closing } at line 206. The test module already has write_cargo_toml and setup_crate (one-param) helpers via `use super::*`. No new imports needed.\n\nEdge cases handled: empty directory tree (ancestors() always includes start_path itself), Cargo.toml without [package] (continues walking), I/O error reading Cargo.toml (returns FileRead error)."
        }
      ],
      "tdd_interface": {
        "test_file": "src/workspace.rs",
        "test_module": "tests",
        "test_fn_name": "find_crate_root_finds_package_section",
        "test_code": "#[test]\nfn find_crate_root_finds_package_section() {\n    let tmp = tempfile::tempdir().unwrap();\n    setup_crate(tmp.path());\n    // Walk from a nested subdirectory inside src\n    let nested = tmp.path().join(\"src\").join(\"subdir\");\n    std::fs::create_dir_all(&nested).unwrap();\n    let result = find_crate_root(&nested).unwrap();\n    assert_eq!(result, tmp.path());\n}\n\n#[test]\nfn find_crate_root_returns_err_for_no_package() {\n    let tmp = tempfile::tempdir().unwrap();\n    // No Cargo.toml at all — ancestors() walks up and finds nothing\n    let result = find_crate_root(tmp.path());\n    assert!(result.is_err());\n    match result.unwrap_err() {\n        Error::CrateRootNotFound(_) => {},\n        other => panic!(\"expected CrateRootNotFound, got {:?}\", other),\n    }\n}",
        "signature": "pub fn find_crate_root(start_path: &Path) -> Result<PathBuf>",
        "expected_behavior": "find_crate_root walks up from start_path through ancestors, returns the first directory whose Cargo.toml contains a [package] section. Returns CrateRootNotFound error if no Cargo.toml with [package] is found in any ancestor (including start_path itself). Returns FileRead error if a Cargo.toml exists but cannot be read."
      },
      "wiring_checklist": [],
      "type_reference": {
        "find_crate_root": "pub fn find_crate_root(start_path: &Path) -> Result<PathBuf>"
      },
      "acceptance": [
        "cargo test find_crate_root_finds_package_section",
        "cargo test find_crate_root_returns_err_for_no_package",
        "cargo check --workspace"
      ],
      "depends_on": ["TASK-single-crate-core-01"]
    },
    {
      "id": "TASK-pipeline-fallback",
      "kind": "direct",
      "description": "Modify build_map in lib.rs to add fallback logic: try workspace discovery first, fall back to single-crate on WorkspaceRootNotFound or MissingWorkspaceSection.",
      "files_in_scope": [
        "src/lib.rs"
      ],
      "changes": [
        {
          "path": "src/lib.rs",
          "action": "modify",
          "guidance": "Two changes to src/lib.rs:\n\n**(1) Add Error import**: On line 18-21, the `use schema::{...}` block currently imports CrateInfo, CrateType, DiagnosticKind, ErrorEntry, ErrorSeverity, ModuleInfo, WorkspaceInfo, WorkspaceMap. Add `Error` to this import. The block should include `Error` alongside the existing types.\n\n**(2) Replace workspace discovery lines (37-38) with fallback match**: Replace the two lines:\n```rust\nlet workspace_root = workspace::find_workspace_root(&config.workspace_path)?;\nlet member_dirs = workspace::enumerate_members(&workspace_root)?;\n```\n\nWith a match expression that tries workspace discovery first, then falls back. The structure:\n```rust\nlet (workspace_root, member_dirs) = match workspace::find_workspace_root(&config.workspace_path) {\n    Ok(root) => {\n        match workspace::enumerate_members(&root) {\n            Ok(members) => (root, members),\n            Err(Error::MissingWorkspaceSection) => {\n                // [workspace] was a false positive (e.g., in a comment).\n                // Fall back to single-crate discovery.\n                let crate_dir = workspace::find_crate_root(&config.workspace_path)?;\n                (crate_dir.clone(), vec![crate_dir])\n            }\n            Err(other) => return Err(other.into()),\n        }\n    }\n    Err(Error::WorkspaceRootNotFound(_)) => {\n        let crate_dir = workspace::find_crate_root(&config.workspace_path)?;\n        (crate_dir.clone(), vec![crate_dir])\n    }\n    Err(other) => return Err(other.into()),\n};\n```\n\nKey points:\n- The inner match on enumerate_members handles the D1 false-positive case where Cargo.toml contains \"[workspace]\" as a literal string but no actual workspace section.\n- The fallback path is identical for both MissingWorkspaceSection and WorkspaceRootNotFound: find_crate_root then create a single-element vec.\n- Other errors (TomlParse, FileRead, etc.) propagate as fatal.\n- workspace_root is set to crate_dir in fallback mode; member_dirs is vec![crate_dir].\n- The rest of build_map (lines 40-171) works unchanged — it uses workspace_root and member_dirs identically regardless of discovery path.\n\nThe function signature and return type do NOT change."
        }
      ],
      "wiring_checklist": [
        {
          "kind": "fn_call",
          "file": "src/lib.rs",
          "detail": "build_map calls workspace::find_crate_root in two fallback branches (MissingWorkspaceSection and WorkspaceRootNotFound)"
        },
        {
          "kind": "type_annotation",
          "file": "src/lib.rs",
          "detail": "schema::Error is imported for the match patterns on Error::WorkspaceRootNotFound and Error::MissingWorkspaceSection"
        }
      ],
      "acceptance": [
        "cargo check --workspace",
        "cargo clippy -- -D warnings"
      ],
      "depends_on": ["TASK-single-crate-core-02"]
    },
    {
      "id": "TASK-silent-exit-fix",
      "kind": "direct",
      "description": "Fix 4 silent error exit sites in main.rs to print error messages to stderr before exiting.",
      "files_in_scope": [
        "src/main.rs"
      ],
      "changes": [
        {
          "path": "src/main.rs",
          "action": "modify",
          "guidance": "Fix four silent error exit sites in main.rs where errors are discarded without printing to stderr:\n\n**(1) File write error (lines 88-90)**: Change:\n```rust\nif rust_workspace_map::render::render_to_writer(&map, writer).is_err() {\n    std::process::exit(1);\n}\n```\nTo:\n```rust\nif let Err(e) = rust_workspace_map::render::render_to_writer(&map, writer) {\n    eprintln!(\"error writing JSON to file: {e}\");\n    std::process::exit(1);\n}\n```\nUse plain `{e}` for simple I/O errors.\n\n**(2) Stdout write error (lines 93-95)**: Same pattern, change to `if let Err(e)` with `eprintln!(\"error writing JSON to stdout: {e}\")`.\n\n**(3) Pipeline error (lines 102-104)**: Change `Err(_)` to `Err(e)` and add `eprintln!(\"error: {e:#}\")` before `exit(1)`. Use `{e:#}` (alternate format) for anyhow error chains — this produces multi-line output with context.\n\n**(4) Lookup pipeline error (lines 165-167)**: Same pattern as (3) — `Err(e) => { eprintln!(\"error: {e:#}\"); std::process::exit(1); }`.\n\n**Do NOT change**:\n- Validation exit (lines 98-100): exit code 2 is intentional for validation findings; stderr is intentionally silent.\n- Lookup serialization errors (lines 123-128, 140-145): these already print messages to stderr before exiting."
        }
      ],
      "wiring_checklist": [],
      "acceptance": [
        "cargo check --workspace",
        "cargo clippy -- -D warnings"
      ],
      "depends_on": []
    },
    {
      "id": "TASK-test-existing-update",
      "kind": "direct",
      "description": "Rename and invert test_missing_workspace_section to test_single_crate_without_workspace — the existing test expects non-zero exit for a standalone crate, which is now a supported scenario.",
      "files_in_scope": [
        "tests/integration_test.rs"
      ],
      "changes": [
        {
          "path": "tests/integration_test.rs",
          "action": "modify",
          "guidance": "Rename `test_missing_workspace_section` (line 200) to `test_single_crate_without_workspace` and invert the test logic to match the new single-crate support behavior.\n\nReplace the test body (lines 199-223) with:\n\n1. Create a temp directory using `tempfile::tempdir().unwrap()`. Create a subdirectory `let crate_dir = root.join(\"standalone\");`.\n2. Use `setup_crate(&crate_dir, \"pub struct Standalone { pub x: i32 }\")` — the existing two-param helper writes both a [package]-only Cargo.toml (with name=\"standalone\") and src/lib.rs for that subdirectory.\n3. Run the index command via `run_index(crate_dir.to_str().unwrap())`.\n4. Assert exit 0: `assert!(output.status.success(), ...)`.\n5. Parse the JSON: `let json = parse_output(&output)`.\n6. Extract crates: `let crates = extract_array(&json, \"crates\")`.\n7. Assert exactly one crate (len == 1).\n8. Assert the crate name is \"standalone\": `crates[0][\"name\"] == \"standalone\"`.\n9. Assert the crate has at least one module and that \"Standalone\" appears in public items — iterate over modules' publicItems and find a struct named \"Standalone\".\n\nThe test name change from `test_missing_workspace_section` to `test_single_crate_without_workspace` signals the behavior change: what was an error condition is now a supported scenario."
        }
      ],
      "wiring_checklist": [],
      "acceptance": [
        "cargo test --test integration_test test_single_crate_without_workspace"
      ],
      "depends_on": ["TASK-pipeline-fallback"]
    },
    {
      "id": "TASK-test-single-crate-module-tree",
      "kind": "direct",
      "description": "Add test_single_crate_module_tree integration test with depth >= 2 submodule resolution in single-crate mode.",
      "files_in_scope": [
        "tests/integration_test.rs"
      ],
      "changes": [
        {
          "path": "tests/integration_test.rs",
          "action": "modify",
          "guidance": "Add a new integration test `test_single_crate_module_tree` that verifies module tree construction works in single-crate mode at depth >= 2.\n\nImplementation:\n\n1. Create a temp directory and a subdirectory with a known name: `let tmp = tempfile::tempdir().unwrap(); let root = tmp.path().join(\"single-crate\"); std::fs::create_dir_all(&root).unwrap();`\n2. Use `setup_crate(&root, \"pub mod helpers;\")` to create a standalone (non-workspace) crate with lib.rs declaring a submodule.\n3. Create `src/helpers/mod.rs` with `pub mod sub;` (a module at depth 1 declaring depth 2).\n4. Create `src/helpers/sub.rs` with `pub fn assist() {}` — this is the depth-2 module with a public item.\n5. Run the index command: `let output = run_index(root.to_str().unwrap());`\n6. Assert exit 0: `assert!(output.status.success(), ...)`\n7. Parse JSON: `let json = parse_output(&output);`\n8. Extract crates array and assert length 1.\n9. Assert the crate name is `\"single-crate\"` (derived from the dir basename by setup_crate).\n10. Collect module paths from the crate and assert all three expected paths exist:\n    - `\"single-crate\"` (root module)\n    - `\"single-crate::helpers\"` (depth 1)\n    - `\"single-crate::helpers::sub\"` (depth 2)\n11. Find the `helpers::sub` module and verify it has the `assist` function in its publicItems array (kind = \"fn\", name = \"assist\").\n\nUse the existing helpers: `setup_crate(dir, lib_content)`, `run_index(path)`, `parse_output(output)`, `extract_array(val, key)`. Follow the pattern established by `test_deeply_nested_modules` (lines 284-329).\n\nThe maximum depth for this test (2) exercises recursive module resolution in single-crate mode — the key risk flagged by the architectural review. The existing `test_deeply_nested_modules` goes to depth 4 but uses workspace mode with `members = [\".\"]` — this new test confirms the same depth works in pure single-crate mode without any `[workspace]` section."
        }
      ],
      "wiring_checklist": [],
      "acceptance": [
        "cargo test --test integration_test test_single_crate_module_tree"
      ],
      "depends_on": ["TASK-pipeline-fallback"]
    }
  ]
}
## File: notes/directions/single-crate-support/draft-directions.json
{
  "meta": {
    "title": "Single-Crate Support + Fix Silent Error Handling",
    "source_branch": "single-crate-support"
  },
  "architecture_notes": [
    "Auto-detect: try workspace discovery first; on WorkspaceRootNotFound or MissingWorkspaceSection, fall back to single-crate discovery. No new CLI flags.",
    "Two separate functions: find_workspace_root (looks for [workspace]) and find_crate_root (looks for [package]). A combined function would stop at a workspace member's Cargo.toml.",
    "D1 enhancement: catch MissingWorkspaceSection from enumerate_members as a fallback trigger, not just WorkspaceRootNotFound. This handles the false positive where Cargo.toml contains '[workspace]' as a comment or string literal.",
    "validate::validate path safety confirmed — check_orphan_files joins workspace_root with crate_info.root which is always workspace-relative; safe in both multi-crate and single-crate modes.",
    "workspace_name in single-crate mode will be the crate directory name (e.g., 'rust-workspace-map') derived from workspace_root.file_name().",
    "find_crate_root must structurally mirror find_workspace_root: same ancestors() walk, same FileRead error mapping, same .contains() check style, same doc-comment structure with # Errors section.",
    "Orchestration lives in lib.rs:build_map — workspace.rs remains pure with single-responsibility functions.",
    "All changes stay within the single rust-workspace-map crate. No new crates introduced.",
    "Library code in this phase follows ch12-04 TDD: tests claim interfaces first via tdd_interface, then implementations evolve to meet them. See the tdd-pattern.md reference for the full workflow."

  ],
  "known_pitfalls": [
    "lib.rs line 18-21 does NOT import schema::Error — must add Error to the use schema::{...} import for the match pattern in TASK-pipeline-fallback.",
    "relativize_path silently returns original path on strip_prefix failure — no error propagation, works correctly because crate_info.root paths are always workspace-relative.",
    "workspace_name is derived from workspace_root.file_name().unwrap_or_default(). In single-crate mode this is the crate directory name, which is reasonable.",
    "D1 enhancement nests a second match inside the Ok(root) arm: match enumerate_members, catch MissingWorkspaceSection as fallback trigger alongside the outer WorkspaceRootNotFound.",
    "MissingWorkspaceSection false positive: Cargo.toml with '[workspace]' in a comment or string literal triggers find_workspace_root to return Ok, but enumerate_members fails. The D1 enhancement catches this.",
    "TASK-test-single-crate-module-tree MUST exercise depth >= 2 submodule resolution (e.g., lib -> helpers/mod.rs -> helpers/sub.rs).",
    "Do NOT touch these modules: cargo_info.rs, file_parser.rs, module_tree.rs, cross_refs.rs, render.rs, validate.rs, indexes.rs, lookup.rs.",
    "No new CLI flags — the feature is auto-detect only. Do not add --single-crate or similar flags.",
    "Both find_workspace_root and find_crate_root use .contains() (string match) rather than toml parsing. This is consistent with existing code. The D1 MissingWorkspaceSection fallback mitigates the false-positive risk.",
    "unit tests in workspace.rs use the existing write_cargo_toml and setup_crate helpers (one-param version). Integration tests use the separate setup_crate(dir, lib_content) helper (two-param version)."
  ],
  "task_groups": [
    {
      "group_id": "group-core",
      "reason": "All tasks modify shared schema.rs/workspace.rs files — the core discovery enhancement. TASK-single-crate-core-01 is a prerequisite for TASK-single-crate-core-02 (the error variant must exist before the function compiles).",
      "tasks": ["TASK-single-crate-core-01", "TASK-single-crate-core-02"],
      "depends_on_groups": []
    },
    {
      "group_id": "group-fallback",
      "reason": "Pipeline fallback logic depends on group-core (needs find_crate_root and CrateRootNotFound).",
      "tasks": ["TASK-pipeline-fallback"],
      "depends_on_groups": ["group-core"]
    },
    {
      "group_id": "group-silent-exit",
      "reason": "Self-contained fix for silent error exits in main.rs — no dependencies on other groups. Can run in parallel with group-fallback once group-core completes.",
      "tasks": ["TASK-silent-exit-fix"],
      "depends_on_groups": []
    },
    {
      "group_id": "group-tests",
      "reason": "Both are integration test changes that verify the feature. They depend on all production code changes being in place. Can run in parallel with each other.",
      "tasks": ["TASK-test-existing-update", "TASK-test-single-crate-module-tree"],
      "depends_on_groups": ["group-fallback"]
    }
  ],
  "tasks": [
    {
      "id": "TASK-single-crate-core-01",
      "kind": "direct",
      "description": "Add CrateRootNotFound variant to schema::Error enum so find_crate_root can signal discovery failure.",
      "files_in_scope": [
        "src/schema.rs"
      ],
      "changes": [
        {
          "path": "src/schema.rs",
          "action": "modify",
          "guidance": "Add a new error variant to the Error enum. Insert it after the WorkspaceRootNotFound variant (line 55), before the FileRead variant (line 57):\n\n```rust\n#[error(\"no Cargo.toml with [package] section found starting from {0}\")]\nCrateRootNotFound(PathBuf),\n```\n\nThis variant carries a PathBuf (the start_path that was searched), matching the pattern of WorkspaceRootNotFound. The thiserror attribute format is consistent with other variants. The variant is automatically reachable — schema::Error is already imported by workspace.rs via `use crate::schema::{CrateType, Error, Result}`."
        }
      ],
      "wiring_checklist": [],
      "type_reference": {
        "CrateRootNotFound": "#[error(\"no Cargo.toml with [package] section found starting from {0}\")] CrateRootNotFound(PathBuf)"
      },
      "acceptance": [
        "cargo check --workspace"
      ],
      "depends_on": []
    },
    {
      "id": "TASK-single-crate-core-02",
      "kind": "lib-tdd",
      "description": "Add find_crate_root discovery function + 2 unit tests via test-driven development.",
      "files_in_scope": [
        "src/workspace.rs"
      ],
      "changes": [
        {
          "path": "src/workspace.rs",
          "action": "modify",
          "guidance": "Implement find_crate_root following the TDD cycle. The test (tdd_interface.test_code) defines the contract — write it first, confirm failure, then implement.\n\nImplementation approach: Walk up the directory tree using start_path.ancestors(). For each ancestor, check if Cargo.toml exists. If it does, read the file contents. Map I/O errors to Error::FileRead { path, source }. Check if the content contains the string \"[package]\". If found, return Ok(ancestor.to_path_buf()). If the loop completes without finding a [package] section, return Err(Error::CrateRootNotFound(start_path.to_path_buf())).\n\nPlacement: Insert after find_workspace_root (after line 25), before enumerate_members (line 35). Use the same doc-comment structure as find_workspace_root: /// description, /// # Errors section. The function must be pub.\n\nFor the unit tests: Add both test functions inside the existing #[cfg(test)] mod tests block, before its closing } at line 206. The test module already has write_cargo_toml and setup_crate (one-param) helpers via `use super::*`. No new imports needed.\n\nEdge cases handled: empty directory tree (ancestors() always includes start_path itself), Cargo.toml without [package] (continues walking), I/O error reading Cargo.toml (returns FileRead error)."
        }
      ],
      "tdd_interface": {
        "test_file": "src/workspace.rs",
        "test_module": "tests",
        "test_fn_name": "find_crate_root_finds_package_section",
        "test_code": "#[test]\nfn find_crate_root_finds_package_section() {\n    let tmp = tempfile::tempdir().unwrap();\n    setup_crate(tmp.path());\n    // Walk from a nested subdirectory inside src\n    let nested = tmp.path().join(\"src\").join(\"subdir\");\n    std::fs::create_dir_all(&nested).unwrap();\n    let result = find_crate_root(&nested).unwrap();\n    assert_eq!(result, tmp.path());\n}\n\n#[test]\nfn find_crate_root_returns_err_for_no_package() {\n    let tmp = tempfile::tempdir().unwrap();\n    // No Cargo.toml at all — ancestors() walks up and finds nothing\n    let result = find_crate_root(tmp.path());\n    assert!(result.is_err());\n    match result.unwrap_err() {\n        Error::CrateRootNotFound(_) => {},\n        other => panic!(\"expected CrateRootNotFound, got {:?}\", other),\n    }\n}",
        "signature": "pub fn find_crate_root(start_path: &Path) -> Result<PathBuf>",
        "expected_behavior": "find_crate_root walks up from start_path through ancestors, returns the first directory whose Cargo.toml contains a [package] section. Returns CrateRootNotFound error if no Cargo.toml with [package] is found in any ancestor (including start_path itself). Returns FileRead error if a Cargo.toml exists but cannot be read."
      },
      "wiring_checklist": [],
      "type_reference": {
        "find_crate_root": "pub fn find_crate_root(start_path: &Path) -> Result<PathBuf>"
      },
      "acceptance": [
        "cargo test find_crate_root_finds_package_section",
        "cargo test find_crate_root_returns_err_for_no_package",
        "cargo check --workspace"
      ],
      "depends_on": ["TASK-single-crate-core-01"]
    },
    {
      "id": "TASK-pipeline-fallback",
      "kind": "direct",
      "description": "Modify build_map in lib.rs to add fallback logic: try workspace discovery first, fall back to single-crate on WorkspaceRootNotFound or MissingWorkspaceSection.",
      "files_in_scope": [
        "src/lib.rs"
      ],
      "changes": [
        {
          "path": "src/lib.rs",
          "action": "modify",
          "guidance": "Two changes to src/lib.rs:\n\n**(1) Add Error import**: On line 18-21, the `use schema::{...}` block currently imports CrateInfo, CrateType, DiagnosticKind, ErrorEntry, ErrorSeverity, ModuleInfo, WorkspaceInfo, WorkspaceMap. Add `Error` to this import. The block should include `Error` alongside the existing types.\n\n**(2) Replace workspace discovery lines (37-38) with fallback match**: Replace the two lines:\n```rust\nlet workspace_root = workspace::find_workspace_root(&config.workspace_path)?;\nlet member_dirs = workspace::enumerate_members(&workspace_root)?;\n```\n\nWith a match expression that tries workspace discovery first, then falls back. The structure:\n```rust\nlet (workspace_root, member_dirs) = match workspace::find_workspace_root(&config.workspace_path) {\n    Ok(root) => {\n        match workspace::enumerate_members(&root) {\n            Ok(members) => (root, members),\n            Err(Error::MissingWorkspaceSection) => {\n                // [workspace] was a false positive (e.g., in a comment).\n                // Fall back to single-crate discovery.\n                let crate_dir = workspace::find_crate_root(&config.workspace_path)?;\n                (crate_dir.clone(), vec![crate_dir])\n            }\n            Err(other) => return Err(other.into()),\n        }\n    }\n    Err(Error::WorkspaceRootNotFound(_)) => {\n        let crate_dir = workspace::find_crate_root(&config.workspace_path)?;\n        (crate_dir.clone(), vec![crate_dir])\n    }\n    Err(other) => return Err(other.into()),\n};\n```\n\nKey points:\n- The inner match on enumerate_members handles the D1 false-positive case where Cargo.toml contains \"[workspace]\" as a literal string but no actual workspace section.\n- The fallback path is identical for both MissingWorkspaceSection and WorkspaceRootNotFound: find_crate_root then create a single-element vec.\n- Other errors (TomlParse, FileRead, etc.) propagate as fatal.\n- workspace_root is set to crate_dir in fallback mode; member_dirs is vec![crate_dir].\n- The rest of build_map (lines 40-171) works unchanged — it uses workspace_root and member_dirs identically regardless of discovery path.\n\nThe function signature and return type do NOT change."
        }
      ],
      "wiring_checklist": [
        {
          "kind": "fn_call",
          "file": "src/lib.rs",
          "detail": "build_map calls workspace::find_crate_root in two fallback branches (MissingWorkspaceSection and WorkspaceRootNotFound)"
        },
        {
          "kind": "type_annotation",
          "file": "src/lib.rs",
          "detail": "schema::Error is imported for the match patterns on Error::WorkspaceRootNotFound and Error::MissingWorkspaceSection"
        }
      ],
      "acceptance": [
        "cargo check --workspace",
        "cargo clippy -- -D warnings"
      ],
      "depends_on": ["TASK-single-crate-core-02"]
    },
    {
      "id": "TASK-silent-exit-fix",
      "kind": "direct",
      "description": "Fix 4 silent error exit sites in main.rs to print error messages to stderr before exiting.",
      "files_in_scope": [
        "src/main.rs"
      ],
      "changes": [
        {
          "path": "src/main.rs",
          "action": "modify",
          "guidance": "Fix four silent error exit sites in main.rs where errors are discarded without printing to stderr:\n\n**(1) File write error (lines 88-90)**: Change:\n```rust\nif rust_workspace_map::render::render_to_writer(&map, writer).is_err() {\n    std::process::exit(1);\n}\n```\nTo:\n```rust\nif let Err(e) = rust_workspace_map::render::render_to_writer(&map, writer) {\n    eprintln!(\"error writing JSON to file: {e}\");\n    std::process::exit(1);\n}\n```\nUse plain `{e}` for simple I/O errors.\n\n**(2) Stdout write error (lines 93-95)**: Same pattern, change to `if let Err(e)` with `eprintln!(\"error writing JSON to stdout: {e}\")`.\n\n**(3) Pipeline error (lines 102-104)**: Change `Err(_)` to `Err(e)` and add `eprintln!(\"error: {e:#}\")` before `exit(1)`. Use `{e:#}` (alternate format) for anyhow error chains — this produces multi-line output with context.\n\n**(4) Lookup pipeline error (lines 165-167)**: Same pattern as (3) — `Err(e) => { eprintln!(\"error: {e:#}\"); std::process::exit(1); }`.\n\n**Do NOT change**:\n- Validation exit (lines 98-100): exit code 2 is intentional for validation findings; stderr is intentionally silent.\n- Lookup serialization errors (lines 123-128, 140-145): these already print messages to stderr before exiting."
        }
      ],
      "wiring_checklist": [],
      "acceptance": [
        "cargo check --workspace",
        "cargo clippy -- -D warnings"
      ],
      "depends_on": []
    },
    {
      "id": "TASK-test-existing-update",
      "kind": "direct",
      "description": "Rename and invert test_missing_workspace_section to test_single_crate_without_workspace — the existing test expects non-zero exit for a standalone crate, which is now a supported scenario.",
      "files_in_scope": [
        "tests/integration_test.rs"
      ],
      "changes": [
        {
          "path": "tests/integration_test.rs",
          "action": "modify",
          "guidance": "Rename `test_missing_workspace_section` (line 200) to `test_single_crate_without_workspace` and invert the test logic to match the new single-crate support behavior.\n\nReplace the test body (lines 199-223) with:\n\n1. Create a temp directory using `tempfile::tempdir().unwrap()`. Create a subdirectory `let crate_dir = root.join(\"standalone\");`.\n2. Use `setup_crate(&crate_dir, \"pub struct Standalone { pub x: i32 }\")` — the existing two-param helper writes both a [package]-only Cargo.toml (with name=\"standalone\") and src/lib.rs for that subdirectory.\n3. Run the index command via `run_index(crate_dir.to_str().unwrap())`.\n4. Assert exit 0: `assert!(output.status.success(), ...)`.\n5. Parse the JSON: `let json = parse_output(&output)`.\n6. Extract crates: `let crates = extract_array(&json, \"crates\")`.\n7. Assert exactly one crate (len == 1).\n8. Assert the crate name is \"standalone\": `crates[0][\"name\"] == \"standalone\"`.\n9. Assert the crate has at least one module and that \"Standalone\" appears in public items — iterate over modules' publicItems and find a struct named \"Standalone\".\n\nThe test name change from `test_missing_workspace_section` to `test_single_crate_without_workspace` signals the behavior change: what was an error condition is now a supported scenario."
        }
      ],
      "wiring_checklist": [],
      "acceptance": [
        "cargo test --test integration_test test_single_crate_without_workspace"
      ],
      "depends_on": ["TASK-pipeline-fallback"]
    },
    {
      "id": "TASK-test-single-crate-module-tree",
      "kind": "direct",
      "description": "Add test_single_crate_module_tree integration test with depth >= 2 submodule resolution in single-crate mode.",
      "files_in_scope": [
        "tests/integration_test.rs"
      ],
      "changes": [
        {
          "path": "tests/integration_test.rs",
          "action": "modify",
          "guidance": "Add a new integration test `test_single_crate_module_tree` that verifies module tree construction works in single-crate mode at depth >= 2.\n\nImplementation:\n\n1. Create a temp directory and a subdirectory with a known name: `let tmp = tempfile::tempdir().unwrap(); let root = tmp.path().join(\"single-crate\"); std::fs::create_dir_all(&root).unwrap();`\n2. Use `setup_crate(&root, \"pub mod helpers;\")` to create a standalone (non-workspace) crate with lib.rs declaring a submodule.\n3. Create `src/helpers/mod.rs` with `pub mod sub;` (a module at depth 1 declaring depth 2).\n4. Create `src/helpers/sub.rs` with `pub fn assist() {}` — this is the depth-2 module with a public item.\n5. Run the index command: `let output = run_index(root.to_str().unwrap());`\n6. Assert exit 0: `assert!(output.status.success(), ...)`\n7. Parse JSON: `let json = parse_output(&output);`\n8. Extract crates array and assert length 1.\n9. Assert the crate name is `\"single-crate\"` (derived from the dir basename by setup_crate).\n10. Collect module paths from the crate and assert all three expected paths exist:\n    - `\"single-crate\"` (root module)\n    - `\"single-crate::helpers\"` (depth 1)\n    - `\"single-crate::helpers::sub\"` (depth 2)\n11. Find the `helpers::sub` module and verify it has the `assist` function in its publicItems array (kind = \"fn\", name = \"assist\").\n\nUse the existing helpers: `setup_crate(dir, lib_content)`, `run_index(path)`, `parse_output(output)`, `extract_array(val, key)`. Follow the pattern established by `test_deeply_nested_modules` (lines 284-329).\n\nThe maximum depth for this test (2) exercises recursive module resolution in single-crate mode — the key risk flagged by the architectural review. The existing `test_deeply_nested_modules` goes to depth 4 but uses workspace mode with `members = [\".\"]` — this new test confirms the same depth works in pure single-crate mode without any `[workspace]` section."
        }
      ],
      "wiring_checklist": [],
      "acceptance": [
        "cargo test --test integration_test test_single_crate_module_tree"
      ],
      "depends_on": ["TASK-pipeline-fallback"]
    }
  ]
}
## File: notes/directions/single-crate-support/draft-elaboration.md
# Draft Elaboration -- Single-Crate Support Phase

> Generated: 2026-05-06 | Phase: single-crate-support
> Plan reviewed: commit `5961d63`
> Inputs: `plans/single-crate-support/PLAN.md`, `notes/directions/single-crate-support/deferred-and-patterns.md`, `notes/directions/single-crate-support/workspace-map.json`, `notes/architecture-current.md`

---

## 0. Architecture Review Summary

The plan is architecturally sound. The auto-detect design (no new CLI flags), two separate discovery functions, and orchestration in `build_map` all follow existing patterns correctly. Two adjustments are recommended:

1. **D1 (incorporate now)**: Extend the `build_map` error match to also catch `MissingWorkspaceSection` from `enumerate_members` as a fallback trigger.
2. **validate.rs path safety confirmed**: The plan's Implementation Note concern is resolved -- `check_orphan_files` is safe in single-crate mode.

No structural changes to the plan are needed. The 7-step breakdown is correct; task groupings are described in Section 6.

---

## 1. Design Decisions Per Goal

### Goal A: Single-crate auto-detection (Steps 1-4)

**Decision A1 -- Auto-detect, no new CLI flag**

The plan correctly chooses auto-detection over a `--single-crate` flag. The CLI surface stays clean. The disambiguation rule is: workspace mode takes priority; single-crate is only a fallback when `WorkspaceRootNotFound` is returned.

No change to `Config` (schema.rs:89-100) or `Cli` (main.rs:36-39).

**Decision A2 -- Two separate functions, not one combined**

`find_workspace_root` checks for `[workspace]`; `find_crate_root` checks for `[package]`. A combined function that checks for either would stop at a workspace member's `Cargo.toml` (which has `[package]` but no `[workspace]`), returning the member directory instead of the workspace root. The separate-function design is correct for both multi-crate and single-crate scenarios.

**Decision A3 -- Orchestration lives in `build_map`, not `workspace.rs`**

`workspace.rs` remains pure: each function does one thing. The orchestration (try A, on specific error fall back to B) stays in `lib.rs:build_map`. This preserves single-responsibility.

**Decision A4 -- D1: Catch `MissingWorkspaceSection` as fallback trigger (RECOMMENDED ADDITION)**

The plan's Step 4 match only catches `WorkspaceRootNotFound`:

```rust
Err(Error::WorkspaceRootNotFound(_)) => {
    let crate_dir = workspace::find_crate_root(&config.workspace_path)?;
    ...
}
```

But `find_workspace_root` can produce a false positive: a Cargo.toml containing `[workspace]` in a comment or string literal (e.g., `# This project used to be a [workspace]`). In this case, `find_workspace_root` returns `Ok(root)`, but `enumerate_members` fails with `MissingWorkspaceSection` (the TOML has no actual `[workspace]` section).

**Recommendation**: Extend the match to also catch `MissingWorkspaceSection` from `enumerate_members`:

```rust
let (workspace_root, member_dirs) = match workspace::find_workspace_root(&config.workspace_path) {
    Ok(root) => {
        match workspace::enumerate_members(&root) {
            Ok(members) => (root, members),
            Err(Error::MissingWorkspaceSection) => {
                // [workspace] was a false positive (e.g., in a comment).
                // Fall back to single-crate discovery.
                let crate_dir = workspace::find_crate_root(&config.workspace_path)?;
                (crate_dir.clone(), vec![crate_dir])
            }
            Err(other) => return Err(other.into()),
        }
    }
    Err(Error::WorkspaceRootNotFound(_)) => {
        let crate_dir = workspace::find_crate_root(&config.workspace_path)?;
        (crate_dir.clone(), vec![crate_dir])
    }
    Err(other) => return Err(other.into()),
};
```

This is a small change with minimal risk: the fallback path is identical to the `WorkspaceRootNotFound` case.

### Goal B: Fix silent error exits (Step 5)

**Decision B1 -- Fix all four sites at once**

The plan treats all four silent-exit sites as a single work item. This is correct: fixing only some would leave the user with a partial experience (single-crate works but some failures remain invisible).

**Decision B2 -- Use `{e:#}` for pipeline/render errors**

The plan uses `{e:#}` (alternate format) for the pipeline and render error sites (lines 102-104 and 165-167). This produces a multi-line error chain with `anyhow` context, which is desirable for `build_map` failures. The JSON write errors (lines 88-95) use plain `{e}` since they are simple I/O errors.

**Decision B3 -- No additional silent-exit sites**

A scan of `main.rs` confirms the four sites identified in the plan are the only ones. No other `.is_err()` or `Err(_)` patterns exist that swallow errors without printing.

Validation exit (lines 98-100) uses `process::exit(2)` with no stderr message -- this is intentional: exit code 2 signals validation findings were present, and the findings themselves are in the JSON output on stdout.

### Goal C: Test updates (Steps 6-7)

**Decision C1 -- Invert `test_missing_workspace_section`**

The existing test expects a non-zero exit for a standalone crate without `[workspace]`. After the fix, this scenario must succeed. Renaming to `test_single_crate_without_workspace` and inverting the assertion is the correct signal: the behavior change is a new capability, not a regression.

**Decision C2 -- Depth >= 2 in new module tree test**

The deferred analysis (F5) correctly flags that `test_single_crate_module_tree` should cover modules at depth >= 2. The existing `test_deeply_nested_modules` goes 4 levels deep but uses `members = ["."]` (workspace mode). The new test should exercise at minimum `lib.rs` -> `helpers.rs` -> `helpers/sub.rs` (depth-2 submodule resolution in single-crate mode).

---

## 2. Crate Boundary Decisions

This is a single-crate tool (`rust-workspace-map`). All changes stay within this crate. No new crates are introduced.

| Change | Module | Rationale |
|--------|--------|-----------|
| `CrateRootNotFound` variant | `schema.rs` | Follows existing pattern: all library errors in `schema::Error` |
| `find_crate_root` function | `workspace.rs` | Follows existing pattern: discovery functions live in `workspace` module |
| `find_crate_root` unit tests (2) | `workspace.rs` (inline `#[cfg(test)]`) | Follows existing pattern: 5 unit tests already inline |
| Fallback logic | `lib.rs:build_map` | Follows existing pattern: pipeline orchestration in `build_map` |
| Silent exit fixes (4 sites) | `main.rs` | CLI presentation layer |
| Integration test update | `tests/integration_test.rs` | Follows existing pattern: `Command`-based integration tests |
| New integration test | `tests/integration_test.rs` | Same as above |

**Modules NOT modified and why:**

| Module | Reason unchanged |
|--------|-----------------|
| `cargo_info.rs` | Parses per-crate Cargo.toml; unaffected by workspace vs. single-crate distinction |
| `file_parser.rs` | AST extraction; operates on individual `.rs` files |
| `module_tree.rs` | Operates on a single crate root path; `build_map` passes the same paths regardless of mode |
| `cross_refs.rs` | Operates on `&mut [CrateInfo]`; in single-crate mode the slice has one element; degenerate case handled correctly |
| `render.rs` | Thin serde wrappers; unaware of workspace structure |
| `validate.rs` | Path safety confirmed (see Section 5); no changes needed |
| `indexes.rs` | Operates on `&[CrateInfo]`; single-crate case handled correctly |
| `lookup.rs` | Operates on `WorkspaceMap`; same structure regardless of mode |

---

## 3. Pattern Requirements

### 3.1 `find_crate_root` must mirror `find_workspace_root`

**Structural mirror** (workspace.rs:11-25 -> new function):

```rust
/// Walk up the directory tree from `start_path` to find a `Cargo.toml`
/// containing a `[package]` section. Returns the directory containing it.
///
/// This is the fallback path when no workspace root is found — it treats
/// a single crate as a "workspace of one".
///
/// # Errors
///
/// Returns `Error::CrateRootNotFound` if no `Cargo.toml` with a
/// `[package]` section is found in any ancestor directory.
pub fn find_crate_root(start_path: &Path) -> Result<PathBuf> {
    for ancestor in start_path.ancestors() {
        let cargo_toml = ancestor.join("Cargo.toml");
        if cargo_toml.exists() {
            let content = std::fs::read_to_string(&cargo_toml)
                .map_err(|source| Error::FileRead {
                    path: cargo_toml.clone(),
                    source,
                })?;
            if content.contains("[package]") {
                return Ok(ancestor.to_path_buf());
            }
        }
    }
    Err(Error::CrateRootNotFound(start_path.to_path_buf()))
}
```

Checklist:
- [x] Same signature: `pub fn find_crate_root(start_path: &Path) -> Result<PathBuf>`
- [x] Same `ancestors()` walk pattern
- [x] Same `FileRead` error mapping
- [x] Same `.contains()` check style (string match, same as workspace)
- [x] Same doc comment structure (`# Errors` section)
- [x] Uses `schema::Error` (already imported via `use crate::schema::{CrateType, Error, Result}`)

**Why not use TOML parsing?** Both `find_workspace_root` and `find_crate_root` use `content.contains("[...]")` -- a simple string match. Using `toml::from_str` would be more robust (handles comments, string literals) but adds a dependency on the `toml` crate for a simple existence check. The string-match approach is consistent with the existing code and avoids pulling in TOML parsing for discovery. The `MissingWorkspaceSection` fallback in `build_map` (Decision A4) mitigates the false-positive risk.

### 3.2 Unit tests must follow existing `#[cfg(test)] mod tests` pattern

Two tests, following the structure of existing workspace tests:

1. **`find_crate_root_finds_package_section`** -- creates a temp directory with `Cargo.toml` containing `[package]`, places a `lib.rs` in `src/`, walks from a nested subdirectory to find the crate root. Modeled after `find_workspace_root_finds_cargo_toml` (workspace.rs:134-142).

2. **`find_crate_root_returns_err_for_no_package`** -- creates a temp directory with no `Cargo.toml`, verifies `CrateRootNotFound`. Modeled after `enumerate_members_returns_err_for_missing_workspace` (workspace.rs:157-167).

Both tests use the existing helper `write_cargo_toml` and `setup_crate` functions already in the test module (workspace.rs:117-132).

### 3.3 Integration tests must follow `Command`-based pattern

All 10 existing integration tests and both new tests use `std::process::Command` to invoke the compiled binary. This is the established pattern.

### 3.4 Error handling pattern

`find_crate_root` returns `schema::Result<PathBuf>`. The plan's Step 4 code converts to `anyhow::Error` via `?` in the `build_map` context. This matches the existing pattern: `schema::Error` variants are converted to `anyhow::Error` at the pipeline boundary.

### 3.5 New error variant following existing conventions

```rust
#[error("no Cargo.toml with [package] section found starting from {0}")]
CrateRootNotFound(PathBuf),
```

Position: after `WorkspaceRootNotFound` (schema.rs:55), before `FileRead` (schema.rs:57). The variant carries a `PathBuf` (the start path), matching `WorkspaceRootNotFound`'s pattern. The `thiserror` format is consistent with the other variants.

---

## 4. Type Signatures for Key New Types

### 4.1 New error variant

```rust
// schema.rs, in enum Error
#[error("no Cargo.toml with [package] section found starting from {0}")]
CrateRootNotFound(PathBuf),
```

### 4.2 New discovery function

```rust
// workspace.rs
pub fn find_crate_root(start_path: &Path) -> Result<PathBuf>;
```

Where `Result<T>` is `schema::Result<T>` (aliased to `std::result::Result<T, schema::Error>`).

### 4.3 Modified `build_map` signature (unchanged, but semantics change)

```rust
// lib.rs -- signature unchanged, behavior extended
pub fn build_map(config: &Config) -> anyhow::Result<WorkspaceMap>;
```

The function now accepts both workspace-root and single-crate `config.workspace_path` values. The `WorkspaceMap.workspace_root` field will be the crate directory in single-crate mode (not a parent-of-crates directory). All downstream consumers (`validate`, `indexes`, `lookup`, `render`) are agnostic to this value.

---

## 5. Known Pitfalls and Constraints

### 5.1 Path safety in validate.rs -- CONFIRMED SAFE

The plan's Implementation Note (lines 139-141) flags a concern about `validate::validate` assuming `workspace_root` is a parent-of-crates directory. Analysis of `check_orphan_files` (validate.rs:90-158) confirms this is safe:

**In multi-crate workspace mode:**
- `workspace_root` = `/path/to/workspace/`
- `crate_info.root` = `"core/src/lib.rs"` (workspace-relative)
- `workspace_root.join("core/src/lib.rs")` = `/path/to/workspace/core/src/lib.rs` -- correct

**In single-crate mode:**
- `workspace_root` = `/path/to/crate/` (same as crate_dir)
- `crate_info.root` = `"src/lib.rs"` (workspace-relative, and workspace_root == crate_dir)
- `workspace_root.join("src/lib.rs")` = `/path/to/crate/src/lib.rs` -- correct
- `.parent()` = `/path/to/crate/src/` -- correct (finds the src directory)
- `path.strip_prefix(workspace_root)` on orphan files produces `"src/orphan_file.rs"` -- matches how `declared_files` stores module paths (via `relativize_path`)

The invariant holds: `crate_info.root` is always workspace-relative, and `workspace_root` is always the prefix that was stripped. In single-crate mode, the prefix is the crate directory itself, so `crate_info.root` values are paths like `"src/lib.rs"` rather than `"crate_name/src/lib.rs"`.

**No changes needed to validate.rs.**

### 5.2 F3 (hardcoded `strip_prefix("src/")`) -- ALREADY FIXED

The deferred item F3 referenced a prior MVP issue where `determine_parent_file` hardcoded `strip_prefix("src/")`. This was fixed in the MVP cleanup phase (commit `39ef75d`). The current code (validate.rs:161-186) derives the src directory from `crate_info.root`:

```rust
let src_dir = std::path::Path::new(&crate_info.root)
    .parent()
    .map_or(std::path::Path::new(""), |p| p);
let src_prefix = format!("{}/", src_dir.display());
```

This is correct for both workspace and single-crate modes.

### 5.3 D3 (`unwrap_or_default` in workspace.rs) -- NOT TRIGGERED IN SINGLE-CRATE MODE

`enumerate_members` uses `.unwrap_or_default()` on lines 57 and 69 for missing/malformed `members` and `exclude` arrays. In single-crate mode, `enumerate_members` is never called -- the fallback creates `vec![crate_dir]` directly. This is an existing issue specific to workspace mode and is not exacerbated by this phase.

### 5.4 `unwrap_or_default()` in `workspace_name` derivation

In `lib.rs:build_map` (line 150-153), the workspace name is derived from the last component of `workspace_root`:

```rust
let workspace_name = workspace_root
    .file_name()
    .map(|n| n.to_string_lossy().to_string())
    .unwrap_or_default();
```

In single-crate mode, `workspace_root` is the crate directory, so `workspace_name` will be the crate directory name (e.g., `"rust-workspace-map"`). This is a reasonable default and consistent with how Cargo names workspaces by their directory.

### 5.5 Cross-references in single-crate mode

`cross_refs::compute(&mut crate_infos)` (lib.rs:139) receives a slice with one element. The function initializes `TypeRef` entries for all public items, scans imports, and attempts to match the first import segment against crate names. With only one crate, all cross-crate imports will be self-referential and correctly skipped (the heuristic already checks `other_name != crate_name`). The `CrossReferences` output will be empty, which is correct.

### 5.6 Clippy on `content.contains("[package]")`

The plan uses the same `.contains()` pattern as the existing `find_workspace_root`. Clippy has no lint for this specifically; the project already uses `#[warn(clippy::pedantic)]` at the crate level (lib.rs:1) and no suppression is needed.

### 5.7 Error printing at render sites -- `main.rs` vs `run()`

The plan's Step 5 fixes the two render error sites (lines 88-90 and 93-95) as:

```rust
if let Err(e) = ... {
    eprintln!("error writing JSON to file: {e}");
    exit(1);
}
```

These sites bypass `lib.rs::run()` and call `render::render_to_writer` directly. This is the existing architecture (main.rs handles output routing, not `run()`). The fix is consistent: `main.rs` is responsible for CLI presentation, including error messages.

### 5.8 No `ErrorEntry` wiring for `CrateRootNotFound`

The deferred item D2 suggests wiring errors into `WorkspaceMap.errors`. If `find_crate_root` fails (returns `CrateRootNotFound`), the error propagates as a fatal `anyhow::Error` via `?` in `build_map`. This is consistent with the existing architecture: pre-pipeline failures (workspace root not found, missing workspace section) are fatal, not collected in `errors`. Per-crate failures (toml parse, syn parse, missing crate roots) are soft. D2 is correctly deferred as low-priority.

---

## 6. Suggested Task Grouping Rationale

The plan's 7 steps naturally group into three task groups:

### Group 1: Schema + Discovery (Steps 1-3) -- Library, TDD

**Goal tag: `G1-single-crate-core`**

| Step | Description | Approach |
|------|-------------|----------|
| 1 | Add `CrateRootNotFound` to `schema::Error` | `lib-tdd` (pure type addition, compile-checked) |
| 2 | Add `find_crate_root` to `workspace.rs` | `lib-tdd` (pure discovery logic, testable in isolation) |
| 3 | Add 2 unit tests for `find_crate_root` | `lib-tdd` (tests written before/during Step 2) |

**Rationale for grouping**: These three steps form a cohesive unit. Step 1 is a prerequisite for Step 2 (the function returns the new error variant). Step 3 validates Step 2. All three can be verified with `cargo test` without touching any other module. No integration test changes needed yet.

**TDD sequence**:
1. Add `CrateRootNotFound` variant (compiles but unused)
2. Write the two unit tests (they fail -- function doesn't exist yet)
3. Implement `find_crate_root` (tests pass)

### Group 2: Pipeline Integration + Error Visibility (Steps 4-5) -- Plumbing, Direct

**Goal tags: `G2-pipeline-fallback`, `G3-silent-exit-fix`**

| Step | Description | Approach |
|------|-------------|----------|
| 4 | Modify `build_map` fallback logic (+ D1 enhancement) | `direct` (wires existing functions; test via integration tests) |
| 5 | Fix 4 silent error exits in `main.rs` | `direct` (presentation-layer change; test via manual run or integration tests) |

**Rationale for grouping**: Steps 4 and 5 together make the feature "work end-to-end." Step 4 alone would make single-crate mode functional but leave failures invisible. Step 5 alone would improve error visibility but not add single-crate support. Together they produce a testable, user-facing improvement.

Note: Step 4 is classified as `direct` (not `lib-tdd`) because the orchestration logic in `build_map` calls existing functions (`find_workspace_root`, `enumerate_members`, `find_crate_root`) which are already unit-tested. The integration is verified by the new/modified integration tests in Group 3.

### Group 3: Integration Test Updates (Steps 6-7) -- Verification, Direct

**Goal tags: `G4-test-existing-update`, `G5-test-single-crate-module-tree`**

| Step | Description | Approach |
|------|-------------|----------|
| 6 | Update `test_missing_workspace_section` | `direct` (test behavior inversion, no new production code) |
| 7 | Add `test_single_crate_module_tree` | `direct` (new test, depth >= 2) |

**Rationale for grouping**: Both steps are integration test changes that depend on Groups 1 and 2 being complete. They can be implemented and verified together after the production code changes are in place.

**Step 7 must include depth >= 2 per F5**: The test should create a module tree of `lib.rs` -> `helpers.rs` -> `helpers/sub.rs` (at minimum). This exercises recursive module resolution in single-crate mode, which is the risk flagged by F5.

---

## 7. Verification Summary

### Per-group verification

| Group | Verification |
|-------|-------------|
| G1 | `cargo test --lib workspace` (new unit tests pass) |
| G2 | `cargo check`, `cargo clippy -- -D warnings` |
| G3 | `cargo test --test integration_test` (all 19 tests pass) |

### End-to-end verification

| # | Command | Expected |
|---|---------|----------|
| 1 | `cargo run -- index .` | Valid JSON, one crate (`rust-workspace-map`), all modules present |
| 2 | `cargo run -- index . \| jq '.crates[0].name'` | `"rust-workspace-map"` |
| 3 | `cargo run -- index /tmp/nonexistent` | Non-zero exit, clear error on stderr |
| 4 | `cargo run -- lookup . --symbol <public_item>` | Valid JSON (confirm lookup works in single-crate mode) |
| 5 | `cargo test` | All 30 unit + 19 integration tests pass |
| 6 | `cargo clippy -- -D warnings` | Clean |

### Additional verification (from deferred-and-patterns.md)

| # | Scenario | Expected |
|---|----------|----------|
| 7 | Malformed Cargo.toml (no `[workspace]` or `[package]`) in temp dir, run `index` on it | Clear error on stderr, non-zero exit |
| 8 | Workspace with valid `[workspace]` but pointing to non-existent member | `enumerate_members` error surfaces (not swallowed) |

---

## 8. Interaction Matrix: Plan Steps vs. Deferred Items

| Plan Step | Deferred Items | Risk | Disposition |
|-----------|---------------|------|-------------|
| Step 1: `CrateRootNotFound` | None | Low | Proceed as planned |
| Step 2: `find_crate_root` | D1 (MissingWorkspaceSection) | Low | Proceed as planned; D1 handled in Step 4 |
| Step 3: Unit tests | F5 (depth coverage) | Low | Proceed as planned |
| Step 4: `build_map` | **D1**, D2, D3 | **Medium** | **Extend match per Decision A4** |
| Step 5: Silent exits | F1 (being fixed) | Low | Proceed as planned |
| Step 6: Test rename | None | Low | Proceed as planned |
| Step 7: New test | F5 (depth >= 2) | Low | **Ensure depth >= 2 per Decision C2** |
## File: notes/directions/single-crate-support/task-checklist.md
# Task Checklist -- Single-Crate Support + Fix Silent Error Handling

> Generated: 2026-05-06 | Review of `draft-directions.json`
> Source: `notes/directions/single-crate-support/draft-directions.json`
> Codebase: `notes/directions/single-crate-support/codebase-state.md`

---

## Overall Assessment

**Ready to Implement with minor corrections and observations noted below.** No task is blocked; no task is ambiguous to the point of being unimplementable. One line-number error in the placement guidance for unit tests (TASK-single-crate-core-02) needs attention before coding.

---

## TASK-single-crate-core-01: Add CrateRootNotFound variant

| Criterion | Verdict | Details |
|-----------|---------|---------|
| Goal clear? | CLEAR | Add a single variant to the Error enum. |
| Files in scope correct? | CLEAR | `src/schema.rs` is the only file. The Error enum lives there (lines 52-83). |
| Implementation detail sufficient? | CLEAR | Exact variant code provided with field type, thiserror attribute, and precise insertion point (after line 55, before line 57). Verified against the source: line 55 ends `WorkspaceRootNotFound(PathBuf),`, line 57 begins `FileRead {`. Correct. |
| New types/interfaces defined clearly? | CLEAR | The variant `CrateRootNotFound(PathBuf)` is fully specified. |
| Dependencies explicit? | CLEAR | No dependencies on other tasks. |
| Verification sufficient? | CLEAR | `cargo check --workspace` is appropriate — the variant only needs to compile. |
| Wiring checklist | CLEAR | Empty (trivial task). |

### Finding

- The `known_pitfalls` entry about lib.rs line 18-21 not importing `Error` is correct — I verified `src/lib.rs` lines 18-21 import `CrateInfo, CrateType, DiagnosticKind, ErrorEntry, ErrorSeverity, ModuleInfo, WorkspaceInfo, WorkspaceMap` with no `Error`.

---

## TASK-single-crate-core-02: Add find_crate_root + 2 unit tests (lib-tdd)

| Criterion | Verdict | Details |
|-----------|---------|---------|
| Goal clear? | CLEAR | Implement `find_crate_root` following the provided TDD test code. |
| Files in scope correct? | CLEAR | `src/workspace.rs` — the function belongs in the discovery module alongside `find_workspace_root`. Verified that `Error` and `Result` are already imported (line 1: `use crate::schema::{CrateType, Error, Result}`). |
| Implementation detail sufficient? | CLEAR | Guidance provides the algorithm (ancestors walk, FileRead error mapping, `.contains("[package]")` check, CrateRootNotFound fallback), placement instructions (after `find_workspace_root`, before `enumerate_members`), doc-comment requirements, and `pub` visibility. |
| New types/interfaces defined clearly? | CLEAR | Signature is provided: `pub fn find_crate_root(start_path: &Path) -> Result<PathBuf>`. |
| Dependencies explicit? | CLEAR | Depends on TASK-single-crate-core-01 (CrateRootNotFound variant must exist first). |
| Wiring checklist | CLEAR | Empty (function definition only, no wiring needed). |

### CRITICAL: Line-number error in unit test placement

The guidance says:

> "Add both test functions to the existing #[cfg(test)] mod tests block (after line 206, before resolve_crate_roots)."

This is **incorrect**. Line 206 is the closing `}` of the `#[cfg(test)] mod tests` block. Lines 207-209 are:
```
/// Returns `(path, CrateType)` pairs — one for `src/lib.rs` (Lib),
/// one for `src/main.rs` (Bin), or empty if neither exists.
#[must_use]
```

"After line 206" places the tests **outside** the test module and inside the doc comment for `resolve_crate_roots`. The correct instruction should be: **inside the `mod tests` block, before the closing `}` at line 206** (i.e., insert before line 206, not after).

This is a minor issue — the intent is clear and any implementer would realize the error. But it should be corrected before automated tooling relies on the line numbers.

### TDD Interface Review -- PASS

| Check | Verdict | Details |
|-------|---------|---------|
| Test code specific and falsifiable? | CLEAR | `find_crate_root_finds_package_section` walks from a nested subdirectory and asserts the result equals the temp root. `find_crate_root_returns_err_for_no_package` asserts `Error::CrateRootNotFound(_)`. Both assert concrete behavior. |
| Signature matches test_code? | CLEAR | `pub fn find_crate_root(start_path: &Path) -> Result<PathBuf>` — the test code calls `find_crate_root(&nested)` and `find_crate_root(tmp.path())`, both passing `&Path`. |
| Expected behavior documented? | CLEAR | Describes ancestor walk, `[package]` detection, CrateRootNotFound and FileRead error conditions. |

One note on the test: `find_crate_root_returns_err_for_no_package` creates a temp dir with no Cargo.toml at all. Walk from `tmp.path()` up ancestors depends on no ancestor having a `[package]` Cargo.toml (e.g., inside the system temp directory). This is the same risk as the existing `find_workspace_root_finds_cargo_toml` test and is acceptable as a pattern. Not a flag.

---

## TASK-pipeline-fallback: Modify build_map for fallback logic

| Criterion | Verdict | Details |
|-----------|---------|---------|
| Goal clear? | CLEAR | Replace unconditional `?` propagation with a match that falls back to single-crate on WorkspaceRootNotFound or MissingWorkspaceSection. |
| Files in scope correct? | CLEAR | `src/lib.rs` — the `build_map` function. Verified lines 37-38 contain the two lines to replace. |
| Implementation detail sufficient? | CLEAR | Full match expression code provided with inline comments. Guidance explains the nested match, error propagation, and identical fallback path. |
| New types/interfaces defined clearly? | CLEAR | No new types/interfaces. Reuses existing `Error`, `WorkspaceRootNotFound`, `MissingWorkspaceSection`, and `find_crate_root`. |
| Dependencies explicit? | CLEAR | Depends on both TASK-single-crate-core-01 (Error::CrateRootNotFound) and TASK-single-crate-core-02 (find_crate_root function exists). |
| Verification sufficient? | CLEAR | `cargo check --workspace` and `cargo clippy -- -D warnings`. Appropriate for a logic change that doesn't expose new test surface. |

### Wiring checklist review

| Item | Verdict | Details |
|------|---------|---------|
| fn_call: `build_map calls workspace::find_crate_root` | CLEAR | The code shows two `workspace::find_crate_root(...)` calls in the match arms. `workspace` module is already imported implicitly via `pub mod workspace;` at line 12. |
| type_annotation: `schema::Error` imported for match patterns | CLEAR | The guidance explicitly says to add `Error` to the `use schema::{...}` block. I verified that `Error` is missing from the current import (lines 18-21). |

### Finding

- The match expression uses `Err(other) => return Err(other.into())` for non-fallback errors. The `into()` converts `schema::Error` into `anyhow::Error`. This is correct because `build_map` returns `anyhow::Result<WorkspaceMap>`.

---

## TASK-silent-exit-fix: Fix 4 silent error exits in main.rs

| Criterion | Verdict | Details |
|-----------|---------|---------|
| Goal clear? | CLEAR | Print error messages to stderr before `exit(1)` at four specific sites. |
| Files in scope correct? | CLEAR | `src/main.rs` only. I verified all four sites exist at the claimed locations. |
| Implementation detail sufficient? | CLEAR | Exact before/after code provided for each of the four sites, including the correct format string (`{e}` for I/O errors vs `{e:#}` for anyhow errors). Guidance also correctly marks what NOT to change (validation exit code 2, lookup serialization errors). |
| New types/interfaces defined clearly? | CLEAR | No new types. |
| Dependencies explicit? | CLEAR | No dependencies on other tasks. |
| Verification sufficient? | CLEAR | `cargo check --workspace` and `cargo clippy -- -D warnings`. Correct — no test changes, pure refactor of existing error handling. |
| Wiring checklist | CLEAR | Empty (self-contained changes to main.rs). |

### Findings

- The guidance correctly identifies four sites. I verified all four:
  - Lines 88-90: file write `.is_err()` -> silent `exit(1)` (file write)
  - Lines 93-95: stdout write `.is_err()` -> silent `exit(1)` (stdout write)
  - Lines 102-104: pipeline `Err(_)` -> silent `exit(1)` (pipeline error)
  - Lines 165-167: lookup `Err(_)` -> silent `exit(1)` (lookup error)

---

## TASK-test-existing-update: Rename and invert test_missing_workspace_section

| Criterion | Verdict | Details |
|-----------|---------|---------|
| Goal clear? | CLEAR | Rename an existing test and invert its expectations (exit 1 -> exit 0 with assertions). |
| Files in scope correct? | CLEAR | `tests/integration_test.rs`. The test `test_missing_workspace_section` is at lines 199-223. Verified. |
| Implementation detail sufficient? | CLEAR | Nine-step sequence provided for the new test body. All helper functions (`setup_crate`, `run_index`, `parse_output`, `extract_array`) exist and signatures match usage. The test name is given. |
| New types/interfaces defined clearly? | CLEAR | No new types. Reuses existing helpers. |
| Dependencies explicit? | CLEAR | Depends on TASK-pipeline-fallback (the fallback must be in place for the test to pass). |
| Verification sufficient? | CLEAR | `cargo test --test integration_test test_single_crate_without_workspace` — single test invocation. |
| Wiring checklist | CLEAR | Empty (test change only). |

### Finding

- The guidance writes `setup_crate(root, "pub struct Standalone { pub x: i32 }")` then asserts `crates[0]["name"] == "standalone"`. The `setup_crate` helper derives the crate name from `dir.file_name().unwrap()`. Since the test creates a temporary directory with a random name (not "standalone"), the assertion `crates[0]["name"] == "standalone"` will FAIL. The temp directory name is a random hex string from `tempfile::tempdir()`.

  This is a **bug in the guidance**. The test should either:
  - (a) Check that the crate name matches the **directory basename** (i.e., `root.file_name().unwrap()`), or
  - (b) Use `write_cargo_toml` to create a Cargo.toml with `name = "standalone"` explicitly.

  The guidance itself says "Assert the crate name matches the directory name: `crates[0]["name"] == "standalone"` (directory basename, from setup_crate's naming)" — but the directory basename is NOT `"standalone"`, it's the tempdir's random name. The implementer will need to fix this: either capture the dir name and assert against it, or use a known directory name like `root.join("standalone")`.

---

## TASK-test-single-crate-module-tree: Add depth >= 2 submodule test

| Criterion | Verdict | Details |
|-----------|---------|---------|
| Goal clear? | CLEAR | Add an integration test for single-crate mode with module depth >= 2. |
| Files in scope correct? | CLEAR | `tests/integration_test.rs`. |
| Implementation detail sufficient? | CLEAR | 11-step sequence provided. All helper functions confirmed to exist. The test structure mirrors the existing `test_deeply_nested_modules` (lines 284-329), which is a good reference. |
| New types/interfaces defined clearly? | CLEAR | No new types. |
| Dependencies explicit? | CLEAR | Depends on TASK-pipeline-fallback. |
| Verification sufficient? | CLEAR | `cargo test --test integration_test test_single_crate_module_tree` — single test invocation. |
| Wiring checklist | CLEAR | Empty (test change only). |

### Finding

- The guidance has the same "standalone" naming issue as the previous task, but less critically: `setup_crate(root, "pub mod helpers;")` will create a crate named by the tempdir's basename. The test asserts module paths like `"crate::helpers"` and `"crate::helpers::sub"` where `"crate"` should be replaced by the actual directory basename. The existing `test_deeply_nested_modules` avoids this by using `write_cargo_toml` with an explicit `name = "nested"` and then asserting against `"nested"`. The implementer should follow that pattern and NOT rely on `setup_crate` if they want a known crate name. Alternatively, the test should assert module paths relative to the actual dir name. This is not a hard blocker but will cause a test failure if followed literally.

---

## Task Grouping Review

| Group | Dependencies | Issues |
|-------|-------------|--------|
| group-core | None | Line number error in TASK-single-crate-core-02 for test placement. Blocking for automated tool-based implementation; not blocking for a human. |
| group-plumbing | group-core | Sound. TASK-pipeline-fallback and TASK-silent-exit-fix are independent. |
| group-tests | group-plumbing | Sound. Both tests can run in parallel. |

---

## Summary of Issues

1.  **TASK-single-crate-core-02 -- line number error (minor):** "after line 206, before resolve_crate_roots" places tests outside the `#[cfg(test)]` module. Correct insertion point is **before line 206**, not after it. Human implementers will catch this; automated code gen will not.

2.  **TASK-test-existing-update -- crate name assertion is wrong (medium):** The test guidance asserts `crates[0]["name"] == "standalone"` but `setup_crate(root, ...)` creates a crate named by the tempdir's random basename. This will fail at runtime. The assertion should match the actual directory basename or the test should use a subdirectory with a known name.

3.  **TASK-test-single-crate-module-tree -- implicit crate name issue (minor):** The test uses `setup_crate(root, ...)` and later asserts module paths like `"crate::helpers"`. The crate name will be the tempdir's random basename, not `"crate"`. The implementer needs to either use a known directory name or derive the expected paths dynamically. Follow the pattern of `test_deeply_nested_modules` which uses `write_cargo_toml` with an explicit name.

---

## Verdict

**Ready to Implement with minor corrections.** The two test assertions that hardcode crate names ("standalone" and "crate") need to be fixed in the guidance or caught during implementation. The unit test line-number error is non-blocking for a human but should be corrected for tool-assisted implementation.
## File: notes/directions/single-crate-support/workspace-map.json
{
  "files": {
    "src/lib.rs": {
      "modulePath": "rust_workspace_map",
      "parentModuleFile": null,
      "isCrateRoot": true
    },
    "src/schema.rs": {
      "modulePath": "rust_workspace_map::schema",
      "parentModuleFile": "src/lib.rs",
      "isCrateRoot": false
    },
    "src/workspace.rs": {
      "modulePath": "rust_workspace_map::workspace",
      "parentModuleFile": "src/lib.rs",
      "isCrateRoot": false
    },
    "src/cargo_info.rs": {
      "modulePath": "rust_workspace_map::cargo_info",
      "parentModuleFile": "src/lib.rs",
      "isCrateRoot": false
    },
    "src/file_parser.rs": {
      "modulePath": "rust_workspace_map::file_parser",
      "parentModuleFile": "src/lib.rs",
      "isCrateRoot": false
    },
    "src/module_tree.rs": {
      "modulePath": "rust_workspace_map::module_tree",
      "parentModuleFile": "src/lib.rs",
      "isCrateRoot": false
    },
    "src/cross_refs.rs": {
      "modulePath": "rust_workspace_map::cross_refs",
      "parentModuleFile": "src/lib.rs",
      "isCrateRoot": false
    },
    "src/render.rs": {
      "modulePath": "rust_workspace_map::render",
      "parentModuleFile": "src/lib.rs",
      "isCrateRoot": false
    },
    "src/validate.rs": {
      "modulePath": "rust_workspace_map::validate",
      "parentModuleFile": "src/lib.rs",
      "isCrateRoot": false
    },
    "src/indexes.rs": {
      "modulePath": "rust_workspace_map::indexes",
      "parentModuleFile": "src/lib.rs",
      "isCrateRoot": false
    },
    "src/lookup.rs": {
      "modulePath": "rust_workspace_map::lookup",
      "parentModuleFile": "src/lib.rs",
      "isCrateRoot": false
    },
    "src/main.rs": {
      "modulePath": "rust_workspace_map::main",
      "parentModuleFile": null,
      "isCrateRoot": false
    }
  },
  "symbols": {
    "rust_workspace_map::build_map": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map",
      "file": "src/lib.rs",
      "line": 36,
      "kind": "fn"
    },
    "rust_workspace_map::run": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map",
      "file": "src/lib.rs",
      "line": 186,
      "kind": "fn"
    },
    "rust_workspace_map::relativize_path": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map",
      "file": "src/lib.rs",
      "line": 205,
      "kind": "fn"
    },
    "rust_workspace_map::schema::CanonicalPath": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 6,
      "kind": "struct",
      "deriveAttrs": ["serde::Deserialize", "serde::Serialize"]
    },
    "rust_workspace_map::schema::WorkspaceRelativePath": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 28,
      "kind": "struct",
      "deriveAttrs": ["serde::Deserialize", "serde::Serialize"]
    },
    "rust_workspace_map::schema::Error": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 52,
      "kind": "enum",
      "deriveAttrs": ["thiserror::Error"]
    },
    "rust_workspace_map::schema::Config": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 89,
      "kind": "struct",
      "deriveAttrs": ["bon::Builder"]
    },
    "rust_workspace_map::schema::CrateType": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 104,
      "kind": "enum",
      "deriveAttrs": ["serde::Serialize"]
    },
    "rust_workspace_map::schema::WorkspaceMap": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 115,
      "kind": "struct",
      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
    },
    "rust_workspace_map::schema::WorkspaceInfo": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 141,
      "kind": "struct",
      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
    },
    "rust_workspace_map::schema::CrateInfo": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 150,
      "kind": "struct",
      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
    },
    "rust_workspace_map::schema::PackageInfo": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 164,
      "kind": "struct",
      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
    },
    "rust_workspace_map::schema::DepInfo": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 175,
      "kind": "struct",
      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
    },
    "rust_workspace_map::schema::ModuleInfo": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 193,
      "kind": "struct",
      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
    },
    "rust_workspace_map::schema::PublicItem": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 219,
      "kind": "struct",
      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
    },
    "rust_workspace_map::schema::ItemKind": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 243,
      "kind": "enum",
      "deriveAttrs": ["serde::Serialize"]
    },
    "rust_workspace_map::schema::ItemAttrs": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 254,
      "kind": "struct",
      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
    },
    "rust_workspace_map::schema::ImplInfo": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 268,
      "kind": "struct",
      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
    },
    "rust_workspace_map::schema::ImplItem": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 277,
      "kind": "struct",
      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
    },
    "rust_workspace_map::schema::ImplItemKind": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 285,
      "kind": "enum",
      "deriveAttrs": ["serde::Serialize"]
    },
    "rust_workspace_map::schema::Import": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 295,
      "kind": "struct",
      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
    },
    "rust_workspace_map::schema::ReExport": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 302,
      "kind": "struct",
      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
    },
    "rust_workspace_map::schema::CrossCrateImport": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 312,
      "kind": "struct",
      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
    },
    "rust_workspace_map::schema::CrossReferences": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 321,
      "kind": "struct",
      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
    },
    "rust_workspace_map::schema::TypeRef": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 328,
      "kind": "struct",
      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
    },
    "rust_workspace_map::schema::DiagnosticKind": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 345,
      "kind": "enum",
      "deriveAttrs": ["serde::Serialize"]
    },
    "rust_workspace_map::schema::ErrorSeverity": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 362,
      "kind": "enum",
      "deriveAttrs": ["serde::Serialize"]
    },
    "rust_workspace_map::schema::ErrorContext": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 373,
      "kind": "struct",
      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
    },
    "rust_workspace_map::schema::FileInfo": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 394,
      "kind": "struct",
      "deriveAttrs": []
    },
    "rust_workspace_map::schema::SubmoduleDecl": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 403,
      "kind": "struct",
      "deriveAttrs": ["bon::Builder"]
    },
    "rust_workspace_map::schema::SymbolEntry": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 412,
      "kind": "struct",
      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
    },
    "rust_workspace_map::schema::FileEntry": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 426,
      "kind": "struct",
      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
    },
    "rust_workspace_map::schema::ErrorEntry": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::schema",
      "file": "src/schema.rs",
      "line": 437,
      "kind": "struct",
      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
    },
    "rust_workspace_map::workspace::find_workspace_root": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::workspace",
      "file": "src/workspace.rs",
      "line": 11,
      "kind": "fn"
    },
    "rust_workspace_map::workspace::enumerate_members": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::workspace",
      "file": "src/workspace.rs",
      "line": 35,
      "kind": "fn"
    },
    "rust_workspace_map::workspace::resolve_crate_roots": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::workspace",
      "file": "src/workspace.rs",
      "line": 210,
      "kind": "fn"
    },
    "rust_workspace_map::cargo_info::parse_cargo_toml": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::cargo_info",
      "file": "src/cargo_info.rs",
      "line": 11,
      "kind": "fn"
    },
    "rust_workspace_map::file_parser::parse_file": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::file_parser",
      "file": "src/file_parser.rs",
      "line": 35,
      "kind": "fn"
    },
    "rust_workspace_map::file_parser::ParsedFile": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::file_parser",
      "file": "src/file_parser.rs",
      "line": 15,
      "kind": "struct"
    },
    "rust_workspace_map::file_parser::SynParseError": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::file_parser",
      "file": "src/file_parser.rs",
      "line": 22,
      "kind": "struct"
    },
    "rust_workspace_map::file_parser::extract_public_items": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::file_parser",
      "file": "src/file_parser.rs",
      "line": 104,
      "kind": "fn"
    },
    "rust_workspace_map::file_parser::extract_imports": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::file_parser",
      "file": "src/file_parser.rs",
      "line": 119,
      "kind": "fn"
    },
    "rust_workspace_map::file_parser::extract_re_exports": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::file_parser",
      "file": "src/file_parser.rs",
      "line": 139,
      "kind": "fn"
    },
    "rust_workspace_map::file_parser::extract_submodules": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::file_parser",
      "file": "src/file_parser.rs",
      "line": 168,
      "kind": "fn"
    },
    "rust_workspace_map::file_parser::extract_impls": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::file_parser",
      "file": "src/file_parser.rs",
      "line": 202,
      "kind": "fn"
    },
    "rust_workspace_map::module_tree::resolve_module_path": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::module_tree",
      "file": "src/module_tree.rs",
      "line": 11,
      "kind": "fn"
    },
    "rust_workspace_map::module_tree::build_module_tree": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::module_tree",
      "file": "src/module_tree.rs",
      "line": 27,
      "kind": "fn"
    },
    "rust_workspace_map::cross_refs::compute": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::cross_refs",
      "file": "src/cross_refs.rs",
      "line": 12,
      "kind": "fn"
    },
    "rust_workspace_map::render::render_json": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::render",
      "file": "src/render.rs",
      "line": 9,
      "kind": "fn"
    },
    "rust_workspace_map::render::render_to_writer": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::render",
      "file": "src/render.rs",
      "line": 18,
      "kind": "fn"
    },
    "rust_workspace_map::validate::validate": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::validate",
      "file": "src/validate.rs",
      "line": 72,
      "kind": "fn"
    },
    "rust_workspace_map::validate::is_external_crate_re_export": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::validate",
      "file": "src/validate.rs",
      "line": 15,
      "kind": "fn"
    },
    "rust_workspace_map::validate::check_orphan_files": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::validate",
      "file": "src/validate.rs",
      "line": 90,
      "kind": "fn"
    },
    "rust_workspace_map::validate::check_dead_reexports": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::validate",
      "file": "src/validate.rs",
      "line": 189,
      "kind": "fn"
    },
    "rust_workspace_map::validate::is_derive_companion": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::validate",
      "file": "src/validate.rs",
      "line": 254,
      "kind": "fn"
    },
    "rust_workspace_map::validate::resolve_import_path": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::validate",
      "file": "src/validate.rs",
      "line": 278,
      "kind": "fn"
    },
    "rust_workspace_map::indexes::derive_from_crates": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::indexes",
      "file": "src/indexes.rs",
      "line": 28,
      "kind": "fn"
    },
    "rust_workspace_map::lookup::lookup_symbol": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::lookup",
      "file": "src/lookup.rs",
      "line": 47,
      "kind": "fn"
    },
    "rust_workspace_map::lookup::lookup_file": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::lookup",
      "file": "src/lookup.rs",
      "line": 88,
      "kind": "fn"
    },
    "rust_workspace_map::lookup::SymbolLookupResult": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::lookup",
      "file": "src/lookup.rs",
      "line": 7,
      "kind": "enum",
      "deriveAttrs": ["serde::Serialize"]
    },
    "rust_workspace_map::lookup::DisambiguationHint": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::lookup",
      "file": "src/lookup.rs",
      "line": 22,
      "kind": "struct",
      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
    },
    "rust_workspace_map::lookup::FileLookupResult": {
      "crateName": "rust-workspace-map",
      "module": "rust_workspace_map::lookup",
      "file": "src/lookup.rs",
      "line": 33,
      "kind": "struct",
      "deriveAttrs": ["bon::Builder", "serde::Serialize"]
    }
  },
  "nameIndex": {
    "CanonicalPath": ["rust_workspace_map::schema::CanonicalPath"],
    "WorkspaceRelativePath": ["rust_workspace_map::schema::WorkspaceRelativePath"],
    "Error": ["rust_workspace_map::schema::Error"],
    "Config": ["rust_workspace_map::schema::Config"],
    "CrateType": ["rust_workspace_map::schema::CrateType"],
    "WorkspaceMap": ["rust_workspace_map::schema::WorkspaceMap"],
    "WorkspaceInfo": ["rust_workspace_map::schema::WorkspaceInfo"],
    "CrateInfo": ["rust_workspace_map::schema::CrateInfo"],
    "PackageInfo": ["rust_workspace_map::schema::PackageInfo"],
    "DepInfo": ["rust_workspace_map::schema::DepInfo"],
    "ModuleInfo": ["rust_workspace_map::schema::ModuleInfo"],
    "PublicItem": ["rust_workspace_map::schema::PublicItem"],
    "ItemKind": ["rust_workspace_map::schema::ItemKind"],
    "ItemAttrs": ["rust_workspace_map::schema::ItemAttrs"],
    "ImplInfo": ["rust_workspace_map::schema::ImplInfo"],
    "ImplItem": ["rust_workspace_map::schema::ImplItem"],
    "ImplItemKind": ["rust_workspace_map::schema::ImplItemKind"],
    "Import": ["rust_workspace_map::schema::Import"],
    "ReExport": ["rust_workspace_map::schema::ReExport"],
    "CrossCrateImport": ["rust_workspace_map::schema::CrossCrateImport"],
    "CrossReferences": ["rust_workspace_map::schema::CrossReferences"],
    "TypeRef": ["rust_workspace_map::schema::TypeRef"],
    "DiagnosticKind": ["rust_workspace_map::schema::DiagnosticKind"],
    "ErrorSeverity": ["rust_workspace_map::schema::ErrorSeverity"],
    "ErrorContext": ["rust_workspace_map::schema::ErrorContext"],
    "FileInfo": ["rust_workspace_map::schema::FileInfo"],
    "SubmoduleDecl": ["rust_workspace_map::schema::SubmoduleDecl"],
    "SymbolEntry": ["rust_workspace_map::schema::SymbolEntry"],
    "FileEntry": ["rust_workspace_map::schema::FileEntry"],
    "ErrorEntry": ["rust_workspace_map::schema::ErrorEntry"],
    "find_workspace_root": ["rust_workspace_map::workspace::find_workspace_root"],
    "enumerate_members": ["rust_workspace_map::workspace::enumerate_members"],
    "resolve_crate_roots": ["rust_workspace_map::workspace::resolve_crate_roots"],
    "parse_cargo_toml": ["rust_workspace_map::cargo_info::parse_cargo_toml"],
    "parse_file": ["rust_workspace_map::file_parser::parse_file"],
    "ParsedFile": ["rust_workspace_map::file_parser::ParsedFile"],
    "SynParseError": ["rust_workspace_map::file_parser::SynParseError"],
    "extract_public_items": ["rust_workspace_map::file_parser::extract_public_items"],
    "extract_imports": ["rust_workspace_map::file_parser::extract_imports"],
    "extract_re_exports": ["rust_workspace_map::file_parser::extract_re_exports"],
    "extract_submodules": ["rust_workspace_map::file_parser::extract_submodules"],
    "extract_impls": ["rust_workspace_map::file_parser::extract_impls"],
    "resolve_module_path": ["rust_workspace_map::module_tree::resolve_module_path"],
    "build_module_tree": ["rust_workspace_map::module_tree::build_module_tree"],
    "compute": ["rust_workspace_map::cross_refs::compute"],
    "render_json": ["rust_workspace_map::render::render_json"],
    "render_to_writer": ["rust_workspace_map::render::render_to_writer"],
    "validate": ["rust_workspace_map::validate::validate"],
    "is_external_crate_re_export": ["rust_workspace_map::validate::is_external_crate_re_export"],
    "check_orphan_files": ["rust_workspace_map::validate::check_orphan_files"],
    "check_dead_reexports": ["rust_workspace_map::validate::check_dead_reexports"],
    "is_derive_companion": ["rust_workspace_map::validate::is_derive_companion"],
    "resolve_import_path": ["rust_workspace_map::validate::resolve_import_path"],
    "derive_from_crates": ["rust_workspace_map::indexes::derive_from_crates"],
    "lookup_symbol": ["rust_workspace_map::lookup::lookup_symbol"],
    "lookup_file": ["rust_workspace_map::lookup::lookup_file"],
    "SymbolLookupResult": ["rust_workspace_map::lookup::SymbolLookupResult"],
    "DisambiguationHint": ["rust_workspace_map::lookup::DisambiguationHint"],
    "FileLookupResult": ["rust_workspace_map::lookup::FileLookupResult"],
    "build_map": ["rust_workspace_map::build_map"],
    "run": ["rust_workspace_map::run"],
    "relativize_path": ["rust_workspace_map::relativize_path"]
  },
  "crossReferences": {
    "types": {}
  }
}
## File: src/lib.rs
#![warn(clippy::pedantic)]

pub mod cargo_info;
pub mod cross_refs;
pub mod file_parser;
pub mod indexes;
pub mod lookup;
pub mod module_tree;
pub mod render;
pub mod schema;
pub mod validate;
pub mod workspace;

pub use schema::Config;

use anyhow::Context;
use rayon::prelude::*;
use schema::{
    CrateInfo, CrateType, DiagnosticKind, Error, ErrorEntry, ErrorSeverity, ModuleInfo,
    WorkspaceInfo, WorkspaceMap,
};
use std::path::Path;

/// Build a `WorkspaceMap` from the given config, without rendering.
///
/// This function contains all pipeline logic up to and including
/// `WorkspaceMap` construction — workspace discovery, per-crate processing,
/// cross-refs computation, index derivation, optional validation, and map building.
///
/// # Errors
///
/// Returns an error if the workspace root cannot be found, the workspace
/// Cargo.toml is missing a `[workspace]` section, member crates cannot be
/// parsed, or the JSON output cannot be written.
#[allow(clippy::too_many_lines)]
pub fn build_map(config: &Config) -> anyhow::Result<WorkspaceMap> {
    let (workspace_root, member_dirs) = match workspace::find_workspace_root(&config.workspace_path) {
        Ok(root) => {
            match workspace::enumerate_members(&root) {
                Ok(members) => (root, members),
                Err(Error::MissingWorkspaceSection) => {
                    // [workspace] was a false positive (e.g., in a comment).
                    // Fall back to single-crate discovery.
                    let crate_dir = workspace::find_crate_root(&config.workspace_path)?;
                    (crate_dir.clone(), vec![crate_dir])
                }
                Err(other) => return Err(other.into()),
            }
        }
        Err(Error::WorkspaceRootNotFound(_)) => {
            let crate_dir = workspace::find_crate_root(&config.workspace_path)?;
            (crate_dir.clone(), vec![crate_dir])
        }
        Err(other) => return Err(other.into()),
    };

    let mut crate_errors: Vec<ErrorEntry> = Vec::new();

    let results: Vec<(Option<CrateInfo>, Vec<ErrorEntry>)> = member_dirs
        .par_iter()
        .map(|dir| {
            let cargo_toml = dir.join("Cargo.toml");
            let mut crate_errors = Vec::new();

            let (pkg, deps) = match cargo_info::parse_cargo_toml(&cargo_toml) {
                Ok(v) => v,
                Err(e) => {
                    crate_errors.push(ErrorEntry::builder()
                        .file(cargo_toml.to_string_lossy().to_string())
                        .message(format!("failed to parse Cargo.toml: {e}"))
                        .severity(ErrorSeverity::Error)
                        .kind(DiagnosticKind::TomlParseError)
                        .cause(e.to_string())
                        .build());
                    return (None, crate_errors);
                }
            };

            let roots = workspace::resolve_crate_roots(dir);
            if roots.is_empty() {
                crate_errors.push(ErrorEntry::builder()
                    .file(dir.to_string_lossy().to_string())
                    .message("no crate entry points found".to_string())
                    .severity(ErrorSeverity::Warning)
                    .kind(DiagnosticKind::MissingCrateRoots)
                    .build());
                return (None, crate_errors);
            }

            let crate_type = if roots.iter().any(|(_, t)| *t == CrateType::Lib)
                && roots.iter().any(|(_, t)| *t == CrateType::Bin)
            {
                CrateType::LibAndBin
            } else {
                roots.first().map_or(CrateType::Lib, |(_, t)| *t)
            };

            let pkg_name = pkg.name.clone();
            let mut modules: Vec<ModuleInfo> = Vec::new();
            let mut collected_errors = Vec::new();
            for (root, _ty) in &roots {
                let (m, e) = module_tree::build_module_tree(root, &pkg_name);
                modules.extend(m);
                collected_errors.extend(e);
            }
            for err in collected_errors {
                crate_errors.push(err);
            }

            // Relativize all paths to the workspace root.
            for m in &mut modules {
                m.file = relativize_path(&m.file, &workspace_root);
                for item in &mut m.public_items {
                    item.file = relativize_path(&item.file, &workspace_root);
                }
            }

            let crate_root = roots
                .first()
                .map(|(r, _)| relativize_path(&r.to_string_lossy(), &workspace_root))
                .unwrap_or_default();

            let rebuilt_pkg = schema::PackageInfo::builder()
                .name(pkg.name)
                .version(pkg.version)
                .edition(pkg.edition)
                .crate_type(crate_type)
                .build();

            let crate_info = CrateInfo::builder()
                .name(pkg_name)
                .root(crate_root)
                .package(rebuilt_pkg)
                .modules(modules)
                .deps(deps)
                .build();

            (Some(crate_info), crate_errors)
        })
        .collect();

    let mut crate_infos: Vec<CrateInfo> = Vec::new();

    for (info, errs) in results {
        // Extend errors in both branches before checking info.
        // `errs` is moved by `extend` — this is fine because results is consumed by the for loop (move iteration).
        if let Some(ci) = info {
            crate_infos.push(ci);
        }
        crate_errors.extend(errs);
    }

    // Deterministic sort by crate name.
    crate_infos.sort_by(|a, b| a.name.cmp(&b.name));

    let cross_refs = cross_refs::compute(&mut crate_infos);

    // Derive flat indexes from crate info.
    let (symbols, name_index, files) = indexes::derive_from_crates(&crate_infos);

    // Run validation if enabled.
    if config.validate {
        let validate_findings = validate::validate(&crate_infos, &symbols, &workspace_root);
        crate_errors.extend(validate_findings);
    }

    let workspace_name = workspace_root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    let workspace_info = WorkspaceInfo::builder()
        .root(workspace_root.to_string_lossy().to_string())
        .workspace_name(workspace_name)
        .build();

    let map = WorkspaceMap::builder()
        .workspace(workspace_info)
        .crates(crate_infos)
        .cross_references(cross_refs)
        .symbols(symbols)
        .name_index(name_index)
        .files(files)
        .errors(crate_errors)
        .workspace_root(workspace_root.clone())
        .build();

    Ok(map)
}

/// Run the full workspace mapping pipeline.
///
/// 1. Discover workspace root and member crates.
/// 2. Process each crate in parallel (Cargo.toml parsing + module tree).
/// 3. Compute cross-crate references.
/// 4. Render JSON to stdout or the configured output file.
///
/// # Errors
///
/// Returns an error if the workspace root cannot be found, the workspace
/// Cargo.toml is missing a `[workspace]` section, member crates cannot be
/// parsed, or the JSON output cannot be written.
pub fn run(config: &Config) -> anyhow::Result<()> {
    let map = build_map(config)?;

    if let Some(ref output_path) = config.output_path {
        let file = std::fs::File::create(output_path)
            .with_context(|| format!("failed to create output file: {}", output_path.display()))?;
        let writer = std::io::BufWriter::new(file);
        render::render_to_writer(&map, writer)?;
    } else {
        let stdout = std::io::stdout();
        render::render_to_writer(&map, stdout.lock())?;
    }

    Ok(())
}

/// Strip the workspace root prefix from a path string, returning a
/// workspace-relative path. If the prefix doesn't match, returns the
/// original string unchanged.
fn relativize_path(path_str: &str, root: &Path) -> String {
    let p = Path::new(path_str);
    match p.strip_prefix(root) {
        Ok(rel) => rel.to_string_lossy().to_string(),
        Err(_) => path_str.to_string(),
    }
}
## File: src/main.rs
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Subcommand)]
enum Command {
    /// Generate a JSON map of a Rust workspace's public API surface
    Index {
        /// Path to the workspace root or any directory within it
        path: PathBuf,
        /// Write JSON output to file instead of stdout
        #[arg(short = 'o', long = "output", value_name = "FILE")]
        output: Option<PathBuf>,
        /// Run validation checks (orphan files, dead re-exports)
        #[arg(long)]
        validate: bool,
    },
    /// Look up a symbol or file in a previously generated workspace map
    Lookup {
        /// Path to the workspace root or any directory within it
        path: PathBuf,
        /// Look up by symbol name
        #[arg(long, conflicts_with = "file")]
        symbol: Option<String>,
        /// Look up by file path
        #[arg(long, conflicts_with = "symbol")]
        file: Option<String>,
    },
}

#[derive(Parser)]
#[command(
    name = "rust-workspace-map",
    version,
    about = "Generate a JSON map of a Rust workspace's public API surface"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Index { path, output, validate } => {
            let workspace_path = std::path::absolute(&path)
                .unwrap_or_else(|e| {
                    eprintln!("invalid path {}: {}", path.display(), e);
                    std::process::exit(1);
                });

            let config = if let Some(ref output) = output {
                rust_workspace_map::Config::builder()
                    .workspace_path(workspace_path)
                    .output_path(output.clone())
                    .validate(validate)
                    .build()
            } else {
                rust_workspace_map::Config::builder()
                    .workspace_path(workspace_path)
                    .validate(validate)
                    .build()
            };

            match rust_workspace_map::build_map(&config) {
                Ok(map) => {
                    let exit_code = if validate {
                        map.errors.iter().any(|e| {
                            matches!(
                                e.kind,
                                rust_workspace_map::schema::DiagnosticKind::OrphanFile
                                    | rust_workspace_map::schema::DiagnosticKind::DeadReExport
                            ) && e.severity == rust_workspace_map::schema::ErrorSeverity::Warning
                        })
                    } else {
                        false
                    };

                    if let Some(ref output_path) = config.output_path {
                        let file = match std::fs::File::create(output_path) {
                            Ok(f) => f,
                            Err(e) => {
                                eprintln!("failed to create output file: {e}");
                                std::process::exit(1);
                            }
                        };
                        let writer = std::io::BufWriter::new(file);
                        if let Err(e) = rust_workspace_map::render::render_to_writer(&map, writer) {
                            eprintln!("error writing JSON to file: {e}");
                            std::process::exit(1);
                        }
                    } else {
                        let stdout = std::io::stdout();
                        if let Err(e) = rust_workspace_map::render::render_to_writer(&map, stdout.lock()) {
                            eprintln!("error writing JSON to stdout: {e}");
                            std::process::exit(1);
                        }
                    }

                    if exit_code {
                        std::process::exit(2);
                    }
                }
                Err(e) => {
                    eprintln!("error: {e:#}");
                    std::process::exit(1);
                }
            }
        }
        Command::Lookup { path, symbol, file } => {
            let workspace_path = std::path::absolute(&path)
                .unwrap_or_else(|e| {
                    eprintln!("invalid path {}: {}", path.display(), e);
                    std::process::exit(1);
                });

            let config = rust_workspace_map::Config::builder()
                .workspace_path(workspace_path)
                .build();

            match rust_workspace_map::build_map(&config) {
                Ok(map) => {
                    match (symbol, file) {
                        (Some(sym), None) => {
                            let result = rust_workspace_map::lookup::lookup_symbol(&map, &sym);
                            let json = match serde_json::to_string_pretty(&result) {
                                Ok(j) => j,
                                Err(_) => {
                                    eprintln!("serialization error");
                                    std::process::exit(1);
                                }
                            };
                            println!("{json}");
                            // Exit 1 on NotFound, 0 otherwise.
                            if matches!(result, rust_workspace_map::lookup::SymbolLookupResult::NotFound) {
                                std::process::exit(1);
                            }
                        }
                        (None, Some(f)) => {
                            match rust_workspace_map::lookup::lookup_file(&map, &f) {
                                Some(result) => {
                                    let json = match serde_json::to_string_pretty(&result) {
                                        Ok(j) => j,
                                        Err(_) => {
                                            eprintln!("serialization error");
                                            std::process::exit(1);
                                        }
                                    };
                                    println!("{json}");
                                }
                                None => {
                                    eprintln!("file not found: {f}");
                                    std::process::exit(1);
                                }
                            }
                        }
                        (Some(_), Some(_)) => {
                            // clap's conflicts_with handles this, but be defensive.
                            eprintln!("cannot specify both --symbol and --file");
                            std::process::exit(1);
                        }
                        (None, None) => {
                            eprintln!("must specify either --symbol or --file");
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("error: {e:#}");
                    std::process::exit(1);
                }
            }
        }
    }
}
## File: src/schema.rs
use std::collections::BTreeMap;
use std::path::PathBuf;

// ── Path newtypes for flat indexes ──────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct CanonicalPath(pub String);

impl std::fmt::Display for CanonicalPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for CanonicalPath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for CanonicalPath {
    fn from(s: String) -> Self {
        Self(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct WorkspaceRelativePath(pub String);

impl std::fmt::Display for WorkspaceRelativePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for WorkspaceRelativePath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for WorkspaceRelativePath {
    fn from(s: String) -> Self {
        Self(s)
    }
}

// ── Error type ──────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no workspace root found starting from {0}")]
    WorkspaceRootNotFound(PathBuf),

    #[error("no Cargo.toml with [package] section found starting from {0}")]
    CrateRootNotFound(PathBuf),

    #[error("failed to read file {path}: {source}")]
    FileRead {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("failed to parse {path}: {source}")]
    TomlParse {
        path: PathBuf,
        source: toml::de::Error,
    },

    #[error("failed to parse Rust source {path}: {source}")]
    SynParse {
        path: PathBuf,
        source: syn::Error,
    },

    #[error("workspace member {0} does not exist")]
    MemberNotFound(PathBuf),

    #[error("glob pattern error: {0}")]
    GlobPattern(String),

    #[error("workspace Cargo.toml is missing the [workspace] section")]
    MissingWorkspaceSection,
}

pub type Result<T> = std::result::Result<T, Error>;

// ── Config ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, bon::Builder)]
pub struct Config {
    /// Absolute, canonical path to the workspace root (or a subdirectory within it).
    pub workspace_path: PathBuf,

    /// If Some, write JSON to this file instead of stdout.
    pub output_path: Option<PathBuf>,

    /// When true, run validation checks (orphan files, dead re-exports).
    #[builder(default)]
    pub validate: bool,
}

// ── Crate type ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CrateType {
    Lib,
    Bin,
    #[serde(rename = "lib_and_bin")]
    LibAndBin,
}

// ── Top-level output ────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceMap {
    pub workspace: WorkspaceInfo,
    pub crates: Vec<CrateInfo>,
    pub cross_references: CrossReferences,

    #[builder(default)]
    pub symbols: BTreeMap<CanonicalPath, SymbolEntry>,

    #[builder(default)]
    pub name_index: BTreeMap<String, Vec<CanonicalPath>>,

    #[builder(default)]
    pub files: BTreeMap<WorkspaceRelativePath, FileEntry>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<ErrorEntry>,

    /// Not serialized — used for path relativization during construction.
    #[builder(default)]
    #[serde(skip)]
    pub workspace_root: PathBuf,
}

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceInfo {
    pub root: String,
    pub workspace_name: String,
}

// ── Crate ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct CrateInfo {
    pub name: String,
    pub root: String,
    pub package: PackageInfo,
    pub modules: Vec<ModuleInfo>,
    pub deps: DepInfo,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cross_crate_imports: Vec<CrossCrateImport>,
}

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub edition: String,
    pub crate_type: CrateType,
}

// ── Dependencies ────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, Default, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct DepInfo {
    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub normal: Vec<String>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dev: Vec<String>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub workspace_members: Vec<String>,
}

// ── Module ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ModuleInfo {
    pub path: String,
    pub file: String,
    pub visibility: String,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub public_items: Vec<PublicItem>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub imports: Vec<Import>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub re_exports: Vec<ReExport>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub submodules: Vec<String>,
}

// ── Public items ────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct PublicItem {
    pub kind: ItemKind,
    pub name: String,
    pub file: String,
    pub line: usize,
    pub attrs: ItemAttrs,
    pub generics: String,
    pub visibility: String,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<String>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub variants: Vec<String>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub impls: Vec<ImplInfo>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ItemKind {
    Struct,
    Enum,
    Trait,
    Fn,
    Type,
    Macro,
}

#[derive(Debug, Clone, serde::Serialize, Default, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ItemAttrs {
    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub derive: Vec<String>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub doc: Vec<String>,
}

// ── Impl blocks ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ImplInfo {
    /// Serialized as "type" in JSON.
    #[serde(rename = "type")]
    pub type_: String,
    pub items: Vec<ImplItem>,
}

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ImplItem {
    pub kind: ImplItemKind,
    pub name: String,
    pub params: String,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ImplItemKind {
    Fn,
    Type,
    Const,
}

// ── Imports / Re-exports ────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct Import {
    pub path: String,
    pub line: usize,
}

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ReExport {
    pub import_path: String,
    pub export_path: String,
    pub line: usize,
}

// ── Cross-crate ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct CrossCrateImport {
    pub import_path: String,
    pub target_crate: String,
    pub symbol: String,
    pub line: usize,
}

#[derive(Debug, Clone, serde::Serialize, Default, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct CrossReferences {
    #[builder(default)]
    pub types: BTreeMap<String, TypeRef>,
}

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct TypeRef {
    pub crate_name: String,
    pub kind: String,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub imported_by: Vec<String>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub exported_by: Vec<String>,
}

// ── Diagnostic kind ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticKind {
    OrphanedModule,
    TomlParseError,
    MissingCrateRoots,
    ModuleTreeError,
    SynParseError,
    MissingWorkspaceSection,
    GlobPatternError,
    MemberNotFound,
    OrphanFile,
    DeadReExport,
}

// ── Error severity ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorSeverity {
    Error,
    Warning,
}

// ── Error context ───────────────────────────────────────────────────────

/// Optional context attached to an error, providing additional location
/// and source information for diagnostics.
#[derive(Debug, Clone, Default, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ErrorContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crate_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_path: Option<String>,

    /// Line number in the source file where the error occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<usize>,

    /// A short source snippet near the error location (if available).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
}

// ── Internal types ──────────────────────────────────────────────────────

/// Internal intermediate type consumed by `module_tree`.
#[derive(Debug, Clone, Default)]
pub struct FileInfo {
    pub public_items: Vec<PublicItem>,
    pub imports: Vec<Import>,
    pub re_exports: Vec<ReExport>,
    pub submodules: Vec<SubmoduleDecl>,
    pub impls: Vec<ImplInfo>,
}

#[derive(Debug, Clone, bon::Builder)]
pub struct SubmoduleDecl {
    pub name: String,
    #[builder(default)]
    pub is_test: bool,
}

// ── Flat index entry types ──────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct SymbolEntry {
    pub crate_name: String,
    pub module: String,
    pub file: String,
    pub line: usize,
    pub kind: ItemKind,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub derive_attrs: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct FileEntry {
    pub module_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_module_file: Option<String>,
    pub is_crate_root: bool,
}

// ── Error reporting ─────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ErrorEntry {
    pub file: String,
    #[builder(default)]
    pub line: usize,
    pub message: String,
    pub severity: ErrorSeverity,
    pub kind: DiagnosticKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ErrorContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}
## File: src/workspace.rs
use crate::schema::{CrateType, Error, Result};
use std::path::{Path, PathBuf};

/// Walk up the directory tree from `start_path` to find a `Cargo.toml`
/// containing a `[workspace]` section. Returns the directory containing it.
///
/// # Errors
///
/// Returns `Error::WorkspaceRootNotFound` if no `Cargo.toml` with a
/// `[workspace]` section is found in any ancestor directory.
pub fn find_workspace_root(start_path: &Path) -> Result<PathBuf> {
    for ancestor in start_path.ancestors() {
        let cargo_toml = ancestor.join("Cargo.toml");
        if cargo_toml.exists() {
            let content = std::fs::read_to_string(&cargo_toml).map_err(|source| Error::FileRead {
                path: cargo_toml.clone(),
                source,
            })?;
            if content.contains("[workspace]") {
                return Ok(ancestor.to_path_buf());
            }
        }
    }
    Err(Error::WorkspaceRootNotFound(start_path.to_path_buf()))
}

/// Walk up the directory tree from `start_path` to find a `Cargo.toml`
/// containing a `[package]` section. Returns the directory containing it.
///
/// # Errors
///
/// Returns `Error::CrateRootNotFound` if no `Cargo.toml` with a `[package]`
/// section is found in any ancestor directory.
pub fn find_crate_root(start_path: &Path) -> Result<PathBuf> {
    for ancestor in start_path.ancestors() {
        let cargo_toml = ancestor.join("Cargo.toml");
        if cargo_toml.exists() {
            let content =
                std::fs::read_to_string(&cargo_toml).map_err(|source| Error::FileRead {
                    path: cargo_toml.clone(),
                    source,
                })?;
            if content.contains("[package]") {
                return Ok(ancestor.to_path_buf());
            }
        }
    }
    Err(Error::CrateRootNotFound(start_path.to_path_buf()))
}

/// Parse the workspace `Cargo.toml`, resolve member paths (including glob
/// patterns), apply `exclude` list, and return absolute paths to each member
/// crate directory.
///
/// # Errors
///
/// Returns `Error::MissingWorkspaceSection` if the `Cargo.toml` lacks a
/// `[workspace]` section entirely.
pub fn enumerate_members(root: &Path) -> Result<Vec<PathBuf>> {
    let cargo_toml_path = root.join("Cargo.toml");
    let content = std::fs::read_to_string(&cargo_toml_path).map_err(|source| Error::FileRead {
        path: cargo_toml_path.clone(),
        source,
    })?;

    let parsed: toml::Value = toml::from_str(&content).map_err(|source| Error::TomlParse {
        path: cargo_toml_path.clone(),
        source,
    })?;

    let members: Vec<String> = match parsed.get("workspace") {
        None => return Err(Error::MissingWorkspaceSection),
        Some(workspace) => workspace
            .get("members")
            .and_then(|m| m.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default(),
    };

    let exclude: Vec<String> = parsed
        .get("workspace")
        .and_then(|w| w.get("exclude"))
        .and_then(|e| e.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let mut result = Vec::new();
    for member in &members {
        let has_glob = member.contains('*') || member.contains('?') || member.contains('[');
        if has_glob {
            let pattern = root.join(member).to_string_lossy().to_string();
            let iter = glob::glob(&pattern).map_err(|e| Error::GlobPattern(e.to_string()))?;
            for entry in iter {
                let path = entry.map_err(|e| Error::GlobPattern(e.to_string()))?;
                if path.is_dir() && path.join("Cargo.toml").exists() {
                    result.push(path);
                }
            }
        } else {
            let path = root.join(member);
            if path.is_dir() && path.join("Cargo.toml").exists() {
                result.push(path);
            } else {
                eprintln!(
                    "warning: workspace member {} does not exist",
                    path.display()
                );
            }
        }
    }

    result.retain(|p| {
        let name = p
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        !exclude.contains(&name)
    });

    result.sort();
    result.dedup();
    Ok(result)
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_cargo_toml(dir: &std::path::Path, content: &str) {
        let mut f = std::fs::File::create(dir.join("Cargo.toml")).unwrap();
        f.write_all(content.as_bytes()).unwrap();
    }

    fn setup_crate(dir: &std::path::Path) {
        let src = dir.join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("lib.rs"), "").unwrap();
        let mut f = std::fs::File::create(dir.join("Cargo.toml")).unwrap();
        use std::io::Write;
        writeln!(f, "[package]").unwrap();
        writeln!(f, "name = \"{}\"", dir.file_name().unwrap().to_string_lossy()).unwrap();
        writeln!(f, "version = \"0.1.0\"").unwrap();
        writeln!(f, "edition = \"2021\"").unwrap();
    }

    #[test]
    fn find_workspace_root_finds_cargo_toml() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("subdir").join("nested");
        std::fs::create_dir_all(&path).unwrap();
        write_cargo_toml(tmp.path(), "[workspace]");
        let result = find_workspace_root(&path).unwrap();
        assert_eq!(result, tmp.path());
    }

    #[test]
    fn enumerate_members_returns_members() {
        let tmp = tempfile::tempdir().unwrap();
        write_cargo_toml(tmp.path(), r#"
[workspace]
members = ["crate_a", "crate_b"]
"#);
        setup_crate(tmp.path().join("crate_a").as_path());
        setup_crate(tmp.path().join("crate_b").as_path());
        let members = enumerate_members(tmp.path()).unwrap();
        assert_eq!(members.len(), 2);
    }

    #[test]
    fn enumerate_members_returns_err_for_missing_workspace() {
        let tmp = tempfile::tempdir().unwrap();
        write_cargo_toml(tmp.path(), "[dependencies]\nfoo = \"1\"");
        let result = enumerate_members(tmp.path());
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::MissingWorkspaceSection => {},
            other => panic!("expected MissingWorkspaceSection, got {:?}", other),
        }
    }

    #[test]
    fn enumerate_members_applies_exclude() {
        let tmp = tempfile::tempdir().unwrap();
        write_cargo_toml(tmp.path(), r#"
[workspace]
members = ["a", "b", "c"]
exclude = ["b"]
"#);
        setup_crate(tmp.path().join("a").as_path());
        setup_crate(tmp.path().join("b").as_path());
        setup_crate(tmp.path().join("c").as_path());
        let members = enumerate_members(tmp.path()).unwrap();
        let names: Vec<_> = members.iter().map(|p| p.file_name().unwrap().to_string_lossy()).collect();
        assert!(names.iter().any(|n| *n == "a"));
        assert!(!names.iter().any(|n| *n == "b"));
        assert!(names.iter().any(|n| *n == "c"));
    }

    #[test]
    fn resolve_crate_roots_detects_lib() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join("src")).unwrap();
        std::fs::write(tmp.path().join("src").join("lib.rs"), "").unwrap();
        let roots = resolve_crate_roots(tmp.path());
        assert_eq!(roots.len(), 1);
        assert_eq!(roots[0].1, CrateType::Lib);
    }

    #[test]
    fn resolve_crate_roots_detects_bin() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join("src")).unwrap();
        std::fs::write(tmp.path().join("src").join("main.rs"), "").unwrap();
        let roots = resolve_crate_roots(tmp.path());
        assert_eq!(roots.len(), 1);
        assert_eq!(roots[0].1, CrateType::Bin);
    }

    #[test]
    fn find_crate_root_finds_package_section() {
        let tmp = tempfile::tempdir().unwrap();
        setup_crate(tmp.path());
        // Walk from a nested subdirectory inside src
        let nested = tmp.path().join("src").join("subdir");
        std::fs::create_dir_all(&nested).unwrap();
        let result = find_crate_root(&nested).unwrap();
        assert_eq!(result, tmp.path());
    }

    #[test]
    fn find_crate_root_returns_err_for_no_package() {
        let tmp = tempfile::tempdir().unwrap();
        // No Cargo.toml at all — ancestors() walks up and finds nothing
        let result = find_crate_root(tmp.path());
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::CrateRootNotFound(_) => {},
            other => panic!("expected CrateRootNotFound, got {:?}", other),
        }
    }
}
/// Returns `(path, CrateType)` pairs — one for `src/lib.rs` (Lib),
/// one for `src/main.rs` (Bin), or empty if neither exists.
#[must_use]
pub fn resolve_crate_roots(crate_dir: &Path) -> Vec<(PathBuf, CrateType)> {
    let mut roots = Vec::new();
    let lib_rs = crate_dir.join("src").join("lib.rs");
    let main_rs = crate_dir.join("src").join("main.rs");
    if lib_rs.exists() {
        roots.push((lib_rs, CrateType::Lib));
    }
    if main_rs.exists() {
        roots.push((main_rs, CrateType::Bin));
    }
    roots
}
## File: tests/integration_test.rs
use std::process::Command;

fn binary_path() -> String {
    let root = env!("CARGO_MANIFEST_DIR");
    format!("{}/target/debug/rust-workspace-map", root)
}

fn extract_array<'a>(val: &'a serde_json::Value, key: &str) -> Vec<&'a serde_json::Value> {
    val.get(key)
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().collect())
        .unwrap_or_default()
}

// ── Existing tests (rewritten for index subcommand) ─────────────────────

#[test]
fn test_sample_workspace_output() {
    let fixture = std::path::Path::new("tests/fixtures/sample-workspace");

    let output = Command::new(&binary_path())
        .arg("index")
        .arg(fixture)
        .output()
        .expect("failed to execute binary");

    assert!(
        output.status.success(),
        "binary exited with: {}\nstderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("output is not valid JSON");

    // Top-level structure.
    assert!(!json["workspace"]["root"].as_str().unwrap().is_empty());
    assert!(!json["workspace"]["workspaceName"].as_str().unwrap().is_empty());
    assert!(json["crates"].is_array(), "crates must be an array");

    let crates = json["crates"].as_array().unwrap();

    // Both crates should be present.
    let names: Vec<&str> = crates.iter().map(|c| c["name"].as_str().unwrap()).collect();
    assert!(names.contains(&"core"), "missing core crate");
    assert!(names.contains(&"engine"), "missing engine crate");

    // Find the core crate and verify its modules.
    let core_crate = crates.iter().find(|c| c["name"] == "core").unwrap();
    assert_eq!(core_crate["package"]["crateType"], "lib");
    assert!(!core_crate["modules"].as_array().unwrap().is_empty());

     // core should have a public Task struct.
    let has_task = {
        let items: Vec<_> = core_crate["modules"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|m| extract_array(m, "publicItems"))
            .collect();
        items.iter().any(|item| item["name"] == "Task" && item["kind"] == "struct")
    };
    assert!(has_task, "core should export pub struct Task");

    // Find the engine crate.
    let engine_crate = crates.iter().find(|c| c["name"] == "engine").unwrap();
    assert!(!engine_crate["modules"].as_array().unwrap().is_empty());

    // engine should import from core.
    let engine_imports_core = {
        let imports: Vec<_> = engine_crate["modules"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|m| extract_array(m, "imports"))
            .collect();
        imports.iter().any(|imp| imp["path"].as_str().unwrap().contains("core"))
    };
    assert!(engine_imports_core, "engine should import from core");

    // Cross-references: keys are now canonical paths (module.path::item.name).
    let cross_refs = &json["crossReferences"]["types"];
    // Task is exported from the "core" module, so key is "core::Task".
    let task_ref = cross_refs
        .get("core::Task")
        .expect("core::Task should appear in crossReferences.types (canonical path)");
    assert!(
        task_ref["exportedBy"]
            .as_array()
            .unwrap()
            .contains(&serde_json::Value::String("core".into())),
        "Task should be exported by core"
    );
}

#[test]
fn test_deterministic_output() {
    let fixture = std::path::Path::new("tests/fixtures/sample-workspace");

    let output1 = Command::new(&binary_path())
        .arg("index")
        .arg(fixture)
        .output()
        .expect("failed to execute binary (run 1)");
    assert!(output1.status.success());

    let output2 = Command::new(&binary_path())
        .arg("index")
        .arg(fixture)
        .output()
        .expect("failed to execute binary (run 2)");
    assert!(output2.status.success());

    assert_eq!(
        output1.stdout, output2.stdout,
        "output must be byte-identical across runs"
    );
}

#[test]
fn test_missing_path_exits_nonzero() {
    let bin = binary_path();
    let output = Command::new(&bin)
        .arg("index")
        .arg("/tmp/nonexistent-path-12345")
        .output()
        .expect("failed to execute binary");

    assert!(
        !output.status.success(),
        "should exit non-zero for invalid path"
    );
}

fn run_index(path: &str) -> std::process::Output {
    Command::new(&binary_path())
        .arg("index")
        .arg(path)
        .output()
        .expect("failed to execute binary")
}

fn parse_output(output: &std::process::Output) -> serde_json::Value {
    serde_json::from_str(&String::from_utf8_lossy(&output.stdout)).unwrap()
}

fn write_cargo_toml(dir: &std::path::Path, content: &str) {
    let mut f = std::fs::File::create(dir.join("Cargo.toml")).unwrap();
    use std::io::Write;
    f.write_all(content.as_bytes()).unwrap();
}

fn setup_crate(dir: &std::path::Path, lib_content: &str) {
    let src = dir.join("src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(src.join("lib.rs"), lib_content).unwrap();
    let name = dir.file_name().unwrap().to_string_lossy();
    let cargo = format!(
        "[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    );
    std::fs::write(dir.join("Cargo.toml"), cargo).unwrap();
}

#[test]
fn test_parse_failure_error_entry() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    // Create workspace Cargo.toml
    write_cargo_toml(root, r#"
[workspace]
members = ["good_crate", "bad_crate"]
"#);

    // Good crate with valid Rust
    setup_crate(&root.join("good_crate"), "pub struct Good {}");

    // Bad crate with invalid Rust syntax
    setup_crate(&root.join("bad_crate"), "pub struct { invalid rust syntax");

    let output = run_index(root.to_str().unwrap());
    assert!(output.status.success());

    let json = parse_output(&output);
    let errors: Vec<&serde_json::Value> = extract_array(&json, "errors");

    let parse_errors: Vec<_> = errors.iter()
        .filter(|e| {
            e["kind"].as_str().unwrap() == "syn_parse_error"
        })
        .collect();

    assert!(!parse_errors.is_empty(), "should have parse error entries");
    assert_eq!(parse_errors[0]["severity"].as_str().unwrap(), "error");
}

#[test]
fn test_single_crate_without_workspace() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let crate_dir = root.join("standalone");

    setup_crate(&crate_dir, "pub struct Standalone { pub x: i32 }");

    let output = run_index(crate_dir.to_str().unwrap());
    assert!(output.status.success(), "standalone crate should succeed: {}", String::from_utf8_lossy(&output.stderr));

    let json = parse_output(&output);
    let crates = extract_array(&json, "crates");
    assert_eq!(crates.len(), 1, "should have exactly one crate");

    assert_eq!(crates[0]["name"], "standalone");

    let has_standalone = crates[0]["modules"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|m| m["publicItems"].as_array().unwrap())
        .any(|item| item["name"] == "Standalone" && item["kind"] == "struct");
    assert!(has_standalone, "should find pub struct Standalone");
}

#[test]
fn test_single_crate_module_tree() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("single-crate");
    std::fs::create_dir_all(&root).unwrap();

    setup_crate(&root, "pub mod helpers;");

    // Create depth-1 module
    let helpers_dir = root.join("src").join("helpers");
    std::fs::create_dir_all(&helpers_dir).unwrap();
    std::fs::write(helpers_dir.join("mod.rs"), "pub mod sub;").unwrap();

    // Create depth-2 module with a public item
    std::fs::write(helpers_dir.join("sub.rs"), "pub fn assist() {}").unwrap();

    let output = run_index(root.to_str().unwrap());
    assert!(output.status.success(), "single-crate module tree should succeed: {}", String::from_utf8_lossy(&output.stderr));

    let json = parse_output(&output);
    let crates = extract_array(&json, "crates");
    assert_eq!(crates.len(), 1);

    let crate_info = &crates[0];
    assert_eq!(crate_info["name"], "single-crate");

    let modules = extract_array(crate_info, "modules");
    let module_paths: Vec<&str> = modules
        .iter()
        .map(|m| m["path"].as_str().unwrap())
        .collect();

    assert!(
        module_paths.iter().any(|p| *p == "single-crate"),
        "should have root module"
    );
    assert!(
        module_paths.iter().any(|p| *p == "single-crate::helpers"),
        "should have depth-1 module"
    );
    assert!(
        module_paths.iter().any(|p| *p == "single-crate::helpers::sub"),
        "should have depth-2 module"
    );

    // Verify the depth-2 module has the `assist` function
    let sub_module = modules
        .iter()
        .find(|m| m["path"].as_str().unwrap() == "single-crate::helpers::sub")
        .unwrap();
    let has_assist = sub_module["publicItems"]
        .as_array()
        .unwrap()
        .iter()
        .any(|item| item["name"] == "assist" && item["kind"] == "fn");
    assert!(has_assist, "helpers::sub should have pub fn assist");
}

#[test]
fn test_glob_member_patterns() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    write_cargo_toml(root, r#"
[workspace]
members = ["crates/*"]
"#);

    for name in &["alpha", "beta", "gamma"] {
        setup_crate(&root.join("crates").join(name), format!("pub struct {name} {{}}").as_str());
    }

    let output = run_index(root.to_str().unwrap());
    assert!(output.status.success());

    let json = parse_output(&output);
    let crates = extract_array(&json, "crates");
    let names: Vec<&str> = crates.iter()
        .map(|c| c["name"].as_str().unwrap())
        .collect();

    assert!(names.contains(&"alpha"));
    assert!(names.contains(&"beta"));
    assert!(names.contains(&"gamma"));
    assert_eq!(names.len(), 3);
}

#[test]
fn test_workspace_with_exclude() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    write_cargo_toml(root, r#"
[workspace]
members = ["a", "b", "c"]
exclude = ["b"]
"#);

    setup_crate(&root.join("a"), "pub struct A {}");
    setup_crate(&root.join("b"), "pub struct B {}");
    setup_crate(&root.join("c"), "pub struct C {}");

    let output = run_index(root.to_str().unwrap());
    assert!(output.status.success());

    let json = parse_output(&output);
    let crates = extract_array(&json, "crates");
    let names: Vec<&str> = crates.iter()
        .map(|c| c["name"].as_str().unwrap())
        .collect();

    assert!(names.contains(&"a"));
    assert!(!names.contains(&"b"));
    assert!(names.contains(&"c"));
}

#[test]
fn test_deeply_nested_modules() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    write_cargo_toml(root, r#"
[workspace]
members = ["."]

[package]
name = "nested"
version = "0.1.0"
edition = "2021"
"#);

    let src = root.join("src");
    let foo = src.join("foo");
    let bar = foo.join("bar");
    std::fs::create_dir_all(&bar).unwrap();

    // lib.rs declares mod foo (resolves to src/foo/mod.rs)
    std::fs::write(src.join("lib.rs"), "mod foo;").unwrap();
    // foo/mod.rs declares mod bar
    std::fs::write(foo.join("mod.rs"), "mod bar;").unwrap();
    // bar/mod.rs declares mod baz
    std::fs::write(bar.join("mod.rs"), "mod baz;").unwrap();
    // bar/baz.rs with a struct
    std::fs::write(bar.join("baz.rs"), "pub struct Deep {}").unwrap();

    let output = run_index(root.to_str().unwrap());
    assert!(output.status.success());

    let json = parse_output(&output);
    let crates = extract_array(&json, "crates");
    let nested_crate = crates.iter().find(|c| c["name"].as_str().unwrap() == "nested").unwrap();

    let modules = extract_array(&nested_crate, "modules");
    let module_paths: Vec<&str> = modules
        .iter()
        .map(|m| m["path"].as_str().unwrap())
        .collect();

    assert!(module_paths.iter().any(|p| *p == "nested"));
    assert!(module_paths.iter().any(|p| *p == "nested::foo"));
    assert!(module_paths.iter().any(|p| *p == "nested::foo::bar"));
    assert!(module_paths.iter().any(|p| *p == "nested::foo::bar::baz"));
}

#[test]
fn test_reexport_chains() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    write_cargo_toml(root, r#"
[workspace]
members = ["."]

[package]
name = "reexporter"
version = "0.1.0"
edition = "2021"
"#);

    let src = root.join("src");
    std::fs::create_dir_all(&src).unwrap();

    // lib.rs with re-export chain
    std::fs::write(src.join("lib.rs"), "
mod inner {
    pub struct Secret;
}
pub use inner::Secret;
").unwrap();

    let output = run_index(root.to_str().unwrap());
    assert!(output.status.success());

    let json = parse_output(&output);
    let crates = extract_array(&json, "crates");
    let reexporter = crates.iter().find(|c| c["name"].as_str().unwrap() == "reexporter").unwrap();

    let re_exports: Vec<&serde_json::Value> = extract_array(&reexporter, "modules")
        .iter()
        .flat_map(|m| extract_array(m, "reExports"))
        .collect();

    let has_secret = re_exports.iter().any(|re| {
        re["importPath"].as_str().unwrap().contains("Secret")
    });
    assert!(has_secret, "should have re-export for Secret");
}

#[test]
fn test_output_via_flag() {
    let tmp = tempfile::tempdir().unwrap();
    let fixture = std::path::Path::new("tests/fixtures/sample-workspace");
    let output_path = tmp.path().join("output.json");

    // Run with -o flag
    let output1 = Command::new(&binary_path())
        .arg("index")
        .arg(fixture)
        .arg("-o")
        .arg(output_path.clone())
        .output()
        .expect("failed to execute binary");
    assert!(output1.status.success());

    // Run without -o, capture stdout
    let output2 = Command::new(&binary_path())
        .arg("index")
        .arg(fixture)
        .output()
        .expect("failed to execute binary");
    assert!(output2.status.success());

    // Compare file content with stdout
    let file_content = std::fs::read_to_string(&output_path).unwrap();
    let stdout_content = String::from_utf8_lossy(&output2.stdout);
    assert_eq!(
        file_content.trim(),
        stdout_content.trim(),
        "file output should match stdout"
    );
}

// ── New validation tests ────────────────────────────────────────────────

#[test]
fn test_validate_orphan_file_exits_2() {
    let fixture = std::path::Path::new("tests/fixtures/bad-orphan");

    let output = Command::new(&binary_path())
        .arg("index")
        .arg("--validate")
        .arg(fixture)
        .output()
        .expect("failed to execute binary");

    assert_eq!(
        output.status.code().unwrap(),
        2,
        "validation should exit 2 for orphan files"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("output is not valid JSON");

    let errors = extract_array(&json, "errors");
    assert!(!errors.is_empty(), "should have validation errors");

    let orphan_error = errors.iter().find(|e| e["kind"].as_str().unwrap() == "orphan_file")
        .expect("should have orphan_file error");

    assert_eq!(orphan_error["severity"].as_str().unwrap(), "warning");
    let message = orphan_error["message"].as_str().unwrap();
    assert!(message.contains("forgotten"), "message should mention the orphan file");
    assert!(message.contains("pub mod"), "message should suggest fix hint");
}

#[test]
fn test_validate_dead_reexport_exits_2() {
    let fixture = std::path::Path::new("tests/fixtures/bad-dead-reexport");

    let output = Command::new(&binary_path())
        .arg("index")
        .arg("--validate")
        .arg(fixture)
        .output()
        .expect("failed to execute binary");

    assert_eq!(
        output.status.code().unwrap(),
        2,
        "validation should exit 2 for dead re-exports"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("output is not valid JSON");

    let errors = extract_array(&json, "errors");
    let dead_reexport = errors.iter().find(|e| e["kind"].as_str().unwrap() == "dead_re_export")
        .expect("should have dead_re_export error");

    assert_eq!(dead_reexport["severity"].as_str().unwrap(), "warning");
}

#[test]
fn test_recursive_orphan_detection() {
    let fixture = std::path::Path::new("tests/fixtures/bad-orphan");

    let output = Command::new(&binary_path())
        .arg("index")
        .arg("--validate")
        .arg(fixture)
        .output()
        .expect("failed to execute binary");

    assert_eq!(
        output.status.code().unwrap(),
        2,
        "validation should exit 2 for orphan files"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("output is not valid JSON");

    let errors = extract_array(&json, "errors");

    let orphan_errors: Vec<_> = errors
        .iter()
        .filter(|e| e["kind"].as_str().unwrap() == "orphan_file")
        .collect();

    assert!(
        !orphan_errors.is_empty(),
        "should have orphan_file errors"
    );

    // The depth-2 orphan at sub/deep/orphan.rs must be detected
    let has_deep_orphan = orphan_errors.iter().any(|e| {
        e["file"].as_str().unwrap().contains("deep/orphan.rs")
    });
    assert!(
        has_deep_orphan,
        "should detect the deeply nested orphan at sub/deep/orphan.rs"
    );

    // The shallow orphan forgotten.rs must still be detected
    let has_forgotten = orphan_errors.iter().any(|e| {
        e["file"].as_str().unwrap().contains("forgotten.rs")
    });
    assert!(
        has_forgotten,
        "should still detect the shallow orphan forgotten.rs"
    );
}

#[test]
fn test_index_no_validate_exits_0() {
    let fixture = std::path::Path::new("tests/fixtures/bad-orphan");

    let output = Command::new(&binary_path())
        .arg("index")
        .arg(fixture)
        .output()
        .expect("failed to execute binary");

    assert_eq!(
        output.status.code().unwrap(),
        0,
        "without --validate, should exit 0"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("output is not valid JSON");

    let errors = extract_array(&json, "errors");
    let has_orphan = errors.iter().any(|e| {
        e["kind"].as_str().unwrap() == "orphan_file"
    });
    assert!(!has_orphan, "without --validate, should not have OrphanFile/DeadReExport errors");
}

// ── New lookup tests ────────────────────────────────────────────────────

#[test]
fn test_lookup_symbol_found() {
    let fixture = std::path::Path::new("tests/fixtures/sample-workspace");

    let output = Command::new(&binary_path())
        .arg("lookup")
        .arg(fixture)
        .arg("--symbol")
        .arg("Task")
        .output()
        .expect("failed to execute binary");

    assert!(output.status.success(), "should find Task symbol");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("output is not valid JSON");

    assert_eq!(json["status"].as_str().unwrap(), "found");
    assert!(json["crateName"].as_str().unwrap().is_empty() == false);
    assert_eq!(json["kind"].as_str().unwrap(), "struct");
}

#[test]
fn test_lookup_symbol_not_found() {
    let fixture = std::path::Path::new("tests/fixtures/sample-workspace");

    let output = Command::new(&binary_path())
        .arg("lookup")
        .arg(fixture)
        .arg("--symbol")
        .arg("DoesNotExist")
        .output()
        .expect("failed to execute binary");

    assert_eq!(
        output.status.code().unwrap(),
        1,
        "should exit 1 for not found symbol"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("output is not valid JSON");

    assert_eq!(json["status"].as_str().unwrap(), "not_found");
}

#[test]
fn test_lookup_file() {
    let fixture = std::path::Path::new("tests/fixtures/sample-workspace");

    let output = Command::new(&binary_path())
        .arg("lookup")
        .arg(fixture)
        .arg("--file")
        .arg("core/src/lib.rs")
        .output()
        .expect("failed to execute binary");

    assert!(output.status.success(), "should find core/src/lib.rs");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("output is not valid JSON");

    assert!(json.get("fileEntry").is_some(), "should have fileEntry");
    assert!(json.get("primaryModule").is_some(), "should have primaryModule");
}

// ── Serde regression test ───────────────────────────────────────────────

#[test]
fn test_orphaned_module_serde_regression() {
    // Verify DiagnosticKind::OrphanedModule serializes as "orphaned_module".
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    write_cargo_toml(root, r#"
[workspace]
members = ["."]

[package]
name = "orphan-mod-test"
version = "0.1.0"
edition = "2021"
"#);

    let src = root.join("src");
    std::fs::create_dir_all(&src).unwrap();

    // lib.rs with an undeclared module
    std::fs::write(src.join("lib.rs"), "mod nonexistent;").unwrap();

    let output = run_index(root.to_str().unwrap());
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("output is not valid JSON");

    let errors = extract_array(&json, "errors");
    let orphaned = errors.iter().find(|e| e["kind"].as_str().unwrap() == "orphaned_module")
        .expect("should have orphaned_module error (snake_case string)");

    assert_eq!(orphaned["severity"].as_str().unwrap(), "warning");
}
