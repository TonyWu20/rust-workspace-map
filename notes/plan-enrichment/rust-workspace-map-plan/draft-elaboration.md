# Draft Elaboration — rust-workspace-map

## Preamble

This is a greenfield project. Codebase state confirms zero source files exist; there are no existing patterns to follow. Proposals below are derived from the plan's stated conventions (functional style, `bon::Builder`, `anyhow` for the binary, pure functions for the library, Rust 2024 edition) and idiomatic Rust practice for each domain.

Items are organized by architectural layer, not by plan step number. Cross-cutting concerns come first.

---

## Cross-Cutting: Error Handling Strategy

**Plan says:** `anyhow::Result` for the binary, `?` propagation throughout.

**Underspecified:** The plan never defines a library error type. Functions like `find_workspace_root`, `parse_cargo_toml`, `parse_file`, `build_module_tree` all return `Result<...>` but the E in `Result<T, E>` is unspecified.

**Proposal:** Introduce a single `Error` enum in `src/schema.rs` (colocated with the data types since errors are part of the domain model). Use `thiserror` for derive macros. The binary's `run()` wraps these into `anyhow::Error` via `?` and `.context()`.

```rust
/// Proposed addition to dependencies:
/// thiserror = "2"
```

**Proposed type signature:**
```rust
// In src/schema.rs
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no workspace root found starting from {0}")]
    WorkspaceRootNotFound(PathBuf),

    #[error("failed to read file {path}: {source}")]
    FileRead { path: PathBuf, source: std::io::Error },

    #[error("failed to parse {path}: {source}")]
    TomlParse { path: PathBuf, source: toml::de::Error },

    #[error("failed to parse Rust source {path}: {source}")]
    SynParse { path: PathBuf, source: syn::Error },

    #[error("workspace member {path} does not exist")]
    MemberNotFound(PathBuf),

    #[error("glob pattern error: {0}")]
    GlobPattern(String),
}

/// Convenience alias used by all library functions.
pub type Result<T> = std::result::Result<T, Error>;
```

**Error handling strategy:** Add `thiserror = "2"` to dependencies. All library modules import `crate::schema::Result` and return `schema::Error` variants. The binary's `main.rs` maps these to `anyhow::Result` via `?`; `lib.rs::run()` optionally uses `.context()` for anyhow wrapping.

**Uncertain:** Whether `GlobPattern` error is needed. The plan says "glob pattern resolution" for `[workspace] members`. If the `glob` crate is used, this variant holds `glob::PatternError`. If simple manual expansion is done instead, the variant may not be needed. **Decision deferred to implementation.**

---

## Cross-Cutting: Path Relativity Strategy

**Plan says:** "All file paths in the JSON output are relative to the workspace root."

**Underspecified:** When and where does the workspace-relative conversion happen? Internal functions could store absolute paths and relativize at render time, or store relative paths from the start.

**Proposal:** Store all paths internally as `PathBuf` relative to the workspace root. The conversion happens at the point of ingestion — `file_parser::parse_file` receives a workspace-relative path, `module_tree::build_module_tree` constructs workspace-relative paths.

The rationale: each `CrateInfo` has a `root` field that is the crate root relative to workspace root (e.g., `crates/engine/src/lib.rs`). Module `file` fields are also workspace-relative (e.g., `crates/engine/src/pipeline/mod.rs`). Storing relative paths internally avoids passing the workspace root around as context at render time and makes testing easier (test fixtures can have a predictable base path).

**Uncertain:** Whether `walkdir` produces absolute paths that need trimming. If the workspace root is passed as `&Path` (absolute, from `std::env::current_dir` or CLI argument canonicalization), then `walkdir` results are also absolute and need a `strip_prefix(workspace_root)` step in `workspace.rs`. If the workspace root is passed as a relative path, no stripping is needed. **Recommendation:** require `run()` to canonicalize the workspace path at the start; all internal paths are absolute; relativize to workspace root as a final step in `render.rs`. This avoids subtle bugs where two different relative paths point to the same directory.

**Revised proposal:** Store paths internally as absolute `PathBuf`. In `render.rs`, apply `path.strip_prefix(workspace_root)` before serialization. The `WorkspaceMap` struct gains a `workspace_root: PathBuf` field (not serialized, `#[serde(skip)]`) passed through from `Config`.

---

## Cross-Cutting: Determinism and Sort Order

**Plan says:** `BTreeMap` for `crossReferences.types`. "Run twice, diff outputs — identical."

**Underspecified:** Sort order for `crates` array, `modules` array within a crate, `imports` array, `publicItems` array, `reExports` array, `submodules` array, `fields` array, `impls` array, `crossCrateImports` array. For byte-identical output, every array must have a deterministic order.

**Proposal:** Apply deterministic sort at the point of collection for every `Vec` that ends up in the output:

