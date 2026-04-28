## Deferred Improvements: `phase-0.2` — 2026-04-28

### Eliminate `errors.clone()` in recursive `process_module_info`

**Source:** Round 1 review
**Rationale:** The function uses dual-path error reporting — a `&mut Vec<ErrorEntry>` parameter for accumulation AND a return value `(Vec<ModuleInfo>, Vec<ErrorEntry>)`. This forces a clone at each recursion level, incurring O(depth × errors) cost. A single-path design (either all via return value or all via `&mut` parameter) would eliminate the clone and simplify the control flow.
**Candidate for:** Phase 0.3
**Precondition:** A measurable performance issue or a refactoring pass over `module_tree.rs`

## Deferred Improvements: `phase-0.2` — 2026-04-29

### Eliminate `errors.clone()` in recursive `process_module_info` (Round 2 confirmation)

**Source:** Round 2 review
**Rationale:** Same observation as Round 1 — the dual-path error reporting pattern in `process_module_info` forces a clone at each recursion level. Re-identified in Round 2 review; still deferred rather than addressed. No fix tasks in this round; the improvement remains appropriate for Phase 0.3.
**Candidate for:** Phase 0.3
**Precondition:** A measurable performance issue or a refactoring pass over `module_tree.rs`
