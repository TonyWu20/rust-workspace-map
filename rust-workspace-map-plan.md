# rust-workspace-map: Codebase Map Generator

## Context

The `rust-development-pipeline`'s recurring failure analysis shows ~60% of fix tasks stem from the LLM lacking accurate codebase structure knowledge — missing `pub mod` (~40%), missing `pub use` (~20%), stale imports. The LLM currently explores via LSP (slow, interactive, token-wasteful) or reads files piecemeal.

The solution is a standalone Rust binary that parses a workspace with `syn` and emits a single JSON map of the **public API surface** — modules, types, re-exports, cross-crate deps. This is a pre-pass that runs once per commit, feeding LLM agents exactly what they need to correctly wire new code without reading every file.

**Design philosophy:** This is an **authoritative, deterministic source of truth**. Unlike LSP exploration (interactive, probabilistic, model-dependent) or grep-based guessing (lossy, error-prone), a well-written program produces byte-identical output for the same commit every time. LLM agents consume this as ground truth — no interpretation, no ambiguity, no exploration needed. The program is the single source of record for "what is the public API of this workspace?"

After a first-principles analysis, the project was validated:
- The problem is real (finite context → structural errors)
- No existing tool serves LLMs with a minimal, typed codebase map
- The approach is minimal: `syn` parsing + traversal + JSON output
- The implementation is bounded (~9 source files, ~1500 lines with tests)

## Coding Style & Principles

### Single Responsibility Principle (SRP)
- Each module has exactly one reason to change
- Each struct has exactly one responsibility
- Each function does exactly one thing — if a function needs a comment describing multiple actions, split it

### Test-Driven Development (TDD)
- Write failing tests **first** for every unit of logic
- Implement minimal code to make tests pass
- Refactor while keeping tests green
- Unit test coverage target: ≥ 85% for all lib modules (measured via `cargo-llvm-cov`)

### Builder Pattern
- Use **`bon`** crate (`#[derive(bon::Builder)]`) for all complex structs with >3 fields
- Provides ergonomic method chaining, automatic `Vec` collection, and `Option` handling
- All schema types with optional fields get builders

### Functional Programming Style
- **Prefer iterators** (`iter()`, `map`, `filter`, `filter_map`, `flat_map`, `fold`, `collect`) over imperative `for` loops
- **Minimize mutable state** — pure functions with immutable input → output transformations
- **No `syn::visit::Visit`** — use iterator chains over `syn::File.items` instead of the visitor pattern (the visitor pattern is inherently imperative/mutable)
- Recursion for nested module processing (functional, no mutable accumulation context)

### Other Conventions
- `rustfmt` defaults (no custom config)
- `#![warn(clippy::pedantic)]` in `lib.rs`
- No doc comments on private items
- Comments only for non-obvious logic
- `anyhow::Result` for the binary, `?` propagation throughout

## Module Architecture (SRP-aligned)

```
src/
├── main.rs           # CLI entry only (~20 lines) — reason to change: CLI interface
├── lib.rs            # run() orchestration — reason to change: pipeline order
├── schema.rs         # Data types with bon::Builder + serde — reason to change: JSON schema
├── workspace.rs      # Workspace discovery — reason to change: workspace layout rules
├── cargo_info.rs     # Cargo.toml parsing — reason to change: Cargo.toml format
├── file_parser.rs    # Extract items/imports/re-exports from syn AST — reason to change: Rust syntax
├── module_tree.rs    # Resolve mod declarations → file paths — reason to change: module resolution
├── cross_refs.rs     # Compute type → importer/exporter mapping — reason to change: cross-reference logic
└── render.rs         # Assemble + serialize to JSON — reason to change: output format
```

## Dependencies (Cargo.toml)

