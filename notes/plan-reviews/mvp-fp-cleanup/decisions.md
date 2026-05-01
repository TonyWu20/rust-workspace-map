## Plan Review Decisions — mvp-fp-cleanup — 2026-04-30

### Design Assessment

The plan is architecturally sound. It correctly identifies two root causes of DeadReExport false positives and proposes a prefix-decomposition heuristic that is elegant in its simplicity — zero hardcoded derive names, zero suffix lists, purely structural signal. No trait coherence issues, no lifetime problems, no crate boundary violations. Two specification gaps required refinement: (1) the G1 external-crate check must be prefix-aware to avoid incorrectly skipping `crate::`-prefixed paths, and (2) the G2 private-base-type limitation (public-only symbol index can't find private structs with derive-generated public companions) must be documented. Six amendments applied — all specification clarifications, not design changes.

### Deferred Item Decisions

#### 1. Empty errors vector in run() (phase-0.1)
**Decision:** Defer again
**Rationale:** Errors vec is now partially populated; remaining wiring is orthogonal to false-positive cleanup.
**Action:** Revisit when adding new validate rules that produce errors.

#### 2. Silent error swallowing in module tree construction (phase-0.1)
**Decision:** Close
**Rationale:** Already resolved — `build_module_tree` returns `(Vec<ModuleInfo>, Vec<ErrorEntry>)` and `lib.rs` extends errors into `collected_errors`.
**Action:** Remove from deferred tracking.

#### 3. Path fallback to "." in module_tree.rs (phase-0.1)
**Decision:** Defer again
**Rationale:** Defensive `.unwrap_or_else(|| Path::new("."))` never triggered for valid crate roots; no correctness impact.
**Action:** Revisit when adding support for unusual filesystem layouts.

#### 4. Path fallback to "" in workspace.rs (phase-0.1)
**Decision:** Close
**Rationale:** `enumerate_members` now returns `Result`; the `unwrap_or_default()` on `members` only fires when `[workspace]` has no `members` key — returning empty is correct behavior.
**Action:** Remove from deferred tracking.

#### 5. No unit tests for public API functions (phase-0.1)
**Decision:** Defer again
**Rationale:** Broad coverage work is scope creep for this phase. However, new G1/G2 logic MUST receive targeted unit tests (see Amendment B).
**Action:** Revisit as a dedicated testing phase; G1/G2 tests are in-scope for this plan.

#### 6. Eliminate `errors.clone()` in recursive process_module_info (phase-0.2)
**Decision:** Close
**Rationale:** No `errors.clone()` exists in current code; errors are moved via `extend`, not cloned.
**Action:** Remove from deferred tracking.

#### 7. Fix determine_parent_file path calculation (MVP fix review)
**Decision:** Absorb
**Rationale:** Already in plan as Goal 3.

#### 8. Add integration test for recursive orphan detection (MVP fix review)
**Decision:** Absorb
**Rationale:** Already in plan as Goal 4.

#### 9. Make pub mod suggestion context-aware (MVP fix review)
**Decision:** Defer again
**Rationale:** Cosmetic UX fix unrelated to false-positive correctness.
**Action:** Revisit in a UX polish phase.

#### 10. Document src/tests/ directory pruning trade-off (MVP fix review)
**Decision:** Defer again
**Rationale:** Low-priority documentation unrelated to current phase.
**Action:** Revisit when users report false negatives in src/tests/ subtrees.

### Plan Amendments

1. **A — Prefix-aware G1 check:** Replace "check original unresolved path" with prefix-aware logic — bare paths check first segment against workspace members + crate modules; `crate::` paths strip prefix then check next segment (always internal); `self::`/`super::` paths skip the check entirely.
2. **B — Unit tests for G1 and G2:** Add tests in `validate.rs` for: bare-path external skip, `crate::` path proceed, companion-type suppressed, companion-type NOT suppressed when base has no derives, cross-module no-false-suppress.
3. **C — Document private-base-type limitation:** Add to README: private struct with derive-generated public companion can't be suppressed (base type absent from public-only symbol index).
4. **D — Correct derive_attrs data source:** `SymbolEntry.derive_attrs` is a straight copy of `item.attrs.derive` (already `Vec<String>`), not a re-extraction.
5. **E — Remove resolved deferred items:** Items 2 and 6 are already resolved in current code; remove from tracking.
6. **F — Clarify G2 module-scoped lookup:** Lookup key uses `module.path` as prefix — for `module.path = "mycrate::sub"` and target name `CellDocumentBuilder`, lookup keys are `CanonicalPath("mycrate::sub::CellDocument")`, etc.
