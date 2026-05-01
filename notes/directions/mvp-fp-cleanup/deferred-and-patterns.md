# Synthesized Context: mvp-fp-cleanup

> Generated: 2026-04-30 | Source files: PHASE_PLAN.md, decisions.md, notes/architecture-current.md, deferred.md (mvp, phase-0.1, phase-0.2)

---

## 1. Key Architectural Patterns from the Plan

### G1 — Prefix-Aware External-Crate Check

The core insight is that the **original (unresolved) path** is inspected before the `crate::`/`self::`/`super::` resolution step. The first segment of the import path determines whether the target is external. Three cases:

| Path prefix | Action |
|---|---|
| Bare (`serde::Serialize`) | Check first segment against `crate_names` + top-level modules of current crate. If neither, skip (external). |
| `crate::` | Strip prefix, check next segment against crate modules only (always internal — never skip). |
| `self::` / `super::` | Always internal — skip the external-crate check entirely. |

**Where:** `validate.rs::check_dead_reexports` — new pre-check before `resolve_import_path`.

**Amendment A** (from review): The original plan had a "check original unresolved path" that was insufficiently specified. The prefix-aware logic above is the refined version.

### G2 — Prefix-Decomposition Heuristic (Derive-Companion Awareness)

A purely structural heuristic — **zero hardcoded derive names, zero suffix lists**:

1. DeadReExport fires on `crate::module::CellDocumentBuilder` (not found in symbols).
2. Iterate decreasing prefix lengths of the name (`CellDocumentBuilder`, `CellDocument`, `CellDocumentBuil`, ...).
3. For each prefix, build lookup key as `"{module.path}::{prefix}"` (e.g., `"mycrate::sub::CellDocument"`).
4. **First match in symbols:** if `symbol.derive_attrs` is non-empty, suppress the DeadReExport.
5. If no prefix matches, the finding stands.

**Data flow changes:**
- `src/indexes.rs`: Copy `item.attrs.derive` (already `Vec<String>`) into `SymbolEntry.derive_attrs`.
- `src/schema.rs`: Add `derive_attrs: Vec<String>` with `#[builder(default)]` and `#[serde(skip_serializing_if = "Vec::is_empty")]`.
- `src/validate.rs`: Prefix-decomposition logic in the DeadReExport handler.

**Same-module scoping:** Prefix lookup is constrained to the re-export's own module — prevents cross-module false matches.

**Conservative-recall policy:** False negatives (suppressing a genuinely-dead re-export where the base type coincidentally has derives) are preferred over false positives. The genuinely-dead-with-derives-on-base case is vanishingly rare.

**Known limitation (documented):** Private structs with `#[derive(bon::Builder)]` generating a public builder — the base type is absent from the public-only symbol index, so prefix-decomposition cannot find it. Accepted edge case.

### G3 — `determine_parent_file` Path Fix

Current (broken): `strip_prefix("src/")` — fails for members not at workspace root.
Fixed: Derive crate-relative `src/` prefix from `crate_info.root`:
```rust
let crate_src = crate_info.root.join("src");
let parent = orphan_path.strip_prefix(crate_src.parent()?)?;
let parent_file = parent.parent()?.join("mod.rs");
```

The `_crate_info` parameter is already passed but was unused — this fix just uses it.

### G4 — Recursive Orphan Integration Test

Extend `tests/fixtures/bad-orphan/` with a deeply nested orphan (e.g., `src/sub/deep/orphan.rs`). Add `test_recursive_orphan_detection` to `tests/integration_test.rs`.

### Execution Sequence

```
G1 + G3 + G4   (parallel — independent, touch different functions/files)
      │
      └──→ G2   (sequential — touches check_dead_reexports after G1 lands, avoids merge conflicts)
```

### Verification Criteria

1. G1: `pub use serde::Serialize;` — no DeadReExport finding. `pub use missing::Type;` with unknown first segment — also skipped (correct).
2. G2: `index --validate` on `castep-cell-io` — derive-generated builder types absent from DeadReExport; genuinely-dead re-exports still present.
3. G3: Multi-crate workspace with non-root member — fix hint names correct parent file.
4. G4: `test_recursive_orphan_detection` passes.
5. Regression: `cargo test -p rust-workspace-map` and `cargo test -p rust-workspace-map --test integration_test` green.
6. Clippy: `cargo clippy -p rust-workspace-map -- -D warnings` clean.

---

## 2. Known Failure Modes to Avoid

### Spec Gaps Caught in Review

1. **G1 must be prefix-aware.** The original plan specified "check original unresolved path" without considering `crate::`/`self::`/`super::` prefixes. A `crate::foo::Type` path with `foo` not in `crate_names` or modules would be incorrectly skipped. Amendment A fixed this with the three-case dispatch.

2. **G2 private-base-type limitation.** Document this in README. The public-only symbol index cannot contain private structs, so derive-generated companions from private bases will never be suppressed by prefix-decomposition. This is an accepted false-negative edge case.

### Repeated Deferred Items (Resurface Risk)

