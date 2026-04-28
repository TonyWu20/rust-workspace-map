# Plan: `rust-workspace-map` MVP for `rust-development-pipeline` integration

## Context

`rust-workspace-map` already exists at `/Users/tony/programming/rust-workspace-map/` as a single-binary `syn`-based tool that emits a hierarchical JSON map of a Rust workspace's public API surface (`crates → modules → publicItems`). Two feature-request docs in this repo (`feature-requests/rust-workspace-map-agent-centric-design.md`, `feature-requests/rust-workspace-map-agent-centric-development-plan.md`) propose six new features at once: flat indexes, validation, structural diff, compact mode, scope filtering, plus broad pipeline integration.

The user clarified the motivation: this tool is **not a community-facing product**. It is internal infrastructure for `rust-development-pipeline` and exists to **smoothen the pipeline, accelerate it, and reduce token waste**. The two will evolve together.

The user also picked **Option C** in the prior comparison with `tirth8205/code-review-graph` (CRG): build a Rust-native tool independently and adopt CRG's *architectural* lessons (subcommand surface, edge-confidence labeling stance, conservative-recall posture) **without** importing CRG's heavyweight machinery (SQLite store, daemon, MCP server, watch hooks, vector search, eval harness — all explicitly out of MVP scope).

The measured failure data (`feature-requests/reduce_recurring_problems.md`) names three top recurring fix-task categories:
- **#1 (~40%)** — missing `pub mod` declarations
- **#2 (~20%)** — missing `pub use` re-exports
- **#3 (~20%)** — incomplete consumer updates

The MVP's job is to deterministically prevent #1 and #2 at plan-execution time, and structurally support #3 prevention at plan-decomposition time. Everything beyond that is out of MVP scope.

### Core Design Goal: O(1) lookup for LLM agents

This tool exists to answer structural questions about a Rust workspace in **constant time — one hashmap access, no tree traversal, no path guessing**. An LLM agent asking a question should get an answer the way an IDE answers: instant, precise, pre-computed.

The three flat indexes (`symbols`, `name_index`, `files`) pre-compute the relationships a human engineer builds through interactive IDE navigation:

| Agent question | IDE equivalent | Lookup | Cost |
|---|---|---|---|
| "Is `Task` a type? Where is it defined?" | Go-to-definition | `name_index["Task"]` → `symbols[path]` | O(1) |
| "What file contains module `foo::bar`?" | Go-to-file | iterate `files`, match `entry.module_path == "foo::bar"` (or in-memory reverse map built once at index time) | O(1) amortized |
| "What modules does `lib.rs` declare?" | File structure view | `files["core/src/lib.rs"].module_path` → `crates[].modules[]` join → `submodules` | O(1) hop, then O(1) join |
| "Is this file reachable from the module tree?" | Module graph view | `files[path].parent_module_file.is_some()` (or file is a crate root) | O(1) |
| "Which crate exports `Task`?" | (no direct IDE analog) | `cross_references.types["Task"].exported_by` | O(1) — already shipping in 0.1.0 |

This is the fundamental difference from the hierarchical `crates → modules → publicItems` tree alone. That tree requires the agent to traverse, filter, and guess module paths. The flat indexes eliminate traversal entirely — the agent asks one question, makes one lookup, gets one answer. The tool is not a JSON dump for humans to browse; it is a query engine where every query is O(1) and every answer fits in a few hundred tokens.

**Note on what's *not* O(1) in MVP:** "Who imports this symbol?" at file:line granularity is intentionally deferred — the existing `cross_references.types[name].imported_by` answers it at crate granularity, which covers most measured workflows. File:line precision adds extraction cost and per-symbol storage that no current pipeline integration cites. Add when a measured workflow demands it.

---

## Coding style

### Builder pattern
All new structs (`SymbolEntry`, `FileEntry`) use `#[derive(bon::Builder)]`, consistent with the existing codebase idiom.

### Newtype pattern
Two newtypes wrap the BTreeMap key strings to prevent passing the wrong key kind at compile time:

```rust
pub struct CanonicalPath(String);        // "crate::module::Name" — key for symbols, name_index values, cross_references.types
pub struct WorkspaceRelativePath(String); // "core/src/task.rs"   — key for files
```

