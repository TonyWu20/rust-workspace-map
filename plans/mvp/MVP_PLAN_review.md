# Review of `MVP_PLAN.md` — KISS / first-principles critique

## Context

The MVP plan claims "very faithful" KISS and "ships only what closes a measured pipeline failure." Read end-to-end against the current state of `/Users/tony/programming/rust-workspace-map/`, parts of the proposed surface are speculative scaffolding that don't survive first-principles scrutiny on the right cost axis. The measured failure data — #1 `pub mod` (~40%), #2 `pub use` (~20%), #3 consumer updates (~20%) — supports a tighter MVP that earns the right to grow only if it doesn't close enough of the failure rate.

This document records the points where the plan over-builds and proposes a tighter cut. After review with the user (who clarified that the "O(1) lookup" framing was about token cost / agent reasoning load, not wall-clock latency), the critique was sharpened: flat indexes are accepted, but `Confidence`, `UnreachablePub`, parallel warning channel, `--from-stdin`, and other items remain over-built when the principle is applied uniformly.

## Ground truth that changes the calculus

Things the plan understates or omits about what already exists today:

- `schema::CrossReferences { types: BTreeMap<String, TypeRef> }` is **already** keyed by short symbol name, with `imported_by: Vec<String>` and `exported_by: Vec<String>` populated by `cross_refs::compute`. This is structurally what the plan calls `name_index` + part of `imported_by`. The plan presents flat indexes as new infrastructure when ~70% of one already ships in 0.1.0.
- `module_tree.rs` **already detects orphan modules in one direction**: a `mod foo;` declared but with no resolvable file emits an `orphaned_module` warning entry (`module_tree.rs:120–134`). The plan's `OrphanFile` is the reverse direction (file exists but no `mod foo;`), which is a real gap, but the plan doesn't note that the existing detector and the new one should share an `ErrorEntry` / warning emission pipeline rather than a parallel `Vec<Warning>` channel.
- `WorkspaceMap.errors: Vec<ErrorEntry>` already exists, with `severity: ErrorSeverity::{Error, Warning}`, `kind`, `context`, `cause`, `file`, `line`. The plan adds a parallel `warnings: Vec<Warning>` channel without explaining why the existing `ErrorEntry` with `Severity::Warning` won't do.
- All schema types derive `serde::Serialize` only — none derive `Deserialize`. The plan's `--from-stdin` requires `Deserialize` on every type plus a JSON-roundtrip stability commitment. This is a substantive cost the plan doesn't account for.
- `bon::Builder` is used pervasively; new types fitting that idiom is correct.
- `clap` derive is already wired; subcommand split is mechanical.

## The KISS critique, point by point

### 1. "O(1) lookup" — accepted on token-cost grounds, with a remaining tension

The plan justifies three pre-computed flat indexes (`symbols`, `name_index`, `files`) under "Core Design Goal: O(1) lookup for LLM agents." The intended cost axis is **token cost / agent reasoning load**, not wall-clock latency: a flat index lets an agent (or a `lookup` subcommand) jump straight to the answer rather than mentally tree-walking the hierarchical `crates[].modules[].public_items[]` shape — and prevents the agent's common fallback of running `rg`/`read` against the source when the JSON's shape isn't query-friendly. That argument is legitimate and KISS-aligned: pre-shaped answers are exactly the kind of thing measured "agent friction" should pay for.

The tension that remains, worth making explicit before building all three indexes:

- **If the dominant access pattern is `lookup --symbol/--file`** — i.e., the agent never sees the raw map — then the indexes only need to live as in-memory acceleration *inside* the `lookup` implementation. The JSON output of `index` does not need to carry them, because no agent reads the dump directly.
- **If the dominant access pattern is "load full map into agent context, reason globally"** — e.g., enrich-plan-gather, plan-review, rust-architect — then the indexes belong in the JSON output, because the JSON's shape *is* the agent's query interface. The duplication (each item appears in both the hierarchical view and the flat view) is the cost paid for query ergonomics.

The plan currently does both: indexes *and* `lookup`. That's a reasonable belt-and-suspenders position, but it's worth recording which access pattern is primary so future iteration knows what to trim. If `lookup` is the primary path, the indexes can be dropped from JSON output (kept in-memory only) once that's measured. If raw-map reading is primary, `lookup` becomes the redundant path.

