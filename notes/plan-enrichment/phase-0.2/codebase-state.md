# Phase 0.2 — Codebase State Snapshot

Captured on 2026-04-28. This documents the current state of every source file mentioned in the phase plan.

---

## File: src/schema.rs

### Public API

**Error enum:**
```rust
#[derive(Debug, thiserror::Error)]
pub enum Error {
    WorkspaceRootNotFound(PathBuf),
    FileRead { path: PathBuf, source: std::io::Error },
    TomlParse { path: PathBuf, source: toml::de::Error },
    SynParse { path: PathBuf, source: syn::Error },
    MemberNotFound(PathBuf),
    GlobPattern(String),
}
pub type Result<T> = std::result::Result<T, Error>;
```

**Config:**
```rust
#[derive(Debug, Clone, bon::Builder)]
pub struct Config {
    pub workspace_path: PathBuf,
    pub output_path: Option<PathBuf>,
}
```

**CrateType enum:** `Lib`, `Bin`, `LibAndBin` (serde rename_all = "camelCase")

**Output structs (all bon::Builder, serde rename_all = "camelCase"):**
- `WorkspaceMap` — fields: `workspace: WorkspaceInfo`, `crates: Vec<CrateInfo>`, `cross_references: CrossReferences`, `errors: Vec<ErrorEntry>` (skip_serializing_if), `workspace_root: PathBuf` (skip)
- `WorkspaceInfo` — fields: `root: String`, `workspace_name: String`
- `CrateInfo` — fields: `name`, `root`, `package: PackageInfo`, `modules: Vec<ModuleInfo>`, `deps: DepInfo`, `cross_crate_imports: Vec<CrossCrateImport>` (skip_serializing_if)
- `PackageInfo` — fields: `name`, `version`, `edition`, `crate_type: CrateType`
- `DepInfo` (Default) — fields: `normal`, `dev`, `workspace_members` (all skip_serializing_if)
- `ModuleInfo` — fields: `path`, `file`, `visibility`, `public_items: Vec<PublicItem>`, `imports: Vec<Import>`, `re_exports: Vec<ReExport>`, `submodules: Vec<String>` (all optional)
- `PublicItem` — fields: `kind: ItemKind`, `name`, `file`, `line: usize`, `attrs: ItemAttrs`, `generics`, `visibility`, `fields`, `variants`, `impls: Vec<ImplInfo>`
- `ItemKind` enum: `Struct`, `Enum`, `Trait`, `Fn`, `Type`, `Macro`
- `ItemAttrs` (Default) — fields: `derive`, `doc`
- `ImplInfo` — fields: `type_: String`, `items: Vec<ImplItem>`
- `ImplItem` — fields: `kind: ImplItemKind`, `name`, `params`
- `ImplItemKind` enum: `Fn`, `Type`, `Const`
- `Import` — fields: `path`, `line`
- `ReExport` — fields: `import_path`, `export_path`, `line`
- `CrossCrateImport` — fields: `import_path`, `target_crate`, `symbol`, `line`
- `CrossReferences` (Default) — field: `types: BTreeMap<String, TypeRef>`
- `TypeRef` — fields: `crate_name`, `kind`, `imported_by`, `exported_by`

**Internal types:**
- `FileInfo` (Default) — `public_items`, `imports`, `re_exports`, `submodules: Vec<SubmoduleDecl>`, `impls`
- `SubmoduleDecl` — `name`, `is_test: bool` (default)

**ErrorEntry (current):**
```rust
pub struct ErrorEntry {
    pub file: String,
    pub line: usize,  // builder(default)
    pub message: String,
}
```

### Module wiring
- Declared as `pub mod schema;` in `lib.rs`
- `pub use schema::Config;` re-export in `lib.rs`

