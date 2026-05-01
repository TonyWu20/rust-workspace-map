# Second Review: MVP Plan — Structural Critique

## Context

The first review (`MVP_PLAN_review.md`) correctly trimmed scope: it killed `Confidence`, `UnreachablePub`, `--from-stdin`, the parallel `Vec<Warning>` channel, and the plan-decomposer hard mandate. All correct cuts.

This second review scrutinizes the **algorithms, data model integrity, and implementation sequencing** that the first review didn't cover. Grounded in the actual codebase at `/Users/tony/programming/rust-workspace-map/src/`.

---

## User walkthrough decisions

| # | Topic | Decision |
|---|---|---|
| 2 | `DeadReExport` false-positive risk | **Specify 5-case algorithm** before implementation |
| 7 | `validate` doubles parse cost | **Merge as `index --validate`** flag |
| 1 & 3 | `cross_references.types` collision + `files` inline-module | **Fix both in MVP** |
| 4 | `OrphanFile` algorithm underspecified | **Option C**: diff `ModuleInfo.file` set vs fs walk |
| 5 | `indexes::derive` sequencing | **Option B**: `derive_from_crates(&[CrateInfo])` before builder |
| 6 | "O(1) lookup" claim | **Resolved** — O(1) means one agent call, not algorithmic complexity |
| 8 | `lookup` parse-per-invocation cost | **Document as human tool only** until cache ships |
| 9 | `symbols` key format for root items | **Specify** `crate::Name` (2 segments) vs `crate::mod::Name` (3+) |
| 10 | `WorkspaceInfo.root` hardcoded to `"."` | **Fix**: set to actual workspace root |
| 11 | `module_tree.rs` existing bugs | **Fix** during DiagnosticKind refactor |

---

## Finding 2 (DECIDED: Specify 5-case algorithm)

**Severity: False positives — would block valid plans**

The plan originally said: *"for every `pub use a::b::C`, check whether any module under `a::*` has a `public_item` named `C`."*

This only works for intra-crate re-exports. Real code has five cases:

| Re-export form | Naive check result |
|---|---|
| `pub use crate::inner::Thing;` | Works (after prefix stripping) |
| `pub use super::thing;` | Fails — `super` is relative |
| `pub use self::detail::Type;` | Fails — `self` needs resolution |
| `pub use serde::Serialize;` | **False positive** — external dep, not in workspace |
| `pub use some::path::*;` | Cannot evaluate — glob |

Case 4 is critical: `pub use serde::Serialize;` is valid but the naive algorithm won't find `Serialize` in workspace modules → emits `DeadReExport` warning → `compile-plan` pre-check **blocks a valid plan**.

**Decision:** The algorithm now specified in `MVP_PLAN.md` must:
1. Resolve `crate::`, `self::`, `super::` prefixes
2. Skip re-exports where the first segment is NOT a workspace member (external crate)
3. Skip glob re-exports (conservative — false negatives preferred per policy)
4. Document that transitive re-export chains are not traced in MVP

**Amendment applied:** Yes — §"validate rules" table updated.

---

## Finding 7 (DECIDED: Merge as `index --validate`)

**Severity: Efficiency**

`index` and `validate` both do full AST parses. Sequential invocations double the cost for zero new information. The `validate` output is a superset of `index` output (same JSON + extra ErrorEntries).

**Decision:** `index` gets a `--validate` flag. Single invocation:
- `rust-workspace-map index <path>` — current behavior (no validation)
- `rust-workspace-map index <path> --validate` — runs OrphanFile + DeadReExport, findings appear in `errors`

**Amendments applied:**
- CLI section: removed `validate` subcommand, added `[--validate]` flag to `index`
- Files table: removed separate `src/validate.rs` row (merged into `index --validate` flow)
- Pipeline integrations: `compile-plan` pre-check calls `index --validate`
- Verification steps: all `validate` references → `index --validate`
- Locked decisions D2: updated

---

## Findings 1 & 3 (DECIDED: Fix both in MVP)

### 1. `cross_references.types` short-name collision corrupts data