**No change to recommendation on the indexes themselves.** Keep `symbols`, `name_index`, `files` in the MVP, but flag in the plan: *"primary access pattern TBD; revisit at the 3-phase review whether `lookup` or raw-map reading dominates, drop the redundant path then."*

The remaining first-principles concerns from the original critique survive on a different axis:

- `Confidence` is still YAGNI (every value will be `EXTRACTED`; no inference path exists in the MVP). This is independent of token cost — a constant-valued field neither shapes a query nor saves agent reasoning.
- `imported_by` / `re_exported_at` per-symbol, at file:line granularity, is justified *if* it's actually queried by a measured workflow. The MVP's three rules don't need it. `lookup --symbol` can return it cheaply — but if no agent prompt cites "who imports X," the data ships unread. Worth gating on a concrete pipeline citation.

### 2. `Confidence { EXTRACTED, INFERRED, AMBIGUOUS }` is pure YAGNI

The plan imports a CRG idiom into a tool that does only `syn` AST extraction. Every value will be `EXTRACTED`. There is no inference path, no ambiguity-resolver. Adding the field means consumers either ignore a constant field (noise) or branch on a value that never differs (dead code).

**Recommendation:** delete `Confidence` from the MVP. Add the day a non-AST data source is introduced, never before.

### 3. The new MVP rules don't need per-symbol reverse indexes

Walk through what each `validate` rule actually requires:

| Rule | Inputs needed | Already have? |
|---|---|---|
| `OrphanFile` | filesystem walk of crate `src/` + `module_tree::resolve_module_path` to check inclusion | Yes — `resolve_module_path` exists |
| `DeadReExport` | forward scan: for each `pub use a::b::C`, look for `C` in module `a` (or transitively) | Yes — `crates[].modules[].public_items[]` is the forward index |
| `UnreachablePub` | reachability over module tree | Tree exists; pass is new but cheap |

None of these require `imported_by` or `re_exported_at` per-symbol. The plan adds them to feed `lookup`, then mandates `lookup` in the plan-decomposer. That's circular justification — the indexes exist to support a subcommand that exists to motivate the indexes.

**Recommendation:** keep flat indexes on token-cost grounds (per §1), but defer the per-symbol reverse fields (`imported_by` at file:line, `re_exported_at` at file:line) until a measured workflow cites them. The crate-granularity `imported_by`/`exported_by` already in `CrossReferences.types` covers the common "which crate exports X" question.

### 4. `UnreachablePub` is scope creep mislabeled as "partial #3"

#3 is "incomplete consumer updates" — when an existing public item's signature/usage changes and callers don't get updated. `UnreachablePub` detects `pub` items with no public path from a crate root. These are different problems with negligible overlap. Calling it "partial #3" lets a speculative rule ride into the MVP on real-failure-data coattails.

The rule also adds graph-reachability logic for the smallest expected hit rate of the four rules.

**Recommendation:** drop `UnreachablePub` from MVP. Ship the two rules that close the measured 60%, observe for ≥3 phases per the plan's own "Out of scope" methodology, then decide.

### 5. `UnsupportedLayout` is not a rule, it's a README sentence

`mod foo { … }` and `#[path = ...]` exist in real code; emitting a structured warning that says "we didn't analyze this" produces noise on every run for workspaces that use either form. The user (the pipeline) has no action to take on it.

**Recommendation:** document the limitation in `README.md` under "What this tool does and doesn't analyze." If a real plan-execution failure ever traces to an unanalyzed inline mod, then add the warning.

### 6. `--from-stdin` adds composability with no consumer

The plan justifies `--from-stdin` with `index | validate --from-stdin | jq` as a "Unix property" worth preserving. But:

- Both pipeline integrations call subcommands directly (`validate <project>`, `lookup --file <parent>`). Neither pipes.
- No external user exists.
- Verification step #5 tests piping but the test exists to validate infrastructure with no caller.

The hidden cost: every schema type currently derives `serde::Serialize` only. `--from-stdin` requires `serde::Deserialize` everywhere plus a roundtrip-stability commitment (camelCase serialization is asymmetric on some types). That's ~30 derives plus a stability gate the plan doesn't acknowledge.

**Recommendation:** drop `--from-stdin` from MVP. Subcommands take a path. If a third caller asks for piping, add it then.

### 7. `FileEntry` duplicates `ModuleInfo` content — but the duplication is the point if the index is keyed on file path

Field-by-field:

| `FileEntry` field | Already in `ModuleInfo`? |
|---|---|
| `crate_name` | derivable from `path.split("::").first()` |
| `module` | yes — `path` |
| `submodules` | yes — `submodules` |
| `exports` | derivable — `public_items.iter().map(|p| &p.name)` |
| `is_crate_root` | derivable — `path` has no `::` |
| `parent_module_file` | new — needs a parent-of map built from `submodules` reverse |

Under the time-cost lens, this is wasteful denormalization. Under the token-cost lens (per §1 above), the `files: BTreeMap<String, FileEntry>` is *keyed on the file path string*, which the hierarchical `crates[].modules[]` view is not. An agent asking "what's in `core/src/task.rs`?" hits the file index directly; without it, the agent has to scan modules and match on `file`.

So the right framing isn't "delete `FileEntry`," it's:

- The genuinely new fields are `is_crate_root` and `parent_module_file`. Everything else is denormalization driven by the indexing-key choice (file path).
- Pick the slimmest `FileEntry` that pays for the new key. One viable shape: `{ module_path, parent_module_file, is_crate_root }` — three fields, with `lookup --file` re-joining to the matching `ModuleInfo` for the rest. Avoids re-serializing `submodules` and `exports` into the JSON when they're already in the hierarchical view.
- Or accept the duplication outright and document that the JSON is pre-denormalized for agent ergonomics.

**Recommendation:** keep `files` as a flat index, but slim `FileEntry` to the three fields the hierarchical view doesn't already cover (`module_path`, `parent_module_file`, `is_crate_root`). `lookup --file` reads the slim entry and joins to the corresponding `ModuleInfo`. JSON output stays compact; agents loading the raw map can still hashmap-lookup by file path.

### 8. Replace stringly-typed `ErrorEntry.kind` with an enum, don't add a parallel `Vec<Warning>` channel

Workspace already has `ErrorEntry { file, line, message, severity, kind: String, context, cause }` with `ErrorSeverity::Warning`. The plan's new `Vec<Warning>` channel duplicates infrastructure for no reason — the same row of data fits in `ErrorEntry` exactly. Per user preference, an enum is the right shape (over the current stringly-typed `kind`), so:

**Recommendation:**

1. Introduce a single `enum DiagnosticKind` (or `WarningKind` if you want to keep the name; "diagnostic" reads better given existing rows already cover both errors and warnings) with variants for the existing string values currently passed (`OrphanedModule`, `TomlParseError`, `MissingCrateRoots`, `ModuleTreeError`, plus the new `OrphanFile`, `DeadReExport`, etc.).
2. Migrate `ErrorEntry.kind: String → kind: DiagnosticKind`. This is a one-shot refactor: every emit-site in `lib.rs`, `module_tree.rs`, `file_parser.rs` becomes a typed variant; the JSON `kind` field stays a string via serde rename, so consumers don't see a breaking change.
3. The new validate rules emit `ErrorEntry { severity: Warning, kind: DiagnosticKind::OrphanFile, ... }` into the existing `WorkspaceMap.errors`. No `Vec<Warning>`. No parallel channel.

This is the disciplined version of the plan's intent — typed kinds — applied consistently rather than only to the new code path. It also eliminates the asymmetry where existing `orphaned_module` warnings and new `OrphanFile` warnings would have lived in different collections with different typing.

### 9. Plan-decomposer hard-mandate-to-call-lookup is fragile defense-in-depth

The plan requires the plan-decomposer LLM to call `lookup --file <parent>` before emitting any new-file `[[changes]]` entry, with the result cited in plan rationale.

- LLM compliance with hard procedural mandates is probabilistic. A skipped call can't be detected in-band.
- The downstream `compile-plan` pre-check already catches the issue deterministically.
- If the pre-check is the gate, the plan-decomposer's call is redundant work; if the pre-check misses, the plan-decomposer almost certainly missed the same case.

**Recommendation:** make the `compile-plan` pre-check the authoritative gate. Plan-decomposer integration becomes a soft suggestion ("when introducing a new file, consider `lookup --file <parent>` to confirm intended siblings"). Soft suggestions cost nothing if skipped; hard gates relying on agent compliance fail silently.

### 10. The verification matrix has the wrong cache trigger

Verification step #6 sets a 2-second wall-time trigger on `castep-cell-io` for the cache deferral. First-principles estimate: AST parse 303 small files in parallel (`rayon` is already used) on modern hardware ≈ 100–400 ms. The 2-second threshold is unlikely to ever trigger. That's fine, but the plan should record the **expected** baseline so the cache truly remains deferred rather than becoming a "we measured 1.8s, let's be safe" creep target.

