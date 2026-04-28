# Phase 0.2 — Draft Elaboration

Grounded in existing patterns from `codebase-state.md`. No new patterns invented.

---

## Item: Add ErrorSeverity enum to schema

**Proposed type signature:**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorSeverity {
    Error,
    Warning,
}
```

**Module placement:** `src/schema.rs`, placed immediately before the `ErrorEntry` struct definition (line ~300, same section block).

**Error handling strategy:** New standalone enum. `serde::Serialize` with `rename_all = "snake_case"` follows the convention already established by `CrateType` and `ItemKind`. No `thiserror` involvement.

**Ownership/lifetime notes:** None. Copy, Clone — cheap to copy into every ErrorEntry.

**Trait coherence notes:** None. No generics, no external trait bounds beyond serde and standard derives.

---

## Item: Add ErrorContext struct to schema

**Proposed type signature:**
```rust
#[derive(Debug, Clone, Default, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ErrorContext {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crate_name: Option<String>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_path: Option<String>,

    /// Line number in the source file where the error occurred.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<usize>,

    /// A short source snippet near the error location (if available).
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
}
```

**Module placement:** `src/schema.rs`, placed immediately before `ErrorEntry` struct.

**Error handling strategy:** New struct with `Default`. Uses `Option<T>` for all fields so a fully-populated context and a minimal one (just `line`) are both valid. The `bon::Builder` defaults follow the pattern already used everywhere in schema.rs.

**Ownership/lifetime notes:** None. All `String`/`usize` owned types.

**Trait coherence notes:** None. `bon::Builder` requires types to be clonable or constructible — `Option<String>` and `Option<usize>` both satisfy this.

---

## Item: Expand ErrorEntry with new fields

**Proposed type signature (full struct):**
```rust
#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ErrorEntry {
    pub file: String,
    #[builder(default)]
    pub line: usize,
    pub message: String,
    pub severity: ErrorSeverity,
    pub kind: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ErrorContext>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}
