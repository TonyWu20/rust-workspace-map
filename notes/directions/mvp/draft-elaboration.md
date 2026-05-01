# MVP Design Elaboration

> Derived from `plans/mvp/MVP_PLAN.md`, `notes/architecture-current.md`, and
> source-level analysis of the current codebase. This document adds design
> decisions, type signatures, crate boundaries, pattern requirements, pitfalls,
> and task grouping rationale to the architectural commitments already locked
> in the plan.

---

## 1. Goal Decomposition and Design Decisions

### Goal A: Schema foundations (`src/schema.rs`)

#### A1: `DiagnosticKind` enum

**Decision**: Full coverage of all 10 existing `ErrorEntry.kind` string values,
not just the 6 listed in the plan.

The plan's `DiagnosticKind` block names 6 variants (4 legacy + 2 new). But the
codebase currently uses 8 distinct string values across `lib.rs`,
`workspace.rs`, `file_parser.rs`, and `module_tree.rs`:

| Variant | Serde rename | Current source |
|---|---|---|
| `OrphanedModule` | `"orphaned_module"` | `module_tree.rs:123,159` |
| `TomlParseError` | `"toml_parse_error"` | `lib.rs:53` |
| `MissingCrateRoots` | `"missing_crate_roots"` | `lib.rs:66` |
| `ModuleTreeError` | `"module_tree_error"` | `lib.rs:90` |
| `SynParseError` | `"syn_parse_error"` | `file_parser.rs:95` |
| `MissingWorkspaceSection` | `"missing_workspace_section"` | (workspace.rs — used if migrated) |
| `GlobPatternError` | `"glob_pattern_error"` | (workspace.rs — used if migrated) |
| `MemberNotFound` | `"member_not_found"` | (workspace.rs — used if migrated) |
| `OrphanFile` | `"orphan_file"` | NEW |
| `DeadReExport` | `"dead_re_export"` | NEW |

All 10 must exist as variants. Partial coverage would leave some emit-sites
unmigrated, forcing a `String` fallback that defeats the purpose of the typed
enum. The four workspace-discovery kinds are currently emitted only as
`anyhow::Error` propagations (not `ErrorEntry`), but they must exist as
variants for completeness. When the missing-member `eprintln!` gap is closed
(phase-0.2 deferred), it will need `MemberNotFound`.

**Serde strategy**: `#[serde(rename_all = "snake_case")]` at the enum level
gives correct JSON output for all variants except `DeadReExport`
(`snake_case` would produce `"dead_re_export"` — correct) and `MissingCrateRoots`
(`snake_case` would produce `"missing_crate_roots"` — correct). All legacy
values match their `snake_case` form. Use the enum-level rename_all rather
than per-variant renames — cleaner and avoids drift.

**Pattern**: Follow the existing `ErrorSeverity` enum style (derive
`Debug, Clone, Copy, PartialEq, Eq, serde::Serialize`) with
`#[serde(rename_all = "snake_case")]`.

#### A2: Newtype keys (`CanonicalPath`, `WorkspaceRelativePath`)

**Decision**: Newtypes used as function parameter types and local variable
types; BTreeMap keys remain `String` for clean serde serialization.

The plan says "Two newtypes wrap the BTreeMap key strings." The schema block
shows `BTreeMap<String, ...>` — not `BTreeMap<CanonicalPath, ...>`. The
newtypes serve as compile-time guards in internal function signatures to
prevent accidentally passing a file path where a canonical symbol path is
expected, and vice versa. The BTreeMaps in `WorkspaceMap` use `String` keys
for zero-friction `serde_json` serialization.

```rust
// Location: src/schema.rs, near the top (before the output types)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalPath(pub String);
// "crate::module::Name" — key for symbols, name_index values

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkspaceRelativePath(pub String);
// "core/src/task.rs" — key for files

// Trait impls (macro-generated or manual):
// Display, AsRef<str>, From<String>, serde::Serialize, serde::Deserialize
```

These are used in `indexes.rs` function signatures and local bindings, and in
`validate.rs` for lookup keys. They are NOT used in `WorkspaceMap` field
types (which stay `BTreeMap<String, ...>`).

#### A3: `SymbolEntry`, `FileEntry` structs

**Decision**: Builder derive (`bon::Builder`), consistent with all existing
output structs. No `#[serde(skip_serializing_if)]` needed — all fields are
always present.

```rust
#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct SymbolEntry {
    pub crate_name: String,
    pub module: String,
    pub file: String,
    pub line: usize,
    pub kind: ItemKind,
}

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct FileEntry {
    pub module_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_module_file: Option<String>,
    pub is_crate_root: bool,
}
```

#### A4: Cross-references re-keying

**Decision**: Re-key inside `cross_refs::compute()` itself, not in a
post-processing pass. The canoncial path key is `{module.path}::{item.name}`.

Currently `cross_refs::compute()` builds `crate_exports` as
`BTreeMap<CrateName, Vec<(ShortName, Kind)>>` and keys `TypeRef` by `ShortName`.
The re-keying changes the `TypeRef` map key from `ShortName` to the canonical
path. This requires `crate_exports` to capture the module path alongside each
item:

```rust
// Before (current):
let items: Vec<(String, String)> = c
    .modules.iter()
    .flat_map(|m| &m.public_items)
    .map(|item| (item.name.clone(), item.kind_to_string()));

// After (MVP):
let items: Vec<(String, String)> = c
    .modules.iter()
    .flat_map(|m| {
        let mod_path = &m.path; // e.g., "core" or "core::sub"
        m.public_items.iter().map(move |item| {
            let canonical = format!("{mod_path}::{}", item.name);
            (canonical, item.kind_to_string())
        })
    });
```

The `imported_by` detection uses the first import segment to match crate
names. This stays unchanged — it still works because the first segment of an
import path is still a crate name. The symbol in `imported_by` was already the
last segment (short name), which becomes inaccurate after re-keying. Fix:
match the last segment against the canonical path's last segment in
`types_map`, using a suffix check.

**Risk**: Name collisions within a single crate's modules. A `Task` in
`core::foo` and a `Task` in `core::bar` would previously collide (silent
data loss); after re-keying to `core::foo::Task` and `core::bar::Task`, they
are distinct. This is correctness, not a regression.

#### A5: Schema additions integration test