**Recommendation:** measure current 0.1.0 wall-time on `castep-cell-io` once. Lock that as baseline. Cache layer is allowed only if a future change pushes wall-time past 2× baseline AND `compile-plan` runs validate ≥3× per phase.

### 11. The breaking-change mechanics are under-specified

`index` becomes mandatory in v0.2.0; bare-path form deleted entirely. Fine decision. But the plan doesn't enumerate:

- Every invocation in `tests/integration_test.rs` needs rewriting.
- Any wrapper scripts in `rust-development-pipeline/` need updating in the same commit.
- README examples need updating.
- Anything in the user's `notes/` that has copy-pasteable invocations.

This isn't an objection to the decision, just to the breeziness. A breaking change is a flag-day event; the plan should have a checkbox for the call-site sweep.

## The principle the plan violates

The plan articulates the right principle: *"ships only what closes a measured pipeline failure or directly enables a pipeline integration that closes one. Anything else is recorded as deferred."* It then applies that principle unevenly. Some features get the discipline:

- `diff` subcommand → deferred
- `--compact` / scope filters → deferred
- File-mtime cache → deferred
- `serve` mode / MCP → deferred
- Macro expansion → deferred
- Eval harness → deferred
- NDJSON streaming → deferred

Other features remain in scope. Of those, some are justified on the right axis (per §1 token-cost argument) and survive the principle:

- Flat indexes (`symbols`, `name_index`, slim `files`) → justified by token cost / agent reasoning load. **Keep.**

Others remain over-built when the principle is applied uniformly:

- `Confidence` enum → no inference path exists; constant-valued field. **Cut.**
- `imported_by` / `re_exported_at` per-symbol at file:line granularity → not required by the three rules; ships unread unless a measured workflow cites it. **Defer.**
- `UnreachablePub` rule → "partial #3" attribution is overstated; cheap to add later. **Defer.**
- `UnsupportedLayout` warning → no actionable response from the consumer. **Make a README sentence.**
- `--from-stdin` → no piping consumer; hidden `Deserialize` cost. **Defer.**
- Parallel `Vec<Warning>` channel → duplicates `WorkspaceMap.errors`. **Reuse `ErrorEntry` with typed `kind`.**
- Plan-decomposer hard-mandate-to-call-`lookup` → fragile compliance gate when `compile-plan` pre-check is the deterministic gate. **Soft suggestion only.**

## A KISS-cut MVP

What ships if the plan's stated principle is applied uniformly, with §1's token-cost reasoning preserved:

**Schema delta** (in `src/schema.rs`):
- Migrate `ErrorEntry.kind: String → kind: DiagnosticKind` (typed enum, serde-renamed to a string in JSON for backward-compatible output). Variants cover existing kinds (`OrphanedModule`, `TomlParseError`, `MissingCrateRoots`, `ModuleTreeError`) plus new (`OrphanFile`, `DeadReExport`).
- Add `SymbolEntry` (slim — `crate_name`, `module`, `file`, `line`, `kind: ItemKind`, no `confidence`, no `imported_by`/`re_exported_at` until a workflow cites them).
- Add `FileEntry` (slim — `module_path`, `parent_module_file`, `is_crate_root`).
- Add three `BTreeMap` fields on `WorkspaceMap`: `symbols`, `name_index`, `files`. Built in a single derivation pass after `cross_refs::compute`.
- Do **not** add: `Confidence`, `Warning`/`WarningKind` (use `ErrorEntry`), `ImportSite`/`ReExportSite`, parallel `warnings: Vec<Warning>`.

**CLI delta** (`src/main.rs`):
- Subcommands: `index` (current behavior), `validate <PATH>`, `lookup <PATH> --symbol|--file`. No `--from-stdin`.
- Bare-path form removed; `tests/integration_test.rs` rewrites in lockstep.
- Exit codes: `0` clean, `1` tool error, `2` validation findings.

