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

---

## Architectural commitments (locked)

### KISS faithfulness — very faithful

The MVP ships only what closes a measured pipeline failure or directly enables a pipeline integration that closes one. Anything else is recorded as deferred and built only when measured pain re-surfaces it.

### Single binary with subcommands (not multiple binaries)

Decomposition into separate binaries (`rwm-index`, `rwm-validate`) is rejected. Pipeline skills make one tool call per step — they do not chain N tools. A single binary with subcommands (`cargo`, `git`, `gh`, `kubectl` idiom) gives one Cargo build, one install path, one `--help`. The Unix property is preserved *inside* the binary:

- Each subcommand has one job and writes self-contained JSON to stdout.
- Consumer subcommands (`validate`, `lookup`) accept `--from-stdin` to read a previously-built map.
- Exit codes carry semantics: `0` clean, `1` tool error, `2` validation found issues (grep convention).
- `rust-workspace-map index . | rust-workspace-map validate --from-stdin | jq …` works.

### CLI breaking change — accepted

`index` becomes mandatory in v0.2.0. The bare-path form `rust-workspace-map <PATH>` is removed entirely (not aliased, not deprecation-warned). Internal-only tool, no external users to migrate.

### Cache / MCP / daemon / eval harness — explicitly out of MVP scope

These are CRG-shaped maturity items. Built only when (a) the MVP has shipped, (b) the pipeline is using it for at least 3 phases, and (c) measured pain points justify each one individually. Recorded in §"Out of MVP scope" below.

---

## MVP surface

### CLI

```
rust-workspace-map index    [PATH] [-o FILE]                  # current behavior, behind subcommand
rust-workspace-map validate [PATH | --from-stdin] [-o FILE]   # emits warnings only
rust-workspace-map lookup   [PATH | --from-stdin] (--symbol NAME | --file PATH)
```

### Schema additions (additive only — no removals from existing schema)

```rust
// in src/schema.rs
pub struct WorkspaceMap {
    pub workspace: WorkspaceInfo,
    pub crates: Vec<CrateInfo>,
    pub cross_references: CrossReferences,
    #[builder(default)]
    pub symbols:    BTreeMap<String, SymbolEntry>,   // NEW — canonical-path keyed
    #[builder(default)]
    pub name_index: BTreeMap<String, Vec<String>>,   // NEW — short name → list of paths (collision support)
    #[builder(default)]
    pub files:      BTreeMap<String, FileEntry>,     // NEW — flat reverse index
    #[builder(default)]
    pub warnings:   Vec<Warning>,                    // NEW — populated only by `validate`
    pub errors: Vec<ErrorEntry>,                     // existing
    pub workspace_root: PathBuf,                     // existing
}

pub struct SymbolEntry {     // map key = "crate::module::Name"
    pub crate_name: String,
    pub module: String,
    pub file: String,
    pub line: usize,
    pub kind: ItemKind,
    pub re_exported_at: Vec<ReExportSite>,   // {file, line}
    pub imported_by:    Vec<ImportSite>,     // {crate_name, file, line}
    pub confidence: Confidence,              // EXTRACTED | INFERRED | AMBIGUOUS — CRG-style
}

pub struct FileEntry {
    pub crate_name: String,
    pub module: String,
    pub is_crate_root: bool,
    pub parent_module_file: Option<String>,  // critical for plan-decomposer wiring check
    pub submodules: Vec<String>,
    pub exports: Vec<String>,                // names only; full record lives in `symbols`
}

pub enum WarningKind {
    OrphanFile,        // .rs in src/ with no `pub mod` / `mod` in parent — fires on category #1
    DeadReExport,      // `pub use foo::Bar` where Bar isn't found — fires on category #2
    UnreachablePub,    // `pub` with no public path AND no rescuing re-export — partial #3
    UnsupportedLayout, // inline `mod {…}` or `#[path = …]` — explicit "we don't analyze this" rather than false-positive
}
```

### `validate` rules — three only, plus the unsupported-layout disclosure

| Rule | Fires when | Pipeline category |
|---|---|---|
| `OrphanFile` | a `.rs` exists under a crate's source dir but no `mod foo;` / `pub mod foo;` declaration exists in the parent (per standard Rust `foo.rs` next to `lib.rs`/`mod.rs` or `foo/mod.rs` resolution). | #1 (~40%) |
| `DeadReExport` | a `pub use a::b::C` where no item `C` exists in any module of `a` (or transitively through it). | #2 (~20%) |
| `UnreachablePub` | a `pub` item where (a) no path to a public crate-root exists AND (b) no `pub use` rescues it to a public path. `pub(crate)` items are exempt. | partial #3 |
| `UnsupportedLayout` | inline `mod foo { … }` or `#[path = …]` encountered — disclosed, not analyzed. | none |