| Array | Sort Key | Where Applied |
|---|---|---|
| `crates` | `name` (alphabetical) | `lib.rs::run()` after parallel crate collection |
| `modules` | `path` (alphabetical) | `module_tree.rs` after recursive build |
| `imports` | `path` (alphabetical) | `file_parser.rs::extract_imports` |
| `reExports` | `exportPath` (alphabetical) | `file_parser.rs::extract_re_exports` |
| `submodules` | `name` (alphabetical) | `file_parser.rs::extract_submodules` |
| `publicItems` | `name` (alphabetical), then `line` for tiebreak | `file_parser.rs::extract_public_items` |
| `fields` | source order (as written in source) | `file_parser.rs` — preserve declaration order |
| `impls` | source order (as written in source) | `file_parser.rs` — preserve declaration order |
| `impl.items` | source order | `file_parser.rs` |
| `crossCrateImports` | `importPath` (alphabetical) | `cross_refs.rs` or post-hoc in `lib.rs` |
| `attrs.derive` | alphabetical | `file_parser.rs` |
| `attrs.doc` | source order (preserved) | `file_parser.rs` |
| `deps.normal` / `deps.dev` / `deps.workspaceMembers` | alphabetical | `cargo_info.rs` |

**Rationale for preserving source order on fields/impls:** The LLM consumer benefits from seeing the declaration order (it matches what it sees when reading the file). Alphabetical sort on cross-module collections (imports, public items list, crates) ensures determinism regardless of filesystem iteration order (which is platform- and filesystem-dependent).

---

## Item: `Config` Type

**Proposed type signature:**
```rust
// In src/schema.rs
#[derive(Debug, Clone, bon::Builder)]
pub struct Config {
    /// Absolute, canonical path to the workspace root (or a subdirectory within it).
    /// `run()` canonicalizes this at startup.
    pub workspace_path: PathBuf,

    /// If Some, write JSON to this file instead of stdout.
    /// Path is resolved relative to current working directory.
    #[builder(default)]
    pub output_path: Option<PathBuf>,
}
```

**Module placement:** `src/schema.rs` — colocated with other data types.

**Error handling strategy:** Validation (path exists, is a directory) happens in `main.rs` before constructing `Config`, not in the type itself. The type carries the validated, canonicalized path.

