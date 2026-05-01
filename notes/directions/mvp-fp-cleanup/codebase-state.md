# Codebase State Report: mvp-fp-cleanup

## 1. Workspace and Crate Structure

**Single crate:** `rust-workspace-map` (v0.2.0, edition 2024).

**Dependencies (production):** `serde`, `serde_json`, `syn` (full + extra-traits), `toml`, `rayon`, `anyhow`, `clap`, `bon`, `thiserror`, `glob`, `proc-macro2` (span-locations), `walkdir`.

**Dev dependencies:** `tempfile`.

**Source file tree:**

```
src/
  main.rs           -- CLI entry point (clap subcommands: index, lookup)
  lib.rs            -- Pipeline orchestrator: build_map(), run()
  schema.rs         -- All data types, Error enum (thiserror), newtypes
  workspace.rs      -- Workspace discovery, member enumeration, crate roots
  cargo_info.rs     -- Cargo.toml parsing
  file_parser.rs    -- AST extraction (syn), ~800 lines, always infallible
  module_tree.rs    -- Recursive module tree construction
  cross_refs.rs     -- Cross-crate type reference analysis
  indexes.rs        -- Flat index derivation (symbols, name_index, files)
  lookup.rs         -- Symbol/file lookup API
  validate.rs       -- OrphanFile and DeadReExport checks + unit tests
  render.rs         -- JSON serialization wrappers
```

## 2. Key Type/Function Signatures (Plan-Touch Points)

### `src/schema.rs` -- `SymbolEntry`

Current (no `derive_attrs` field):
```rust
pub struct SymbolEntry {
    pub crate_name: String,
    pub module: String,
    pub file: String,
    pub line: usize,
    pub kind: ItemKind,
}
```

### `src/file_parser.rs` -- `extract_attrs` (line 456-483)

Already extracts derive attrs into `ItemAttrs.derive: Vec<String>` by splitting on commas and trimming.

### `src/indexes.rs` -- `derive_from_crates` (line 28-67)

Builds `SymbolEntry` in a `.fold()` pipeline — currently ignores `item.attrs.derive`.

### `src/validate.rs` -- `check_dead_reexports` (line 127-173)

```rust
fn check_dead_reexports(
    crate_info: &CrateInfo,
    symbols: &BTreeMap<CanonicalPath, SymbolEntry>,
    crate_names: &HashSet<&str>,
) -> Vec<ErrorEntry>
```

### `src/validate.rs` -- `determine_parent_file` (line 105-124)

```rust
fn determine_parent_file(
    orphan_file: &str,
    crate_name: &str,
    _crate_info: &CrateInfo,  // UNUSED
) -> String {
    let stripped = orphan_file.strip_prefix("src/").unwrap_or(orphan_file);  // HARDCODED
```

### `src/validate.rs` -- `resolve_import_path` (line 177-194)

```rust
fn resolve_import_path(path: &str, crate_name: &str, module_path: &str) -> String
```

## 3. Module Dependency Graph

```
main.rs → lib.rs
            ├── workspace.rs       (schema::Error, CrateType)
            ├── cargo_info.rs      (schema::PackageInfo, DepInfo, Error)
            ├── module_tree.rs     (file_parser, schema::ModuleInfo, ErrorEntry)
            ├── cross_refs.rs      (schema::CrateInfo, CrossReferences, TypeRef)
            ├── indexes.rs         (schema::CrateInfo, CanonicalPath, SymbolEntry)
            ├── validate.rs        (schema::CanonicalPath, DiagnosticKind, ErrorEntry, CrateInfo)
            ├── lookup.rs          (schema::FileEntry, ModuleInfo, SymbolEntry, WorkspaceMap)
            ├── render.rs          (schema::WorkspaceMap)
            └── schema.rs          (standalone — all types)
                  └── file_parser.rs (schema::FileInfo, Import, ReExport, PublicItem, etc.)
```

## 4. Existing Test Structure

**Unit tests** (inline `#[cfg(test)]`): 44 total across all modules.
- `validate.rs`: 5 tests (resolve_import_path variants)

**Integration tests** (`tests/integration_test.rs`): 17 tests, all via binary invocation.
- `test_validate_dead_reexport_exits_2` — fires on `bad-dead-reexport` fixture

**All 44 unit + 17 integration tests pass.** `cargo check` and `cargo clippy -D warnings` are clean.

## 5. Current Validation Logic Detail

### DeadReExport False Positives: Two Root Causes

1. **External-crate re-exports not skipped** (G1): `pub use serde::Serialize;` → DeadReExport false positive. The resolved-path first-segment check against `crate_names` is effectively dead code for bare paths because `resolve_import_path` always prepends crate name.

2. **Derive-generated companion types** (G2): `#[derive(bon::Builder)]` generates public `CellDocumentBuilder` alongside private `CellDocument`. Public-only symbol index has no `CellDocument` entry, so `pub use CellDocumentBuilder` triggers false positive.

## 6. Changes Needed Per File

| File | Changes | Goal |
|------|---------|------|
| `src/schema.rs` | Add `derive_attrs: Vec<String>` to `SymbolEntry` | G2 |
| `src/indexes.rs` | Copy `item.attrs.derive.clone()` into `SymbolEntry.derive_attrs` | G2 |
| `src/validate.rs` | G1: prefix-aware external check; G2: prefix-decomposition heuristic; G3: use `crate_info.root`; Unit tests for G1/G2 | G1,G2,G3 |
| `README.md` | Document private-base-type limitation | G2 |
| `tests/fixtures/bad-orphan/` | Add deeply nested orphan fixture | G4 |
| `tests/integration_test.rs` | Add recursive orphan detection test | G4 |

## 7. Compilation Status

- `cargo check`: Clean
- `cargo clippy -D warnings`: Clean
- `cargo test`: All passing
