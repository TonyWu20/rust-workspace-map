# Task Checklist Review: mvp-fp-cleanup

## Overview

This document reviews the draft `directions.json` against the actual codebase state (validated against source files at `src/validate.rs`, `src/schema.rs`, `src/indexes.rs`, `tests/integration_test.rs`, `tests/fixtures/bad-orphan/src/lib.rs`, and `README.md`).

## Task-by-Task Review

### TASK-G1: Prefix-aware external-crate pre-check

**Is the goal clear?** YES. Skip re-exports targeting external crates (e.g., `serde::Serialize`) by adding `is_external_crate_re_export` and wiring it into `check_dead_reexports`.

**Are `files_in_scope` correct?** YES. Only `src/validate.rs` — all changes are additions to that single file.

**Are the line number references accurate?** YES. Verified against source: the glob-reexport skip is at lines 139-141, `resolve_import_path` is at line 145, and the existing cross-crate check is at lines 147-151. All match the guidance.

**Is implementation detail sufficient?** YES, with one minor wording concern:

- The three-case dispatch logic for `is_external_crate_re_export` is clearly described.
- The known pitfalls document a subtle edge case (bare path whose first segment matches both a workspace member name AND a top-level module of the current crate) and the guidance correctly addresses it.
- **Wording concern (non-blocking):** The known pitfall says "the existing check handles cross-workspace-member re-exports that pass G1." The phrase "pass G1" is ambiguous — it could mean "G1 returns true for" or "the path passes through G1 without being skipped." The actual implementation guidance makes clear that G1 should return `true` (skip) for cross-workspace-member paths that are NOT also top-level modules. The existing check only handles cross-workspace-member paths that use `crate::` or other prefixed forms, not bare paths. This is correctly reflected in the three-case logic but the pitfall wording could cause a moment of confusion.

**Is the TDD test code meaningful?** YES. The test covers four scenarios:
1. `serde::Serialize` (bare path, first segment is neither workspace member nor top-level module) returns true
2. `utils::Helper` (bare path, first segment IS a top-level module) returns false
3. `crate::sub::missing` (`crate::` prefix) returns false
4. `self::inner::Type` (`self::` prefix) returns false

Each assertion is concrete and falsifiable. The `signature` matches the function called in `test_code`. The guidance explicitly notes additional edge-case tests to add during refactor (`super::`, single-segment, cross-workspace-member, empty string). This is a valid specification.

**Are `wiring_checklist` items correct?** YES. The single entry correctly identifies the call site (in the per-re-export loop, after glob skip, before `resolve_import_path`).

**Are `acceptance` commands sufficient?** YES. Three commands: unit test, full test suite, clippy clean.

**Verdict: CLEAR**

---

### TASK-G3: Fix `determine_parent_file` to use `crate_info.root`

**Is the goal clear?** YES. Replace the hardcoded `"src/"` prefix with a prefix derived from `crate_info.root` so that non-root workspace members (e.g., `crates/mylib/`) get correct parent-file paths.

**Are `files_in_scope` correct?** YES. Only `src/validate.rs`, specifically the `determine_parent_file` function.

**Are the line number references accurate?** YES. `determine_parent_file` is at lines 104-124 of the actual file (the guidance says "lines 105-124" — off by one on the opening brace, but functionally correct). The key target is `orphan_file.strip_prefix("src/")` at line 111.

**Is implementation detail sufficient?** YES. The guidance specifies:
- Remove underscore prefix from `_crate_info` parameter
- Use `Path::new(&crate_info.root).parent()` to get the `src/` directory
- Format the prefix string with `{}/`
- Keep rest of the function unchanged
- The known pitfall correctly notes: `crate_info.root` is a file path (e.g., `"src/lib.rs"`), not a directory — use `.parent()`, not `.join("src")`

**Are acceptance commands sufficient?** YES. Full test suite, integration test, clippy clean.

**Verdict: CLEAR**

---

### TASK-G4: Add deeply nested orphan fixture and integration test

**Is the goal clear?** YES. Add `sub/` directory with a `deep/orphan.rs` at depth 2, plus an integration test verifying walkdir discovers it.

**Are `files_in_scope` correct?** YES. All six files listed are verified to exist (or will be created by this task):
- `tests/fixtures/bad-orphan/src/lib.rs` — currently has `// root module comment`; needs `pub mod sub;` added
- `tests/fixtures/bad-orphan/src/sub/mod.rs` — does not exist yet (to be created)
- `tests/fixtures/bad-orphan/src/sub/legal.rs` — does not exist yet (to be created)
- `tests/fixtures/bad-orphan/src/sub/deep/orphan.rs` — does not exist yet (to be created)
- `tests/integration_test.rs` — exists, needs new test function

**Are the fixture file contents sufficiently specified?** YES. The guidance gives the exact content for each new file.

