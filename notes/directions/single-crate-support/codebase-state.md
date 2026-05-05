# Codebase State -- Single-Crate Support Phase

> Generated: 2026-05-06 | Phase: single-crate-support

## 1. File Tree (Affected Areas)

```
src/
  schema.rs     (451 lines)  -- Error enum, all schema types
  workspace.rs  (222 lines)  -- find_workspace_root, enumerate_members, resolve_crate_roots
  lib.rs        (212 lines)  -- build_map, run, relativize_path (pipeline orchestrator)
  main.rs       (172 lines)  -- CLI entry point
  validate.rs   (664 lines)  -- validate, check_orphan_files, check_dead_reexports
  cargo_info.rs (156 lines)  -- parse_cargo_toml
  module_tree.rs(307 lines)  -- build_module_tree, resolve_module_path
  cross_refs.rs(213 lines)   -- compute (cross-crate references)
  indexes.rs   (287 lines)   -- derive_from_crates
  render.rs    ( 91 lines)   -- render_json, render_to_writer
  lookup.rs    (206 lines)   -- lookup_symbol, lookup_file
tests/
  integration_test.rs (660 lines) -- all integration tests
  fixtures/
    sample-workspace/   -- workspace with [workspace] section
    bad-orphan/         -- workspace with orphan .rs files
    bad-dead-reexport/  -- workspace with dead re-exports
```

## 2. Key Type/Function Signatures

### schema.rs (Error enum, lines 52-83)
```rust
pub enum Error {
    WorkspaceRootNotFound(PathBuf),      // line 54
    // NEW variant to add after line 55: CrateRootNotFound(PathBuf)
    FileRead { path: PathBuf, source: std::io::Error },
    TomlParse { path: PathBuf, source: toml::de::Error },
    SynParse { path: PathBuf, source: syn::Error },
    MemberNotFound(PathBuf),
    GlobPattern(String),
    MissingWorkspaceSection,
}
pub type Result<T> = std::result::Result<T, Error>;
```

### workspace.rs (discovery functions)
```rust
pub fn find_workspace_root(start_path: &Path) -> Result<PathBuf>          // line 11
pub fn enumerate_members(root: &Path) -> Result<Vec<PathBuf>>             // line 35
pub fn resolve_crate_roots(crate_dir: &Path) -> Vec<(PathBuf, CrateType)> // line 210
```

### lib.rs (pipeline)
```rust
pub fn build_map(config: &Config) -> anyhow::Result<WorkspaceMap>  // line 36
pub fn run(config: &Config) -> anyhow::Result<()>                  // line 186
fn relativize_path(path_str: &str, root: &Path) -> String          // line 205
```

### main.rs (CLI) -- silent error exits
Lines 88-90: file write `.is_err()` -> silent `exit(1)`
Lines 93-95: stdout write `.is_err()` -> silent `exit(1)`
Lines 102-104: pipeline error `Err(_)` -> silent `exit(1)`
Lines 165-167: lookup error `Err(_)` -> silent `exit(1)`

## 3. Module Dependency Graph

```
main.rs
  └── lib.rs::build_map, lib.rs::run, render::render_to_writer, lookup::lookup_*

lib.rs::build_map
  ├── workspace::find_workspace_root   (dependency: schema::Error)
  ├── workspace::enumerate_members     (dependency: schema::Error, glob)
  ├── cargo_info::parse_cargo_toml     (dependency: schema::*)
  ├── workspace::resolve_crate_roots
  ├── module_tree::build_module_tree   (dependency: file_parser::parse_file)
  ├── cross_refs::compute
  ├── indexes::derive_from_crates
  ├── validate::validate               (dependency: check_orphan_files, check_dead_reexports)
  └── render::render_to_writer
```

## 4. Existing Test Structure

### Unit tests per module
- `workspace.rs` (6 tests): `find_workspace_root_finds_cargo_toml`, `enumerate_members_returns_members`, `enumerate_members_returns_err_for_missing_workspace`, `enumerate_members_applies_exclude`, `resolve_crate_roots_detects_lib`, `resolve_crate_roots_detects_bin`
- `cargo_info.rs` (3 tests), `module_tree.rs` (4 tests), `cross_refs.rs` (2 tests), `indexes.rs` (5 tests), `validate.rs` (13 tests), `render.rs` (3 tests), `lookup.rs` (4 tests)

### Integration tests (18 tests)
All in `tests/integration_test.rs`. Key ones affected by this phase:
- `test_missing_workspace_section` (line 199): expects exit 1 for crate without [workspace] — WILL BREAK after fix
- `test_deeply_nested_modules` (line 284): uses workspace mode with `members = ["."]`

## 5. Validate Path Safety Analysis

### check_orphan_files (validate.rs:90-158)
`crate_root_path = workspace_root.join(&crate_info.root)`

**In workspace mode:** workspace_root=/ws, crate_info.root="core/src/lib.rs" -> /ws/core/src/lib.rs
**In single-crate mode:** workspace_root=/crate, crate_info.root="src/lib.rs" -> /crate/src/lib.rs

Both modes produce correct paths. The `strip_prefix(workspace_root)` call at line 135 also works identically. **Safe.**

## 6. Key Observations

1. **lib.rs does NOT import `schema::Error`** (lines 18-21 only import specific types). The plan's Step 4a (add Error to import) is required for the match pattern to compile.
2. **The plan's D1 enhancement** (catch MissingWorkspaceSection too) is recommended by the architect. The match becomes: `Ok(root)` -> try `enumerate_members`, catch `MissingWorkspaceSection` -> fallback.
3. **No additional silent-exit sites** beyond the four identified.
4. **validate.rs path safety is confirmed** — no changes needed.
5. **Four silent-exit serialization errors in Lookup branch** (lines 123-128, 140-145) already print messages on stderr so are NOT "silent". Not in scope.