**`validate` rules** (new file `src/validate.rs`, pure function over `&WorkspaceMap` returning `Vec<ErrorEntry>` to merge into the existing `errors` field):
- `OrphanFile` — fs walk of each crate's `src/` minus the set of files reachable through `module_tree::resolve_module_path`. Reuses the existing resolver.
- `DeadReExport` — for every `pub use a::b::C` across all modules, check whether any module under `a::*` has a `public_item` named `C`.
- **Not** `UnreachablePub` (defer until measured #3 cases attributable to it).
- **Not** `UnsupportedLayout` as a rule — document the limitation in `README.md`.

**Index derivation** (new file `src/indexes.rs`, pure function):
- `derive_indexes(&WorkspaceMap) -> (symbols, name_index, files)` in one pass over `crates[].modules[]`.

**Pipeline integration** (one only):
- `skills/compile-plan/SKILL.md` adds a pre-check step: run `validate`, block on any `OrphanFile` / `DeadReExport` finding.
- Plan-decomposer integration is a *soft suggestion* — "when introducing a new file, consider `lookup --file <parent>` to learn the intended siblings." No hard mandate.

**Out of MVP** (additive, defer until measured):
- `Confidence` enum → defer until a non-AST data source is introduced.
- Per-symbol `imported_by` / `re_exported_at` at file:line → defer until a measured workflow cites "find references."
- `UnreachablePub` rule → defer until measured #3 cases appear.
- `UnsupportedLayout` warning → README sentence.
- `--from-stdin` → defer until a piping consumer exists; carries hidden `Deserialize` cost.
- Plan-decomposer hard-call-`lookup` mandate → soft suggestion only.
- File-mtime cache → defer per the plan's existing methodology.
- Everything else from the plan's own "Out of MVP scope" stays out.

**Coverage projection:** ~60% of measured failure rate (#1 + #2 caught deterministically). Same as the bigger plan's hard-block coverage; the bigger plan's `UnreachablePub` adds at most a few percent on a partial-#3 axis with high false-negative tolerance.

**Effective trim vs the plan as written:** four schema types removed (`Confidence`, `Warning`, `ImportSite`, `ReExportSite`), one rule removed (`UnreachablePub`), one rule downgraded to documentation (`UnsupportedLayout`), one CLI flag removed (`--from-stdin`), one mandate softened (plan-decomposer), one channel collapsed (warnings → `errors` with typed kind). Indexes kept; `lookup` kept; subcommand split kept; breaking change kept.

## Critical files referenced

- `/Users/tony/programming/rust-workspace-map/src/schema.rs` — the existing `WorkspaceMap`, `ErrorEntry`, `ErrorSeverity`, `CrossReferences`, `Import`, `ReExport`. The plan's additive changes mostly belong here.
- `/Users/tony/programming/rust-workspace-map/src/module_tree.rs` — `resolve_module_path` and existing orphan detection. `OrphanFile` rule reuses both.
- `/Users/tony/programming/rust-workspace-map/src/cross_refs.rs` — already does name-keyed `imported_by`/`exported_by`. The plan's `name_index` is largely a rename.
- `/Users/tony/programming/rust-workspace-map/src/main.rs` — the bare-path CLI; subcommand split lands here.
- `/Users/tony/programming/rust-workspace-map/src/lib.rs` — the orchestrator; would call into `validate::run` after `cross_refs::compute`.
- `/Users/tony/programming/rust-workspace-map/Cargo.toml` — `clap`, `bon`, `syn`, `rayon`, `serde` already present. No new crates needed for the KISS-cut MVP.
- `/Users/tony/programming/rust-workspace-map/tests/integration_test.rs` — every existing invocation rewrites to `index <path>`.
- `/Users/tony/programming/rust-workspace-map/tests/fixtures/sample-workspace/` — add `bad-orphan/` and `bad-dead-reexport/`.
- `/Users/tony/programming/rust-development-pipeline/skills/compile-plan/SKILL.md` — single integration; pre-check step.

## Verification (for the KISS-cut MVP)

1. `cargo build --release` clean.
2. All existing integration tests pass after subcommand rewrite (`<path>` → `index <path>`).
3. `bad-orphan/` fixture → `validate` exits 2, warning names parent file.
4. `bad-dead-reexport/` fixture → `validate` exits 2, warning names the dead path.
5. `validate /Users/tony/programming/castep-cell-io` runs to completion. Wall-time recorded as cache baseline.
6. `compile-plan` against a synthetic plan that creates a file without `pub mod` blocks. Same for a synthetic `pub use` of a nonexistent symbol.

That's a real MVP. Ship it, run it through `castep-cell-io`, watch the fix-task ratio in `notes/pr-reviews/` for ≥3 phases. Then revisit `lookup`, `UnreachablePub`, flat indexes — whichever the *measured* gap demands, in priority order, one at a time.
