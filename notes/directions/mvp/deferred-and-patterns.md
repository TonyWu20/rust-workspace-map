# Deferred Improvements and Architectural Patterns: MVP

## Key Architectural Patterns from the Plan

### O(1) lookup design principle

The entire flat-index design (`symbols`, `name_index`, `files`) is built around a single axiom: an LLM agent should answer structural questions in one hashmap access, not tree traversal. Every index key is chosen so the agent asks one question, makes one lookup, gets one answer. The hierarchical `crates -> modules -> publicItems` tree still ships but is no longer the primary query interface.
**User notes**: The `O(1) lookup` proposed by me is somewhat misleading: It is not about the actual time spent by the binary, it is whether the LLM agent could retrieve the answer with 1 call of this tool

### Newtype key segregation

`CanonicalPath(String)` and `WorkspaceRelativePath(String)` prevent passing the wrong key kind at compile time. Both are transparent wrappers with `Display`, `AsRef<str>`, `From<String>`, `PartialOrd/Ord`, `PartialEq/Eq`, `Hash`, `Clone`, serde. No extra methods.

### Single binary, subcommand surface

Rejects decomposition into separate binaries. One `cargo build`, one install path, one `--help`. Subcommands (`index`, `lookup`) each write self-contained JSON to stdout. Exit codes: 0 = clean, 1 = tool error, 2 = validation found issues.

### Error collection consolidation

All diagnostics (existing errors plus new `OrphanFile`/`DeadReExport` warnings) flow into the single `WorkspaceMap.errors: Vec<ErrorEntry>` channel. The parallel `warnings: Vec<Warning>` approach was considered and rejected. `ErrorEntry.kind` is migrated from `String` to the typed `DiagnosticKind` enum with serde-rename preserving JSON output stability.

### Conservative recall policy for validation

False negatives over false positives. The `DeadReExport` rule skips external crates and glob re-exports explicitly. Transitive re-export chains are not traced. The goal is deterministic prevention of the top failure categories, not exhaustive coverage.

### Pipeline integrations ship with the tool

The MVP is not done until the pipeline uses it. The `compile-plan` pre-check (deterministic gate blocking on `OrphanFile`/`DeadReExport`) and `plan-decomposer` soft suggestion (Module Wiring Check) are part of the MVP deliverable, not a follow-up.

---

## Known Failure Modes to Avoid

### 1. Silent error swallowing (phase-0.1 deferred entry)

`build_module_tree` errors are swallowed via `unwrap_or_default()`, making it impossible to distinguish "no public items" from "parsing failed." The plan explicitly calls out the fragile `unwrap_or` at `module_tree.rs:33` and the dropped-errors bug at `lib.rs:132-137` where the `None` branch of the crate-results drain loop silently discards errors from failed crates. Both must be fixed.

### 2. O(n^2) error-vec cloning (phase-0.2 deferred entry)

The dual-path error reporting pattern in `process_module_info` (both `&mut Vec<ErrorEntry>` parameter AND `(Vec<ModuleInfo>, Vec<ErrorEntry>)` return value) forces a clone at each recursion level. The plan prescribes replacing this with `&mut Vec<ErrorEntry>` passed through recursion, using `extend` in place. Cost: O(depth x errors) today. Fixed by `mem::take` pattern at `module_tree.rs:252`.

### 3. Path fallback to `.` (phase-0.1 deferred entry)

Three `.unwrap_or_else(|| Path::new("."))` calls in `module_tree.rs` produce incorrect relative paths when `.parent()` returns None. Practically unreachable for real projects but masks bugs. The plan does not prioritize fixing these in the MVP but records them for when enhanced debugging is needed.

### 4. Path fallback to `""` in workspace.rs (phase-0.1 deferred entry)

Defaulting to empty vectors when the workspace section is missing silently includes zero members. Defaulting to `""` for file names bypasses the exclude filter. Lenient defaults mask configuration errors.

### 5. No unit tests for public API functions (phase-0.1 deferred entry)

Thirteen public functions across five modules have no unit tests. Integration tests cover the full pipeline but do not isolate individual function behavior. The plan does not require unit tests for the MVP but this increases regression risk during refactoring.

### 6. CLI breaking-change flag day

The `index` subcommand becomes mandatory (bare-path form removed), and `cross_references.types` re-keys from short-name to fully-qualified path — both in the same commit. The call-site sweep in `tests/integration_test.rs` must happen in the same commit. No deprecation period for an internal tool, but the sweep must be exhaustive.

### 7. Schema bloat risk

The plan explicitly gates additions: no `Confidence`, no `Warning` parallel channel, no `ImportSite`/`ReExportSite`, no per-symbol file:line reverse data. These are recorded as deferred and must not be added speculatively — they would ship unread data and obligate maintenance forever.

---

## Deferred Improvements That Should Be Incorporated

### From phase-0.1 deferred.md (carried forward)

