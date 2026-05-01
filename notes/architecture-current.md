# Architecture Note — Current Codebase Status
> Generated: 2026-04-29 | Phase: 0.2 (Hardening for Trustworthiness)

---

## Overview

`rust-workspace-map` is a CLI tool that walks a Rust workspace, parses each member crate's public API surface using `syn`, and emits a single structured JSON document designed for LLM consumption. The core design principle is **partial results over abort**: a single crate or module parse failure produces an `ErrorEntry` in the output rather than crashing the whole run.

---

## Module Map

```
src/
├── main.rs         CLI entry point (clap)
├── lib.rs          Pipeline orchestrator (run())
├── schema.rs       All data types + Error enum
├── workspace.rs    Workspace/member discovery
├── cargo_info.rs   Cargo.toml parsing
├── file_parser.rs  Rust AST extraction (syn)
├── module_tree.rs  Recursive module tree construction
├── cross_refs.rs   Cross-crate type reference analysis
└── render.rs       JSON serialization
```

---

## Data Flow

```
CLI args (PATH, -o FILE)
    │
    ▼
workspace::find_workspace_root()   ← walks ancestors for [workspace] Cargo.toml
    │
    ▼
workspace::enumerate_members()     ← parses members[], exclude[], expands globs
    │
    ▼ (rayon par_iter per crate)
┌─────────────────────────────────────────────────────┐
│  cargo_info::parse_cargo_toml()  →  (PackageInfo, DepInfo)         │
│  workspace::resolve_crate_roots() →  [(path, CrateType)]           │
│  module_tree::build_module_tree() →  (Vec<ModuleInfo>, Vec<ErrorEntry>) │
│  path relativization (strip workspace root prefix)                 │
└─────────────────────────────────────────────────────┘
    │
    ▼
sort crates by name (determinism)
    │
    ▼
cross_refs::compute()              ← mutates CrateInfo.cross_crate_imports,
    │                                 returns CrossReferences
    ▼
render::render_to_writer()         ← serde_json pretty to stdout or -o file
```

---

## Module Responsibilities

### `schema.rs` — All types, one place

Defines every serializable and internal type. All output structs use `#[serde(rename_all = "camelCase")]` and `bon::Builder`. `skip_serializing_if = "Vec::is_empty"` suppresses empty arrays throughout.

**Output types:** `WorkspaceMap` → `WorkspaceInfo`, `CrateInfo`, `CrossReferences`, `Vec<ErrorEntry>`  
**Per-crate:** `PackageInfo`, `DepInfo` (normal/dev/workspace_members), `ModuleInfo`, `PublicItem`, `ImplInfo`  
**Error types:** `ErrorEntry` (file, line, message, severity, kind, context, cause) + `ErrorSeverity` + `ErrorContext`  
**Internal-only:** `FileInfo`, `SubmoduleDecl` (not serialized)  
**Library errors:** `schema::Error` (thiserror) — 7 variants covering file I/O, TOML parse, syn parse, workspace structure

The `ErrorEntry.kind` field uses 8 defined string values:
`toml_parse_error`, `syn_parse_error`, `missing_crate_roots`, `orphaned_module`, `module_tree_error`, `missing_workspace_section`, `glob_pattern_error`, `member_not_found`

### `workspace.rs` — Discovery

- `find_workspace_root(path)` — walks `path.ancestors()`, returns first dir whose `Cargo.toml` contains `[workspace]`
- `enumerate_members(root)` — parses workspace TOML, resolves member paths, expands globs via the `glob` crate, applies exclude list, deduplicates and sorts result
- `resolve_crate_roots(crate_dir)` — checks for `src/lib.rs` (→ Lib) and/or `src/main.rs` (→ Bin); returns both if both exist

**Known issue:** missing-member warning uses `eprintln!` rather than returning an `ErrorEntry` (Path Safety goal, phase 0.2).  
**Known issue:** exclude filter calls `path.file_name().unwrap_or_default()` — matches only by directory basename, not full path segment; can silently misfire on deeply-nested members.

### `cargo_info.rs` — Cargo.toml parsing