The verification step 4 in the plan requires: "the existing `orphaned_module`
warning emitted by `module_tree.rs` still serializes as the string
`"orphaned_module"` in JSON." Add to `test_parse_failure_error_entry` (or a
new test) that asserts `errors[].kind` is `"orphaned_module"` (not
`"OrphanedModule"` or any camelCase variant). This is the serde rename
contract — snake_case in JSON, PascalCase in Rust.

---

### Goal B: Bug fixes (lib.rs, module_tree.rs)

#### B1: Dropped errors at lib.rs:132-137

**Decision**: `extend` errors before the `if let`, not inside it.

```rust
// Before:
for (info, errs) in results {
    if let Some(ci) = info {
        crate_errors.extend(errs);  // only on success
        crate_infos.push(ci);
    }
    // errs silently discarded when info is None
}

// After:
for (info, errs) in results {
    crate_errors.extend(errs);  // both branches
    if let Some(ci) = info {
        crate_infos.push(ci);
    }
}
```

**Note**: The `errs` vec is consumed by `extend` on the first iteration of the
loop — this works because `results` is used in a `for` loop (move iteration).
No clone needed.

#### B2: `errors.clone()` at module_tree.rs:252

**Current behavior**: `process_module_info` takes `errors: &mut Vec<ErrorEntry>`
AND returns `(Vec<ModuleInfo>, Vec<ErrorEntry>)`. The `errors.clone()` at
line 252 copies the mutable accumulator into the return value, duplicating
errors at each recursion level (O(depth x errors) memory).

**Caller analysis**:
- `process_submodule` (line 183): passes `&mut errors`, ignores return tuple
  (just calls `process_module_info(...);` without binding)
- `process_module_items` (line 210): passes `&mut Vec::new()`, captures the
  tuple return

**Decision**: Eliminate the dual-path entirely. Change `process_module_info`
to return only `Vec<ModuleInfo>`. All callers already have or can immediately
capture a `Vec<ErrorEntry>` from the mutable borrow.

```rust
// Before:
fn process_module_info(
    ...,
    errors: &mut Vec<ErrorEntry>,
) -> (Vec<ModuleInfo>, Vec<ErrorEntry>)

// After:
fn process_module_info(
    ...,
    errors: &mut Vec<ErrorEntry>,
) -> Vec<ModuleInfo>
```

Caller changes:
- Line 183 (`process_submodule`): already passes `&mut errors`, just change
  `let _ = process_module_info(...);` to capture only `Vec<ModuleInfo>`.
- Line 210 (`process_module_items`): currently creates `&mut Vec::new()` and
  returns the tuple. Change to: create a local `let mut errs = Vec::new()`,
  pass `&mut errs`, then return `(modules, errs)`.

Line 252 becomes `modules` — no clone. **This is a `mem::take`-free fix**
because the mutable borrow pattern already gives ownership to the caller.

**Risk**: `process_submodule` at line 183 currently ignores the returned
modules (the tuple). After the fix, it MUST capture and use the returned
`Vec<ModuleInfo>`. Verify: line 183's call is inside a statement expression
that discards the return. The `&mut errors` reference already accumulated
errors. The returned modules were silently dropped — a pre-existing bug! Fix
by capturing and extending the caller's `modules` vec.

This is actually a second bug the fix incidentally catches: `process_submodule`
at line 183 drops the modules returned by `process_module_info`. After the fix,
it must extend the caller's `modules` with the returned modules.

#### B3: `WorkspaceInfo.root` hardcoded to `"."`

```rust
// Before (lib.rs:149-152):
let workspace_info = WorkspaceInfo::builder()
    .root(".".to_string())         // hardcoded
    .workspace_name(workspace_name)
    .build();

// After:
let workspace_info = WorkspaceInfo::builder()
    .root(workspace_root.to_string_lossy().to_string())
    .workspace_name(workspace_name)
    .build();
```

**Note**: This changes the JSON output. The existing integration test
`test_sample_workspace_output` asserts `json["workspace"]["root"] == "."`.
Update the assertion to check it's a non-empty string (since the fixture
root varies by checkout location) rather than a hardcoded value. Use
`!json["workspace"]["root"].as_str().unwrap().is_empty()`.

---

### Goal C: `indexes.rs` — flat index derivation

**Decision**: Pure function, one pass over `&[CrateInfo]`, no second AST
traversal. Called in `lib.rs::run()` after `cross_refs::compute()` and
before `WorkspaceMap::builder()`.

**Module path**: `src/indexes.rs` (NEW)

**Public function**:

```rust
use crate::schema::{FileEntry, SymbolEntry};
use std::collections::BTreeMap;

/// Derive three flat indexes from the hierarchical crate data.
///
/// Called once after `cross_refs::compute()`. No I/O, no AST traversal —
/// purely a reshaping of already-extracted data. The indexes provide
/// O(1) lookup for LLM agents: hashmap access, no tree traversal.
///
/// Returns:
/// - `symbols`: canonical path → SymbolEntry. Key is "crate::module::Name".
/// - `name_index`: short name → list of canonical paths (collision support).
/// - `files`: workspace-relative path → FileEntry. Inline modules excluded.
pub fn derive_from_crates(crates: &[CrateInfo]) -> (BTreeMap<String, SymbolEntry>, BTreeMap<String, Vec<String>>, BTreeMap<String, FileEntry>)
```

**Algorithm for `symbols` and `name_index`**:

```
for crate in crates:
    for module in crate.modules:
        for item in module.public_items:
            canonical = "{module.path}::{item.name}"
            entry = SymbolEntry { crate_name, module: module.path, file: item.file, line: item.line, kind: item.kind }
            symbols.insert(canonical, entry)
            name_index[item.name].push(canonical)
```

This is a flat iterator pipeline over `crates.iter().flat_map(|c| c.modules.iter().flat_map(|m| m.public_items.iter().map(...)))`. Use `fold` to accumulate into `BTreeMap`s.

**Algorithm for `files`**:

```
for crate in crates:
    for module in crate.modules:
        if module.file == "<unresolved>":
            continue
        if files already has this file path:
            skip (inline module — shares parent's file)
        else:
            // Determine parent_module_file:
            // If module.path has a parent (contains "::"), the parent's
            // ModuleInfo.file is parent_module_file.
            // If this is the crate root module, parent_module_file is None.
            parent = find_parent_file(crate, module)
            entry = FileEntry {
                module_path: module.path,
                parent_module_file: parent,
                is_crate_root: module.path == crate.name,
            }
            files.insert(module.file, entry)
```