```

**Module placement:** `src/schema.rs`, replacing the existing 3-field `ErrorEntry` (lines 302–309).

**Error handling strategy:** `severity` is required (no Option) — every ErrorEntry always has a severity. `kind` is required (no Option) — every ErrorEntry always has a machine-readable tag. `context` and `cause` are optional for backwards compatibility with existing callers during construction.

**Ownership/lifetime notes:** None.

**Trait coherence notes:** `bon::Builder` on `ErrorSeverity` (Copy + Clone) — no issue. The existing `WorkspaceMap::builder().errors(vec)` pattern works unchanged since `Vec<ErrorEntry>` is the same type.

**Implementation detail:** The `WorkspaceMap::builder()` call in `lib.rs::run()` currently passes `.errors(errors)` where `errors: Vec<ErrorEntry> = Vec::new()`. Since `ErrorEntry` is changed, callers constructing ErrorEntry values must use the new builder which now requires `severity` and `kind` (no defaults for those). This is intentional — it is impossible to construct an ErrorEntry without specifying what kind of error it is.

---

## Item: Add MissingWorkspaceSection to Error enum

**Proposed type signature (enum addition):**
```rust
#[error("workspace Cargo.toml is missing the [workspace] section")]
MissingWorkspaceSection,
```

**Module placement:** `src/schema.rs`, added to the `Error` enum after `GlobPattern` (before the closing `}`).

**Error handling strategy:** New unit variant. No source error, no associated data. The message is a static string.

**Ownership/lifetime notes:** None.

**Trait coherence notes:** None.

**Caller wiring:** In `workspace.rs::enumerate_members()`, the current code on lines 37–46 uses `unwrap_or_default()` on the `workspace.members` lookup. The change is:
- After the `members` extraction, check if the `[workspace]` table was absent entirely (not just if `members` was absent). Return `Err(Error::MissingWorkspaceSection)` when the workspace table itself is missing.
- When the workspace table exists but `members` is absent (empty array or missing key), keep returning an empty `Vec<PathBuf>` (this is valid for a workspace that just has `exclude`).

**Uncertain:** The distinction between "no [workspace] section at all" vs "[workspace] section with no members" matters. If `parsed.get("workspace")` returns `None`, it is `MissingWorkspaceSection`. If it returns `Some` but `members` is missing/empty, return empty vec. This preserves backwards compatibility for workspaces that have `[workspace]` but no members (valid TOML).

---

## Item: Change parse_file to return errors alongside results

**Proposed type signature:**
```rust
/// On parse failure, returns the original file content and a
/// `SynParseError` alongside an empty `FileInfo`. Callers use the
/// error to construct an `ErrorEntry`.
pub fn parse_file(path: &Path) -> ParsedFile {
```

Where `ParsedFile` is an internal type defined in `file_parser.rs`:

```rust
/// Result of parsing a Rust source file.
///
/// Unlike `Result<T, Error>`, this type always succeeds — parse
/// failures are reported as data, not as errors, so the caller
/// can continue processing other files. The caller constructs
/// `ErrorEntry` values from `SynParseError` when needed.
pub struct ParsedFile {
    pub ast: syn::File,
    pub file_info: FileInfo,
    pub parse_error: Option<SynParseError>,
}

pub struct SynParseError {
    pub message: String,
    pub line: usize,
}
```

**Module placement:** `src/file_parser.rs`, placed in a new section block before `parse_file`.

**Error handling strategy:** `parse_file` never returns `Err` — it always returns `Ok(ParsedFile)`. Parse errors are captured in the `parse_error` field. This is the "partial results" approach from the plan: the function collects errors rather than propagating them.

**Why not `Result<(syn::File, FileInfo), ErrorEntry>`:** Because `build_module_tree` already uses `?` on `parse_file` calls. If `parse_file` returned `Err`, it would abort the entire crate — violating the partial results requirement. An internal `ParsedFile` type avoids changing the public signature while solving the problem internally.

**Ownership/lifetime notes:** `ParsedFile` owns the AST and FileInfo. `SynParseError` owns only a message string (no source span from syn — we extract line from `syn::Error` but the full diagnostic is captured as a string).

**Trait coherence notes:** None. No generics.

**Call-site changes:** All `let (ast, file_info) = parse_file(path)?;` calls become:
```rust
let parsed = parse_file(path);
if let Some(err) = &parsed.parse_error {
    errors_vec.push(build_parse_error_entry(path, err));
}
let ast = parsed.ast;
let file_info = parsed.file_info;
```

The `build_parse_error_entry` helper constructs an `ErrorEntry` from the path and `SynParseError`:
```rust
fn build_parse_error_entry(path: &Path, err: &SynParseError) -> ErrorEntry {
    ErrorEntry::builder()
        .file(path.to_string_lossy().to_string())
        .line(err.line)
        .message(err.message.clone())
        .severity(ErrorSeverity::Error)
        .kind("syn_parse_error".to_string())
        .build()
}
```

---

## Item: Change build_module_tree to collect errors

**Proposed type signature:**
```rust
/// Same as before, but errors from submodule parsing are collected
/// into the returned `Vec<ModuleInfo>` as placeholder entries with
/// an `ErrorEntry` attached via `ModuleInfo.extra_error` — NO, that
/// would break the schema. Instead, errors are returned via a
/// side-channel: `run()` collects them.
///
/// Actually: build_module_tree returns (Vec<ModuleInfo>, Vec<ErrorEntry>).
pub fn build_module_tree(crate_root: &Path, crate_name: &str) -> (Vec<ModuleInfo>, Vec<ErrorEntry>) {
```

**Module placement:** `src/module_tree.rs`.

**Error handling strategy:** `build_module_tree` no longer returns `Result`. Instead, it returns a tuple `(Vec<ModuleInfo>, Vec<ErrorEntry>)`. Parse errors are captured in the second element. Orphaned modules produce `ErrorEntry` with `kind = "orphaned_module"` instead of `eprintln!`. Cycle detection returns `Ok(vec![])` (no error entry — cycles are normal in Rust modules).

**Why change the return type:** The current `Result<Vec<ModuleInfo>>` signature forces single-error propagation via `?`. With parallel rayon processing in `run()`, we need to collect multiple errors per crate. Returning a tuple lets callers extract errors without aborting.

**Trait coherence notes:** Changing the return type of a public function. The callers are only in `lib.rs::run()`. This is an internal API surface — no downstream crates call `module_tree::build_module_tree`.

**Call-site changes in `lib.rs`:**
```rust
// Before:
module_tree::build_module_tree(root, &pkg_name).unwrap_or_default()

// After:
let (mods, errs) = module_tree::build_module_tree(root, &pkg_name);
crate_errors.extend(errs);
mods
```

---

## Item: Fix path fallback in module_tree.rs (Goal 3)

**Three occurrences to fix:**

### Fix 1: `build_module_tree` line 25
```rust
// Before:
let parent_dir = crate_root.parent().unwrap_or_else(|| Path::new("."));

// After:
let parent_dir = crate_root.parent().unwrap_or(crate_root);
```

**Rationale:** When `crate_root` is at the filesystem root (extremely rare), `.parent()` returns `None`. Using `crate_root` itself as the fallback is a safe no-op — the caller then tries `{crate_root}/{mod_name}.rs` which is the same as the original path. This is better than `"."` which silently produces wrong relative paths.

### Fix 2: `process_submodule` line 155
```rust
// Before:
&file_path.parent().unwrap_or_else(|| Path::new("."))

// After:
file_path.parent().unwrap_or(file_path)
```

Same rationale. The `file_path` here is the parent directory for recursive `process_submodule` calls.

### Fix 3: `process_module_info` line 203
```rust
// Before:
let child_dir = file_path.parent().unwrap_or_else(|| Path::new("."));

// After:
let child_dir = file_path.parent().unwrap_or(file_path);
```

Same rationale.

**Module placement:** `src/module_tree.rs`, inline replacements.

**Ownership/lifetime notes:** None. All `&Path` borrows.

---

## Item: Fix enumerate_members path fallback (Goal 3)

**Proposed change in `workspace.rs`:**
```rust
// Before (lines 37-46):
let members: Vec<String> = parsed
    .get("workspace")
    .and_then(|w| w.get("members"))
    .and_then(|m| m.as_array())
    .map(|arr| {
        arr.iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect()
    })
    .unwrap_or_default();

// After:
let members = match parsed.get("workspace") {
    None => return Err(Error::MissingWorkspaceSection),
    Some(workspace) => workspace
        .get("members")
        .and_then(|m| m.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default(),
};
```

**Module placement:** `src/workspace.rs`, lines 37–46.

**Error handling strategy:** Returns `Error::MissingWorkspaceSection` (new variant) when the `[workspace]` section is entirely absent. When `[workspace]` exists but `members` is absent or empty, returns empty vec (valid for workspace with only `exclude`).

---

## Item: Convert eprintln! paths in run() to ErrorEntry (Goal 1)

### Cargo parse failure (lines 46-56)
```rust
// Before:
Err(e) => {
    eprintln!("warning: failed to parse {}: {}", cargo_toml.display(), e);
    return None;
}

// After:
Err(e) => {
    crate_errors.push(ErrorEntry::builder()
        .file(cargo_toml.to_string_lossy().to_string())
        .message(format!("failed to parse Cargo.toml: {}", e))
        .severity(ErrorSeverity::Error)
        .kind("toml_parse_error".to_string())
        .cause(e.to_string())
        .build());
    return None;
}
```

### Resolve crate roots empty (lines 58-65)
```rust
// Before:
if roots.is_empty() {
    eprintln!("warning: no crate entry points found in {}", dir.display());
    return None;
}

// After:
if roots.is_empty() {
    crate_errors.push(ErrorEntry::builder()
        .file(dir.to_string_lossy().to_string())
        .message("no crate entry points found".to_string())
        .severity(ErrorSeverity::Warning)
        .kind("missing_crate_roots".to_string())
        .build());
    return None;
}
```

**Severity rationale:** `missing_crate_roots` is a `Warning` — the run completed fully, this crate just has no entry points. `toml_parse_error` is an `Error` — data loss for this crate.

### Module tree error (lines 78-80)
```rust
// Before:
module_tree::build_module_tree(root, &pkg_name).unwrap_or_default()

// After:
let (mods, tree_errors) = module_tree::build_module_tree(root, &pkg_name);
crate_errors.extend(tree_errors);
mods
```

### Orphaned modules in module_tree.rs (lines 106-113, 134-141)
```rust
// Before (both occurrences):
eprintln!("warning: orphaned module {}", module_path);
return Ok(vec![ModuleInfo::builder()
    .path(module_path.to_string())
    .file("<unresolved>".to_string())
    .visibility("private".to_string())
    .build()]);

// After (both occurrences):
crate_errors.push(ErrorEntry::builder()
    .file(String::new())
    .message(format!("orphaned module: {}", module_path))
    .severity(ErrorSeverity::Warning)
    .kind("orphaned_module".to_string())
    .context(ErrorContext::builder()
        .module_path(module_path.to_string())
        .build())
    .build());
return Ok(vec![ModuleInfo::builder()
    .path(module_path.to_string())
    .file("<unresolved>".to_string())
    .visibility("private".to_string())
    .build()]);
```

**Module placement:** `src/module_tree.rs` for orphaned module changes. `src/lib.rs` for run() changes.

---

## Item: Parallel error collection in run()

**Current state:** `run()` uses `.par_iter().filter_map().collect()` on `member_dirs`. Errors are currently `eprintln!`d inside the parallel closure, which is racy with stdout.

**Proposed approach:** Use `rayon::scope` with shared `Vec<ErrorEntry>` via interior mutability, or collect per-thread errors and merge afterward.

**Best approach matching existing patterns:** Collect per-crate errors alongside crate info using a parallel-join pattern:

```rust
let mut crate_errors: Vec<ErrorEntry> = Vec::new();

let mut crate_infos: Vec<CrateInfo> = member_dirs
    .par_iter()
    .map(|dir| {
        let mut crate_errors = Vec::new();
        // ... processing with errors pushed to crate_errors ...
        (crate_info, crate_errors)
    })
    .unzip::<_, _, Vec<CrateInfo>, Vec<Vec<ErrorEntry>>>();

for errs in crate_infos_errors {
    crate_errors.extend(errs);
}
```

**Uncertain:** `unzip` on a `par_iter().map()` returning tuples is not directly supported by rayon — `unzip` is a sequential iterator trait. The correct approach is:

```rust
let results: Vec<(CrateInfo, Vec<ErrorEntry>)> = member_dirs
    .par_iter()
    .map(|dir| {
        let mut crate_errors = Vec::new();
        // ... process, push errors ...
        (crate_info, crate_errors)
    })
    .collect();

let mut crate_errors = Vec::new();
let crate_infos: Vec<CrateInfo> = results
    .into_iter()
    .map(|(info, errs)| {
        crate_errors.extend(errs);
        info
    })
    .collect();
```

This collects parallel results into `(CrateInfo, Vec<ErrorEntry>)` tuples, then sequentially extracts errors. The parallel work is in `.map()`, the error merge is sequential over the already-collected results (typically tens of entries, negligible cost).

**Module placement:** `src/lib.rs`, `run()` function.

**Trait coherence notes:** `CrateInfo` and `Vec<ErrorEntry>` must both be `Send + Sync` for rayon. `CrateInfo` contains `String`, `Vec<ModuleInfo>`, etc. — all `Send + Sync`. `ErrorEntry` contains `String`, `ErrorSeverity`, `Option<ErrorContext>` — all `Send + Sync`. Coherence is satisfied.

---

## Item: Replace errors Vec with populated collection in WorkspaceMap builder

**Current:** `errors(errors)` where `errors: Vec<ErrorEntry> = Vec::new()`.

**After:** Collect all errors from parallel processing into the vec, then pass it:
```rust
let map = WorkspaceMap::builder()
    .workspace(workspace_info)
    .crates(crate_infos)
    .cross_references(cross_refs)
    .errors(crate_errors)
    .workspace_root(workspace_root.clone())
    .build();
```

**Module placement:** `src/lib.rs`, `run()` function.

---

## Item: Unit test suite (Goal 2)

All tests use `#[cfg(test)]` modules in the respective source files, following the existing pattern where `file_parser.rs` and `module_tree.rs` already have internal helper functions that are testable in isolation.

### workspace.rs tests
- `find_workspace_root`: Test with `tempfile::tempdir()`, create a minimal workspace Cargo.toml, verify path resolution.
- `enumerate_members`: Test with `tempfile::tempdir()` — create workspace with members, verify returned paths. Test with missing workspace section — verify `Err(MissingWorkspaceSection)`. Test with exclude list — verify excluded crate absent.
- `resolve_crate_roots`: Test with `tempfile::tempdir()` — create lib.rs only, main.rs only, both. Verify returned `CrateType`.

### cargo_info.rs tests
- `parse_cargo_toml`: Parse a minimal Cargo.toml string using `tempfile::tempdir()`, verify `PackageInfo` fields. Test missing `[package]` table — verify defaults (name="unknown", edition="2021"). Test valid dependencies — verify normal/dev/workspace distinction.

### file_parser.rs tests
- `parse_file`: Parse inline Rust string via `tempfile::tempdir()` — create file with struct/enum/trait, verify extracted types. Test parse failure — verify `SynParseError` is populated in `ParsedFile::parse_error`. Test empty file — verify empty results (not error).
- `extract_public_items`: Pass `syn::File` constructed from parsed string — verify correct item kinds extracted. Test no public items — verify empty vec.
- `extract_imports`: Parse `use std::collections::BTreeMap;` — verify single import. Test braced imports — verify expansion to individual entries.
- `extract_re_exports`: Parse `pub use crate::foo;` — verify re-export captured. Test `pub use crate::foo as bar;` — verify rename.
- `extract_submodules`: Parse `mod foo;` and `#[cfg(test)] mod bar;` — verify first extracted, second marked `is_test: true`.
- `extract_impls`: Parse impl block with fn/type/const items — verify all three kinds captured.

### module_tree.rs tests
- `resolve_module_path`: Test with `tempfile::tempdir()` — create `{dir}/foo.rs` and `{dir}/foo/mod.rs`, verify correct path for each. Test non-existent module — verify None.
- `build_module_tree`: Test with a multi-file workspace crate — verify module hierarchy, paths, and public items. Test with parse-failure submodule — verify `ErrorEntry` in errors vec, other modules still extracted.

### cross_refs.rs tests
- `compute`: Build two `CrateInfo` values manually (no fixtures) — one exports `Task`, the other imports `core::Task`. Verify `cross_crate_imports` populated and `CrossReferences.types` contains the reference.

### render.rs tests
- `render_json`: Build a minimal `WorkspaceMap` manually, serialize, verify JSON contains expected keys. Test with empty errors — verify `errors` field is absent from JSON (skip_serializing_if).
- `render_to_writer`: Build minimal map, write to `Vec<u8>`, verify output matches `render_json`.

**Module placement:** `#[cfg(test)] mod tests { ... }` at the end of each source file. Integration tests in `tests/integration_test.rs`.

**Test fixtures:** `tempfile::tempdir()` is sufficient for most tests. The existing `tests/fixtures/sample-workspace/` is an integration test fixture that could be extended with additional sub-fixtures for the 7 integration tests listed in the plan.

---

## Item: Integration test expansion (Goal 4)

### 7 new integration tests needed:

1. **Parse failure workspace:** Create a workspace with one well-formed crate and one crate containing invalid Rust syntax. Run binary, verify `ErrorEntry` with `kind: "syn_parse_error"` and `severity: "error"` appears in JSON output.

2. **Missing workspace section:** Create a `Cargo.toml` with `[dependencies]` but no `[workspace]` section. Run binary, verify `ErrorEntry` with `kind: "missing_workspace_section"` appears (or the binary exits non-zero via `Error::MissingWorkspaceSection` propagated through `anyhow`).

3. **Glob member patterns:** Create a workspace with `members = ["crates/*"]` containing 3 sub-crate directories. Run binary, verify all 3 crates appear in output.

4. **Workspace with exclude:** Create a workspace with `members = ["a", "b", "c"]` and `exclude = ["b"]`. Run binary, verify only `a` and `c` appear in output.

5. **Deeply nested modules:** Create a crate with 3+ levels of `mod` declarations (`src/lib.rs` → `src/foo.rs` → `src/foo/bar.rs` → `src/foo/bar/baz.rs`). Verify correct module paths (`foo::bar::baz`), file references, and hierarchy in output.

6. **Re-export chains:** Create a crate with `pub use inner::Secret;` where `mod inner { pub struct Secret; }`. Verify `ReExport` entries are populated with correct import/export paths.

7. **Output via -o flag:** Run binary with `-o /tmp/test-output.json`, read the file, verify content matches stdout output from a separate run (byte-identical).

**Module placement:** `tests/integration_test.rs`.

**Fixture strategy:** Each test creates its fixture in a `tempfile::tempdir()` and passes the path to the binary via CLI argument. No need for new files under `tests/fixtures/`.

---

## Item: Remove clippy suppressions (Goal 5)

### Suppression by suppression:

**`missing_errors_doc`:** Add `# Errors` doc sections to `parse_file`, `build_module_tree` (signature change), and `enumerate_members` (now returns error for missing section). Other functions either return `Result` and have existing docs, or don't return `Result` and don't need the section.

**`must_use_candidate`:** Add `#[must_use]` to pure functions returning `Vec<...>`: `extract_public_items`, `extract_imports`, `extract_re_exports`, `extract_submodules`, `extract_impls`, `resolve_module_path`, `render_json`, `kind_to_string`. Functions with side effects or that are builder-style do not get `#[must_use]`.

**`doc_markdown`:** Fix doc comments that contain identifiers not wrapped in backticks. E.g., `syn::File` → already correct, but check for bare `Path`, `Vec`, `Option`, etc.

**`uninlined_format_args`:** Inline all `format!("{}", x)` to `format!("{x}")` and `eprintln!("text: {}", x)` to `eprintln!("text: {x}")`.

**`redundant_closure`:** Find closures like `.map(|x| foo(x))` and replace with `.map(foo)`.

**`collapsible_if`:** Merge nested `if` expressions.

**`needless_pass_by_value`:** Change function parameters from `String` to `&str` or `PathBuf` to `&Path` where callers only need to borrow.

**`needless_borrow`:** Remove unnecessary `&` in function calls.

**`redundant_closure_for_method_calls`:** Replace `.map(|x| x.method())` with `.map(|x| x.method())` clippy is fine... actually: `.iter().map(|s| s.to_string())` should become `.iter().map(ToString::to_string)` or `.cloned()` depending on context.

**Module placement:** `src/lib.rs` (removal of crate-level allows), then per-file fixes in each `src/*.rs`.

**Uncertain:** The exact locations of each clippy lint require running `cargo clippy -- -W clippy::pedantic` to see which lints actually fire. The disposition table in the plan maps each suppression to a fix action, but the actual code locations may differ from what's expected. A safe approach: remove all 9 crate-level allows, run clippy, fix each firing at the per-item level, then verify clean.

---

## Deferred Items Assessment

- **Empty errors vector in `run()`:** Absorbed — Goal 1 directly addresses this by wiring error collection into `run()`.
- **Silent error swallowing in `build_module_tree`:** Absorbed — Goal 1 changes `build_module_tree` to return `(Vec<ModuleInfo>, Vec<ErrorEntry>)` instead of `Result<Vec<ModuleInfo>>`, eliminating `unwrap_or_default()`.
- **Path fallback to `"."` in `module_tree.rs`:** Absorbed — Goal 3 fixes all three `unwrap_or_else(|| Path::new("."))` calls.
- **Path fallback to `""` in `workspace.rs`:** Absorbed — Goal 3 changes `enumerate_members` to return `Err(MissingWorkspaceSection)` when the workspace section is absent, and Goal 1 converts the remaining paths to ErrorEntry construction in `run()`.
- **No unit tests for public API functions:** Absorbed — Goal 2 adds comprehensive unit tests across all 5 core modules.

**Deferred items absorbed: 5/5. No items skipped.**