Each implements `Display`, `AsRef<str>`, `From<String>`, `PartialOrd/Ord`, `PartialEq/Eq`, `Hash`, `Clone`, and serde `Serialize`/`Deserialize` (transparent). No other pass-through methods.

### Functional style — iterators over for-loops
New code in `indexes.rs`, `validate.rs`, and `lookup.rs` uses iterator pipelines (`flat_map`, `filter_map`, `fold`, `collect`) rather than `mut` accumulator loops. Existing code in `module_tree.rs` and `lib.rs` is touched only where the bug fixes require it — no opportunistic refactoring.

### `mem::take` for the O(n²) error-vec fix
The `errors.clone()` at `module_tree.rs:252` is replaced by passing `&mut Vec<ErrorEntry>` down the recursion and using `extend` in place, eliminating the per-level clone.

### `Option` as iterator
`Option<T>` fields (e.g. `parent_module_file`) are consumed via `.into_iter()` / `.extend(opt)` / `.chain(opt.iter())` in pipeline contexts rather than `if let Some` unwrapping.

---

## Architectural commitments (locked)

### KISS faithfulness — very faithful

The MVP ships only what closes a measured pipeline failure or directly enables a pipeline integration that closes one. Anything else is recorded as deferred and built only when measured pain re-surfaces it.

### Single binary with subcommands (not multiple binaries)

Decomposition into separate binaries (`rwm-index`, `rwm-validate`) is rejected. Pipeline skills make one tool call per step — they do not chain N tools. A single binary with subcommands (`cargo`, `git`, `gh`, `kubectl` idiom) gives one Cargo build, one install path, one `--help`. The Unix property holds *inside* the binary:

- Each subcommand has one job and writes self-contained JSON to stdout.
- Subcommands take a workspace path argument. **`--from-stdin` is deferred** — adding it would force `serde::Deserialize` on every schema type plus a JSON-roundtrip stability commitment, and no current pipeline integration pipes (`compile-plan` calls `validate <project>`, plan-decomposer calls `lookup --file`). Revisit when a piping consumer exists.
- Exit codes carry semantics: `0` clean, `1` tool error, `2` validation found issues (grep convention).

### CLI breaking change — accepted

`index` becomes mandatory in v0.2.0. The bare-path form `rust-workspace-map <PATH>` is removed entirely (not aliased, not deprecation-warned). Internal-only tool, no external users to migrate.

### Cache / MCP / daemon / eval harness — explicitly out of MVP scope

These are CRG-shaped maturity items. Built only when (a) the MVP has shipped, (b) the pipeline is using it for at least 3 phases, and (c) measured pain points justify each one individually. Recorded in §"Out of MVP scope" below.

---

## MVP surface

### CLI

```
rust-workspace-map index  [PATH] [-o FILE] [--validate]        # --validate runs OrphanFile + DeadReExport checks
rust-workspace-map lookup [PATH] (--symbol NAME | --file PATH)
```

### Schema additions

Field additions are additive (no struct fields removed). The `cross_references.types` map key format changes from short-name to fully-qualified path — this is a **breaking change** to the JSON output and is intentional (v0.2.0 breaking-change window, same commit as the `index` subcommand mandate).

Two principles drive the shape:

1. **Reuse `WorkspaceMap.errors: Vec<ErrorEntry>` for new diagnostics.** It already carries `severity: ErrorSeverity::{Error, Warning}`, `kind`, `file`, `line`, `message`, `context`, `cause`. A parallel `warnings: Vec<Warning>` channel duplicates that pipeline; new validate rules emit `ErrorEntry { severity: Warning, kind: DiagnosticKind::OrphanFile, … }` into the existing collection.
2. **Migrate `ErrorEntry.kind: String → kind: DiagnosticKind`.** Today the field is stringly-typed. The MVP introduces a typed enum covering both existing values (currently passed as strings — `OrphanedModule`, `TomlParseError`, `MissingCrateRoots`, `ModuleTreeError`) and new ones (`OrphanFile`, `DeadReExport`). Serde-rename keeps the JSON output a string for backward compatibility; the change is purely internal typing.