`parse_cargo_toml(path)` returns `(PackageInfo, DepInfo)`. Missing `[package]` fields fall back to `"unknown"` / `"0.0.0"` / `"2021"`. Workspace deps (`{ workspace = true }`) are separated from normal deps. `crate_type` in the returned `PackageInfo` is always `Lib`; the caller overrides it after calling `resolve_crate_roots`.

### `file_parser.rs` — AST extraction (largest module, ~800 lines)

**Always infallible.** `parse_file(path)` returns `ParsedFile` even on IO or `syn` failure — errors are stored in `ParsedFile.parse_error: Option<SynParseError>`, not propagated. Callers use `build_parse_error_entry()` to convert to `ErrorEntry`.

Extraction functions (all `#[must_use]`, all return sorted results for determinism):
- `extract_public_items` — filters `syn::Visibility::Inherited`, maps to `PublicItem`; sorted by name then line
- `extract_imports` — flattens braced use-trees recursively; sorted by path
- `extract_re_exports` — `pub use` statements only; sorted by export_path
- `extract_submodules` — `mod` declarations; marks `#[cfg(test)]` via token matching; sorted by name
- `extract_impls` — `impl` blocks → `ImplInfo` with fn/type/const items

Internal helpers cover: visibility stringification, generic parameter rendering, full `syn::Type` to string (paths, references, tuples, slices, arrays, pointers, closures, trait objects, impl Trait), field/variant extraction, attribute extraction (derive + doc).

### `module_tree.rs` — Recursive traversal

`build_module_tree(crate_root, crate_name)` is the public entry. It:
1. Parses `crate_root` via `file_parser::parse_file`
2. Builds the root `ModuleInfo`
3. For each non-test submodule, calls `process_submodule` recursively
4. Tracks visited paths in a `HashSet<PathBuf>` to prevent cycles

`process_submodule` handles two cases:
- **Inline modules** (`mod foo { ... }`) — extracts items from AST directly, no file I/O
- **External modules** (`mod foo;`) — resolves via `resolve_module_path` (tries `foo.rs` then `foo/mod.rs`), recurses

Unresolvable external modules produce an `orphaned_module` ErrorEntry and a placeholder `ModuleInfo` with `file = "<unresolved>"`.

**Known issue:** `process_module_info` returns `errors.clone()` — redundant allocation that doubles the error vec for deeply nested trees.  
**Known issue:** `parent_dir` computation uses `.unwrap_or(crate_root)` — semantically wrong if `crate_root` has no parent (root path).

### `cross_refs.rs` — Cross-crate analysis

`compute(crates: &mut [CrateInfo])` takes mutable access to all crates at once. In a single pass:
1. Builds `crate_exports: BTreeMap<String, Vec<(name, kind)>>` from all public items
2. Initializes `TypeRef` entries, populating `exported_by`
3. Scans every import in every module; if the first path segment matches a known crate name (other than self), records a `CrossCrateImport` and updates `TypeRef.imported_by`

Limitation: cross-crate detection is heuristic — it matches the first import segment against crate names. An import like `use serde::Serialize` would be missed unless `serde` itself is a workspace member. Re-exports are not traced across crates.

### `render.rs` — Output

Thin wrappers over `serde_json::to_string_pretty` and `serde_json::to_writer_pretty`. No custom serialization logic.

### `lib.rs` — Pipeline glue

`run()` is the single public function. It orchestrates the full pipeline, collects per-crate `ErrorEntry` vecs, and passes everything through to `WorkspaceMap`. Errors from `find_workspace_root` and `enumerate_members` are fatal (propagated as `anyhow::Error`). Errors from per-crate processing are soft (collected into `WorkspaceMap.errors`).

---

## Error Handling Architecture