| Improvement                                                                                            | Trigger for re-evaluation                                                   |
| ------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------- | ---------------------- | -------------------------------------------------------------------------- |
| **Empty errors vector in run()** — no code path populates `WorkspaceMap.errors` today                  | Before first consumer who needs visibility into workspace-level diagnostics |
| **Silent error swallowing in module tree construction** — `unwrap_or_default()` in `build_module_tree` | Before first consumer who needs per-crate health visibility                 |
| **Path fallback to `"."` in module_tree.rs** — three `.unwrap_or_else(                                 |                                                                             | Path::new("."))` calls | Before adding support for unusual filesystem layouts or enhanced debugging |
| **Path fallback to `""` in workspace.rs** — empty defaults mask config errors                          | Before extending member/exclude resolution logic                            |
| **No unit tests for public API functions** — 13 public functions un-tested                             | Before adding new features or refactoring existing code                     |

### From phase-0.2 deferred.md (carried forward)

| Improvement                                                                                                          | Trigger for re-evaluation                                                  |
| -------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| **Eliminate `errors.clone()` in `process_module_info`** — dual-path error reporting forces O(depth x errors) cloning | A measurable performance issue or a refactoring pass over `module_tree.rs` |

### From the MVP implementation (gaps discovered at audit)

| Improvement                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | Trigger for re-evaluation                                      |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------- |
| **DeadReExport false positives on external crates** — `pub use serde::Serialize;` (bare, no `crate::` prefix) is resolved to `mycrate::serde::Serialize` and then checked against the symbol index. Since `serde::Serialize` is not a member of `mycrate`, the lookup fails and produces a false positive. The plan mandates skipping external-crate re-exports (step 2), but the current code resolves before checking, so the first segment is always `mycrate`. Fix: check the original import path (before resolution) for a first segment that is neither a workspace member nor a known module name. | Before pipeline relies on DeadReExport as a deterministic gate |

### From the MVP plan itself (explicitly out of scope)

| Improvement                                                          | Trigger for re-evaluation                                                                     |
| -------------------------------------------------------------------- | --------------------------------------------------------------------------------------------- |
| `UnreachablePub` validate rule                                       | At least one measured plan-execution failure traces to an unreachable-pub case                |
| `UnsupportedLayout` warning (inline mod / #[path])                   | Covered by README documentation instead                                                       |
| `Confidence` enum (EXTRACTED/INFERRED/AMBIGUOUS)                     | When a non-AST data source is introduced                                                      |
| Per-symbol `imported_by` / `re_exported_at` at file:line granularity | A measured workflow cites "find references" with file:line precision                          |
| `--from-stdin` on validate and lookup                                | A third caller requests it                                                                    |
| `diff <base-ref>` subcommand                                         | At least one fix round attributable to a missed structural change in review                   |
| `--compact` and scope filters                                        | Measured token use exceeds 50% of context window on castep-cell-io                            |
| File-mtime-keyed cache                                               | Wall-time exceeds 2x the verification baseline AND compile-plan runs validate >= 3x per phase |
| MCP server / serve mode                                              | Pipeline grows its first MCP server                                                           |
| Eval harness                                                         | Deferred indefinitely — fix-task ratio already tracked in notes/pr-reviews                    |
| NDJSON streaming output                                              | A streaming consumer exists                                                                   |
| Multi-language Tree-sitter                                           | Out of scope permanently                                                                      |

### From the MVP plan (already being addressed in MVP)

These were deferred in prior rounds but are now explicitly scheduled for the MVP:

- **`errors.clone()` elimination at `module_tree.rs:252`** — replaced with `mem::take` + `&mut Vec<ErrorEntry>` parameter passing. This is a coded fix in the plan, not a deferral.
- **Dropped-errors bug at `lib.rs:132-137`** — the `None` branch of the crate-results drain loop now collects `errs` from both branches.
- **Empty errors vector** — resolved indirectly: the validate rules (`OrphanFile`, `DeadReExport`) populate `errors` for the first time, and the `DiagnosticKind` migration gives the existing `orphaned_module` warning a typed home.
- **`WorkspaceInfo.root` hardcoded to `"."`** — explicitly fixed in the plan to use the discovered workspace root path.

## Promoted from deferred (trigger met — now active)

| Item                                                                                                                                                                                                                                                                                                                                                          | Trigger met                                                                                                                                                                                                         | Approach                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Derive-macro symbol awareness** — symbols generated by derives (e.g. `bon::Builder` → `FooBuilder`) are invisible to `syn` AST scanning. `castep-cell-io` re-exports `CellDocumentBuilder` which is a derive-generated symbol, producing 283 DeadReExport findings — a mix of real dead re-exports and false positives from derive-generated builder types. | `castep-cell-io` use of `bon::Builder` generates builder types that are legitimately `pub use`-re-exported but absent from the symbol index. The `--validate` gate cannot be used on this workspace until resolved. | Add derive-macro awareness to `indexes.rs`: when building `symbols`, detect `#[derive(bon::Builder)]` on `pub struct` items and synthesize a corresponding `{Name}Builder` `SymbolEntry` with `kind: "builder"` and a `synthetic: true` flag. In `validate.rs`, dead-re-export lookups that miss should cross-check: if the base type `X` exists and has `#[derive(bon::Builder)]`, the `XBuilder` target is not dead — it's derive-generated. |
