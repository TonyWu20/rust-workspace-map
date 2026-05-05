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