```rust
// in src/schema.rs
pub struct WorkspaceMap {
    pub workspace: WorkspaceInfo,
    pub crates: Vec<CrateInfo>,
    pub cross_references: CrossReferences,
    #[builder(default)]
    pub symbols:    BTreeMap<String, SymbolEntry>,   // NEW — canonical-path keyed ("crate::module::Name")
    #[builder(default)]
    pub name_index: BTreeMap<String, Vec<String>>,   // NEW — short name → list of canonical paths (collision support)
    #[builder(default)]
    pub files:      BTreeMap<String, FileEntry>,     // NEW — keyed on workspace-relative file path
    pub errors: Vec<ErrorEntry>,                     // existing — also receives validate findings
    pub workspace_root: PathBuf,                     // existing
}

pub struct SymbolEntry {     // map key = "crate::module::Name"
    pub crate_name: String,
    pub module: String,
    pub file: String,
    pub line: usize,
    pub kind: ItemKind,
    // Deferred until a measured workflow cites them:
    //   - re_exported_at: Vec<{file, line}>
    //   - imported_by:    Vec<{crate_name, file, line}>
    //   - confidence:     EXTRACTED | INFERRED | AMBIGUOUS  (no inference path exists in MVP)
    // Crate-granularity imported_by/exported_by already lives in CrossReferences.types.
    // Key format: root-module items = "crate::Name" (2 segments);
    // nested-module items = "crate::module::Name" (3+ segments).
}

// The `cross_references.types` map is re-keyed from short-name to fully-qualified path
// ("crate::module::Name") to avoid silent data loss on name collisions across crates.

pub struct FileEntry {
    pub module_path: String,                 // joins back to the matching ModuleInfo
    pub parent_module_file: Option<String>,  // critical for plan-decomposer wiring check
    pub is_crate_root: bool,
    // Deliberately *not* duplicating `crate_name`, `submodules`, or `exports` —
    // they already exist in the hierarchical view via `module_path → ModuleInfo`.
    // This keeps the `files` index small while still giving agents an O(1) hop
    // from file path to the structural facts only available through this index.
    // Inline modules (mod foo { ... }) share their parent's file.
    // They are excluded from the `files` index to avoid one-file-to-many-module key collisions.
}

// New typed kind for ErrorEntry. Variants serde-rename to existing string values
// where applicable so JSON output stays stable.
pub enum DiagnosticKind {
    // Existing string-valued kinds, now typed:
    OrphanedModule,         // serde rename: "orphaned_module"
    TomlParseError,         // serde rename: "toml_parse_error"
    MissingCrateRoots,      // serde rename: "missing_crate_roots"
    ModuleTreeError,        // serde rename: "module_tree_error"
    // New kinds emitted by `validate`:
    OrphanFile,             // .rs in src/ with no `pub mod` / `mod` in parent — fires on category #1
    DeadReExport,           // `pub use foo::Bar` where Bar isn't found — fires on category #2
}
```

**Note on flat indexes vs `lookup`.** The plan keeps both: indexes inside the JSON output *and* the `lookup` subcommand. They cover overlapping access patterns — direct map-load by an agent, and targeted CLI queries respectively. Primary access pattern is TBD; revisit at the 3-phase castep-cell-io review and drop whichever path proves redundant.

### `validate` rules — two, with two more deferred

| Rule | Fires when | Pipeline category |
|---|---|---|
| `OrphanFile` | after `build_module_tree` returns for a crate, collect all `ModuleInfo.file` paths (excluding `<unresolved>`); walk the crate's `src/` for all `*.rs` files; any `.rs` on disk absent from the module-tree file set is orphan. Excludes `build.rs`, `src/bin/*.rs`, and test/example directories. | #1 (~40%) |
| `DeadReExport` | a `pub use` path whose target cannot be resolved within the workspace. Algorithm: (1) resolve `crate::`, `self::`, `super::` prefixes; (2) if the first path segment is not a workspace member crate, skip (external re-export — not dead); (3) if the path is a glob (`pub use path::*`), skip (cannot evaluate); (4) for remaining intra-workspace paths, check whether the named item exists in any module reachable through the path prefix. Transitive re-export chains are not traced in MVP. | #2 (~20%) |