These items have been deferred across multiple review rounds and may need attention soon:

3. **Empty errors vector in `run()`** (deferred phase-0.1, deferred again in this review). The `WorkspaceMap.errors` field is never populated. If any new validate rules produce errors, the wiring must be verified. Currently only partially populated.

4. **Path fallback to "." in `module_tree.rs`** (deferred phase-0.1, deferred again). Three `.unwrap_or_else(|| Path::new("."))` calls. Practically unreachable for real Rust projects, but masks bugs during development.

5. **No unit tests for public API functions** (deferred phase-0.1, deferred again). Broad coverage is scope creep for this phase, but G1 and G2 MUST receive targeted unit tests (Amendment B requirement).

### Non-functional Risks

6. **`determine_parent_file` hardcoded `"src/"` prefix.** The motivating bug for G3 — fix hint is actively misleading for non-root workspace members. This is now in-scope as G3.

7. **`pub mod` suggestion is never context-aware.** The fix hint always says `pub mod {stem};` regardless of crate type. Binary crates and private internal modules should use `mod {stem};`. Deferred (cosmetic UX), but easy to get wrong if touched incidentally.

8. **`src/tests/` pruning may hide false negatives.** The `filter_entry` predicate prunes all `tests/` directories. `src/tests/` can be a legitimate module directory; files inside a declared `src/tests/` subtree that are themselves undeclared would not be detected. Deferred.

### Structural Invariants to Preserve

9. **Deterministic output.** All collections in output must be sorted (crates by name, public items by name+line, imports by path, re-exports by export_path, submodules by name). New fields in `SymbolEntry` must not break this.

10. **`skip_serializing_if = "Vec::is_empty"`.** The new `derive_attrs` field must use this (already specified in plan, but enforce during implementation).

11. **Paths are workspace-relative strings.** `relativize_path` strips the workspace root prefix. G3's path fix must ensure the output remains workspace-relative.

---

## 3. Deferred Improvements That Should Be Incorporated

### Absorbed into This Phase (from deferred tracking)

These items were previously flagged as deferred but have been absorbed into the current plan:

| Item | Source | Status |
|---|---|---|
| Fix `determine_parent_file` path calc | mvp/deferred.md #1 | Absorbed as G3 |
| Integration test for recursive orphan | mvp/deferred.md #2 | Absorbed as G4 |
| Silent error swallowing in `build_module_tree` | phase-0.1/deferred.md | Already resolved (returns `(Vec<ModuleInfo>, Vec<ErrorEntry>)`) |
| Eliminate `errors.clone()` in `process_module_info` | phase-0.2/deferred.md | Already resolved (no `errors.clone()` exists; moves via `extend`) |
| Path fallback to "" in workspace.rs | phase-0.1/deferred.md | Already resolved (`enumerate_members` returns `Result`; empty members is correct) |

### Still Deferred (Monitor for Future Phases)

| Item | Source | Trigger to Revisit |
|---|---|---|
| Empty errors vector in `run()` | phase-0.1, decisions.md #1 | When adding new validate rules that produce errors |
| Path fallback to "." in `module_tree.rs` | phase-0.1, decisions.md #3 | When adding support for unusual filesystem layouts |
| No unit tests for public API functions | phase-0.1, decisions.md #5 | Dedicated testing phase |
| `pub mod` suggestion context-awareness | mvp/deferred.md #3 | UX polish phase |
| Document `src/tests/` pruning trade-off | mvp/deferred.md #4 | When users report false negatives in `src/tests/` subtrees |

### Wiring Checklist (from directions decomposition patterns)

Based on patterns observed in prior directions, the implementation should be decomposed by SRP:

**Group 1 (G1 + G3 + G4, parallel):**
- `validate.rs` (G1): Bare-path external check, `crate::` prefix check, `self::`/`super::` skip. 15-20 lines + unit tests.
- `validate.rs` (G3): Replace `strip_prefix("src/")` with `crate_info.root`-derived prefix. ~5 lines.
- `tests/fixtures/bad-orphan/` (G4): Extend with `src/sub/deep/orphan.rs`.
- `tests/integration_test.rs` (G4): Add `test_recursive_orphan_detection`.

**Group 2 (G2, sequential after Group 1):**
- `src/schema.rs`: Add `derive_attrs: Vec<String>` to `SymbolEntry`.
- `src/indexes.rs`: Copy `item.attrs.derive` into `SymbolEntry.derive_attrs`.
- `src/validate.rs`: Prefix-decomposition + base-type derive check. Unit tests.
- `README.md`: Document prefix-decomposition design rationale.

### Correctness Constraints

- `symbol.derive_attrs` is a straight copy of `item.attrs.derive` (already `Vec<String>`), NOT a re-extraction from the AST.
- Lookup key for prefix decomposition: `CanonicalPath("{module.path}::{prefix}")` — uses `module.path` of the re-export, not the base type's module.
- G2 suppression occurs AFTER G1 bypass (G1 may skip the symbol entirely if external).