### Plan relationship
- **Plan says:** Add `severity: ErrorSeverity` (enum: `Error`, `Warning`), `kind: String`, `context: Option<ErrorContext>`, `cause: Option<String>` to `ErrorEntry`. Add `ErrorSeverity` enum. Add `ErrorContext` struct with `crate_name`, `module_path`, `line`, `snippet` fields. Add `MissingWorkspaceSection` variant to `Error` enum.
- **Current state:** `ErrorEntry` has only 3 fields (`file`, `line`, `message`). No `ErrorSeverity` enum. No `ErrorContext` struct. `Error` enum has 6 variants but no `MissingWorkspaceSection`.
- **Gap:** All 4 new `ErrorEntry` fields, both new types (`ErrorSeverity`, `ErrorContext`), and the `MissingWorkspaceSection` error variant need to be added. `ErrorEntry` serde serialization annotations and bon::Builder annotations need to be added for the new fields.

---

## File: src/workspace.rs

### Public API

```rust
pub fn find_workspace_root(start_path: &Path) -> Result<PathBuf>
pub fn enumerate_members(root: &Path) -> Result<Vec<PathBuf>>
pub fn resolve_crate_roots(crate_dir: &Path) -> Vec<(PathBuf, CrateType)>
```

### Module wiring
- `pub mod workspace;` in `lib.rs`
- No `pub use` re-exports of workspace items.

### Plan relationship
- **Plan says:** `enumerate_members` should return `Err(Error::MissingWorkspaceSection)` (new variant) when `[workspace]` section or `members` key is absent. `resolve_crate_roots` empty results should become an `ErrorEntry` with `kind = "missing_crate_roots"` in the caller. The `eprintln!` at line 76 (member not found) should become `Error::MemberNotFound`.
- **Current state:** `enumerate_members` uses `.unwrap_or_default()` on lines 46 and 57 — when `[workspace]` or `members` is absent, it silently returns an empty list. `resolve_crate_roots` already returns an empty Vec when no entry points are found (no error handling). The `eprintln!` at line 76–79 logs a warning for missing members but does not return an error.
- **Gap:** `enumerate_members` needs to distinguish "no workspace section" from "workspace section with no members." The missing-member `eprintln!` should return `Error::MemberNotFound`. `resolve_crate_roots` is called in `run()` which currently does nothing with empty results beyond `eprintln!` — the plan wants this converted to an `ErrorEntry` in `run()` (not in `workspace.rs` itself).

---

## File: src/cargo_info.rs

### Public API

```rust
pub fn parse_cargo_toml(path: &Path) -> Result<(PackageInfo, DepInfo)>
```

### Module wiring
- `pub mod cargo_info;` in `lib.rs`
- No `pub use` re-exports.

### Plan relationship
- **Plan says:** Convert `parse_cargo_toml` failures (currently `eprintln!` + `return None` in the caller `run()`) to `ErrorEntry` with `kind = "toml_parse_error"`.
- **Current state:** `parse_cargo_toml` returns `Result<(PackageInfo, DepInfo)>` and already uses `?` to propagate parse errors via the `Error::TomlParse` variant. The caller in `run()` (line 46–55) matches on `Err(e)` and does `eprintln!` + `return None` — errors are swallowed silently.
- **Gap:** No changes needed in `cargo_info.rs` itself. The caller in `main.rs`/`lib.rs` `run()` needs to be updated to convert the error into an `ErrorEntry`.

---

## File: src/file_parser.rs

### Public API

```rust
pub fn parse_file(path: &Path) -> Result<(syn::File, FileInfo)>
pub fn extract_public_items(items: &[syn::Item]) -> Vec<PublicItem>
pub fn extract_imports(items: &[syn::Item]) -> Vec<Import>
pub fn extract_re_exports(items: &[syn::Item]) -> Vec<ReExport>
pub fn extract_submodules(items: &[syn::Item]) -> Vec<SubmoduleDecl>
pub fn extract_impls(items: &[syn::Item]) -> Vec<ImplInfo>
```

### Module wiring
- `pub mod file_parser;` in `lib.rs`
- No `pub use` re-exports.

### Plan relationship
- **Plan says:** Change `parse_file()` to propagate `SynParse` errors as `ErrorEntry` with severity `error`, rather than returning empty results. On parse failure, return structured error data instead of `(empty syn::File, FileInfo::default())`. This applies to all files including inline `#[cfg(test)]` module bodies and external `#[cfg(test)]` module files.
- **Current state:** `parse_file` (lines 18–30) catches `syn::parse_file` errors, prints a warning via `eprintln!`, and returns an empty `syn::File` + default `FileInfo` — the error is completely lost. All extraction functions are pure and return their collections.
- **Gap:** `parse_file` needs to change its return signature or behavior to report parse errors as `ErrorEntry` data alongside results. The plan says: "parse_file returns errors alongside results rather than propagating via `?` on parse failures." This means either a new return type like `Result<(syn::File, FileInfo), ErrorEntry>` or a tuple `(Result<..., ...>, Vec<ErrorEntry>)`.