**Inline module detection**: The module tree is depth-first (guaranteed by
`build_module_tree`). For each file path, the first module encountered in
iteration order is the "primary" module; subsequent modules with the same file
are inline modules. This works because `build_module_tree` processes the root
module first, then submodules depth-first. Inline modules always appear after
their parent in the flattened list.

**`parent_module_file` computation**: The parent of `crate::foo::bar` is
`crate::foo`. Walk `crate.modules` to find the entry whose `.path` matches
the parent path. If found, that entry's `.file` is `parent_module_file`. If
not found (crate root), it's `None`.

**Pattern requirement**: Iterator pipelines over for-loops. The `symbols` and
`name_index` construction is a single `flat_map` → `fold` pipeline. The
`files` construction uses a `for` loop because it needs to consult the
accumulating map (inline detection), but the body uses iterator methods.

**Testability**: `derive_from_crates` is a pure function. Unit-test it in
`src/indexes.rs` with hand-constructed `CrateInfo` values (same pattern as
`cross_refs.rs` tests using `make_crate()`). Test:
- Single crate with root-level `pub struct Foo` → `symbols` has `"crate::Foo"`
- Nested module `pub struct Bar` → `symbols` has `"crate::module::Bar"`
- Two crates both exporting `Task` → `name_index["Task"]` has two entries
- Inline module excluded from `files`
- `parent_module_file` is `None` for crate root, `Some` for submodules

---

### Goal D: `validate.rs` — validation rules

**Decision**: Single public function called from `lib.rs::run()` when
`--validate` is set. Two internal rules: `OrphanFile` and `DeadReExport`.

**Module path**: `src/validate.rs` (NEW)

**Public function**:

```rust
use crate::schema::{CrateInfo, ErrorEntry};
use std::path::Path;

/// Run structural validation rules against the workspace.
///
/// Only the `OrphanFile` and `DeadReExport` rules fire in MVP.
/// All findings are `ErrorEntry { severity: Warning, ... }`.
/// Called after module tree construction and `cross_refs::compute()`.
/// False negatives over false positives (conservative recall).
pub fn validate(crates: &[CrateInfo], workspace_root: &Path) -> Vec<ErrorEntry>
```

**Internal functions**:

```rust
fn check_orphan_files(crate_info: &CrateInfo, workspace_root: &Path) -> Vec<ErrorEntry>
fn check_dead_reexports(crate_info: &CrateInfo) -> Vec<ErrorEntry>
```

**`check_orphan_files` algorithm**:

1. Collect all `ModuleInfo.file` values for this crate (excluding
   `"<unresolved>"`). Convert to a set of workspace-absolute paths by
   joining with `workspace_root`.
2. Walk the crate's source directory (`{crate_root}/src/`) for all `*.rs`
   files using `std::fs::read_dir` recursively (but skip `src/bin/`,
   test directories, and `build.rs` at the crate root).
3. For each `.rs` file on disk not in the module-tree file set, emit:
   ```rust
   ErrorEntry::builder()
       .file(relativize(&disk_file, workspace_root))
       .message(format!("orphan file: {} is not declared in the module tree", name))
       .severity(ErrorSeverity::Warning)
       .kind(DiagnosticKind::OrphanFile)
       // context: which parent file should declare it
       .build()
   ```
4. The "which parent file" hint: strip the `src/` prefix, resolve the parent
   directory. For `src/foo.rs`, the parent module file is `src/lib.rs` (or
   `src/main.rs`). For `src/foo/bar.rs`, the parent is `src/foo/mod.rs`.

**Reuse**: Step 2 uses `resolve_module_path` from `module_tree.rs` in reverse.
Since `resolve_module_path` checks file existence for `{parent}/{name}.rs`
and `{parent}/{name}/mod.rs`, the orphan-file check can: for each `.rs` on
disk, compute the `mod name;` that would resolve to it, then check whether
the parent file contains `mod name;`. Alternatively (simpler): just walk
the `.rs` files and check set membership. The plan says "check whether the
resolver included it" — set membership is sufficient.

**`check_dead_reexports` algorithm** — five cases per the plan:

For each `ReExport` in each module of each crate:
1. Parse the `import_path` (e.g., `crate::foo::Bar`, `self::baz::Quux`,
   `super::x::Y`).
2. **Resolve prefix**: `crate::` → this crate's name; `self::` → current
   module path; `super::` → parent module path.
3. **External check**: If the first path segment after resolution is NOT a
   workspace member crate name, skip — it's an external re-export, not dead.
4. **Glob check**: If the import path ends with `*` (glob re-export), skip.
5. **Intra-workspace resolve**: For the remaining resolved path, check whether
   the named item exists in the `symbols` flat index. The canonical path to
   look up is the resolved path itself (it should be in `symbols`).
6. If not found, emit:
   ```rust
   ErrorEntry::builder()
       .file(module.file)
       .line(re_export.export_line)  // need ReExport.line — already exists
       .message(format!("dead re-export: {} not found", import_path))
       .severity(ErrorSeverity::Warning)
       .kind(DiagnosticKind::DeadReExport)
       .context(ErrorContext::builder()
           .module_path(module_path)
           .build())
       .build()
   ```

**Dependency**: DeadReExport check needs the `symbols` index. Since
`validate()` is called after `derive_from_crates()`, the symbols map is
available. The simplest approach: accept the `symbols` BTreeMap as a
parameter, or accept the full `WorkspaceMap` (if validate is called after
construction).

Alternatively, `check_dead_reexports` operates on `&[CrateInfo]` directly
and builds its own name lookup. But that duplicates work. Better: accept
`&BTreeMap<String, SymbolEntry>` (the `symbols` index) as a parameter.

```rust
// Revised signature:
pub fn validate(
    crates: &[CrateInfo],
    symbols: &BTreeMap<String, SymbolEntry>,
    workspace_root: &Path,
) -> Vec<ErrorEntry>
```

The caller in `lib.rs` passes the symbols map returned by `derive_from_crates`.

**Crate names set for DeadReExport**: Need to know which crate names are
workspace members (step 3). This is trivially `crates.iter().map(|c| &c.name)`.