```toml
[package]
name = "rust-workspace-map"
version = "0.1.0"
edition = "2024"

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
syn = { version = "2", features = ["full", "extra-traits"] }
walkdir = "2"
toml = "0.8"
rayon = "1"
anyhow = "1"
clap = { version = "4", features = ["derive"] }
bon = "3"

[dev-dependencies]
cargo-llvm-cov = "0.6"
```

Note: `syn` does not need the `visit` feature — we use functional iterator chains over `File.items` instead of the visitor pattern.

## JSON Schema (key structure)

```jsonc
{
  "workspace": { "root": "...", "workspaceName": "..." },
  "crates": [{
    "name": "engine",
    "root": "src/lib.rs",
    "package": { "name", "version", "edition", "crateType" },
    "deps": { "normal": [...], "dev": [...], "workspaceMembers": [...] },
    "modules": [{
      "path": "engine::pipeline",
      "file": "src/pipeline/mod.rs",
      "visibility": "pub",
      "imports": [{ "path": "core::Task", "line": 2 }],
      "reExports": [{ "original": "engine::pipeline::transform::PipelineTransform", "exportPath": "engine::PipelineTransform", "line": 5 }],
      "submodules": ["transform"],
      "publicItems": [{
        "kind": "struct", "name": "Pipeline",
        "file": "src/pipeline/mod.rs", "line": 1,
        "attrs": { "derive": ["Debug"], "doc": [...] },
        "generics": "T", "visibility": "pub",
        "fields": ["pub transforms: Vec<Box<dyn Runner<T>>>"],
        "impls": [{ "type": "Pipeline", "items": [{"kind":"fn","name":"new","params":""}, ...] }]
      }]
    }],
    "crossCrateImports": [{ "importPath": "core::Task", "importer": "src/lib.rs:2", "targetCrate": "core", "symbol": "Task" }]
  }],
  "crossReferences": {
    "types": {
      "Task": { "importers": [...], "exporters": ["core"] },
      "Pipeline": { "importers": [...], "exporters": ["engine"] }
    }
  }
}
```

## TDD Implementation Steps

### Step 1: Scaffolding and schema types

**1a. Create crate skeleton:**
- `Cargo.toml` with dependencies above
- `src/lib.rs` — empty `run()` stub returning `anyhow::Result<()>`
- `src/main.rs` — clap CLI, calls `run()`, prints result
  - **CLI arguments:**
    - `PATH` (positional, required): path to the workspace root or any directory within it
    - `--output`, `-o` (optional): write JSON to this file instead of stdout

**1b. Define schema types (TDD):**
- Write tests for JSON serialization shape first (expected camelCase keys, expected structure)
- Define all types in `src/schema.rs` with `#[derive(Serialize, bon::Builder)]` and `#[serde(rename_all = "camelCase")]`:
  - `WorkspaceMap` — root: `workspace`, `crates`, `crossReferences`
  - `CrateInfo` — `name`, `root`, `package`, `deps`, `modules`, `crossCrateImports`
  - `PackageInfo` — `name`, `version`, `edition`, `crateType`
  - `ModuleInfo` — `path`, `file`, `visibility`, `imports`, `reExports`, `submodules`, `publicItems`
  - `PublicItem` — `kind` (struct/enum/trait/fn/type/macro), `name`, `file`, `line`, `attrs`, `generics`, `visibility`, `fields`, `impls`
  - `ImplInfo` — `type_`, `items` (vec of `ImplItem` with kind/name/params)
  - `ReExport` — `original`, `exportPath`, `line`
  - `CrossCrateImport` — `importPath`, `importer`, `targetCrate`, `symbol`
  - `CrossReferences` — `types: BTreeMap<String, TypeRef>`
  - `TypeRef` — `importers: Vec<String>`, `exporters: Vec<String>`
  - `FileInfo` — `public_items: Vec<PublicItem>`, `imports: Vec<Import>`, `re_exports: Vec<ReExport>`, `submodules: Vec<SubmoduleDecl>`, `impls: Vec<ImplInfo>` — returned by `file_parser::parse_file`
  - `Import` — `path: String` (e.g. `"std::collections::HashMap"`), `line: usize`
  - `SubmoduleDecl` — `name: String`, `is_test: bool` — marks whether the module declaration was `#[cfg(test)]`
  - `DepInfo` — `normal: Vec<String>`, `dev: Vec<String>`, `workspace_members: Vec<String>`
  - `Config` — passed to `run()`: `workspace_path: PathBuf`, `output_path: Option<PathBuf>`
  - `ErrorEntry` — `file: String`, `line: usize`, `message: String` — optional parse errors