Each finding becomes an `ErrorEntry { severity: Warning, kind: DiagnosticKind::OrphanFile | DeadReExport, file, line, message, … }` in `WorkspaceMap.errors`. Message includes a fix hint (e.g., `add 'pub mod foo;' to core/src/lib.rs`). Conservative recall is policy: false negatives over false positives.

**Deferred from MVP:**

- `UnreachablePub` — a `pub` item with no path to a public crate root and no `pub use` rescuing it. The plan originally cited this as "partial #3," but #3 (incomplete consumer updates) and unreachable-pub describe negligibly overlapping problem shapes. Defer until a measured plan-execution failure traces to it.
- `UnsupportedLayout` warning — `mod foo { … }` and `#[path = ...]` are out of scope for AST analysis. **Document the limitation in `README.md`** under "What this tool does and doesn't analyze," rather than emitting a structured diagnostic the consumer cannot act on.

### `lookup` semantics

- `--symbol Task` → resolves through `name_index["Task"]`; if it points to one canonical path, emits the matching `SymbolEntry`. If many, emits the list of canonical paths plus disambiguation hints. Exit `0` on found, `1` on not-found.
- `--file core/src/task.rs` → looks up `FileEntry` and scans `crates[].modules[]` for **all** `ModuleInfo` entries whose `.file` matches the requested path. Returns `{ file_entry: FileEntry, primary_module: ModuleInfo, inline_modules: Vec<ModuleInfo> }` where `primary_module` is the entry whose `path` matches `FileEntry.module_path`, and `inline_modules` are all other entries sharing the same `.file` (e.g. `mod tests { … }` in `lib.rs`). The `files` index still excludes inline modules (avoids key collisions); the join step recovers them.

This is the *targeted query* form — the plan-decomposer asks one specific question and gets one specific answer, no JSON-dump-in-context.

`lookup` is a human CLI convenience for the MVP. Pipeline agents should embed flat indexes in context (single JSON load) rather than making per-symbol CLI calls — each `lookup` invocation performs a full workspace scan. Revisit `lookup` as a pipeline tool after the cache layer ships.

---

## Pipeline integrations shipped *with* the MVP

The user's reframing makes pipeline integration part of the MVP, not a follow-up phase. The MVP isn't done until the pipeline actually uses it.

| Pipeline file | Change |
|---|---|
| `skills/compile-plan/SKILL.md` | **Authoritative gate.** Add a pre-check step: run `rust-workspace-map index --validate <project>`. Block plan execution on any `OrphanFile` or `DeadReExport` finding (exit code 2). This is deterministic — no agent compliance required. |
| `agents/plan-decomposer.md` | **Soft suggestion only.** Add the **Module Wiring Check** guidance from `feature-requests/reduce_recurring_problems.md` §3.1, recommending — not requiring — that the agent call `rust-workspace-map lookup --file <parent>` when introducing a new file, to confirm intended siblings. Optionally call `lookup --symbol <name>` to check for name collisions on a planned re-export. The plan-decomposer's correctness is *not* gated on this; the deterministic gate lives in `compile-plan`. |

The `compile-plan` pre-check is what turns the binary from a CLI demo into a real failure-prevention tool. The plan-decomposer suggestion is upstream defense-in-depth: free if the agent follows it, harmless if it doesn't, never the sole gate.

The other integrations from the original design (`enrich-plan-gather` Step 2 replacement, `review-pr-gather` `diff` ground truth) are explicitly **deferred until after the MVP has been used through ≥3 phases of `castep-cell-io` work** — that gives concrete data on whether the cheaper integrations close enough of the failure rate to make the more ambitious ones worth building.

---

## Files to create / modify

### `rust-workspace-map/`