**Is implementation detail for the integration test sufficient?** YES. The test logic is described step by step: run binary with `--validate`, assert exit 2, parse JSON, filter for `orphan_file` kind, assert `deep/orphan.rs` is found, assert `forgotten.rs` is still found. The guidance also says to follow existing patterns (`binary_path()`, `extract_array`, `String::from_utf8_lossy`).

**One concern:** The acceptance criteria list `cargo test -p rust-workspace-map --test integration_test test_validate_orphan_file_exits_2` which verifies that the existing test still passes after modifying `bad-orphan/src/lib.rs`. The fixture change (adding `pub mod sub;`) does NOT affect the `forgotten.rs` orphan — that file remains undeclared. The existing test asserts an `orphan_file` error mentioning `forgotten`, so this should still pass. This concern is noted but is not a blocker.

**Are `wiring_checklist` items correct?** YES. Two entries: the `pub mod sub;` declaration in `lib.rs` and the `pub mod legal;` declaration in `sub/mod.rs` (with the explicit note that `deep` must NOT be declared).

**Verdict: CLEAR**

---

### TASK-G2: `derive_attrs` field and `is_derive_companion` heuristic

**Is the goal clear?** YES. Suppress DeadReExport false positives for derive-generated companion types (e.g., `CellDocumentBuilder` from `#[derive(bon::Builder)]` on `CellDocument`).

**Are `files_in_scope` correct?** YES. Three files: `src/schema.rs` (add field), `src/indexes.rs` (populate field), `src/validate.rs` (implement and wire heuristic).

**Is implementation detail sufficient?** YES. The guidance is very thorough:
1. Schema change: exact struct field, serde annotation, builder default — all specified
2. Index change: one-liner `.derive_attrs(item.attrs.derive.clone())` at the fold closure
3. The `is_derive_companion` algorithm is fully specified: longest-prefix-first iteration, key construction (`format!("{}::{}", module_path, P)`), empty `derive_attrs` skips to shorter prefix
4. Wiring into `check_dead_reexports`: inside the `!symbols.contains_key(&canonical)` branch, before `findings.push()`
5. The known pitfall correctly warns against re-extracting from AST

**One design property to verify:** The algorithm iterates prefix lengths from `len(target_name)` down to 1. For `"PlainStructBuilder"` (18 chars) against a base type `"PlainStruct"` (11 chars) with empty `derive_attrs`:
- Prefix length 11 gives key `"mycrate::sub::PlainStruct"` which is FOUND but has empty `derive_attrs` → continues to shorter prefixes
- No shorter prefix matches any symbol → returns false (correctly NOT suppressed)

For `"CellDocumentBuilder"` (19 chars) against base `"CellDocument"` (12 chars) with non-empty `derive_attrs`:
- Prefix length 12 gives key `"mycrate::sub::CellDocument"` which is FOUND with non-empty `derive_attrs` → returns true (correctly suppressed)

The test covers both of these cases plus cross-module scoping. The algorithm is sound.

**Is the TDD test code meaningful?** YES. The test covers four scenarios:
1. `CellDocumentBuilder` with base `CellDocument` having derive_attrs → suppressed (true)
2. `PlainStructBuilder` with base `PlainStruct` having empty derive_attrs → NOT suppressed (false)
3. `TotallyMissing` with no prefix matching any symbol → NOT suppressed (false)
4. `CellDocumentBuilder` in different module (`mycrate::other` vs `mycrate::sub`) → NOT suppressed (false)

Each assertion is concrete and falsifiable. The `signature` matches the function called in `test_code`. The test uses `derive_attrs(...)` directly in the builder which requires the schema change from step 0, correctly sequenced.

**Are `wiring_checklist` items correct?** YES. Three entries:
1. Schema field addition
2. Index construction one-liner
3. Function call in `check_dead_reexports`

**Are `acceptance` commands sufficient?** YES. `cargo check`, unit test, full test suite, integration test, clippy.

**Are `depends_on` correct?** YES. `depends_on: ["TASK-G1"]` — G2 modifies the same function body as G1, so sequential execution is required.

**Verdict: CLEAR**

---

### TASK-README: Document prefix-decomposition heuristic

**Is the goal clear?** YES. Add documentation about the G2 design rationale and the private-base-type limitation.

**Are `files_in_scope` correct?** YES. Only `README.md`.

**Is implementation detail sufficient?** YES. The guidance specifies two content blocks:
1. Prefix-decomposition heuristic explanation
2. Private-base-type known limitation

**Is there a way to verify the step is done correctly?** PARTIALLY. The acceptance command `cargo check -p rust-workspace-map` verifies the project still compiles — but `README.md` is a markdown file and is not checked by `cargo check`. There is no automated verification of the README content. The task can only be verified by manual inspection.

This is a minor concern for a documentation task. The guidance is sufficiently detailed that an implementer with clear instructions would produce the right content. The acceptance criteria should ideally include "Manually review the updated README" or a diff check.