**Ownership/lifetime notes:** All fields are owned. `Config` is `Clone` (needed for potential closure captures, though with rayon it's passed by reference).

**Trait coherence notes:** None.

---

## Item: CLI Argument Parsing (Clap Struct)

**Proposed type signature:**
```rust
// In src/main.rs
#[derive(clap::Parser)]
#[command(name = "rust-workspace-map", version, about = "Generate a JSON map of a Rust workspace's public API surface")]
struct Cli {
    /// Path to the workspace root or any directory within it
    #[arg(value_name = "PATH")]
    path: PathBuf,

    /// Write JSON output to file instead of stdout
    #[arg(short = 'o', long = "output", value_name = "FILE")]
    output: Option<PathBuf>,
}
```

**Module placement:** `src/main.rs` — private to the binary.

**Error handling strategy:** Clap handles invalid input (missing required arg, invalid path) with its own error messages and non-zero exit code. The binary converts `Cli` to `Config` after canonicalizing the path:

```rust
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let workspace_path = std::path::absolute(&cli.path)
        .with_context(|| format!("invalid path: {}", cli.path.display()))?;
    let config = Config::builder()
        .workspace_path(workspace_path)
        .maybe_output_path(cli.output)
        .build();
    rust_workspace_map::run(config)
}
```

**Uncertain:** Whether `std::path::absolute` is stable in Rust 2024 edition. This function was stabilized in Rust 1.79.0 (2024-06-13). It should be available, but if the user's toolchain is older, fall back to `std::env::current_dir()?.join(&path)` followed by `canonicalize()`. **Recommendation:** use `std::path::absolute`; check toolchain version in CI if needed.

---

## Item: `ImplInfo` Type — `type_` vs `type` Field

**Plan says:** Schema type `ImplInfo` has `type_` field (Rust keyword escape), but JSON shows `type` key.

**Proposed type signature:**
```rust
// In src/schema.rs
#[derive(Debug, Clone, Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ImplInfo {
    /// The type name this impl block is for (e.g., "Pipeline").
    /// Serialized as "type" in JSON.
    #[serde(rename = "type")]
    pub type_: String,

    pub items: Vec<ImplItem>,
}

#[derive(Debug, Clone, Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ImplItem {
    pub kind: ImplItemKind,
    pub name: String,
    pub params: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ImplItemKind {
    Fn,
    Type,
    Const,
}
```

**Module placement:** `src/schema.rs`.

**Error handling strategy:** None — pure data type.

**Ownership/lifetime notes:** All owned `String`s.

**Trait coherence notes:** None.

---

## Item: `PublicItem` — Missing Enum Variants and Trait Items

**Plan says:** `PublicItem` has `kind` (struct/enum/trait/fn/type/macro), `fields` for structs. But enums have variants and traits have methods, neither of which appear in the schema.

**Proposed type signature:**
```rust
// In src/schema.rs — add fields to PublicItem
#[derive(Debug, Clone, Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct PublicItem {
    pub kind: ItemKind,
    pub name: String,
    pub file: String,       // workspace-relative path
    pub line: usize,
    pub attrs: ItemAttrs,
    pub generics: String,
    pub visibility: String,

    /// Struct fields (empty for non-struct items).
    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<String>,

    /// Enum variant names and their optional discriminant/fields (empty for non-enum items).
    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub variants: Vec<String>,

    /// Impl blocks associated with this item (empty if none).
    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub impls: Vec<ImplInfo>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ItemKind {
    Struct,
    Enum,
    Trait,
    Fn,
    Type,
    Macro,
}

#[derive(Debug, Clone, Serialize, Default, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ItemAttrs {
    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub derive: Vec<String>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub doc: Vec<String>,
}
```

**Module placement:** `src/schema.rs`.

**Error handling strategy:** None — pure data type. Empty vectors are omitted from serialization via `skip_serializing_if`.

**Ownership/lifetime notes:** All owned `String`s.

**Trait coherence notes:** None.

**Uncertain:** Whether `variants` should be structured (name + optional fields) or just `Vec<String>` like `fields`. The plan's JSON shows `fields: ["pub x: i32"]` as stringified representations. **Recommendation:** keep variants as `Vec<String>` for consistency with the fields representation. Each variant string is the source-text representation (e.g., `"Red"`, `"Blue"`, `"Green(u32)"`). If downstream consumers (LLMs) need structured variant data, a future version can expand this. For the initial version, string representation keeps the scope bounded.

Also uncertain: whether trait methods should be included. The plan's JSON schema shows `impls` on `PublicItem` but doesn't show trait methods. A `pub trait Runner { fn run(&self); }` would appear as a `PublicItem` with `kind: Trait` and `name: Runner`, but its methods are not captured. **Recommendation:** add trait methods when this becomes a demonstrated need. For the LLM's purpose (knowing that a type exists and where to wire it), the trait name and location are sufficient.

---

## Item: `PublicItem.visibility` — Full Visibility Modifier or Just "pub"?

**Plan says:** `visibility` field is `"pub"`. Also says `pub(crate)` and `pub(super)` items inside private modules should be included.

**Proposed type signature:** Store the source-text representation of the visibility modifier.

```rust
// The visibility field stores the verbatim visibility string from syn.
// Examples: "pub", "pub(crate)", "pub(super)", "pub(in crate::foo)"
```

**Implementation note:** `syn::Visibility` has variants: `Public(Token![pub])`, `Restricted(syn::VisRestricted)`, `Inherited`. For `Restricted`, extract the `path` and parenthesized restriction (e.g., `pub(crate)` becomes `"pub(crate)"`). For `Inherited` (private), the item is filtered out by `extract_public_items` unless it is inside a private module — in which case `pub(crate)` or `pub(super)` items ARE included.

**Uncertain:** The plan says `ModuleInfo.visibility` is `"pub"` if `pub mod` and `"private"` otherwise. This is unambiguous. But `PublicItem.visibility` within a private module: a `pub(crate)` item inside a private module is still `"pub(crate)"` in the output. The LLM consumer can see the module visibility and the item visibility and judge for itself whether the item is accessible. **Recommendation:** accept this and document it — no automated visibility calculation across module boundaries.

---

## Item: `extract_submodules` Return Type — `Vec<String>` vs `Vec<SubmoduleDecl>`

**Plan says:** Line 220 says `extract_submodules(items: &[Item]) -> Vec<String>`. Line 152 defines `SubmoduleDecl` with `name: String` and `is_test: bool`. Line 211 says `#[cfg(test)]` modules are recorded as submodule with `is_test: true`.

**Resolution:** The plan has an inconsistency. `extract_submodules` must return `Vec<SubmoduleDecl>` to carry the `is_test` flag. Line 220's `Vec<String>` is an error in the plan.

**Proposed type signature:**
```rust
// In src/file_parser.rs
pub fn extract_submodules(items: &[syn::Item]) -> Vec<SubmoduleDecl>
```

**Module placement:** `src/file_parser.rs`.

---

## Item: `SubmoduleDecl` Type

**Proposed type signature:**
```rust
// In src/schema.rs
#[derive(Debug, Clone, Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct SubmoduleDecl {
    pub name: String,
    #[builder(default)]
    pub is_test: bool,
}
```

Note: `SubmoduleDecl` is an internal type used between `file_parser` and `module_tree`. It does not necessarily appear in the JSON output (the `submodules` field of `ModuleInfo` is `Vec<String>` — just the names for quick navigation). The `is_test` flag influences whether `module_tree` recurses into the submodule's contents.

---

## Item: `ModuleInfo.submodules` — Names vs Full Submodule Info

**Plan says:** JSON schema shows `"submodules": ["transform"]` — just names. But `file_parser` returns `SubmoduleDecl` with `is_test`.

**Resolution:** `ModuleInfo.submodules` stores `Vec<String>` (just names) for JSON output. The `is_test` flag from `SubmoduleDecl` is consumed by `module_tree` during traversal and does not appear in the final output. The rationale: the LLM consumer needs to know that submodules exist, but the `is_test` flag is a processing hint, not API surface information.

---

## Item: `extract_public_items` — Filtering Logic for `pub(crate)` / `pub(super)`

**Plan says:** "Private modules (and all their contents) are still traversed and appear in the output — `pub(crate)` and `pub(super)` items inside private modules are part of the crate's internal API."

**Underspecified:** Does `extract_public_items` include items with restricted visibility (pub(crate), pub(super)) unconditionally, or only when inside a private module? The function signature `extract_public_items(items: &[Item]) -> Vec<PublicItem>` has no module context.

**Proposed type signature:**
```rust
// In src/file_parser.rs
pub fn extract_public_items(items: &[syn::Item]) -> Vec<PublicItem>
```

**Resolution:** Include ALL items that are not `syn::Visibility::Inherited` (i.e., any form of `pub`). The function operates on a single file's items without module context. The module-level visibility decision (`pub mod` vs `mod`) is handled separately in `module_tree.rs` which sets `ModuleInfo.visibility`.

In practice, this means:
- In a `pub mod` file: `pub` items are publicly visible; `pub(crate)` items are crate-visible.
- In a `mod` (private) file: `pub` items are module-private but still appear in output; `pub(crate)` items are crate-visible and appear.
- Items with no visibility (`Inherited`) are always excluded.

The LLM consumer gets all items with any form of `pub` and can reason about accessibility from the module visibility chain.

**Uncertain:** Whether this causes noise for LLM consumers. A deeply nested private module with many `pub` items (that are effectively private outside the module) may clutter the map. **Recommendation:** implement as specified now; add a `--skip-private-modules` flag later if needed.

---

## Item: `FileInfo` Type — Internal vs JSON-Visible

**Plan says:** `FileInfo` is returned by `file_parser::parse_file` but is not listed in the JSON schema. It is an internal intermediate type.

**Proposed type signature:**
```rust
// In src/schema.rs — NOT serialized, internal type
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub public_items: Vec<PublicItem>,
    pub imports: Vec<Import>,
    pub re_exports: Vec<ReExport>,
    pub submodules: Vec<SubmoduleDecl>,
    pub impls: Vec<ImplInfo>,
}
```

`FileInfo` is consumed by `module_tree.rs` to construct `ModuleInfo`. The `impls` field here captures all `impl` blocks in the file; `module_tree` associates each `ImplInfo` with its target `PublicItem` by matching `ImplInfo.type_` against `PublicItem.name` within the same module.

---

## Item: Crate Target Resolution — Module Ownership

**Plan says:** "Insert after Step 3 (before Step 4): For each crate member directory, determine the crate entry point(s)."

**Underspecified:** Which module owns this logic? It touches both file existence checks and workspace member iteration.

**Resolution:** Owned by `workspace.rs` as part of member enumeration. The logic flow:

1. `workspace::enumerate_members` returns `Vec<PathBuf>` (crate directories).
2. For each crate directory, `workspace::resolve_crate_roots(crate_dir: &Path) -> Vec<(PathBuf, CrateType)>` checks for `src/lib.rs` and `src/main.rs`.
3. Result feeds into `module_tree` per entry point.

**Proposed type signature:**
```rust
// In src/workspace.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrateType {
    Lib,
    Bin,
}

/// Returns the entry-point files for a crate directory.
/// Typically returns [(src/lib.rs, Lib)] for library crates,
/// [(src/main.rs, Bin)] for binary crates, or both for mixed crates.
pub fn resolve_crate_roots(crate_dir: &Path) -> Vec<(PathBuf, CrateType)>
```

**PackageInfo.crateType:** Derived from the resolved roots — `"lib"` if only lib.rs found, `"bin"` if only main.rs found, `"lib_and_bin"` if both found.

**Uncertain:** Binary crates in `src/bin/*.rs` — the plan only mentions `src/main.rs`. Multi-binary crates with additional `src/bin/` targets are a common pattern. **Recommendation:** defer `src/bin/` support to a future version. The initial implementation handles only `src/lib.rs` and `src/main.rs` as entry points. Document this as a known limitation.

---

## Item: `enumerate_members` — Glob Expansion Strategy

**Plan says:** "parse workspace TOML, resolve member paths (including glob pattern resolution)."

**Underspecified:** How are glob patterns in `[workspace] members` resolved? The plan does not list a `glob` crate dependency.

**Resolution:** Two approaches:

1. **Add `glob` crate dependency.** `glob::glob(workspace_root.join(pattern))` resolves patterns like `crates/*`. This is the standard approach and handles `*`, `?`, `[...]` patterns correctly.

2. **Manual `walkdir`-based resolution.** For simple patterns like `crates/*` (the most common case), iterate the parent directory and check if each entry is a directory containing a `Cargo.toml`. This avoids an extra dependency but does not handle complex globs.

**Recommendation:** Add the `glob` crate (lightweight, widely used). It is the correct tool for this job and avoids reinventing glob matching.

```toml
# Addition to Cargo.toml dependencies:
glob = "0.3"
```

**Proposed type signature:**
```rust
// In src/workspace.rs
pub fn enumerate_members(root: &Path) -> Result<Vec<PathBuf>>
```

Implementation: parse `[workspace]` TOML, extract `members` as `Vec<String>`, iterate each member string, if it contains glob characters (`*`, `?`, `[`) use `glob::glob`, otherwise use direct path join. Apply `exclude` list after expansion by checking each resolved path against exclude patterns.

---

## Item: `DepInfo` — Dependency Name Extraction from TOML

**Plan says:** `DepInfo` has `normal: Vec<String>`, `dev: Vec<String>`, `workspace_members: Vec<String>`.

**Underspecified:** How are dependency names extracted from `[dependencies]` TOML? TOML dependency tables can be inline strings (`dep = "1.0"`) or tables (`dep = { version = "1.0", features = [...] }`). How do we detect `workspace = true`?

**Resolution:** Parse `[dependencies]` as `toml::Value`, iterate its keys (which are the dep names). For each value:
- If it's a string (inline format), add to `normal`/`dev` list.
- If it's a table, check for `workspace = true` key — if present and `true`, add to `workspace_members` list; otherwise, add to `normal`/`dev` list.

Separate `[dependencies]` from `[dev-dependencies]` by parsing both TOML sections independently.

**Proposed type signature:**
```rust
// In src/cargo_info.rs
pub fn parse_cargo_toml(path: &Path) -> Result<(PackageInfo, DepInfo)>
```

The function:
1. Reads file content.
2. Parses as `toml::Value`.
3. Extracts `[package]` → `PackageInfo`.
4. Extracts `[dependencies]` → collect keys for `normal`.
5. Extracts `[dev-dependencies]` → collect keys for `dev`.
6. From both dependency tables, identifies entries with `workspace = true` → `workspace_members`.
7. Handles missing `[package]` for virtual workspace roots (returns default/empty `PackageInfo`).

---

## Item: `module_tree::build_module_tree` — Full Signature and Recursion Strategy

**Plan says:** `build_module_tree(crate_root: &Path, crate_name: &str) -> Result<Vec<ModuleInfo>>`.

**Underspecified:** How is the module path constructed (e.g., `engine::pipeline::transform`)? How does inline module recursion work? How does binary crate module resolution differ?

**Proposed type signature:**
```rust
// In src/module_tree.rs
pub fn build_module_tree(
    crate_root: &Path,      // e.g., "crates/engine/src/lib.rs" (workspace-relative)
    crate_name: &str,       // e.g., "engine"
) -> Result<Vec<ModuleInfo>>
```

Internal helper for recursion:
```rust
fn build_module_subtree(
    file_path: &Path,                    // workspace-relative path to the source file
    parent_module_path: &str,            // e.g., "engine::pipeline"
    items: &[syn::Item],                 // pre-parsed items (for inline modules)
    visited: &mut HashSet<PathBuf>,      // cycle detection
) -> Result<Vec<ModuleInfo>>
```

**Recursion strategy:**

1. Entry: `build_module_tree` calls `file_parser::parse_file(crate_root)` to get `FileInfo` + raw `syn::File`.
2. For the entry file, construct `ModuleInfo` with `path: crate_name` and `file: crate_root`.
3. For each `SubmoduleDecl` in the file's submodules:
   a. If `is_test: true` — record the submodule name but do NOT recurse into its contents.
   b. If the submodule declaration has a body (inline: `mod foo { ... }`) — extract the inline items from the `syn::File` and recurse into `build_module_subtree` with those items. No file lookup needed.
   c. If the submodule declaration has no body (external: `mod foo;`) — call `resolve_module_path` to find the file. If found, parse it and recurse. If not found, record as orphaned.
4. The `parent_module_path` accumulates: `"engine"` → `"engine::pipeline"` → `"engine::pipeline::transform"`.
5. Cycle detection: `visited` set of `PathBuf` prevents infinite recursion from circular `mod` declarations (shouldn't happen in valid code, but defensive).

**`resolve_module_path` strategy:**
```rust
pub fn resolve_module_path(parent_dir: &Path, mod_name: &str) -> Option<PathBuf> {
    // parent_dir is the directory containing the parent source file
    // Try: parent_dir/{mod_name}.rs
    // Then: parent_dir/{mod_name}/mod.rs
}
```

For `src/main.rs` as a binary entry point: the parent directory is `src/`, not `src/bin/`. The resolution logic is identical to library crates — `mod foo;` in `src/main.rs` looks for `src/foo.rs` or `src/foo/mod.rs`.

---

## Item: Orphaned Module Handling

**Plan says:** "Given `mod foo;` but no `foo.rs` or `foo/mod.rs` exists, verify it's reported as orphaned."

**Underspecified:** What does "reported as orphaned" mean? Warning to stderr? Entry in output? Abort?

**Resolution:** Emit a warning to stderr via `eprintln!`. Include the orphaned module in the output `ModuleInfo` with an empty items list and `file` set to `"<unresolved>"`. The `submodules` list of the parent module still includes the orphaned module name, so the LLM can see that a module declaration exists without its resolved file path.

Do NOT abort the run. An orphaned module is a resolvability issue but not a parsing failure — other crates in the workspace may still be successfully mapped.

---

## Item: Inline Module Item Extraction

**Plan says:** "Given inline module (`mod foo { pub struct Bar; }`), verify it's processed without a file lookup."

**Underspecified:** How does `build_module_tree` extract items from an inline module? The `syn::File` from `parse_file` contains inline module bodies in its `Item::Mod` variant with `content: Some((brace, items))`.

**Resolution:** After `extract_submodules` identifies the inline module declaration, `module_tree` passes the inline `items` slice to `build_module_subtree` directly (no file read). The item extraction functions (`extract_public_items`, `extract_imports`, etc.) operate on `&[syn::Item]` and work identically whether the items came from a file or from an inline module body.

**Type signature of the internal helper:**
```rust
fn process_items_for_module(
    items: &[syn::Item],
    module_path: &str,
    file_path: &Path,
) -> (Vec<PublicItem>, Vec<Import>, Vec<ReExport>, Vec<SubmoduleDecl>, Vec<ImplInfo>)
```

This is a pure function called by both `parse_file` (for file-level items) and `build_module_subtree` (for inline module items). It aggregates the results of all extractor functions.

---

## Item: `workspace::find_workspace_root` — Walk-Up Termination

**Plan says:** "walk up directory tree, find `Cargo.toml` with `[workspace]` section."

**Underspecified:** What happens when we reach the filesystem root without finding a workspace?

**Proposed type signature:**
```rust
// In src/workspace.rs
pub fn find_workspace_root(start_path: &Path) -> Result<PathBuf>
```

Start from `start_path`, iterate ancestors (including `start_path` itself). For each ancestor, check if `Cargo.toml` exists. If found, parse it and check for `[workspace]` section. Return the directory containing that `Cargo.toml`. If no workspace root is found after reaching the filesystem root, return `Err(Error::WorkspaceRootNotFound(start_path.to_path_buf()))`.

The walk-up should use `Path::ancestors()` which is available from `std::path` and yields parent directories until the filesystem root.

---

## Item: Cross-Reference — Import Path to Crate Name Matching

**Plan says:** `compute(crates: &[CrateInfo]) -> CrossReferences` builds type→importers/exporters.

**Underspecified:** How does a `use core::Task` import get matched to the crate named `core`? An import path like `core::Task` could also match `core::task::Task` (a module within the same crate). The algorithm needs to determine "is the first segment of this import path a crate name?"

**Resolution:** For a `use path::to::Type` import:
1. Extract the first path segment (e.g., `core`).
2. Check if any crate in the workspace has `name == first_segment`.
3. If yes, this is a cross-crate import: the `targetCrate` is that crate, the `symbol` is the last segment of the import path.
4. If no, this is an intra-crate import — skip it for cross-references (intra-crate references are handled by the module tree structure).

For `use a::b::c::{Type1, Type2}` (braced imports): expand to individual imports, each with path `a::b::c::Type1` and `a::b::c::Type2`.

For `use a::b::c::{self, Type1}` (self + named): `self` refers to `c`, so `a::b::c` is imported. The `self` keyword needs special handling — it represents the module itself, not a type.

**Proposed type signature remains:** `pub fn compute(crates: &[CrateInfo]) -> CrossReferences`

**Uncertain:** What about `extern crate` declarations? The plan doesn't mention them. In Rust 2018+, `extern crate` is rare (implicit for most deps). **Recommendation:** skip `extern crate` support. The tool targets Edition 2024 workspaces where `use crate_name::Type` is the standard import form.

---

## Item: Cross-Reference — What Qualifies as a "Type"?

**Plan says:** Cross-references track "types" — structs, enums, traits, type aliases, macros?

**Resolution:** Track all `PublicItem` kinds that can be imported by name: `Struct`, `Enum`, `Trait`, `Type` (type alias), `Macro`. Exclude `Fn` (functions are not typically the target of cross-reference tracking in this tool — they are module-level items, not independently imported types).

If a `use` statement imports a function (e.g., `use core::some_fn`), the cross-reference still captures it under the `symbol` field, but `some_fn` won't appear as an exporter in `crossReferences.types` unless it's a `pub fn` that appears as a `PublicItem`. This is acceptable — the cross-reference records the import relationship regardless of whether the target is a "type."

**Revised resolution:** Include ALL public items as potential cross-reference targets. An import's symbol could be a function, type alias, or macro. The `crossReferences.types` map should probably be renamed to `crossReferences.symbols` to reflect this broader scope. If keeping the name `types` for backward compatibility with the plan's JSON schema, document that it includes all named public items, not just types.

**Uncertain:** The plan's JSON schema uses `"crossReferences": { "types": { ... } }`. Renaming to `symbols` is cleaner but deviates from the plan. **Recommendation:** keep `types` in the JSON key for now (matches the plan exactly) but internally document that it tracks all named public items. Revisit naming in a post-implementation review.

---

## Item: `rayon` Parallelism Strategy

**Plan says:** "For each crate (rayon `par_iter`): a. `cargo_info::parse_cargo_toml` b. `module_tree::build_module_tree` c. Return `CrateInfo::builder()...build()`"

**Underspecified:** Which specific steps are parallelized? What are the `Send + Sync` constraints?

**Resolution:** The parallelism boundary is crate-level processing. After workspace member enumeration:

1. Enumerate crate directories (sequential, fast).
2. For each crate, in parallel (`rayon::par_iter`):
   a. Parse Cargo.toml → `(PackageInfo, DepInfo)`.
   b. Resolve crate roots (lib.rs / main.rs).
   c. For each entry point, `build_module_tree` → `Vec<ModuleInfo>`.
   d. Collect into `CrateInfo`.
3. After parallel step, `cross_refs::compute(&crates)` runs sequentially (needs all crates).
4. Render sequentially.

**Send + Sync constraints:**
- All schema types (`CrateInfo`, `PackageInfo`, `ModuleInfo`, etc.) must be `Send + Sync`. Since they contain only `String`, `PathBuf`, `Vec`, `BTreeMap`, `bool`, `usize`, and `Option` of these — they are all `Send + Sync` automatically.
- `Config` is read-only after construction — `&Config` is `Sync`.
- Avoid capturing `&mut` references in parallel closures (each crate processing is independent and read-only from shared state).

**Pattern:** The parallel closure returns `Result<CrateInfo>` for each crate. `rayon::collect()` into a `Vec<Result<CrateInfo>>`, then handle errors (see next item).

---

## Item: Error Aggregation in `run()` — Per-Crate Failures

**Plan says:** "If `syn::parse_file` fails on a source file, emit a warning to stderr and return FileInfo with empty collections. Do not abort the entire run."

**Underspecified:** What happens if an entire crate fails to process (e.g., workspace member path doesn't exist, Cargo.toml is malformed)? Does the run abort, or skip that crate and continue?

**Resolution:** Distinguish two error severities:

| Failure | Severity | Action |
|---|---|---|
| Source file parse failure | Warning | Emit to stderr, return empty `FileInfo`, continue |
| Module file not found (orphaned) | Warning | Emit to stderr, record as unresolved, continue |
| Cargo.toml parse failure | Error | Emit to stderr, skip the crate entirely, continue |
| Workspace member path missing | Error | Emit to stderr, skip the crate, continue |
| `find_workspace_root` fails | Fatal | Abort entire run |

The `run()` function in `lib.rs` collects `Result<CrateInfo>` from the parallel step. Any `Err` results are logged to stderr; `Ok` results are fed forward to cross-references and rendering. The final `WorkspaceMap` may have fewer crates than discovered members.

`WorkspaceMap.errors` (if present in the schema) carries a `Vec<ErrorEntry>` recording each non-fatal failure with file, line, and message.

**Uncertain:** Whether `WorkspaceMap.errors` should be in the final schema. The plan mentions it optionally ("The parse failure is noted in `WorkspaceMap.errors` if that field is present"). **Recommendation:** include it. LLM consumers benefit from knowing which files could not be parsed, rather than silently missing data.

---

## Item: `lib.rs::run()` — Full Orchestration Flow

**Plan says:** 6-step pure orchestration in `run()`.

**Proposed type signature:**
```rust
// In src/lib.rs
pub fn run(config: Config) -> anyhow::Result<()>
```

**Elaborated flow (replacing plan's 6-step outline with concrete calls):**

```
run(config):
  1. workspace_root = find_workspace_root(&config.workspace_path)?
  2. member_dirs = enumerate_members(&workspace_root)?
  3. crate_infos = member_dirs.par_iter().filter_map(|dir| {
       let (pkg, deps) = parse_cargo_toml(&dir.join("Cargo.toml")).ok()?;
       let roots = resolve_crate_roots(dir);
       let modules: Vec<ModuleInfo> = roots.iter().flat_map(|(root, ty)| {
           build_module_tree(root, &pkg.name).unwrap_or_default()
       }).collect();
       Some(CrateInfo::builder().name(pkg.name).root(...)....build())
     }).collect::<Vec<_>>()
  4. cross_refs = cross_refs::compute(&crate_infos)
  5. map = WorkspaceMap::builder()
       .workspace(...)
       .crates(crate_infos)
       .cross_references(cross_refs)
       .build()
  6. render::render_to_writer(&map, writer)
```

**Uncertain:** The exact rayon closure shape — `par_iter().filter_map()` requires a `Send` closure. If `parse_cargo_toml` takes `&Path` (non-owned), the path borrows must be valid across threads. Since `member_dirs` is a `Vec<PathBuf>`, `par_iter()` yields `&PathBuf`, and `&PathBuf: Send + Sync`, this works.

---

## Item: `render.rs` — Path Relativization and Writer Abstraction

**Plan says:** `render_to_writer(map: &WorkspaceMap, writer: impl Write) -> Result<()>`.

**Underspecified:** How does `render_to_writer` handle the path relativization? All internal paths are absolute; they must be relativized to workspace root before serialization.

**Resolution:** `WorkspaceMap` gains a `#[serde(skip)] workspace_root: PathBuf` field. `render_to_writer` or a pre-serialization helper calls `.strip_prefix(&map.workspace_root)` on every `PathBuf` in the map, producing a new `WorkspaceMap` with relative paths that is then serialized. Alternatively, implement a custom `Serialize` or use `#[serde(serialize_with = "...")]` on path fields.

**Simpler approach:** Relativize paths during map construction in `lib.rs::run()`, not in render. Since `run()` knows the workspace root, it can relativize each `PathBuf` as it converts parsed results into schema types. This avoids threading the workspace root through the serialization layer.

**Recommendation:** Relativize at construction time in `lib.rs::run()`. Each module (`file_parser`, `module_tree`, `cargo_info`) works with absolute paths internally. `run()` strips the workspace root prefix before assembling `WorkspaceMap`. This keeps `render.rs` a pure serialization module with no path manipulation logic, matching the plan's SRP intent.

**Revised `render.rs` signatures:**
```rust
// In src/render.rs
pub fn render_json(map: &WorkspaceMap) -> serde_json::Result<String>
pub fn render_to_writer(map: &WorkspaceMap, writer: impl Write) -> serde_json::Result<()>
```

Error type is `serde_json::Result<()>`, not `anyhow::Result` — `render.rs` has no domain errors, only serialization failures.

---

## Item: `WorkspaceMap` — Root-Level Fields

**Plan says:** JSON shows `workspace`, `crates`, `crossReferences` top-level keys. `ErrorEntry` is "optional."

**Proposed type signature:**
```rust
// In src/schema.rs
#[derive(Debug, Clone, Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceMap {
    pub workspace: WorkspaceInfo,
    pub crates: Vec<CrateInfo>,
    pub cross_references: CrossReferences,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<ErrorEntry>,

    /// Not serialized — used for path relativization during construction.
    #[builder(default)]
    #[serde(skip)]
    pub workspace_root: PathBuf,
}

#[derive(Debug, Clone, Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceInfo {
    pub root: String,           // workspace-root-relative path (always "." in practice)
    pub workspace_name: String, // from [workspace] package name or directory name
}
```

---

## Item: `CrateInfo.crossCrateImports` — When Is This Populated?

**Plan says:** `crossCrateImports` is a field on `CrateInfo`.

**Underspecified:** Is `crossCrateImports` populated by `lib.rs` after `cross_refs::compute`, or does each crate's parallel processing populate it?

**Resolution:** `cross_refs::compute` processes all crates and populates both `CrossReferences.types` AND each `CrateInfo.crossCrateImports`. The function signature becomes:

```rust
pub fn compute(crates: &mut [CrateInfo]) -> CrossReferences
```

Taking `&mut [CrateInfo]` allows `compute` to write `crossCrateImports` into each crate as a side effect, while also returning the global `CrossReferences`. This avoids duplicating the import-matching logic.

Alternatively, take `&[CrateInfo]` (immutable) and return a `HashMap<usize, Vec<CrossCrateImport>>` keyed by crate index, letting `lib.rs` assign the results. This is more functional (no mutation of input) but requires index tracking.

**Recommendation:** Take `&mut [CrateInfo]`. It is simpler and the function is internal — the mutation is contained within the orchestration flow. The plan's "pure function" intent is preserved: the function still computes output from input deterministically; the mutation is an output channel, not hidden state.

---

## Item: `ErrorEntry` — When and How Populated

**Proposed type signature:**
```rust
// In src/schema.rs (already in plan)
#[derive(Debug, Clone, Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ErrorEntry {
    pub file: String,    // workspace-relative path
    pub line: usize,     // 0 if not applicable
    pub message: String,
}
```

**Populated by:** `file_parser::parse_file` on syn parse failure populates `ErrorEntry` with `file` (the source file path), `line` from `syn::Error::span().start().line` if available (default 0), and `message` from the syn error. These are collected in `lib.rs::run()` and added to `WorkspaceMap.errors`.

Also: `module_tree::build_module_tree` for orphaned module declarations. `cargo_info::parse_cargo_toml` for malformed Cargo.toml. `workspace::enumerate_members` for missing member directories.

**Collection strategy in `run()`:** Maintain a `Vec<ErrorEntry>` alongside the crate results. Each processing step that can produce non-fatal errors appends to this vec. The vec is passed to `WorkspaceMap::builder().errors(...)`.

---

## Deferred Items Assessment

The deferred-and-patterns.md file contains no deferred items and no known failure modes. Nothing to absorb or skip.

**Verdict:** No deferred items to assess. The project is starting from a clean slate.

---

## Confidence Notes

**High confidence:**
- Error handling with `thiserror` + `anyhow` is the standard Rust pattern for library/binary split.
- Path relativization at construction time in `run()` is the simplest correct approach.
- Deterministic sort for all output arrays is necessary for byte-identical output guarantee.
- `resolve_module_path` using `{name}.rs` then `{name}/mod.rs` follows standard Rust module resolution.

**Moderate confidence:**
- `crossCrateImports` population strategy (mutate `CrateInfo` in `compute`, vs return indices). Either works; the mutation approach is simpler but less "pure."
- `PublicItem.variants` as `Vec<String>` rather than structured enum variants. Structured variants add complexity the plan does not call for.
- Whether `WorkspaceMap.errors` should always be present or only when non-empty. `skip_serializing_if = "Vec::is_empty"` handles either.

**Uncertain and flagged:**
- `glob` crate dependency for workspace member glob expansion — not in the plan's dependency list. Either add `glob` or implement simple manual expansion.
- `src/bin/*.rs` binary targets — plan only mentions `src/main.rs`. Defer to future version.
- Whether `crossReferences.types` should be renamed to `symbols` — plan uses `types` but the concept covers all imported public items. Keep `types` for now to match the plan's JSON schema exactly.
- `std::path::absolute` availability on the target toolchain — check Rust version in CI.
