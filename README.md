# rust-workspace-map

A Rust binary that produces a structured JSON map of a Rust workspace's public API surface, designed for LLM agents that need O(1) structural lookups rather than interactive LSP exploration or tree traversal.

## Quick start

```bash
# Build
cargo build --release

# Generate a workspace map
rust-workspace-map index /path/to/workspace > map.json

# Generate + validate (blocks on missing pub mod / dead re-exports)
rust-workspace-map index --validate /path/to/workspace

# Look up a symbol by name
rust-workspace-map lookup /path/to/workspace --symbol Task

# Look up a file's module wiring
rust-workspace-map lookup /path/to/workspace --file core/src/lib.rs
```

## Subcommands

### `index [PATH] [-o FILE] [--validate]`

Generates a complete JSON map of the workspace. Walks every crate member, parses each Rust source file with `syn`, builds the module tree, computes cross-crate references, and derives flat indexes for O(1) lookup.

- `-o FILE` — write output to file instead of stdout
- `--validate` — after indexing, run the OrphanFile and DeadReExport checks described below

Exit codes: `0` = clean, `1` = tool error, `2` = validation found issues (only with `--validate`).

### `lookup [PATH] (--symbol NAME | --file PATH)`

Runs a full workspace scan and returns a targeted result for one symbol or one file. This is a human CLI convenience; pipeline agents should embed the full map in context rather than making per-symbol CLI calls (each `lookup` invocation performs a complete workspace scan).

- `--symbol Task` — returns the matching `SymbolEntry` (or an ambiguous-candidates list, or not-found)
- `--file core/src/lib.rs` — returns `FileEntry` + the `ModuleInfo` entries sharing that file (primary + inline modules)

Exit `0` on found, `1` on not-found.

## JSON output structure

The `index` subcommand emits a single JSON object with these top-level keys:

| Key               | Type                                         | Purpose                                                                   |
| ----------------- | -------------------------------------------- | ------------------------------------------------------------------------- |
| `workspace`       | `WorkspaceInfo`                              | Workspace root path and name                                              |
| `crates`          | `Vec<CrateInfo>`                             | Per-crate module trees with items, imports, re-exports, dependencies      |
| `crossReferences` | `CrossReferences`                            | Type-level import/export mapping keyed by canonical path                  |
| `symbols`         | `BTreeMap<CanonicalPath, SymbolEntry>`       | O(1) lookup: canonical path → symbol metadata                             |
| `nameIndex`       | `BTreeMap<String, Vec<CanonicalPath>>`       | O(1) lookup: short name → canonical paths (handles collisions)            |
| `files`           | `BTreeMap<WorkspaceRelativePath, FileEntry>` | O(1) lookup: file path → module wiring info                               |
| `errors`          | `Vec<ErrorEntry>`                            | Diagnostics (parse failures, orphaned modules, and `--validate` findings) |
| `workspaceRoot`   | `PathBuf`                                    | Absolute workspace root path                                              |

### The three flat indexes

The three indexes (`symbols`, `nameIndex`, `files`) are the primary query surface for LLM agents. They pre-compute relationships a human engineer gets through IDE navigation:

| Agent question                                 | Lookup                                                                      | Cost |
| ---------------------------------------------- | --------------------------------------------------------------------------- | ---- |
| "Is `Task` a type? Where is it defined?"       | `nameIndex["Task"]` → `symbols[path]`                                       | O(1) |
| "What module does this file belong to?"        | `files["core/src/lib.rs"].modulePath`                                       | O(1) |
| "Is this file reachable from the module tree?" | `files[path].parentModuleFile` is `Some(...)` (or the file is a crate root) | O(1) |
| "Which crate exports `Task`?"                  | `crossReferences.types["crate::Task"].exporters`                            | O(1) |

This is the fundamental difference from the hierarchical `crates → modules → publicItems` tree alone: the flat indexes eliminate traversal — the agent asks one question, makes one lookup, gets one answer.

## Validation rules (`index --validate`)

The `--validate` flag runs two deterministic checks after indexing. Both emit `ErrorEntry` records with `severity: "warning"`. When either fires, `index` exits `2`.

| Rule           | Fires when                                                                                                                      | Pipeline category                                 |
| -------------- | ------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------- |
| `OrphanFile`   | A `.rs` file exists on disk under `src/` but is not declared in the module tree (no `mod` or `pub mod` statement in its parent) | #1 missing `pub mod` (~40% of pipeline fix tasks) |
| `DeadReExport` | A `pub use` path cannot be resolved within the workspace                                                                        | #2 missing `pub use` (~20% of pipeline fix tasks) |