**Verification:** `cargo test` — serialization tests pass

### Step 2: Workspace discovery (TDD)

**2a. Write failing tests:**
- Given a temp dir with a workspace root `Cargo.toml` containing `[workspace] members = ["crate-a"]`, verify `find_workspace_root` returns the root path
- Given a subdirectory within the workspace (e.g., `crate-a/src/`), verify it walks up to find root
- Given a directory with no `Cargo.toml`, verify it returns an error
- Verify `enumerate_members` returns correct crate paths (including glob pattern resolution)

**2b. Implement `src/workspace.rs`:**
- `find_workspace_root(start_path: &Path) -> Result<PathBuf>` — walk up directory tree, find `Cargo.toml` with `[workspace]` section
- `enumerate_members(root: &Path) -> Result<Vec<PathBuf>>` — parse workspace TOML, resolve member paths
- Should also respect the `exclude` key in `[workspace]`, if present — excluded paths are removed from the member list after glob expansion

**Verification:** `cargo test` — workspace tests pass

### Step 3: Cargo.toml parsing (TDD)

**3a. Write failing tests:**
- Parse a minimal `Cargo.toml` with `[package] name/version/edition` and `[dependencies]` — verify `PackageInfo` and dep list
- Parse deps with `workspace = true` — verify they're tagged as workspace members
- Parse dev-dependencies — verify they're separated from normal deps
- Handle missing optional fields (no `[package]` in a virtual workspace root)

**3b. Implement `src/cargo_info.rs`:**
- `parse_cargo_toml(path: &Path) -> Result<(PackageInfo, DepInfo)>` — read + parse TOML
- Pure function: `PathBuf -> Result<(PackageInfo, DepInfo)>`

**Verification:** `cargo test` — cargo parsing tests pass

### Crate target resolution

**Insert after Step 3 (before Step 4):**

For each crate member directory, determine the crate entry point(s):
- If `src/lib.rs` exists, it is a library crate
- If `src/main.rs` exists, it is a binary crate
- If both exist, process both as separate module trees (the crate contributes two `CrateInfo` entries)
- If neither exists, skip with a warning to stderr

The `crateType` field in `PackageInfo` should reflect this: `"lib"`, `"bin"`, or `"lib_and_bin"`. The `root` field in `CrateInfo` lists the entry file path relative to the crate root.

### Step 4: File parser — item extraction (TDD)

**4a. Write failing tests:**
- Parse `pub struct Foo { pub x: i32 }` — verify one `PublicItem` with `kind: "struct"`, `name: "Foo"`, `fields: ["pub x: i32"]`
- Parse `pub enum Color { Red, Blue }` — verify enum with variants
- Parse `pub fn new() -> Self` — verify fn item with params
- Parse `pub trait Runner { fn run(&self); }` — verify trait item
- Parse `impl Foo { pub fn bar(&self) {} }` — verify impl block associated with `Foo`
- Parse `pub use foo::bar;` — verify re-export extracted
- Parse `use std::collections::HashMap;` — verify import extracted
- Parse `mod submodule;` — verify submodule declaration extracted
- Parse `#[cfg(test)] mod tests { ... }` — verify the module exists but its contents are skipped (module recorded as submodule with `is_test: true`, no items extracted)
- Parse `include!(...)` macro — verify it's skipped
- Parse a file with no pub items — verify empty collections returned

