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