Each warning carries `file`, `line`, `kind`, `msg`, plus a fix hint (e.g., `add 'pub mod foo;' to core/src/lib.rs`). Conservative recall is policy: false negatives over false positives.

### `lookup` semantics

- `--symbol Task` → emits the `SymbolEntry` if `name_index["Task"]` resolves to one path; emits a list with disambiguation hints if many. Exit `0` on found, `1` on not-found.
- `--file core/src/task.rs` → emits the `FileEntry`.

This is the *targeted query* form — the plan-decomposer asks one specific question and gets one specific answer, no JSON-dump-in-context.

---

## Pipeline integrations shipped *with* the MVP

The user's reframing makes pipeline integration part of the MVP, not a follow-up phase. The MVP isn't done until the pipeline actually uses it.

| Pipeline file | Change |
|---|---|
| `agents/plan-decomposer.md` | Add the **Module Wiring Check** rule from `feature-requests/reduce_recurring_problems.md` §3.1 *and* require the agent to call `rust-workspace-map lookup --file <parent>` before emitting a new-file `[[changes]]` entry. The returned `submodules` list is cited in the plan rationale. For new symbols intended as crate-root re-exports, require `rust-workspace-map lookup --symbol <name>` to confirm absence of name collision. |
| `skills/compile-plan/SKILL.md` | Add a pre-check step: run `rust-workspace-map validate <project>`. Block plan execution on any `OrphanFile` or `DeadReExport` warning. `UnreachablePub` is reported but does not block (conservative-recall stance). |

These two integrations are what turn the binary from a CLI demo into the actual MVP. Without them, the tool exists but the pipeline still produces #1/#2 errors.

The other integrations from the original design (`enrich-plan-gather` Step 2 replacement, `review-pr-gather` `diff` ground truth) are explicitly **deferred until after the MVP has been used through ≥3 phases of `castep-cell-io` work** — that gives concrete data on whether the cheaper integrations close enough of the failure rate to make the more ambitious ones worth building.

---

## Files to create / modify

### `rust-workspace-map/`

| File | Action |
|---|---|
| `src/schema.rs` | Add `SymbolEntry`, `FileEntry`, `Warning`, `WarningKind`, `Confidence`, `ImportSite`, `ReExportSite`, `name_index`. Extend `WorkspaceMap`. |
| `src/lib.rs` | After existing `crate_infos` build, derive `symbols` / `files` / `name_index` flat indexes from the same data (single pass, no new parsing). |
| `src/main.rs` | Switch to `clap` subcommands: `index`, `validate`, `lookup`. Bare-path form removed. Add `--from-stdin` to consumer subcommands. |
| `src/validate.rs` | NEW — implement the 3+1 rules. Pure function over `&WorkspaceMap` returning `Vec<Warning>`. |
| `src/lookup.rs` | NEW — implement `--symbol` and `--file` filters. Pure function. |
| `src/cross_refs.rs` | Extend existing crate-granularity `imported_by`/`exported_by` to include `file:line` per the new `ImportSite`/`ReExportSite` types. Reuses `Import.line` already collected — no second AST traversal. |
| `tests/fixtures/sample-workspace/` | Add `bad-orphan/` and `bad-dead-reexport/` sub-fixtures. |
| `tests/integration_test.rs` | Add tests for `validate` exit code 2, `lookup --symbol` (single + ambiguous), `lookup --file`, and the `index | validate --from-stdin` pipe. |
| `README.md` | Document subcommand surface; add an "Inspiration" section crediting `tirth8205/code-review-graph` (MIT) and stating the differentiation: Rust-only via `syn`, native `pub`/`pub use`/`mod` semantics, one-shot CLI, no daemon/MCP/SQLite. |

### Existing utilities reused (do NOT reinvent)

- `src/cargo_info.rs::parse_cargo_toml` — existing dep + package + edition parser.
- `src/module_tree.rs::build_module_tree` — already resolves `mod foo;` to `foo.rs` / `foo/mod.rs`. **Phase-1 `OrphanFile` rule reuses this resolver in reverse**: walk every `.rs` under the crate source dir, check whether the resolver included it.
- `src/cross_refs.rs::compute` — already populates `crossReferences.types`. The MVP extends granularity within the existing pass.
- `src/render.rs::render_to_writer` — single rendering path, used by all subcommands.
- `src/file_parser.rs` — `syn` parsing wrapper, no changes.
- `bon::Builder` derive pattern — already idiomatic in the codebase; new types use it for consistency.

