# Codebase State — rust-workspace-map

Recorded: 2026-04-27
Plan file: `/Users/tony/programming/rust-workspace-map/rust-workspace-map-plan.md`
Repo commit: `6a74caa` (HEAD on `main`)

## Summary

This is a **greenfield project**. The plan is a full specification for a crate that does not yet exist. There are zero Rust source files, no `Cargo.toml`, no `src/` directory. The only content in the repository is the plan document itself, a README, a LICENSE, and supporting notes directories.

---

## File: `Cargo.toml` (workspace root)

**Status: DOES NOT EXIST**

### Plan relationship
- Plan says: Create `Cargo.toml` with package `rust-workspace-map`, version `0.1.0`, edition `2024`, dependencies on `serde`, `serde_json`, `syn` (features `full`, `extra-traits`), `walkdir`, `toml`, `rayon`, `anyhow`, `clap` (feature `derive`), `bon`, and dev-dependency `cargo-llvm-cov`.
- Current state: No `Cargo.toml` exists anywhere in the repository.
- Gap: Create the file from scratch per the plan's dependency specification.

---

## File: `src/main.rs`

**Status: DOES NOT EXIST**

### Plan relationship
- Plan says: CLI entry point (~20 lines). Uses `clap` for argument parsing: positional `PATH` (required, path to workspace root or subdirectory), optional `--output`/`-o` (write JSON to file instead of stdout). Calls `lib::run()` and prints result.
- Current state: No `src/` directory exists.
- Gap: Create file from scratch.

---

## File: `src/lib.rs`

**Status: DOES NOT EXIST**

### Plan relationship
- Plan says: Orchestration module. Contains `run(config: Config) -> Result<()>` that executes the pipeline: workspace discovery -> member enumeration -> parallel crate processing (cargo parsing + module tree) -> cross-references -> render JSON. Also contains `#![warn(clippy::pedantic)]`.
- Current state: No file exists.
- Gap: Create file from scratch with the `run()` function and `Config` type.

---

## File: `src/schema.rs`

**Status: DOES NOT EXIST**

### Plan relationship
- Plan says: All data types with `#[derive(Serialize, bon::Builder)]` and `#[serde(rename_all = "camelCase")]`. Types: `WorkspaceMap`, `CrateInfo`, `PackageInfo`, `ModuleInfo`, `PublicItem`, `ImplInfo`, `ReExport`, `Import`, `CrossCrateImport`, `CrossReferences`, `TypeRef`, `FileInfo`, `SubmoduleDecl`, `DepInfo`, `Config`, `ErrorEntry`.
- Current state: No file exists.
- Gap: Create file from scratch with all types. Tests for serialization shape must be written first (TDD).

---

## File: `src/workspace.rs`

**Status: DOES NOT EXIST**

### Plan relationship
- Plan says: Workspace discovery. `find_workspace_root(start_path: &Path) -> Result<PathBuf>` (walk up tree for `Cargo.toml` with `[workspace]` section). `enumerate_members(root: &Path) -> Result<Vec<PathBuf>>` (parse workspace TOML, resolve member paths, respect `exclude` key).
- Current state: No file exists.
- Gap: Create file from scratch. Write failing tests first (TDD Step 2a).

---

## File: `src/cargo_info.rs`

**Status: DOES NOT EXIST**

### Plan relationship
- Plan says: Cargo.toml parsing. `parse_cargo_toml(path: &Path) -> Result<(PackageInfo, DepInfo)>` — pure function reading and parsing TOML, extracting package info and dependencies (normal, dev, workspace members).
- Current state: No file exists.
- Gap: Create file from scratch. Write failing tests first (TDD Step 3a).

---

## File: `src/file_parser.rs`

**Status: DOES NOT EXIST**

### Plan relationship
- Plan says: Extract items/imports/re-exports from `syn` AST. Functions: `parse_file(path: &Path) -> Result<FileInfo>`, `extract_public_items(items: &[Item]) -> Vec<PublicItem>`, `extract_imports(items: &[Item]) -> Vec<Import>`, `extract_re_exports(items: &[Item]) -> Vec<ReExport>`, `extract_submodules(items: &[Item]) -> Vec<String>`, `extract_impls(items: &[Item]) -> Vec<ImplInfo>`. All pure functions, no `syn::visit::Visit`. Handles `#[cfg(test)]` mod skip. On parse failure, warns to stderr and returns empty FileInfo.
- Current state: No file exists.
- Gap: Create file from scratch. Write failing tests first (TDD Step 4a). Target >=85% coverage.

---

## File: `src/module_tree.rs`

**Status: DOES NOT EXIST**

### Plan relationship
- Plan says: Resolve mod declarations to file paths. `build_module_tree(crate_root: &Path, crate_name: &str) -> Result<Vec<ModuleInfo>>` — recursive traversal. `resolve_module_path(parent_dir: &Path, mod_name: &str) -> Option<PathBuf>` — try `{name}.rs` then `{name}/mod.rs`. Functional recursion, no mutable state.
- Current state: No file exists.
- Gap: Create file from scratch. Write failing tests first (TDD Step 5a).

---

## File: `src/cross_refs.rs`

**Status: DOES NOT EXIST**

### Plan relationship
- Plan says: Compute type-to-importer/exporter mapping. `compute(crates: &[CrateInfo]) -> CrossReferences` — pure function using `BTreeMap` for determinism.
- Current state: No file exists.
- Gap: Create file from scratch. Write failing tests first (TDD Step 6a).

---

## File: `src/render.rs`

**Status: DOES NOT EXIST**

### Plan relationship
- Plan says: JSON output. `render_json(map: &WorkspaceMap) -> Result<String>` — serde serialize with 2-space indent, camelCase. `render_to_writer(map: &WorkspaceMap, writer: impl Write) -> Result<()>`. All paths relative to workspace root.
- Current state: No file exists.
- Gap: Create file from scratch. Write failing tests first (TDD Step 7a).

---

## File: `tests/fixtures/sample-workspace/` (integration test fixture)

**Status: DOES NOT EXIST**

### Plan relationship
- Plan says: Test fixture with 2 crates: `core` (defines `pub struct Task`, `pub trait Runner`) and `engine` (imports from `core`, defines `pub struct Pipeline`). Includes modules, re-exports, `#[cfg(test)]` blocks.
- Current state: No `tests/` directory exists.
- Gap: Create fixture workspace directory with both crates. Write integration test first (TDD Step 8a).

---

## Files mentioned in plan but not found

| File | Status |
|---|---|
| `Cargo.toml` | Does not exist |
| `src/main.rs` | Does not exist |
| `src/lib.rs` | Does not exist |
| `src/schema.rs` | Does not exist |
| `src/workspace.rs` | Does not exist |
| `src/cargo_info.rs` | Does not exist |
| `src/file_parser.rs` | Does not exist |
| `src/module_tree.rs` | Does not exist |
| `src/cross_refs.rs` | Does not exist |
| `src/render.rs` | Does not exist |
| `tests/fixtures/sample-workspace/` | Does not exist |

All 11 file paths mentioned in the plan are absent — the project has not been implemented yet.
