## Draft Fix Document

### Issue 1: `module_tree_error` kind string never emitted

**Classification:** Defect
**File:** `src/lib.rs`
**Severity:** Major
**Problem:** The phase-0.2 plan (Goal 1) specifies that errors from `build_module_tree` MUST be emitted as `ErrorEntry` entries with `kind = "module_tree_error"`. The current implementation at lines 83-86 collects errors from the `(Vec<ModuleInfo>, Vec<ErrorEntry>)` tuple return using the errors' own specific kinds (e.g., `syn_parse_error`, `orphaned_module`) rather than wrapping them under the `"module_tree_error"` kind. A grep for `"module_tree_error"` across all Rust source files returns zero matches. Downstream consumers filtering for `"module_tree_error"` will miss these errors entirely.
**Fix:** Either (a) emit the module-tree errors using `kind = "module_tree_error"` as the plan prescribes, or (b) update the plan taxonomy in `notes/plan-enrichment/phase-0.2/deferred-and-patterns.md` to document the specific kinds being used instead. Option (a) is preferred to keep the implementation aligned with the plan. If choosing (a), wrap each error from `build_module_tree` in a new `ErrorEntry` with `kind: "module_tree_error".to_string()` before appending to the error list in `src/lib.rs` around lines 83-86.