| File | Action |
|---|---|
| `src/schema.rs` | Add `SymbolEntry` (slim — see schema block), `FileEntry` (slim — three fields), `DiagnosticKind` enum. Extend `WorkspaceMap` with `symbols`, `name_index`, `files` BTreeMap fields. Migrate `ErrorEntry.kind: String → kind: DiagnosticKind` (serde-renamed to keep JSON output stable). **Do not add** `Confidence`, `Warning`, `WarningKind`, `ImportSite`, `ReExportSite`, or `warnings: Vec<Warning>`. |
| `src/lib.rs` | After existing `cross_refs::compute`, call `indexes::derive_from_crates(&crate_infos)` and pass results to `WorkspaceMap::builder()`. Run validation (if `--validate`) after construction, merging findings into `errors`. Update emit-sites in this file to use the new `DiagnosticKind` variants. Set `WorkspaceInfo.root` to the discovered workspace root path (currently hardcoded to `"."`). Fix dropped-errors bug at lines 132–137: collect `errs` from both `Some` and `None` branches of the crate-results drain loop (currently the `None` branch silently discards errors from failed crates). |
| `src/main.rs` | Switch to `clap` subcommands: `index` (with `--validate` flag), `lookup`. Bare-path form removed. Each subcommand takes a path. **No `--from-stdin`** — deferred. |
| `src/lookup.rs` | NEW — pure function over `&WorkspaceMap` implementing `--symbol` and `--file` filters. `--file` looks up `FileEntry` then scans all `ModuleInfo` entries sharing the same `.file`, returning `primary_module` + `inline_modules`. |
| `src/validate.rs` | Validation is merged into `index --validate`. No separate subcommand. The `OrphanFile` and `DeadReExport` logic lives in `src/validate.rs` as a function called by `lib.rs::run()` when `--validate` is set. |
| `src/indexes.rs` | NEW — pure function `derive_from_crates(&[CrateInfo]) -> (symbols, name_index, files)` in one pass over `crates[].modules[]`. Called before `WorkspaceMap` builder. No second AST traversal. |
| `src/module_tree.rs`, `src/file_parser.rs` | Update emit-sites to construct `ErrorEntry` with `kind: DiagnosticKind::OrphanedModule` etc. Fix error-vec cloning at line 252 (O(n²) memory waste) and fragile `unwrap_or` at line 33. |
| `src/cross_refs.rs` | No structural changes for the MVP. The crate-granularity `imported_by`/`exported_by` already collected stays as-is. File:line granularity per symbol is deferred until a measured workflow cites it. |
| `tests/fixtures/sample-workspace/` | Add `bad-orphan/` and `bad-dead-reexport/` sub-fixtures. |
| `tests/integration_test.rs` | (a) Rewrite **every** existing invocation from the bare-path form to `index <path>` (this is the breaking-change call-site sweep). (b) Add tests for `validate` exit code 2 on the new fixtures. (c) Add tests for `lookup --symbol` (single + ambiguous + not-found) and `lookup --file`. |
| `README.md` | Document subcommand surface; add a **"What this tool does and doesn't analyze"** section explicitly noting that inline `mod foo { … }` and `#[path = ...]` items are not analyzed. Add an "Inspiration" section crediting `tirth8205/code-review-graph` (MIT) and stating the differentiation: Rust-only via `syn`, native `pub`/`pub use`/`mod` semantics, one-shot CLI, no daemon/MCP/SQLite. |

### Existing utilities reused (do NOT reinvent)

- `src/cargo_info.rs::parse_cargo_toml` — existing dep + package + edition parser.
- `src/module_tree.rs::build_module_tree` — already resolves `mod foo;` to `foo.rs` / `foo/mod.rs`. **Phase-1 `OrphanFile` rule reuses this resolver in reverse**: walk every `.rs` under the crate source dir, check whether the resolver included it.
- `src/cross_refs.rs::compute` — already populates `crossReferences.types` with crate-granularity `imported_by`/`exported_by` keyed by short symbol name. The MVP **does not** extend its granularity (file:line per-symbol is deferred). The `name_index` flat index is largely a re-shaping of the same data.
- `src/render.rs::render_to_writer` — single rendering path, used by all subcommands.
- `src/file_parser.rs` — `syn` parsing wrapper, no changes.
- `bon::Builder` derive pattern — already idiomatic in the codebase; new types use it for consistency.

### `rust-development-pipeline/`