`cross_refs.rs:30-36`: `TypeRef` is keyed by short name only. When two crates define different `Config` types, the first one processed wins — the second's `kind`, `crate_name` are silently lost (only `exported_by` gets appended). Any consumer trusting `TypeRef.kind` gets wrong data.

Names like `Config`, `Error`, `Result` appear across crates routinely — this is not an edge case.

**Fix:** Key `cross_references.types` by fully-qualified path (`crate::module::Name`).

### 3. `files` index silently drops inline modules

One `.rs` file can host multiple modules (`mod tests { ... }` in `lib.rs`). `files: BTreeMap<String, FileEntry>` stores only the last-inserted entry per file path — other modules silently dropped. Non-deterministic which wins.

When `plan-decomposer` calls `lookup --file core/src/lib.rs`, it gets whichever module happened to be processed last.

**Fix:** Exclude inline modules from `files` index (they have no independent file).

**Amendments applied:**
- Schema section: added comment documenting `cross_references.types` re-keying
- Schema section: added inline-module exclusion note to `FileEntry`
- Locked decisions D2: added "re-key `cross_references.types` from short-name to fully-qualified path"

---

## Finding 4 (DECIDED: Option C — diff ModuleInfo.file vs fs walk)

**Severity: Under-specification**

The plan said: *"walk every `.rs` under the crate source dir, check whether the resolver included it."* This is not specific enough to implement.

Three approaches exist:

| Approach | Correctness | Cost |
|---|---|---|
| **A:** Collect visited file set from `build_module_tree` return | Correct — uses actual parsed tree | Requires changing `build_module_tree`'s return signature |
| **B:** Use `resolve_module_path` in reverse for each `.rs` file | Partial — proves file *could* be included, not that it *was* | No refactoring; misses deeply nested orphans |
| **C:** Deduplicate `ModuleInfo.file` paths from already-returned modules | Correct — uses finalized module tree | No refactoring needed; path normalization required |

**Decision: Option C (simplest correct approach):**
1. After `build_module_tree` returns for a crate, collect all `ModuleInfo.file` values (excluding `<unresolved>`)
2. Walk the crate's `src/` directory for all `*.rs` files
3. Any `.rs` on disk absent from the file set = orphan → `OrphanFile` warning
4. Exclude known non-module files: `build.rs`, `src/bin/*.rs`, test/example directories

No refactoring of `build_module_tree`'s return type needed — the file set is already in the returned `Vec<ModuleInfo>`.

**Amendment applied:** Yes — §"validate rules" table updated for `OrphanFile`.

---

## Finding 5 (DECIDED: Option B — derive_from_crates before builder)

**Severity: Implementation friction**

The plan said `indexes::derive(&WorkspaceMap)` but the map is built via builder at the end of `run()`. The indexes need to go INTO the map.

**Options:**
- **Option A:** Build map first, mutate afterward — works mechanically (all fields are `pub`) but awkward
- **Option B (cleaner):** `derive_from_crates(&[CrateInfo])` before builder, pass results directly to builder

**Decision: Option B.** Matches how `cross_refs::compute` works today (takes `&[CrateInfo]` before the map exists).

**Amendments applied:**
- Files table: updated `src/indexes.rs` and `src/lib.rs` rows with new signature

---

## Finding 6 (RESOLVED — user clarification)

**Original critique:** "O(1) lookup" for `lookup --file` is O(1)+O(n) internally (hashmap hit + linear ModuleInfo scan).

**User clarification:** The O(1) stands for "the agent needs one call to get the correct, desired answer for its query." It's about **agent interaction cost** (one tool call → one answer), not internal algorithmic complexity. The internal O(n) scan is irrelevant to the agent interaction model.

**Resolution:** Design rationale is correct under this framing. Withdrawn — no plan changes needed.

---

## Finding 8 (DECIDED: Accept — document as human tool)

**Severity: Efficiency**

`lookup` pays full AST parse per invocation. If the plan-decomposer calls `lookup` 5 times, that's 5x parse cost for 5 hashmap lookups. Without a cache, this wastes time for pipeline use.