**Conservative recall**: The glob skip and external skip are explicit gates.
Transitive chains (A re-exports B which re-exports C) are not traced — false
negative is acceptable per policy.

---

### Goal E: `lookup.rs` — targeted queries

**Decision**: Pure functions over `&WorkspaceMap`. Two lookup modes.

**Module path**: `src/lookup.rs` (NEW)

**Result types**:

```rust
use crate::schema::{FileEntry, ItemKind, ModuleInfo, SymbolEntry, WorkspaceMap};

/// Result of `lookup_file`.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileLookupResult {
    pub file_entry: FileEntry,
    pub primary_module: ModuleInfo,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub inline_modules: Vec<ModuleInfo>,
}

/// Result of `lookup_symbol`.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "status")]
pub enum SymbolLookupResult {
    #[serde(rename = "found")]
    Found(SymbolEntry),
    #[serde(rename = "ambiguous")]
    Ambiguous {
        name: String,
        candidates: Vec<DisambiguationHint>,
    },
    #[serde(rename = "not_found")]
    NotFound,
}

/// A candidate when a short name matches multiple canonical paths.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisambiguationHint {
    pub canonical_path: String,
    pub crate_name: String,
    pub kind: ItemKind,
    pub file: String,
    pub line: usize,
}

/// Look up a symbol by short name (e.g., "Task").
///
/// Resolves through `name_index`. If one candidate, returns `Found`.
/// If multiple (name collision across crates), returns `Ambiguous` with
/// disambiguation hints. If no match, returns `NotFound`.
pub fn lookup_symbol(map: &WorkspaceMap, name: &str) -> SymbolLookupResult

/// Look up a file by workspace-relative path (e.g., "core/src/task.rs").
///
/// Returns `None` if the file is not in the `files` index.
/// When found, joins against `crates[].modules[]` to recover the primary
/// module and any inline modules sharing the same file.
pub fn lookup_file(map: &WorkspaceMap, file: &str) -> Option<FileLookupResult>
```

**`lookup_symbol` algorithm**:

1. `map.name_index.get(name)` — if `None`, return `NotFound`.
2. If `paths.len() == 1`: `map.symbols.get(&paths[0])` → `Found(entry)`.
3. If `paths.len() > 1`: for each path, `map.symbols.get(path)` → populate
   `DisambiguationHint { canonical_path, crate_name, kind, file, line }`.
   Return `Ambiguous { name, candidates }`.

**`lookup_file` algorithm**:

1. `map.files.get(file)` — if `None`, return `None`.
2. `file_entry = map.files[file]`.
3. Scan `map.crates.iter().flat_map(|c| &c.modules)` for all `ModuleInfo`
   entries whose `.file` matches the requested path.
4. Partition: `primary_module` is the one whose `.path == file_entry.module_path`;
   `inline_modules` is everything else with the same `.file`.
5. Return `FileLookupResult { file_entry, primary_module, inline_modules }`.

**Design note on separation from CLI**: These are pure library functions,
callable from tests without process spawning. The `main.rs` `lookup`
subcommand calls `run()` (or a new `build_map()` function — see Goal F) to
get the `WorkspaceMap`, then dispatches to `lookup_symbol` or `lookup_file`.

**Why `lookup_file` returns `Option<FileLookupResult>` not `Result`**:
"File not in index" is a lookup miss, not an error. Exit code in CLI is `1`
for not-found, but the library function uses `Option` — the CLI wrapper
converts `None` to exit code 1.

**Why `lookup_symbol` uses an enum not `Option<SymbolEntry>`**: Ambiguous
matches need a third state. The enum with `#[serde(tag = "status")]` produces
clean JSON:
```json
{ "status": "found", "crateName": "core", ... }
{ "status": "ambiguous", "name": "Task", "candidates": [...] }
{ "status": "not_found" }
```

---

### Goal F: CLI refactor (`src/main.rs`)

**Decision**: `clap` subcommands. `index` required, `lookup` optional. Bare
path removed. Exit codes: 0 = success, 1 = tool error, 2 = validation issues.

```rust
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rust-workspace-map", version, about = "Generate a JSON map of a Rust workspace")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Build the workspace map, optionally with validation
    Index {
        /// Path to the workspace root or a subdirectory
        path: PathBuf,

        /// Write JSON output to file instead of stdout
        #[arg(short = 'o', long = "output")]
        output: Option<PathBuf>,

        /// Run OrphanFile and DeadReExport validation checks
        #[arg(long)]
        validate: bool,
    },

    /// Look up a symbol or file in the workspace (CLI convenience)
    Lookup {
        /// Path to the workspace root or a subdirectory
        path: PathBuf,

        /// Look up a symbol by short name
        #[arg(long = "symbol", conflicts_with = "file")]
        symbol: Option<String>,

        /// Look up a file by workspace-relative path
        #[arg(long = "file", conflicts_with = "symbol")]
        file: Option<String>,
    },
}
```

**Config changes**: `Config` gains a `validate: bool` field with
`#[builder(default)]`.

```rust
// Before:
pub struct Config {
    pub workspace_path: PathBuf,
    pub output_path: Option<PathBuf>,
}

// After:
pub struct Config {
    pub workspace_path: PathBuf,
    pub output_path: Option<PathBuf>,
    #[builder(default)]
    pub validate: bool,
}
```

**`lib.rs` orchestration for lookup**: A clean split between constructing the
map and rendering it.

```rust
/// Build the full WorkspaceMap (discovery + parsing + indexes + optional validation).
/// Returns the map without writing to output.
pub fn build_map(config: &Config) -> anyhow::Result<WorkspaceMap>

/// Build the map and render to stdout/file. Existing behavior, refactored to
/// call `build_map` internally.
pub fn run(config: &Config) -> anyhow::Result<()>
```