| File | Action |
|---|---|
| `skills/compile-plan/SKILL.md` | **Authoritative gate.** Add `validate` pre-check that blocks on `OrphanFile` / `DeadReExport` findings (exit code 2). |
| `agents/plan-decomposer.md` | **Soft suggestion only.** Add Module Wiring Check section recommending `rust-workspace-map lookup --file <parent>` and `lookup --symbol <name>` calls when introducing new files / re-exports. Phrase as guidance, not a procedural mandate. |

---

## Verification (MVP done = all of these green)

1. **Build green**: `cargo build --release` in `rust-workspace-map/`.
2. **Call-site sweep**: every invocation in `tests/integration_test.rs` rewritten from the bare-path form to `index <path>` in the same commit as the breaking change. README examples and any wrapper scripts in `rust-development-pipeline/` updated in lockstep. This is the flag-day checklist for D1.
3. **Existing tests green**: all 8 integration tests in `rust-workspace-map/tests/integration_test.rs` still pass after the rewrite, with the additive schema.
4. **DiagnosticKind migration green**: the existing `orphaned_module` warning emitted by `module_tree.rs` still serializes as the string `"orphaned_module"` in JSON (verified by an integration test on a known-orphan fixture). No JSON-output regression.
5. **New unit fixtures**: `bad-orphan/` triggers an `OrphanFile` finding naming the parent file; `bad-dead-reexport/` triggers a `DeadReExport` finding. `index --validate` exits `2` in both cases. Findings appear in `WorkspaceMap.errors` with `severity: Warning`.
6. **Lookup**: against the existing `sample-workspace`, `lookup --symbol Task` returns the matching `SymbolEntry`; `lookup --file core/src/task.rs` returns the joined `FileEntry`+`ModuleInfo`; `lookup --symbol DoesNotExist` exits `1`.
7. **Real-world dry-run + cache baseline**: `rust-workspace-map index --validate /Users/tony/programming/castep-cell-io` and `rust-workspace-map index --validate /Users/tony/programming/castep-cell-io` (303 files) run to completion. Record wall-time in `notes/` as the **0.1.0→0.2.0 cache baseline**. The cache layer (currently deferred) is allowed only if a future change pushes wall-time past 2× this baseline AND `compile-plan` runs index --validate ≥3× per phase. Estimated baseline: 100–400 ms; the original 2 s threshold is unlikely to ever trigger.
8. **Pipeline smoke**: in `rust-development-pipeline`, run `compile-plan` against a synthetic plan that creates a file without a `pub mod`. The pre-check blocks (via `index --validate`). Run another synthetic plan that introduces a `pub use` to a nonexistent symbol. The pre-check blocks (via `index --validate`). Both behaviors are deterministic — no LLM compliance involved.

---

## Out of MVP scope (explicitly deferred — built only when the pipeline measurably needs them)

These are recorded so they don't get rebuilt as ad-hoc additions. Each requires concrete pipeline pain to justify:

**Cut from this revision of the plan (after KISS review):**

- **`UnreachablePub` validate rule** — was originally framed as "partial #3," but #3 (incomplete consumer updates) and unreachable-pub describe negligibly overlapping problem shapes. Defer until at least one measured plan-execution failure traces to an unreachable-pub case the rule would have caught.
- **`UnsupportedLayout` warning** — covered by a README sentence ("What this tool does and doesn't analyze") rather than a structured diagnostic the consumer cannot act on.
- **`Confidence` enum (`EXTRACTED`/`INFERRED`/`AMBIGUOUS`)** — every value would be `EXTRACTED` in the MVP because no inference path exists. Add the day a non-AST data source is introduced.
- **Per-symbol `imported_by` / `re_exported_at` at file:line granularity** (`ImportSite`, `ReExportSite` types) — the three rules don't need them and crate-granularity already lives in `CrossReferences.types`. Add when a measured workflow cites "find references" with file:line precision.
- **`--from-stdin` on `validate` and `lookup`** — would force `serde::Deserialize` on every schema type plus a JSON-roundtrip stability commitment, with no current piping consumer. Add when a third caller requests it.
- **Parallel `Vec<Warning>` channel** — collapsed into the existing `WorkspaceMap.errors` with the typed `DiagnosticKind`. No reason to revive the parallel channel.
- **Hard-mandate plan-decomposer `lookup` calls** — soft suggestion is the chosen design. Hard mandates depending on LLM compliance fail silently; the deterministic gate is `compile-plan`'s pre-check.