**Conservative recall policy:** The tool prioritizes false negatives over false positives. External-crate re-exports that go through path resolution before the workspace-member check can produce false positives — record these when encountered and evaluate whether the skip logic needs tightening.

**Derive-companion awareness (prefix-decomposition heuristic):** DeadReExport false positives commonly occur when derive macros like `bon::Builder` generate public companion types (e.g. `CellDocumentBuilder`) that get re-exported but are not present in the symbol index. When a re-exported name is not found, the tool decomposes the name into decreasing prefixes, looking for a base type in the same module whose `SymbolEntry` has non-empty `derive_attrs`. If such a base is found, the finding is suppressed. This approach uses a purely structural signal — zero hardcoded derive names, zero suffix lists — and automatically handles any ecosystem derive that generates named types. The heuristic is deliberately conservative: it prefers a false negative (suppressing a genuinely-dead re-export whose base type coincidentally has derives) over a false positive (flagging every derive-generated companion).

**Known limitation — private base types:** A private struct with a derive macro like `bon::Builder` that generates a public builder type cannot be suppressed by this heuristic. The private base type is absent from the public-only symbol index, so the prefix-decomposition lookup has nothing to match against. This is an accepted limitation — populating the index with private types would change the contract of `--validate` from "public API surface" to "all types", which is a separate concern.

## What this tool does and doesn't analyze

### Does

- All `pub` items: structs, enums, traits, functions, type aliases, macros
- Module tree: `mod` declarations resolved to `{name}.rs` / `{name}/mod.rs`
- Re-exports: `pub use` paths traced through module prefixes (`crate::`, `self::`, `super::`)
- Cross-crate references: which types each crate imports from / exports to other workspace members
- Flat indexes: O(1) symbol-name, file-path, and canonical-path lookups
- `#[cfg(test)] mod tests { ... }` — module is recorded, contents are skipped
- Workspace member discovery via `[workspace] members` in root `Cargo.toml`, including glob patterns (`crates/*`)
- `exclude` key in `[workspace]` — excluded members are removed from the member list

### Doesn't (deferred — built only when the pipeline measurably needs them)

- **Inline modules** — `mod foo { pub struct Bar; }` share their parent's file and are excluded from the `files` index to avoid key collisions. Their items still appear in the hierarchical module tree.
- **`#[path = "..."]` module declarations** — not resolved.
- **Derive macro expansion** — `bon::Builder`, `serde::Serialize` etc. are not expanded. Symbols generated by derives are not in the index.
- **Per-symbol `imported_by` at file:line granularity** — crate-level cross-references only.
- **`diff <base-ref>` subcommand** — no structural diff against another ref.
- **`--compact` / scope filters** — no token-reduction output modes.
- **File-mtime cache** — no incremental rebuild.
- **MCP server / daemon mode** — one-shot CLI only.

For the full deferred-items list, see the pipeline's Phase D plan or the deferred improvements note in this repo.

## Current status (v0.2.0 — MVP)

The MVP implementation is complete. The tool is being integrated into the `rust-development-pipeline` as "Phase D" — the pipeline's `elaborate-directions` and `explore-implement` skills will consume the map to reduce token waste and deterministically block on missing `pub mod` / dead `pub use` before plan execution.

**Integration points proposed for the pipeline:**

1. **`explore-implement` pre-check** — run `rust-workspace-map index --validate <project>` before implementing tasks. Block on exit code 2 (deterministic gate — no LLM compliance required).
2. **`elaborate-directions` context enrichment** — embed the flat indexes in agent context to replace file-by-file LSP exploration with O(1) lookups.

These are described in more detail in the pipeline's `REFACTOR_PLAN.md` (Phase D) and the `feature-requests/` files in this repo.

## Inspiration

The subcommand surface and conservative-recall validation policy are inspired by [`tirth8205/code-review-graph`](https://github.com/tirth8205/code-review-graph) (MIT). This tool is differently shaped: Rust-only via `syn`, models `pub`/`pub use`/`mod` semantics natively, and is a one-shot CLI rather than a daemon-backed graph store. CRG's edge-confidence stance (`EXTRACTED`/`INFERRED`/`AMBIGUOUS`) is _not_ yet adopted — the MVP performs only AST extraction, so every value would be `EXTRACTED`. Revisit if a non-AST data source is introduced.