| Layer | Mechanism | Fate |
|---|---|---|
| Workspace root not found | `schema::Error::WorkspaceRootNotFound` → `anyhow` | Fatal — exits non-zero |
| Missing `[workspace]` section | `schema::Error::MissingWorkspaceSection` → `anyhow` | Fatal — exits non-zero |
| TOML parse failure (workspace) | `schema::Error::TomlParse` → `anyhow` | Fatal |
| TOML parse failure (crate) | `ErrorEntry { kind: "toml_parse_error" }` | Soft — crate skipped, run continues |
| No crate roots found | `ErrorEntry { kind: "missing_crate_roots" }` | Soft — crate skipped |
| `syn` parse failure | `ErrorEntry { kind: "syn_parse_error" }` | Soft — module empty, siblings continue |
| Orphaned module | `ErrorEntry { kind: "orphaned_module" }` | Soft — placeholder entry |
| Missing workspace member | `eprintln!` (not ErrorEntry) | **Gap** — not in JSON output |

---

## Test Coverage

### Unit tests (inline `#[cfg(test)]` modules)

| Module | Tests |
|---|---|
| `file_parser.rs` | 13 |
| `workspace.rs` | 5 |
| `render.rs` | 3 |
| `cargo_info.rs` | 3 |
| `module_tree.rs` | 4 |
| `cross_refs.rs` | 2 |
| **Total** | **30** |

### Integration tests (`tests/integration_test.rs`, 10 tests)

All integration tests run the compiled binary via `std::process::Command`:

1. `test_sample_workspace_output` — full fixture smoke test (cross-refs, modules, imports)
2. `test_deterministic_output` — byte-identical across two sequential runs
3. `test_missing_path_exits_nonzero` — invalid path → non-zero exit
4. `test_parse_failure_error_entry` — bad Rust syntax → `syn_parse_error` in JSON errors
5. `test_missing_workspace_section` — no `[workspace]` → non-zero exit
6. `test_glob_member_patterns` — `crates/*` glob expansion
7. `test_workspace_with_exclude` — exclude list respected
8. `test_deeply_nested_modules` — 4-level deep module tree (`nested::foo::bar::baz`)
9. `test_reexport_chains` — `pub use inner::Secret` captured in `reExports`
10. `test_output_via_flag` — `-o file` content matches stdout

### Test fixture

`tests/fixtures/sample-workspace/` — two-crate workspace (`core` + `engine`). `engine` imports from `core`. Pre-compiled (debug artifacts present in fixture tree).

---

## Dependencies

| Crate | Role |
|---|---|
| `serde` + `serde_json` | Serialization to JSON |
| `syn` (full, extra-traits) | Rust source parsing |
| `proc-macro2` (span-locations) | Source line numbers from `syn` spans |
| `toml` | Cargo.toml parsing |
| `rayon` | Parallel crate processing |
| `clap` (derive) | CLI argument parsing |
| `bon` | Builder pattern macros for all output structs |
| `thiserror` | Typed `schema::Error` enum |
| `anyhow` | Error chaining in `run()` / `main()` |
| `glob` | Workspace member glob expansion |
| `tempfile` (dev) | Temp directories in unit/integration tests |

---

## Phase 0.2 Goals — Status Summary

| Goal | Description | Status |
|---|---|---|
| G1 Error Visibility Pipeline | Wire `ErrorEntry` end-to-end | Partial — most paths covered, missing-member gap remains |
| G2 Unit Test Suite (≥80% line coverage) | 5 core modules | 30 unit tests exist; coverage not yet measured |
| G3 Path Safety Fixes | 4 `unwrap_or_default` path bugs | Not yet fixed |
| G4 Integration Test Expansion | 3→8+ tests | Done — 10 tests exist |
| G5 Remove Clippy Suppressions | 9 crate-level allows | Partially — `#[warn(clippy::pedantic)]` at crate level; 2× `too_many_lines` + 1× `too_many_arguments` remain inline |

---

## Structural Invariants

- All collections in output are sorted for deterministic JSON (crates by name, public items by name+line, imports by path, re-exports by export_path, submodules by name).
- `WorkspaceMap.errors` is omitted from JSON when empty (`skip_serializing_if`). Same for all `Vec` fields on output structs.
- All paths in output are workspace-relative strings (prefix stripped by `relativize_path`).
- `WorkspaceMap.workspace_root` is `#[serde(skip)]` — internal use only.
- `FileInfo` and `SubmoduleDecl` are `pub` in `schema.rs` but not serialized — internal intermediate types.