**Originally deferred (carried forward unchanged):**

- **`diff <base-ref>` subcommand** — defer until `review-pr` regex parsing produces wrong results that an AST-based diff would have caught. The bar: at least one fix round attributable to a missed structural change in review.
- **`--compact` and scope filters (`--crates`, `--files`)** — defer until measured token use exceeds 50% of an agent's context window on `castep-cell-io`-size workspaces.
- **File-mtime-keyed cache** at `~/.cache/rust-workspace-map/<workspace-hash>.json` — defer until verification step 7 measures wall-time past 2× the recorded baseline, OR the `compile-plan` pre-check is observed running ≥3× per phase.
- **`serve` mode / MCP server** — defer until the pipeline grows its first MCP server. The pipeline currently has zero (`/Users/tony/programming/rust-development-pipeline/.mcp.json` does not exist).
- **Derive-macro expansion** (`bon::Builder`, `serde`, `thiserror`) — defer until a real plan-decomposer task fails because the symbol it needed was macro-generated and missing from `symbols`. Track misses by adding a "no symbol found, but file matches a `#[derive]` site" hint to `lookup`.
- **Eval harness (CRG `eval/`-style A/B replay)** — defer indefinitely. The MVP does not need a published reduction number; it needs the pipeline's fix-task ratio to drop. That metric already exists in `notes/pr-reviews/`.
- **NDJSON streaming output** — defer until a streaming consumer exists.
- **Multi-language Tree-sitter coverage** — out of scope permanently. Rust-only is the moat.

---

## CRG attribution

In `README.md` of `rust-workspace-map`, add a short "Inspiration" section:

> The subcommand surface and conservative-recall validation policy are inspired by [`tirth8205/code-review-graph`](https://github.com/tirth8205/code-review-graph) (MIT). This tool is differently shaped: Rust-only via `syn`, models `pub`/`pub use`/`mod` semantics natively, and is a one-shot CLI rather than a daemon-backed graph store. CRG's edge-confidence stance (`EXTRACTED`/`INFERRED`/`AMBIGUOUS`) is *not* yet adopted — the MVP performs only AST extraction, so every value would be `EXTRACTED`. Revisit if a non-AST data source is introduced.

---

## Locked decisions (recorded so they don't drift)

- **D1**: `index` mandatory immediately. Bare-path form removed in v0.2.0. No deprecation alias. Call-site sweep is part of the same commit (verification step 2). The `cross_references.types` re-keying (short-name → fully-qualified path) is also a breaking change and ships in this same commit.
- **D2**: MVP scope =
  - **subcommands**: `index` (with `--validate` flag), `lookup` (no `--from-stdin`)
  - **schema additions**: slim `SymbolEntry`, slim `FileEntry`, three flat indexes (`symbols`, `name_index`, `files`), `DiagnosticKind` enum (replacing `ErrorEntry.kind: String`), re-key `cross_references.types` from short-name to fully-qualified path
  - **schema *not* added**: `Confidence`, `Warning`, `WarningKind`, `ImportSite`, `ReExportSite`, parallel `warnings: Vec<Warning>`
  - **validate rules**: `OrphanFile`, `DeadReExport` (5-case algorithm; run via `index --validate`)
  - **pipeline integrations**: `compile-plan` pre-check (deterministic gate); `plan-decomposer` Module Wiring Check (soft suggestion only)
- **D3**: Cache layer deferred. Gate: wall-time on `castep-cell-io` exceeds 2× the verification-step-7 baseline AND `compile-plan` runs validate ≥3× per phase.
- **D4**: Pipeline integrations ship *with* the MVP, not after — the tool isn't done until the pipeline uses it.
- **D5**: Flat indexes and `lookup` are belt-and-suspenders. Primary access pattern (raw-map reading vs `lookup` calls) to be determined empirically over ≥3 castep-cell-io phases; the redundant path may be dropped at that review.
- **D6**: Per-symbol reverse data (file:line `imported_by`, `re_exported_at`) is gated on a measured workflow citation. Adding it speculatively would ship unread data and obligate maintenance forever.