**Verdict: CLEAR (with minor verification concern)**

---

## Group Structure Review

### group-g1-g3-g4 (TASK-G1, TASK-G3, TASK-G4)

**Claim:** These tasks touch different physical locations with zero overlap.

**Verified:**
- G1 modifies `check_dead_reexports` (lines 126-174) and adds a function before it
- G3 modifies `determine_parent_file` (lines 104-124)
- G4 creates fixture files and adds an integration test in `tests/integration_test.rs`

G1 and G3 are in the same file (`src/validate.rs`) but in separate, non-overlapping functions. G1 adds its new function AND modifies `check_dead_reexports`, while G3 modifies only `determine_parent_file`. These are well-separated — there is a blank line between `determine_parent_file` (ends at line 124) and the next function doc comment (line 126). An agent applying both changes simultaneously would not conflict.

G4 is in completely different files.

**Verdict: CORRECT.** These can be implemented independently.

### group-g2-docs (TASK-G2, TASK-README)

**Claim:** G2 modifies the same function body as G1, so sequential dependency is required.

**Verified:** G2 adds logic inside the `!symbols.contains_key(&canonical)` branch of `check_dead_reexports` (after line 157). G1 adds logic at the top of the re-export loop (between lines 141 and 145). These are adjacent but non-overlapping regions of the same function. However, the known pitfalls correctly note they should be implemented sequentially to avoid merge conflicts.

`depends_on_groups: ["group-g1-g3-g4"]` is correct — G2 must be implemented after all tasks in the first group.

**Verdict: CORRECT.**

---

## Known Pitfalls Verification

All known pitfalls were checked against the actual codebase:

1. **G1: `crate_names` alone insufficient** — Confirmed. The guidance correctly adds a `crate_info.modules` check for top-level modules.

2. **G1: G1 and existing check coexist** — Confirmed. G1 operates on unresolved paths; existing check on resolved paths. G1 skips early for external crates; existing check handles cross-workspace-member paths that use `crate::` prefix.

3. **G2: `derive_attrs` is a straight copy** — Confirmed. `item.attrs.derive` already exists as `Vec<String>` in `PublicItem`. The guidance correctly says to clone it once.

4. **G2: Same-module scoping** — Confirmed. The algorithm uses `module_path` in the lookup key, not a global search.

5. **G2: Lookup key format** — Confirmed. `format!("{}::{}", module_path, prefix)` matches how keys are constructed in indexes.rs.

6. **G2: Longest prefix first** — Confirmed. Iteration starts from `len(target_name)` down to 1.

7. **G2: Private-base-type limitation** — Confirmed. The guidance explicitly documents this as an accepted limitation.

8. **G3: `crate_info.root` is a file** — Confirmed. Root values are like `"src/lib.rs"`. Using `.parent()` is correct.

9. **G3: Orphan file is workspace-relative** — Confirmed. The `strip_prefix` operation in `check_orphan_files` produces workspace-relative paths.

10. **G4: Preserve existing orphan behavior** — Confirmed. Adding `pub mod sub;` does NOT affect the `forgotten.rs` orphan.

11. **G4: `sub/deep/orphan.rs` must NOT be declared** — Confirmed. The guidance explicitly states this.

12. **Serde: `serialize_with` annotation** — Confirmed. The field will serialize as `deriveAttrs` due to `#[serde(rename_all = "camelCase")]` on `SymbolEntry`.

13. **Clippy: No `#[allow(...)]`** — Confirmed. No allow annotations are suggested.

14. **Determinism: No HashMap iteration order** — Confirmed. The test uses `BTreeMap` and the codebase uses `BTreeMap` throughout.

---

## Final Assessment

**Overall verdict: Ready to Implement**

All four implementation tasks (G1, G2, G3, G4) are well-specified with clear goals, correct target files, sufficient implementation detail, and meaningful verification criteria. The documentation task (G1-README) is also well-specified but lacks a meaningful automated acceptance check (cargo check does not verify README content).

### Minor issues flagged (none are blockers):

1. **TASK-G1 (wording clarity):** The known pitfall "the existing check handles cross-workspace-member re-exports that pass G1" uses "pass G1" ambiguously — it could mean "G1 returns true" vs "the path falls through G1 without being skipped." The actual implementation guidance is unambiguous, so this is a documentation style issue only.

2. **TASK-README (acceptance criteria):** The `cargo check` acceptance command does not verify the README content. Consider adding "Manually verify the updated README content" as an additional step. This is a minor concern for a documentation task.

### TDD test quality:

- **TASK-G1 (lib-tdd):** The `test_code` is specific, falsifiable, and covers the core scenarios. The `signature` matches the function called. **PASS.**
- **TASK-G2 (lib-tdd):** The `test_code` is specific, falsifiable, and covers four key scenarios including cross-module scoping. The `signature` matches the function called. **PASS.**
