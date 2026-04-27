## Plan Review Decisions — phase-0.2 — 2026-04-28

### Design Assessment

The plan is substantially sound. The goals, scope boundaries, error severity semantics, and execution sequence are well-structured. One architectural gap was identified and resolved: the all-or-nothing module tree failure mode, which would have discarded an entire crate's data on a single submodule parse failure. The amended plan adopts a partial-results pattern where parse failures are collected as `ErrorEntry` values and sibling modules continue processing. Minor gaps around parallel-closure error coverage, the `kind` taxonomy, test module handling, and workspace error conversion were all addressed through plan amendments.

### Deferred Item Decisions

#### Empty errors vector in run()
**Decision:** Absorb
**Rationale:** Absorbed by Goal 1, which wires error collection end-to-end. The precondition (defining which errors to collect) is satisfied by the ErrorSeverity semantics and kind taxonomy tables.
**Action:** Goal 1 in the amended plan.

#### Silent error swallowing in module tree construction
**Decision:** Absorb
**Rationale:** Absorbed by Goal 1, which explicitly removes `.unwrap_or_default()` on `build_module_tree`.
**Action:** Goal 1 in the amended plan.

#### Path fallback to "." in module_tree.rs
**Decision:** Absorb
**Rationale:** Absorbed by Goal 3, which explicitly lists all three `unwrap_or_else(|| Path::new("."))` callsites.
**Action:** Goal 3 in the amended plan.

#### Path fallback to "" in workspace.rs
**Decision:** Absorb
**Rationale:** Absorbed by Goal 3 item 4 (now clarified to use `MissingWorkspaceSection` error) and Goal 1 (ErrorEntry conversion path).
**Action:** Goals 1 and 3 in the amended plan.

#### No unit tests for public API functions
**Decision:** Absorb
**Rationale:** Absorbed by Goal 2, which comprehensively lists the 13 functions across 5 modules.
**Action:** Goal 2 in the amended plan.

### Plan Amendments

1. **Test module handling** — Clarified that external `#[cfg(test)]` modules are parsed for error reporting but public items are not extracted
2. **Parallel-closure error paths** — Added explicit bullets for `parse_cargo_toml` failures, `resolve_crate_roots` empty results, and `build_module_tree` error conversion with specific `kind` values
3. **MemberNotFound usage** — Added bullet to emit ErrorEntry for missing workspace members via existing `Error::MemberNotFound`
4. **Goal 3 item 4 clarification** — Updated to specify `Err(Error::MissingWorkspaceSection)` path with `kind = "missing_workspace_section"`
5. **Partial module tree preservation** — Added Design Notes subsection documenting the partial-results pattern
6. **Error kind taxonomy** — Added Design Notes subsection with 8-value initial kind set
7. **Error handling crates** — Added Design Notes subsection confirming `thiserror` + `anyhow` are sufficient
8. **Decisions section** — Updated with all new design decisions from the review