**4b. Implement `src/file_parser.rs`:**
- `parse_file(path: &Path) -> Result<FileInfo>` — read file, parse with `syn::parse_file`, call extractors
- `extract_public_items(items: &[Item]) -> Vec<PublicItem>` — `items.iter().filter(is_public).filter_map(into_public_item).collect()`
- `extract_imports(items: &[Item]) -> Vec<Import>` — `items.iter().filter_map(into_import).collect()`
- `extract_re_exports(items: &[Item]) -> Vec<ReExport>` — `items.iter().filter(is_pub_use).filter_map(into_reexport).collect()`
- `extract_submodules(items: &[Item]) -> Vec<String>` — `items.iter().filter_map(into_mod_name).collect()`
- `extract_impls(items: &[Item]) -> Vec<ImplInfo>` — impl blocks for types defined in this file
- Each extractor is a pure function: `&[Item] -> Vec<T>` with no mutable state
- **Error handling:** If `syn::parse_file` fails on a source file, emit a warning to stderr (via `eprintln!`) and return `FileInfo` with empty collections. Do not abort the entire run. The module appears in the output with no items, and the parse failure is noted in `WorkspaceMap.errors` if that field is present.
- **cfg handling:** The tool does **not** perform conditional compilation evaluation. All items are extracted regardless of `#[cfg(...)]` attributes, **except** that `#[cfg(test)] mod tests { ... }` blocks have their contents skipped (the attribute text is matched literally as `cfg(test)`). Other cfg-gated items (e.g., `#[cfg(feature = "...")]`) appear in the output normally. This is a deliberate trade-off: test-only code is noise for LLM agents mapping the public API, but feature/platform-gated code is part of the API surface that may be active in the target compilation.
- **Private module visibility:** The `visibility` field on `ModuleInfo` is `"pub"` if the `mod` declaration is `pub mod name;`, and `"private"` otherwise. Private modules (and all their contents) are still traversed and appear in the output — `pub(crate)` and `pub(super)` items inside private modules are part of the crate's internal API that the LLM may need to use.

**Verification:** `cargo test` — all extraction tests pass. Target ≥85% coverage for this module.

### Step 5: Module tree builder (TDD)

**5a. Write failing tests:**
- Given a crate with `src/lib.rs` containing `mod foo;` and file `src/foo.rs` exists, verify `foo` is in the tree with correct path
- Given `mod foo;` but no `foo.rs` or `foo/mod.rs` exists, verify it's reported as orphaned
- Given `mod foo;` and `src/foo/mod.rs` exists (directory module), verify correct file resolution
- Given nested modules (`mod a;` in lib.rs, `mod b;` in a.rs), verify full hierarchy
- Given inline module (`mod foo { pub struct Bar; }`), verify it's processed without a file lookup

**5b. Implement `src/module_tree.rs`:**
- `build_module_tree(crate_root: &Path, crate_name: &str) -> Result<Vec<ModuleInfo>>` — recursive traversal
- `resolve_module_path(parent_dir: &Path, mod_name: &str) -> Option<PathBuf>` — try `{name}.rs` then `{name}/mod.rs`
- For each resolved file, call `file_parser::parse_file` to get items, recursively process submodules
- Functional recursion: `submodules.iter().flat_map(|m| build_subtree(...)).collect()`

**Verification:** `cargo test` — module tree tests pass

### Step 6: Cross-references (TDD)

**6a. Write failing tests:**
- Given two crates where crate A defines `pub struct Task` and crate B imports `use a::Task`, verify `Task` appears in `crossReferences.types` with exporter `a` and importer from `b`
- Given a type defined but never imported, verify it has empty importers list
- Verify BTreeMap produces sorted output (determinism)