The `lookup` subcommand in `main.rs`:
1. Create a Config with `validate: false` (lookup doesn't validate)
2. Call `build_map(&config)` → `WorkspaceMap`
3. Dispatch to `lookup::lookup_symbol()` or `lookup::lookup_file()`
4. Serialize result to JSON on stdout
5. Exit code: 0 for found, 1 for not-found

**`build_map` function extraction**: The current `run()` body becomes
`build_map()`, returning `WorkspaceMap` instead of calling
`render::render_to_writer()`. The `run()` function becomes:

```rust
pub fn run(config: &Config) -> anyhow::Result<()> {
    let map = build_map(config)?;
    if let Some(ref output_path) = config.output_path {
        let file = std::fs::File::create(output_path)
            .with_context(|| format!("failed to create output file: {}", output_path.display()))?;
        render::render_to_writer(&map, std::io::BufWriter::new(file))?;
    } else {
        render::render_to_writer(&map, std::io::stdout().lock())?;
    }
    Ok(())
}
```

**Validation integration in `build_map`**:

```rust
pub fn build_map(config: &Config) -> anyhow::Result<WorkspaceMap> {
    // ... (existing pipeline up to cross_refs::compute) ...

    let (symbols, name_index, files) = indexes::derive_from_crates(&crate_infos);

    let mut all_errors = crate_errors;

    if config.validate {
        let validate_findings = validate::validate(&crate_infos, &symbols, &workspace_root);
        all_errors.extend(validate_findings);
    }

    let map = WorkspaceMap::builder()
        .workspace(workspace_info)
        .crates(crate_infos)
        .cross_references(cross_refs)
        .symbols(symbols)
        .name_index(name_index)
        .files(files)
        .errors(all_errors)
        .workspace_root(workspace_root)
        .build();

    Ok(map)
}
```

**Exit code logic in main.rs**: After `build_map()` or `lookup`, check for
validation findings. If any `ErrorEntry` in `map.errors` has
`severity == Warning` and `kind` is `OrphanFile` or `DeadReExport`, AND the
`--validate` flag was set, exit code is 2. Otherwise, exit 0.

---

### Goal G: Test expansion

#### G1: Fixture `bad-orphan/`

```
tests/fixtures/bad-orphan/
├── Cargo.toml        # [workspace] members = ["."]; [package] name = "bad-orphan"
└── src/
    ├── lib.rs        # (empty or minimal)
    └── forgotten.rs  # pub fn not_declared() {} — no `mod forgotten;` in lib.rs
```

`index --validate` exits 2. `errors` contains one `ErrorEntry` with
`kind == "orphan_file"`, `severity == "warning"`, `file` referencing
`src/forgotten.rs`, message hinting `add 'pub mod forgotten;'`.

#### G2: Fixture `bad-dead-reexport/`

```
tests/fixtures/bad-dead-reexport/
├── Cargo.toml        # [workspace] members = ["."]; [package] name = "bad-dead-reexport"
└── src/
    └── lib.rs        # pub use crate::DoesNotExist;
```

`index --validate` exits 2. `errors` contains one `ErrorEntry` with
`kind == "dead_re_export"`, `severity == "warning"`.

#### G3: Integration test additions

| Test | Invocation | Assertion |
|---|---|---|
| `validate_orphan_file_exits_2` | `index --validate bad-orphan/` | exit code 2, errors[0].kind == "orphan_file" |
| `validate_dead_reexport_exits_2` | `index --validate bad-dead-reexport/` | exit code 2, errors[0].kind == "dead_re_export" |
| `index_no_validate_exits_0` | `index bad-orphan/` (no --validate) | exit code 0, errors is empty (validate not run) |
| `lookup_symbol_found` | `lookup --symbol Task` on sample-workspace | status "found", symbol.kind == "struct" |
| `lookup_symbol_not_found` | `lookup --symbol DoesNotExist` on sample-workspace | exit code 1, status "not_found" |
| `lookup_file` | `lookup --file core/src/task.rs` on sample-workspace | contains file_entry, primary_module |
| `index_mandatory_flag_day` | All existing tests rewritten from `bare-path` to `index bare-path` | passes as before with additive schema |

#### G4: Existing test rewrite

Every existing integration test changes from:
```rust
Command::new(&binary_path()).arg(fixture).output()
```
to:
```rust
Command::new(&binary_path()).arg("index").arg(fixture).output()
```

This is the flag-day sweep required by verification step 2 and locked decision D1.

**`test_workspace_root_not_hardcoded`**: Remove assertion
`json["workspace"]["root"] == "."` from `test_sample_workspace_output`.
Replace with non-empty check.

#### G5: `DiagnosticKind` migration regression test

Add to `test_parse_failure_error_entry` or create new test: construct a
workspace with an orphaned module (a `mod nonexistent;` declaration where the
file doesn't exist) and assert `errors[].kind == "orphaned_module"` (the
string — verifying the serde rename contract). This is verification step 4.

---

### Goal H: Pipeline integration files

These are in the `rust-development-pipeline/` repo, not `rust-workspace-map/`.
The plan specifies:
- `skills/compile-plan/SKILL.md`: Add authoritative gate — `rust-workspace-map index --validate <project>` pre-check that blocks on exit code 2.
- `agents/plan-decomposer.md`: Add Module Wiring Check — soft suggestion for `lookup --file <parent>` and `lookup --symbol <name>`.

These are documentation/skill changes, not code. They belong in the
`rust-development-pipeline` working directory.

---

## 2. Crate Boundary Decisions

### New modules (within the single binary crate)

| Module | Role | Dependencies |
|---|---|---|
| `src/indexes.rs` | Pure data reshaping | `schema::CrateInfo`, `schema::SymbolEntry`, `schema::FileEntry` |
| `src/validate.rs` | Structural validation rules | `schema::CrateInfo`, `schema::ErrorEntry`, `schema::SymbolEntry` (flat index), `module_tree::resolve_module_path` (for file discovery) |
| `src/lookup.rs` | Targeted queries over WorkspaceMap | `schema::WorkspaceMap`, `schema::SymbolEntry`, `schema::FileEntry`, `schema::ModuleInfo` |

All three are `pub mod` in `lib.rs`. Each exposes 1–2 public functions. All
are pure (no I/O except `validate.rs` which walks the filesystem for orphan
detection — and even that is parameterized on `&Path` for testability).

### Modified modules

| Module | Changes | Risk |
|---|---|---|
| `src/schema.rs` | Add ~4 types, extend ~2 types, re-key 1 field | Moderate — serialization contract must stay stable (serde renames) |
| `src/lib.rs` | Add `build_map()`, call `indexes::derive_from_crates()`, wire `validate`, fix 2 bugs | Moderate — orchestrator, integration-tested |
| `src/main.rs` | Subcommand refactor, bare-path removal | Low — thin CLI layer |
| `src/module_tree.rs` | Fix `errors.clone()` + orphaned-module path | Low — change is isolated to one function |
| `src/cross_refs.rs` | Re-key TypeRef map to canonical path | Moderate — changes the public JSON surface |
| `src/render.rs` | No changes | — |
| `src/file_parser.rs` | Update emit-sites to use `DiagnosticKind` | Low — mechanical migration |
| `src/workspace.rs` | Update emit-sites if migrating to `DiagnosticKind` | Low |
| `src/cargo_info.rs` | No changes | — |

### What stays where

- **Schema**: ALL types stay in `schema.rs`. No splitting into submodules.
  The file is 350 lines — still manageable. `DiagnosticKind`, `SymbolEntry`,
  `FileEntry`, newtypes all go here.
- **Library errors**: `schema::Error` (thiserror) stays unchanged. No new
  variants — validation failures are soft `ErrorEntry` values, not propagated
  errors.
- **`Config`**: Stays in `schema.rs`. Gains `validate: bool` field with
  `#[builder(default)]`.
- **`WorkspaceMap`**: Stays in `schema.rs`. Gains three BTreeMap fields.

---

## 3. Pattern Requirements

### Must-follow patterns

1. **Builder pattern**: All new structs (`SymbolEntry`, `FileEntry`,
   `FileLookupResult`, `DisambiguationHint`) use `#[derive(bon::Builder)]`.
   Construction uses `.builder().field(v).field(v).build()`.

2. **Serde camelCase**: All serialized structs use
   `#[serde(rename_all = "camelCase")]`. Enums use `#[serde(rename_all = "snake_case")]`
   for variant names (DiagnosticKind, SymbolLookupResult).

3. **`skip_serializing_if`**: `Option` fields use
   `#[serde(skip_serializing_if = "Option::is_none")]`. `Vec` fields use
   `#[serde(skip_serializing_if = "Vec::is_empty")]`. Empty collections are
   absent from JSON output.

4. **Must-use annotations**: All public functions in new modules use
   `#[must_use]` (consistent with `file_parser.rs` convention).

5. **Iterator pipelines**: `indexes.rs` uses `flat_map` / `fold` for the
   `symbols`/`name_index` pass. `files` construction uses a `for` loop
   (accumulator dependency) with `.iter().map()` chains internally. No
   `mut` accumulator variables for the symbols/name_index pass.

6. **`Option` as iterator**: `FileEntry.parent_module_file` is consumed via
   `.into_iter()` in pipeline contexts (per coding style section of plan).

7. **Deterministic ordering**: `symbols`, `name_index`, and `files` use
   `BTreeMap<String, ...>` — sorted by key, deterministic. The iteration
   order in `derive_from_crates` must be deterministic: iterate `crates`
   sorted by name (already guaranteed by `lib.rs`), iterate modules in
   the order returned by `build_module_tree` (depth-first, deterministic).
   Sort `name_index` Vec values by canonical path for deterministic order.

8. **Error handling architecture**: Fatal errors (workspace not found,
   missing `[workspace]`) propagate via `anyhow::Result`. Soft errors
   (parse failures, validation findings) collect as `ErrorEntry` values
   in `WorkspaceMap.errors`. No new fatal error variants.

### Must-avoid patterns

1. **No speculative additions**: `Confidence`, `Warning`, `WarningKind`,
   `ImportSite`, `ReExportSite`, `warnings: Vec<Warning>` are all deferred.
   Do not add their struct definitions, enum variants, or fields.

2. **No `expect`/`unwrap` in new code**: Use `Result`, `Option`, `?`,
   `unwrap_or_else`, or let-else with meaningful error messages. The only
   exception is test code.

3. **No `--from-stdin`**: Deferred. Do not add `serde::Deserialize` to
   output types for stdin piping.

4. **No separate binary**: Do not create `src/bin/lookup.rs` or similar.
   Subcommands live in `main.rs` with `clap::Subcommand`.

5. **No opportunistic refactoring in existing modules**: Touch
   `module_tree.rs`, `file_parser.rs`, `cross_refs.rs` only where bug fixes
   or `DiagnosticKind` migration requires it. Do not "clean up" existing code.

---

## 4. Key Type Signatures (Exact)

### In `src/schema.rs`:

```rust
// Newtypes
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalPath(pub String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkspaceRelativePath(pub String);

// impl Display, AsRef<str>, From<String>, Serialize, Deserialize for both

// New diagnostic kind (replaces String in ErrorEntry)
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticKind {
    OrphanedModule,
    TomlParseError,
    MissingCrateRoots,
    ModuleTreeError,
    SynParseError,
    MissingWorkspaceSection,
    GlobPatternError,
    MemberNotFound,
    OrphanFile,
    DeadReExport,
}

// Modified ErrorEntry
pub struct ErrorEntry {
    pub file: String,
    #[builder(default)]
    pub line: usize,
    pub message: String,
    pub severity: ErrorSeverity,
    pub kind: DiagnosticKind,        // WAS: kind: String
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ErrorContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}

// New flat-index entries
pub struct SymbolEntry {
    pub crate_name: String,
    pub module: String,
    pub file: String,
    pub line: usize,
    pub kind: ItemKind,
}

pub struct FileEntry {
    pub module_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_module_file: Option<String>,
    pub is_crate_root: bool,
}

// Extended WorkspaceMap
pub struct WorkspaceMap {
    // ...existing fields unchanged...
    #[builder(default)]
    pub symbols: BTreeMap<String, SymbolEntry>,
    #[builder(default)]
    pub name_index: BTreeMap<String, Vec<String>>,
    #[builder(default)]
    pub files: BTreeMap<String, FileEntry>,
    pub errors: Vec<ErrorEntry>,
    #[serde(skip)]
    pub workspace_root: PathBuf,
}

// Modified Config
pub struct Config {
    pub workspace_path: PathBuf,
    pub output_path: Option<PathBuf>,
    #[builder(default)]
    pub validate: bool,
}
```

### In `src/indexes.rs`:

```rust
#[must_use]
pub fn derive_from_crates(
    crates: &[CrateInfo],
) -> (
    BTreeMap<String, SymbolEntry>,
    BTreeMap<String, Vec<String>>,
    BTreeMap<String, FileEntry>,
)
```

### In `src/validate.rs`:

```rust
#[must_use]
pub fn validate(
    crates: &[CrateInfo],
    symbols: &BTreeMap<String, SymbolEntry>,
    workspace_root: &Path,
) -> Vec<ErrorEntry>
```

### In `src/lookup.rs`:

```rust
pub struct FileLookupResult { ... }
pub enum SymbolLookupResult { Found(SymbolEntry), Ambiguous { ... }, NotFound }
pub struct DisambiguationHint { ... }

#[must_use]
pub fn lookup_symbol(map: &WorkspaceMap, name: &str) -> SymbolLookupResult

#[must_use]
pub fn lookup_file(map: &WorkspaceMap, file: &str) -> Option<FileLookupResult>
```

### In `src/lib.rs`:

```rust
pub fn build_map(config: &Config) -> anyhow::Result<WorkspaceMap>
pub fn run(config: &Config) -> anyhow::Result<()>   // existing, refactored to call build_map
```

### In `src/module_tree.rs`:

```rust
// Changed signature (private):
fn process_module_info(
    module_path: &str,
    file_path: &Path,
    visibility: &str,
    file_info: &FileInfo,
    items: &[syn::Item],
    _parent_dir: &Path,
    visited: &mut HashSet<PathBuf>,
    errors: &mut Vec<ErrorEntry>,
) -> Vec<ModuleInfo>       // WAS: (Vec<ModuleInfo>, Vec<ErrorEntry>)
```

---

## 5. Known Pitfalls and Constraints

### Pitfall 1: Serialization drift on `DiagnosticKind`
The plan's verification step 4 demands that `orphaned_module` (the string)
still appears in JSON output. Use `#[serde(rename_all = "snake_case")]` at
the enum level and verify with an integration test. **Do not** use per-variant
`#[serde(rename = "...")]` — it's error-prone for 10 variants and harder
to audit. The only possible mismatch is `DeadReExport` → `dead_re_export`
(vs `dead_reexport` if someone expects a different convention). `snake_case`
splits `DeadReExport` as `dead_re_export` — verify against the plan's
expectation (the plan has no explicit serde-rename for `DeadReExport` in
its table, so `dead_re_export` is the canonical form).

### Pitfall 2: `ErrorEntry` emit-site migration
Every construct-site that currently does `.kind("some_string".to_string())`
must change to `.kind(DiagnosticKind::Variant)`. These sites span 4 files
(`lib.rs`, `module_tree.rs`, `file_parser.rs`, and potentially `workspace.rs`).
Missing one produces a compile error (good), but getting the wrong variant
produces a silent JSON-value change that the regression test catches.

### Pitfall 3: `cross_references.types` re-keying breaks downstream consumers
The key format changes from `"Task"` to `"core::Task"`. Any pipeline code
that reads `crossReferences.types["Task"]` must update to
`crossReferences.types["core::Task"]`. In the pipeline integration
(`compile-plan` SKILL.md and `plan-decomposer`), verify that no code
hardcodes short-name access. The `lookup --symbol Task` path is unaffected —
it uses `name_index`, not direct `cross_references.types` access.

### Pitfall 4: Inline module detection edge cases
The "first module encountered per file" heuristic for inline detection
assumes depth-first ordering from `build_module_tree`. This holds for the
current implementation but:
- If `build_module_tree` changes traversal order, the heuristic silently
  breaks.
- A file with two external modules (not possible in current Rust module
  system) would cause the second to be incorrectly treated as inline.
These are acceptable risks for MVP; document the assumption in a code comment.

### Pitfall 5: `canonical_path` for the crate root module
For root-level items, the canonical path is `"crate_name::ItemName"` (2
segments). For nested items, `"crate_name::module::ItemName"` (3+ segments).
The `derive_from_crates` algorithm uses `format!("{}::{}", module.path, item.name)`.
For the root module, `module.path` is the crate name (e.g., `"core"`), producing
`"core::Task"`. Correct. For a nested module, `module.path` is `"core::foo"`,
producing `"core::foo::Bar"`. Correct.

### Pitfall 6: Mocking in tests
`validate.rs` contains filesystem I/O (walking `src/` for orphan files). Make
the directory-walk path a parameter (`crate_src_dir: &Path`) so tests can
point it at temp directories. Do NOT mock the filesystem — use real temp
directories (same pattern as integration tests with `tempfile`).

### Pitfall 7: `test_deterministic_output` after schema additions
The additive schema fields (`symbols`, `name_index`, `files`) change the
JSON output. The deterministic-output test compares byte-identical runs
(not pre- and post-change output), so it remains valid — two runs of the
new binary should produce identical output.

### Pitfall 8: Exit-code propagation for `index --validate`
The exit code `2` (validation found issues) must be checked AFTER the map
is successfully built. If the map fails to build (workspace not found,
parse errors), exit code is `1` (tool error), not `2`. The precedence in
`main.rs`:
```
1. If build_map() returns Err => exit 1
2. If --validate && any Warning in errors with OrphanFile/DeadReExport kind => exit 2
3. Otherwise => exit 0
```

### Pitfall 9: `Config.validate` defaults to false
The `#[builder(default)]` on `validate: bool` makes it `false` by default.
The CLP `--validate` flag sets it to `true`. If someone constructs `Config`
manually (rare, but test code might), omitted means no validation. This is
correct — validation is opt-in.

---

## 6. Task Grouping Rationale

The plan breaks into task groups ordered by dependency chain. Each group can
be implemented, tested, and committed independently (incremental integration).

### Group 1: Schema foundations
- `src/schema.rs`: Add `CanonicalPath`, `WorkspaceRelativePath`, `DiagnosticKind`, `SymbolEntry`, `FileEntry`
- `src/schema.rs`: Extend `WorkspaceMap` with `symbols`, `name_index`, `files`
- `src/schema.rs`: Migrate `ErrorEntry.kind` from `String` to `DiagnosticKind`
- `src/schema.rs`: Add `validate: bool` to `Config`

**Why first**: Everything else depends on these types. Without them, no other
module compiles. This is the foundation task.

**Test**: Compile check only. No runtime tests needed — these are data types
with no behavior.

### Group 2: Bug fixes (no schema dependency beyond current state)
- `src/lib.rs:132-137`: Fix dropped-errors bug
- `src/module_tree.rs:252`: Fix `errors.clone()` (change `process_module_info` return type)
- `src/module_tree.rs: ~line 183`: Fix module-drop bug (incidental finding from the clone fix)
- `src/lib.rs:150`: Fix `WorkspaceInfo.root` from `"."` to actual path

**Why second**: These are low-risk, isolated fixes. They reduce regression
surface before introducing new features. The `errors.clone()` fix in
`module_tree.rs` touches the function signature — doing it before adding new
features prevents merge conflicts.

**Why decoupled from Group 1**: Bug fixes compile against the current schema
(no new types needed). The `DiagnosticKind` migration in Group 1 changes
the emit-sites; doing the bug fixes first means Group 3 doesn't need to
juggle both the migration AND the structural changes simultaneously.

**Test**: Existing integration tests pass. No new test behavior —
just reliable error collection.

### Group 3: Emit-site migration
- `src/lib.rs`: Change all `.kind("...")` to `.kind(DiagnosticKind::...)`
- `src/module_tree.rs`: Same
- `src/file_parser.rs`: Same
- `src/workspace.rs`: Same (if emit-sites exist)

**Why third**: Depends on `DiagnosticKind` existing (Group 1). The mechanical
nature makes it quick. Doing this BEFORE `indexes.rs` means `indexes.rs`
writes fresh code with the new types from the start.

**Test**: Compile check + regression test for `"orphaned_module"` string in
JSON (verification step 4).

### Group 4: Flat index derivation (`indexes.rs`)
- `src/indexes.rs`: New file, `derive_from_crates()`
- `src/lib.rs`: Call `derive_from_crates()` after `cross_refs::compute()`,
  pass results to `WorkspaceMap::builder()`

**Why fourth**: Depends on schema types (Group 1) and correct error collection
(Group 2). The cross-references re-keying can be done here or as a predecessor
step — it shares the canonical-path construction logic.

**Test**: Unit tests in `src/indexes.rs` with hand-constructed `CrateInfo`
values. Integration test: the new `symbols`/`name_index`/`files` fields appear
in JSON output against `sample-workspace`.

### Group 5: Cross-references re-keying
- `src/cross_refs.rs`: Change `TypeRef` key from short-name to canonical path
  (`{module.path}::{item.name}`)

**Why fifth**: Can be done independently of indexes, but `derive_from_crates`
builds identical canonical paths — re-keying first avoids two different
canonical-path constructions. Alternatively, do them together (shared logic).

**Test**: Update `cross_refs.rs` unit tests. Integration test:
`crossReferences.types` keys are canonical paths.

### Group 6: Validation (`validate.rs`)
- `src/validate.rs`: New file, `validate()` with `check_orphan_files()` and
  `check_dead_reexports()`
- `src/lib.rs`: Call `validate()` in `build_map()` when `config.validate` is true

**Why sixth**: Depends on indexes (`symbols` map for DeadReExport lookup),
schema types, and correct error collection. The exit-code 2 logic in `main.rs`
depends on the `validate` flag being wired through `Config`.

**Test**: New integration tests with `bad-orphan/` and `bad-dead-reexport/`
fixtures. Unit tests for individual rules (pass hand-constructed ModuleInfo
and symbols map, verify ErrorEntry output).

### Group 7: CLI refactor
- `src/main.rs`: Subcommand enum (`Index`, `Lookup`), bare-path removal
- `src/lib.rs`: Extract `build_map()` from `run()`

**Why seventh**: Depends on `Config.validate` (Group 1), `build_map()` needs
all pipeline steps wired (Groups 2–6). The `build_map()` extraction is
blocked on: knowing exactly which steps to include (so after Groups 2, 4, 6).

**Test**: Rewrite ALL existing integration tests from bare-path to `index
<path>`. Add exit-code assertion tests. This is the flag-day commit.

### Group 8: Lookup (`lookup.rs`)
- `src/lookup.rs`: New file, `lookup_symbol()`, `lookup_file()`
- `src/main.rs`: Wire `lookup` subcommand to `lookup_*` functions

**Why eighth**: Depends on CLI refactor (Group 7 — `lookup` is a subcommand)
and flat indexes being in `WorkspaceMap` (Group 4). The lookup functions are
pure — they only read the map. No dependency on validation.

**Test**: `lookup --symbol Task` (found), `lookup --symbol DoesNotExist`
(not found), `lookup --symbol` with ambiguous name (multi-crate collision),
`lookup --file core/src/task.rs`.

### Group 9: Real-world dry-run and pipeline integration
- Run `rust-workspace-map index --validate` against `castep-cell-io` (303 files)
- Record wall-time as cache baseline
- Modify `rust-development-pipeline/skills/compile-plan/SKILL.md`
- Modify `rust-development-pipeline/agents/plan-decomposer.md`

**Why ninth**: Depends on a fully working binary (Groups 1–8). The pipeline
smoke test (verification step 8) requires both repos.

### Group sequencing summary

```
G1 (schema) ──── G3 (emit-site migration)
    │                │
    └── G4 (indexes) │
         │           │
         ├── G5 (cross_refs re-key)    ← can be parallel with G4
         │
    G2 (bug fixes) ── independent, can run anytime before G7
         │
         └── G6 (validate) ── depends on G4 (symbols map)
              │
              └── G7 (CLI refactor) ── depends on G1, G2, G4, G6
                   │
                   └── G8 (lookup) ── depends on G4, G7
                        │
                        └── G9 (dry-run + pipeline)
```

G2 (bug fixes) is intentionally parallelizable with G1–G5. It touches
different code paths (error handling in `module_tree.rs`, `lib.rs` loops)
and can be done before or after the schema changes. Doing it before G7
is the only hard requirement (because `build_map()` must include the fixed
error collection).

---

## 7. Verification Checklist Mapping

Each verification step in the plan maps to a task group:

| Step | Description | Tested by |
|---|---|---|
| 1. Build green | `cargo build --release` | Every group |
| 2. Call-site sweep | All invocations → `index <path>` | G7 |
| 3. Existing tests green | 10 integration tests pass with additive schema | G7 |
| 4. DiagnosticKind migration | `orphaned_module` string preserved in JSON | G3 |
| 5. New unit fixtures | `bad-orphan/` + `bad-dead-reexport/` exit 2 | G6 |
| 6. Lookup | `--symbol` and `--file` queries return correct results | G8 |
| 7. Dry-run + cache baseline | Wall-time recorded | G9 |
| 8. Pipeline smoke | `compile-plan` pre-check blocks on violations | G9 |