**Decision:** For the MVP, document `lookup` as a **human CLI convenience**. Pipeline agents should embed flat indexes in their context window (single JSON load) rather than making per-symbol CLI calls. Revisit `lookup` as a pipeline tool only after the cache layer ships.

**Amendment applied:** §"lookup semantics" — added note about human-convenience scope.

---

## Finding 9 (DECIDED: Specify key format)

**Severity: Under-specification**

The plan says `symbols` keyed by `"crate::module::Name"` but root-module items (in `lib.rs`) have no module segment.

**Decision:**
- Root-module items: `"crate::Name"` (2 segments)
- Nested-module items: `"crate::module::Name"` (3+ segments)
- Parsers split on `::`: first segment = crate name, last = item name, middle = module path

**Amendment applied:** §"Schema additions" — added key-format comment to `SymbolEntry`.

---

## Finding 10 (DECIDED: Fix)

**Severity: Latent bug**

`lib.rs:149-152`: `WorkspaceInfo.root` is always `"."`. With subcommands, input `PATH` can be a subdirectory (since `find_workspace_root` walks up), but file paths are relative to the **actual** workspace root. If a consumer tries to resolve file paths using `root` as base, they get wrong paths.

**Decision:** Set `root` to the discovered workspace root. One-line fix in `lib.rs` in the WorkspaceInfo builder call.

**Amendment applied:** Added to `src/lib.rs` row in files table.

---

## Finding 11 (DECIDED: Fix during refactor)

**Severity: Existing bugs**

- **Bug A** (`module_tree.rs:252`): `errors.clone()` at every recursion level duplicates errors O(n²) in memory. For a 4-level tree with errors at each level, waste compounds. `OrphanFile` will add more errors, making this worse.
- **Bug B** (`module_tree.rs:33`): `crate_root.parent().unwrap_or(crate_root)` — if `crate_root` somehow has no parent, `parent_dir` falls back to root itself, which is semantically wrong for submodule resolution.

**Decision:** Fix both during the planned `DiagnosticKind` mechanical refactor. The code is already being touched.

**Amendment applied:** `src/module_tree.rs` row in files table now includes "Fix error-vec cloning at line 252 and fragile `unwrap_or` at line 33."

---

## Summary of amendments applied to MVP_PLAN.md

| Amendment | Section | Change |
|---|---|---|
| A: `index --validate` | §CLI, §Files, §Pipeline, §Verification | Removed `validate` subcommand; merged as `index --validate` flag |
| B: 5-case DeadReExport | §Validate rules | Algorithm now handles 5 distinct re-export forms |
| C: OrphanFile Option C | §Validate rules | Algorithm now specified as ModuleInfo.file set diff vs fs walk |
| D: cross_references fix | §Schema additions | Documented re-key to fully-qualified path (collision fix) |
| D: FileEntry inline-module | §Schema additions | Added inline-module exclusion note |
| E: derive_from_crates | §Files (indexes.rs, lib.rs) | Changed signature from `derive(&WorkspaceMap)` to `derive_from_crates(&[CrateInfo])` |
| F: lookup as human tool | §Lookup semantics | Added paragraph about human-convenience scope |
| G: symbols key format | §Schema additions | Specified `crate::Name` (root) vs `crate::mod::Name` (nested) |
| H: WorkspaceInfo.root fix | §Files (lib.rs) | Added fix note for hardcoded `"."` root |
| I: module_tree.rs bugs | §Files (module_tree.rs) | Added error-cloning and `unwrap_or` fixes |
| J: Verification & D2 | §Verification, §Locked decisions | Updated all `validate` refs → `index --validate` |

**Outstanding items for future review rounds (from this review):**
- `cross_references.types` re-keying: implementation decision needed (fully-qualified key vs `Vec<TypeRef>` per short name)
- `files` inline-module handling: implementation decision needed (exclusion vs `Vec<FileEntry>`)
- `lookup` cache: revisit when a measured pipeline use case demands it