**6b. Implement `src/cross_refs.rs`:**
- `compute(crates: &[CrateInfo]) -> CrossReferences` — iterate all crates' modules, build type→importers/exporters mapping
- Pure function: `&[CrateInfo] -> CrossReferences`
- Use `BTreeMap` for deterministic ordering

**Verification:** `cargo test` — cross-reference tests pass

### Step 7: Render (TDD)

**7a. Write failing tests:**
- Given a minimal `WorkspaceMap`, serialize to JSON — verify 2-space indent, camelCase keys
- Given the same map twice, verify byte-identical output (determinism)
- Verify `BTreeMap` keys appear in sorted order in JSON

**7b. Implement `src/render.rs`:**
- `render_json(map: &WorkspaceMap) -> Result<String>` — `serde_json::to_string_pretty`
- `render_to_writer(map: &WorkspaceMap, writer: impl Write) -> Result<()>` — streaming variant
- All file paths in the JSON output are relative to the **workspace root** (not the crate root). This ensures the LLM consumer can locate any file with a single base path.

**Verification:** `cargo test` — render tests pass

### Step 8: Integration — wire pipeline in `lib.rs`

**8a. Create test fixture workspace:**
- `tests/fixtures/sample-workspace/` with 2 small crates
- Crate `core` defines `pub struct Task`, `pub trait Runner`
- Crate `engine` imports from `core`, defines `pub struct Pipeline`
- Includes modules, re-exports, `#[cfg(test)]` blocks

**8b. Write integration test:**
- TDD: Write the integration test first (expect specific JSON structure)
- Run full pipeline against fixture workspace
- Assert JSON output structure matches expected schema
- Assert cross-references correctly link `Task` between crates

**8c. Implement `src/lib.rs`:**
- `run(config: Config) -> Result<()>` — pure orchestration:
  1. `workspace::find_workspace_root` → root path
  2. `workspace::enumerate_members` → crate paths
  3. For each crate (rayon `par_iter`):
     a. `cargo_info::parse_cargo_toml` → package + deps
     b. `module_tree::build_module_tree` → modules with items
     c. Return `CrateInfo::builder()...build()`
  4. `cross_refs::compute(&crates)` → cross-references
  5. `WorkspaceMap::builder()...build()`
  6. `render::render_to_writer(&map, stdout)`

**Verification:** `cargo test` — integration test passes

### Step 9: CLI integration tests

- Run binary against fixture workspace, capture stdout
- Verify exit code 0
- Verify JSON parses successfully
- Verify determinism: run twice, assert identical output
- Run against non-existent path, verify exit code non-zero

## Verification (end-to-end)

1. `cargo build` — compiles cleanly
2. `cargo test` — all tests pass, coverage ≥85% (`cargo llvm-cov --html`)
3. `cargo clippy -- -D warnings` — clean
4. `cargo fmt --check` — clean
5. Run against fixture workspace:
   ```
   cargo run -- tests/fixtures/sample-workspace > map.json
   ```
6. Run twice, `diff` outputs — identical (determinism)
7. Manual `map.json` inspection — schema matches design

## Integration with Pipeline

The binary runs as a pre-pass in enrich-phase-plan:

1. After plan-decomposer produces TOML plan
2. **Run `rust-workspace-map <target-workspace>` to generate `workspace-map.json`**
3. Feed the map to the LLM as context for plan-decomposer and impl-plan-reviewer
4. The LLM can now verify module wiring accuracy without LSP exploration
5. Proceed to dry-run compilation (fewer expected failures since plan was informed by accurate map)

## Non-goals (explicit)

- No macro expansion (procedural macros, `include!`)
- No full conditional compilation evaluation (`#[cfg]`) — except a hardcoded literal `#[cfg(test)]` heuristic that skips test module contents
- No type resolution (e.g., resolving `type Alias = ...` to its target)
- No function body / implementation detail extraction
- No `cargo metadata` — all info from direct TOML parsing
- No `syn::visit::Visit` — functional iterator chains over `File.items` instead