### `rust-development-pipeline/`

| File | Action |
|---|---|
| `agents/plan-decomposer.md` | Add Module Wiring Check section with explicit `rust-workspace-map lookup` invocations. |
| `skills/compile-plan/SKILL.md` | Add `validate` pre-check step that blocks on `OrphanFile`/`DeadReExport`. |

---

## Verification (MVP done = all of these green)

1. **Build green**: `cargo build --release` in `rust-workspace-map/`.
2. **Existing tests green**: all 8 integration tests in `rust-workspace-map/tests/integration_test.rs` still pass with the additive schema.
3. **New unit fixtures**: `bad-orphan/` triggers an `OrphanFile` warning naming the parent file; `bad-dead-reexport/` triggers a `DeadReExport` warning. `validate` exits `2` in both cases.
4. **Lookup**: against the existing `sample-workspace`, `lookup --symbol Task` returns the matching entry; `--file core/src/task.rs` returns the matching `FileEntry`; `--symbol DoesNotExist` exits `1`.
5. **Pipe composition**: `index . | validate --from-stdin` produces the same `warnings` array as `validate .`.
6. **Real-world dry-run**: `rust-workspace-map validate /Users/tony/programming/castep-cell-io` (303 files) runs to completion. Record wall-time and warning count in `notes/`. If wall-time >2 s, the cache item below moves from "deferred" to "next iteration."
7. **Pipeline smoke**: in `rust-development-pipeline`, run `compile-plan` against a synthetic plan that creates a file without a `pub mod`. The pre-check blocks. Run another synthetic plan that introduces a `pub use` to a nonexistent symbol. The pre-check blocks.
8. **Plan-decomposer integration test**: a plan-decomposer prompt for a "create new module + use new type" task produces a TOML that includes both the `pub mod` change in `lib.rs` and the consumer-side change, citing `lookup --file` output as evidence.

---

## Out of MVP scope (explicitly deferred — built only when the pipeline measurably needs them)

These are recorded so they don't get rebuilt as ad-hoc additions. Each requires concrete pipeline pain to justify:

- **`diff <base-ref>` subcommand** — defer until `review-pr` regex parsing produces wrong results that an AST-based diff would have caught. The bar: at least one fix round attributable to a missed structural change in review.
- **`--compact` and scope filters (`--crates`, `--files`)** — defer until measured token use exceeds 50% of an agent's context window on `castep-cell-io`-size workspaces.
- **File-mtime-keyed cache** at `~/.cache/rust-workspace-map/<workspace-hash>.json` — defer until verification step 6 measures >2 s wall-time, OR the `compile-plan` pre-check is observed running ≥3× per phase.
- **`serve` mode / MCP server** — defer until the pipeline grows its first MCP server. The pipeline currently has zero (`/Users/tony/programming/rust-development-pipeline/.mcp.json` does not exist).
- **Derive-macro expansion** (`bon::Builder`, `serde`, `thiserror`) — defer until a real plan-decomposer task fails because the symbol it needed was macro-generated and missing from `symbols`. Track misses by adding a "no symbol found, but file matches a `#[derive]` site" hint to `lookup`.
- **Eval harness (CRG `eval/`-style A/B replay)** — defer indefinitely. The MVP does not need a published reduction number; it needs the pipeline's fix-task ratio to drop. That metric already exists in `notes/pr-reviews/`.
- **NDJSON streaming output** — defer until a streaming consumer exists.
- **Multi-language Tree-sitter coverage** — out of scope permanently. Rust-only is the moat.

---

## CRG attribution

In `README.md` of `rust-workspace-map`, add a short "Inspiration" section:

> The subcommand surface, edge-confidence stance (`EXTRACTED`/`INFERRED`/`AMBIGUOUS`), and conservative-recall validation policy are inspired by [`tirth8205/code-review-graph`](https://github.com/tirth8205/code-review-graph) (MIT). This tool is differently shaped: Rust-only via `syn`, models `pub`/`pub use`/`mod` semantics natively, and is a one-shot CLI rather than a daemon-backed graph store.

---

## Locked decisions (recorded so they don't drift)

- **D1**: `index` mandatory immediately. Bare-path form removed in v0.2.0. No deprecation alias.
- **D2**: MVP scope = `index` + `validate` + `lookup` + flat indexes + the two pipeline integrations (`plan-decomposer` Module Wiring Check, `compile-plan` pre-check).
- **D3**: Cache layer deferred to "next iteration"; gate is verification step 6's measured wall-time on `castep-cell-io`.
- **D4**: Pipeline integrations ship *with* the MVP, not after — the tool isn't done until the pipeline uses it.