---

## File: src/module_tree.rs

### Public API

```rust
pub fn resolve_module_path(parent_dir: &Path, mod_name: &str) -> Option<PathBuf>
pub fn build_module_tree(crate_root: &Path, crate_name: &str) -> Result<Vec<ModuleInfo>>
```

**Internal functions (private):**
- `fn build_module_info(...)` — builds ModuleInfo from components
- `fn process_submodule(...)` — processes a single submodule, handles inline vs external
- `fn process_module_items(...)` — extracts FileInfo from inline module items
- `fn process_module_info(...)` — builds module info for a file-path-backed module

### Module wiring
- `pub mod module_tree;` in `lib.rs`
- No `pub use` re-exports.

### Plan relationship

**Path safety (Goal 3):**
- **Plan says:** Fix four `unwrap_or_else(|| Path::new("."))` fallback bugs:
  1. Line 25: `crate_root.parent().unwrap_or_else(|| Path::new("."))` in `build_module_tree`
  2. Line ~155: `file_path.parent().unwrap_or_else(|| Path::new("."))` in `process_submodule`
  3. Line ~203: `file_path.parent().unwrap_or_else(|| Path::new("."))` in `process_module_info`
  4. (workspace.rs `enumerate_members` defaulting to empty list — covered there)
- **Current state:** All three instances in this file use `unwrap_or_else(|| Path::new("."))`. When `parent()` returns `None` (e.g., the path is at the filesystem root), the fallback to `"."` silently produces incorrect relative paths.
- **Gap:** Replace all three `unwrap_or_else(|| Path::new("."))` calls with explicit handling that either returns an error, uses a better default, or propagates the issue visibly.

**Error collection (Goal 1):**
- **Plan says:** `build_module_tree` should collect per-module errors into a `Vec<ErrorEntry>` returned alongside the module tree. Orphaned module warnings (lines 107, 135) should emit `ErrorEntry` with `kind = "orphaned_module"`. Silent error swallowing in `build_module_tree` should be eliminated.
- **Current state:** `build_module_tree` returns `Result<Vec<ModuleInfo>>` and propagates errors via `?`. Orphaned modules produce `eprintln!` + a placeholder `ModuleInfo` with `<unresolved>` file. `process_submodule` uses `?` on `parse_file` calls, which currently never fail (they return empty results).
- **Gap:** `build_module_tree` signature needs to change to collect and return errors alongside results. Orphaned module handling needs to produce `ErrorEntry` values instead of `eprintln!`.

---

## File: src/cross_refs.rs

### Public API

```rust
pub fn compute(crates: &mut [CrateInfo]) -> CrossReferences
```

**Impl block:**
```rust
impl PublicItem {
    fn kind_to_string(&self) -> String  // private helper
}
```

### Module wiring
- `pub mod cross_refs;` in `lib.rs`
- No `pub use` re-exports.

### Plan relationship
- **Plan says:** Add unit tests for `compute`. No source changes mentioned.
- **Current state:** `compute` is fully implemented — builds `crate_exports` map, initializes `TypeRef` entries, scans imports for cross-crate references, populates `cross_crate_imports` on each crate.
- **Gap:** No code changes needed per the plan. Only unit tests are required (Goal 2).

---

## File: src/render.rs

### Public API

```rust
pub fn render_json(map: &WorkspaceMap) -> serde_json::Result<String>
pub fn render_to_writer(map: &WorkspaceMap, writer: impl Write) -> serde_json::Result<()>
```

### Module wiring
- `pub mod render;` in `lib.rs`
- No `pub use` re-exports.

### Plan relationship
- **Plan says:** Add unit tests for `render_json` and `render_to_writer`. No source changes mentioned.
- **Current state:** Both functions are thin wrappers around `serde_json::to_string_pretty` and `serde_json::to_writer_pretty`.
- **Gap:** No code changes needed per the plan. Only unit tests are required (Goal 2).

