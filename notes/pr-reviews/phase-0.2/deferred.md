## Deferred Improvements: `phase-0.2` — 2026-04-28

### Eliminate `errors.clone()` in recursive `process_module_info`

**Source:** Round 1 review
**Rationale:** The function uses dual-path error reporting — a `&mut Vec<ErrorEntry>` parameter for accumulation AND a return value `(Vec<ModuleInfo>, Vec<ErrorEntry>)`. This forces a clone at each recursion level, incurring O(depth × errors) cost. A single-path design (either all via return value or all via `&mut` parameter) would eliminate the clone and simplify the control flow.
**Candidate for:** Phase 0.3
**Precondition:** A measurable performance issue or a refactoring pass over `module_tree.rs`