---

## File: src/main.rs

### Public API

```rust
struct Cli { path: PathBuf, output: Option<PathBuf> }  // clap Parser
fn main() -> anyhow::Result<()>
```

### Module wiring
- Calls `rust_workspace_map::run(config)` from the library crate.
- Uses `rust_workspace_map::Config::builder()` for configuration.

### Plan relationship
- **Plan says:** No changes mentioned. The `run()` function lives in `lib.rs` (not `main.rs`).
- **Current state:** Thin CLI entry point. Delegates all logic to `lib.rs::run()`.
- **Gap:** No changes needed in `main.rs` itself per the plan.

---

## File: src/lib.rs

### Public API

**Crate-level attributes:**
```rust
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::redundant_closure_for_method_calls)]
```

**Module declarations:**
```rust
pub mod cargo_info;
pub mod cross_refs;
pub mod file_parser;
pub mod module_tree;
pub mod render;
pub mod schema;
pub mod workspace;
```

**Re-exports:**
```rust
pub use schema::Config;
```

**Public functions:**
```rust
pub fn run(config: Config) -> anyhow::Result<()>
fn relativize_path(path_str: &str, root: &Path) -> String  // private
```

### Plan relationship

**Error collection (Goal 1):**
- **Plan says:** `run()` must: remove `.unwrap_or_default()` on `module_tree::build_module_tree` (line 79), capture errors as `ErrorEntry` with `kind = "module_tree_error"`. Collect `ErrorEntry` values from parallel crate processing (use `Mutex<Vec<ErrorEntry>>` or `rayon::collect`). Convert `cargo_info::parse_cargo_toml` failures to `ErrorEntry`. Convert `resolve_crate_roots` empty results to `ErrorEntry`. Replace `errors: Vec<ErrorEntry> = Vec::new()` with actual collection.
- **Current state:** `run()` creates an empty `errors` vec (line 39). Line 79 uses `.unwrap_or_default()` on `build_module_tree`, silently discarding errors. Parse failures and resolve failures go to `eprintln!` + `return None`. No error collection from parallel processing.
- **Gap:** Major changes needed: error collection from parallel rayon processing, removal of `.unwrap_or_default()`, conversion of `eprintln!` paths to `ErrorEntry` construction, and wiring of collected errors into `WorkspaceMap`.

**Clippy (Goal 5):**
- **Plan says:** No `#![allow(clippy::*)]` remains. Every suppression must be removed. Add `# Errors` doc sections, `#[must_use]`, fix doc comments, inline format args, replace closures, merge nested `if`, change to `&` references, remove unnecessary borrows.
- **Current state:** 9 crate-level `#![allow(clippy::*)]` attributes present (lines 2–10) plus `#![warn(clippy::pedantic)]`.
- **Gap:** All 9 crate-level suppressions must be resolved — either by fixing the underlying code or adding per-item suppressions (plan says: "no crate-level suppressions are converted to per-item suppressions; if a lint fires on legitimate code, the code is changed, not silenced").

---

## File: tests/integration_test.rs

### Public API (test functions)

```rust
fn test_sample_workspace_output()
fn test_deterministic_output()
fn test_missing_path_exits_nonzero()
```

Helper: `fn binary_path() -> String`, `fn extract_array<'a>(...)`

### Plan relationship
- **Plan says (Goal 4):** Expand from 3 to 8+ integration tests covering:
  - Parse failure workspace → verify `ErrorEntry` with severity in JSON
  - Missing workspace section → verify appropriate error
  - Glob member patterns → verify all matching crates discovered
  - Workspace with `exclude` → verify excluded crate absent
  - Deeply nested modules (3+ levels) → verify correct paths and hierarchy
  - Re-export chains → verify correct re-export tracking
  - Output via `-o` flag → verify file content matches stdout content
- **Current state:** 3 integration tests covering happy path, determinism, and missing path.
- **Gap:** Need 5+ new integration tests (plan lists 7, 3 already exist). All need fixture workspace structures.

---

## Files mentioned in plan but not found

none — all 10 files identified in the plan were found in the codebase.
