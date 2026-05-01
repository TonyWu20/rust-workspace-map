# Raw Diff: `mvp-dev` -> `main`
Generated: 2026-04-29T19:29:15Z

## Commits
1438041 merge impl/mvp-fix: recursive orphan walk via walkdir + test cleanup
ed6dc9f fix(mvp-fix): recursive orphan walk via walkdir + test cleanup
0c1bfbb fix(mvp): resolve all clippy -D warnings
d663b9a feat(mvp): T9-T15: flat indexes, cross-refs re-keying, validation, CLI subcommands, lookup, tests
d14b866 feat(mvp): T1-T8 + T7: schema newtypes, DiagnosticKind, bugfixes, import migration
7d51fe9 docs(mvp): add coding style section to MVP plan
44dc853 docs(mvp): apply third-review amendments to MVP plan
587969c review(mvp): apply second-review structural amendments to MVP plan
b0f3a95 Add project memory about architecture note
783f4c3 docs: add current codebase architecture note
aec1b1c docs(mvp): tighten MVP plan after KISS / first-principles review
958b62d Clean up, prepare for mvp implementation phase

## Diff Stat
 .claude/hooks/current_task_TASK-PREP.json   |  24 ---
 CLAUDE.md                                   |   2 +
 Cargo.lock                                  |  29 +++
 Cargo.toml                                  |   1 +
 notes/architecture-current.md               | 223 ++++++++++++++++++++
 plans/compiled/TASK-1.py                    |  44 ----
 plans/compiled/TASK-1.sh                    |   7 -
 plans/compiled/TASK-10.py                   |  44 ----
 plans/compiled/TASK-10.sh                   |   7 -
 plans/compiled/TASK-2.py                    |  44 ----
 plans/compiled/TASK-2.sh                    |   7 -
 plans/compiled/TASK-3.py                    |  44 ----
 plans/compiled/TASK-3.sh                    |   7 -
 plans/compiled/TASK-4.py                    |  44 ----
 plans/compiled/TASK-4.sh                    |   7 -
 plans/compiled/TASK-5.py                    |  44 ----
 plans/compiled/TASK-5.sh                    |   7 -
 plans/compiled/TASK-6.py                    |  44 ----
 plans/compiled/TASK-6.sh                    |   7 -
 plans/compiled/TASK-7.py                    |  44 ----
 plans/compiled/TASK-7.sh                    |   7 -
 plans/compiled/TASK-8.py                    |  44 ----
 plans/compiled/TASK-8.sh                    |   7 -
 plans/compiled/TASK-9.py                    |  44 ----
 plans/compiled/TASK-9.sh                    |   7 -
 plans/compiled/TASK-PREP.py                 |  44 ----
 plans/compiled/TASK-PREP.sh                 |   7 -
 plans/compiled/manifest.json                | 139 -------------
 plans/mvp/MVP_PLAN.md                       | 304 ++++++++++++++++++++++++++++
 plans/mvp/MVP_PLAN_review.md                | 238 ++++++++++++++++++++++
 plans/mvp/MVP_PLAN_review2.md               | 224 ++++++++++++++++++++
 plans/{ => phase-0.1}/phase-0.1.toml        |   0
 src/cross_refs.rs                           |  50 +++--
 src/file_parser.rs                          |   6 +-
 src/indexes.rs                              | 285 ++++++++++++++++++++++++++
 src/lib.rs                                  |  67 ++++--
 src/lookup.rs                               | 205 +++++++++++++++++++
 src/main.rs                                 | 176 +++++++++++++---
 src/module_tree.rs                          |  27 +--
 src/schema.rs                               | 101 ++++++++-
 src/validate.rs                             | 242 ++++++++++++++++++++++
 tests/fixtures/bad-dead-reexport/Cargo.toml |   7 +
 tests/fixtures/bad-dead-reexport/src/lib.rs |   1 +
 tests/fixtures/bad-orphan/Cargo.toml        |   7 +
 tests/fixtures/bad-orphan/src/forgotten.rs  |   1 +
 tests/fixtures/bad-orphan/src/lib.rs        |   1 +
 tests/integration_test.rs                   | 236 ++++++++++++++++++++-
 47 files changed, 2348 insertions(+), 809 deletions(-)

## Full Diff
diff --git a/.claude/hooks/current_task_TASK-PREP.json b/.claude/hooks/current_task_TASK-PREP.json
deleted file mode 100644
index 1b12c30..0000000
--- a/.claude/hooks/current_task_TASK-PREP.json
+++ /dev/null
@@ -1,24 +0,0 @@
-{
-  "task_id": "TASK-PREP",
-  "task_description": "Add tempfile dev-dependency for unit and integration tests",
-  "plan_path": "/Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml",
-  "plan_slug": "phase-0.2",
-  "acceptance_commands": [
-    "cargo check -p rust-workspace-map"
-  ],
-  "acceptance_prose": [],
-  "all_task_ids": [
-    "TASK-PREP",
-    "TASK-1",
-    "TASK-2",
-    "TASK-3",
-    "TASK-4",
-    "TASK-5",
-    "TASK-6",
-    "TASK-7",
-    "TASK-8",
-    "TASK-9",
-    "TASK-10"
-  ],
-  "timestamp": "2026-04-28T08:54:28Z"
-}
diff --git a/CLAUDE.md b/CLAUDE.md
new file mode 100644
index 0000000..e9249f7
--- /dev/null
+++ b/CLAUDE.md
@@ -0,0 +1,2 @@
+- Refer to [architecture note](./notes/architecture-current.md) whenever you
+  want to explore the codebase for understanding.
diff --git a/Cargo.lock b/Cargo.lock
index 15d9c74..fdd4bff 100644
--- a/Cargo.lock
+++ b/Cargo.lock
@@ -422,6 +422,7 @@ dependencies = [
  "tempfile",
  "thiserror",
  "toml",
+ "walkdir",
 ]
 
 [[package]]
@@ -443,6 +444,15 @@ version = "1.0.22"
 source = "registry+https://github.com/rust-lang/crates.io-index"
 checksum = "b39cdef0fa800fc44525c84ccb54a029961a8215f9619753635a9c0d2538d46d"
 
+[[package]]
+name = "same-file"
+version = "1.0.6"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "93fc1dc3aaa9bfed95e02e6eadabb4baf7e3078b0bd1b4d7b6b0b68378900502"
+dependencies = [
+ "winapi-util",
+]
+
 [[package]]
 name = "semver"
 version = "1.0.28"
@@ -610,6 +620,16 @@ version = "0.2.2"
 source = "registry+https://github.com/rust-lang/crates.io-index"
 checksum = "06abde3611657adf66d383f00b093d7faecc7fa57071cce2578660c9f1010821"
 
+[[package]]
+name = "walkdir"
+version = "2.5.0"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "29790946404f91d9c5d06f9874efddea1dc06c5efe94541a7d6863108e3a5e4b"
+dependencies = [
+ "same-file",
+ "winapi-util",
+]
+
 [[package]]
 name = "wasip2"
 version = "1.0.3+wasi-0.2.9"
@@ -662,6 +682,15 @@ dependencies = [
  "semver",
 ]
 
+[[package]]
+name = "winapi-util"
+version = "0.1.11"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "c2a7b1c03c876122aa43f3020e6c3c3ee5c05081c9a00739faf7503aeba10d22"
+dependencies = [
+ "windows-sys",
+]
+
 [[package]]
 name = "windows-link"
 version = "0.2.1"
diff --git a/Cargo.toml b/Cargo.toml
index b89c9b7..4ed8115 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -15,6 +15,7 @@ bon = "3"
 thiserror = "2"
 glob = "0.3"
 proc-macro2 = { version = "1", features = ["span-locations"] }
+walkdir = "2.5.0"
 
 [dev-dependencies]
 tempfile = "3"
diff --git a/notes/architecture-current.md b/notes/architecture-current.md
new file mode 100644
index 0000000..1e8b467
--- /dev/null
+++ b/notes/architecture-current.md
@@ -0,0 +1,223 @@
+# Architecture Note — Current Codebase Status
+> Generated: 2026-04-29 | Phase: 0.2 (Hardening for Trustworthiness)
+
+---
+
+## Overview
+
+`rust-workspace-map` is a CLI tool that walks a Rust workspace, parses each member crate's public API surface using `syn`, and emits a single structured JSON document designed for LLM consumption. The core design principle is **partial results over abort**: a single crate or module parse failure produces an `ErrorEntry` in the output rather than crashing the whole run.
+
+---
+
+## Module Map
+
+```
+src/
+├── main.rs         CLI entry point (clap)
+├── lib.rs          Pipeline orchestrator (run())
+├── schema.rs       All data types + Error enum
+├── workspace.rs    Workspace/member discovery
+├── cargo_info.rs   Cargo.toml parsing
+├── file_parser.rs  Rust AST extraction (syn)
+├── module_tree.rs  Recursive module tree construction
+├── cross_refs.rs   Cross-crate type reference analysis
+└── render.rs       JSON serialization
+```
+
+---
+
+## Data Flow
+
+```
+CLI args (PATH, -o FILE)
+    │
+    ▼
+workspace::find_workspace_root()   ← walks ancestors for [workspace] Cargo.toml
+    │
+    ▼
+workspace::enumerate_members()     ← parses members[], exclude[], expands globs
+    │
+    ▼ (rayon par_iter per crate)
+┌─────────────────────────────────────────────────────┐
+│  cargo_info::parse_cargo_toml()  →  (PackageInfo, DepInfo)         │
+│  workspace::resolve_crate_roots() →  [(path, CrateType)]           │
+│  module_tree::build_module_tree() →  (Vec<ModuleInfo>, Vec<ErrorEntry>) │
+│  path relativization (strip workspace root prefix)                 │
+└─────────────────────────────────────────────────────┘
+    │
+    ▼
+sort crates by name (determinism)
+    │
+    ▼
+cross_refs::compute()              ← mutates CrateInfo.cross_crate_imports,
+    │                                 returns CrossReferences
+    ▼
+render::render_to_writer()         ← serde_json pretty to stdout or -o file
+```
+
+---
+
+## Module Responsibilities
+
+### `schema.rs` — All types, one place
+
+Defines every serializable and internal type. All output structs use `#[serde(rename_all = "camelCase")]` and `bon::Builder`. `skip_serializing_if = "Vec::is_empty"` suppresses empty arrays throughout.
+
+**Output types:** `WorkspaceMap` → `WorkspaceInfo`, `CrateInfo`, `CrossReferences`, `Vec<ErrorEntry>`  
+**Per-crate:** `PackageInfo`, `DepInfo` (normal/dev/workspace_members), `ModuleInfo`, `PublicItem`, `ImplInfo`  
+**Error types:** `ErrorEntry` (file, line, message, severity, kind, context, cause) + `ErrorSeverity` + `ErrorContext`  
+**Internal-only:** `FileInfo`, `SubmoduleDecl` (not serialized)  
+**Library errors:** `schema::Error` (thiserror) — 7 variants covering file I/O, TOML parse, syn parse, workspace structure
+
+The `ErrorEntry.kind` field uses 8 defined string values:
+`toml_parse_error`, `syn_parse_error`, `missing_crate_roots`, `orphaned_module`, `module_tree_error`, `missing_workspace_section`, `glob_pattern_error`, `member_not_found`
+
+### `workspace.rs` — Discovery
+
+- `find_workspace_root(path)` — walks `path.ancestors()`, returns first dir whose `Cargo.toml` contains `[workspace]`
+- `enumerate_members(root)` — parses workspace TOML, resolves member paths, expands globs via the `glob` crate, applies exclude list, deduplicates and sorts result
+- `resolve_crate_roots(crate_dir)` — checks for `src/lib.rs` (→ Lib) and/or `src/main.rs` (→ Bin); returns both if both exist
+
+**Known issue:** missing-member warning uses `eprintln!` rather than returning an `ErrorEntry` (Path Safety goal, phase 0.2).  
+**Known issue:** exclude filter calls `path.file_name().unwrap_or_default()` — matches only by directory basename, not full path segment; can silently misfire on deeply-nested members.
+
+### `cargo_info.rs` — Cargo.toml parsing
+
+`parse_cargo_toml(path)` returns `(PackageInfo, DepInfo)`. Missing `[package]` fields fall back to `"unknown"` / `"0.0.0"` / `"2021"`. Workspace deps (`{ workspace = true }`) are separated from normal deps. `crate_type` in the returned `PackageInfo` is always `Lib`; the caller overrides it after calling `resolve_crate_roots`.
+
+### `file_parser.rs` — AST extraction (largest module, ~800 lines)
+
+**Always infallible.** `parse_file(path)` returns `ParsedFile` even on IO or `syn` failure — errors are stored in `ParsedFile.parse_error: Option<SynParseError>`, not propagated. Callers use `build_parse_error_entry()` to convert to `ErrorEntry`.
+
+Extraction functions (all `#[must_use]`, all return sorted results for determinism):
+- `extract_public_items` — filters `syn::Visibility::Inherited`, maps to `PublicItem`; sorted by name then line
+- `extract_imports` — flattens braced use-trees recursively; sorted by path
+- `extract_re_exports` — `pub use` statements only; sorted by export_path
+- `extract_submodules` — `mod` declarations; marks `#[cfg(test)]` via token matching; sorted by name
+- `extract_impls` — `impl` blocks → `ImplInfo` with fn/type/const items
+
+Internal helpers cover: visibility stringification, generic parameter rendering, full `syn::Type` to string (paths, references, tuples, slices, arrays, pointers, closures, trait objects, impl Trait), field/variant extraction, attribute extraction (derive + doc).
+
+### `module_tree.rs` — Recursive traversal
+
+`build_module_tree(crate_root, crate_name)` is the public entry. It:
+1. Parses `crate_root` via `file_parser::parse_file`
+2. Builds the root `ModuleInfo`
+3. For each non-test submodule, calls `process_submodule` recursively
+4. Tracks visited paths in a `HashSet<PathBuf>` to prevent cycles
+
+`process_submodule` handles two cases:
+- **Inline modules** (`mod foo { ... }`) — extracts items from AST directly, no file I/O
+- **External modules** (`mod foo;`) — resolves via `resolve_module_path` (tries `foo.rs` then `foo/mod.rs`), recurses
+
+Unresolvable external modules produce an `orphaned_module` ErrorEntry and a placeholder `ModuleInfo` with `file = "<unresolved>"`.
+
+**Known issue:** `process_module_info` returns `errors.clone()` — redundant allocation that doubles the error vec for deeply nested trees.  
+**Known issue:** `parent_dir` computation uses `.unwrap_or(crate_root)` — semantically wrong if `crate_root` has no parent (root path).
+
+### `cross_refs.rs` — Cross-crate analysis
+
+`compute(crates: &mut [CrateInfo])` takes mutable access to all crates at once. In a single pass:
+1. Builds `crate_exports: BTreeMap<String, Vec<(name, kind)>>` from all public items
+2. Initializes `TypeRef` entries, populating `exported_by`
+3. Scans every import in every module; if the first path segment matches a known crate name (other than self), records a `CrossCrateImport` and updates `TypeRef.imported_by`
+
+Limitation: cross-crate detection is heuristic — it matches the first import segment against crate names. An import like `use serde::Serialize` would be missed unless `serde` itself is a workspace member. Re-exports are not traced across crates.
+
+### `render.rs` — Output
+
+Thin wrappers over `serde_json::to_string_pretty` and `serde_json::to_writer_pretty`. No custom serialization logic.
+
+### `lib.rs` — Pipeline glue
+
+`run()` is the single public function. It orchestrates the full pipeline, collects per-crate `ErrorEntry` vecs, and passes everything through to `WorkspaceMap`. Errors from `find_workspace_root` and `enumerate_members` are fatal (propagated as `anyhow::Error`). Errors from per-crate processing are soft (collected into `WorkspaceMap.errors`).
+
+---
+
+## Error Handling Architecture
+
+| Layer | Mechanism | Fate |
+|---|---|---|
+| Workspace root not found | `schema::Error::WorkspaceRootNotFound` → `anyhow` | Fatal — exits non-zero |
+| Missing `[workspace]` section | `schema::Error::MissingWorkspaceSection` → `anyhow` | Fatal — exits non-zero |
+| TOML parse failure (workspace) | `schema::Error::TomlParse` → `anyhow` | Fatal |
+| TOML parse failure (crate) | `ErrorEntry { kind: "toml_parse_error" }` | Soft — crate skipped, run continues |
+| No crate roots found | `ErrorEntry { kind: "missing_crate_roots" }` | Soft — crate skipped |
+| `syn` parse failure | `ErrorEntry { kind: "syn_parse_error" }` | Soft — module empty, siblings continue |
+| Orphaned module | `ErrorEntry { kind: "orphaned_module" }` | Soft — placeholder entry |
+| Missing workspace member | `eprintln!` (not ErrorEntry) | **Gap** — not in JSON output |
+
+---
+
+## Test Coverage
+
+### Unit tests (inline `#[cfg(test)]` modules)
+
+| Module | Tests |
+|---|---|
+| `file_parser.rs` | 13 |
+| `workspace.rs` | 5 |
+| `render.rs` | 3 |
+| `cargo_info.rs` | 3 |
+| `module_tree.rs` | 4 |
+| `cross_refs.rs` | 2 |
+| **Total** | **30** |
+
+### Integration tests (`tests/integration_test.rs`, 10 tests)
+
+All integration tests run the compiled binary via `std::process::Command`:
+
+1. `test_sample_workspace_output` — full fixture smoke test (cross-refs, modules, imports)
+2. `test_deterministic_output` — byte-identical across two sequential runs
+3. `test_missing_path_exits_nonzero` — invalid path → non-zero exit
+4. `test_parse_failure_error_entry` — bad Rust syntax → `syn_parse_error` in JSON errors
+5. `test_missing_workspace_section` — no `[workspace]` → non-zero exit
+6. `test_glob_member_patterns` — `crates/*` glob expansion
+7. `test_workspace_with_exclude` — exclude list respected
+8. `test_deeply_nested_modules` — 4-level deep module tree (`nested::foo::bar::baz`)
+9. `test_reexport_chains` — `pub use inner::Secret` captured in `reExports`
+10. `test_output_via_flag` — `-o file` content matches stdout
+
+### Test fixture
+
+`tests/fixtures/sample-workspace/` — two-crate workspace (`core` + `engine`). `engine` imports from `core`. Pre-compiled (debug artifacts present in fixture tree).
+
+---
+
+## Dependencies
+
+| Crate | Role |
+|---|---|
+| `serde` + `serde_json` | Serialization to JSON |
+| `syn` (full, extra-traits) | Rust source parsing |
+| `proc-macro2` (span-locations) | Source line numbers from `syn` spans |
+| `toml` | Cargo.toml parsing |
+| `rayon` | Parallel crate processing |
+| `clap` (derive) | CLI argument parsing |
+| `bon` | Builder pattern macros for all output structs |
+| `thiserror` | Typed `schema::Error` enum |
+| `anyhow` | Error chaining in `run()` / `main()` |
+| `glob` | Workspace member glob expansion |
+| `tempfile` (dev) | Temp directories in unit/integration tests |
+
+---
+
+## Phase 0.2 Goals — Status Summary
+
+| Goal | Description | Status |
+|---|---|---|
+| G1 Error Visibility Pipeline | Wire `ErrorEntry` end-to-end | Partial — most paths covered, missing-member gap remains |
+| G2 Unit Test Suite (≥80% line coverage) | 5 core modules | 30 unit tests exist; coverage not yet measured |
+| G3 Path Safety Fixes | 4 `unwrap_or_default` path bugs | Not yet fixed |
+| G4 Integration Test Expansion | 3→8+ tests | Done — 10 tests exist |
+| G5 Remove Clippy Suppressions | 9 crate-level allows | Partially — `#[warn(clippy::pedantic)]` at crate level; 2× `too_many_lines` + 1× `too_many_arguments` remain inline |
+
+---
+
+## Structural Invariants
+
+- All collections in output are sorted for deterministic JSON (crates by name, public items by name+line, imports by path, re-exports by export_path, submodules by name).
+- `WorkspaceMap.errors` is omitted from JSON when empty (`skip_serializing_if`). Same for all `Vec` fields on output structs.
+- All paths in output are workspace-relative strings (prefix stripped by `relativize_path`).
+- `WorkspaceMap.workspace_root` is `#[serde(skip)]` — internal use only.
+- `FileInfo` and `SubmoduleDecl` are `pub` in `schema.rs` but not serialized — internal intermediate types.
diff --git a/plans/compiled/TASK-1.py b/plans/compiled/TASK-1.py
deleted file mode 100644
index 2883ca5..0000000
--- a/plans/compiled/TASK-1.py
+++ /dev/null
@@ -1,44 +0,0 @@
-#!/usr/bin/env python3
-"""TASK-1: Add ErrorSeverity enum and ErrorContext struct to schema.rs"""
-import base64, json, subprocess, sys
-from pathlib import Path
-
-TASK_ID = "TASK-1"
-STEPS = json.loads('[{"before_b64": "Ly8g4pSA4pSAIEludGVybmFsIHR5cGVzIOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgAoKLy8vIEludGVybmFsIGludGVybWVkaWF0ZSB0eXBlIGNvbnN1bWVkIGJ5IG1vZHVsZV90cmVlLgojW2Rlcml2ZShEZWJ1ZywgQ2xvbmUsIERlZmF1bHQpXQpwdWIgc3RydWN0IEZpbGVJbmZvIHsK", "after_b64": "Ly8g4pSA4pSAIEVycm9yIHNldmVyaXR5IOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgAoKI1tkZXJpdmUoRGVidWcsIENsb25lLCBDb3B5LCBQYXJ0aWFsRXEsIEVxLCBzZXJkZTo6U2VyaWFsaXplKV0KI1tzZXJkZShyZW5hbWVfYWxsID0gInNuYWtlX2Nhc2UiKV0KcHViIGVudW0gRXJyb3JTZXZlcml0eSB7CiAgICBFcnJvciwKICAgIFdhcm5pbmcsCn0KCi8vIOKUgOKUgCBFcnJvciBjb250ZXh0IOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgAoKLy8vIE9wdGlvbmFsIGNvbnRleHQgYXR0YWNoZWQgdG8gYW4gZXJyb3IsIHByb3ZpZGluZyBhZGRpdGlvbmFsIGxvY2F0aW9uCi8vLyBhbmQgc291cmNlIGluZm9ybWF0aW9uIGZvciBkaWFnbm9zdGljcy4KI1tkZXJpdmUoRGVidWcsIENsb25lLCBEZWZhdWx0LCBzZXJkZTo6U2VyaWFsaXplLCBib246OkJ1aWxkZXIpXQojW3NlcmRlKHJlbmFtZV9hbGwgPSAiY2FtZWxDYXNlIildCnB1YiBzdHJ1Y3QgRXJyb3JDb250ZXh0IHsKICAgICNbc2VyZGUoc2tpcF9zZXJpYWxpemluZ19pZiA9ICJPcHRpb246OmlzX25vbmUiKV0KICAgIHB1YiBjcmF0ZV9uYW1lOiBPcHRpb248U3RyaW5nPiwKCiAgICAjW3NlcmRlKHNraXBfc2VyaWFsaXppbmdfaWYgPSAiT3B0aW9uOjppc19ub25lIildCiAgICBwdWIgbW9kdWxlX3BhdGg6IE9wdGlvbjxTdHJpbmc+LAoKICAgIC8vLyBMaW5lIG51bWJlciBpbiB0aGUgc291cmNlIGZpbGUgd2hlcmUgdGhlIGVycm9yIG9jY3VycmVkLgogICAgI1tzZXJkZShza2lwX3NlcmlhbGl6aW5nX2lmID0gIk9wdGlvbjo6aXNfbm9uZSIpXQogICAgcHViIGxpbmU6IE9wdGlvbjx1c2l6ZT4sCgogICAgLy8vIEEgc2hvcnQgc291cmNlIHNuaXBwZXQgbmVhciB0aGUgZXJyb3IgbG9jYXRpb24gKGlmIGF2YWlsYWJsZSkuCiAgICAjW3NlcmRlKHNraXBfc2VyaWFsaXppbmdfaWYgPSAiT3B0aW9uOjppc19ub25lIildCiAgICBwdWIgc25pcHBldDogT3B0aW9uPFN0cmluZz4sCn0KCi8vIOKUgOKUgCBJbnRlcm5hbCB0eXBlcyDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIAKCi8vLyBJbnRlcm5hbCBpbnRlcm1lZGlhdGUgdHlwZSBjb25zdW1lZCBieSBtb2R1bGVfdHJlZS4KI1tkZXJpdmUoRGVidWcsIENsb25lLCBEZWZhdWx0KV0KcHViIHN0cnVjdCBGaWxlSW5mbyB7Cg==", "target": "src/schema.rs", "index": 0, "is_create": false}]')
-
-for step in STEPS:
-    before = base64.b64decode(step["before_b64"]).decode()
-    after = base64.b64decode(step["after_b64"]).decode()
-    target = step["target"]
-    idx = step["index"]
-    is_create = step["is_create"]
-
-    if is_create:
-        target_path = Path(target)
-        target_path.parent.mkdir(parents=True, exist_ok=True)
-        target_path.write_text(after)
-        print(f"OK {TASK_ID} change {idx}: created {target}")
-    else:
-        target_path = Path(target)
-        content = target_path.read_text()
-        if before not in content:
-            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
-            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
-            sys.exit(1)
-
-        result = subprocess.run(
-            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
-            capture_output=True, text=True,
-        )
-        if result.returncode != 0:
-            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
-            sys.exit(result.returncode)
-
-        new_content = target_path.read_text()
-        if after and after not in new_content:
-            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
-            sys.exit(1)
-
-        print(f"OK {TASK_ID} change {idx}: applied to {target}")
-
-print(f"OK {TASK_ID}: all changes applied")
diff --git a/plans/compiled/TASK-1.sh b/plans/compiled/TASK-1.sh
deleted file mode 100755
index 86e78d3..0000000
--- a/plans/compiled/TASK-1.sh
+++ /dev/null
@@ -1,7 +0,0 @@
-#!/usr/bin/env bash
-set -euo pipefail
-# TASK-1: Add ErrorSeverity enum and ErrorContext struct to schema.rs
-# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
-# Type: replace
-# File: src/schema.rs
-python3 "$(dirname "$0")/TASK-1.py"
diff --git a/plans/compiled/TASK-10.py b/plans/compiled/TASK-10.py
deleted file mode 100644
index f9753bc..0000000
--- a/plans/compiled/TASK-10.py
+++ /dev/null
@@ -1,44 +0,0 @@
-#!/usr/bin/env python3
-"""TASK-10: Expand integration tests from 3 to 10 covering error cases, workspace patterns, and output"""
-import base64, json, subprocess, sys
-from pathlib import Path
-
-TASK_ID = "TASK-10"
-STEPS = json.loads('[{"before_b64": "I1t0ZXN0XQpmbiB0ZXN0X21pc3NpbmdfcGF0aF9leGl0c19ub256ZXJvKCkgewogICAgbGV0IGJpbiA9IGJpbmFyeV9wYXRoKCk7CiAgICBsZXQgb3V0cHV0ID0gQ29tbWFuZDo6bmV3KCZiaW4pCiAgICAgICAgLmFyZygiL3RtcC9ub25leGlzdGVudC1wYXRoLTEyMzQ1IikKICAgICAgICAub3V0cHV0KCkKICAgICAgICAuZXhwZWN0KCJmYWlsZWQgdG8gZXhlY3V0ZSBiaW5hcnkiKTsKCiAgICBhc3NlcnQhKAogICAgICAgICFvdXRwdXQuc3RhdHVzLnN1Y2Nlc3MoKSwKICAgICAgICAic2hvdWxkIGV4aXQgbm9uLXplcm8gZm9yIGludmFsaWQgcGF0aCIKICAgICk7Cn0K", "after_b64": "I1t0ZXN0XQpmbiB0ZXN0X21pc3NpbmdfcGF0aF9leGl0c19ub256ZXJvKCkgewogICAgbGV0IGJpbiA9IGJpbmFyeV9wYXRoKCk7CiAgICBsZXQgb3V0cHV0ID0gQ29tbWFuZDo6bmV3KCZiaW4pCiAgICAgICAgLmFyZygiL3RtcC9ub25leGlzdGVudC1wYXRoLTEyMzQ1IikKICAgICAgICAub3V0cHV0KCkKICAgICAgICAuZXhwZWN0KCJmYWlsZWQgdG8gZXhlY3V0ZSBiaW5hcnkiKTsKCiAgICBhc3NlcnQhKAogICAgICAgICFvdXRwdXQuc3RhdHVzLnN1Y2Nlc3MoKSwKICAgICAgICAic2hvdWxkIGV4aXQgbm9uLXplcm8gZm9yIGludmFsaWQgcGF0aCIKICAgICk7Cn0KCmZuIHJ1bl9iaW5hcnkocGF0aDogJnN0cikgLT4gc3RkOjpwcm9jZXNzOjpPdXRwdXQgewogICAgQ29tbWFuZDo6bmV3KCZiaW5hcnlfcGF0aCgpKQogICAgICAgIC5hcmcocGF0aCkKICAgICAgICAub3V0cHV0KCkKICAgICAgICAuZXhwZWN0KCJmYWlsZWQgdG8gZXhlY3V0ZSBiaW5hcnkiKQp9CgpmbiBwYXJzZV9vdXRwdXQob3V0cHV0OiAmc3RkOjpwcm9jZXNzOjpPdXRwdXQpIC0+IHNlcmRlX2pzb246OlZhbHVlIHsKICAgIHNlcmRlX2pzb246OmZyb21fc3RyKCZTdHJpbmc6OmZyb21fdXRmOF9sb3NzeSgmb3V0cHV0LnN0ZG91dCkpLnVud3JhcCgpCn0KCmZuIHdyaXRlX2NhcmdvX3RvbWwoZGlyOiAmc3RkOjpwYXRoOjpQYXRoLCBjb250ZW50OiAmc3RyKSB7CiAgICBsZXQgbXV0IGYgPSBzdGQ6OmZzOjpGaWxlOjpjcmVhdGUoZGlyLmpvaW4oIkNhcmdvLnRvbWwiKSkudW53cmFwKCk7CiAgICB1c2Ugc3RkOjppbzo6V3JpdGU7CiAgICBmLndyaXRlX2FsbChjb250ZW50LmFzX2J5dGVzKCkpLnVud3JhcCgpOwp9CgpmbiBzZXR1cF9jcmF0ZShkaXI6ICZzdGQ6OnBhdGg6OlBhdGgsIGxpYl9jb250ZW50OiAmc3RyKSB7CiAgICBsZXQgc3JjID0gZGlyLmpvaW4oInNyYyIpOwogICAgc3RkOjpmczo6Y3JlYXRlX2Rpcl9hbGwoJnNyYykudW53cmFwKCk7CiAgICBzdGQ6OmZzOjp3cml0ZShzcmMuam9pbigibGliLnJzIiksIGxpYl9jb250ZW50KS51bndyYXAoKTsKfQoKI1t0ZXN0XQpmbiB0ZXN0X3BhcnNlX2ZhaWx1cmVfZXJyb3JfZW50cnkoKSB7CiAgICBsZXQgdG1wID0gdGVtcGZpbGU6OnRlbXBkaXIoKS51bndyYXAoKTsKICAgIGxldCByb290ID0gdG1wLnBhdGgoKTsKCiAgICAvLyBDcmVhdGUgd29ya3NwYWNlIENhcmdvLnRvbWwKICAgIHdyaXRlX2NhcmdvX3RvbWwocm9vdCwgciMiClt3b3Jrc3BhY2VdCm1lbWJlcnMgPSBbImdvb2RfY3JhdGUiLCAiYmFkX2NyYXRlIl0KIiMpOwoKICAgIC8vIEdvb2QgY3JhdGUgd2l0aCB2YWxpZCBSdXN0CiAgICBzZXR1cF9jcmF0ZSgmcm9vdC5qb2luKCJnb29kX2NyYXRlIiksICJwdWIgc3RydWN0IEdvb2Qge30iKTsKCiAgICAvLyBCYWQgY3JhdGUgd2l0aCBpbnZhbGlkIFJ1c3Qgc3ludGF4CiAgICBzZXR1cF9jcmF0ZSgmcm9vdC5qb2luKCJiYWRfY3JhdGUiKSwgInB1YiBzdHJ1Y3QgeyBpbnZhbGlkIHJ1c3Qgc3ludGF4Iik7CgogICAgbGV0IG91dHB1dCA9IHJ1bl9iaW5hcnkocm9vdC50b19zdHIoKS51bndyYXAoKSk7CiAgICBhc3NlcnQhKG91dHB1dC5zdGF0dXMuc3VjY2VzcygpKTsKCiAgICBsZXQganNvbiA9IHBhcnNlX291dHB1dCgmb3V0cHV0KTsKICAgIGxldCBlcnJvcnM6IFZlYzwmc2VyZGVfanNvbjo6VmFsdWU+ID0gZXh0cmFjdF9hcnJheSgmanNvbiwgImVycm9ycyIpOwoKICAgIGxldCBwYXJzZV9lcnJvcnM6IFZlYzxfPiA9IGVycm9ycy5pdGVyKCkKICAgICAgICAuZmlsdGVyKHxlfCB7CiAgICAgICAgICAgIGVbImtpbmQiXS5hc19zdHIoKS51bndyYXAoKSA9PSAic3luX3BhcnNlX2Vycm9yIgogICAgICAgIH0pCiAgICAgICAgLmNvbGxlY3QoKTsKCiAgICBhc3NlcnQhKCFwYXJzZV9lcnJvcnMuaXNfZW1wdHkoKSwgInNob3VsZCBoYXZlIHBhcnNlIGVycm9yIGVudHJpZXMiKTsKICAgIGFzc2VydF9lcSEocGFyc2VfZXJyb3JzWzBdWyJzZXZlcml0eSJdLmFzX3N0cigpLnVud3JhcCgpLCAiZXJyb3IiKTsKfQoKI1t0ZXN0XQpmbiB0ZXN0X21pc3Npbmdfd29ya3NwYWNlX3NlY3Rpb24oKSB7CiAgICBsZXQgdG1wID0gdGVtcGZpbGU6OnRlbXBkaXIoKS51bndyYXAoKTsKICAgIGxldCByb290ID0gdG1wLnBhdGgoKTsKCiAgICAvLyBDYXJnby50b21sIHdpdGhvdXQgW3dvcmtzcGFjZV0gc2VjdGlvbgogICAgd3JpdGVfY2FyZ29fdG9tbChyb290LCByIyIKW3BhY2thZ2VdCm5hbWUgPSAic3RhbmRhbG9uZSIKdmVyc2lvbiA9ICIwLjEuMCIKZWRpdGlvbiA9ICIyMDIxIgoiIyk7CgogICAgbGV0IG91dHB1dCA9IHJ1bl9iaW5hcnkocm9vdC50b19zdHIoKS51bndyYXAoKSk7CgogICAgLy8gU2hvdWxkIGV4aXQgbm9uLXplcm8gYmVjYXVzZSB3b3Jrc3BhY2UgaXMgbWlzc2luZwogICAgYXNzZXJ0ISgKICAgICAgICAhb3V0cHV0LnN0YXR1cy5zdWNjZXNzKCksCiAgICAgICAgInNob3VsZCBleGl0IG5vbi16ZXJvIGZvciBtaXNzaW5nIHdvcmtzcGFjZSBzZWN0aW9uIgogICAgKTsKfQoKI1t0ZXN0XQpmbiB0ZXN0X2dsb2JfbWVtYmVyX3BhdHRlcm5zKCkgewogICAgbGV0IHRtcCA9IHRlbXBmaWxlOjp0ZW1wZGlyKCkudW53cmFwKCk7CiAgICBsZXQgcm9vdCA9IHRtcC5wYXRoKCk7CgogICAgd3JpdGVfY2FyZ29fdG9tbChyb290LCByIyIKW3dvcmtzcGFjZV0KbWVtYmVycyA9IFsiY3JhdGVzLyoiXQoiIyk7CgogICAgZm9yIG5hbWUgaW4gJlsiYWxwaGEiLCAiYmV0YSIsICJnYW1tYSJdIHsKICAgICAgICBzZXR1cF9jcmF0ZSgmcm9vdC5qb2luKCJjcmF0ZXMiKS5qb2luKG5hbWUpLCBmb3JtYXQhKCJwdWIgc3RydWN0IHtuYW1lfSB7e319IikuYXNfc3RyKCkpOwogICAgfQoKICAgIGxldCBvdXRwdXQgPSBydW5fYmluYXJ5KHJvb3QudG9fc3RyKCkudW53cmFwKCkpOwogICAgYXNzZXJ0IShvdXRwdXQuc3RhdHVzLnN1Y2Nlc3MoKSk7CgogICAgbGV0IGpzb24gPSBwYXJzZV9vdXRwdXQoJm91dHB1dCk7CiAgICBsZXQgY3JhdGVzID0gZXh0cmFjdF9hcnJheSgmanNvbiwgImNyYXRlcyIpOwogICAgbGV0IG5hbWVzOiBWZWM8JnN0cj4gPSBjcmF0ZXMuaXRlcigpCiAgICAgICAgLm1hcCh8Y3wgY1sibmFtZSJdLmFzX3N0cigpLnVud3JhcCgpKQogICAgICAgIC5jb2xsZWN0KCk7CgogICAgYXNzZXJ0IShuYW1lcy5jb250YWlucygmImFscGhhIikpOwogICAgYXNzZXJ0IShuYW1lcy5jb250YWlucygmImJldGEiKSk7CiAgICBhc3NlcnQhKG5hbWVzLmNvbnRhaW5zKCYiZ2FtbWEiKSk7CiAgICBhc3NlcnRfZXEhKG5hbWVzLmxlbigpLCAzKTsKfQoKI1t0ZXN0XQpmbiB0ZXN0X3dvcmtzcGFjZV93aXRoX2V4Y2x1ZGUoKSB7CiAgICBsZXQgdG1wID0gdGVtcGZpbGU6OnRlbXBkaXIoKS51bndyYXAoKTsKICAgIGxldCByb290ID0gdG1wLnBhdGgoKTsKCiAgICB3cml0ZV9jYXJnb190b21sKHJvb3QsIHIjIgpbd29ya3NwYWNlXQptZW1iZXJzID0gWyJhIiwgImIiLCAiYyJdCmV4Y2x1ZGUgPSBbImIiXQoiIyk7CgogICAgc2V0dXBfY3JhdGUoJnJvb3Quam9pbigiYSIpLCAicHViIHN0cnVjdCBBIHt9Iik7CiAgICBzZXR1cF9jcmF0ZSgmcm9vdC5qb2luKCJiIiksICJwdWIgc3RydWN0IEIge30iKTsKICAgIHNldHVwX2NyYXRlKCZyb290LmpvaW4oImMiKSwgInB1YiBzdHJ1Y3QgQyB7fSIpOwoKICAgIGxldCBvdXRwdXQgPSBydW5fYmluYXJ5KHJvb3QudG9fc3RyKCkudW53cmFwKCkpOwogICAgYXNzZXJ0IShvdXRwdXQuc3RhdHVzLnN1Y2Nlc3MoKSk7CgogICAgbGV0IGpzb24gPSBwYXJzZV9vdXRwdXQoJm91dHB1dCk7CiAgICBsZXQgY3JhdGVzID0gZXh0cmFjdF9hcnJheSgmanNvbiwgImNyYXRlcyIpOwogICAgbGV0IG5hbWVzOiBWZWM8JnN0cj4gPSBjcmF0ZXMuaXRlcigpCiAgICAgICAgLm1hcCh8Y3wgY1sibmFtZSJdLmFzX3N0cigpLnVud3JhcCgpKQogICAgICAgIC5jb2xsZWN0KCk7CgogICAgYXNzZXJ0IShuYW1lcy5jb250YWlucygmImEiKSk7CiAgICBhc3NlcnQhKCFuYW1lcy5jb250YWlucygmImIiKSk7CiAgICBhc3NlcnQhKG5hbWVzLmNvbnRhaW5zKCYiYyIpKTsKfQoKI1t0ZXN0XQpmbiB0ZXN0X2RlZXBseV9uZXN0ZWRfbW9kdWxlcygpIHsKICAgIGxldCB0bXAgPSB0ZW1wZmlsZTo6dGVtcGRpcigpLnVud3JhcCgpOwogICAgbGV0IHJvb3QgPSB0bXAucGF0aCgpOwoKICAgIHdyaXRlX2NhcmdvX3RvbWwocm9vdCwgciMiClt3b3Jrc3BhY2VdCm1lbWJlcnMgPSBbIi4iXQoKW3BhY2thZ2VdCm5hbWUgPSAibmVzdGVkIgp2ZXJzaW9uID0gIjAuMS4wIgplZGl0aW9uID0gIjIwMjEiCiIjKTsKCiAgICBsZXQgc3JjID0gcm9vdC5qb2luKCJzcmMiKTsKICAgIGxldCBmb28gPSBzcmMuam9pbigiZm9vIik7CiAgICBsZXQgYmFyID0gZm9vLmpvaW4oImJhciIpOwogICAgc3RkOjpmczo6Y3JlYXRlX2Rpcl9hbGwoJmJhcikudW53cmFwKCk7CgogICAgLy8gbGliLnJzIGRlY2xhcmVzIG1vZCBmb28KICAgIHN0ZDo6ZnM6OndyaXRlKHNyYy5qb2luKCJsaWIucnMiKSwgIm1vZCBmb287IikudW53cmFwKCk7CiAgICAvLyBmb28ucnMgZGVjbGFyZXMgbW9kIGJhcgogICAgc3RkOjpmczo6d3JpdGUoZm9vLmpvaW4oImZvby5ycyIpLCAibW9kIGJhcjsiKS51bndyYXAoKTsKICAgIC8vIGJhci9iYXoucnMgZGVjbGFyZXMgbW9kIGJhegogICAgc3RkOjpmczo6d3JpdGUoYmFyLmpvaW4oImJhci5ycyIpLCAibW9kIGJhejsiKS51bndyYXAoKTsKICAgIC8vIGJhei5ycyB3aXRoIGEgc3RydWN0CiAgICBzdGQ6OmZzOjp3cml0ZShiYXIuam9pbigiYmF6LnJzIiksICJwdWIgc3RydWN0IERlZXAge30iKS51bndyYXAoKTsKCiAgICBsZXQgb3V0cHV0ID0gcnVuX2JpbmFyeShyb290LnRvX3N0cigpLnVud3JhcCgpKTsKICAgIGFzc2VydCEob3V0cHV0LnN0YXR1cy5zdWNjZXNzKCkpOwoKICAgIGxldCBqc29uID0gcGFyc2Vfb3V0cHV0KCZvdXRwdXQpOwogICAgbGV0IGNyYXRlcyA9IGV4dHJhY3RfYXJyYXkoJmpzb24sICJjcmF0ZXMiKTsKICAgIGxldCBuZXN0ZWRfY3JhdGUgPSBjcmF0ZXMuaXRlcigpLmZpbmQofGN8IGNbIm5hbWUiXS5hc19zdHIoKS51bndyYXAoKSA9PSAibmVzdGVkIikudW53cmFwKCk7CgogICAgbGV0IG1vZHVsZV9wYXRoczogVmVjPCZzdHI+ID0gZXh0cmFjdF9hcnJheSgmbmVzdGVkX2NyYXRlWyJtb2R1bGVzIl0sICJwYXRoIikKICAgICAgICAuaXRlcigpCiAgICAgICAgLm1hcCh8bXwgbS5hc19zdHIoKS51bndyYXAoKSkKICAgICAgICAuY29sbGVjdCgpOwoKICAgIGFzc2VydCEobW9kdWxlX3BhdGhzLml0ZXIoKS5hbnkofHB8ICpwID09ICJuZXN0ZWQiKSk7CiAgICBhc3NlcnQhKG1vZHVsZV9wYXRocy5pdGVyKCkuYW55KHxwfCAqcCA9PSAibmVzdGVkOjpmb28iKSk7CiAgICBhc3NlcnQhKG1vZHVsZV9wYXRocy5pdGVyKCkuYW55KHxwfCAqcCA9PSAibmVzdGVkOjpmb286OmJhciIpKTsKICAgIGFzc2VydCEobW9kdWxlX3BhdGhzLml0ZXIoKS5hbnkofHB8ICpwID09ICJuZXN0ZWQ6OmZvbzo6YmFyOjpiYXoiKSk7Cn0KCiNbdGVzdF0KZm4gdGVzdF9yZWV4cG9ydF9jaGFpbnMoKSB7CiAgICBsZXQgdG1wID0gdGVtcGZpbGU6OnRlbXBkaXIoKS51bndyYXAoKTsKICAgIGxldCByb290ID0gdG1wLnBhdGgoKTsKCiAgICB3cml0ZV9jYXJnb190b21sKHJvb3QsIHIjIgpbd29ya3NwYWNlXQptZW1iZXJzID0gWyIuIl0KCltwYWNrYWdlXQpuYW1lID0gInJlZXhwb3J0ZXIiCnZlcnNpb24gPSAiMC4xLjAiCmVkaXRpb24gPSAiMjAyMSIKIiMpOwoKICAgIGxldCBzcmMgPSByb290LmpvaW4oInNyYyIpOwogICAgc3RkOjpmczo6Y3JlYXRlX2Rpcl9hbGwoJnNyYykudW53cmFwKCk7CgogICAgLy8gbGliLnJzIHdpdGggcmUtZXhwb3J0IGNoYWluCiAgICBzdGQ6OmZzOjp3cml0ZShzcmMuam9pbigibGliLnJzIiksICIKbW9kIGlubmVyIHsKICAgIHB1YiBzdHJ1Y3QgU2VjcmV0Owp9CnB1YiB1c2UgaW5uZXI6OlNlY3JldDsKIikudW53cmFwKCk7CgogICAgbGV0IG91dHB1dCA9IHJ1bl9iaW5hcnkocm9vdC50b19zdHIoKS51bndyYXAoKSk7CiAgICBhc3NlcnQhKG91dHB1dC5zdGF0dXMuc3VjY2VzcygpKTsKCiAgICBsZXQganNvbiA9IHBhcnNlX291dHB1dCgmb3V0cHV0KTsKICAgIGxldCBjcmF0ZXMgPSBleHRyYWN0X2FycmF5KCZqc29uLCAiY3JhdGVzIik7CiAgICBsZXQgcmVleHBvcnRlciA9IGNyYXRlcy5pdGVyKCkuZmluZCh8Y3wgY1sibmFtZSJdLmFzX3N0cigpLnVud3JhcCgpID09ICJyZWV4cG9ydGVyIikudW53cmFwKCk7CgogICAgbGV0IHJlX2V4cG9ydHM6IFZlYzwmc2VyZGVfanNvbjo6VmFsdWU+ID0gZXh0cmFjdF9hcnJheSgmcmVleHBvcnRlclsibW9kdWxlcyJdKQogICAgICAgIC5pdGVyKCkKICAgICAgICAuZmxhdF9tYXAofG18IGV4dHJhY3RfYXJyYXkobSwgInJlRXhwb3J0cyIpKQogICAgICAgIC5jb2xsZWN0KCk7CgogICAgbGV0IGhhc19zZWNyZXQgPSByZV9leHBvcnRzLml0ZXIoKS5hbnkofHJlfCB7CiAgICAgICAgcmVbImltcG9ydFBhdGgiXS5hc19zdHIoKS51bndyYXAoKS5jb250YWlucygiU2VjcmV0IikKICAgIH0pOwogICAgYXNzZXJ0IShoYXNfc2VjcmV0LCAic2hvdWxkIGhhdmUgcmUtZXhwb3J0IGZvciBTZWNyZXQiKTsKfQoKI1t0ZXN0XQpmbiB0ZXN0X291dHB1dF92aWFfZmxhZygpIHsKICAgIGxldCB0bXAgPSB0ZW1wZmlsZTo6dGVtcGRpcigpLnVud3JhcCgpOwogICAgbGV0IGZpeHR1cmUgPSBzdGQ6OnBhdGg6OlBhdGg6Om5ldygidGVzdHMvZml4dHVyZXMvc2FtcGxlLXdvcmtzcGFjZSIpOwogICAgbGV0IG91dHB1dF9wYXRoID0gdG1wLnBhdGgoKS5qb2luKCJvdXRwdXQuanNvbiIpOwoKICAgIC8vIFJ1biB3aXRoIC1vIGZsYWcKICAgIGxldCBvdXRwdXQxID0gQ29tbWFuZDo6bmV3KCZiaW5hcnlfcGF0aCgpKQogICAgICAgIC5hcmcoZml4dHVyZSkKICAgICAgICAuYXJnKCItbyIpCiAgICAgICAgLmFyZyhvdXRwdXRfcGF0aC5jbG9uZSgpKQogICAgICAgIC5vdXRwdXQoKQogICAgICAgIC5leHBlY3QoImZhaWxlZCB0byBleGVjdXRlIGJpbmFyeSIpOwogICAgYXNzZXJ0IShvdXRwdXQxLnN0YXR1cy5zdWNjZXNzKCkpOwoKICAgIC8vIFJ1biB3aXRob3V0IC1vLCBjYXB0dXJlIHN0ZG91dAogICAgbGV0IG91dHB1dDIgPSBDb21tYW5kOjpuZXcoJmJpbmFyeV9wYXRoKCkpCiAgICAgICAgLmFyZyhmaXh0dXJlKQogICAgICAgIC5vdXRwdXQoKQogICAgICAgIC5leHBlY3QoImZhaWxlZCB0byBleGVjdXRlIGJpbmFyeSIpOwogICAgYXNzZXJ0IShvdXRwdXQyLnN0YXR1cy5zdWNjZXNzKCkpOwoKICAgIC8vIENvbXBhcmUgZmlsZSBjb250ZW50IHdpdGggc3Rkb3V0CiAgICBsZXQgZmlsZV9jb250ZW50ID0gc3RkOjpmczo6cmVhZF90b19zdHJpbmcoJm91dHB1dF9wYXRoKS51bndyYXAoKTsKICAgIGxldCBzdGRvdXRfY29udGVudCA9IFN0cmluZzo6ZnJvbV91dGY4X2xvc3N5KCZvdXRwdXQyLnN0ZG91dCk7CiAgICBhc3NlcnRfZXEhKAogICAgICAgIGZpbGVfY29udGVudC50cmltKCksCiAgICAgICAgc3Rkb3V0X2NvbnRlbnQudHJpbSgpLAogICAgICAgICJmaWxlIG91dHB1dCBzaG91bGQgbWF0Y2ggc3Rkb3V0IgogICAgKTsKfQo=", "target": "tests/integration_test.rs", "index": 0, "is_create": false}]')
-
-for step in STEPS:
-    before = base64.b64decode(step["before_b64"]).decode()
-    after = base64.b64decode(step["after_b64"]).decode()
-    target = step["target"]
-    idx = step["index"]
-    is_create = step["is_create"]
-
-    if is_create:
-        target_path = Path(target)
-        target_path.parent.mkdir(parents=True, exist_ok=True)
-        target_path.write_text(after)
-        print(f"OK {TASK_ID} change {idx}: created {target}")
-    else:
-        target_path = Path(target)
-        content = target_path.read_text()
-        if before not in content:
-            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
-            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
-            sys.exit(1)
-
-        result = subprocess.run(
-            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
-            capture_output=True, text=True,
-        )
-        if result.returncode != 0:
-            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
-            sys.exit(result.returncode)
-
-        new_content = target_path.read_text()
-        if after and after not in new_content:
-            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
-            sys.exit(1)
-
-        print(f"OK {TASK_ID} change {idx}: applied to {target}")
-
-print(f"OK {TASK_ID}: all changes applied")
diff --git a/plans/compiled/TASK-10.sh b/plans/compiled/TASK-10.sh
deleted file mode 100755
index bccaa8a..0000000
--- a/plans/compiled/TASK-10.sh
+++ /dev/null
@@ -1,7 +0,0 @@
-#!/usr/bin/env bash
-set -euo pipefail
-# TASK-10: Expand integration tests from 3 to 10 covering error cases, workspace patterns, and output
-# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
-# Type: replace
-# File: tests/integration_test.rs
-python3 "$(dirname "$0")/TASK-10.py"
diff --git a/plans/compiled/TASK-2.py b/plans/compiled/TASK-2.py
deleted file mode 100644
index fbade65..0000000
--- a/plans/compiled/TASK-2.py
+++ /dev/null
@@ -1,44 +0,0 @@
-#!/usr/bin/env python3
-"""TASK-2: Add MissingWorkspaceSection variant to Error enum in schema.rs"""
-import base64, json, subprocess, sys
-from pathlib import Path
-
-TASK_ID = "TASK-2"
-STEPS = json.loads('[{"before_b64": "ICAgICNbZXJyb3IoImdsb2IgcGF0dGVybiBlcnJvcjogezB9IildCiAgICBHbG9iUGF0dGVybihTdHJpbmcpLAp9CgpwdWIgdHlwZSBSZXN1bHQ8VD4gPSBzdGQ6OnJlc3VsdDo6UmVzdWx0PFQsIEVycm9yPjs=", "after_b64": "ICAgICNbZXJyb3IoImdsb2IgcGF0dGVybiBlcnJvcjogezB9IildCiAgICBHbG9iUGF0dGVybihTdHJpbmcpLAoKICAgICNbZXJyb3IoIndvcmtzcGFjZSBDYXJnby50b21sIGlzIG1pc3NpbmcgdGhlIFt3b3Jrc3BhY2VdIHNlY3Rpb24iKV0KICAgIE1pc3NpbmdXb3Jrc3BhY2VTZWN0aW9uLAp9CgpwdWIgdHlwZSBSZXN1bHQ8VD4gPSBzdGQ6OnJlc3VsdDo6UmVzdWx0PFQsIEVycm9yPjs=", "target": "src/schema.rs", "index": 0, "is_create": false}]')
-
-for step in STEPS:
-    before = base64.b64decode(step["before_b64"]).decode()
-    after = base64.b64decode(step["after_b64"]).decode()
-    target = step["target"]
-    idx = step["index"]
-    is_create = step["is_create"]
-
-    if is_create:
-        target_path = Path(target)
-        target_path.parent.mkdir(parents=True, exist_ok=True)
-        target_path.write_text(after)
-        print(f"OK {TASK_ID} change {idx}: created {target}")
-    else:
-        target_path = Path(target)
-        content = target_path.read_text()
-        if before not in content:
-            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
-            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
-            sys.exit(1)
-
-        result = subprocess.run(
-            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
-            capture_output=True, text=True,
-        )
-        if result.returncode != 0:
-            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
-            sys.exit(result.returncode)
-
-        new_content = target_path.read_text()
-        if after and after not in new_content:
-            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
-            sys.exit(1)
-
-        print(f"OK {TASK_ID} change {idx}: applied to {target}")
-
-print(f"OK {TASK_ID}: all changes applied")
diff --git a/plans/compiled/TASK-2.sh b/plans/compiled/TASK-2.sh
deleted file mode 100755
index 2cd1783..0000000
--- a/plans/compiled/TASK-2.sh
+++ /dev/null
@@ -1,7 +0,0 @@
-#!/usr/bin/env bash
-set -euo pipefail
-# TASK-2: Add MissingWorkspaceSection variant to Error enum in schema.rs
-# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
-# Type: replace
-# File: src/schema.rs
-python3 "$(dirname "$0")/TASK-2.py"
diff --git a/plans/compiled/TASK-3.py b/plans/compiled/TASK-3.py
deleted file mode 100644
index cc00199..0000000
--- a/plans/compiled/TASK-3.py
+++ /dev/null
@@ -1,44 +0,0 @@
-#!/usr/bin/env python3
-"""TASK-3: Expand ErrorEntry struct with severity, kind, context, and cause fields"""
-import base64, json, subprocess, sys
-from pathlib import Path
-
-TASK_ID = "TASK-3"
-STEPS = json.loads('[{"before_b64": "I1tkZXJpdmUoRGVidWcsIENsb25lLCBzZXJkZTo6U2VyaWFsaXplLCBib246OkJ1aWxkZXIpXQojW3NlcmRlKHJlbmFtZV9hbGwgPSAiY2FtZWxDYXNlIildCnB1YiBzdHJ1Y3QgRXJyb3JFbnRyeSB7CiAgICBwdWIgZmlsZTogU3RyaW5nLAogICAgI1tidWlsZGVyKGRlZmF1bHQpXQogICAgcHViIGxpbmU6IHVzaXplLAogICAgcHViIG1lc3NhZ2U6IFN0cmluZywKfQo=", "after_b64": "I1tkZXJpdmUoRGVidWcsIENsb25lLCBzZXJkZTo6U2VyaWFsaXplLCBib246OkJ1aWxkZXIpXQojW3NlcmRlKHJlbmFtZV9hbGwgPSAiY2FtZWxDYXNlIildCnB1YiBzdHJ1Y3QgRXJyb3JFbnRyeSB7CiAgICBwdWIgZmlsZTogU3RyaW5nLAogICAgI1tidWlsZGVyKGRlZmF1bHQpXQogICAgcHViIGxpbmU6IHVzaXplLAogICAgcHViIG1lc3NhZ2U6IFN0cmluZywKICAgIHB1YiBzZXZlcml0eTogRXJyb3JTZXZlcml0eSwKICAgIHB1YiBraW5kOiBTdHJpbmcsCiAgICAjW3NlcmRlKHNraXBfc2VyaWFsaXppbmdfaWYgPSAiT3B0aW9uOjppc19ub25lIildCiAgICBwdWIgY29udGV4dDogT3B0aW9uPEVycm9yQ29udGV4dD4sCiAgICAjW3NlcmRlKHNraXBfc2VyaWFsaXppbmdfaWYgPSAiT3B0aW9uOjppc19ub25lIildCiAgICBwdWIgY2F1c2U6IE9wdGlvbjxTdHJpbmc+LAp9Cg==", "target": "src/schema.rs", "index": 0, "is_create": false}]')
-
-for step in STEPS:
-    before = base64.b64decode(step["before_b64"]).decode()
-    after = base64.b64decode(step["after_b64"]).decode()
-    target = step["target"]
-    idx = step["index"]
-    is_create = step["is_create"]
-
-    if is_create:
-        target_path = Path(target)
-        target_path.parent.mkdir(parents=True, exist_ok=True)
-        target_path.write_text(after)
-        print(f"OK {TASK_ID} change {idx}: created {target}")
-    else:
-        target_path = Path(target)
-        content = target_path.read_text()
-        if before not in content:
-            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
-            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
-            sys.exit(1)
-
-        result = subprocess.run(
-            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
-            capture_output=True, text=True,
-        )
-        if result.returncode != 0:
-            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
-            sys.exit(result.returncode)
-
-        new_content = target_path.read_text()
-        if after and after not in new_content:
-            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
-            sys.exit(1)
-
-        print(f"OK {TASK_ID} change {idx}: applied to {target}")
-
-print(f"OK {TASK_ID}: all changes applied")
diff --git a/plans/compiled/TASK-3.sh b/plans/compiled/TASK-3.sh
deleted file mode 100755
index 0e353e0..0000000
--- a/plans/compiled/TASK-3.sh
+++ /dev/null
@@ -1,7 +0,0 @@
-#!/usr/bin/env bash
-set -euo pipefail
-# TASK-3: Expand ErrorEntry struct with severity, kind, context, and cause fields
-# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
-# Type: replace
-# File: src/schema.rs
-python3 "$(dirname "$0")/TASK-3.py"
diff --git a/plans/compiled/TASK-4.py b/plans/compiled/TASK-4.py
deleted file mode 100644
index 966b3aa..0000000
--- a/plans/compiled/TASK-4.py
+++ /dev/null
@@ -1,44 +0,0 @@
-#!/usr/bin/env python3
-"""TASK-4: Change parse_file to return ParsedFile with optional parse errors instead of Result"""
-import base64, json, subprocess, sys
-from pathlib import Path
-
-TASK_ID = "TASK-4"
-STEPS = json.loads('[{"before_b64": "dXNlIGNyYXRlOjpzY2hlbWE6OnsKICAgIEVycm9yLCBGaWxlSW5mbywgSW1wbEluZm8sIEltcGxJdGVtLCBJbXBsSXRlbUtpbmQsIEltcG9ydCwgSXRlbUF0dHJzLCBJdGVtS2luZCwgUHVibGljSXRlbSwKICAgIFJlRXhwb3J0LCBSZXN1bHQsIFN1Ym1vZHVsZURlY2wsCn07CnVzZSBzdGQ6OnBhdGg6OlBhdGg7CgovLyDilIDilIAgcGFyc2VfZmlsZSDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIAKCi8vLyBSZWFkIGFuZCBwYXJzZSBhIFJ1c3Qgc291cmNlIGZpbGUuIFJldHVybnMgdGhlIHJhdyBgc3luOjpGaWxlYCBBU1QgKG5lZWRlZAovLy8gYnkgYG1vZHVsZV90cmVlYCBmb3IgaW5saW5lIG1vZHVsZSBpdGVtIGV4dHJhY3Rpb24pIGFuZCB0aGUgZXh0cmFjdGVkCi8vLyBgRmlsZUluZm9gLiBPbiBwYXJzZSBmYWlsdXJlLCB3YXJucyB0byBzdGRlcnIgYW5kIHJldHVybnMgZW1wdHkgcmVzdWx0cy4KcHViIGZuIHBhcnNlX2ZpbGUocGF0aDogJlBhdGgpIC0+IFJlc3VsdDwoc3luOjpGaWxlLCBGaWxlSW5mbyk+IHsKICAgIGxldCBjb250ZW50ID0gc3RkOjpmczo6cmVhZF90b19zdHJpbmcocGF0aCkubWFwX2Vycih8c291cmNlfCBFcnJvcjo6RmlsZVJlYWQgewogICAgICAgIHBhdGg6IHBhdGgudG9fcGF0aF9idWYoKSwKICAgICAgICBzb3VyY2UsCiAgICB9KT87CgogICAgbGV0IGZpbGUgPSBtYXRjaCBzeW46OnBhcnNlX2ZpbGUoJmNvbnRlbnQpIHsKICAgICAgICBPayhmKSA9PiBmLAogICAgICAgIEVycihlKSA9PiB7CiAgICAgICAgICAgIGVwcmludGxuISgid2FybmluZzogZmFpbGVkIHRvIHBhcnNlIHt9OiB7fSIsIHBhdGguZGlzcGxheSgpLCBlKTsKICAgICAgICAgICAgbGV0IGVtcHR5ID0gc3luOjpGaWxlIHsKICAgICAgICAgICAgICAgIHNoZWJhbmc6IE5vbmUsCiAgICAgICAgICAgICAgICBhdHRyczogdmVjIVtdLAogICAgICAgICAgICAgICAgaXRlbXM6IHZlYyFbXSwKICAgICAgICAgICAgfTsKICAgICAgICAgICAgbGV0IGluZm8gPSBGaWxlSW5mbzo6ZGVmYXVsdCgpOwogICAgICAgICAgICByZXR1cm4gT2soKGVtcHR5LCBpbmZvKSk7CiAgICAgICAgfQogICAgfTsKCiAgICBsZXQgaW5mbyA9IEZpbGVJbmZvIHsKICAgICAgICBwdWJsaWNfaXRlbXM6IGV4dHJhY3RfcHVibGljX2l0ZW1zKCZmaWxlLml0ZW1zKSwKICAgICAgICBpbXBvcnRzOiBleHRyYWN0X2ltcG9ydHMoJmZpbGUuaXRlbXMpLAogICAgICAgIHJlX2V4cG9ydHM6IGV4dHJhY3RfcmVfZXhwb3J0cygmZmlsZS5pdGVtcyksCiAgICAgICAgc3VibW9kdWxlczogZXh0cmFjdF9zdWJtb2R1bGVzKCZmaWxlLml0ZW1zKSwKICAgICAgICBpbXBsczogZXh0cmFjdF9pbXBscygmZmlsZS5pdGVtcyksCiAgICB9OwoKICAgIE9rKChmaWxlLCBpbmZvKSkKfQ==", "after_b64": "dXNlIGNyYXRlOjpzY2hlbWE6OnsKICAgIEVycm9yLCBFcnJvckVudHJ5LCBFcnJvclNldmVyaXR5LCBGaWxlSW5mbywgSW1wbEluZm8sIEltcGxJdGVtLCBJbXBsSXRlbUtpbmQsIEltcG9ydCwKICAgIEl0ZW1BdHRycywgSXRlbUtpbmQsIFB1YmxpY0l0ZW0sIFJlRXhwb3J0LCBSZXN1bHQsIFN1Ym1vZHVsZURlY2wsCn07CnVzZSBzdGQ6OnBhdGg6OlBhdGg7CgovLyDilIDilIAgSW50ZXJuYWwgcGFyc2UgcmVzdWx0IHR5cGVzIOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgAoKLy8vIFJlc3VsdCBvZiBwYXJzaW5nIGEgUnVzdCBzb3VyY2UgZmlsZS4KLy8vCi8vLyBVbmxpa2UgYFJlc3VsdDxULCBFcnJvcj5gLCB0aGlzIHR5cGUgYWx3YXlzIHN1Y2NlZWRzIOKAlCBwYXJzZQovLy8gZmFpbHVyZXMgYXJlIHJlcG9ydGVkIGFzIGRhdGEsIG5vdCBhcyBlcnJvcnMsIHNvIHRoZSBjYWxsZXIKLy8vIGNhbiBjb250aW51ZSBwcm9jZXNzaW5nIG90aGVyIGZpbGVzLiBUaGUgY2FsbGVyIGNvbnN0cnVjdHMKLy8vIGBFcnJvckVudHJ5YCB2YWx1ZXMgZnJvbSBgU3luUGFyc2VFcnJvcmAgd2hlbiBuZWVkZWQuCnB1YiBzdHJ1Y3QgUGFyc2VkRmlsZSB7CiAgICBwdWIgYXN0OiBzeW46OkZpbGUsCiAgICBwdWIgZmlsZV9pbmZvOiBGaWxlSW5mbywKICAgIHB1YiBwYXJzZV9lcnJvcjogT3B0aW9uPFN5blBhcnNlRXJyb3I+LAp9CgovLy8gU3RydWN0dXJlZCBpbmZvcm1hdGlvbiBhYm91dCBhIHBhcnNlIGZhaWx1cmUuCnB1YiBzdHJ1Y3QgU3luUGFyc2VFcnJvciB7CiAgICBwdWIgbWVzc2FnZTogU3RyaW5nLAogICAgcHViIGxpbmU6IHVzaXplLAp9CgovLyDilIDilIAgcGFyc2VfZmlsZSDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIAKCi8vLyBSZWFkIGFuZCBwYXJzZSBhIFJ1c3Qgc291cmNlIGZpbGUuCi8vLwovLy8gT24gcGFyc2UgZmFpbHVyZSwgcmV0dXJucyB0aGUgb3JpZ2luYWwgZmlsZSBjb250ZW50IGFuZCBhCi8vLyBgU3luUGFyc2VFcnJvcmAgYWxvbmdzaWRlIGFuIGVtcHR5IGBGaWxlSW5mb2AuIENhbGxlcnMgdXNlIHRoZQovLy8gZXJyb3IgdG8gY29uc3RydWN0IGFuIGBFcnJvckVudHJ5YC4KcHViIGZuIHBhcnNlX2ZpbGUocGF0aDogJlBhdGgpIC0+IFBhcnNlZEZpbGUgewogICAgbGV0IGNvbnRlbnQgPSBtYXRjaCBzdGQ6OmZzOjpyZWFkX3RvX3N0cmluZyhwYXRoKSB7CiAgICAgICAgT2soYykgPT4gYywKICAgICAgICBFcnIoc291cmNlKSA9PiB7CiAgICAgICAgICAgIGxldCBlcnIgPSBTeW5QYXJzZUVycm9yIHsKICAgICAgICAgICAgICAgIG1lc3NhZ2U6IHNvdXJjZS50b19zdHJpbmcoKSwKICAgICAgICAgICAgICAgIGxpbmU6IDAsCiAgICAgICAgICAgIH07CiAgICAgICAgICAgIHJldHVybiBQYXJzZWRGaWxlIHsKICAgICAgICAgICAgICAgIGFzdDogc3luOjpGaWxlIHsKICAgICAgICAgICAgICAgICAgICBzaGViYW5nOiBOb25lLAogICAgICAgICAgICAgICAgICAgIGF0dHJzOiB2ZWMhW10sCiAgICAgICAgICAgICAgICAgICAgaXRlbXM6IHZlYyFbXSwKICAgICAgICAgICAgICAgIH0sCiAgICAgICAgICAgICAgICBmaWxlX2luZm86IEZpbGVJbmZvOjpkZWZhdWx0KCksCiAgICAgICAgICAgICAgICBwYXJzZV9lcnJvcjogU29tZShlcnIpLAogICAgICAgICAgICB9OwogICAgICAgIH0KICAgIH07CgogICAgbWF0Y2ggc3luOjpwYXJzZV9maWxlKCZjb250ZW50KSB7CiAgICAgICAgT2soZmlsZSkgPT4gewogICAgICAgICAgICBsZXQgZmlsZV9pbmZvID0gRmlsZUluZm8gewogICAgICAgICAgICAgICAgcHVibGljX2l0ZW1zOiBleHRyYWN0X3B1YmxpY19pdGVtcygmZmlsZS5pdGVtcyksCiAgICAgICAgICAgICAgICBpbXBvcnRzOiBleHRyYWN0X2ltcG9ydHMoJmZpbGUuaXRlbXMpLAogICAgICAgICAgICAgICAgcmVfZXhwb3J0czogZXh0cmFjdF9yZV9leHBvcnRzKCZmaWxlLml0ZW1zKSwKICAgICAgICAgICAgICAgIHN1Ym1vZHVsZXM6IGV4dHJhY3Rfc3VibW9kdWxlcygmZmlsZS5pdGVtcyksCiAgICAgICAgICAgICAgICBpbXBsczogZXh0cmFjdF9pbXBscygmZmlsZS5pdGVtcyksCiAgICAgICAgICAgIH07CiAgICAgICAgICAgIFBhcnNlZEZpbGUgewogICAgICAgICAgICAgICAgYXN0OiBmaWxlLAogICAgICAgICAgICAgICAgZmlsZV9pbmZvLAogICAgICAgICAgICAgICAgcGFyc2VfZXJyb3I6IE5vbmUsCiAgICAgICAgICAgIH0KICAgICAgICB9LAogICAgICAgIEVycihlKSA9PiB7CiAgICAgICAgICAgIGxldCBsaW5lID0gZS5zcGFuKCkuc3RhcnQoKS5saW5lOwogICAgICAgICAgICBsZXQgZXJyID0gU3luUGFyc2VFcnJvciB7CiAgICAgICAgICAgICAgICBtZXNzYWdlOiBlLnRvX3N0cmluZygpLAogICAgICAgICAgICAgICAgbGluZSwKICAgICAgICAgICAgfTsKICAgICAgICAgICAgUGFyc2VkRmlsZSB7CiAgICAgICAgICAgICAgICBhc3Q6IHN5bjo6RmlsZSB7CiAgICAgICAgICAgICAgICAgICAgc2hlYmFuZzogTm9uZSwKICAgICAgICAgICAgICAgICAgICBhdHRyczogdmVjIVtdLAogICAgICAgICAgICAgICAgICAgIGl0ZW1zOiB2ZWMhW10sCiAgICAgICAgICAgICAgICB9LAogICAgICAgICAgICAgICAgZmlsZV9pbmZvOiBGaWxlSW5mbzo6ZGVmYXVsdCgpLAogICAgICAgICAgICAgICAgcGFyc2VfZXJyb3I6IFNvbWUoZXJyKSwKICAgICAgICAgICAgfQogICAgICAgIH0KICAgIH0KfQoKcHViKGNyYXRlKSBmbiBidWlsZF9wYXJzZV9lcnJvcl9lbnRyeShwYXRoOiAmUGF0aCwgZXJyOiAmU3luUGFyc2VFcnJvcikgLT4gRXJyb3JFbnRyeSB7CiAgICBFcnJvckVudHJ5OjpidWlsZGVyKCkKICAgICAgICAuZmlsZShwYXRoLnRvX3N0cmluZ19sb3NzeSgpLnRvX3N0cmluZygpKQogICAgICAgIC5saW5lKGVyci5saW5lKQogICAgICAgIC5tZXNzYWdlKGVyci5tZXNzYWdlLmNsb25lKCkpCiAgICAgICAgLnNldmVyaXR5KEVycm9yU2V2ZXJpdHk6OkVycm9yKQogICAgICAgIC5raW5kKCJzeW5fcGFyc2VfZXJyb3IiLnRvX3N0cmluZygpKQogICAgICAgIC5idWlsZCgpCn0=", "target": "src/file_parser.rs", "index": 0, "is_create": false}]')
-
-for step in STEPS:
-    before = base64.b64decode(step["before_b64"]).decode()
-    after = base64.b64decode(step["after_b64"]).decode()
-    target = step["target"]
-    idx = step["index"]
-    is_create = step["is_create"]
-
-    if is_create:
-        target_path = Path(target)
-        target_path.parent.mkdir(parents=True, exist_ok=True)
-        target_path.write_text(after)
-        print(f"OK {TASK_ID} change {idx}: created {target}")
-    else:
-        target_path = Path(target)
-        content = target_path.read_text()
-        if before not in content:
-            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
-            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
-            sys.exit(1)
-
-        result = subprocess.run(
-            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
-            capture_output=True, text=True,
-        )
-        if result.returncode != 0:
-            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
-            sys.exit(result.returncode)
-
-        new_content = target_path.read_text()
-        if after and after not in new_content:
-            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
-            sys.exit(1)
-
-        print(f"OK {TASK_ID} change {idx}: applied to {target}")
-
-print(f"OK {TASK_ID}: all changes applied")
diff --git a/plans/compiled/TASK-4.sh b/plans/compiled/TASK-4.sh
deleted file mode 100755
index aae910d..0000000
--- a/plans/compiled/TASK-4.sh
+++ /dev/null
@@ -1,7 +0,0 @@
-#!/usr/bin/env bash
-set -euo pipefail
-# TASK-4: Change parse_file to return ParsedFile with optional parse errors instead of Result
-# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
-# Type: replace
-# File: src/file_parser.rs
-python3 "$(dirname "$0")/TASK-4.py"
diff --git a/plans/compiled/TASK-5.py b/plans/compiled/TASK-5.py
deleted file mode 100644
index 94c7ef6..0000000
--- a/plans/compiled/TASK-5.py
+++ /dev/null
@@ -1,44 +0,0 @@
-#!/usr/bin/env python3
-"""TASK-5: Refactor module_tree.rs: fix path fallbacks, add error collection, change build_module_tree return type"""
-import base64, json, subprocess, sys
-from pathlib import Path
-
-TASK_ID = "TASK-5"
-STEPS = json.loads('[{"before_b64": "dXNlIGNyYXRlOjpzY2hlbWE6OntGaWxlSW5mbywgTW9kdWxlSW5mbywgUmVzdWx0LCBTdWJtb2R1bGVEZWNsfTs=", "after_b64": "dXNlIGNyYXRlOjpzY2hlbWE6OntFcnJvckNvbnRleHQsIEVycm9yRW50cnksIEVycm9yU2V2ZXJpdHksIEZpbGVJbmZvLCBNb2R1bGVJbmZvLCBSZXN1bHQsIFN1Ym1vZHVsZURlY2x9Ow==", "target": "src/module_tree.rs", "index": 0, "is_create": false}, {"before_b64": "Ly8vIEJ1aWxkIHRoZSBmdWxsIG1vZHVsZSB0cmVlIGZvciBhIGNyYXRlIHN0YXJ0aW5nIGZyb20gaXRzIGVudHJ5IHBvaW50Ci8vLyAoZS5nLiwgYHNyYy9saWIucnNgKS4gUmV0dXJucyBhIGZsYXQgYFZlYzxNb2R1bGVJbmZvPmAgY29udGFpbmluZyB0aGUKLy8vIHJvb3QgbW9kdWxlIGFuZCBhbGwgcmVjdXJzaXZlbHkgZGlzY292ZXJlZCBzdWJtb2R1bGVzLgpwdWIgZm4gYnVpbGRfbW9kdWxlX3RyZWUoY3JhdGVfcm9vdDogJlBhdGgsIGNyYXRlX25hbWU6ICZzdHIpIC0+IFJlc3VsdDxWZWM8TW9kdWxlSW5mbz4+IHsKICAgIGxldCBtdXQgdmlzaXRlZCA9IEhhc2hTZXQ6Om5ldygpOwogICAgbGV0IHBhcmVudF9kaXIgPSBjcmF0ZV9yb290LnBhcmVudCgpLnVud3JhcF9vcl9lbHNlKHx8IFBhdGg6Om5ldygiLiIpKTsKCiAgICBsZXQgKGFzdCwgZmlsZV9pbmZvKSA9IGZpbGVfcGFyc2VyOjpwYXJzZV9maWxlKGNyYXRlX3Jvb3QpPzsKICAgIHZpc2l0ZWQuaW5zZXJ0KGNyYXRlX3Jvb3QudG9fcGF0aF9idWYoKSk7CgogICAgbGV0IHJvb3RfbW9kdWxlID0gYnVpbGRfbW9kdWxlX2luZm8oCiAgICAgICAgY3JhdGVfbmFtZSwKICAgICAgICBjcmF0ZV9yb290LAogICAgICAgICJwdWIiLAogICAgICAgICZmaWxlX2luZm8ucHVibGljX2l0ZW1zLAogICAgICAgICZmaWxlX2luZm8uaW1wb3J0cywKICAgICAgICAmZmlsZV9pbmZvLnJlX2V4cG9ydHMsCiAgICAgICAgJmZpbGVfaW5mby5zdWJtb2R1bGVzLAogICAgKTsKCiAgICBsZXQgbXV0IG1vZHVsZXMgPSB2ZWMhW3Jvb3RfbW9kdWxlXTsKCiAgICBmb3Igc3ViIGluICZmaWxlX2luZm8uc3VibW9kdWxlcyB7CiAgICAgICAgaWYgc3ViLmlzX3Rlc3QgewogICAgICAgICAgICBjb250aW51ZTsKICAgICAgICB9CiAgICAgICAgbGV0IHN1Yl9tb2R1bGVfcGF0aCA9IGZvcm1hdCEoInt9Ojp7fSIsIGNyYXRlX25hbWUsIHN1Yi5uYW1lKTsKICAgICAgICBsZXQgY2hpbGRfbW9kdWxlcyA9IHByb2Nlc3Nfc3VibW9kdWxlKAogICAgICAgICAgICAmc3ViX21vZHVsZV9wYXRoLAogICAgICAgICAgICAmc3ViLm5hbWUsCiAgICAgICAgICAgICZhc3QuaXRlbXMsCiAgICAgICAgICAgIHBhcmVudF9kaXIsCiAgICAgICAgICAgIGNyYXRlX3Jvb3QsCiAgICAgICAgICAgICZtdXQgdmlzaXRlZCwKICAgICAgICApPzsKICAgICAgICBtb2R1bGVzLmV4dGVuZChjaGlsZF9tb2R1bGVzKTsKICAgIH0KCiAgICBPayhtb2R1bGVzKQp9", "after_b64": "Ly8vIEJ1aWxkIHRoZSBmdWxsIG1vZHVsZSB0cmVlIGZvciBhIGNyYXRlIHN0YXJ0aW5nIGZyb20gaXRzIGVudHJ5IHBvaW50Ci8vLyAoZS5nLiwgYHNyYy9saWIucnNgKS4gUmV0dXJucyBhIHR1cGxlIG9mIG1vZHVsZSBpbmZvIGFuZCBhbnkgZXJyb3JzCi8vLyBlbmNvdW50ZXJlZCBkdXJpbmcgc3VibW9kdWxlIHBhcnNpbmcgKGluY2x1ZGluZyBvcnBoYW5lZCBtb2R1bGUgd2FybmluZ3MpLgpwdWIgZm4gYnVpbGRfbW9kdWxlX3RyZWUoCiAgICBjcmF0ZV9yb290OiAmUGF0aCwKICAgIGNyYXRlX25hbWU6ICZzdHIsCikgLT4gKFZlYzxNb2R1bGVJbmZvPiwgVmVjPGNyYXRlOjpzY2hlbWE6OkVycm9yRW50cnk+KSB7CgogICAgbGV0IG11dCB2aXNpdGVkID0gSGFzaFNldDo6bmV3KCk7CiAgICBsZXQgcGFyZW50X2RpciA9IGNyYXRlX3Jvb3QucGFyZW50KCkudW53cmFwX29yKGNyYXRlX3Jvb3QpOwoKICAgIGxldCBwYXJzZWQgPSBmaWxlX3BhcnNlcjo6cGFyc2VfZmlsZShjcmF0ZV9yb290KTsKICAgIGxldCBtdXQgZXJyb3JzOiBWZWM8RXJyb3JFbnRyeT4gPSBWZWM6Om5ldygpOwogICAgaWYgbGV0IFNvbWUocmVmIGVycikgPSBwYXJzZWQucGFyc2VfZXJyb3IgewogICAgICAgIGVycm9ycy5wdXNoKGNyYXRlOjpmaWxlX3BhcnNlcjo6YnVpbGRfcGFyc2VfZXJyb3JfZW50cnkoY3JhdGVfcm9vdCwgZXJyKSk7CiAgICB9CiAgICB2aXNpdGVkLmluc2VydChjcmF0ZV9yb290LnRvX3BhdGhfYnVmKCkpOwoKICAgIGxldCByb290X21vZHVsZSA9IGJ1aWxkX21vZHVsZV9pbmZvKAogICAgICAgIGNyYXRlX25hbWUsCiAgICAgICAgY3JhdGVfcm9vdCwKICAgICAgICAicHViIiwKICAgICAgICAmcGFyc2VkLmZpbGVfaW5mby5wdWJsaWNfaXRlbXMsCiAgICAgICAgJnBhcnNlZC5maWxlX2luZm8uaW1wb3J0cywKICAgICAgICAmcGFyc2VkLmZpbGVfaW5mby5yZV9leHBvcnRzLAogICAgICAgICZwYXJzZWQuZmlsZV9pbmZvLnN1Ym1vZHVsZXMsCiAgICApOwoKICAgIGxldCBtdXQgbW9kdWxlcyA9IHZlYyFbcm9vdF9tb2R1bGVdOwoKICAgIGZvciBzdWIgaW4gJnBhcnNlZC5maWxlX2luZm8uc3VibW9kdWxlcyB7CiAgICAgICAgaWYgc3ViLmlzX3Rlc3QgewogICAgICAgICAgICBjb250aW51ZTsKICAgICAgICB9CiAgICAgICAgbGV0IHN1Yl9tb2R1bGVfcGF0aCA9IGZvcm1hdCEoInt9Ojp7fSIsIGNyYXRlX25hbWUsIHN1Yi5uYW1lKTsKICAgICAgICBsZXQgKGNoaWxkX21vZHVsZXMsIGNoaWxkX2Vycm9ycykgPSBwcm9jZXNzX3N1Ym1vZHVsZSgKICAgICAgICAgICAgJnN1Yl9tb2R1bGVfcGF0aCwKICAgICAgICAgICAgJnN1Yi5uYW1lLAogICAgICAgICAgICAmcGFyc2VkLmFzdC5pdGVtcywKICAgICAgICAgICAgcGFyZW50X2RpciwKICAgICAgICAgICAgY3JhdGVfcm9vdCwKICAgICAgICAgICAgJm11dCB2aXNpdGVkLAogICAgICAgICk7CiAgICAgICAgZXJyb3JzLmV4dGVuZChjaGlsZF9lcnJvcnMpOwogICAgICAgIG1vZHVsZXMuZXh0ZW5kKGNoaWxkX21vZHVsZXMpOwogICAgfQoKICAgIChtb2R1bGVzLCBlcnJvcnMpCn0=", "target": "src/module_tree.rs", "index": 1, "is_create": false}, {"before_b64": "ICAgIGxldCBTb21lKG1vZF9pdGVtKSA9IG1vZF9pdGVtIGVsc2UgewogICAgICAgIGVwcmludGxuISgid2FybmluZzogb3JwaGFuZWQgbW9kdWxlIHt9IiwgbW9kdWxlX3BhdGgpOwogICAgICAgIHJldHVybiBPayh2ZWMhW01vZHVsZUluZm86OmJ1aWxkZXIoKQogICAgICAgICAgICAucGF0aChtb2R1bGVfcGF0aC50b19zdHJpbmcoKSkKICAgICAgICAgICAgLmZpbGUoIjx1bnJlc29sdmVkPiIudG9fc3RyaW5nKCkpCiAgICAgICAgICAgIC52aXNpYmlsaXR5KCJwcml2YXRlIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgLmJ1aWxkKCldKTsKICAgIH07", "after_b64": "ICAgIGxldCBTb21lKG1vZF9pdGVtKSA9IG1vZF9pdGVtIGVsc2UgewogICAgICAgIGxldCBlcnIgPSBFcnJvckVudHJ5OjpidWlsZGVyKCkKICAgICAgICAgICAgLmZpbGUoU3RyaW5nOjpuZXcoKSkKICAgICAgICAgICAgLm1lc3NhZ2UoZm9ybWF0ISgib3JwaGFuZWQgbW9kdWxlOiB7bW9kdWxlX3BhdGh9IikpCiAgICAgICAgICAgIC5zZXZlcml0eShFcnJvclNldmVyaXR5OjpXYXJuaW5nKQogICAgICAgICAgICAua2luZCgib3JwaGFuZWRfbW9kdWxlIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgLmNvbnRleHQoRXJyb3JDb250ZXh0OjpidWlsZGVyKCkKICAgICAgICAgICAgICAgIC5tb2R1bGVfcGF0aChtb2R1bGVfcGF0aC50b19zdHJpbmcoKSkKICAgICAgICAgICAgICAgIC5idWlsZCgpKQogICAgICAgICAgICAuYnVpbGQoKTsKICAgICAgICByZXR1cm4gKHZlYyFbTW9kdWxlSW5mbzo6YnVpbGRlcigpCiAgICAgICAgICAgIC5wYXRoKG1vZHVsZV9wYXRoLnRvX3N0cmluZygpKQogICAgICAgICAgICAuZmlsZSgiPHVucmVzb2x2ZWQ+Ii50b19zdHJpbmcoKSkKICAgICAgICAgICAgLnZpc2liaWxpdHkoInByaXZhdGUiLnRvX3N0cmluZygpKQogICAgICAgICAgICAuYnVpbGQoKV0sIHZlYyFbZXJyXSk7CiAgICB9Ow==", "target": "src/module_tree.rs", "index": 2, "is_create": false}, {"before_b64": "ICAgICAgICBsZXQgU29tZShyZWYgZmlsZV9wYXRoKSA9IGZpbGVfcGF0aCBlbHNlIHsKICAgICAgICAgICAgZXByaW50bG4hKCJ3YXJuaW5nOiBvcnBoYW5lZCBtb2R1bGUge30iLCBtb2R1bGVfcGF0aCk7CiAgICAgICAgICAgIHJldHVybiBPayh2ZWMhW01vZHVsZUluZm86OmJ1aWxkZXIoKQogICAgICAgICAgICAgICAgLnBhdGgobW9kdWxlX3BhdGgudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAuZmlsZSgiPHVucmVzb2x2ZWQ+Ii50b19zdHJpbmcoKSkKICAgICAgICAgICAgICAgIC52aXNpYmlsaXR5KHZpc2liaWxpdHkudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAuYnVpbGQoKV0pOwogICAgICAgIH07", "after_b64": "ICAgICAgICBsZXQgU29tZShyZWYgZmlsZV9wYXRoKSA9IGZpbGVfcGF0aCBlbHNlIHsKICAgICAgICAgICAgbGV0IGVyciA9IEVycm9yRW50cnk6OmJ1aWxkZXIoKQogICAgICAgICAgICAgICAgLmZpbGUoU3RyaW5nOjpuZXcoKSkKICAgICAgICAgICAgICAgIC5tZXNzYWdlKGZvcm1hdCEoIm9ycGhhbmVkIG1vZHVsZToge21vZHVsZV9wYXRofSIpKQogICAgICAgICAgICAgICAgLnNldmVyaXR5KEVycm9yU2V2ZXJpdHk6Oldhcm5pbmcpCiAgICAgICAgICAgICAgICAua2luZCgib3JwaGFuZWRfbW9kdWxlIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgICAgIC5jb250ZXh0KEVycm9yQ29udGV4dDo6YnVpbGRlcigpCiAgICAgICAgICAgICAgICAgICAgLm1vZHVsZV9wYXRoKG1vZHVsZV9wYXRoLnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgICAgIC5idWlsZCgpKQogICAgICAgICAgICAgICAgLmJ1aWxkKCk7CiAgICAgICAgICAgIHJldHVybiAodmVjIVtNb2R1bGVJbmZvOjpidWlsZGVyKCkKICAgICAgICAgICAgICAgIC5wYXRoKG1vZHVsZV9wYXRoLnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgLmZpbGUoIjx1bnJlc29sdmVkPiIudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAudmlzaWJpbGl0eSh2aXNpYmlsaXR5LnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgLmJ1aWxkKCldLCB2ZWMhW2Vycl0pOwogICAgICAgIH07", "target": "src/module_tree.rs", "index": 3, "is_create": false}, {"before_b64": "ICAgICAgICBsZXQgKGFzdCwgZmlsZV9pbmZvKSA9IGZpbGVfcGFyc2VyOjpwYXJzZV9maWxlKGZpbGVfcGF0aCk/OwogICAgICAgIHByb2Nlc3NfbW9kdWxlX2luZm8oCiAgICAgICAgICAgIG1vZHVsZV9wYXRoLAogICAgICAgICAgICBmaWxlX3BhdGgsCiAgICAgICAgICAgIHZpc2liaWxpdHksCiAgICAgICAgICAgICZmaWxlX2luZm8sCiAgICAgICAgICAgICZhc3QuaXRlbXMsCiAgICAgICAgICAgICZmaWxlX3BhdGgucGFyZW50KCkudW53cmFwX29yX2Vsc2UofHwgUGF0aDo6bmV3KCIuIikpLAogICAgICAgICAgICB2aXNpdGVkLAogICAgICAgICk=", "after_b64": "ICAgICAgICBsZXQgcGFyc2VkID0gZmlsZV9wYXJzZXI6OnBhcnNlX2ZpbGUoZmlsZV9wYXRoKTsKICAgICAgICBsZXQgbXV0IGVycm9yczogVmVjPEVycm9yRW50cnk+ID0gVmVjOjpuZXcoKTsKICAgICAgICBpZiBsZXQgU29tZShyZWYgZXJyKSA9IHBhcnNlZC5wYXJzZV9lcnJvciB7CiAgICAgICAgICAgIGVycm9ycy5wdXNoKGNyYXRlOjpmaWxlX3BhcnNlcjo6YnVpbGRfcGFyc2VfZXJyb3JfZW50cnkoZmlsZV9wYXRoLCBlcnIpKTsKICAgICAgICB9CiAgICAgICAgcHJvY2Vzc19tb2R1bGVfaW5mbygKICAgICAgICAgICAgbW9kdWxlX3BhdGgsCiAgICAgICAgICAgIGZpbGVfcGF0aCwKICAgICAgICAgICAgdmlzaWJpbGl0eSwKICAgICAgICAgICAgJnBhcnNlZC5maWxlX2luZm8sCiAgICAgICAgICAgICZwYXJzZWQuYXN0Lml0ZW1zLAogICAgICAgICAgICAmZmlsZV9wYXRoLnBhcmVudCgpLnVud3JhcF9vcihmaWxlX3BhdGgpLAogICAgICAgICAgICB2aXNpdGVkLAogICAgICAgICAgICAmbXV0IGVycm9ycywKICAgICAgICAp", "target": "src/module_tree.rs", "index": 4, "is_create": false}, {"before_b64": "ICAgICAgICBpZiB2aXNpdGVkLmNvbnRhaW5zKGZpbGVfcGF0aC5hc19wYXRoKCkpIHsKICAgICAgICAgICAgcmV0dXJuIE9rKHZlYyFbXSk7IC8vIGN5Y2xlIGRldGVjdGVkCiAgICAgICAgfQ==", "after_b64": "ICAgICAgICBpZiB2aXNpdGVkLmNvbnRhaW5zKGZpbGVfcGF0aC5hc19wYXRoKCkpIHsKICAgICAgICAgICAgcmV0dXJuICh2ZWMhW10sIHZlYyFbXSk7IC8vIGN5Y2xlIGRldGVjdGVkCiAgICAgICAgfQ==", "target": "src/module_tree.rs", "index": 5, "is_create": false}, {"before_b64": "Ly8vIFJlc29sdmUgYSBgbW9kIG5hbWU7YCBkZWNsYXJhdGlvbiB0byBhIGZpbGUgcGF0aC4KLy8vIFRyaWVzIGB7cGFyZW50X2Rpcn0ve21vZF9uYW1lfS5yc2AgZmlyc3QsIHRoZW4gYHtwYXJlbnRfZGlyfS97bW9kX25hbWV9L21vZC5yc2AuCnB1YiBmbiByZXNvbHZlX21vZHVsZV9wYXRoKHBhcmVudF9kaXI6ICZQYXRoLCBtb2RfbmFtZTogJnN0cikgLT4gT3B0aW9uPFBhdGhCdWY+IHs=", "after_b64": "Ly8vIFJlc29sdmUgYSBgbW9kIG5hbWU7YCBkZWNsYXJhdGlvbiB0byBhIGZpbGUgcGF0aC4KLy8vIFRyaWVzIGB7cGFyZW50X2Rpcn0ve21vZF9uYW1lfS5yc2AgZmlyc3QsIHRoZW4gYHtwYXJlbnRfZGlyfS97bW9kX25hbWV9L21vZC5yc2AuCi8vLwovLy8gUmV0dXJucyBgTm9uZWAgaWYgbmVpdGhlciBwYXRoIGV4aXN0cy4KcHViIGZuIHJlc29sdmVfbW9kdWxlX3BhdGgocGFyZW50X2RpcjogJlBhdGgsIG1vZF9uYW1lOiAmc3RyKSAtPiBPcHRpb248UGF0aEJ1Zj4gew==", "target": "src/module_tree.rs", "index": 6, "is_create": false}, {"before_b64": "Zm4gcHJvY2Vzc19zdWJtb2R1bGUoCiAgICBtb2R1bGVfcGF0aDogJnN0ciwKICAgIG1vZF9uYW1lOiAmc3RyLAogICAgcGFyZW50X2l0ZW1zOiAmW3N5bjo6SXRlbV0sCiAgICBwYXJlbnRfZGlyOiAmUGF0aCwKICAgIHBhcmVudF9maWxlOiAmUGF0aCwKICAgIHZpc2l0ZWQ6ICZtdXQgSGFzaFNldDxQYXRoQnVmPiwKKSAtPiBSZXN1bHQ8VmVjPE1vZHVsZUluZm8+PiB7", "after_b64": "Zm4gcHJvY2Vzc19zdWJtb2R1bGUoCiAgICBtb2R1bGVfcGF0aDogJnN0ciwKICAgIG1vZF9uYW1lOiAmc3RyLAogICAgcGFyZW50X2l0ZW1zOiAmW3N5bjo6SXRlbV0sCiAgICBwYXJlbnRfZGlyOiAmUGF0aCwKICAgIHBhcmVudF9maWxlOiAmUGF0aCwKICAgIHZpc2l0ZWQ6ICZtdXQgSGFzaFNldDxQYXRoQnVmPiwKKSAtPiAoVmVjPE1vZHVsZUluZm8+LCBWZWM8Y3JhdGU6OnNjaGVtYTo6RXJyb3JFbnRyeT4pIHs=", "target": "src/module_tree.rs", "index": 7, "is_create": false}, {"before_b64": "ICAgIGlmIGxldCBTb21lKChfLCByZWYgaW5saW5lX2l0ZW1zKSkgPSBtb2RfaXRlbS5jb250ZW50IHsKICAgICAgICAvLyBJbmxpbmUgbW9kdWxlOiBwcm9jZXNzIGl0cyBib2R5IGl0ZW1zIGRpcmVjdGx5IChubyBmaWxlIGxvb2t1cCkuCiAgICAgICAgcHJvY2Vzc19tb2R1bGVfaXRlbXMoCiAgICAgICAgICAgIG1vZHVsZV9wYXRoLAogICAgICAgICAgICBwYXJlbnRfZmlsZSwKICAgICAgICAgICAgdmlzaWJpbGl0eSwKICAgICAgICAgICAgaW5saW5lX2l0ZW1zLAogICAgICAgICAgICBwYXJlbnRfZGlyLAogICAgICAgICAgICB2aXNpdGVkLAogICAgICAgICkKICAgIH0gZWxzZSB7", "after_b64": "ICAgIGlmIGxldCBTb21lKChfLCByZWYgaW5saW5lX2l0ZW1zKSkgPSBtb2RfaXRlbS5jb250ZW50IHsKICAgICAgICAvLyBJbmxpbmUgbW9kdWxlOiBwcm9jZXNzIGl0cyBib2R5IGl0ZW1zIGRpcmVjdGx5IChubyBmaWxlIGxvb2t1cCkuCiAgICAgICAgbGV0IChtb2R1bGVzLCBlcnJzKSA9IHByb2Nlc3NfbW9kdWxlX2l0ZW1zKAogICAgICAgICAgICBtb2R1bGVfcGF0aCwKICAgICAgICAgICAgcGFyZW50X2ZpbGUsCiAgICAgICAgICAgIHZpc2liaWxpdHksCiAgICAgICAgICAgIGlubGluZV9pdGVtcywKICAgICAgICAgICAgcGFyZW50X2RpciwKICAgICAgICAgICAgdmlzaXRlZCwKICAgICAgICApOwogICAgICAgIHJldHVybiAobW9kdWxlcywgZXJycyk7CiAgICB9IGVsc2Ugew==", "target": "src/module_tree.rs", "index": 8, "is_create": false}, {"before_b64": "Zm4gcHJvY2Vzc19tb2R1bGVfaXRlbXMoCiAgICBtb2R1bGVfcGF0aDogJnN0ciwKICAgIGZpbGVfcGF0aDogJlBhdGgsCiAgICB2aXNpYmlsaXR5OiAmc3RyLAogICAgaXRlbXM6ICZbc3luOjpJdGVtXSwKICAgIHBhcmVudF9kaXI6ICZQYXRoLAogICAgdmlzaXRlZDogJm11dCBIYXNoU2V0PFBhdGhCdWY+LAopIC0+IFJlc3VsdDxWZWM8TW9kdWxlSW5mbz4+IHs=", "after_b64": "Zm4gcHJvY2Vzc19tb2R1bGVfaXRlbXMoCiAgICBtb2R1bGVfcGF0aDogJnN0ciwKICAgIGZpbGVfcGF0aDogJlBhdGgsCiAgICB2aXNpYmlsaXR5OiAmc3RyLAogICAgaXRlbXM6ICZbc3luOjpJdGVtXSwKICAgIHBhcmVudF9kaXI6ICZQYXRoLAogICAgdmlzaXRlZDogJm11dCBIYXNoU2V0PFBhdGhCdWY+LAopIC0+IChWZWM8TW9kdWxlSW5mbz4sIFZlYzxjcmF0ZTo6c2NoZW1hOjpFcnJvckVudHJ5Pikgew==", "target": "src/module_tree.rs", "index": 9, "is_create": false}, {"before_b64": "ICAgIGxldCBmaWxlX2luZm8gPSBGaWxlSW5mbyB7CiAgICAgICAgcHVibGljX2l0ZW1zOiBmaWxlX3BhcnNlcjo6ZXh0cmFjdF9wdWJsaWNfaXRlbXMoaXRlbXMpLAogICAgICAgIGltcG9ydHM6IGZpbGVfcGFyc2VyOjpleHRyYWN0X2ltcG9ydHMoaXRlbXMpLAogICAgICAgIHJlX2V4cG9ydHM6IGZpbGVfcGFyc2VyOjpleHRyYWN0X3JlX2V4cG9ydHMoaXRlbXMpLAogICAgICAgIHN1Ym1vZHVsZXM6IGZpbGVfcGFyc2VyOjpleHRyYWN0X3N1Ym1vZHVsZXMoaXRlbXMpLAogICAgICAgIGltcGxzOiBmaWxlX3BhcnNlcjo6ZXh0cmFjdF9pbXBscyhpdGVtcyksCiAgICB9OwogICAgcHJvY2Vzc19tb2R1bGVfaW5mbyhtb2R1bGVfcGF0aCwgZmlsZV9wYXRoLCB2aXNpYmlsaXR5LCAmZmlsZV9pbmZvLCBpdGVtcywgcGFyZW50X2RpciwgdmlzaXRlZCkKfQ==", "after_b64": "ICAgIGxldCBmaWxlX2luZm8gPSBGaWxlSW5mbyB7CiAgICAgICAgcHVibGljX2l0ZW1zOiBmaWxlX3BhcnNlcjo6ZXh0cmFjdF9wdWJsaWNfaXRlbXMoaXRlbXMpLAogICAgICAgIGltcG9ydHM6IGZpbGVfcGFyc2VyOjpleHRyYWN0X2ltcG9ydHMoaXRlbXMpLAogICAgICAgIHJlX2V4cG9ydHM6IGZpbGVfcGFyc2VyOjpleHRyYWN0X3JlX2V4cG9ydHMoaXRlbXMpLAogICAgICAgIHN1Ym1vZHVsZXM6IGZpbGVfcGFyc2VyOjpleHRyYWN0X3N1Ym1vZHVsZXMoaXRlbXMpLAogICAgICAgIGltcGxzOiBmaWxlX3BhcnNlcjo6ZXh0cmFjdF9pbXBscyhpdGVtcyksCiAgICB9OwogICAgcHJvY2Vzc19tb2R1bGVfaW5mbyhtb2R1bGVfcGF0aCwgZmlsZV9wYXRoLCB2aXNpYmlsaXR5LCAmZmlsZV9pbmZvLCBpdGVtcywgcGFyZW50X2RpciwgdmlzaXRlZCwgJm11dCBWZWM6Om5ldygpKQp9", "target": "src/module_tree.rs", "index": 10, "is_create": false}, {"before_b64": "Zm4gcHJvY2Vzc19tb2R1bGVfaW5mbygKICAgIG1vZHVsZV9wYXRoOiAmc3RyLAogICAgZmlsZV9wYXRoOiAmUGF0aCwKICAgIHZpc2liaWxpdHk6ICZzdHIsCiAgICBmaWxlX2luZm86ICZGaWxlSW5mbywKICAgIGl0ZW1zOiAmW3N5bjo6SXRlbV0sCiAgICBfcGFyZW50X2RpcjogJlBhdGgsCiAgICB2aXNpdGVkOiAmbXV0IEhhc2hTZXQ8UGF0aEJ1Zj4sCikgLT4gUmVzdWx0PFZlYzxNb2R1bGVJbmZvPj4gewogICAgbGV0IG11dCBtb2R1bGVzID0gdmVjIVtidWlsZF9tb2R1bGVfaW5mbygKICAgICAgICBtb2R1bGVfcGF0aCwKICAgICAgICBmaWxlX3BhdGgsCiAgICAgICAgdmlzaWJpbGl0eSwKICAgICAgICAmZmlsZV9pbmZvLnB1YmxpY19pdGVtcywKICAgICAgICAmZmlsZV9pbmZvLmltcG9ydHMsCiAgICAgICAgJmZpbGVfaW5mby5yZV9leHBvcnRzLAogICAgICAgICZmaWxlX2luZm8uc3VibW9kdWxlcywKICAgICldOwoKICAgIGZvciBzdWIgaW4gJmZpbGVfaW5mby5zdWJtb2R1bGVzIHsKICAgICAgICBpZiBzdWIuaXNfdGVzdCB7CiAgICAgICAgICAgIGNvbnRpbnVlOwogICAgICAgIH0KICAgICAgICBsZXQgY2hpbGRfcGF0aCA9IGZvcm1hdCEoInt9Ojp7fSIsIG1vZHVsZV9wYXRoLCBzdWIubmFtZSk7CiAgICAgICAgbGV0IGNoaWxkX2RpciA9IGZpbGVfcGF0aC5wYXJlbnQoKS51bndyYXBfb3JfZWxzZSh8fCBQYXRoOjpuZXcoIi4iKSk7CiAgICAgICAgbGV0IGNoaWxkX21vZHVsZXMgPSBwcm9jZXNzX3N1Ym1vZHVsZSgKICAgICAgICAgICAgJmNoaWxkX3BhdGgsCiAgICAgICAgICAgICZzdWIubmFtZSwKICAgICAgICAgICAgaXRlbXMsCiAgICAgICAgICAgIGNoaWxkX2RpciwKICAgICAgICAgICAgZmlsZV9wYXRoLAogICAgICAgICAgICB2aXNpdGVkLAogICAgICAgICk/OwogICAgICAgIG1vZHVsZXMuZXh0ZW5kKGNoaWxkX21vZHVsZXMpOwogICAgfQoKICAgIE9rKG1vZHVsZXMpCn0=", "after_b64": "Zm4gcHJvY2Vzc19tb2R1bGVfaW5mbygKICAgIG1vZHVsZV9wYXRoOiAmc3RyLAogICAgZmlsZV9wYXRoOiAmUGF0aCwKICAgIHZpc2liaWxpdHk6ICZzdHIsCiAgICBmaWxlX2luZm86ICZGaWxlSW5mbywKICAgIGl0ZW1zOiAmW3N5bjo6SXRlbV0sCiAgICBfcGFyZW50X2RpcjogJlBhdGgsCiAgICB2aXNpdGVkOiAmbXV0IEhhc2hTZXQ8UGF0aEJ1Zj4sCiAgICBlcnJvcnM6ICZtdXQgVmVjPGNyYXRlOjpzY2hlbWE6OkVycm9yRW50cnk+LAopIC0+IChWZWM8TW9kdWxlSW5mbz4sIFZlYzxjcmF0ZTo6c2NoZW1hOjpFcnJvckVudHJ5PikgewogICAgbGV0IG11dCBtb2R1bGVzID0gdmVjIVtidWlsZF9tb2R1bGVfaW5mbygKICAgICAgICBtb2R1bGVfcGF0aCwKICAgICAgICBmaWxlX3BhdGgsCiAgICAgICAgdmlzaWJpbGl0eSwKICAgICAgICAmZmlsZV9pbmZvLnB1YmxpY19pdGVtcywKICAgICAgICAmZmlsZV9pbmZvLmltcG9ydHMsCiAgICAgICAgJmZpbGVfaW5mby5yZV9leHBvcnRzLAogICAgICAgICZmaWxlX2luZm8uc3VibW9kdWxlcywKICAgICldOwoKICAgIGZvciBzdWIgaW4gJmZpbGVfaW5mby5zdWJtb2R1bGVzIHsKICAgICAgICBpZiBzdWIuaXNfdGVzdCB7CiAgICAgICAgICAgIGNvbnRpbnVlOwogICAgICAgIH0KICAgICAgICBsZXQgY2hpbGRfcGF0aCA9IGZvcm1hdCEoInt9Ojp7fSIsIG1vZHVsZV9wYXRoLCBzdWIubmFtZSk7CiAgICAgICAgbGV0IGNoaWxkX2RpciA9IGZpbGVfcGF0aC5wYXJlbnQoKS51bndyYXBfb3IoZmlsZV9wYXRoKTsKICAgICAgICBsZXQgKGNoaWxkX21vZHVsZXMsIGNoaWxkX2Vycm9ycykgPSBwcm9jZXNzX3N1Ym1vZHVsZSgKICAgICAgICAgICAgJmNoaWxkX3BhdGgsCiAgICAgICAgICAgICZzdWIubmFtZSwKICAgICAgICAgICAgaXRlbXMsCiAgICAgICAgICAgIGNoaWxkX2RpciwKICAgICAgICAgICAgZmlsZV9wYXRoLAogICAgICAgICAgICB2aXNpdGVkLAogICAgICAgICk7CiAgICAgICAgZXJyb3JzLmV4dGVuZChjaGlsZF9lcnJvcnMpOwogICAgICAgIG1vZHVsZXMuZXh0ZW5kKGNoaWxkX21vZHVsZXMpOwogICAgfQoKICAgIChtb2R1bGVzLCBlcnJvcnMuY2xvbmUoKSkKfQ==", "target": "src/module_tree.rs", "index": 11, "is_create": false}]')
-
-for step in STEPS:
-    before = base64.b64decode(step["before_b64"]).decode()
-    after = base64.b64decode(step["after_b64"]).decode()
-    target = step["target"]
-    idx = step["index"]
-    is_create = step["is_create"]
-
-    if is_create:
-        target_path = Path(target)
-        target_path.parent.mkdir(parents=True, exist_ok=True)
-        target_path.write_text(after)
-        print(f"OK {TASK_ID} change {idx}: created {target}")
-    else:
-        target_path = Path(target)
-        content = target_path.read_text()
-        if before not in content:
-            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
-            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
-            sys.exit(1)
-
-        result = subprocess.run(
-            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
-            capture_output=True, text=True,
-        )
-        if result.returncode != 0:
-            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
-            sys.exit(result.returncode)
-
-        new_content = target_path.read_text()
-        if after and after not in new_content:
-            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
-            sys.exit(1)
-
-        print(f"OK {TASK_ID} change {idx}: applied to {target}")
-
-print(f"OK {TASK_ID}: all changes applied")
diff --git a/plans/compiled/TASK-5.sh b/plans/compiled/TASK-5.sh
deleted file mode 100755
index 186cdd0..0000000
--- a/plans/compiled/TASK-5.sh
+++ /dev/null
@@ -1,7 +0,0 @@
-#!/usr/bin/env bash
-set -euo pipefail
-# TASK-5: Refactor module_tree.rs: fix path fallbacks, add error collection, change build_module_tree return type
-# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
-# Type: replace
-# File: src/module_tree.rs
-python3 "$(dirname "$0")/TASK-5.py"
diff --git a/plans/compiled/TASK-6.py b/plans/compiled/TASK-6.py
deleted file mode 100644
index 8030cb1..0000000
--- a/plans/compiled/TASK-6.py
+++ /dev/null
@@ -1,44 +0,0 @@
-#!/usr/bin/env python3
-"""TASK-6: Update workspace.rs enumerate_members to return MissingWorkspaceSection error"""
-import base64, json, subprocess, sys
-from pathlib import Path
-
-TASK_ID = "TASK-6"
-STEPS = json.loads('[{"before_b64": "ICAgIGxldCBtZW1iZXJzOiBWZWM8U3RyaW5nPiA9IHBhcnNlZAogICAgICAgIC5nZXQoIndvcmtzcGFjZSIpCiAgICAgICAgLmFuZF90aGVuKHx3fCB3LmdldCgibWVtYmVycyIpKQogICAgICAgIC5hbmRfdGhlbih8bXwgbS5hc19hcnJheSgpKQogICAgICAgIC5tYXAofGFycnwgewogICAgICAgICAgICBhcnIuaXRlcigpCiAgICAgICAgICAgICAgICAuZmlsdGVyX21hcCh8dnwgdi5hc19zdHIoKS5tYXAoU3RyaW5nOjpmcm9tKSkKICAgICAgICAgICAgICAgIC5jb2xsZWN0KCkKICAgICAgICB9KQogICAgICAgIC51bndyYXBfb3JfZGVmYXVsdCgpOw==", "after_b64": "ICAgIGxldCBtZW1iZXJzOiBWZWM8U3RyaW5nPiA9IG1hdGNoIHBhcnNlZC5nZXQoIndvcmtzcGFjZSIpIHsKICAgICAgICBOb25lID0+IHJldHVybiBFcnIoRXJyb3I6Ok1pc3NpbmdXb3Jrc3BhY2VTZWN0aW9uKSwKICAgICAgICBTb21lKHdvcmtzcGFjZSkgPT4gd29ya3NwYWNlCiAgICAgICAgICAgIC5nZXQoIm1lbWJlcnMiKQogICAgICAgICAgICAuYW5kX3RoZW4ofG18IG0uYXNfYXJyYXkoKSkKICAgICAgICAgICAgLm1hcCh8YXJyfCB7CiAgICAgICAgICAgICAgICBhcnIuaXRlcigpCiAgICAgICAgICAgICAgICAgICAgLmZpbHRlcl9tYXAofHZ8IHYuYXNfc3RyKCkubWFwKFN0cmluZzo6ZnJvbSkpCiAgICAgICAgICAgICAgICAgICAgLmNvbGxlY3Q6OjxWZWM8Xz4+KCkKICAgICAgICAgICAgfSkKICAgICAgICAgICAgLnVud3JhcF9vcl9kZWZhdWx0KCksCiAgICB9Ow==", "target": "src/workspace.rs", "index": 0, "is_create": false}, {"before_b64": "Ly8vIFdhbGsgdXAgdGhlIGRpcmVjdG9yeSB0cmVlIGZyb20gYHN0YXJ0X3BhdGhgIHRvIGZpbmQgYSBgQ2FyZ28udG9tbGAKLy8vIGNvbnRhaW5pbmcgYSBgW3dvcmtzcGFjZV1gIHNlY3Rpb24uIFJldHVybnMgdGhlIGRpcmVjdG9yeSBjb250YWluaW5nIGl0LgpwdWIgZm4gZmluZF93b3Jrc3BhY2Vfcm9vdChzdGFydF9wYXRoOiAmUGF0aCkgLT4gUmVzdWx0PFBhdGhCdWY+IHs=", "after_b64": "Ly8vIFdhbGsgdXAgdGhlIGRpcmVjdG9yeSB0cmVlIGZyb20gYHN0YXJ0X3BhdGhgIHRvIGZpbmQgYSBgQ2FyZ28udG9tbGAKLy8vIGNvbnRhaW5pbmcgYSBgW3dvcmtzcGFjZV1gIHNlY3Rpb24uIFJldHVybnMgdGhlIGRpcmVjdG9yeSBjb250YWluaW5nIGl0LgovLy8KLy8vICMgRXJyb3JzCi8vLwovLy8gUmV0dXJucyBgRXJyb3I6OldvcmtzcGFjZVJvb3ROb3RGb3VuZGAgaWYgbm8gYENhcmdvLnRvbWxgIHdpdGggYQovLy8gYFt3b3Jrc3BhY2VdYCBzZWN0aW9uIGlzIGZvdW5kIGluIGFueSBhbmNlc3RvciBkaXJlY3RvcnkuCnB1YiBmbiBmaW5kX3dvcmtzcGFjZV9yb290KHN0YXJ0X3BhdGg6ICZQYXRoKSAtPiBSZXN1bHQ8UGF0aEJ1Zj4gew==", "target": "src/workspace.rs", "index": 1, "is_create": false}, {"before_b64": "Ly8vIFBhcnNlIHRoZSB3b3Jrc3BhY2UgYENhcmdvLnRvbWxgLCByZXNvbHZlIG1lbWJlciBwYXRocyAoaW5jbHVkaW5nIGdsb2IKLy8vIHBhdHRlcm5zKSwgYXBwbHkgYGV4Y2x1ZGVgIGxpc3QsIGFuZCByZXR1cm4gYWJzb2x1dGUgcGF0aHMgdG8gZWFjaCBtZW1iZXIKLy8vIGNyYXRlIGRpcmVjdG9yeS4KcHViIGZuIGVudW1lcmF0ZV9tZW1iZXJzKHJvb3Q6ICZQYXRoKSAtPiBSZXN1bHQ8VmVjPFBhdGhCdWY+PiB7", "after_b64": "Ly8vIFBhcnNlIHRoZSB3b3Jrc3BhY2UgYENhcmdvLnRvbWxgLCByZXNvbHZlIG1lbWJlciBwYXRocyAoaW5jbHVkaW5nIGdsb2IKLy8vIHBhdHRlcm5zKSwgYXBwbHkgYGV4Y2x1ZGVgIGxpc3QsIGFuZCByZXR1cm4gYWJzb2x1dGUgcGF0aHMgdG8gZWFjaCBtZW1iZXIKLy8vIGNyYXRlIGRpcmVjdG9yeS4KLy8vCi8vLyAjIEVycm9ycwovLy8KLy8vIFJldHVybnMgYEVycm9yOjpNaXNzaW5nV29ya3NwYWNlU2VjdGlvbmAgaWYgdGhlIGBDYXJnby50b21sYCBsYWNrcyBhCi8vLyBgW3dvcmtzcGFjZV1gIHNlY3Rpb24gZW50aXJlbHkuCnB1YiBmbiBlbnVtZXJhdGVfbWVtYmVycyhyb290OiAmUGF0aCkgLT4gUmVzdWx0PFZlYzxQYXRoQnVmPj4gew==", "target": "src/workspace.rs", "index": 2, "is_create": false}]')
-
-for step in STEPS:
-    before = base64.b64decode(step["before_b64"]).decode()
-    after = base64.b64decode(step["after_b64"]).decode()
-    target = step["target"]
-    idx = step["index"]
-    is_create = step["is_create"]
-
-    if is_create:
-        target_path = Path(target)
-        target_path.parent.mkdir(parents=True, exist_ok=True)
-        target_path.write_text(after)
-        print(f"OK {TASK_ID} change {idx}: created {target}")
-    else:
-        target_path = Path(target)
-        content = target_path.read_text()
-        if before not in content:
-            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
-            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
-            sys.exit(1)
-
-        result = subprocess.run(
-            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
-            capture_output=True, text=True,
-        )
-        if result.returncode != 0:
-            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
-            sys.exit(result.returncode)
-
-        new_content = target_path.read_text()
-        if after and after not in new_content:
-            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
-            sys.exit(1)
-
-        print(f"OK {TASK_ID} change {idx}: applied to {target}")
-
-print(f"OK {TASK_ID}: all changes applied")
diff --git a/plans/compiled/TASK-6.sh b/plans/compiled/TASK-6.sh
deleted file mode 100755
index 2a89fc7..0000000
--- a/plans/compiled/TASK-6.sh
+++ /dev/null
@@ -1,7 +0,0 @@
-#!/usr/bin/env bash
-set -euo pipefail
-# TASK-6: Update workspace.rs enumerate_members to return MissingWorkspaceSection error
-# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
-# Type: replace
-# File: src/workspace.rs
-python3 "$(dirname "$0")/TASK-6.py"
diff --git a/plans/compiled/TASK-7.py b/plans/compiled/TASK-7.py
deleted file mode 100644
index 5f61e22..0000000
--- a/plans/compiled/TASK-7.py
+++ /dev/null
@@ -1,44 +0,0 @@
-#!/usr/bin/env python3
-"""TASK-7: Refactor lib.rs run(): parallel error collection, error entry construction, remove unwrap_or_default"""
-import base64, json, subprocess, sys
-from pathlib import Path
-
-TASK_ID = "TASK-7"
-STEPS = json.loads('[{"before_b64": "dXNlIGFueWhvdzo6Q29udGV4dDsKdXNlIHJheW9uOjpwcmVsdWRlOjoqOwp1c2Ugc2NoZW1hOjp7CiAgICBDcmF0ZUluZm8sIENyYXRlVHlwZSwgRXJyb3JFbnRyeSwgTW9kdWxlSW5mbywgV29ya3NwYWNlSW5mbywgV29ya3NwYWNlTWFwLAp9Owp1c2Ugc3RkOjpwYXRoOjpQYXRoOw==", "after_b64": "dXNlIGFueWhvdzo6Q29udGV4dDsKdXNlIHJheW9uOjpwcmVsdWRlOjoqOwp1c2Ugc2NoZW1hOjp7CiAgICBDcmF0ZUluZm8sIENyYXRlVHlwZSwgRXJyb3JFbnRyeSwgRXJyb3JTZXZlcml0eSwgTW9kdWxlSW5mbywgV29ya3NwYWNlSW5mbywKICAgIFdvcmtzcGFjZU1hcCwKfTsKdXNlIHN0ZDo6cGF0aDo6UGF0aDs=", "target": "src/lib.rs", "index": 0, "is_create": false}, {"before_b64": "ICAgIGxldCBlcnJvcnM6IFZlYzxFcnJvckVudHJ5PiA9IFZlYzo6bmV3KCk7CgogICAgbGV0IG11dCBjcmF0ZV9pbmZvczogVmVjPENyYXRlSW5mbz4gPSBtZW1iZXJfZGlycwogICAgICAgIC5wYXJfaXRlcigpCiAgICAgICAgLmZpbHRlcl9tYXAofGRpcnwgewogICAgICAgICAgICBsZXQgY2FyZ29fdG9tbCA9IGRpci5qb2luKCJDYXJnby50b21sIik7CgogICAgICAgICAgICBsZXQgKHBrZywgZGVwcykgPSBtYXRjaCBjYXJnb19pbmZvOjpwYXJzZV9jYXJnb190b21sKCZjYXJnb190b21sKSB7CiAgICAgICAgICAgICAgICBPayh2KSA9PiB2LAogICAgICAgICAgICAgICAgRXJyKGUpID0+IHsKICAgICAgICAgICAgICAgICAgICBlcHJpbnRsbiEoCiAgICAgICAgICAgICAgICAgICAgICAgICJ3YXJuaW5nOiBmYWlsZWQgdG8gcGFyc2Uge306IHt9IiwKICAgICAgICAgICAgICAgICAgICAgICAgY2FyZ29fdG9tbC5kaXNwbGF5KCksCiAgICAgICAgICAgICAgICAgICAgICAgIGUKICAgICAgICAgICAgICAgICAgICApOwogICAgICAgICAgICAgICAgICAgIHJldHVybiBOb25lOwogICAgICAgICAgICAgICAgfQogICAgICAgICAgICB9OwoKICAgICAgICAgICAgbGV0IHJvb3RzID0gd29ya3NwYWNlOjpyZXNvbHZlX2NyYXRlX3Jvb3RzKGRpcik7CiAgICAgICAgICAgIGlmIHJvb3RzLmlzX2VtcHR5KCkgewogICAgICAgICAgICAgICAgZXByaW50bG4hKAogICAgICAgICAgICAgICAgICAgICJ3YXJuaW5nOiBubyBjcmF0ZSBlbnRyeSBwb2ludHMgZm91bmQgaW4ge30iLAogICAgICAgICAgICAgICAgICAgIGRpci5kaXNwbGF5KCkKICAgICAgICAgICAgICAgICk7CiAgICAgICAgICAgICAgICByZXR1cm4gTm9uZTsKICAgICAgICAgICAgfQoKICAgICAgICAgICAgbGV0IGNyYXRlX3R5cGUgPSBpZiByb290cy5pdGVyKCkuYW55KHwoXywgdCl8ICp0ID09IENyYXRlVHlwZTo6TGliKQogICAgICAgICAgICAgICAgJiYgcm9vdHMuaXRlcigpLmFueSh8KF8sIHQpfCAqdCA9PSBDcmF0ZVR5cGU6OkJpbikKICAgICAgICAgICAgewogICAgICAgICAgICAgICAgQ3JhdGVUeXBlOjpMaWJBbmRCaW4KICAgICAgICAgICAgfSBlbHNlIHsKICAgICAgICAgICAgICAgIHJvb3RzLmZpcnN0KCkubWFwX29yKENyYXRlVHlwZTo6TGliLCB8KF8sIHQpfCAqdCkKICAgICAgICAgICAgfTsKCiAgICAgICAgICAgIGxldCBwa2dfbmFtZSA9IHBrZy5uYW1lLmNsb25lKCk7CiAgICAgICAgICAgIGxldCBtdXQgbW9kdWxlczogVmVjPE1vZHVsZUluZm8+ID0gcm9vdHMKICAgICAgICAgICAgICAgIC5pdGVyKCkKICAgICAgICAgICAgICAgIC5mbGF0X21hcCh8KHJvb3QsIF90eSl8IHsKICAgICAgICAgICAgICAgICAgICBtb2R1bGVfdHJlZTo6YnVpbGRfbW9kdWxlX3RyZWUocm9vdCwgJnBrZ19uYW1lKS51bndyYXBfb3JfZGVmYXVsdCgpCiAgICAgICAgICAgICAgICB9KQogICAgICAgICAgICAgICAgLmNvbGxlY3QoKTsKCiAgICAgICAgICAgIC8vIFJlbGF0aXZpemUgYWxsIHBhdGhzIHRvIHRoZSB3b3Jrc3BhY2Ugcm9vdC4KICAgICAgICAgICAgZm9yIG0gaW4gJm11dCBtb2R1bGVzIHsKICAgICAgICAgICAgICAgIG0uZmlsZSA9IHJlbGF0aXZpemVfcGF0aCgmbS5maWxlLCAmd29ya3NwYWNlX3Jvb3QpOwogICAgICAgICAgICAgICAgZm9yIGl0ZW0gaW4gJm11dCBtLnB1YmxpY19pdGVtcyB7CiAgICAgICAgICAgICAgICAgICAgaXRlbS5maWxlID0gcmVsYXRpdml6ZV9wYXRoKCZpdGVtLmZpbGUsICZ3b3Jrc3BhY2Vfcm9vdCk7CiAgICAgICAgICAgICAgICB9CiAgICAgICAgICAgIH0KCiAgICAgICAgICAgIGxldCBjcmF0ZV9yb290ID0gcm9vdHMKICAgICAgICAgICAgICAgIC5maXJzdCgpCiAgICAgICAgICAgICAgICAubWFwKHwociwgXyl8IHJlbGF0aXZpemVfcGF0aCgmci50b19zdHJpbmdfbG9zc3koKSwgJndvcmtzcGFjZV9yb290KSkKICAgICAgICAgICAgICAgIC51bndyYXBfb3JfZGVmYXVsdCgpOwoKICAgICAgICAgICAgbGV0IHJlYnVpbHRfcGtnID0gc2NoZW1hOjpQYWNrYWdlSW5mbzo6YnVpbGRlcigpCiAgICAgICAgICAgICAgICAubmFtZShwa2cubmFtZSkKICAgICAgICAgICAgICAgIC52ZXJzaW9uKHBrZy52ZXJzaW9uKQogICAgICAgICAgICAgICAgLmVkaXRpb24ocGtnLmVkaXRpb24pCiAgICAgICAgICAgICAgICAuY3JhdGVfdHlwZShjcmF0ZV90eXBlKQogICAgICAgICAgICAgICAgLmJ1aWxkKCk7CgogICAgICAgICAgICBTb21lKAogICAgICAgICAgICAgICAgQ3JhdGVJbmZvOjpidWlsZGVyKCkKICAgICAgICAgICAgICAgICAgICAubmFtZShwa2dfbmFtZSkKICAgICAgICAgICAgICAgICAgICAucm9vdChjcmF0ZV9yb290KQogICAgICAgICAgICAgICAgICAgIC5wYWNrYWdlKHJlYnVpbHRfcGtnKQogICAgICAgICAgICAgICAgICAgIC5tb2R1bGVzKG1vZHVsZXMpCiAgICAgICAgICAgICAgICAgICAgLmRlcHMoZGVwcykKICAgICAgICAgICAgICAgICAgICAuYnVpbGQoKSwKICAgICAgICAgICAgKQogICAgICAgIH0pCiAgICAgICAgLmNvbGxlY3QoKTs=", "after_b64": "ICAgIGxldCBtdXQgY3JhdGVfZXJyb3JzOiBWZWM8RXJyb3JFbnRyeT4gPSBWZWM6Om5ldygpOwoKICAgIGxldCByZXN1bHRzOiBWZWM8KENyYXRlSW5mbywgVmVjPEVycm9yRW50cnk+KT4gPSBtZW1iZXJfZGlycwogICAgICAgIC5wYXJfaXRlcigpCiAgICAgICAgLm1hcCh8ZGlyfCB7CiAgICAgICAgICAgIGxldCBjYXJnb190b21sID0gZGlyLmpvaW4oIkNhcmdvLnRvbWwiKTsKICAgICAgICAgICAgbGV0IG11dCBjcmF0ZV9lcnJvcnMgPSBWZWM6Om5ldygpOwoKICAgICAgICAgICAgbGV0IChwa2csIGRlcHMpID0gbWF0Y2ggY2FyZ29faW5mbzo6cGFyc2VfY2FyZ29fdG9tbCgmY2FyZ29fdG9tbCkgewogICAgICAgICAgICAgICAgT2sodikgPT4gdiwKICAgICAgICAgICAgICAgIEVycihlKSA9PiB7CiAgICAgICAgICAgICAgICAgICAgY3JhdGVfZXJyb3JzLnB1c2goRXJyb3JFbnRyeTo6YnVpbGRlcigpCiAgICAgICAgICAgICAgICAgICAgICAgIC5maWxlKGNhcmdvX3RvbWwudG9fc3RyaW5nX2xvc3N5KCkudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAgICAgICAgIC5tZXNzYWdlKGZvcm1hdCEoImZhaWxlZCB0byBwYXJzZSBDYXJnby50b21sOiB7ZX0iKSkKICAgICAgICAgICAgICAgICAgICAgICAgLnNldmVyaXR5KEVycm9yU2V2ZXJpdHk6OkVycm9yKQogICAgICAgICAgICAgICAgICAgICAgICAua2luZCgidG9tbF9wYXJzZV9lcnJvciIudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAgICAgICAgIC5jYXVzZShlLnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgICAgICAgICAuYnVpbGQoKSk7CiAgICAgICAgICAgICAgICAgICAgcmV0dXJuIChOb25lLCBjcmF0ZV9lcnJvcnMpOwogICAgICAgICAgICAgICAgfQogICAgICAgICAgICB9OwoKICAgICAgICAgICAgbGV0IHJvb3RzID0gd29ya3NwYWNlOjpyZXNvbHZlX2NyYXRlX3Jvb3RzKGRpcik7CiAgICAgICAgICAgIGlmIHJvb3RzLmlzX2VtcHR5KCkgewogICAgICAgICAgICAgICAgY3JhdGVfZXJyb3JzLnB1c2goRXJyb3JFbnRyeTo6YnVpbGRlcigpCiAgICAgICAgICAgICAgICAgICAgLmZpbGUoZGlyLnRvX3N0cmluZ19sb3NzeSgpLnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgICAgIC5tZXNzYWdlKCJubyBjcmF0ZSBlbnRyeSBwb2ludHMgZm91bmQiLnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgICAgIC5zZXZlcml0eShFcnJvclNldmVyaXR5OjpXYXJuaW5nKQogICAgICAgICAgICAgICAgICAgIC5raW5kKCJtaXNzaW5nX2NyYXRlX3Jvb3RzIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgICAgICAgICAuYnVpbGQoKSk7CiAgICAgICAgICAgICAgICByZXR1cm4gKE5vbmUsIGNyYXRlX2Vycm9ycyk7CiAgICAgICAgICAgIH0KCiAgICAgICAgICAgIGxldCBjcmF0ZV90eXBlID0gaWYgcm9vdHMuaXRlcigpLmFueSh8KF8sIHQpfCAqdCA9PSBDcmF0ZVR5cGU6OkxpYikKICAgICAgICAgICAgICAgICYmIHJvb3RzLml0ZXIoKS5hbnkofChfLCB0KXwgKnQgPT0gQ3JhdGVUeXBlOjpCaW4pCiAgICAgICAgICAgIHsKICAgICAgICAgICAgICAgIENyYXRlVHlwZTo6TGliQW5kQmluCiAgICAgICAgICAgIH0gZWxzZSB7CiAgICAgICAgICAgICAgICByb290cy5maXJzdCgpLm1hcF9vcihDcmF0ZVR5cGU6OkxpYiwgfChfLCB0KXwgKnQpCiAgICAgICAgICAgIH07CgogICAgICAgICAgICBsZXQgcGtnX25hbWUgPSBwa2cubmFtZS5jbG9uZSgpOwogICAgICAgICAgICBsZXQgbXV0IG1vZHVsZXM6IFZlYzxNb2R1bGVJbmZvPiA9IFZlYzo6bmV3KCk7CiAgICAgICAgICAgIGxldCBtdXQgY29sbGVjdGVkX2Vycm9ycyA9IFZlYzo6bmV3KCk7CiAgICAgICAgICAgIGZvciAocm9vdCwgX3R5KSBpbiAmcm9vdHMgewogICAgICAgICAgICAgICAgbGV0IChtLCBlKSA9IG1vZHVsZV90cmVlOjpidWlsZF9tb2R1bGVfdHJlZSgmcm9vdCwgJnBrZ19uYW1lKTsKICAgICAgICAgICAgICAgIG1vZHVsZXMuZXh0ZW5kKG0pOwogICAgICAgICAgICAgICAgY29sbGVjdGVkX2Vycm9ycy5leHRlbmQoZSk7CiAgICAgICAgICAgIH0KICAgICAgICAgICAgY3JhdGVfZXJyb3JzLmV4dGVuZChjb2xsZWN0ZWRfZXJyb3JzKTsKCiAgICAgICAgICAgIC8vIFJlbGF0aXZpemUgYWxsIHBhdGhzIHRvIHRoZSB3b3Jrc3BhY2Ugcm9vdC4KICAgICAgICAgICAgZm9yIG0gaW4gJm11dCBtb2R1bGVzIHsKICAgICAgICAgICAgICAgIG0uZmlsZSA9IHJlbGF0aXZpemVfcGF0aCgmbS5maWxlLCAmd29ya3NwYWNlX3Jvb3QpOwogICAgICAgICAgICAgICAgZm9yIGl0ZW0gaW4gJm11dCBtLnB1YmxpY19pdGVtcyB7CiAgICAgICAgICAgICAgICAgICAgaXRlbS5maWxlID0gcmVsYXRpdml6ZV9wYXRoKCZpdGVtLmZpbGUsICZ3b3Jrc3BhY2Vfcm9vdCk7CiAgICAgICAgICAgICAgICB9CiAgICAgICAgICAgIH0KCiAgICAgICAgICAgIGxldCBjcmF0ZV9yb290ID0gcm9vdHMKICAgICAgICAgICAgICAgIC5maXJzdCgpCiAgICAgICAgICAgICAgICAubWFwKHwociwgXyl8IHJlbGF0aXZpemVfcGF0aCgmci50b19zdHJpbmdfbG9zc3koKSwgJndvcmtzcGFjZV9yb290KSkKICAgICAgICAgICAgICAgIC51bndyYXBfb3JfZGVmYXVsdCgpOwoKICAgICAgICAgICAgbGV0IHJlYnVpbHRfcGtnID0gc2NoZW1hOjpQYWNrYWdlSW5mbzo6YnVpbGRlcigpCiAgICAgICAgICAgICAgICAubmFtZShwa2cubmFtZSkKICAgICAgICAgICAgICAgIC52ZXJzaW9uKHBrZy52ZXJzaW9uKQogICAgICAgICAgICAgICAgLmVkaXRpb24ocGtnLmVkaXRpb24pCiAgICAgICAgICAgICAgICAuY3JhdGVfdHlwZShjcmF0ZV90eXBlKQogICAgICAgICAgICAgICAgLmJ1aWxkKCk7CgogICAgICAgICAgICBsZXQgY3JhdGVfaW5mbyA9IENyYXRlSW5mbzo6YnVpbGRlcigpCiAgICAgICAgICAgICAgICAubmFtZShwa2dfbmFtZSkKICAgICAgICAgICAgICAgIC5yb290KGNyYXRlX3Jvb3QpCiAgICAgICAgICAgICAgICAucGFja2FnZShyZWJ1aWx0X3BrZykKICAgICAgICAgICAgICAgIC5tb2R1bGVzKG1vZHVsZXMpCiAgICAgICAgICAgICAgICAuZGVwcyhkZXBzKQogICAgICAgICAgICAgICAgLmJ1aWxkKCk7CgogICAgICAgICAgICAoU29tZShjcmF0ZV9pbmZvKSwgY3JhdGVfZXJyb3JzKQogICAgICAgIH0pCiAgICAgICAgLmNvbGxlY3QoKTsKCiAgICBsZXQgbXV0IGNyYXRlX2luZm9zOiBWZWM8Q3JhdGVJbmZvPiA9IFZlYzo6bmV3KCk7CgogICAgZm9yIChpbmZvLCBlcnJzKSBpbiByZXN1bHRzIHsKICAgICAgICBpZiBsZXQgU29tZShjaSkgPSBpbmZvIHsKICAgICAgICAgICAgY3JhdGVfZXJyb3JzLmV4dGVuZChlcnJzKTsKICAgICAgICAgICAgY3JhdGVfaW5mb3MucHVzaChjaSk7CiAgICAgICAgfQogICAgfQ==", "target": "src/lib.rs", "index": 1, "is_create": false}, {"before_b64": "ICAgIGxldCBtYXAgPSBXb3Jrc3BhY2VNYXA6OmJ1aWxkZXIoKQogICAgICAgIC53b3Jrc3BhY2Uod29ya3NwYWNlX2luZm8pCiAgICAgICAgLmNyYXRlcyhjcmF0ZV9pbmZvcykKICAgICAgICAuY3Jvc3NfcmVmZXJlbmNlcyhjcm9zc19yZWZzKQogICAgICAgIC5lcnJvcnMoZXJyb3JzKQogICAgICAgIC53b3Jrc3BhY2Vfcm9vdCh3b3Jrc3BhY2Vfcm9vdC5jbG9uZSgpKQogICAgICAgIC5idWlsZCgpOw==", "after_b64": "ICAgIGxldCBtYXAgPSBXb3Jrc3BhY2VNYXA6OmJ1aWxkZXIoKQogICAgICAgIC53b3Jrc3BhY2Uod29ya3NwYWNlX2luZm8pCiAgICAgICAgLmNyYXRlcyhjcmF0ZV9pbmZvcykKICAgICAgICAuY3Jvc3NfcmVmZXJlbmNlcyhjcm9zc19yZWZzKQogICAgICAgIC5lcnJvcnMoY3JhdGVfZXJyb3JzKQogICAgICAgIC53b3Jrc3BhY2Vfcm9vdCh3b3Jrc3BhY2Vfcm9vdC5jbG9uZSgpKQogICAgICAgIC5idWlsZCgpOw==", "target": "src/lib.rs", "index": 2, "is_create": false}]')
-
-for step in STEPS:
-    before = base64.b64decode(step["before_b64"]).decode()
-    after = base64.b64decode(step["after_b64"]).decode()
-    target = step["target"]
-    idx = step["index"]
-    is_create = step["is_create"]
-
-    if is_create:
-        target_path = Path(target)
-        target_path.parent.mkdir(parents=True, exist_ok=True)
-        target_path.write_text(after)
-        print(f"OK {TASK_ID} change {idx}: created {target}")
-    else:
-        target_path = Path(target)
-        content = target_path.read_text()
-        if before not in content:
-            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
-            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
-            sys.exit(1)
-
-        result = subprocess.run(
-            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
-            capture_output=True, text=True,
-        )
-        if result.returncode != 0:
-            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
-            sys.exit(result.returncode)
-
-        new_content = target_path.read_text()
-        if after and after not in new_content:
-            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
-            sys.exit(1)
-
-        print(f"OK {TASK_ID} change {idx}: applied to {target}")
-
-print(f"OK {TASK_ID}: all changes applied")
diff --git a/plans/compiled/TASK-7.sh b/plans/compiled/TASK-7.sh
deleted file mode 100755
index 35337cf..0000000
--- a/plans/compiled/TASK-7.sh
+++ /dev/null
@@ -1,7 +0,0 @@
-#!/usr/bin/env bash
-set -euo pipefail
-# TASK-7: Refactor lib.rs run(): parallel error collection, error entry construction, remove unwrap_or_default
-# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
-# Type: replace
-# File: src/lib.rs
-python3 "$(dirname "$0")/TASK-7.py"
diff --git a/plans/compiled/TASK-8.py b/plans/compiled/TASK-8.py
deleted file mode 100644
index 52d3e99..0000000
--- a/plans/compiled/TASK-8.py
+++ /dev/null
@@ -1,44 +0,0 @@
-#!/usr/bin/env python3
-"""TASK-8: Remove all crate-level clippy allow attributes and fix individual lint violations"""
-import base64, json, subprocess, sys
-from pathlib import Path
-
-TASK_ID = "TASK-8"
-STEPS = json.loads('[{"before_b64": "IyFbd2FybihjbGlwcHk6OnBlZGFudGljKV0KIyFbYWxsb3coY2xpcHB5OjptaXNzaW5nX2Vycm9yc19kb2MpXQojIVthbGxvdyhjbGlwcHk6Om11c3RfdXNlX2NhbmRpZGF0ZSldCiMhW2FsbG93KGNsaXBweTo6ZG9jX21hcmtkb3duKV0KIyFbYWxsb3coY2xpcHB5Ojp1bmlubGluZWRfZm9ybWF0X2FyZ3MpXQojIVthbGxvdyhjbGlwcHk6OnJlZHVuZGFudF9jbG9zdXJlKV0KIyFbYWxsb3coY2xpcHB5Ojpjb2xsYXBzaWJsZV9pZildCiMhW2FsbG93KGNsaXBweTo6bmVlZGxlc3NfcGFzc19ieV92YWx1ZSldCiMhW2FsbG93KGNsaXBweTo6bmVlZGxlc3NfYm9ycm93KV0KIyFbYWxsb3coY2xpcHB5OjpyZWR1bmRhbnRfY2xvc3VyZV9mb3JfbWV0aG9kX2NhbGxzKV0=", "after_b64": "IyFbd2FybihjbGlwcHk6OnBlZGFudGljKV0=", "target": "src/lib.rs", "index": 0, "is_create": false}, {"before_b64": "Ly8vIFJ1biB0aGUgZnVsbCB3b3Jrc3BhY2UgbWFwcGluZyBwaXBlbGluZS4KLy8vCi8vLyAxLiBEaXNjb3ZlciB3b3Jrc3BhY2Ugcm9vdCBhbmQgbWVtYmVyIGNyYXRlcy4KLy8vIDIuIFByb2Nlc3MgZWFjaCBjcmF0ZSBpbiBwYXJhbGxlbCAoQ2FyZ28udG9tbCBwYXJzaW5nICsgbW9kdWxlIHRyZWUpLgovLy8gMy4gQ29tcHV0ZSBjcm9zcy1jcmF0ZSByZWZlcmVuY2VzLgovLy8gNC4gUmVuZGVyIEpTT04gdG8gc3Rkb3V0IG9yIHRoZSBjb25maWd1cmVkIG91dHB1dCBmaWxlLgpwdWIgZm4gcnVuKGNvbmZpZzogQ29uZmlnKSAtPiBhbnlob3c6OlJlc3VsdDwoKT4gew==", "after_b64": "Ly8vIFJ1biB0aGUgZnVsbCB3b3Jrc3BhY2UgbWFwcGluZyBwaXBlbGluZS4KLy8vCi8vLyAxLiBEaXNjb3ZlciB3b3Jrc3BhY2Ugcm9vdCBhbmQgbWVtYmVyIGNyYXRlcy4KLy8vIDIuIFByb2Nlc3MgZWFjaCBjcmF0ZSBpbiBwYXJhbGxlbCAoQ2FyZ28udG9tbCBwYXJzaW5nICsgbW9kdWxlIHRyZWUpLgovLy8gMy4gQ29tcHV0ZSBjcm9zcy1jcmF0ZSByZWZlcmVuY2VzLgovLy8gNC4gUmVuZGVyIEpTT04gdG8gc3Rkb3V0IG9yIHRoZSBjb25maWd1cmVkIG91dHB1dCBmaWxlLgovLy8KLy8vICMgRXJyb3JzCi8vLwovLy8gUmV0dXJucyBhbiBlcnJvciBpZiB0aGUgd29ya3NwYWNlIHJvb3QgY2Fubm90IGJlIGZvdW5kLCB0aGUgd29ya3NwYWNlCi8vLyBDYXJnby50b21sIGlzIG1pc3NpbmcgYSBgW3dvcmtzcGFjZV1gIHNlY3Rpb24sIG1lbWJlciBjcmF0ZXMgY2Fubm90IGJlCi8vLyBwYXJzZWQsIG9yIHRoZSBKU09OIG91dHB1dCBjYW5ub3QgYmUgd3JpdHRlbi4KcHViIGZuIHJ1bihjb25maWc6IENvbmZpZykgLT4gYW55aG93OjpSZXN1bHQ8KCk+IHs=", "target": "src/lib.rs", "index": 1, "is_create": false}, {"before_b64": "ICAgIGxldCB3b3Jrc3BhY2VfbmFtZSA9IHdvcmtzcGFjZV9yb290CiAgICAgICAgLmZpbGVfbmFtZSgpCiAgICAgICAgLm1hcCh8bnwgbi50b19zdHJpbmdfbG9zc3koKS50b19zdHJpbmcoKSkKICAgICAgICAudW53cmFwX29yX2RlZmF1bHQoKTs=", "after_b64": "ICAgIGxldCB3b3Jrc3BhY2VfbmFtZSA9IHdvcmtzcGFjZV9yb290CiAgICAgICAgLmZpbGVfbmFtZSgpCiAgICAgICAgLm1hcCh8bnwgbi50b19zdHJpbmdfbG9zc3koKS50b19zdHJpbmcoKSkKICAgICAgICAudW53cmFwX29yX2RlZmF1bHQoKTs=", "target": "src/lib.rs", "index": 2, "is_create": false}, {"before_b64": "Ly8vIFN0cmlwIHRoZSB3b3Jrc3BhY2Ugcm9vdCBwcmVmaXggZnJvbSBhIHBhdGggc3RyaW5nLCByZXR1cm5pbmcgYQovLy8gd29ya3NwYWNlLXJlbGF0aXZlIHBhdGguIElmIHRoZSBwcmVmaXggZG9lc24ndCBtYXRjaCwgcmV0dXJucyB0aGUKLy8vIG9yaWdpbmFsIHN0cmluZyB1bmNoYW5nZWQuCmZuIHJlbGF0aXZpemVfcGF0aChwYXRoX3N0cjogJnN0ciwgcm9vdDogJlBhdGgpIC0+IFN0cmluZyB7CiAgICBsZXQgcCA9IFBhdGg6Om5ldyhwYXRoX3N0cik7CiAgICBtYXRjaCBwLnN0cmlwX3ByZWZpeChyb290KSB7CiAgICAgICAgT2socmVsKSA9PiByZWwudG9fc3RyaW5nX2xvc3N5KCkudG9fc3RyaW5nKCksCiAgICAgICAgRXJyKF8pID0+IHBhdGhfc3RyLnRvX3N0cmluZygpLAogICAgfQp9", "after_b64": "Ly8vIFN0cmlwIHRoZSB3b3Jrc3BhY2Ugcm9vdCBwcmVmaXggZnJvbSBhIHBhdGggc3RyaW5nLCByZXR1cm5pbmcgYQovLy8gd29ya3NwYWNlLXJlbGF0aXZlIHBhdGguIElmIHRoZSBwcmVmaXggZG9lc24ndCBtYXRjaCwgcmV0dXJucyB0aGUKLy8vIG9yaWdpbmFsIHN0cmluZyB1bmNoYW5nZWQuCmZuIHJlbGF0aXZpemVfcGF0aChwYXRoX3N0cjogJnN0ciwgcm9vdDogJlBhdGgpIC0+IFN0cmluZyB7CiAgICBsZXQgcCA9IFBhdGg6Om5ldyhwYXRoX3N0cik7CiAgICBtYXRjaCBwLnN0cmlwX3ByZWZpeChyb290KSB7CiAgICAgICAgT2socmVsKSA9PiByZWwudG9fc3RyaW5nX2xvc3N5KCkudG9fc3RyaW5nKCksCiAgICAgICAgRXJyKF8pID0+IHBhdGhfc3RyLnRvX3N0cmluZygpLAogICAgfQp9", "target": "src/lib.rs", "index": 3, "is_create": false}, {"before_b64": "Ly8vIEV4dHJhY3QgYWxsIGl0ZW1zIHdpdGggYW55IGZvcm0gb2YgYHB1YmAgdmlzaWJpbGl0eSAoZXhjbHVkaW5nIGBJbmhlcml0ZWRgKS4KLy8vIFJlc3VsdHMgYXJlIHNvcnRlZCBieSBuYW1lIHRoZW4gbGluZSBmb3IgZGV0ZXJtaW5pc3RpYyBvdXRwdXQuCnB1YiBmbiBleHRyYWN0X3B1YmxpY19pdGVtcyhpdGVtczogJltzeW46Okl0ZW1dKSAtPiBWZWM8UHVibGljSXRlbT4gew==", "after_b64": "Ly8vIEV4dHJhY3QgYWxsIGl0ZW1zIHdpdGggYW55IGZvcm0gb2YgYHB1YmAgdmlzaWJpbGl0eSAoZXhjbHVkaW5nIGBJbmhlcml0ZWRgKS4KLy8vIFJlc3VsdHMgYXJlIHNvcnRlZCBieSBuYW1lIHRoZW4gbGluZSBmb3IgZGV0ZXJtaW5pc3RpYyBvdXRwdXQuCiNbbXVzdF91c2VdCnB1YiBmbiBleHRyYWN0X3B1YmxpY19pdGVtcyhpdGVtczogJltzeW46Okl0ZW1dKSAtPiBWZWM8UHVibGljSXRlbT4gew==", "target": "src/file_parser.rs", "index": 4, "is_create": false}, {"before_b64": "Ly8vIEV4dHJhY3QgYWxsIGB1c2VgIHN0YXRlbWVudHMuIEJyYWNlZCBpbXBvcnRzIGFyZSBleHBhbmRlZCB0byBpbmRpdmlkdWFsCi8vLyBlbnRyaWVzLiBSZXN1bHRzIHNvcnRlZCBieSBwYXRoIGZvciBkZXRlcm1pbmlzbS4KcHViIGZuIGV4dHJhY3RfaW1wb3J0cyhpdGVtczogJltzeW46Okl0ZW1dKSAtPiBWZWM8SW1wb3J0PiB7", "after_b64": "Ly8vIEV4dHJhY3QgYWxsIGB1c2VgIHN0YXRlbWVudHMuIEJyYWNlZCBpbXBvcnRzIGFyZSBleHBhbmRlZCB0byBpbmRpdmlkdWFsCi8vLyBlbnRyaWVzLiBSZXN1bHRzIHNvcnRlZCBieSBwYXRoIGZvciBkZXRlcm1pbmlzbS4KI1ttdXN0X3VzZV0KcHViIGZuIGV4dHJhY3RfaW1wb3J0cyhpdGVtczogJltzeW46Okl0ZW1dKSAtPiBWZWM8SW1wb3J0PiB7", "target": "src/file_parser.rs", "index": 5, "is_create": false}, {"before_b64": "Ly8vIEV4dHJhY3QgYHB1YiB1c2VgIHJlLWV4cG9ydHMuIFJlc3VsdHMgc29ydGVkIGJ5IGV4cG9ydF9wYXRoLgpwdWIgZm4gZXh0cmFjdF9yZV9leHBvcnRzKGl0ZW1zOiAmW3N5bjo6SXRlbV0pIC0+IFZlYzxSZUV4cG9ydD4gew==", "after_b64": "Ly8vIEV4dHJhY3QgYHB1YiB1c2VgIHJlLWV4cG9ydHMuIFJlc3VsdHMgc29ydGVkIGJ5IGV4cG9ydF9wYXRoLgojW211c3RfdXNlXQpwdWIgZm4gZXh0cmFjdF9yZV9leHBvcnRzKGl0ZW1zOiAmW3N5bjo6SXRlbV0pIC0+IFZlYzxSZUV4cG9ydD4gew==", "target": "src/file_parser.rs", "index": 6, "is_create": false}, {"before_b64": "Ly8vIEV4dHJhY3QgYG1vZGAgZGVjbGFyYXRpb25zLiBEZXRlY3RzIGAjW2NmZyh0ZXN0KV1gIHZpYSBsaXRlcmFsIHRva2VuCi8vLyBtYXRjaGluZy4gUmVzdWx0cyBzb3J0ZWQgYnkgbmFtZS4KcHViIGZuIGV4dHJhY3Rfc3VibW9kdWxlcyhpdGVtczogJltzeW46Okl0ZW1dKSAtPiBWZWM8U3VibW9kdWxlRGVjbD4gew==", "after_b64": "Ly8vIEV4dHJhY3QgYG1vZGAgZGVjbGFyYXRpb25zLiBEZXRlY3RzIGAjW2NmZyh0ZXN0KV1gIHZpYSBsaXRlcmFsIHRva2VuCi8vLyBtYXRjaGluZy4gUmVzdWx0cyBzb3J0ZWQgYnkgbmFtZS4KI1ttdXN0X3VzZV0KcHViIGZuIGV4dHJhY3Rfc3VibW9kdWxlcyhpdGVtczogJltzeW46Okl0ZW1dKSAtPiBWZWM8U3VibW9kdWxlRGVjbD4gew==", "target": "src/file_parser.rs", "index": 7, "is_create": false}, {"before_b64": "Ly8vIEV4dHJhY3QgYGltcGxgIGJsb2Nrcy4gRWFjaCBgSW1wbEluZm9gIHJlY29yZHMgdGhlIHRhcmdldCB0eXBlIG5hbWUgYW5kCi8vLyB0aGUgaW1wbCBpdGVtcyAoZm4sIHR5cGUsIGNvbnN0KS4KcHViIGZuIGV4dHJhY3RfaW1wbHMoaXRlbXM6ICZbc3luOjpJdGVtXSkgLT4gVmVjPEltcGxJbmZvPiB7", "after_b64": "Ly8vIEV4dHJhY3QgYGltcGxgIGJsb2Nrcy4gRWFjaCBgSW1wbEluZm9gIHJlY29yZHMgdGhlIHRhcmdldCB0eXBlIG5hbWUgYW5kCi8vLyB0aGUgaW1wbCBpdGVtcyAoZm4sIHR5cGUsIGNvbnN0KS4KI1ttdXN0X3VzZV0KcHViIGZuIGV4dHJhY3RfaW1wbHMoaXRlbXM6ICZbc3luOjpJdGVtXSkgLT4gVmVjPEltcGxJbmZvPiB7", "target": "src/file_parser.rs", "index": 8, "is_create": false}, {"before_b64": "Ly8vIFJlc29sdmUgYSBgbW9kIG5hbWU7YCBkZWNsYXJhdGlvbiB0byBhIGZpbGUgcGF0aC4KLy8vIFRyaWVzIGB7cGFyZW50X2Rpcn0ve21vZF9uYW1lfS5yc2AgZmlyc3QsIHRoZW4gYHtwYXJlbnRfZGlyfS97bW9kX25hbWV9L21vZC5yc2AuCi8vLwovLy8gUmV0dXJucyBgTm9uZWAgaWYgbmVpdGhlciBwYXRoIGV4aXN0cy4KcHViIGZuIHJlc29sdmVfbW9kdWxlX3BhdGgocGFyZW50X2RpcjogJlBhdGgsIG1vZF9uYW1lOiAmc3RyKSAtPiBPcHRpb248UGF0aEJ1Zj4gew==", "after_b64": "Ly8vIFJlc29sdmUgYSBgbW9kIG5hbWU7YCBkZWNsYXJhdGlvbiB0byBhIGZpbGUgcGF0aC4KLy8vIFRyaWVzIGB7cGFyZW50X2Rpcn0ve21vZF9uYW1lfS5yc2AgZmlyc3QsIHRoZW4gYHtwYXJlbnRfZGlyfS97bW9kX25hbWV9L21vZC5yc2AuCi8vLwovLy8gUmV0dXJucyBgTm9uZWAgaWYgbmVpdGhlciBwYXRoIGV4aXN0cy4KI1ttdXN0X3VzZV0KcHViIGZuIHJlc29sdmVfbW9kdWxlX3BhdGgocGFyZW50X2RpcjogJlBhdGgsIG1vZF9uYW1lOiAmc3RyKSAtPiBPcHRpb248UGF0aEJ1Zj4gew==", "target": "src/module_tree.rs", "index": 9, "is_create": false}, {"before_b64": "Ly8vIFNlcmlhbGl6ZSB0aGUgd29ya3NwYWNlIG1hcCB0byBhIEpTT04gc3RyaW5nIHdpdGggMi1zcGFjZSBpbmRlbnRhdGlvbi4KcHViIGZuIHJlbmRlcl9qc29uKG1hcDogJldvcmtzcGFjZU1hcCkgLT4gc2VyZGVfanNvbjo6UmVzdWx0PFN0cmluZz4gew==", "after_b64": "Ly8vIFNlcmlhbGl6ZSB0aGUgd29ya3NwYWNlIG1hcCB0byBhIEpTT04gc3RyaW5nIHdpdGggMi1zcGFjZSBpbmRlbnRhdGlvbi4KI1ttdXN0X3VzZV0KcHViIGZuIHJlbmRlcl9qc29uKG1hcDogJldvcmtzcGFjZU1hcCkgLT4gc2VyZGVfanNvbjo6UmVzdWx0PFN0cmluZz4gew==", "target": "src/render.rs", "index": 10, "is_create": false}, {"before_b64": "Ly8gSGVscGVyOiBjb252ZXJ0IEl0ZW1LaW5kIHRvIGEgc2hvcnQgc3RyaW5nIGZvciB0aGUgVHlwZVJlZi5raW5kIGZpZWxkLgppbXBsIGNyYXRlOjpzY2hlbWE6OlB1YmxpY0l0ZW0gewogICAgZm4ga2luZF90b19zdHJpbmcoJnNlbGYpIC0+IFN0cmluZyB7", "after_b64": "Ly8gSGVscGVyOiBjb252ZXJ0IEl0ZW1LaW5kIHRvIGEgc2hvcnQgc3RyaW5nIGZvciB0aGUgVHlwZVJlZi5raW5kIGZpZWxkLgppbXBsIGNyYXRlOjpzY2hlbWE6OlB1YmxpY0l0ZW0gewogICAgI1ttdXN0X3VzZV0KICAgIGZuIGtpbmRfdG9fc3RyaW5nKCZzZWxmKSAtPiBTdHJpbmcgew==", "target": "src/cross_refs.rs", "index": 11, "is_create": false}, {"before_b64": "ICAgIG1hdGNoIHAuc3RyaXBfcHJlZml4KHJvb3QpIHsKICAgICAgICBPayhyZWwpID0+IHJlbC50b19zdHJpbmdfbG9zc3koKS50b19zdHJpbmcoKSwKICAgICAgICBFcnIoXykgPT4gcGF0aF9zdHIudG9fc3RyaW5nKCksCiAgICB9Cn0=", "after_b64": "ICAgIG1hdGNoIHAuc3RyaXBfcHJlZml4KHJvb3QpIHsKICAgICAgICBPayhyZWwpID0+IHJlbC50b19zdHJpbmdfbG9zc3koKS50b19zdHJpbmcoKSwKICAgICAgICBFcnIoXykgPT4gcGF0aF9zdHIudG9fc3RyaW5nKCksCiAgICB9Cn0=", "target": "src/lib.rs", "index": 12, "is_create": false}, {"before_b64": "Ly8vIEV4dHJhY3QgYHB1YiB1c2VgIHJlLWV4cG9ydHMuIFJlc3VsdHMgc29ydGVkIGJ5IGV4cG9ydF9wYXRoLgojW211c3RfdXNlXQpwdWIgZm4gZXh0cmFjdF9yZV9leHBvcnRz", "after_b64": "Ly8vIEV4dHJhY3QgYHB1YiB1c2VgIHJlLWV4cG9ydHMuIFJlc3VsdHMgc29ydGVkIGJ5IGBleHBvcnRfcGF0aGAuCiNbbXVzdF91c2VdCnB1YiBmbiBleHRyYWN0X3JlX2V4cG9ydHM=", "target": "src/file_parser.rs", "index": 13, "is_create": false}, {"before_b64": "ICAgICAgICAgICAgbGV0IG5hbWUgPSBtLmlkZW50LmFzX3JlZigpLm1hcCh8aXwgaS50b19zdHJpbmcoKSkudW53cmFwX29yX2RlZmF1bHQoKTs=", "after_b64": "ICAgICAgICAgICAgbGV0IG5hbWUgPSBtLmlkZW50LmFzX3JlZigpLm1hcChUb1N0cmluZzo6dG9fc3RyaW5nKS51bndyYXBfb3JfZGVmYXVsdCgpOw==", "target": "src/file_parser.rs", "index": 14, "is_create": false}, {"before_b64": "Zm4gZmxhdHRlbl91c2VfdHJlZSh0cmVlOiAmc3luOjpVc2VUcmVlLCBwcmVmaXg6IFN0cmluZywgbGluZTogdXNpemUpIC0+IFZlYzxJbXBvcnQ+IHs=", "after_b64": "Zm4gZmxhdHRlbl91c2VfdHJlZSh0cmVlOiAmc3luOjpVc2VUcmVlLCBwcmVmaXg6ICZzdHIsIGxpbmU6IHVzaXplKSAtPiBWZWM8SW1wb3J0PiB7", "target": "src/file_parser.rs", "index": 15, "is_create": false}, {"before_b64": "ICAgICAgICAgICAgICAgIFNvbWUoZmxhdHRlbl91c2VfdHJlZSgmdS50cmVlLCBTdHJpbmc6Om5ldygpLCBsaW5lX29mX2l0ZW0oaXRlbSkpKQ==", "after_b64": "ICAgICAgICAgICAgICAgIFNvbWUoZmxhdHRlbl91c2VfdHJlZSgmdS50cmVlLCAiIiwgbGluZV9vZl9pdGVtKGl0ZW0pKSk=", "target": "src/file_parser.rs", "index": 16, "is_create": false}, {"before_b64": "ICAgICAgICAgICAgZmxhdHRlbl91c2VfdHJlZSgmcC50cmVlLCBuZXdfcHJlZml4LCBsaW5lKQ==", "after_b64": "ICAgICAgICAgICAgZmxhdHRlbl91c2VfdHJlZSgmcC50cmVlLCAmbmV3X3ByZWZpeCwgbGluZSk=", "target": "src/file_parser.rs", "index": 17, "is_create": false}, {"before_b64": "Ly8vIEludGVybmFsIGludGVybWVkaWF0ZSB0eXBlIGNvbnN1bWVkIGJ5IG1vZHVsZV90cmVlLgojW2Rlcml2ZShEZWJ1ZywgQ2xvbmUsIERlZmF1bHQpXQpwdWIgc3RydWN0IEZpbGVJbmZv", "after_b64": "Ly8vIEludGVybmFsIGludGVybWVkaWF0ZSB0eXBlIGNvbnN1bWVkIGJ5IGBtb2R1bGVfdHJlZWAuCiNbZGVyaXZlKERlYnVnLCBDbG9uZSwgRGVmYXVsdCldCnB1YiBzdHJ1Y3QgRmlsZUluZm8=", "target": "src/schema.rs", "index": 18, "is_create": false}, {"before_b64": "Ly8vIHBhcnNlZCwgb3IgdGhlIEpTT04gb3V0cHV0IGNhbm5vdCBiZSB3cml0dGVuLgpwdWIgZm4gcnVuKGNvbmZpZzogQ29uZmlnKSAtPiBhbnlob3c6OlJlc3VsdDwoKT4gew==", "after_b64": "Ly8vIHBhcnNlZCwgb3IgdGhlIEpTT04gb3V0cHV0IGNhbm5vdCBiZSB3cml0dGVuLgpwdWIgZm4gcnVuKGNvbmZpZzogJkNvbmZpZykgLT4gYW55aG93OjpSZXN1bHQ8KCk+IHs=", "target": "src/lib.rs", "index": 19, "is_create": false}, {"before_b64": "ICAgIHJ1c3Rfd29ya3NwYWNlX21hcDo6cnVuKGNvbmZpZyk=", "after_b64": "ICAgIHJ1c3Rfd29ya3NwYWNlX21hcDo6cnVuKCZjb25maWcp", "target": "src/main.rs", "index": 20, "is_create": false}]')
-
-for step in STEPS:
-    before = base64.b64decode(step["before_b64"]).decode()
-    after = base64.b64decode(step["after_b64"]).decode()
-    target = step["target"]
-    idx = step["index"]
-    is_create = step["is_create"]
-
-    if is_create:
-        target_path = Path(target)
-        target_path.parent.mkdir(parents=True, exist_ok=True)
-        target_path.write_text(after)
-        print(f"OK {TASK_ID} change {idx}: created {target}")
-    else:
-        target_path = Path(target)
-        content = target_path.read_text()
-        if before not in content:
-            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
-            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
-            sys.exit(1)
-
-        result = subprocess.run(
-            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
-            capture_output=True, text=True,
-        )
-        if result.returncode != 0:
-            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
-            sys.exit(result.returncode)
-
-        new_content = target_path.read_text()
-        if after and after not in new_content:
-            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
-            sys.exit(1)
-
-        print(f"OK {TASK_ID} change {idx}: applied to {target}")
-
-print(f"OK {TASK_ID}: all changes applied")
diff --git a/plans/compiled/TASK-8.sh b/plans/compiled/TASK-8.sh
deleted file mode 100755
index 63050be..0000000
--- a/plans/compiled/TASK-8.sh
+++ /dev/null
@@ -1,7 +0,0 @@
-#!/usr/bin/env bash
-set -euo pipefail
-# TASK-8: Remove all crate-level clippy allow attributes and fix individual lint violations
-# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
-# Type: replace
-# File: src/lib.rs
-python3 "$(dirname "$0")/TASK-8.py"
diff --git a/plans/compiled/TASK-9.py b/plans/compiled/TASK-9.py
deleted file mode 100644
index 7ff9e4a..0000000
--- a/plans/compiled/TASK-9.py
+++ /dev/null
@@ -1,44 +0,0 @@
-#!/usr/bin/env python3
-"""TASK-9: Add unit tests for file_parser, module_tree, workspace, cargo_info, cross_refs, and render"""
-import base64, json, subprocess, sys
-from pathlib import Path
-
-TASK_ID = "TASK-9"
-STEPS = json.loads('[{"before_b64": "Zm4gZXh0cmFjdF9yZV9leHBvcnRzX2Zyb21fdHJlZSgKICAgIHRyZWU6ICZzeW46OlVzZVRyZWUsCiAgICBpbXBvcnRfcGF0aDogU3RyaW5nLAogICAgbGluZTogdXNpemUsCikgLT4gVmVjPFJlRXhwb3J0PiB7CiAgICBtYXRjaCB0cmVlIHsKICAgICAgICBzeW46OlVzZVRyZWU6OlBhdGgocCkgPT4gewogICAgICAgICAgICBsZXQgbmV3X2ltcG9ydCA9IGlmIGltcG9ydF9wYXRoLmlzX2VtcHR5KCkgewogICAgICAgICAgICAgICAgcC5pZGVudC50b19zdHJpbmcoKQogICAgICAgICAgICB9IGVsc2UgewogICAgICAgICAgICAgICAgZm9ybWF0ISgie306Ont9IiwgaW1wb3J0X3BhdGgsIHAuaWRlbnQpCiAgICAgICAgICAgIH07CiAgICAgICAgICAgIGV4dHJhY3RfcmVfZXhwb3J0c19mcm9tX3RyZWUoJnAudHJlZSwgbmV3X2ltcG9ydCwgbGluZSkKICAgICAgICB9CiAgICAgICAgc3luOjpVc2VUcmVlOjpOYW1lKG4pID0+IHsKICAgICAgICAgICAgdmVjIVtSZUV4cG9ydCB7CiAgICAgICAgICAgICAgICBpbXBvcnRfcGF0aCwKICAgICAgICAgICAgICAgIGV4cG9ydF9wYXRoOiBuLmlkZW50LnRvX3N0cmluZygpLAogICAgICAgICAgICAgICAgbGluZSwKICAgICAgICAgICAgfV0KICAgICAgICB9CiAgICAgICAgc3luOjpVc2VUcmVlOjpSZW5hbWUocikgPT4gewogICAgICAgICAgICB2ZWMhW1JlRXhwb3J0IHsKICAgICAgICAgICAgICAgIGltcG9ydF9wYXRoLAogICAgICAgICAgICAgICAgZXhwb3J0X3BhdGg6IHIucmVuYW1lLnRvX3N0cmluZygpLAogICAgICAgICAgICAgICAgbGluZSwKICAgICAgICAgICAgfV0KICAgICAgICB9CiAgICAgICAgc3luOjpVc2VUcmVlOjpHbG9iKF8pID0+IHsKICAgICAgICAgICAgdmVjIVtSZUV4cG9ydCB7CiAgICAgICAgICAgICAgICBpbXBvcnRfcGF0aCwKICAgICAgICAgICAgICAgIGV4cG9ydF9wYXRoOiAiKiIudG9fc3RyaW5nKCksCiAgICAgICAgICAgICAgICBsaW5lLAogICAgICAgICAgICB9XQogICAgICAgIH0KICAgICAgICBzeW46OlVzZVRyZWU6Okdyb3VwKGcpID0+IGcKICAgICAgICAgICAgLml0ZW1zCiAgICAgICAgICAgIC5pdGVyKCkKICAgICAgICAgICAgLmZsYXRfbWFwKHx0fCBleHRyYWN0X3JlX2V4cG9ydHNfZnJvbV90cmVlKHQsIGltcG9ydF9wYXRoLmNsb25lKCksIGxpbmUpKQogICAgICAgICAgICAuY29sbGVjdCgpLAogICAgfQp9", "after_b64": "Zm4gZXh0cmFjdF9yZV9leHBvcnRzX2Zyb21fdHJlZSgKICAgIHRyZWU6ICZzeW46OlVzZVRyZWUsCiAgICBpbXBvcnRfcGF0aDogU3RyaW5nLAogICAgbGluZTogdXNpemUsCikgLT4gVmVjPFJlRXhwb3J0PiB7CiAgICBtYXRjaCB0cmVlIHsKICAgICAgICBzeW46OlVzZVRyZWU6OlBhdGgocCkgPT4gewogICAgICAgICAgICBsZXQgbmV3X2ltcG9ydCA9IGlmIGltcG9ydF9wYXRoLmlzX2VtcHR5KCkgewogICAgICAgICAgICAgICAgcC5pZGVudC50b19zdHJpbmcoKQogICAgICAgICAgICB9IGVsc2UgewogICAgICAgICAgICAgICAgZm9ybWF0ISgie306Ont9IiwgaW1wb3J0X3BhdGgsIHAuaWRlbnQpCiAgICAgICAgICAgIH07CiAgICAgICAgICAgIGV4dHJhY3RfcmVfZXhwb3J0c19mcm9tX3RyZWUoJnAudHJlZSwgbmV3X2ltcG9ydCwgbGluZSkKICAgICAgICB9CiAgICAgICAgc3luOjpVc2VUcmVlOjpOYW1lKG4pID0+IHsKICAgICAgICAgICAgdmVjIVtSZUV4cG9ydCB7CiAgICAgICAgICAgICAgICBpbXBvcnRfcGF0aCwKICAgICAgICAgICAgICAgIGV4cG9ydF9wYXRoOiBuLmlkZW50LnRvX3N0cmluZygpLAogICAgICAgICAgICAgICAgbGluZSwKICAgICAgICAgICAgfV0KICAgICAgICB9CiAgICAgICAgc3luOjpVc2VUcmVlOjpSZW5hbWUocikgPT4gewogICAgICAgICAgICB2ZWMhW1JlRXhwb3J0IHsKICAgICAgICAgICAgICAgIGltcG9ydF9wYXRoLAogICAgICAgICAgICAgICAgZXhwb3J0X3BhdGg6IHIucmVuYW1lLnRvX3N0cmluZygpLAogICAgICAgICAgICAgICAgbGluZSwKICAgICAgICAgICAgfV0KICAgICAgICB9CiAgICAgICAgc3luOjpVc2VUcmVlOjpHbG9iKF8pID0+IHsKICAgICAgICAgICAgdmVjIVtSZUV4cG9ydCB7CiAgICAgICAgICAgICAgICBpbXBvcnRfcGF0aCwKICAgICAgICAgICAgICAgIGV4cG9ydF9wYXRoOiAiKiIudG9fc3RyaW5nKCksCiAgICAgICAgICAgICAgICBsaW5lLAogICAgICAgICAgICB9XQogICAgICAgIH0KICAgICAgICBzeW46OlVzZVRyZWU6Okdyb3VwKGcpID0+IGcKICAgICAgICAgICAgLml0ZW1zCiAgICAgICAgICAgIC5pdGVyKCkKICAgICAgICAgICAgLmZsYXRfbWFwKHx0fCBleHRyYWN0X3JlX2V4cG9ydHNfZnJvbV90cmVlKHQsIGltcG9ydF9wYXRoLmNsb25lKCksIGxpbmUpKQogICAgICAgICAgICAuY29sbGVjdCgpLAogICAgfQp9CgovLyDilIDilIAgVGVzdHMg4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSACgojW2NmZyh0ZXN0KV0KbW9kIHRlc3RzIHsKICAgIHVzZSBzdXBlcjo6KjsKICAgIHVzZSBjcmF0ZTo6c2NoZW1hOjp7SW1wb3J0LCBSZUV4cG9ydCwgU3VibW9kdWxlRGVjbH07CiAgICB1c2Ugc3RkOjpwYXRoOjpQYXRoQnVmOwoKICAgIGZuIHBhcnNlX3NvdXJjZShzcmM6ICZzdHIpIC0+IFBhcnNlZEZpbGUgewogICAgICAgIGxldCB0bXAgPSBzdGQ6OmVudjo6dGVtcF9kaXIoKS5qb2luKCJwYXJzZV90ZXN0LnJzIik7CiAgICAgICAgc3RkOjpmczo6d3JpdGUoJnRtcCwgc3JjKS51bndyYXAoKTsKICAgICAgICBsZXQgcmVzdWx0ID0gcGFyc2VfZmlsZSgmdG1wKTsKICAgICAgICBzdGQ6OmZzOjpyZW1vdmVfZmlsZSgmdG1wKS5vaygpOwogICAgICAgIHJlc3VsdAogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIHBhcnNlX2ZpbGVfcmV0dXJuc19hc3RfZm9yX3ZhbGlkX3NvdXJjZSgpIHsKICAgICAgICBsZXQgc3JjID0gInB1YiBzdHJ1Y3QgRm9vIHsgeDogaTMyIH0iOwogICAgICAgIGxldCByZXN1bHQgPSBwYXJzZV9zb3VyY2Uoc3JjKTsKICAgICAgICBhc3NlcnQhKHJlc3VsdC5wYXJzZV9lcnJvci5pc19ub25lKCkpOwogICAgICAgIGFzc2VydF9lcSEocmVzdWx0LmFzdC5pdGVtcy5sZW4oKSwgMSk7CiAgICB9CgogICAgI1t0ZXN0XQogICAgZm4gcGFyc2VfZmlsZV9yZXR1cm5zX2Vycm9yX2Zvcl9pbnZhbGlkX3NvdXJjZSgpIHsKICAgICAgICBsZXQgc3JjID0gInB1YiBzdHJ1Y3QgeyBpbnZhbGlkIHJ1c3QgfSI7CiAgICAgICAgbGV0IHJlc3VsdCA9IHBhcnNlX3NvdXJjZShzcmMpOwogICAgICAgIGFzc2VydCEocmVzdWx0LnBhcnNlX2Vycm9yLmlzX3NvbWUoKSk7CiAgICAgICAgbGV0IGVyciA9IHJlc3VsdC5wYXJzZV9lcnJvci5hc19yZWYoKS51bndyYXAoKTsKICAgICAgICBhc3NlcnQhKCFlcnIubWVzc2FnZS5pc19lbXB0eSgpKTsKICAgICAgICBhc3NlcnQhKGVyci5saW5lID4gMCk7CiAgICB9CgogICAgI1t0ZXN0XQogICAgZm4gcGFyc2VfZmlsZV9yZXR1cm5zX2VtcHR5X2Zvcl9lbXB0eV9maWxlKCkgewogICAgICAgIGxldCByZXN1bHQgPSBwYXJzZV9zb3VyY2UoIiIpOwogICAgICAgIGFzc2VydCEocmVzdWx0LnBhcnNlX2Vycm9yLmlzX25vbmUoKSk7CiAgICAgICAgYXNzZXJ0IShyZXN1bHQuZmlsZV9pbmZvLnB1YmxpY19pdGVtcy5pc19lbXB0eSgpKTsKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiBleHRyYWN0X3B1YmxpY19pdGVtc19maW5kc19zdHJ1Y3RfZW51bV90cmFpdF9mbigpIHsKICAgICAgICBsZXQgc3JjID0gInB1YiBzdHJ1Y3QgRm9vIHt9IHB1YiBlbnVtIEJhciB7IEEsIEIgfSBwdWIgdHJhaXQgQmF6IHt9IHB1YiBmbiBoZWxsbygpIHt9IjsKICAgICAgICBsZXQgcmVzdWx0ID0gcGFyc2Vfc291cmNlKHNyYyk7CiAgICAgICAgbGV0IGl0ZW1zID0gZXh0cmFjdF9wdWJsaWNfaXRlbXMoJnJlc3VsdC5hc3QuaXRlbXMpOwogICAgICAgIGxldCBuYW1lczogVmVjPF8+ID0gaXRlbXMuaXRlcigpLm1hcCh8aXwgaS5uYW1lLmFzX3N0cigpKS5jb2xsZWN0KCk7CiAgICAgICAgYXNzZXJ0IShuYW1lcy5jb250YWlucygmIkZvbyIpKTsKICAgICAgICBhc3NlcnQhKG5hbWVzLmNvbnRhaW5zKCYiQmFyIikpOwogICAgICAgIGFzc2VydCEobmFtZXMuY29udGFpbnMoJiJCYXoiKSk7CiAgICAgICAgYXNzZXJ0IShuYW1lcy5jb250YWlucygmImhlbGxvIikpOwogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIGV4dHJhY3RfcHVibGljX2l0ZW1zX2VtcHR5X2Zvcl9ub19wdWJsaWNfaXRlbXMoKSB7CiAgICAgICAgbGV0IHNyYyA9ICJzdHJ1Y3QgUHJpdmF0ZSB7fSBmbiBwcml2YXRlX2ZuKCkge30iOwogICAgICAgIGxldCByZXN1bHQgPSBwYXJzZV9zb3VyY2Uoc3JjKTsKICAgICAgICBsZXQgaXRlbXMgPSBleHRyYWN0X3B1YmxpY19pdGVtcygmcmVzdWx0LmFzdC5pdGVtcyk7CiAgICAgICAgYXNzZXJ0IShpdGVtcy5pc19lbXB0eSgpKTsKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiBleHRyYWN0X2ltcG9ydHNfZmluZHNfdXNlX3N0YXRlbWVudHMoKSB7CiAgICAgICAgbGV0IHNyYyA9ICJ1c2Ugc3RkOjpjb2xsZWN0aW9uczo6QlRyZWVNYXA7IjsKICAgICAgICBsZXQgcmVzdWx0ID0gcGFyc2Vfc291cmNlKHNyYyk7CiAgICAgICAgbGV0IGltcG9ydHMgPSBleHRyYWN0X2ltcG9ydHMoJnJlc3VsdC5hc3QuaXRlbXMpOwogICAgICAgIGFzc2VydF9lcSEoaW1wb3J0cy5sZW4oKSwgMSk7CiAgICAgICAgYXNzZXJ0X2VxIShpbXBvcnRzWzBdLnBhdGgsICJzdGQ6OmNvbGxlY3Rpb25zOjpCVHJlZU1hcCIpOwogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIGV4dHJhY3RfcmVfZXhwb3J0c19maW5kc19wdWJfdXNlKCkgewogICAgICAgIGxldCBzcmMgPSAicHViIHVzZSBjcmF0ZTo6Zm9vOyI7CiAgICAgICAgbGV0IHJlc3VsdCA9IHBhcnNlX3NvdXJjZShzcmMpOwogICAgICAgIGxldCByZV9leHBvcnRzID0gZXh0cmFjdF9yZV9leHBvcnRzKCZyZXN1bHQuYXN0Lml0ZW1zKTsKICAgICAgICBhc3NlcnRfZXEhKHJlX2V4cG9ydHMubGVuKCksIDEpOwogICAgICAgIGFzc2VydF9lcSEocmVfZXhwb3J0c1swXS5pbXBvcnRfcGF0aCwgImNyYXRlOjpmb28iKTsKICAgICAgICBhc3NlcnRfZXEhKHJlX2V4cG9ydHNbMF0uZXhwb3J0X3BhdGgsICJmb28iKTsKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiBleHRyYWN0X3JlX2V4cG9ydHNfZmluZHNfcmVuYW1lKCkgewogICAgICAgIGxldCBzcmMgPSAicHViIHVzZSBjcmF0ZTo6Zm9vIGFzIGJhcjsiOwogICAgICAgIGxldCByZXN1bHQgPSBwYXJzZV9zb3VyY2Uoc3JjKTsKICAgICAgICBsZXQgcmVfZXhwb3J0cyA9IGV4dHJhY3RfcmVfZXhwb3J0cygmcmVzdWx0LmFzdC5pdGVtcyk7CiAgICAgICAgYXNzZXJ0X2VxIShyZV9leHBvcnRzLmxlbigpLCAxKTsKICAgICAgICBhc3NlcnRfZXEhKHJlX2V4cG9ydHNbMF0uaW1wb3J0X3BhdGgsICJjcmF0ZTo6Zm9vIGFzIGJhciIpOwogICAgICAgIGFzc2VydF9lcSEocmVfZXhwb3J0c1swXS5leHBvcnRfcGF0aCwgImJhciIpOwogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIGV4dHJhY3Rfc3VibW9kdWxlc19maW5kc19tb2RfZGVjbGFyYXRpb25zKCkgewogICAgICAgIGxldCBzcmMgPSAibW9kIGZvbzsgbW9kIGJhcjsiOwogICAgICAgIGxldCByZXN1bHQgPSBwYXJzZV9zb3VyY2Uoc3JjKTsKICAgICAgICBsZXQgc3VicyA9IGV4dHJhY3Rfc3VibW9kdWxlcygmcmVzdWx0LmFzdC5pdGVtcyk7CiAgICAgICAgYXNzZXJ0X2VxIShzdWJzLmxlbigpLCAyKTsKICAgICAgICBsZXQgbmFtZXM6IFZlYzxfPiA9IHN1YnMuaXRlcigpLm1hcCh8c3wgcy5uYW1lLmFzX3N0cigpKS5jb2xsZWN0KCk7CiAgICAgICAgYXNzZXJ0IShuYW1lcy5jb250YWlucygmImJhciIpKTsKICAgICAgICBhc3NlcnQhKG5hbWVzLmNvbnRhaW5zKCYiZm9vIikpOwogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIGV4dHJhY3Rfc3VibW9kdWxlc19tYXJrc19jZmdfdGVzdCgpIHsKICAgICAgICBsZXQgc3JjID0gIiNbY2ZnKHRlc3QpXSBtb2QgaW5uZXI7IjsKICAgICAgICBsZXQgcmVzdWx0ID0gcGFyc2Vfc291cmNlKHNyYyk7CiAgICAgICAgbGV0IHN1YnMgPSBleHRyYWN0X3N1Ym1vZHVsZXMoJnJlc3VsdC5hc3QuaXRlbXMpOwogICAgICAgIGFzc2VydF9lcSEoc3Vicy5sZW4oKSwgMSk7CiAgICAgICAgYXNzZXJ0IShzdWJzWzBdLmlzX3Rlc3QpOwogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIGV4dHJhY3RfaW1wbHNfZmluZHNfZm5fdHlwZV9jb25zdCgpIHsKICAgICAgICBsZXQgc3JjID0gImltcGwgTXlUeXBlIHsgcHViIGZuIGZvbygmc2VsZikge30gcHViIHR5cGUgQWxpYXMgPSB1MzI7IHB1YiBjb25zdCBOOiB1c2l6ZSA9IDQyOyB9IjsKICAgICAgICBsZXQgcmVzdWx0ID0gcGFyc2Vfc291cmNlKHNyYyk7CiAgICAgICAgbGV0IGltcGxzID0gZXh0cmFjdF9pbXBscygmcmVzdWx0LmFzdC5pdGVtcyk7CiAgICAgICAgYXNzZXJ0X2VxIShpbXBscy5sZW4oKSwgMSk7CiAgICAgICAgYXNzZXJ0X2VxIShpbXBsc1swXS50eXBlXywgIk15VHlwZSIpOwogICAgICAgIGFzc2VydF9lcSEoaW1wbHNbMF0uaXRlbXMubGVuKCksIDMpOwogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIGJ1aWxkX3BhcnNlX2Vycm9yX2VudHJ5X2NvbnN0cnVjdHNfZXJyb3IoKSB7CiAgICAgICAgbGV0IHBhdGggPSBQYXRoQnVmOjpmcm9tKCJ0ZXN0LnJzIik7CiAgICAgICAgbGV0IGVyciA9IFN5blBhcnNlRXJyb3IgewogICAgICAgICAgICBtZXNzYWdlOiAiZXhwZWN0ZWQgYDtgIi50b19zdHJpbmcoKSwKICAgICAgICAgICAgbGluZTogNSwKICAgICAgICB9OwogICAgICAgIGxldCBlbnRyeSA9IGJ1aWxkX3BhcnNlX2Vycm9yX2VudHJ5KCZwYXRoLCAmZXJyKTsKICAgICAgICBhc3NlcnRfZXEhKGVudHJ5LmZpbGUsICJ0ZXN0LnJzIik7CiAgICAgICAgYXNzZXJ0X2VxIShlbnRyeS5saW5lLCA1KTsKICAgICAgICBhc3NlcnRfZXEhKGVudHJ5LmtpbmQsICJzeW5fcGFyc2VfZXJyb3IiKTsKICAgICAgICBhc3NlcnRfZXEhKGVudHJ5LnNldmVyaXR5LCBFcnJvclNldmVyaXR5OjpFcnJvcik7CiAgICB9Cn0=", "target": "src/file_parser.rs", "index": 0, "is_create": false}, {"before_b64": "Zm4gcHJvY2Vzc19tb2R1bGVfaW5mbygKICAgIG1vZHVsZV9wYXRoOiAmc3RyLAogICAgZmlsZV9wYXRoOiAmUGF0aCwKICAgIHZpc2liaWxpdHk6ICZzdHIsCiAgICBmaWxlX2luZm86ICZGaWxlSW5mbywKICAgIGl0ZW1zOiAmW3N5bjo6SXRlbV0sCiAgICBfcGFyZW50X2RpcjogJlBhdGgsCiAgICB2aXNpdGVkOiAmbXV0IEhhc2hTZXQ8UGF0aEJ1Zj4sCiAgICBlcnJvcnM6ICZtdXQgVmVjPGNyYXRlOjpzY2hlbWE6OkVycm9yRW50cnk+LAopIC0+IChWZWM8TW9kdWxlSW5mbz4sIFZlYzxjcmF0ZTo6c2NoZW1hOjpFcnJvckVudHJ5PikgewogICAgbGV0IG11dCBtb2R1bGVzID0gdmVjIVtidWlsZF9tb2R1bGVfaW5mbygKICAgICAgICBtb2R1bGVfcGF0aCwKICAgICAgICBmaWxlX3BhdGgsCiAgICAgICAgdmlzaWJpbGl0eSwKICAgICAgICAmZmlsZV9pbmZvLnB1YmxpY19pdGVtcywKICAgICAgICAmZmlsZV9pbmZvLmltcG9ydHMsCiAgICAgICAgJmZpbGVfaW5mby5yZV9leHBvcnRzLAogICAgICAgICZmaWxlX2luZm8uc3VibW9kdWxlcywKICAgICldOwoKICAgIGZvciBzdWIgaW4gJmZpbGVfaW5mby5zdWJtb2R1bGVzIHsKICAgICAgICBpZiBzdWIuaXNfdGVzdCB7CiAgICAgICAgICAgIGNvbnRpbnVlOwogICAgICAgIH0KICAgICAgICBsZXQgY2hpbGRfcGF0aCA9IGZvcm1hdCEoInt9Ojp7fSIsIG1vZHVsZV9wYXRoLCBzdWIubmFtZSk7CiAgICAgICAgbGV0IGNoaWxkX2RpciA9IGZpbGVfcGF0aC5wYXJlbnQoKS51bndyYXBfb3IoZmlsZV9wYXRoKTsKICAgICAgICBsZXQgKGNoaWxkX21vZHVsZXMsIGNoaWxkX2Vycm9ycykgPSBwcm9jZXNzX3N1Ym1vZHVsZSgKICAgICAgICAgICAgJmNoaWxkX3BhdGgsCiAgICAgICAgICAgICZzdWIubmFtZSwKICAgICAgICAgICAgaXRlbXMsCiAgICAgICAgICAgIGNoaWxkX2RpciwKICAgICAgICAgICAgZmlsZV9wYXRoLAogICAgICAgICAgICB2aXNpdGVkLAogICAgICAgICk7CiAgICAgICAgZXJyb3JzLmV4dGVuZChjaGlsZF9lcnJvcnMpOwogICAgICAgIG1vZHVsZXMuZXh0ZW5kKGNoaWxkX21vZHVsZXMpOwogICAgfQoKICAgIChtb2R1bGVzLCBlcnJvcnMuY2xvbmUoKSkKfQ==", "after_b64": "Zm4gcHJvY2Vzc19tb2R1bGVfaW5mbygKICAgIG1vZHVsZV9wYXRoOiAmc3RyLAogICAgZmlsZV9wYXRoOiAmUGF0aCwKICAgIHZpc2liaWxpdHk6ICZzdHIsCiAgICBmaWxlX2luZm86ICZGaWxlSW5mbywKICAgIGl0ZW1zOiAmW3N5bjo6SXRlbV0sCiAgICBfcGFyZW50X2RpcjogJlBhdGgsCiAgICB2aXNpdGVkOiAmbXV0IEhhc2hTZXQ8UGF0aEJ1Zj4sCiAgICBlcnJvcnM6ICZtdXQgVmVjPGNyYXRlOjpzY2hlbWE6OkVycm9yRW50cnk+LAopIC0+IChWZWM8TW9kdWxlSW5mbz4sIFZlYzxjcmF0ZTo6c2NoZW1hOjpFcnJvckVudHJ5PikgewogICAgbGV0IG11dCBtb2R1bGVzID0gdmVjIVtidWlsZF9tb2R1bGVfaW5mbygKICAgICAgICBtb2R1bGVfcGF0aCwKICAgICAgICBmaWxlX3BhdGgsCiAgICAgICAgdmlzaWJpbGl0eSwKICAgICAgICAmZmlsZV9pbmZvLnB1YmxpY19pdGVtcywKICAgICAgICAmZmlsZV9pbmZvLmltcG9ydHMsCiAgICAgICAgJmZpbGVfaW5mby5yZV9leHBvcnRzLAogICAgICAgICZmaWxlX2luZm8uc3VibW9kdWxlcywKICAgICldOwoKICAgIGZvciBzdWIgaW4gJmZpbGVfaW5mby5zdWJtb2R1bGVzIHsKICAgICAgICBpZiBzdWIuaXNfdGVzdCB7CiAgICAgICAgICAgIGNvbnRpbnVlOwogICAgICAgIH0KICAgICAgICBsZXQgY2hpbGRfcGF0aCA9IGZvcm1hdCEoInt9Ojp7fSIsIG1vZHVsZV9wYXRoLCBzdWIubmFtZSk7CiAgICAgICAgbGV0IGNoaWxkX2RpciA9IGZpbGVfcGF0aC5wYXJlbnQoKS51bndyYXBfb3IoZmlsZV9wYXRoKTsKICAgICAgICBsZXQgKGNoaWxkX21vZHVsZXMsIGNoaWxkX2Vycm9ycykgPSBwcm9jZXNzX3N1Ym1vZHVsZSgKICAgICAgICAgICAgJmNoaWxkX3BhdGgsCiAgICAgICAgICAgICZzdWIubmFtZSwKICAgICAgICAgICAgaXRlbXMsCiAgICAgICAgICAgIGNoaWxkX2RpciwKICAgICAgICAgICAgZmlsZV9wYXRoLAogICAgICAgICAgICB2aXNpdGVkLAogICAgICAgICk7CiAgICAgICAgZXJyb3JzLmV4dGVuZChjaGlsZF9lcnJvcnMpOwogICAgICAgIG1vZHVsZXMuZXh0ZW5kKGNoaWxkX21vZHVsZXMpOwogICAgfQoKICAgIChtb2R1bGVzLCBlcnJvcnMuY2xvbmUoKSkKfQoKLy8g4pSA4pSAIFRlc3RzIOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgAoKI1tjZmcodGVzdCldCm1vZCB0ZXN0cyB7CiAgICB1c2Ugc3VwZXI6Oio7CiAgICB1c2UgY3JhdGU6OnNjaGVtYTo6RXJyb3JFbnRyeTsKCiAgICAjW3Rlc3RdCiAgICBmbiByZXNvbHZlX21vZHVsZV9wYXRoX2ZpbmRzX3JzX2ZpbGUoKSB7CiAgICAgICAgbGV0IHRtcCA9IHN0ZDo6ZW52Ojp0ZW1wX2RpcigpLmpvaW4oInJlc29sdmVfdGVzdCIpOwogICAgICAgIGxldCBfID0gc3RkOjpmczo6Y3JlYXRlX2Rpcl9hbGwoJnRtcCk7CiAgICAgICAgbGV0IG1vZF9maWxlID0gdG1wLmpvaW4oImZvby5ycyIpOwogICAgICAgIHN0ZDo6ZnM6OndyaXRlKCZtb2RfZmlsZSwgIiIpLm9rKCk7CiAgICAgICAgbGV0IHJlc3VsdCA9IHJlc29sdmVfbW9kdWxlX3BhdGgoJnRtcCwgImZvbyIpOwogICAgICAgIGFzc2VydF9lcSEocmVzdWx0LCBTb21lKG1vZF9maWxlKSk7CiAgICAgICAgc3RkOjpmczo6cmVtb3ZlX2Rpcl9hbGwoJnRtcCkub2soKTsKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiByZXNvbHZlX21vZHVsZV9wYXRoX2ZpbmRzX21vZF9ycygpIHsKICAgICAgICBsZXQgdG1wID0gc3RkOjplbnY6OnRlbXBfZGlyKCkuam9pbigicmVzb2x2ZV90ZXN0MiIpOwogICAgICAgIGxldCBfID0gc3RkOjpmczo6Y3JlYXRlX2Rpcl9hbGwoJnRtcCk7CiAgICAgICAgbGV0IG1vZF9kaXIgPSB0bXAuam9pbigiYmFyIik7CiAgICAgICAgbGV0IF8gPSBzdGQ6OmZzOjpjcmVhdGVfZGlyX2FsbCgmbW9kX2Rpcik7CiAgICAgICAgbGV0IG1vZF9ycyA9IG1vZF9kaXIuam9pbigibW9kLnJzIik7CiAgICAgICAgc3RkOjpmczo6d3JpdGUoJm1vZF9ycywgIiIpLm9rKCk7CiAgICAgICAgbGV0IHJlc3VsdCA9IHJlc29sdmVfbW9kdWxlX3BhdGgoJnRtcCwgImJhciIpOwogICAgICAgIGFzc2VydF9lcSEocmVzdWx0LCBTb21lKG1vZF9ycykpOwogICAgICAgIHN0ZDo6ZnM6OnJlbW92ZV9kaXJfYWxsKCZ0bXApLm9rKCk7CiAgICB9CgogICAgI1t0ZXN0XQogICAgZm4gcmVzb2x2ZV9tb2R1bGVfcGF0aF9yZXR1cm5zX25vbmVfZm9yX21pc3NpbmcoKSB7CiAgICAgICAgbGV0IHRtcCA9IHN0ZDo6ZW52Ojp0ZW1wX2RpcigpLmpvaW4oInJlc29sdmVfdGVzdDMiKTsKICAgICAgICBsZXQgXyA9IHN0ZDo6ZnM6OmNyZWF0ZV9kaXJfYWxsKCZ0bXApOwogICAgICAgIGxldCByZXN1bHQgPSByZXNvbHZlX21vZHVsZV9wYXRoKCZ0bXAsICJub25leGlzdGVudCIpOwogICAgICAgIGFzc2VydCEocmVzdWx0LmlzX25vbmUoKSk7CiAgICAgICAgc3RkOjpmczo6cmVtb3ZlX2Rpcl9hbGwoJnRtcCkub2soKTsKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiBidWlsZF9tb2R1bGVfdHJlZV9yZXR1cm5zX2VtcHR5X2Zvcl9ub25leGlzdGVudCgpIHsKICAgICAgICBsZXQgdG1wID0gc3RkOjplbnY6OnRlbXBfZGlyKCkuam9pbigiYm10X3Rlc3QiKTsKICAgICAgICBsZXQgXyA9IHN0ZDo6ZnM6OmNyZWF0ZV9kaXJfYWxsKCZ0bXApOwogICAgICAgIGxldCAobW9kdWxlcywgZXJyb3JzKSA9IGJ1aWxkX21vZHVsZV90cmVlKCZ0bXAsICJ0ZXN0Iik7CiAgICAgICAgYXNzZXJ0IShtb2R1bGVzLmlzX2VtcHR5KCkpOwogICAgICAgIGFzc2VydCEoIWVycm9ycy5pc19lbXB0eSgpKTsKICAgICAgICBzdGQ6OmZzOjpyZW1vdmVfZGlyX2FsbCgmdG1wKS5vaygpOwogICAgfQp9", "target": "src/module_tree.rs", "index": 1, "is_create": false}, {"before_b64": "ICAgIHJlc3VsdC5zb3J0KCk7CiAgICByZXN1bHQuZGVkdXAoKTsKICAgIE9rKHJlc3VsdCkKfQoKLy8vIEZvciBhIGNyYXRlIGRpcmVjdG9yeSwgZGV0ZXJtaW5lIGl0cyBlbnRyeS1wb2ludCBmaWxlKHMpLg==", "after_b64": "ICAgIHJlc3VsdC5zb3J0KCk7CiAgICByZXN1bHQuZGVkdXAoKTsKICAgIE9rKHJlc3VsdCkKfQoKLy8g4pSA4pSAIFRlc3RzIOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgAoKI1tjZmcodGVzdCldCm1vZCB0ZXN0cyB7CiAgICB1c2Ugc3VwZXI6Oio7CiAgICB1c2Ugc3RkOjppbzo6V3JpdGU7CgogICAgZm4gd3JpdGVfY2FyZ29fdG9tbChkaXI6ICZzdGQ6OnBhdGg6OlBhdGgsIGNvbnRlbnQ6ICZzdHIpIHsKICAgICAgICBsZXQgbXV0IGYgPSBzdGQ6OmZzOjpGaWxlOjpjcmVhdGUoZGlyLmpvaW4oIkNhcmdvLnRvbWwiKSkudW53cmFwKCk7CiAgICAgICAgZi53cml0ZV9hbGwoY29udGVudC5hc19ieXRlcygpKS51bndyYXAoKTsKICAgIH0KCiAgICBmbiBzZXR1cF9jcmF0ZShkaXI6ICZzdGQ6OnBhdGg6OlBhdGgpIHsKICAgICAgICBsZXQgc3JjID0gZGlyLmpvaW4oInNyYyIpOwogICAgICAgIHN0ZDo6ZnM6OmNyZWF0ZV9kaXJfYWxsKCZzcmMpLnVud3JhcCgpOwogICAgICAgIHN0ZDo6ZnM6OndyaXRlKHNyYy5qb2luKCJsaWIucnMiKSwgIiIpLnVud3JhcCgpOwogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIGZpbmRfd29ya3NwYWNlX3Jvb3RfZmluZHNfY2FyZ29fdG9tbCgpIHsKICAgICAgICBsZXQgdG1wID0gdGVtcGZpbGU6OnRlbXBkaXIoKS51bndyYXAoKTsKICAgICAgICBsZXQgcGF0aCA9IHRtcC5wYXRoKCkuam9pbigic3ViZGlyIikuam9pbigibmVzdGVkIik7CiAgICAgICAgc3RkOjpmczo6Y3JlYXRlX2Rpcl9hbGwoJnBhdGgpLnVud3JhcCgpOwogICAgICAgIHdyaXRlX2NhcmdvX3RvbWwodG1wLnBhdGgoKSwgIlt3b3Jrc3BhY2VdIik7CiAgICAgICAgbGV0IHJlc3VsdCA9IGZpbmRfd29ya3NwYWNlX3Jvb3QoJnBhdGgpLnVud3JhcCgpOwogICAgICAgIGFzc2VydF9lcSEocmVzdWx0LCB0bXAucGF0aCgpKTsKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiBlbnVtZXJhdGVfbWVtYmVyc19yZXR1cm5zX21lbWJlcnMoKSB7CiAgICAgICAgbGV0IHRtcCA9IHRlbXBmaWxlOjp0ZW1wZGlyKCkudW53cmFwKCk7CiAgICAgICAgd3JpdGVfY2FyZ29fdG9tbCh0bXAucGF0aCgpLCByIyIKW3dvcmtzcGFjZV0KbWVtYmVycyA9IFsiY3JhdGVfYSIsICJjcmF0ZV9iIl0KIiMpOwogICAgICAgIHNldHVwX2NyYXRlKHRtcC5wYXRoKCkuam9pbigiY3JhdGVfYSIpLmFzX3BhdGgoKSk7CiAgICAgICAgc2V0dXBfY3JhdGUodG1wLnBhdGgoKS5qb2luKCJjcmF0ZV9iIikuYXNfcGF0aCgpKTsKICAgICAgICBsZXQgbWVtYmVycyA9IGVudW1lcmF0ZV9tZW1iZXJzKHRtcC5wYXRoKCkpLnVud3JhcCgpOwogICAgICAgIGFzc2VydF9lcSEobWVtYmVycy5sZW4oKSwgMik7CiAgICB9CgogICAgI1t0ZXN0XQogICAgZm4gZW51bWVyYXRlX21lbWJlcnNfcmV0dXJuc19lcnJfZm9yX21pc3Npbmdfd29ya3NwYWNlKCkgewogICAgICAgIGxldCB0bXAgPSB0ZW1wZmlsZTo6dGVtcGRpcigpLnVud3JhcCgpOwogICAgICAgIHdyaXRlX2NhcmdvX3RvbWwodG1wLnBhdGgoKSwgIltkZXBlbmRlbmNpZXNdXG5mb28gPSBcIjFcIiIpOwogICAgICAgIGxldCByZXN1bHQgPSBlbnVtZXJhdGVfbWVtYmVycyh0bXAucGF0aCgpKTsKICAgICAgICBhc3NlcnQhKHJlc3VsdC5pc19lcnIoKSk7CiAgICAgICAgbWF0Y2ggcmVzdWx0LnVud3JhcF9lcnIoKSB7CiAgICAgICAgICAgIEVycm9yOjpNaXNzaW5nV29ya3NwYWNlU2VjdGlvbiA9PiB7fSwKICAgICAgICAgICAgb3RoZXIgPT4gcGFuaWMhKCJleHBlY3RlZCBNaXNzaW5nV29ya3NwYWNlU2VjdGlvbiwgZ290IHs6P30iLCBvdGhlciksCiAgICAgICAgfQogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIGVudW1lcmF0ZV9tZW1iZXJzX2FwcGxpZXNfZXhjbHVkZSgpIHsKICAgICAgICBsZXQgdG1wID0gdGVtcGZpbGU6OnRlbXBkaXIoKS51bndyYXAoKTsKICAgICAgICB3cml0ZV9jYXJnb190b21sKHRtcC5wYXRoKCksIHIjIgpbd29ya3NwYWNlXQptZW1iZXJzID0gWyJhIiwgImIiLCAiYyJdCmV4Y2x1ZGUgPSBbImIiXQoiIyk7CiAgICAgICAgc2V0dXBfY3JhdGUodG1wLnBhdGgoKS5qb2luKCJhIikuYXNfcGF0aCgpKTsKICAgICAgICBzZXR1cF9jcmF0ZSh0bXAucGF0aCgpLmpvaW4oImIiKS5hc19wYXRoKCkpOwogICAgICAgIHNldHVwX2NyYXRlKHRtcC5wYXRoKCkuam9pbigiYyIpLmFzX3BhdGgoKSk7CiAgICAgICAgbGV0IG1lbWJlcnMgPSBlbnVtZXJhdGVfbWVtYmVycyh0bXAucGF0aCgpKS51bndyYXAoKTsKICAgICAgICBsZXQgbmFtZXM6IFZlYzxfPiA9IG1lbWJlcnMuaXRlcigpLm1hcCh8cHwgcC5maWxlX25hbWUoKS51bndyYXAoKS50b19zdHJpbmdfbG9zc3koKSkuY29sbGVjdCgpOwogICAgICAgIGFzc2VydCEobmFtZXMuY29udGFpbnMoJiJhIi5hc19yZWYoKSkpOwogICAgICAgIGFzc2VydCEoIW5hbWVzLmNvbnRhaW5zKCYiYiIuYXNfcmVmKCkpKTsKICAgICAgICBhc3NlcnQhKG5hbWVzLmNvbnRhaW5zKCYiYyIuYXNfcmVmKCkpKTsKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiByZXNvbHZlX2NyYXRlX3Jvb3RzX2RldGVjdHNfbGliKCkgewogICAgICAgIGxldCB0bXAgPSB0ZW1wZmlsZTo6dGVtcGRpcigpLnVud3JhcCgpOwogICAgICAgIHN0ZDo6ZnM6OmNyZWF0ZV9kaXJfYWxsKHRtcC5wYXRoKCkuam9pbigic3JjIikpLnVud3JhcCgpOwogICAgICAgIHN0ZDo6ZnM6OndyaXRlKHRtcC5wYXRoKCkuam9pbigic3JjIikuam9pbigibGliLnJzIiksICIiKS51bndyYXAoKTsKICAgICAgICBsZXQgcm9vdHMgPSByZXNvbHZlX2NyYXRlX3Jvb3RzKHRtcC5wYXRoKCkpOwogICAgICAgIGFzc2VydF9lcSEocm9vdHMubGVuKCksIDEpOwogICAgICAgIGFzc2VydF9lcSEocm9vdHNbMF0uMSwgQ3JhdGVUeXBlOjpMaWIpOwogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIHJlc29sdmVfY3JhdGVfcm9vdHNfZGV0ZWN0c19iaW4oKSB7CiAgICAgICAgbGV0IHRtcCA9IHRlbXBmaWxlOjp0ZW1wZGlyKCkudW53cmFwKCk7CiAgICAgICAgc3RkOjpmczo6Y3JlYXRlX2Rpcl9hbGwodG1wLnBhdGgoKS5qb2luKCJzcmMiKSkudW53cmFwKCk7CiAgICAgICAgc3RkOjpmczo6d3JpdGUodG1wLnBhdGgoKS5qb2luKCJzcmMiKS5qb2luKCJtYWluLnJzIiksICIiKS51bndyYXAoKTsKICAgICAgICBsZXQgcm9vdHMgPSByZXNvbHZlX2NyYXRlX3Jvb3RzKHRtcC5wYXRoKCkpOwogICAgICAgIGFzc2VydF9lcSEocm9vdHMubGVuKCksIDEpOwogICAgICAgIGFzc2VydF9lcSEocm9vdHNbMF0uMSwgQ3JhdGVUeXBlOjpCaW4pOwogICAgfQp9", "target": "src/workspace.rs", "index": 2, "is_create": false}, {"before_b64": "ICAgIE9rKChwYWNrYWdlLCBkZXBzKSkKfQ==", "after_b64": "ICAgIE9rKChwYWNrYWdlLCBkZXBzKSkKfQoKLy8g4pSA4pSAIFRlc3RzIOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgAoKI1tjZmcodGVzdCldCm1vZCB0ZXN0cyB7CiAgICB1c2Ugc3VwZXI6Oio7CiAgICB1c2Ugc3RkOjppbzo6V3JpdGU7CgogICAgZm4gd3JpdGVfY2FyZ29fdG9tbChkaXI6ICZzdGQ6OnBhdGg6OlBhdGgsIGNvbnRlbnQ6ICZzdHIpIHsKICAgICAgICBsZXQgbXV0IGYgPSBzdGQ6OmZzOjpGaWxlOjpjcmVhdGUoZGlyLmpvaW4oIkNhcmdvLnRvbWwiKSkudW53cmFwKCk7CiAgICAgICAgZi53cml0ZV9hbGwoY29udGVudC5hc19ieXRlcygpKS51bndyYXAoKTsKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiBwYXJzZV9jYXJnb190b21sX3BhcnNlc19taW5pbWFsKCkgewogICAgICAgIGxldCB0bXAgPSB0ZW1wZmlsZTo6dGVtcGRpcigpLnVud3JhcCgpOwogICAgICAgIHdyaXRlX2NhcmdvX3RvbWwodG1wLnBhdGgoKSwgciMiCltwYWNrYWdlXQpuYW1lID0gInRlc3QtcGtnIgp2ZXJzaW9uID0gIjEuMC4wIgplZGl0aW9uID0gIjIwMjEiCiIjKTsKICAgICAgICBsZXQgKHBrZywgX2RlcHMpID0gcGFyc2VfY2FyZ29fdG9tbCh0bXAucGF0aCgpLmpvaW4oIkNhcmdvLnRvbWwiKS5hc19wYXRoKCkpLnVud3JhcCgpOwogICAgICAgIGFzc2VydF9lcSEocGtnLm5hbWUsICJ0ZXN0LXBrZyIpOwogICAgICAgIGFzc2VydF9lcSEocGtnLnZlcnNpb24sICIxLjAuMCIpOwogICAgICAgIGFzc2VydF9lcSEocGtnLmVkaXRpb24sICIyMDIxIik7CiAgICB9CgogICAgI1t0ZXN0XQogICAgZm4gcGFyc2VfY2FyZ29fdG9tbF91c2VzX2RlZmF1bHRzX2Zvcl9taXNzaW5nX3BhY2thZ2UoKSB7CiAgICAgICAgbGV0IHRtcCA9IHRlbXBmaWxlOjp0ZW1wZGlyKCkudW53cmFwKCk7CiAgICAgICAgd3JpdGVfY2FyZ29fdG9tbCh0bXAucGF0aCgpLCAiIik7CiAgICAgICAgbGV0IChwa2csIF9kZXBzKSA9IHBhcnNlX2NhcmdvX3RvbWwodG1wLnBhdGgoKS5qb2luKCJDYXJnby50b21sIikuYXNfcGF0aCgpKS51bndyYXAoKTsKICAgICAgICBhc3NlcnRfZXEhKHBrZy5uYW1lLCAidW5rbm93biIpOwogICAgICAgIGFzc2VydF9lcSEocGtnLmVkaXRpb24sICIyMDIxIik7CiAgICB9CgogICAgI1t0ZXN0XQogICAgZm4gcGFyc2VfY2FyZ29fdG9tbF9kaXN0aW5ndWlzaGVzX2RlcHMoKSB7CiAgICAgICAgbGV0IHRtcCA9IHRlbXBmaWxlOjp0ZW1wZGlyKCkudW53cmFwKCk7CiAgICAgICAgd3JpdGVfY2FyZ29fdG9tbCh0bXAucGF0aCgpLCByIyIKW3BhY2thZ2VdCm5hbWUgPSAidGVzdC1wa2ciCnZlcnNpb24gPSAiMC4xLjAiCmVkaXRpb24gPSAiMjAyMSIKCltkZXBlbmRlbmNpZXNdCmZvbyA9ICIxIgpiYXIgPSB7IHdvcmtzcGFjZSA9IHRydWUgfQoKW2Rldi1kZXBlbmRlbmNpZXNdCmJheiA9ICIyIgpxdXggPSB7IHdvcmtzcGFjZSA9IHRydWUgfQoiIyk7CiAgICAgICAgbGV0IChfLCBkZXBzKSA9IHBhcnNlX2NhcmdvX3RvbWwodG1wLnBhdGgoKS5qb2luKCJDYXJnby50b21sIikuYXNfcGF0aCgpKS51bndyYXAoKTsKICAgICAgICBhc3NlcnRfZXEhKGRlcHMubm9ybWFsLCB2ZWMhWyJmb28iXSk7CiAgICAgICAgYXNzZXJ0X2VxIShkZXBzLmRldiwgdmVjIVsiYmF6Il0pOwogICAgICAgIGFzc2VydCEoZGVwcy53b3Jrc3BhY2VfbWVtYmVycy5jb250YWlucygmImJhciIudG9fc3RyaW5nKCkpKTsKICAgICAgICBhc3NlcnQhKGRlcHMud29ya3NwYWNlX21lbWJlcnMuY29udGFpbnMoJiJxdXgiLnRvX3N0cmluZygpKSk7CiAgICB9Cn0=", "target": "src/cargo_info.rs", "index": 3, "is_create": false}, {"before_b64": "ICAgIENyb3NzUmVmZXJlbmNlcyB7IHR5cGVzOiB0eXBlc19tYXAgfQp9CgovLyBIZWxwZXI6IGNvbnZlcnQgSXRlbUtpbmQgdG8gYSBzaG9ydCBzdHJpbmcgZm9yIHRoZSBUeXBlUmVmLmtpbmQgZmllbGQu", "after_b64": "ICAgIENyb3NzUmVmZXJlbmNlcyB7IHR5cGVzOiB0eXBlc19tYXAgfQp9CgovLyDilIDilIAgVGVzdHMg4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSACgojW2NmZyh0ZXN0KV0KbW9kIHRlc3RzIHsKICAgIHVzZSBzdXBlcjo6KjsKICAgIHVzZSBjcmF0ZTo6c2NoZW1hOjp7TW9kdWxlSW5mbywgUHVibGljSXRlbSwgU3VibW9kdWxlRGVjbH07CgogICAgZm4gbWFrZV9jcmF0ZShuYW1lOiAmc3RyLCBpdGVtczogVmVjPChTdHJpbmcsIEl0ZW1LaW5kKT4pIC0+IENyYXRlSW5mbyB7CiAgICAgICAgbGV0IHB1YmxpY19pdGVtczogVmVjPFB1YmxpY0l0ZW0+ID0gaXRlbXMKICAgICAgICAgICAgLmludG9faXRlcigpCiAgICAgICAgICAgIC5tYXAofChuLCBrKXwgewogICAgICAgICAgICAgICAgUHVibGljSXRlbTo6YnVpbGRlcigpCiAgICAgICAgICAgICAgICAgICAgLmtpbmQoaykKICAgICAgICAgICAgICAgICAgICAubmFtZShuKQogICAgICAgICAgICAgICAgICAgIC5maWxlKFN0cmluZzo6bmV3KCkpCiAgICAgICAgICAgICAgICAgICAgLmxpbmUoMSkKICAgICAgICAgICAgICAgICAgICAudmlzaWJpbGl0eSgicHViIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgICAgICAgICAuZ2VuZXJpY3MoU3RyaW5nOjpuZXcoKSkKICAgICAgICAgICAgICAgICAgICAuYXR0cnMoRGVmYXVsdDo6ZGVmYXVsdCgpKQogICAgICAgICAgICAgICAgICAgIC5idWlsZCgpCiAgICAgICAgICAgIH0pCiAgICAgICAgICAgIC5jb2xsZWN0KCk7CiAgICAgICAgbGV0IG1vZHVsZSA9IE1vZHVsZUluZm86OmJ1aWxkZXIoKQogICAgICAgICAgICAucGF0aCgiIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgLmZpbGUoU3RyaW5nOjpuZXcoKSkKICAgICAgICAgICAgLnZpc2liaWxpdHkoInB1YiIudG9fc3RyaW5nKCkpCiAgICAgICAgICAgIC5wdWJsaWNfaXRlbXMocHVibGljX2l0ZW1zKQogICAgICAgICAgICAuYnVpbGQoKTsKICAgICAgICBDcmF0ZUluZm86OmJ1aWxkZXIoKQogICAgICAgICAgICAubmFtZShuYW1lLnRvX3N0cmluZygpKQogICAgICAgICAgICAucm9vdChTdHJpbmc6Om5ldygpKQogICAgICAgICAgICAucGFja2FnZSgKICAgICAgICAgICAgICAgIHNjaGVtYTo6UGFja2FnZUluZm86OmJ1aWxkZXIoKQogICAgICAgICAgICAgICAgICAgIC5uYW1lKG5hbWUudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAgICAgLnZlcnNpb24oIjAuMS4wIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgICAgICAgICAuZWRpdGlvbigiMjAyMSIudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAgICAgLmNyYXRlX3R5cGUoc2NoZW1hOjpDcmF0ZVR5cGU6OkxpYikKICAgICAgICAgICAgICAgICAgICAuYnVpbGQoKSwKICAgICAgICAgICAgKQogICAgICAgICAgICAubW9kdWxlcyh2ZWMhW21vZHVsZV0pCiAgICAgICAgICAgIC5kZXBzKERlZmF1bHQ6OmRlZmF1bHQoKSkKICAgICAgICAgICAgLmJ1aWxkKCkKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiBjb21wdXRlX2ZpbmRzX2Nyb3NzX2NyYXRlX2ltcG9ydCgpIHsKICAgICAgICBsZXQgbXV0IGNyYXRlcyA9IHZlYyFbCiAgICAgICAgICAgIG1ha2VfY3JhdGUoImNvcmUiLCB2ZWMhWwogICAgICAgICAgICAgICAgKCJUYXNrIi50b19zdHJpbmcoKSwgSXRlbUtpbmQ6OlN0cnVjdCksCiAgICAgICAgICAgIF0pLAogICAgICAgICAgICBtYWtlX2NyYXRlKCJlbmdpbmUiLCB2ZWMhW10pLAogICAgICAgIF07CiAgICAgICAgLy8gTWFudWFsbHkgYWRkIGFuIGltcG9ydCBpbiBlbmdpbmUgdGhhdCByZWZlcmVuY2VzIGNvcmU6OlRhc2sKICAgICAgICBsZXQgZW5naW5lX21vZHVsZSA9ICZtdXQgY3JhdGVzWzFdLm1vZHVsZXNbMF07CiAgICAgICAgZW5naW5lX21vZHVsZS5pbXBvcnRzLnB1c2goSW1wb3J0IHsKICAgICAgICAgICAgcGF0aDogImNvcmU6OlRhc2siLnRvX3N0cmluZygpLAogICAgICAgICAgICBsaW5lOiAxLAogICAgICAgIH0pOwogICAgICAgIGxldCByZWZzID0gY29tcHV0ZSgmbXV0IGNyYXRlcyk7CiAgICAgICAgLy8gVGFzayBzaG91bGQgYmUgaW4gY3Jvc3MtcmVmZXJlbmNlcwogICAgICAgIGFzc2VydCEocmVmcy50eXBlcy5jb250YWluc19rZXkoIlRhc2siKSk7CiAgICAgICAgbGV0IHRhc2tfcmVmID0gJnJlZnMudHlwZXNbIlRhc2siXTsKICAgICAgICBhc3NlcnRfZXEhKHRhc2tfcmVmLmNyYXRlX25hbWUsICJjb3JlIik7CiAgICAgICAgLy8gZW5naW5lIHNob3VsZCBoYXZlIGEgY3Jvc3NfY3JhdGVfaW1wb3J0CiAgICAgICAgYXNzZXJ0X2VxIShjcmF0ZXNbMV0uY3Jvc3NfY3JhdGVfaW1wb3J0cy5sZW4oKSwgMSk7CiAgICAgICAgYXNzZXJ0X2VxIShjcmF0ZXNbMV0uY3Jvc3NfY3JhdGVfaW1wb3J0c1swXS50YXJnZXRfY3JhdGUsICJjb3JlIik7CiAgICB9CgogICAgI1t0ZXN0XQogICAgZm4gY29tcHV0ZV9lbXB0eV9mb3Jfbm9fY3Jvc3NfcmVmZXJlbmNlcygpIHsKICAgICAgICBsZXQgY3JhdGVzID0gdmVjIVsKICAgICAgICAgICAgbWFrZV9jcmF0ZSgiYSIsIHZlYyFbKCJGb28iLnRvX3N0cmluZygpLCBJdGVtS2luZDo6U3RydWN0KV0pLAogICAgICAgICAgICBtYWtlX2NyYXRlKCJiIiwgdmVjIVsoIkJhciIudG9fc3RyaW5nKCksIEl0ZW1LaW5kOjpTdHJ1Y3QpXSksCiAgICAgICAgXTsKICAgICAgICBsZXQgbXV0IGNyYXRlc19tdXQgPSBjcmF0ZXM7CiAgICAgICAgbGV0IHJlZnMgPSBjb21wdXRlKCZtdXQgY3JhdGVzX211dCk7CiAgICAgICAgLy8gTm8gY3Jvc3MgcmVmZXJlbmNlcyBzaW5jZSBubyBjcmF0ZSBpbXBvcnRzIGZyb20gYW5vdGhlcgogICAgICAgIGFzc2VydCEocmVmcy50eXBlcy5pc19lbXB0eSgpIHx8IHJlZnMudHlwZXMudmFsdWVzKCkuYWxsKHx0fCB0LmltcG9ydGVkX2J5LmlzX2VtcHR5KCkpKTsKICAgIH0KfQoKLy8gSGVscGVyOiBjb252ZXJ0IEl0ZW1LaW5kIHRvIGEgc2hvcnQgc3RyaW5nIGZvciB0aGUgVHlwZVJlZi5raW5kIGZpZWxkLg==", "target": "src/cross_refs.rs", "index": 4, "is_create": false}, {"before_b64": "dXNlIGNyYXRlOjpzY2hlbWE6OldvcmtzcGFjZU1hcDsKdXNlIHN0ZDo6aW86OldyaXRlOwoKLy8vIFNlcmlhbGl6ZSB0aGUgd29ya3NwYWNlIG1hcCB0byBhIEpTT04gc3RyaW5nIHdpdGggMi1zcGFjZSBpbmRlbnRhdGlvbi4KI1ttdXN0X3VzZV0KcHViIGZuIHJlbmRlcl9qc29uKG1hcDogJldvcmtzcGFjZU1hcCkgLT4gc2VyZGVfanNvbjo6UmVzdWx0PFN0cmluZz4gewogICAgc2VyZGVfanNvbjo6dG9fc3RyaW5nX3ByZXR0eShtYXApCn0KCi8vLyBTZXJpYWxpemUgdGhlIHdvcmtzcGFjZSBtYXAgdG8gdGhlIGdpdmVuIHdyaXRlci4KcHViIGZuIHJlbmRlcl90b193cml0ZXIobWFwOiAmV29ya3NwYWNlTWFwLCB3cml0ZXI6IGltcGwgV3JpdGUpIC0+IHNlcmRlX2pzb246OlJlc3VsdDwoKT4gewogICAgc2VyZGVfanNvbjo6dG9fd3JpdGVyX3ByZXR0eSh3cml0ZXIsIG1hcCkKfQo=", "after_b64": "dXNlIGNyYXRlOjpzY2hlbWE6OldvcmtzcGFjZU1hcDsKdXNlIHN0ZDo6aW86OldyaXRlOwoKLy8vIFNlcmlhbGl6ZSB0aGUgd29ya3NwYWNlIG1hcCB0byBhIEpTT04gc3RyaW5nIHdpdGggMi1zcGFjZSBpbmRlbnRhdGlvbi4KI1ttdXN0X3VzZV0KcHViIGZuIHJlbmRlcl9qc29uKG1hcDogJldvcmtzcGFjZU1hcCkgLT4gc2VyZGVfanNvbjo6UmVzdWx0PFN0cmluZz4gewogICAgc2VyZGVfanNvbjo6dG9fc3RyaW5nX3ByZXR0eShtYXApCn0KCi8vLyBTZXJpYWxpemUgdGhlIHdvcmtzcGFjZSBtYXAgdG8gdGhlIGdpdmVuIHdyaXRlci4KcHViIGZuIHJlbmRlcl90b193cml0ZXIobWFwOiAmV29ya3NwYWNlTWFwLCB3cml0ZXI6IGltcGwgV3JpdGUpIC0+IHNlcmRlX2pzb246OlJlc3VsdDwoKT4gewogICAgc2VyZGVfanNvbjo6dG9fd3JpdGVyX3ByZXR0eSh3cml0ZXIsIG1hcCkKfQoKLy8g4pSA4pSAIFRlc3RzIOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgAoKI1tjZmcodGVzdCldCm1vZCB0ZXN0cyB7CiAgICB1c2Ugc3VwZXI6Oio7CiAgICB1c2UgY3JhdGU6OnNjaGVtYTo6ewogICAgICAgIENyYXRlSW5mbywgQ3JhdGVUeXBlLCBDcm9zc1JlZmVyZW5jZXMsIERlcEluZm8sIE1vZHVsZUluZm8sIFBhY2thZ2VJbmZvLAogICAgICAgIFdvcmtzcGFjZUluZm8sIFdvcmtzcGFjZU1hcCwKICAgIH07CgogICAgZm4gbWFrZV9taW5pbWFsX21hcCgpIC0+IFdvcmtzcGFjZU1hcCB7CiAgICAgICAgV29ya3NwYWNlTWFwOjpidWlsZGVyKCkKICAgICAgICAgICAgLndvcmtzcGFjZShXb3Jrc3BhY2VJbmZvOjpidWlsZGVyKCkKICAgICAgICAgICAgICAgIC5yb290KCIuIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgICAgIC53b3Jrc3BhY2VfbmFtZSgidGVzdCIudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAuYnVpbGQoKSkKICAgICAgICAgICAgLmNyYXRlcyh2ZWMhWwogICAgICAgICAgICAgICAgQ3JhdGVJbmZvOjpidWlsZGVyKCkKICAgICAgICAgICAgICAgICAgICAubmFtZSgidGVzdC1jcmF0ZSIudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAgICAgLnJvb3QoIi4iLnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgICAgIC5wYWNrYWdlKFBhY2thZ2VJbmZvOjpidWlsZGVyKCkKICAgICAgICAgICAgICAgICAgICAgICAgLm5hbWUoInRlc3QtY3JhdGUiLnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgICAgICAgICAudmVyc2lvbigiMC4xLjAiLnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgICAgICAgICAuZWRpdGlvbigiMjAyMSIudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAgICAgICAgIC5jcmF0ZV90eXBlKENyYXRlVHlwZTo6TGliKQogICAgICAgICAgICAgICAgICAgICAgICAuYnVpbGQoKSkKICAgICAgICAgICAgICAgICAgICAubW9kdWxlcyh2ZWMhW01vZHVsZUluZm86OmJ1aWxkZXIoKQogICAgICAgICAgICAgICAgICAgICAgICAucGF0aCgiIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgICAgICAgICAgICAgLmZpbGUoInNyYy9saWIucnMiLnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgICAgICAgICAudmlzaWJpbGl0eSgicHViIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgICAgICAgICAgICAgLmJ1aWxkKCldKQogICAgICAgICAgICAgICAgICAgIC5kZXBzKERlcEluZm86OmRlZmF1bHQoKSkKICAgICAgICAgICAgICAgICAgICAuYnVpbGQoKSwKICAgICAgICAgICAgXSkKICAgICAgICAgICAgLmNyb3NzX3JlZmVyZW5jZXMoQ3Jvc3NSZWZlcmVuY2VzOjpkZWZhdWx0KCkpCiAgICAgICAgICAgIC53b3Jrc3BhY2Vfcm9vdChzdGQ6OnBhdGg6OlBhdGhCdWY6OmZyb20oIi4iKSkKICAgICAgICAgICAgLmJ1aWxkKCkKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiByZW5kZXJfanNvbl9wcm9kdWNlc192YWxpZF9qc29uKCkgewogICAgICAgIGxldCBtYXAgPSBtYWtlX21pbmltYWxfbWFwKCk7CiAgICAgICAgbGV0IGpzb24gPSByZW5kZXJfanNvbigmbWFwKS51bndyYXAoKTsKICAgICAgICBsZXQgcGFyc2VkOiBzZXJkZV9qc29uOjpWYWx1ZSA9IHNlcmRlX2pzb246OmZyb21fc3RyKCZqc29uKS51bndyYXAoKTsKICAgICAgICBhc3NlcnRfZXEhKHBhcnNlZFsid29ya3NwYWNlIl1bInJvb3QiXSwgIi4iKTsKICAgICAgICBhc3NlcnRfZXEhKHBhcnNlZFsiY3JhdGVzIl0uYXNfYXJyYXkoKS51bndyYXAoKS5sZW4oKSwgMSk7CiAgICB9CgogICAgI1t0ZXN0XQogICAgZm4gcmVuZGVyX2pzb25fc2tpcHNfZW1wdHlfZXJyb3JzKCkgewogICAgICAgIGxldCBtYXAgPSBtYWtlX21pbmltYWxfbWFwKCk7CiAgICAgICAgbGV0IGpzb24gPSByZW5kZXJfanNvbigmbWFwKS51bndyYXAoKTsKICAgICAgICBsZXQgcGFyc2VkOiBzZXJkZV9qc29uOjpWYWx1ZSA9IHNlcmRlX2pzb246OmZyb21fc3RyKCZqc29uKS51bndyYXAoKTsKICAgICAgICAvLyBlcnJvcnMgZmllbGQgc2hvdWxkIGJlIGFic2VudCAoc2tpcF9zZXJpYWxpemluZ19pZikKICAgICAgICBhc3NlcnQhKHBhcnNlZC5nZXQoImVycm9ycyIpLmlzX25vbmUoKSk7CiAgICB9CgogICAgI1t0ZXN0XQogICAgZm4gcmVuZGVyX3RvX3dyaXRlcl9tYXRjaGVzX3JlbmRlcl9qc29uKCkgewogICAgICAgIGxldCBtYXAgPSBtYWtlX21pbmltYWxfbWFwKCk7CiAgICAgICAgbGV0IGpzb24gPSByZW5kZXJfanNvbigmbWFwKS51bndyYXAoKTsKCiAgICAgICAgbGV0IG11dCBidWYgPSBWZWM6Om5ldygpOwogICAgICAgIHJlbmRlcl90b193cml0ZXIoJm1hcCwgJm11dCBidWYpLnVud3JhcCgpOwogICAgICAgIGxldCBmcm9tX3dyaXRlciA9IFN0cmluZzo6ZnJvbV91dGY4KGJ1ZikudW53cmFwKCk7CgogICAgICAgIGFzc2VydF9lcSEoanNvbiwgZnJvbV93cml0ZXIpOwogICAgfQp9Cg==", "target": "src/render.rs", "index": 5, "is_create": false}]')
-
-for step in STEPS:
-    before = base64.b64decode(step["before_b64"]).decode()
-    after = base64.b64decode(step["after_b64"]).decode()
-    target = step["target"]
-    idx = step["index"]
-    is_create = step["is_create"]
-
-    if is_create:
-        target_path = Path(target)
-        target_path.parent.mkdir(parents=True, exist_ok=True)
-        target_path.write_text(after)
-        print(f"OK {TASK_ID} change {idx}: created {target}")
-    else:
-        target_path = Path(target)
-        content = target_path.read_text()
-        if before not in content:
-            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
-            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
-            sys.exit(1)
-
-        result = subprocess.run(
-            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
-            capture_output=True, text=True,
-        )
-        if result.returncode != 0:
-            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
-            sys.exit(result.returncode)
-
-        new_content = target_path.read_text()
-        if after and after not in new_content:
-            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
-            sys.exit(1)
-
-        print(f"OK {TASK_ID} change {idx}: applied to {target}")
-
-print(f"OK {TASK_ID}: all changes applied")
diff --git a/plans/compiled/TASK-9.sh b/plans/compiled/TASK-9.sh
deleted file mode 100755
index d78aa7c..0000000
--- a/plans/compiled/TASK-9.sh
+++ /dev/null
@@ -1,7 +0,0 @@
-#!/usr/bin/env bash
-set -euo pipefail
-# TASK-9: Add unit tests for file_parser, module_tree, workspace, cargo_info, cross_refs, and render
-# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
-# Type: replace
-# File: src/file_parser.rs
-python3 "$(dirname "$0")/TASK-9.py"
diff --git a/plans/compiled/TASK-PREP.py b/plans/compiled/TASK-PREP.py
deleted file mode 100644
index d453fa4..0000000
--- a/plans/compiled/TASK-PREP.py
+++ /dev/null
@@ -1,44 +0,0 @@
-#!/usr/bin/env python3
-"""TASK-PREP: Add tempfile dev-dependency for unit and integration tests"""
-import base64, json, subprocess, sys
-from pathlib import Path
-
-TASK_ID = "TASK-PREP"
-STEPS = json.loads('[{"before_b64": "cHJvYy1tYWNybzIgPSB7IHZlcnNpb24gPSAiMSIsIGZlYXR1cmVzID0gWyJzcGFuLWxvY2F0aW9ucyJdIH0=", "after_b64": "cHJvYy1tYWNybzIgPSB7IHZlcnNpb24gPSAiMSIsIGZlYXR1cmVzID0gWyJzcGFuLWxvY2F0aW9ucyJdIH0KCltkZXYtZGVwZW5kZW5jaWVzXQp0ZW1wZmlsZSA9ICIzIg==", "target": "Cargo.toml", "index": 0, "is_create": false}]')
-
-for step in STEPS:
-    before = base64.b64decode(step["before_b64"]).decode()
-    after = base64.b64decode(step["after_b64"]).decode()
-    target = step["target"]
-    idx = step["index"]
-    is_create = step["is_create"]
-
-    if is_create:
-        target_path = Path(target)
-        target_path.parent.mkdir(parents=True, exist_ok=True)
-        target_path.write_text(after)
-        print(f"OK {TASK_ID} change {idx}: created {target}")
-    else:
-        target_path = Path(target)
-        content = target_path.read_text()
-        if before not in content:
-            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
-            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
-            sys.exit(1)
-
-        result = subprocess.run(
-            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
-            capture_output=True, text=True,
-        )
-        if result.returncode != 0:
-            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
-            sys.exit(result.returncode)
-
-        new_content = target_path.read_text()
-        if after and after not in new_content:
-            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
-            sys.exit(1)
-
-        print(f"OK {TASK_ID} change {idx}: applied to {target}")
-
-print(f"OK {TASK_ID}: all changes applied")
diff --git a/plans/compiled/TASK-PREP.sh b/plans/compiled/TASK-PREP.sh
deleted file mode 100755
index a037be6..0000000
--- a/plans/compiled/TASK-PREP.sh
+++ /dev/null
@@ -1,7 +0,0 @@
-#!/usr/bin/env bash
-set -euo pipefail
-# TASK-PREP: Add tempfile dev-dependency for unit and integration tests
-# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
-# Type: replace
-# File: Cargo.toml
-python3 "$(dirname "$0")/TASK-PREP.py"
diff --git a/plans/compiled/manifest.json b/plans/compiled/manifest.json
deleted file mode 100644
index 94a624c..0000000
--- a/plans/compiled/manifest.json
+++ /dev/null
@@ -1,139 +0,0 @@
-{
-  "plan": "/Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml",
-  "compiled_at": "2026-04-28T08:48:13.087314+00:00",
-  "tasks": [
-    {
-      "id": "TASK-PREP",
-      "script": "TASK-PREP.sh",
-      "runner": "TASK-PREP.py",
-      "file": "Cargo.toml",
-      "type": "replace",
-      "changes": 1,
-      "description": "Add tempfile dev-dependency for unit and integration tests",
-      "acceptance": [
-        "cargo check -p rust-workspace-map"
-      ]
-    },
-    {
-      "id": "TASK-1",
-      "script": "TASK-1.sh",
-      "runner": "TASK-1.py",
-      "file": "src/schema.rs",
-      "type": "replace",
-      "changes": 1,
-      "description": "Add ErrorSeverity enum and ErrorContext struct to schema.rs",
-      "acceptance": [
-        "cargo check -p rust-workspace-map"
-      ]
-    },
-    {
-      "id": "TASK-2",
-      "script": "TASK-2.sh",
-      "runner": "TASK-2.py",
-      "file": "src/schema.rs",
-      "type": "replace",
-      "changes": 1,
-      "description": "Add MissingWorkspaceSection variant to Error enum in schema.rs",
-      "acceptance": [
-        "cargo check -p rust-workspace-map"
-      ]
-    },
-    {
-      "id": "TASK-3",
-      "script": "TASK-3.sh",
-      "runner": "TASK-3.py",
-      "file": "src/schema.rs",
-      "type": "replace",
-      "changes": 1,
-      "description": "Expand ErrorEntry struct with severity, kind, context, and cause fields",
-      "acceptance": [
-        "cargo check -p rust-workspace-map"
-      ]
-    },
-    {
-      "id": "TASK-4",
-      "script": "TASK-4.sh",
-      "runner": "TASK-4.py",
-      "file": "src/file_parser.rs",
-      "type": "replace",
-      "changes": 1,
-      "description": "Change parse_file to return ParsedFile with optional parse errors instead of Result",
-      "acceptance": [
-        "true  # applied atomically with TASK-5; compilation verified at TASK-5"
-      ]
-    },
-    {
-      "id": "TASK-5",
-      "script": "TASK-5.sh",
-      "runner": "TASK-5.py",
-      "file": "src/module_tree.rs",
-      "type": "replace",
-      "changes": 12,
-      "description": "Refactor module_tree.rs: fix path fallbacks, add error collection, change build_module_tree return type",
-      "acceptance": [
-        "cargo check -p rust-workspace-map"
-      ]
-    },
-    {
-      "id": "TASK-6",
-      "script": "TASK-6.sh",
-      "runner": "TASK-6.py",
-      "file": "src/workspace.rs",
-      "type": "replace",
-      "changes": 3,
-      "description": "Update workspace.rs enumerate_members to return MissingWorkspaceSection error",
-      "acceptance": [
-        "cargo check -p rust-workspace-map"
-      ]
-    },
-    {
-      "id": "TASK-7",
-      "script": "TASK-7.sh",
-      "runner": "TASK-7.py",
-      "file": "src/lib.rs",
-      "type": "replace",
-      "changes": 3,
-      "description": "Refactor lib.rs run(): parallel error collection, error entry construction, remove unwrap_or_default",
-      "acceptance": [
-        "cargo check -p rust-workspace-map"
-      ]
-    },
-    {
-      "id": "TASK-8",
-      "script": "TASK-8.sh",
-      "runner": "TASK-8.py",
-      "file": "src/lib.rs",
-      "type": "replace",
-      "changes": 21,
-      "description": "Remove all crate-level clippy allow attributes and fix individual lint violations",
-      "acceptance": [
-        "cargo clippy -p rust-workspace-map -- -D warnings"
-      ]
-    },
-    {
-      "id": "TASK-9",
-      "script": "TASK-9.sh",
-      "runner": "TASK-9.py",
-      "file": "src/file_parser.rs",
-      "type": "replace",
-      "changes": 6,
-      "description": "Add unit tests for file_parser, module_tree, workspace, cargo_info, cross_refs, and render",
-      "acceptance": [
-        "cargo test -p rust-workspace-map"
-      ]
-    },
-    {
-      "id": "TASK-10",
-      "script": "TASK-10.sh",
-      "runner": "TASK-10.py",
-      "file": "tests/integration_test.rs",
-      "type": "replace",
-      "changes": 1,
-      "description": "Expand integration tests from 3 to 10 covering error cases, workspace patterns, and output",
-      "acceptance": [
-        "cargo test -p rust-workspace-map --test integration_test"
-      ]
-    }
-  ],
-  "skipped": []
-}
diff --git a/plans/mvp/MVP_PLAN.md b/plans/mvp/MVP_PLAN.md
new file mode 100644
index 0000000..4a20fa0
--- /dev/null
+++ b/plans/mvp/MVP_PLAN.md
@@ -0,0 +1,304 @@
+# Plan: `rust-workspace-map` MVP for `rust-development-pipeline` integration
+
+## Context
+
+`rust-workspace-map` already exists at `/Users/tony/programming/rust-workspace-map/` as a single-binary `syn`-based tool that emits a hierarchical JSON map of a Rust workspace's public API surface (`crates → modules → publicItems`). Two feature-request docs in this repo (`feature-requests/rust-workspace-map-agent-centric-design.md`, `feature-requests/rust-workspace-map-agent-centric-development-plan.md`) propose six new features at once: flat indexes, validation, structural diff, compact mode, scope filtering, plus broad pipeline integration.
+
+The user clarified the motivation: this tool is **not a community-facing product**. It is internal infrastructure for `rust-development-pipeline` and exists to **smoothen the pipeline, accelerate it, and reduce token waste**. The two will evolve together.
+
+The user also picked **Option C** in the prior comparison with `tirth8205/code-review-graph` (CRG): build a Rust-native tool independently and adopt CRG's *architectural* lessons (subcommand surface, edge-confidence labeling stance, conservative-recall posture) **without** importing CRG's heavyweight machinery (SQLite store, daemon, MCP server, watch hooks, vector search, eval harness — all explicitly out of MVP scope).
+
+The measured failure data (`feature-requests/reduce_recurring_problems.md`) names three top recurring fix-task categories:
+- **#1 (~40%)** — missing `pub mod` declarations
+- **#2 (~20%)** — missing `pub use` re-exports
+- **#3 (~20%)** — incomplete consumer updates
+
+The MVP's job is to deterministically prevent #1 and #2 at plan-execution time, and structurally support #3 prevention at plan-decomposition time. Everything beyond that is out of MVP scope.
+
+### Core Design Goal: O(1) lookup for LLM agents
+
+This tool exists to answer structural questions about a Rust workspace in **constant time — one hashmap access, no tree traversal, no path guessing**. An LLM agent asking a question should get an answer the way an IDE answers: instant, precise, pre-computed.
+
+The three flat indexes (`symbols`, `name_index`, `files`) pre-compute the relationships a human engineer builds through interactive IDE navigation:
+
+| Agent question | IDE equivalent | Lookup | Cost |
+|---|---|---|---|
+| "Is `Task` a type? Where is it defined?" | Go-to-definition | `name_index["Task"]` → `symbols[path]` | O(1) |
+| "What file contains module `foo::bar`?" | Go-to-file | iterate `files`, match `entry.module_path == "foo::bar"` (or in-memory reverse map built once at index time) | O(1) amortized |
+| "What modules does `lib.rs` declare?" | File structure view | `files["core/src/lib.rs"].module_path` → `crates[].modules[]` join → `submodules` | O(1) hop, then O(1) join |
+| "Is this file reachable from the module tree?" | Module graph view | `files[path].parent_module_file.is_some()` (or file is a crate root) | O(1) |
+| "Which crate exports `Task`?" | (no direct IDE analog) | `cross_references.types["Task"].exported_by` | O(1) — already shipping in 0.1.0 |
+
+This is the fundamental difference from the hierarchical `crates → modules → publicItems` tree alone. That tree requires the agent to traverse, filter, and guess module paths. The flat indexes eliminate traversal entirely — the agent asks one question, makes one lookup, gets one answer. The tool is not a JSON dump for humans to browse; it is a query engine where every query is O(1) and every answer fits in a few hundred tokens.
+
+**Note on what's *not* O(1) in MVP:** "Who imports this symbol?" at file:line granularity is intentionally deferred — the existing `cross_references.types[name].imported_by` answers it at crate granularity, which covers most measured workflows. File:line precision adds extraction cost and per-symbol storage that no current pipeline integration cites. Add when a measured workflow demands it.
+
+---
+
+## Coding style
+
+### Builder pattern
+All new structs (`SymbolEntry`, `FileEntry`) use `#[derive(bon::Builder)]`, consistent with the existing codebase idiom.
+
+### Newtype pattern
+Two newtypes wrap the BTreeMap key strings to prevent passing the wrong key kind at compile time:
+
+```rust
+pub struct CanonicalPath(String);        // "crate::module::Name" — key for symbols, name_index values, cross_references.types
+pub struct WorkspaceRelativePath(String); // "core/src/task.rs"   — key for files
+```
+
+Each implements `Display`, `AsRef<str>`, `From<String>`, `PartialOrd/Ord`, `PartialEq/Eq`, `Hash`, `Clone`, and serde `Serialize`/`Deserialize` (transparent). No other pass-through methods.
+
+### Functional style — iterators over for-loops
+New code in `indexes.rs`, `validate.rs`, and `lookup.rs` uses iterator pipelines (`flat_map`, `filter_map`, `fold`, `collect`) rather than `mut` accumulator loops. Existing code in `module_tree.rs` and `lib.rs` is touched only where the bug fixes require it — no opportunistic refactoring.
+
+### `mem::take` for the O(n²) error-vec fix
+The `errors.clone()` at `module_tree.rs:252` is replaced by passing `&mut Vec<ErrorEntry>` down the recursion and using `extend` in place, eliminating the per-level clone.
+
+### `Option` as iterator
+`Option<T>` fields (e.g. `parent_module_file`) are consumed via `.into_iter()` / `.extend(opt)` / `.chain(opt.iter())` in pipeline contexts rather than `if let Some` unwrapping.
+
+---
+
+## Architectural commitments (locked)
+
+### KISS faithfulness — very faithful
+
+The MVP ships only what closes a measured pipeline failure or directly enables a pipeline integration that closes one. Anything else is recorded as deferred and built only when measured pain re-surfaces it.
+
+### Single binary with subcommands (not multiple binaries)
+
+Decomposition into separate binaries (`rwm-index`, `rwm-validate`) is rejected. Pipeline skills make one tool call per step — they do not chain N tools. A single binary with subcommands (`cargo`, `git`, `gh`, `kubectl` idiom) gives one Cargo build, one install path, one `--help`. The Unix property holds *inside* the binary:
+
+- Each subcommand has one job and writes self-contained JSON to stdout.
+- Subcommands take a workspace path argument. **`--from-stdin` is deferred** — adding it would force `serde::Deserialize` on every schema type plus a JSON-roundtrip stability commitment, and no current pipeline integration pipes (`compile-plan` calls `validate <project>`, plan-decomposer calls `lookup --file`). Revisit when a piping consumer exists.
+- Exit codes carry semantics: `0` clean, `1` tool error, `2` validation found issues (grep convention).
+
+### CLI breaking change — accepted
+
+`index` becomes mandatory in v0.2.0. The bare-path form `rust-workspace-map <PATH>` is removed entirely (not aliased, not deprecation-warned). Internal-only tool, no external users to migrate.
+
+### Cache / MCP / daemon / eval harness — explicitly out of MVP scope
+
+These are CRG-shaped maturity items. Built only when (a) the MVP has shipped, (b) the pipeline is using it for at least 3 phases, and (c) measured pain points justify each one individually. Recorded in §"Out of MVP scope" below.
+
+---
+
+## MVP surface
+
+### CLI
+
+```
+rust-workspace-map index  [PATH] [-o FILE] [--validate]        # --validate runs OrphanFile + DeadReExport checks
+rust-workspace-map lookup [PATH] (--symbol NAME | --file PATH)
+```
+
+### Schema additions
+
+Field additions are additive (no struct fields removed). The `cross_references.types` map key format changes from short-name to fully-qualified path — this is a **breaking change** to the JSON output and is intentional (v0.2.0 breaking-change window, same commit as the `index` subcommand mandate).
+
+Two principles drive the shape:
+
+1. **Reuse `WorkspaceMap.errors: Vec<ErrorEntry>` for new diagnostics.** It already carries `severity: ErrorSeverity::{Error, Warning}`, `kind`, `file`, `line`, `message`, `context`, `cause`. A parallel `warnings: Vec<Warning>` channel duplicates that pipeline; new validate rules emit `ErrorEntry { severity: Warning, kind: DiagnosticKind::OrphanFile, … }` into the existing collection.
+2. **Migrate `ErrorEntry.kind: String → kind: DiagnosticKind`.** Today the field is stringly-typed. The MVP introduces a typed enum covering both existing values (currently passed as strings — `OrphanedModule`, `TomlParseError`, `MissingCrateRoots`, `ModuleTreeError`) and new ones (`OrphanFile`, `DeadReExport`). Serde-rename keeps the JSON output a string for backward compatibility; the change is purely internal typing.
+
+```rust
+// in src/schema.rs
+pub struct WorkspaceMap {
+    pub workspace: WorkspaceInfo,
+    pub crates: Vec<CrateInfo>,
+    pub cross_references: CrossReferences,
+    #[builder(default)]
+    pub symbols:    BTreeMap<String, SymbolEntry>,   // NEW — canonical-path keyed ("crate::module::Name")
+    #[builder(default)]
+    pub name_index: BTreeMap<String, Vec<String>>,   // NEW — short name → list of canonical paths (collision support)
+    #[builder(default)]
+    pub files:      BTreeMap<String, FileEntry>,     // NEW — keyed on workspace-relative file path
+    pub errors: Vec<ErrorEntry>,                     // existing — also receives validate findings
+    pub workspace_root: PathBuf,                     // existing
+}
+
+pub struct SymbolEntry {     // map key = "crate::module::Name"
+    pub crate_name: String,
+    pub module: String,
+    pub file: String,
+    pub line: usize,
+    pub kind: ItemKind,
+    // Deferred until a measured workflow cites them:
+    //   - re_exported_at: Vec<{file, line}>
+    //   - imported_by:    Vec<{crate_name, file, line}>
+    //   - confidence:     EXTRACTED | INFERRED | AMBIGUOUS  (no inference path exists in MVP)
+    // Crate-granularity imported_by/exported_by already lives in CrossReferences.types.
+    // Key format: root-module items = "crate::Name" (2 segments);
+    // nested-module items = "crate::module::Name" (3+ segments).
+}
+
+// The `cross_references.types` map is re-keyed from short-name to fully-qualified path
+// ("crate::module::Name") to avoid silent data loss on name collisions across crates.
+
+pub struct FileEntry {
+    pub module_path: String,                 // joins back to the matching ModuleInfo
+    pub parent_module_file: Option<String>,  // critical for plan-decomposer wiring check
+    pub is_crate_root: bool,
+    // Deliberately *not* duplicating `crate_name`, `submodules`, or `exports` —
+    // they already exist in the hierarchical view via `module_path → ModuleInfo`.
+    // This keeps the `files` index small while still giving agents an O(1) hop
+    // from file path to the structural facts only available through this index.
+    // Inline modules (mod foo { ... }) share their parent's file.
+    // They are excluded from the `files` index to avoid one-file-to-many-module key collisions.
+}
+
+// New typed kind for ErrorEntry. Variants serde-rename to existing string values
+// where applicable so JSON output stays stable.
+pub enum DiagnosticKind {
+    // Existing string-valued kinds, now typed:
+    OrphanedModule,         // serde rename: "orphaned_module"
+    TomlParseError,         // serde rename: "toml_parse_error"
+    MissingCrateRoots,      // serde rename: "missing_crate_roots"
+    ModuleTreeError,        // serde rename: "module_tree_error"
+    // New kinds emitted by `validate`:
+    OrphanFile,             // .rs in src/ with no `pub mod` / `mod` in parent — fires on category #1
+    DeadReExport,           // `pub use foo::Bar` where Bar isn't found — fires on category #2
+}
+```
+
+**Note on flat indexes vs `lookup`.** The plan keeps both: indexes inside the JSON output *and* the `lookup` subcommand. They cover overlapping access patterns — direct map-load by an agent, and targeted CLI queries respectively. Primary access pattern is TBD; revisit at the 3-phase castep-cell-io review and drop whichever path proves redundant.
+
+### `validate` rules — two, with two more deferred
+
+| Rule | Fires when | Pipeline category |
+|---|---|---|
+| `OrphanFile` | after `build_module_tree` returns for a crate, collect all `ModuleInfo.file` paths (excluding `<unresolved>`); walk the crate's `src/` for all `*.rs` files; any `.rs` on disk absent from the module-tree file set is orphan. Excludes `build.rs`, `src/bin/*.rs`, and test/example directories. | #1 (~40%) |
+| `DeadReExport` | a `pub use` path whose target cannot be resolved within the workspace. Algorithm: (1) resolve `crate::`, `self::`, `super::` prefixes; (2) if the first path segment is not a workspace member crate, skip (external re-export — not dead); (3) if the path is a glob (`pub use path::*`), skip (cannot evaluate); (4) for remaining intra-workspace paths, check whether the named item exists in any module reachable through the path prefix. Transitive re-export chains are not traced in MVP. | #2 (~20%) |
+
+Each finding becomes an `ErrorEntry { severity: Warning, kind: DiagnosticKind::OrphanFile | DeadReExport, file, line, message, … }` in `WorkspaceMap.errors`. Message includes a fix hint (e.g., `add 'pub mod foo;' to core/src/lib.rs`). Conservative recall is policy: false negatives over false positives.
+
+**Deferred from MVP:**
+
+- `UnreachablePub` — a `pub` item with no path to a public crate root and no `pub use` rescuing it. The plan originally cited this as "partial #3," but #3 (incomplete consumer updates) and unreachable-pub describe negligibly overlapping problem shapes. Defer until a measured plan-execution failure traces to it.
+- `UnsupportedLayout` warning — `mod foo { … }` and `#[path = ...]` are out of scope for AST analysis. **Document the limitation in `README.md`** under "What this tool does and doesn't analyze," rather than emitting a structured diagnostic the consumer cannot act on.
+
+### `lookup` semantics
+
+- `--symbol Task` → resolves through `name_index["Task"]`; if it points to one canonical path, emits the matching `SymbolEntry`. If many, emits the list of canonical paths plus disambiguation hints. Exit `0` on found, `1` on not-found.
+- `--file core/src/task.rs` → looks up `FileEntry` and scans `crates[].modules[]` for **all** `ModuleInfo` entries whose `.file` matches the requested path. Returns `{ file_entry: FileEntry, primary_module: ModuleInfo, inline_modules: Vec<ModuleInfo> }` where `primary_module` is the entry whose `path` matches `FileEntry.module_path`, and `inline_modules` are all other entries sharing the same `.file` (e.g. `mod tests { … }` in `lib.rs`). The `files` index still excludes inline modules (avoids key collisions); the join step recovers them.
+
+This is the *targeted query* form — the plan-decomposer asks one specific question and gets one specific answer, no JSON-dump-in-context.
+
+`lookup` is a human CLI convenience for the MVP. Pipeline agents should embed flat indexes in context (single JSON load) rather than making per-symbol CLI calls — each `lookup` invocation performs a full workspace scan. Revisit `lookup` as a pipeline tool after the cache layer ships.
+
+---
+
+## Pipeline integrations shipped *with* the MVP
+
+The user's reframing makes pipeline integration part of the MVP, not a follow-up phase. The MVP isn't done until the pipeline actually uses it.
+
+| Pipeline file | Change |
+|---|---|
+| `skills/compile-plan/SKILL.md` | **Authoritative gate.** Add a pre-check step: run `rust-workspace-map index --validate <project>`. Block plan execution on any `OrphanFile` or `DeadReExport` finding (exit code 2). This is deterministic — no agent compliance required. |
+| `agents/plan-decomposer.md` | **Soft suggestion only.** Add the **Module Wiring Check** guidance from `feature-requests/reduce_recurring_problems.md` §3.1, recommending — not requiring — that the agent call `rust-workspace-map lookup --file <parent>` when introducing a new file, to confirm intended siblings. Optionally call `lookup --symbol <name>` to check for name collisions on a planned re-export. The plan-decomposer's correctness is *not* gated on this; the deterministic gate lives in `compile-plan`. |
+
+The `compile-plan` pre-check is what turns the binary from a CLI demo into a real failure-prevention tool. The plan-decomposer suggestion is upstream defense-in-depth: free if the agent follows it, harmless if it doesn't, never the sole gate.
+
+The other integrations from the original design (`enrich-plan-gather` Step 2 replacement, `review-pr-gather` `diff` ground truth) are explicitly **deferred until after the MVP has been used through ≥3 phases of `castep-cell-io` work** — that gives concrete data on whether the cheaper integrations close enough of the failure rate to make the more ambitious ones worth building.
+
+---
+
+## Files to create / modify
+
+### `rust-workspace-map/`
+
+| File | Action |
+|---|---|
+| `src/schema.rs` | Add `SymbolEntry` (slim — see schema block), `FileEntry` (slim — three fields), `DiagnosticKind` enum. Extend `WorkspaceMap` with `symbols`, `name_index`, `files` BTreeMap fields. Migrate `ErrorEntry.kind: String → kind: DiagnosticKind` (serde-renamed to keep JSON output stable). **Do not add** `Confidence`, `Warning`, `WarningKind`, `ImportSite`, `ReExportSite`, or `warnings: Vec<Warning>`. |
+| `src/lib.rs` | After existing `cross_refs::compute`, call `indexes::derive_from_crates(&crate_infos)` and pass results to `WorkspaceMap::builder()`. Run validation (if `--validate`) after construction, merging findings into `errors`. Update emit-sites in this file to use the new `DiagnosticKind` variants. Set `WorkspaceInfo.root` to the discovered workspace root path (currently hardcoded to `"."`). Fix dropped-errors bug at lines 132–137: collect `errs` from both `Some` and `None` branches of the crate-results drain loop (currently the `None` branch silently discards errors from failed crates). |
+| `src/main.rs` | Switch to `clap` subcommands: `index` (with `--validate` flag), `lookup`. Bare-path form removed. Each subcommand takes a path. **No `--from-stdin`** — deferred. |
+| `src/lookup.rs` | NEW — pure function over `&WorkspaceMap` implementing `--symbol` and `--file` filters. `--file` looks up `FileEntry` then scans all `ModuleInfo` entries sharing the same `.file`, returning `primary_module` + `inline_modules`. |
+| `src/validate.rs` | Validation is merged into `index --validate`. No separate subcommand. The `OrphanFile` and `DeadReExport` logic lives in `src/validate.rs` as a function called by `lib.rs::run()` when `--validate` is set. |
+| `src/indexes.rs` | NEW — pure function `derive_from_crates(&[CrateInfo]) -> (symbols, name_index, files)` in one pass over `crates[].modules[]`. Called before `WorkspaceMap` builder. No second AST traversal. |
+| `src/module_tree.rs`, `src/file_parser.rs` | Update emit-sites to construct `ErrorEntry` with `kind: DiagnosticKind::OrphanedModule` etc. Fix error-vec cloning at line 252 (O(n²) memory waste) and fragile `unwrap_or` at line 33. |
+| `src/cross_refs.rs` | No structural changes for the MVP. The crate-granularity `imported_by`/`exported_by` already collected stays as-is. File:line granularity per symbol is deferred until a measured workflow cites it. |
+| `tests/fixtures/sample-workspace/` | Add `bad-orphan/` and `bad-dead-reexport/` sub-fixtures. |
+| `tests/integration_test.rs` | (a) Rewrite **every** existing invocation from the bare-path form to `index <path>` (this is the breaking-change call-site sweep). (b) Add tests for `validate` exit code 2 on the new fixtures. (c) Add tests for `lookup --symbol` (single + ambiguous + not-found) and `lookup --file`. |
+| `README.md` | Document subcommand surface; add a **"What this tool does and doesn't analyze"** section explicitly noting that inline `mod foo { … }` and `#[path = ...]` items are not analyzed. Add an "Inspiration" section crediting `tirth8205/code-review-graph` (MIT) and stating the differentiation: Rust-only via `syn`, native `pub`/`pub use`/`mod` semantics, one-shot CLI, no daemon/MCP/SQLite. |
+
+### Existing utilities reused (do NOT reinvent)
+
+- `src/cargo_info.rs::parse_cargo_toml` — existing dep + package + edition parser.
+- `src/module_tree.rs::build_module_tree` — already resolves `mod foo;` to `foo.rs` / `foo/mod.rs`. **Phase-1 `OrphanFile` rule reuses this resolver in reverse**: walk every `.rs` under the crate source dir, check whether the resolver included it.
+- `src/cross_refs.rs::compute` — already populates `crossReferences.types` with crate-granularity `imported_by`/`exported_by` keyed by short symbol name. The MVP **does not** extend its granularity (file:line per-symbol is deferred). The `name_index` flat index is largely a re-shaping of the same data.
+- `src/render.rs::render_to_writer` — single rendering path, used by all subcommands.
+- `src/file_parser.rs` — `syn` parsing wrapper, no changes.
+- `bon::Builder` derive pattern — already idiomatic in the codebase; new types use it for consistency.
+
+### `rust-development-pipeline/`
+
+| File | Action |
+|---|---|
+| `skills/compile-plan/SKILL.md` | **Authoritative gate.** Add `validate` pre-check that blocks on `OrphanFile` / `DeadReExport` findings (exit code 2). |
+| `agents/plan-decomposer.md` | **Soft suggestion only.** Add Module Wiring Check section recommending `rust-workspace-map lookup --file <parent>` and `lookup --symbol <name>` calls when introducing new files / re-exports. Phrase as guidance, not a procedural mandate. |
+
+---
+
+## Verification (MVP done = all of these green)
+
+1. **Build green**: `cargo build --release` in `rust-workspace-map/`.
+2. **Call-site sweep**: every invocation in `tests/integration_test.rs` rewritten from the bare-path form to `index <path>` in the same commit as the breaking change. README examples and any wrapper scripts in `rust-development-pipeline/` updated in lockstep. This is the flag-day checklist for D1.
+3. **Existing tests green**: all 8 integration tests in `rust-workspace-map/tests/integration_test.rs` still pass after the rewrite, with the additive schema.
+4. **DiagnosticKind migration green**: the existing `orphaned_module` warning emitted by `module_tree.rs` still serializes as the string `"orphaned_module"` in JSON (verified by an integration test on a known-orphan fixture). No JSON-output regression.
+5. **New unit fixtures**: `bad-orphan/` triggers an `OrphanFile` finding naming the parent file; `bad-dead-reexport/` triggers a `DeadReExport` finding. `index --validate` exits `2` in both cases. Findings appear in `WorkspaceMap.errors` with `severity: Warning`.
+6. **Lookup**: against the existing `sample-workspace`, `lookup --symbol Task` returns the matching `SymbolEntry`; `lookup --file core/src/task.rs` returns the joined `FileEntry`+`ModuleInfo`; `lookup --symbol DoesNotExist` exits `1`.
+7. **Real-world dry-run + cache baseline**: `rust-workspace-map index --validate /Users/tony/programming/castep-cell-io` and `rust-workspace-map index --validate /Users/tony/programming/castep-cell-io` (303 files) run to completion. Record wall-time in `notes/` as the **0.1.0→0.2.0 cache baseline**. The cache layer (currently deferred) is allowed only if a future change pushes wall-time past 2× this baseline AND `compile-plan` runs index --validate ≥3× per phase. Estimated baseline: 100–400 ms; the original 2 s threshold is unlikely to ever trigger.
+8. **Pipeline smoke**: in `rust-development-pipeline`, run `compile-plan` against a synthetic plan that creates a file without a `pub mod`. The pre-check blocks (via `index --validate`). Run another synthetic plan that introduces a `pub use` to a nonexistent symbol. The pre-check blocks (via `index --validate`). Both behaviors are deterministic — no LLM compliance involved.
+
+---
+
+## Out of MVP scope (explicitly deferred — built only when the pipeline measurably needs them)
+
+These are recorded so they don't get rebuilt as ad-hoc additions. Each requires concrete pipeline pain to justify:
+
+**Cut from this revision of the plan (after KISS review):**
+
+- **`UnreachablePub` validate rule** — was originally framed as "partial #3," but #3 (incomplete consumer updates) and unreachable-pub describe negligibly overlapping problem shapes. Defer until at least one measured plan-execution failure traces to an unreachable-pub case the rule would have caught.
+- **`UnsupportedLayout` warning** — covered by a README sentence ("What this tool does and doesn't analyze") rather than a structured diagnostic the consumer cannot act on.
+- **`Confidence` enum (`EXTRACTED`/`INFERRED`/`AMBIGUOUS`)** — every value would be `EXTRACTED` in the MVP because no inference path exists. Add the day a non-AST data source is introduced.
+- **Per-symbol `imported_by` / `re_exported_at` at file:line granularity** (`ImportSite`, `ReExportSite` types) — the three rules don't need them and crate-granularity already lives in `CrossReferences.types`. Add when a measured workflow cites "find references" with file:line precision.
+- **`--from-stdin` on `validate` and `lookup`** — would force `serde::Deserialize` on every schema type plus a JSON-roundtrip stability commitment, with no current piping consumer. Add when a third caller requests it.
+- **Parallel `Vec<Warning>` channel** — collapsed into the existing `WorkspaceMap.errors` with the typed `DiagnosticKind`. No reason to revive the parallel channel.
+- **Hard-mandate plan-decomposer `lookup` calls** — soft suggestion is the chosen design. Hard mandates depending on LLM compliance fail silently; the deterministic gate is `compile-plan`'s pre-check.
+
+**Originally deferred (carried forward unchanged):**
+
+- **`diff <base-ref>` subcommand** — defer until `review-pr` regex parsing produces wrong results that an AST-based diff would have caught. The bar: at least one fix round attributable to a missed structural change in review.
+- **`--compact` and scope filters (`--crates`, `--files`)** — defer until measured token use exceeds 50% of an agent's context window on `castep-cell-io`-size workspaces.
+- **File-mtime-keyed cache** at `~/.cache/rust-workspace-map/<workspace-hash>.json` — defer until verification step 7 measures wall-time past 2× the recorded baseline, OR the `compile-plan` pre-check is observed running ≥3× per phase.
+- **`serve` mode / MCP server** — defer until the pipeline grows its first MCP server. The pipeline currently has zero (`/Users/tony/programming/rust-development-pipeline/.mcp.json` does not exist).
+- **Derive-macro expansion** (`bon::Builder`, `serde`, `thiserror`) — defer until a real plan-decomposer task fails because the symbol it needed was macro-generated and missing from `symbols`. Track misses by adding a "no symbol found, but file matches a `#[derive]` site" hint to `lookup`.
+- **Eval harness (CRG `eval/`-style A/B replay)** — defer indefinitely. The MVP does not need a published reduction number; it needs the pipeline's fix-task ratio to drop. That metric already exists in `notes/pr-reviews/`.
+- **NDJSON streaming output** — defer until a streaming consumer exists.
+- **Multi-language Tree-sitter coverage** — out of scope permanently. Rust-only is the moat.
+
+---
+
+## CRG attribution
+
+In `README.md` of `rust-workspace-map`, add a short "Inspiration" section:
+
+> The subcommand surface and conservative-recall validation policy are inspired by [`tirth8205/code-review-graph`](https://github.com/tirth8205/code-review-graph) (MIT). This tool is differently shaped: Rust-only via `syn`, models `pub`/`pub use`/`mod` semantics natively, and is a one-shot CLI rather than a daemon-backed graph store. CRG's edge-confidence stance (`EXTRACTED`/`INFERRED`/`AMBIGUOUS`) is *not* yet adopted — the MVP performs only AST extraction, so every value would be `EXTRACTED`. Revisit if a non-AST data source is introduced.
+
+---
+
+## Locked decisions (recorded so they don't drift)
+
+- **D1**: `index` mandatory immediately. Bare-path form removed in v0.2.0. No deprecation alias. Call-site sweep is part of the same commit (verification step 2). The `cross_references.types` re-keying (short-name → fully-qualified path) is also a breaking change and ships in this same commit.
+- **D2**: MVP scope =
+  - **subcommands**: `index` (with `--validate` flag), `lookup` (no `--from-stdin`)
+  - **schema additions**: slim `SymbolEntry`, slim `FileEntry`, three flat indexes (`symbols`, `name_index`, `files`), `DiagnosticKind` enum (replacing `ErrorEntry.kind: String`), re-key `cross_references.types` from short-name to fully-qualified path
+  - **schema *not* added**: `Confidence`, `Warning`, `WarningKind`, `ImportSite`, `ReExportSite`, parallel `warnings: Vec<Warning>`
+  - **validate rules**: `OrphanFile`, `DeadReExport` (5-case algorithm; run via `index --validate`)
+  - **pipeline integrations**: `compile-plan` pre-check (deterministic gate); `plan-decomposer` Module Wiring Check (soft suggestion only)
+- **D3**: Cache layer deferred. Gate: wall-time on `castep-cell-io` exceeds 2× the verification-step-7 baseline AND `compile-plan` runs validate ≥3× per phase.
+- **D4**: Pipeline integrations ship *with* the MVP, not after — the tool isn't done until the pipeline uses it.
+- **D5**: Flat indexes and `lookup` are belt-and-suspenders. Primary access pattern (raw-map reading vs `lookup` calls) to be determined empirically over ≥3 castep-cell-io phases; the redundant path may be dropped at that review.
+- **D6**: Per-symbol reverse data (file:line `imported_by`, `re_exported_at`) is gated on a measured workflow citation. Adding it speculatively would ship unread data and obligate maintenance forever.
diff --git a/plans/mvp/MVP_PLAN_review.md b/plans/mvp/MVP_PLAN_review.md
new file mode 100644
index 0000000..8ecc67c
--- /dev/null
+++ b/plans/mvp/MVP_PLAN_review.md
@@ -0,0 +1,238 @@
+# Review of `MVP_PLAN.md` — KISS / first-principles critique
+
+## Context
+
+The MVP plan claims "very faithful" KISS and "ships only what closes a measured pipeline failure." Read end-to-end against the current state of `/Users/tony/programming/rust-workspace-map/`, parts of the proposed surface are speculative scaffolding that don't survive first-principles scrutiny on the right cost axis. The measured failure data — #1 `pub mod` (~40%), #2 `pub use` (~20%), #3 consumer updates (~20%) — supports a tighter MVP that earns the right to grow only if it doesn't close enough of the failure rate.
+
+This document records the points where the plan over-builds and proposes a tighter cut. After review with the user (who clarified that the "O(1) lookup" framing was about token cost / agent reasoning load, not wall-clock latency), the critique was sharpened: flat indexes are accepted, but `Confidence`, `UnreachablePub`, parallel warning channel, `--from-stdin`, and other items remain over-built when the principle is applied uniformly.
+
+## Ground truth that changes the calculus
+
+Things the plan understates or omits about what already exists today:
+
+- `schema::CrossReferences { types: BTreeMap<String, TypeRef> }` is **already** keyed by short symbol name, with `imported_by: Vec<String>` and `exported_by: Vec<String>` populated by `cross_refs::compute`. This is structurally what the plan calls `name_index` + part of `imported_by`. The plan presents flat indexes as new infrastructure when ~70% of one already ships in 0.1.0.
+- `module_tree.rs` **already detects orphan modules in one direction**: a `mod foo;` declared but with no resolvable file emits an `orphaned_module` warning entry (`module_tree.rs:120–134`). The plan's `OrphanFile` is the reverse direction (file exists but no `mod foo;`), which is a real gap, but the plan doesn't note that the existing detector and the new one should share an `ErrorEntry` / warning emission pipeline rather than a parallel `Vec<Warning>` channel.
+- `WorkspaceMap.errors: Vec<ErrorEntry>` already exists, with `severity: ErrorSeverity::{Error, Warning}`, `kind`, `context`, `cause`, `file`, `line`. The plan adds a parallel `warnings: Vec<Warning>` channel without explaining why the existing `ErrorEntry` with `Severity::Warning` won't do.
+- All schema types derive `serde::Serialize` only — none derive `Deserialize`. The plan's `--from-stdin` requires `Deserialize` on every type plus a JSON-roundtrip stability commitment. This is a substantive cost the plan doesn't account for.
+- `bon::Builder` is used pervasively; new types fitting that idiom is correct.
+- `clap` derive is already wired; subcommand split is mechanical.
+
+## The KISS critique, point by point
+
+### 1. "O(1) lookup" — accepted on token-cost grounds, with a remaining tension
+
+The plan justifies three pre-computed flat indexes (`symbols`, `name_index`, `files`) under "Core Design Goal: O(1) lookup for LLM agents." The intended cost axis is **token cost / agent reasoning load**, not wall-clock latency: a flat index lets an agent (or a `lookup` subcommand) jump straight to the answer rather than mentally tree-walking the hierarchical `crates[].modules[].public_items[]` shape — and prevents the agent's common fallback of running `rg`/`read` against the source when the JSON's shape isn't query-friendly. That argument is legitimate and KISS-aligned: pre-shaped answers are exactly the kind of thing measured "agent friction" should pay for.
+
+The tension that remains, worth making explicit before building all three indexes:
+
+- **If the dominant access pattern is `lookup --symbol/--file`** — i.e., the agent never sees the raw map — then the indexes only need to live as in-memory acceleration *inside* the `lookup` implementation. The JSON output of `index` does not need to carry them, because no agent reads the dump directly.
+- **If the dominant access pattern is "load full map into agent context, reason globally"** — e.g., enrich-plan-gather, plan-review, rust-architect — then the indexes belong in the JSON output, because the JSON's shape *is* the agent's query interface. The duplication (each item appears in both the hierarchical view and the flat view) is the cost paid for query ergonomics.
+
+The plan currently does both: indexes *and* `lookup`. That's a reasonable belt-and-suspenders position, but it's worth recording which access pattern is primary so future iteration knows what to trim. If `lookup` is the primary path, the indexes can be dropped from JSON output (kept in-memory only) once that's measured. If raw-map reading is primary, `lookup` becomes the redundant path.
+
+**No change to recommendation on the indexes themselves.** Keep `symbols`, `name_index`, `files` in the MVP, but flag in the plan: *"primary access pattern TBD; revisit at the 3-phase review whether `lookup` or raw-map reading dominates, drop the redundant path then."*
+
+The remaining first-principles concerns from the original critique survive on a different axis:
+
+- `Confidence` is still YAGNI (every value will be `EXTRACTED`; no inference path exists in the MVP). This is independent of token cost — a constant-valued field neither shapes a query nor saves agent reasoning.
+- `imported_by` / `re_exported_at` per-symbol, at file:line granularity, is justified *if* it's actually queried by a measured workflow. The MVP's three rules don't need it. `lookup --symbol` can return it cheaply — but if no agent prompt cites "who imports X," the data ships unread. Worth gating on a concrete pipeline citation.
+
+### 2. `Confidence { EXTRACTED, INFERRED, AMBIGUOUS }` is pure YAGNI
+
+The plan imports a CRG idiom into a tool that does only `syn` AST extraction. Every value will be `EXTRACTED`. There is no inference path, no ambiguity-resolver. Adding the field means consumers either ignore a constant field (noise) or branch on a value that never differs (dead code).
+
+**Recommendation:** delete `Confidence` from the MVP. Add the day a non-AST data source is introduced, never before.
+
+### 3. The new MVP rules don't need per-symbol reverse indexes
+
+Walk through what each `validate` rule actually requires:
+
+| Rule | Inputs needed | Already have? |
+|---|---|---|
+| `OrphanFile` | filesystem walk of crate `src/` + `module_tree::resolve_module_path` to check inclusion | Yes — `resolve_module_path` exists |
+| `DeadReExport` | forward scan: for each `pub use a::b::C`, look for `C` in module `a` (or transitively) | Yes — `crates[].modules[].public_items[]` is the forward index |
+| `UnreachablePub` | reachability over module tree | Tree exists; pass is new but cheap |
+
+None of these require `imported_by` or `re_exported_at` per-symbol. The plan adds them to feed `lookup`, then mandates `lookup` in the plan-decomposer. That's circular justification — the indexes exist to support a subcommand that exists to motivate the indexes.
+
+**Recommendation:** keep flat indexes on token-cost grounds (per §1), but defer the per-symbol reverse fields (`imported_by` at file:line, `re_exported_at` at file:line) until a measured workflow cites them. The crate-granularity `imported_by`/`exported_by` already in `CrossReferences.types` covers the common "which crate exports X" question.
+
+### 4. `UnreachablePub` is scope creep mislabeled as "partial #3"
+
+#3 is "incomplete consumer updates" — when an existing public item's signature/usage changes and callers don't get updated. `UnreachablePub` detects `pub` items with no public path from a crate root. These are different problems with negligible overlap. Calling it "partial #3" lets a speculative rule ride into the MVP on real-failure-data coattails.
+
+The rule also adds graph-reachability logic for the smallest expected hit rate of the four rules.
+
+**Recommendation:** drop `UnreachablePub` from MVP. Ship the two rules that close the measured 60%, observe for ≥3 phases per the plan's own "Out of scope" methodology, then decide.
+
+### 5. `UnsupportedLayout` is not a rule, it's a README sentence
+
+`mod foo { … }` and `#[path = ...]` exist in real code; emitting a structured warning that says "we didn't analyze this" produces noise on every run for workspaces that use either form. The user (the pipeline) has no action to take on it.
+
+**Recommendation:** document the limitation in `README.md` under "What this tool does and doesn't analyze." If a real plan-execution failure ever traces to an unanalyzed inline mod, then add the warning.
+
+### 6. `--from-stdin` adds composability with no consumer
+
+The plan justifies `--from-stdin` with `index | validate --from-stdin | jq` as a "Unix property" worth preserving. But:
+
+- Both pipeline integrations call subcommands directly (`validate <project>`, `lookup --file <parent>`). Neither pipes.
+- No external user exists.
+- Verification step #5 tests piping but the test exists to validate infrastructure with no caller.
+
+The hidden cost: every schema type currently derives `serde::Serialize` only. `--from-stdin` requires `serde::Deserialize` everywhere plus a roundtrip-stability commitment (camelCase serialization is asymmetric on some types). That's ~30 derives plus a stability gate the plan doesn't acknowledge.
+
+**Recommendation:** drop `--from-stdin` from MVP. Subcommands take a path. If a third caller asks for piping, add it then.
+
+### 7. `FileEntry` duplicates `ModuleInfo` content — but the duplication is the point if the index is keyed on file path
+
+Field-by-field:
+
+| `FileEntry` field | Already in `ModuleInfo`? |
+|---|---|
+| `crate_name` | derivable from `path.split("::").first()` |
+| `module` | yes — `path` |
+| `submodules` | yes — `submodules` |
+| `exports` | derivable — `public_items.iter().map(|p| &p.name)` |
+| `is_crate_root` | derivable — `path` has no `::` |
+| `parent_module_file` | new — needs a parent-of map built from `submodules` reverse |
+
+Under the time-cost lens, this is wasteful denormalization. Under the token-cost lens (per §1 above), the `files: BTreeMap<String, FileEntry>` is *keyed on the file path string*, which the hierarchical `crates[].modules[]` view is not. An agent asking "what's in `core/src/task.rs`?" hits the file index directly; without it, the agent has to scan modules and match on `file`.
+
+So the right framing isn't "delete `FileEntry`," it's:
+
+- The genuinely new fields are `is_crate_root` and `parent_module_file`. Everything else is denormalization driven by the indexing-key choice (file path).
+- Pick the slimmest `FileEntry` that pays for the new key. One viable shape: `{ module_path, parent_module_file, is_crate_root }` — three fields, with `lookup --file` re-joining to the matching `ModuleInfo` for the rest. Avoids re-serializing `submodules` and `exports` into the JSON when they're already in the hierarchical view.
+- Or accept the duplication outright and document that the JSON is pre-denormalized for agent ergonomics.
+
+**Recommendation:** keep `files` as a flat index, but slim `FileEntry` to the three fields the hierarchical view doesn't already cover (`module_path`, `parent_module_file`, `is_crate_root`). `lookup --file` reads the slim entry and joins to the corresponding `ModuleInfo`. JSON output stays compact; agents loading the raw map can still hashmap-lookup by file path.
+
+### 8. Replace stringly-typed `ErrorEntry.kind` with an enum, don't add a parallel `Vec<Warning>` channel
+
+Workspace already has `ErrorEntry { file, line, message, severity, kind: String, context, cause }` with `ErrorSeverity::Warning`. The plan's new `Vec<Warning>` channel duplicates infrastructure for no reason — the same row of data fits in `ErrorEntry` exactly. Per user preference, an enum is the right shape (over the current stringly-typed `kind`), so:
+
+**Recommendation:**
+
+1. Introduce a single `enum DiagnosticKind` (or `WarningKind` if you want to keep the name; "diagnostic" reads better given existing rows already cover both errors and warnings) with variants for the existing string values currently passed (`OrphanedModule`, `TomlParseError`, `MissingCrateRoots`, `ModuleTreeError`, plus the new `OrphanFile`, `DeadReExport`, etc.).
+2. Migrate `ErrorEntry.kind: String → kind: DiagnosticKind`. This is a one-shot refactor: every emit-site in `lib.rs`, `module_tree.rs`, `file_parser.rs` becomes a typed variant; the JSON `kind` field stays a string via serde rename, so consumers don't see a breaking change.
+3. The new validate rules emit `ErrorEntry { severity: Warning, kind: DiagnosticKind::OrphanFile, ... }` into the existing `WorkspaceMap.errors`. No `Vec<Warning>`. No parallel channel.
+
+This is the disciplined version of the plan's intent — typed kinds — applied consistently rather than only to the new code path. It also eliminates the asymmetry where existing `orphaned_module` warnings and new `OrphanFile` warnings would have lived in different collections with different typing.
+
+### 9. Plan-decomposer hard-mandate-to-call-lookup is fragile defense-in-depth
+
+The plan requires the plan-decomposer LLM to call `lookup --file <parent>` before emitting any new-file `[[changes]]` entry, with the result cited in plan rationale.
+
+- LLM compliance with hard procedural mandates is probabilistic. A skipped call can't be detected in-band.
+- The downstream `compile-plan` pre-check already catches the issue deterministically.
+- If the pre-check is the gate, the plan-decomposer's call is redundant work; if the pre-check misses, the plan-decomposer almost certainly missed the same case.
+
+**Recommendation:** make the `compile-plan` pre-check the authoritative gate. Plan-decomposer integration becomes a soft suggestion ("when introducing a new file, consider `lookup --file <parent>` to confirm intended siblings"). Soft suggestions cost nothing if skipped; hard gates relying on agent compliance fail silently.
+
+### 10. The verification matrix has the wrong cache trigger
+
+Verification step #6 sets a 2-second wall-time trigger on `castep-cell-io` for the cache deferral. First-principles estimate: AST parse 303 small files in parallel (`rayon` is already used) on modern hardware ≈ 100–400 ms. The 2-second threshold is unlikely to ever trigger. That's fine, but the plan should record the **expected** baseline so the cache truly remains deferred rather than becoming a "we measured 1.8s, let's be safe" creep target.
+
+**Recommendation:** measure current 0.1.0 wall-time on `castep-cell-io` once. Lock that as baseline. Cache layer is allowed only if a future change pushes wall-time past 2× baseline AND `compile-plan` runs validate ≥3× per phase.
+
+### 11. The breaking-change mechanics are under-specified
+
+`index` becomes mandatory in v0.2.0; bare-path form deleted entirely. Fine decision. But the plan doesn't enumerate:
+
+- Every invocation in `tests/integration_test.rs` needs rewriting.
+- Any wrapper scripts in `rust-development-pipeline/` need updating in the same commit.
+- README examples need updating.
+- Anything in the user's `notes/` that has copy-pasteable invocations.
+
+This isn't an objection to the decision, just to the breeziness. A breaking change is a flag-day event; the plan should have a checkbox for the call-site sweep.
+
+## The principle the plan violates
+
+The plan articulates the right principle: *"ships only what closes a measured pipeline failure or directly enables a pipeline integration that closes one. Anything else is recorded as deferred."* It then applies that principle unevenly. Some features get the discipline:
+
+- `diff` subcommand → deferred
+- `--compact` / scope filters → deferred
+- File-mtime cache → deferred
+- `serve` mode / MCP → deferred
+- Macro expansion → deferred
+- Eval harness → deferred
+- NDJSON streaming → deferred
+
+Other features remain in scope. Of those, some are justified on the right axis (per §1 token-cost argument) and survive the principle:
+
+- Flat indexes (`symbols`, `name_index`, slim `files`) → justified by token cost / agent reasoning load. **Keep.**
+
+Others remain over-built when the principle is applied uniformly:
+
+- `Confidence` enum → no inference path exists; constant-valued field. **Cut.**
+- `imported_by` / `re_exported_at` per-symbol at file:line granularity → not required by the three rules; ships unread unless a measured workflow cites it. **Defer.**
+- `UnreachablePub` rule → "partial #3" attribution is overstated; cheap to add later. **Defer.**
+- `UnsupportedLayout` warning → no actionable response from the consumer. **Make a README sentence.**
+- `--from-stdin` → no piping consumer; hidden `Deserialize` cost. **Defer.**
+- Parallel `Vec<Warning>` channel → duplicates `WorkspaceMap.errors`. **Reuse `ErrorEntry` with typed `kind`.**
+- Plan-decomposer hard-mandate-to-call-`lookup` → fragile compliance gate when `compile-plan` pre-check is the deterministic gate. **Soft suggestion only.**
+
+## A KISS-cut MVP
+
+What ships if the plan's stated principle is applied uniformly, with §1's token-cost reasoning preserved:
+
+**Schema delta** (in `src/schema.rs`):
+- Migrate `ErrorEntry.kind: String → kind: DiagnosticKind` (typed enum, serde-renamed to a string in JSON for backward-compatible output). Variants cover existing kinds (`OrphanedModule`, `TomlParseError`, `MissingCrateRoots`, `ModuleTreeError`) plus new (`OrphanFile`, `DeadReExport`).
+- Add `SymbolEntry` (slim — `crate_name`, `module`, `file`, `line`, `kind: ItemKind`, no `confidence`, no `imported_by`/`re_exported_at` until a workflow cites them).
+- Add `FileEntry` (slim — `module_path`, `parent_module_file`, `is_crate_root`).
+- Add three `BTreeMap` fields on `WorkspaceMap`: `symbols`, `name_index`, `files`. Built in a single derivation pass after `cross_refs::compute`.
+- Do **not** add: `Confidence`, `Warning`/`WarningKind` (use `ErrorEntry`), `ImportSite`/`ReExportSite`, parallel `warnings: Vec<Warning>`.
+
+**CLI delta** (`src/main.rs`):
+- Subcommands: `index` (current behavior), `validate <PATH>`, `lookup <PATH> --symbol|--file`. No `--from-stdin`.
+- Bare-path form removed; `tests/integration_test.rs` rewrites in lockstep.
+- Exit codes: `0` clean, `1` tool error, `2` validation findings.
+
+**`validate` rules** (new file `src/validate.rs`, pure function over `&WorkspaceMap` returning `Vec<ErrorEntry>` to merge into the existing `errors` field):
+- `OrphanFile` — fs walk of each crate's `src/` minus the set of files reachable through `module_tree::resolve_module_path`. Reuses the existing resolver.
+- `DeadReExport` — for every `pub use a::b::C` across all modules, check whether any module under `a::*` has a `public_item` named `C`.
+- **Not** `UnreachablePub` (defer until measured #3 cases attributable to it).
+- **Not** `UnsupportedLayout` as a rule — document the limitation in `README.md`.
+
+**Index derivation** (new file `src/indexes.rs`, pure function):
+- `derive_indexes(&WorkspaceMap) -> (symbols, name_index, files)` in one pass over `crates[].modules[]`.
+
+**Pipeline integration** (one only):
+- `skills/compile-plan/SKILL.md` adds a pre-check step: run `validate`, block on any `OrphanFile` / `DeadReExport` finding.
+- Plan-decomposer integration is a *soft suggestion* — "when introducing a new file, consider `lookup --file <parent>` to learn the intended siblings." No hard mandate.
+
+**Out of MVP** (additive, defer until measured):
+- `Confidence` enum → defer until a non-AST data source is introduced.
+- Per-symbol `imported_by` / `re_exported_at` at file:line → defer until a measured workflow cites "find references."
+- `UnreachablePub` rule → defer until measured #3 cases appear.
+- `UnsupportedLayout` warning → README sentence.
+- `--from-stdin` → defer until a piping consumer exists; carries hidden `Deserialize` cost.
+- Plan-decomposer hard-call-`lookup` mandate → soft suggestion only.
+- File-mtime cache → defer per the plan's existing methodology.
+- Everything else from the plan's own "Out of MVP scope" stays out.
+
+**Coverage projection:** ~60% of measured failure rate (#1 + #2 caught deterministically). Same as the bigger plan's hard-block coverage; the bigger plan's `UnreachablePub` adds at most a few percent on a partial-#3 axis with high false-negative tolerance.
+
+**Effective trim vs the plan as written:** four schema types removed (`Confidence`, `Warning`, `ImportSite`, `ReExportSite`), one rule removed (`UnreachablePub`), one rule downgraded to documentation (`UnsupportedLayout`), one CLI flag removed (`--from-stdin`), one mandate softened (plan-decomposer), one channel collapsed (warnings → `errors` with typed kind). Indexes kept; `lookup` kept; subcommand split kept; breaking change kept.
+
+## Critical files referenced
+
+- `/Users/tony/programming/rust-workspace-map/src/schema.rs` — the existing `WorkspaceMap`, `ErrorEntry`, `ErrorSeverity`, `CrossReferences`, `Import`, `ReExport`. The plan's additive changes mostly belong here.
+- `/Users/tony/programming/rust-workspace-map/src/module_tree.rs` — `resolve_module_path` and existing orphan detection. `OrphanFile` rule reuses both.
+- `/Users/tony/programming/rust-workspace-map/src/cross_refs.rs` — already does name-keyed `imported_by`/`exported_by`. The plan's `name_index` is largely a rename.
+- `/Users/tony/programming/rust-workspace-map/src/main.rs` — the bare-path CLI; subcommand split lands here.
+- `/Users/tony/programming/rust-workspace-map/src/lib.rs` — the orchestrator; would call into `validate::run` after `cross_refs::compute`.
+- `/Users/tony/programming/rust-workspace-map/Cargo.toml` — `clap`, `bon`, `syn`, `rayon`, `serde` already present. No new crates needed for the KISS-cut MVP.
+- `/Users/tony/programming/rust-workspace-map/tests/integration_test.rs` — every existing invocation rewrites to `index <path>`.
+- `/Users/tony/programming/rust-workspace-map/tests/fixtures/sample-workspace/` — add `bad-orphan/` and `bad-dead-reexport/`.
+- `/Users/tony/programming/rust-development-pipeline/skills/compile-plan/SKILL.md` — single integration; pre-check step.
+
+## Verification (for the KISS-cut MVP)
+
+1. `cargo build --release` clean.
+2. All existing integration tests pass after subcommand rewrite (`<path>` → `index <path>`).
+3. `bad-orphan/` fixture → `validate` exits 2, warning names parent file.
+4. `bad-dead-reexport/` fixture → `validate` exits 2, warning names the dead path.
+5. `validate /Users/tony/programming/castep-cell-io` runs to completion. Wall-time recorded as cache baseline.
+6. `compile-plan` against a synthetic plan that creates a file without `pub mod` blocks. Same for a synthetic `pub use` of a nonexistent symbol.
+
+That's a real MVP. Ship it, run it through `castep-cell-io`, watch the fix-task ratio in `notes/pr-reviews/` for ≥3 phases. Then revisit `lookup`, `UnreachablePub`, flat indexes — whichever the *measured* gap demands, in priority order, one at a time.
diff --git a/plans/mvp/MVP_PLAN_review2.md b/plans/mvp/MVP_PLAN_review2.md
new file mode 100644
index 0000000..56d5272
--- /dev/null
+++ b/plans/mvp/MVP_PLAN_review2.md
@@ -0,0 +1,224 @@
+# Second Review: MVP Plan — Structural Critique
+
+## Context
+
+The first review (`MVP_PLAN_review.md`) correctly trimmed scope: it killed `Confidence`, `UnreachablePub`, `--from-stdin`, the parallel `Vec<Warning>` channel, and the plan-decomposer hard mandate. All correct cuts.
+
+This second review scrutinizes the **algorithms, data model integrity, and implementation sequencing** that the first review didn't cover. Grounded in the actual codebase at `/Users/tony/programming/rust-workspace-map/src/`.
+
+---
+
+## User walkthrough decisions
+
+| # | Topic | Decision |
+|---|---|---|
+| 2 | `DeadReExport` false-positive risk | **Specify 5-case algorithm** before implementation |
+| 7 | `validate` doubles parse cost | **Merge as `index --validate`** flag |
+| 1 & 3 | `cross_references.types` collision + `files` inline-module | **Fix both in MVP** |
+| 4 | `OrphanFile` algorithm underspecified | **Option C**: diff `ModuleInfo.file` set vs fs walk |
+| 5 | `indexes::derive` sequencing | **Option B**: `derive_from_crates(&[CrateInfo])` before builder |
+| 6 | "O(1) lookup" claim | **Resolved** — O(1) means one agent call, not algorithmic complexity |
+| 8 | `lookup` parse-per-invocation cost | **Document as human tool only** until cache ships |
+| 9 | `symbols` key format for root items | **Specify** `crate::Name` (2 segments) vs `crate::mod::Name` (3+) |
+| 10 | `WorkspaceInfo.root` hardcoded to `"."` | **Fix**: set to actual workspace root |
+| 11 | `module_tree.rs` existing bugs | **Fix** during DiagnosticKind refactor |
+
+---
+
+## Finding 2 (DECIDED: Specify 5-case algorithm)
+
+**Severity: False positives — would block valid plans**
+
+The plan originally said: *"for every `pub use a::b::C`, check whether any module under `a::*` has a `public_item` named `C`."*
+
+This only works for intra-crate re-exports. Real code has five cases:
+
+| Re-export form | Naive check result |
+|---|---|
+| `pub use crate::inner::Thing;` | Works (after prefix stripping) |
+| `pub use super::thing;` | Fails — `super` is relative |
+| `pub use self::detail::Type;` | Fails — `self` needs resolution |
+| `pub use serde::Serialize;` | **False positive** — external dep, not in workspace |
+| `pub use some::path::*;` | Cannot evaluate — glob |
+
+Case 4 is critical: `pub use serde::Serialize;` is valid but the naive algorithm won't find `Serialize` in workspace modules → emits `DeadReExport` warning → `compile-plan` pre-check **blocks a valid plan**.
+
+**Decision:** The algorithm now specified in `MVP_PLAN.md` must:
+1. Resolve `crate::`, `self::`, `super::` prefixes
+2. Skip re-exports where the first segment is NOT a workspace member (external crate)
+3. Skip glob re-exports (conservative — false negatives preferred per policy)
+4. Document that transitive re-export chains are not traced in MVP
+
+**Amendment applied:** Yes — §"validate rules" table updated.
+
+---
+
+## Finding 7 (DECIDED: Merge as `index --validate`)
+
+**Severity: Efficiency**
+
+`index` and `validate` both do full AST parses. Sequential invocations double the cost for zero new information. The `validate` output is a superset of `index` output (same JSON + extra ErrorEntries).
+
+**Decision:** `index` gets a `--validate` flag. Single invocation:
+- `rust-workspace-map index <path>` — current behavior (no validation)
+- `rust-workspace-map index <path> --validate` — runs OrphanFile + DeadReExport, findings appear in `errors`
+
+**Amendments applied:**
+- CLI section: removed `validate` subcommand, added `[--validate]` flag to `index`
+- Files table: removed separate `src/validate.rs` row (merged into `index --validate` flow)
+- Pipeline integrations: `compile-plan` pre-check calls `index --validate`
+- Verification steps: all `validate` references → `index --validate`
+- Locked decisions D2: updated
+
+---
+
+## Findings 1 & 3 (DECIDED: Fix both in MVP)
+
+### 1. `cross_references.types` short-name collision corrupts data
+
+`cross_refs.rs:30-36`: `TypeRef` is keyed by short name only. When two crates define different `Config` types, the first one processed wins — the second's `kind`, `crate_name` are silently lost (only `exported_by` gets appended). Any consumer trusting `TypeRef.kind` gets wrong data.
+
+Names like `Config`, `Error`, `Result` appear across crates routinely — this is not an edge case.
+
+**Fix:** Key `cross_references.types` by fully-qualified path (`crate::module::Name`).
+
+### 3. `files` index silently drops inline modules
+
+One `.rs` file can host multiple modules (`mod tests { ... }` in `lib.rs`). `files: BTreeMap<String, FileEntry>` stores only the last-inserted entry per file path — other modules silently dropped. Non-deterministic which wins.
+
+When `plan-decomposer` calls `lookup --file core/src/lib.rs`, it gets whichever module happened to be processed last.
+
+**Fix:** Exclude inline modules from `files` index (they have no independent file).
+
+**Amendments applied:**
+- Schema section: added comment documenting `cross_references.types` re-keying
+- Schema section: added inline-module exclusion note to `FileEntry`
+- Locked decisions D2: added "re-key `cross_references.types` from short-name to fully-qualified path"
+
+---
+
+## Finding 4 (DECIDED: Option C — diff ModuleInfo.file vs fs walk)
+
+**Severity: Under-specification**
+
+The plan said: *"walk every `.rs` under the crate source dir, check whether the resolver included it."* This is not specific enough to implement.
+
+Three approaches exist:
+
+| Approach | Correctness | Cost |
+|---|---|---|
+| **A:** Collect visited file set from `build_module_tree` return | Correct — uses actual parsed tree | Requires changing `build_module_tree`'s return signature |
+| **B:** Use `resolve_module_path` in reverse for each `.rs` file | Partial — proves file *could* be included, not that it *was* | No refactoring; misses deeply nested orphans |
+| **C:** Deduplicate `ModuleInfo.file` paths from already-returned modules | Correct — uses finalized module tree | No refactoring needed; path normalization required |
+
+**Decision: Option C (simplest correct approach):**
+1. After `build_module_tree` returns for a crate, collect all `ModuleInfo.file` values (excluding `<unresolved>`)
+2. Walk the crate's `src/` directory for all `*.rs` files
+3. Any `.rs` on disk absent from the file set = orphan → `OrphanFile` warning
+4. Exclude known non-module files: `build.rs`, `src/bin/*.rs`, test/example directories
+
+No refactoring of `build_module_tree`'s return type needed — the file set is already in the returned `Vec<ModuleInfo>`.
+
+**Amendment applied:** Yes — §"validate rules" table updated for `OrphanFile`.
+
+---
+
+## Finding 5 (DECIDED: Option B — derive_from_crates before builder)
+
+**Severity: Implementation friction**
+
+The plan said `indexes::derive(&WorkspaceMap)` but the map is built via builder at the end of `run()`. The indexes need to go INTO the map.
+
+**Options:**
+- **Option A:** Build map first, mutate afterward — works mechanically (all fields are `pub`) but awkward
+- **Option B (cleaner):** `derive_from_crates(&[CrateInfo])` before builder, pass results directly to builder
+
+**Decision: Option B.** Matches how `cross_refs::compute` works today (takes `&[CrateInfo]` before the map exists).
+
+**Amendments applied:**
+- Files table: updated `src/indexes.rs` and `src/lib.rs` rows with new signature
+
+---
+
+## Finding 6 (RESOLVED — user clarification)
+
+**Original critique:** "O(1) lookup" for `lookup --file` is O(1)+O(n) internally (hashmap hit + linear ModuleInfo scan).
+
+**User clarification:** The O(1) stands for "the agent needs one call to get the correct, desired answer for its query." It's about **agent interaction cost** (one tool call → one answer), not internal algorithmic complexity. The internal O(n) scan is irrelevant to the agent interaction model.
+
+**Resolution:** Design rationale is correct under this framing. Withdrawn — no plan changes needed.
+
+---
+
+## Finding 8 (DECIDED: Accept — document as human tool)
+
+**Severity: Efficiency**
+
+`lookup` pays full AST parse per invocation. If the plan-decomposer calls `lookup` 5 times, that's 5x parse cost for 5 hashmap lookups. Without a cache, this wastes time for pipeline use.
+
+**Decision:** For the MVP, document `lookup` as a **human CLI convenience**. Pipeline agents should embed flat indexes in their context window (single JSON load) rather than making per-symbol CLI calls. Revisit `lookup` as a pipeline tool only after the cache layer ships.
+
+**Amendment applied:** §"lookup semantics" — added note about human-convenience scope.
+
+---
+
+## Finding 9 (DECIDED: Specify key format)
+
+**Severity: Under-specification**
+
+The plan says `symbols` keyed by `"crate::module::Name"` but root-module items (in `lib.rs`) have no module segment.
+
+**Decision:**
+- Root-module items: `"crate::Name"` (2 segments)
+- Nested-module items: `"crate::module::Name"` (3+ segments)
+- Parsers split on `::`: first segment = crate name, last = item name, middle = module path
+
+**Amendment applied:** §"Schema additions" — added key-format comment to `SymbolEntry`.
+
+---
+
+## Finding 10 (DECIDED: Fix)
+
+**Severity: Latent bug**
+
+`lib.rs:149-152`: `WorkspaceInfo.root` is always `"."`. With subcommands, input `PATH` can be a subdirectory (since `find_workspace_root` walks up), but file paths are relative to the **actual** workspace root. If a consumer tries to resolve file paths using `root` as base, they get wrong paths.
+
+**Decision:** Set `root` to the discovered workspace root. One-line fix in `lib.rs` in the WorkspaceInfo builder call.
+
+**Amendment applied:** Added to `src/lib.rs` row in files table.
+
+---
+
+## Finding 11 (DECIDED: Fix during refactor)
+
+**Severity: Existing bugs**
+
+- **Bug A** (`module_tree.rs:252`): `errors.clone()` at every recursion level duplicates errors O(n²) in memory. For a 4-level tree with errors at each level, waste compounds. `OrphanFile` will add more errors, making this worse.
+- **Bug B** (`module_tree.rs:33`): `crate_root.parent().unwrap_or(crate_root)` — if `crate_root` somehow has no parent, `parent_dir` falls back to root itself, which is semantically wrong for submodule resolution.
+
+**Decision:** Fix both during the planned `DiagnosticKind` mechanical refactor. The code is already being touched.
+
+**Amendment applied:** `src/module_tree.rs` row in files table now includes "Fix error-vec cloning at line 252 and fragile `unwrap_or` at line 33."
+
+---
+
+## Summary of amendments applied to MVP_PLAN.md
+
+| Amendment | Section | Change |
+|---|---|---|
+| A: `index --validate` | §CLI, §Files, §Pipeline, §Verification | Removed `validate` subcommand; merged as `index --validate` flag |
+| B: 5-case DeadReExport | §Validate rules | Algorithm now handles 5 distinct re-export forms |
+| C: OrphanFile Option C | §Validate rules | Algorithm now specified as ModuleInfo.file set diff vs fs walk |
+| D: cross_references fix | §Schema additions | Documented re-key to fully-qualified path (collision fix) |
+| D: FileEntry inline-module | §Schema additions | Added inline-module exclusion note |
+| E: derive_from_crates | §Files (indexes.rs, lib.rs) | Changed signature from `derive(&WorkspaceMap)` to `derive_from_crates(&[CrateInfo])` |
+| F: lookup as human tool | §Lookup semantics | Added paragraph about human-convenience scope |
+| G: symbols key format | §Schema additions | Specified `crate::Name` (root) vs `crate::mod::Name` (nested) |
+| H: WorkspaceInfo.root fix | §Files (lib.rs) | Added fix note for hardcoded `"."` root |
+| I: module_tree.rs bugs | §Files (module_tree.rs) | Added error-cloning and `unwrap_or` fixes |
+| J: Verification & D2 | §Verification, §Locked decisions | Updated all `validate` refs → `index --validate` |
+
+**Outstanding items for future review rounds (from this review):**
+- `cross_references.types` re-keying: implementation decision needed (fully-qualified key vs `Vec<TypeRef>` per short name)
+- `files` inline-module handling: implementation decision needed (exclusion vs `Vec<FileEntry>`)
+- `lookup` cache: revisit when a measured pipeline use case demands it
diff --git a/plans/phase-0.1.toml b/plans/phase-0.1/phase-0.1.toml
similarity index 100%
rename from plans/phase-0.1.toml
rename to plans/phase-0.1/phase-0.1.toml
diff --git a/src/cross_refs.rs b/src/cross_refs.rs
index 5958621..73ab748 100644
--- a/src/cross_refs.rs
+++ b/src/cross_refs.rs
@@ -5,20 +5,24 @@ use std::collections::BTreeMap;
 ///
 /// For every public item in every crate, matches it against imports from
 /// other crates. Populates each `CrateInfo.cross_crate_imports` and returns
-/// the global `CrossReferences` map (keyed by type/symbol name).
+/// the global `CrossReferences` map (keyed by canonical path).
 ///
 /// Takes `&mut [CrateInfo]` so it can write `cross_crate_imports` into each
 /// crate while building the global cross-reference map.
 pub fn compute(crates: &mut [CrateInfo]) -> CrossReferences {
-    // Build a map: crate_name -> set of public item names.
+    // Build a map: crate_name -> set of (canonical_path, kind) pairs.
+    // canonical_path = "{module.path}::{item.name}" (e.g., "core::Task").
     let crate_exports: BTreeMap<String, Vec<(String, String)>> = crates
         .iter()
         .map(|c| {
             let items: Vec<(String, String)> = c
                 .modules
                 .iter()
-                .flat_map(|m| &m.public_items)
-                .map(|item| (item.name.clone(), item.kind_to_string()))
+                .flat_map(|m| {
+                    m.public_items
+                        .iter()
+                        .map(move |item| (format!("{}::{}", m.path, item.name), item.kind_to_string()))
+                })
                 .collect();
             (c.name.clone(), items)
         })
@@ -28,8 +32,8 @@ pub fn compute(crates: &mut [CrateInfo]) -> CrossReferences {
 
     // Initialize TypeRef entries for every exported public item.
     for (crate_name, items) in &crate_exports {
-        for (item_name, kind) in items {
-            let entry = types_map.entry(item_name.clone()).or_insert_with(|| {
+        for (canonical_path, kind) in items {
+            let entry = types_map.entry(canonical_path.clone()).or_insert_with(|| {
                 TypeRef::builder()
                     .crate_name(crate_name.clone())
                     .kind(kind.clone())
@@ -41,6 +45,15 @@ pub fn compute(crates: &mut [CrateInfo]) -> CrossReferences {
         }
     }
 
+    // Build a reverse lookup: short_name -> Vec<canonical_path> for import matching.
+    let mut reverse_lookup: BTreeMap<String, Vec<String>> = BTreeMap::new();
+    for canonical_path in types_map.keys() {
+        if let Some(pos) = canonical_path.rfind("::") {
+            let short_name = &canonical_path[pos + 2..];
+            reverse_lookup.entry(short_name.to_string()).or_default().push(canonical_path.clone());
+        }
+    }
+
     // Scan each crate's imports to find cross-crate references.
     for crate_info in crates.iter_mut() {
         let my_name = crate_info.name.clone();
@@ -75,11 +88,18 @@ pub fn compute(crates: &mut [CrateInfo]) -> CrossReferences {
                         line: import.line,
                     });
 
-                    // Update global cross-references.
-                    if let Some(type_ref) = types_map.get_mut(&symbol) {
-                        let importer_label = format!("{}:{}", my_name, module.path);
-                        if !type_ref.imported_by.contains(&importer_label) {
-                            type_ref.imported_by.push(importer_label.clone());
+                    // Update global cross-references via reverse lookup.
+                    if let Some(candidates) = reverse_lookup.get(&symbol) {
+                        for canonical_path in candidates {
+                            // Only match entries from the target crate.
+                            if let Some(type_ref) = types_map.get_mut(canonical_path)
+                                && type_ref.crate_name == first_seg
+                            {
+                                let importer_label = format!("{}:{}", my_name, module.path);
+                                if !type_ref.imported_by.contains(&importer_label) {
+                                    type_ref.imported_by.push(importer_label.clone());
+                                }
+                            }
                         }
                     }
                 }
@@ -152,9 +172,11 @@ mod tests {
             line: 1,
         });
         let refs = compute(&mut crates);
-        // Task should be in cross-references
-        assert!(refs.types.contains_key("Task"));
-        let task_ref = &refs.types["Task"];
+        // Types map keys are now canonical paths (module.path::item.name).
+        // With empty module path, key is "::Task".
+        let task_key = "::Task".to_string();
+        assert!(refs.types.contains_key(&task_key), "expected key {}", task_key);
+        let task_ref = &refs.types[&task_key];
         assert_eq!(task_ref.crate_name, "core");
         // engine should have a cross_crate_import
         assert_eq!(crates[1].cross_crate_imports.len(), 1);
diff --git a/src/file_parser.rs b/src/file_parser.rs
index f927aef..c69a909 100644
--- a/src/file_parser.rs
+++ b/src/file_parser.rs
@@ -1,5 +1,5 @@
 use crate::schema::{
-    ErrorEntry, ErrorSeverity, FileInfo, ImplInfo, ImplItem, ImplItemKind, Import,
+    DiagnosticKind, ErrorEntry, ErrorSeverity, FileInfo, ImplInfo, ImplItem, ImplItemKind, Import,
     ItemAttrs, ItemKind, PublicItem, ReExport, SubmoduleDecl,
 };
 use std::path::Path;
@@ -92,7 +92,7 @@ pub(crate) fn build_parse_error_entry(path: &Path, err: &SynParseError) -> Error
         .line(err.line)
         .message(err.message.clone())
         .severity(ErrorSeverity::Error)
-        .kind("syn_parse_error".to_string())
+        .kind(DiagnosticKind::SynParseError)
         .build()
 }
 
@@ -786,7 +786,7 @@ mod tests {
         let entry = build_parse_error_entry(&path, &err);
         assert_eq!(entry.file, "test.rs");
         assert_eq!(entry.line, 5);
-        assert_eq!(entry.kind, "syn_parse_error");
+        assert_eq!(entry.kind, crate::schema::DiagnosticKind::SynParseError);
         assert_eq!(entry.severity, ErrorSeverity::Error);
     }
 }
diff --git a/src/indexes.rs b/src/indexes.rs
new file mode 100644
index 0000000..e1e87eb
--- /dev/null
+++ b/src/indexes.rs
@@ -0,0 +1,285 @@
+use crate::schema::{
+    CrateInfo, CanonicalPath, FileEntry, SymbolEntry, WorkspaceRelativePath,
+};
+use std::collections::BTreeMap;
+
+type SymbolsIndex = BTreeMap<CanonicalPath, SymbolEntry>;
+type NameIndex = BTreeMap<String, Vec<CanonicalPath>>;
+type FilesIndex = BTreeMap<WorkspaceRelativePath, FileEntry>;
+
+/// Build flat indexes from a slice of `CrateInfo`.
+///
+/// Returns three maps:
+/// 1. `symbols` — canonical-path-keyed map of all public items
+/// 2. `name_index` — short-name to canonical-path list (for disambiguation)
+/// 3. `files` — workspace-relative-path-keyed map of all source files with module metadata
+///
+/// The symbols and `name_index` are constructed via an iterator pipeline;
+/// the files map uses a for-loop due to the inline-detection accumulator.
+///
+/// # Inline module detection
+///
+/// This function relies on depth-first traversal order from `build_module_tree`.
+/// The heuristic: the first module encountered per file path is the primary
+/// (declared in a separate file); subsequent modules sharing the same file
+/// are inline modules and are excluded from the files map.
+#[allow(clippy::type_complexity)]
+#[must_use]
+pub fn derive_from_crates(
+    crates: &[CrateInfo],
+) -> (SymbolsIndex, NameIndex, FilesIndex) {
+    // ── symbols & name_index via iterator pipeline ────────────────────
+
+    let (syms, mut nidx): (
+        BTreeMap<CanonicalPath, SymbolEntry>,
+        BTreeMap<String, Vec<CanonicalPath>>,
+    ) = crates
+        .iter()
+        .flat_map(|c| {
+            c.modules
+                .iter()
+                .flat_map(move |m| {
+                    m.public_items
+                        .iter()
+                        .map(move |item| (c.name.as_str(), m, item))
+                })
+        })
+        .fold(
+            (
+                BTreeMap::<CanonicalPath, SymbolEntry>::new(),
+                BTreeMap::<String, Vec<CanonicalPath>>::new(),
+            ),
+            |(mut syms, mut nidx), (crate_name, m, item)| {
+                let canonical = CanonicalPath::from(format!("{}::{}", m.path, item.name));
+                syms.insert(
+                    canonical.clone(),
+                    SymbolEntry::builder()
+                        .crate_name(crate_name.to_string())
+                        .module(m.path.clone())
+                        .file(m.file.clone())
+                        .line(item.line)
+                        .kind(item.kind.clone())
+                        .build(),
+                );
+                nidx.entry(item.name.clone()).or_default().push(canonical);
+                (syms, nidx)
+            },
+        );
+
+    // Sort name_index values for determinism.
+    for val in nidx.values_mut() {
+        val.sort();
+    }
+
+    // ── files via for-loop (inline detection requires accumulator) ───
+
+    let mut files = BTreeMap::new();
+    let mut seen_files: BTreeMap<String, String> = BTreeMap::new(); // file -> module_path of first owner
+
+    for crate_info in crates {
+        for module in &crate_info.modules {
+            if module.file == "<unresolved>" {
+                continue;
+            }
+
+            let file = &module.file;
+            if seen_files.contains_key(file) {
+                // Inline module — shares file with an earlier (parent) module.
+                continue;
+            }
+            seen_files.insert(file.clone(), module.path.clone());
+
+            let is_crate_root = module.path == crate_info.name;
+
+            // Compute parent_module_file by walking the crate's modules.
+            let parent_module_file = if module.path.is_empty() {
+                None // Root module has no parent.
+            } else {
+                // Strip last ::name segment to get parent path.
+                if let Some(pos) = module.path.rfind("::") {
+                    let parent_path = &module.path[..pos];
+                    // Find the module entry whose path matches the parent.
+                    crate_info
+                        .modules
+                        .iter()
+                        .find(|m| m.path == parent_path)
+                        .map(|m| m.file.clone())
+                } else {
+                    None
+                }
+            };
+
+            let files_entry = match parent_module_file {
+                Some(pf) => FileEntry::builder()
+                    .module_path(module.path.clone())
+                    .parent_module_file(pf)
+                    .is_crate_root(is_crate_root)
+                    .build(),
+                None => FileEntry::builder()
+                    .module_path(module.path.clone())
+                    .is_crate_root(is_crate_root)
+                    .build(),
+            };
+            files.insert(
+                WorkspaceRelativePath(file.clone()),
+                files_entry,
+            );
+        }
+    }
+
+    (syms, nidx, files)
+}
+
+// ── Tests ───────────────────────────────────────────────────────────────
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use crate::schema::{
+        CrateType, DepInfo, ItemAttrs, ItemKind, ModuleInfo,
+        PackageInfo, PublicItem,
+    };
+
+    fn make_crate(
+        name: &str,
+        modules: Vec<ModuleInfo>,
+    ) -> CrateInfo {
+        CrateInfo::builder()
+            .name(name.to_string())
+            .root("src/lib.rs".to_string())
+            .package(
+                PackageInfo::builder()
+                    .name(name.to_string())
+                    .version("0.1.0".to_string())
+                    .edition("2021".to_string())
+                    .crate_type(CrateType::Lib)
+                    .build(),
+            )
+            .modules(modules)
+            .deps(DepInfo::default())
+            .build()
+    }
+
+    fn make_module(path: &str, file: &str, items: Vec<PublicItem>) -> ModuleInfo {
+        ModuleInfo::builder()
+            .path(path.to_string())
+            .file(file.to_string())
+            .visibility("pub".to_string())
+            .public_items(items)
+            .build()
+    }
+
+    fn make_item(name: &str, kind: ItemKind, line: usize) -> PublicItem {
+        PublicItem::builder()
+            .name(name.to_string())
+            .kind(kind)
+            .file("src/lib.rs".to_string())
+            .line(line)
+            .visibility("pub".to_string())
+            .generics(String::new())
+            .attrs(ItemAttrs::default())
+            .build()
+    }
+
+    #[test]
+    fn symbol_index_single_root_item() {
+        let module = make_module(
+            "mycrate",
+            "src/lib.rs",
+            vec![make_item("MyStruct", ItemKind::Struct, 1)],
+        );
+        let crates = vec![make_crate("mycrate", vec![module])];
+        let (syms, _, _) = derive_from_crates(&crates);
+
+        let key = CanonicalPath::from("mycrate::MyStruct".to_string());
+        let entry = syms.get(&key).expect("expected mycrate::MyStruct");
+        assert_eq!(entry.crate_name, "mycrate");
+        assert_eq!(entry.module, "mycrate");
+        assert_eq!(entry.kind, ItemKind::Struct);
+    }
+
+    #[test]
+    fn symbol_index_nested_module_item() {
+        let root = make_module("mycrate", "src/lib.rs", vec![]);
+        let sub = make_module(
+            "mycrate::sub",
+            "src/sub.rs",
+            vec![make_item("Helper", ItemKind::Fn, 5)],
+        );
+        let crates = vec![make_crate("mycrate", vec![root, sub])];
+        let (syms, _, _) = derive_from_crates(&crates);
+
+        let key = CanonicalPath::from("mycrate::sub::Helper".to_string());
+        let entry = syms.get(&key).expect("expected mycrate::sub::Helper");
+        assert_eq!(entry.module, "mycrate::sub");
+        assert_eq!(entry.file, "src/sub.rs");
+    }
+
+    #[test]
+    fn name_index_multi_crate_collision() {
+        let root_a = make_module(
+            "alpha",
+            "src/lib.rs",
+            vec![make_item("Foo", ItemKind::Struct, 1)],
+        );
+        let root_b = make_module(
+            "beta",
+            "src/lib.rs",
+            vec![make_item("Foo", ItemKind::Fn, 10)],
+        );
+        let crates = vec![make_crate("alpha", vec![root_a]), make_crate("beta", vec![root_b])];
+        let (_, nidx, _) = derive_from_crates(&crates);
+
+        let candidates = nidx.get("Foo").expect("expected Foo in name_index");
+        assert_eq!(candidates.len(), 2);
+        assert!(candidates.iter().any(|p| p.as_ref() == "alpha::Foo"));
+        assert!(candidates.iter().any(|p| p.as_ref() == "beta::Foo"));
+    }
+
+    #[test]
+    fn files_excludes_inline_module() {
+        // Root module in lib.rs, inline sub in lib.rs (same file).
+        let root = make_module(
+            "mycrate",
+            "src/lib.rs",
+            vec![make_item("Item", ItemKind::Struct, 1)],
+        );
+        let inline = make_module(
+            "mycrate::inner",
+            "src/lib.rs", // same file as root
+            vec![make_item("InnerItem", ItemKind::Struct, 5)],
+        );
+        let crates = vec![make_crate("mycrate", vec![root, inline])];
+        let (_, _, files) = derive_from_crates(&crates);
+
+        // Only one file entry should exist (the root).
+        assert_eq!(files.len(), 1);
+        let key = WorkspaceRelativePath::from("src/lib.rs".to_string());
+        let entry = files.get(&key).expect("expected src/lib.rs in files");
+        assert!(entry.is_crate_root);
+    }
+
+    #[test]
+    fn files_parent_module_file() {
+        let root = make_module("mycrate", "src/lib.rs", vec![]);
+        let sub = make_module(
+            "mycrate::sub",
+            "src/sub.rs",
+            vec![make_item("X", ItemKind::Struct, 1)],
+        );
+        let crates = vec![make_crate("mycrate", vec![root, sub])];
+        let (_, _, files) = derive_from_crates(&crates);
+
+        let root_key = WorkspaceRelativePath::from("src/lib.rs".to_string());
+        let root_entry = files.get(&root_key).expect("expected root");
+        assert!(root_entry.parent_module_file.is_none(), "root should have no parent");
+
+        let sub_key = WorkspaceRelativePath::from("src/sub.rs".to_string());
+        let sub_entry = files.get(&sub_key).expect("expected sub");
+        assert_eq!(
+            sub_entry.parent_module_file,
+            Some("src/lib.rs".to_string()),
+            "sub should have lib.rs as parent"
+        );
+    }
+}
diff --git a/src/lib.rs b/src/lib.rs
index 7b47a99..0e1dfa7 100644
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -3,9 +3,12 @@
 pub mod cargo_info;
 pub mod cross_refs;
 pub mod file_parser;
+pub mod indexes;
+pub mod lookup;
 pub mod module_tree;
 pub mod render;
 pub mod schema;
+pub mod validate;
 pub mod workspace;
 
 pub use schema::Config;
@@ -13,17 +16,16 @@ pub use schema::Config;
 use anyhow::Context;
 use rayon::prelude::*;
 use schema::{
-    CrateInfo, CrateType, ErrorEntry, ErrorSeverity, ModuleInfo, WorkspaceInfo,
-    WorkspaceMap,
+    CrateInfo, CrateType, DiagnosticKind, ErrorEntry, ErrorSeverity, ModuleInfo,
+    WorkspaceInfo, WorkspaceMap,
 };
 use std::path::Path;
 
-/// Run the full workspace mapping pipeline.
+/// Build a `WorkspaceMap` from the given config, without rendering.
 ///
-/// 1. Discover workspace root and member crates.
-/// 2. Process each crate in parallel (Cargo.toml parsing + module tree).
-/// 3. Compute cross-crate references.
-/// 4. Render JSON to stdout or the configured output file.
+/// This function contains all pipeline logic up to and including
+/// `WorkspaceMap` construction — workspace discovery, per-crate processing,
+/// cross-refs computation, index derivation, optional validation, and map building.
 ///
 /// # Errors
 ///
@@ -31,7 +33,7 @@ use std::path::Path;
 /// Cargo.toml is missing a `[workspace]` section, member crates cannot be
 /// parsed, or the JSON output cannot be written.
 #[allow(clippy::too_many_lines)]
-pub fn run(config: &Config) -> anyhow::Result<()> {
+pub fn build_map(config: &Config) -> anyhow::Result<WorkspaceMap> {
     let workspace_root = workspace::find_workspace_root(&config.workspace_path)?;
     let member_dirs = workspace::enumerate_members(&workspace_root)?;
 
@@ -50,7 +52,7 @@ pub fn run(config: &Config) -> anyhow::Result<()> {
                         .file(cargo_toml.to_string_lossy().to_string())
                         .message(format!("failed to parse Cargo.toml: {e}"))
                         .severity(ErrorSeverity::Error)
-                        .kind("toml_parse_error".to_string())
+                        .kind(DiagnosticKind::TomlParseError)
                         .cause(e.to_string())
                         .build());
                     return (None, crate_errors);
@@ -63,7 +65,7 @@ pub fn run(config: &Config) -> anyhow::Result<()> {
                     .file(dir.to_string_lossy().to_string())
                     .message("no crate entry points found".to_string())
                     .severity(ErrorSeverity::Warning)
-                    .kind("missing_crate_roots".to_string())
+                    .kind(DiagnosticKind::MissingCrateRoots)
                     .build());
                 return (None, crate_errors);
             }
@@ -85,14 +87,7 @@ pub fn run(config: &Config) -> anyhow::Result<()> {
                 collected_errors.extend(e);
             }
             for err in collected_errors {
-                if err.kind.is_empty() {
-                    crate_errors.push(ErrorEntry {
-                        kind: "module_tree_error".to_string(),
-                        ..err
-                    });
-                } else {
-                    crate_errors.push(err);
-                }
+                crate_errors.push(err);
             }
 
             // Relativize all paths to the workspace root.
@@ -130,10 +125,12 @@ pub fn run(config: &Config) -> anyhow::Result<()> {
     let mut crate_infos: Vec<CrateInfo> = Vec::new();
 
     for (info, errs) in results {
+        // Extend errors in both branches before checking info.
+        // `errs` is moved by `extend` — this is fine because results is consumed by the for loop (move iteration).
         if let Some(ci) = info {
-            crate_errors.extend(errs);
             crate_infos.push(ci);
         }
+        crate_errors.extend(errs);
     }
 
     // Deterministic sort by crate name.
@@ -141,13 +138,22 @@ pub fn run(config: &Config) -> anyhow::Result<()> {
 
     let cross_refs = cross_refs::compute(&mut crate_infos);
 
+    // Derive flat indexes from crate info.
+    let (symbols, name_index, files) = indexes::derive_from_crates(&crate_infos);
+
+    // Run validation if enabled.
+    if config.validate {
+        let validate_findings = validate::validate(&crate_infos, &symbols, &workspace_root);
+        crate_errors.extend(validate_findings);
+    }
+
     let workspace_name = workspace_root
         .file_name()
         .map(|n| n.to_string_lossy().to_string())
         .unwrap_or_default();
 
     let workspace_info = WorkspaceInfo::builder()
-        .root(".".to_string())
+        .root(workspace_root.to_string_lossy().to_string())
         .workspace_name(workspace_name)
         .build();
 
@@ -155,10 +161,31 @@ pub fn run(config: &Config) -> anyhow::Result<()> {
         .workspace(workspace_info)
         .crates(crate_infos)
         .cross_references(cross_refs)
+        .symbols(symbols)
+        .name_index(name_index)
+        .files(files)
         .errors(crate_errors)
         .workspace_root(workspace_root.clone())
         .build();
 
+    Ok(map)
+}
+
+/// Run the full workspace mapping pipeline.
+///
+/// 1. Discover workspace root and member crates.
+/// 2. Process each crate in parallel (Cargo.toml parsing + module tree).
+/// 3. Compute cross-crate references.
+/// 4. Render JSON to stdout or the configured output file.
+///
+/// # Errors
+///
+/// Returns an error if the workspace root cannot be found, the workspace
+/// Cargo.toml is missing a `[workspace]` section, member crates cannot be
+/// parsed, or the JSON output cannot be written.
+pub fn run(config: &Config) -> anyhow::Result<()> {
+    let map = build_map(config)?;
+
     if let Some(ref output_path) = config.output_path {
         let file = std::fs::File::create(output_path)
             .with_context(|| format!("failed to create output file: {}", output_path.display()))?;
diff --git a/src/lookup.rs b/src/lookup.rs
new file mode 100644
index 0000000..266e496
--- /dev/null
+++ b/src/lookup.rs
@@ -0,0 +1,205 @@
+use crate::schema::{
+    FileEntry, ItemKind, ModuleInfo, SymbolEntry, WorkspaceMap,
+    WorkspaceRelativePath,
+};
+
+/// Result of a symbol lookup.
+#[derive(Debug, Clone, serde::Serialize)]
+#[serde(tag = "status")]
+pub enum SymbolLookupResult {
+    #[serde(rename = "found")]
+    Found(SymbolEntry),
+    #[serde(rename = "ambiguous")]
+    Ambiguous {
+        name: String,
+        candidates: Vec<DisambiguationHint>,
+    },
+    #[serde(rename = "not_found")]
+    NotFound,
+}
+
+/// Hint for disambiguating a symbol name collision.
+#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
+#[serde(rename_all = "camelCase")]
+pub struct DisambiguationHint {
+    pub canonical_path: String,
+    pub crate_name: String,
+    pub kind: ItemKind,
+    pub file: String,
+    pub line: usize,
+}
+
+/// Result of a file lookup.
+#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
+#[serde(rename_all = "camelCase")]
+pub struct FileLookupResult {
+    pub file_entry: FileEntry,
+    pub primary_module: ModuleInfo,
+    #[serde(skip_serializing_if = "Vec::is_empty")]
+    pub inline_modules: Vec<ModuleInfo>,
+}
+
+/// Look up a symbol by name in the workspace map's `name_index`.
+///
+/// Returns `Found` if exactly one canonical path is found,
+/// `Ambiguous` if multiple candidates exist, or `NotFound` otherwise.
+#[must_use]
+pub fn lookup_symbol(map: &WorkspaceMap, name: &str) -> SymbolLookupResult {
+    let Some(candidates) = map.name_index.get(name) else {
+        return SymbolLookupResult::NotFound;
+    };
+
+    if candidates.len() == 1 {
+        let canonical = &candidates[0];
+        if let Some(entry) = map.symbols.get(canonical) {
+            return SymbolLookupResult::Found(entry.clone());
+        }
+    }
+
+    if candidates.len() > 1 {
+        let mut hints = Vec::new();
+        for canonical in candidates {
+            if let Some(entry) = map.symbols.get(canonical) {
+                hints.push(
+                    DisambiguationHint::builder()
+                        .canonical_path(canonical.0.clone())
+                        .crate_name(entry.crate_name.clone())
+                        .kind(entry.kind.clone())
+                        .file(entry.file.clone())
+                        .line(entry.line)
+                        .build(),
+                );
+            }
+        }
+        return SymbolLookupResult::Ambiguous {
+            name: name.to_string(),
+            candidates: hints,
+        };
+    }
+
+    SymbolLookupResult::NotFound
+}
+
+/// Look up a file by workspace-relative path.
+///
+/// Returns `Some` with the file entry and associated modules if found,
+/// or `None` if the file is not in the files index.
+#[must_use]
+pub fn lookup_file(map: &WorkspaceMap, file: &str) -> Option<FileLookupResult> {
+    let key = WorkspaceRelativePath(file.to_string());
+    let file_entry = map.files.get(&key)?;
+
+    let mut primary_module: Option<ModuleInfo> = None;
+    let mut inline_modules: Vec<ModuleInfo> = Vec::new();
+
+    for crate_info in &map.crates {
+        for module in &crate_info.modules {
+            if module.file != file {
+                continue;
+            }
+            if module.path == file_entry.module_path {
+                primary_module = Some(module.clone());
+            } else {
+                inline_modules.push(module.clone());
+            }
+        }
+    }
+
+    let primary_module = primary_module?;
+
+    Some(FileLookupResult::builder()
+        .file_entry(file_entry.clone())
+        .primary_module(primary_module)
+        .inline_modules(inline_modules)
+        .build())
+}
+
+// ── Tests ───────────────────────────────────────────────────────────────
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use crate::schema::{
+        CrateInfo, CrateType, CrossReferences, DepInfo, ItemAttrs, ModuleInfo,
+        PackageInfo, PublicItem, WorkspaceInfo,
+    };
+
+   fn make_test_map() -> WorkspaceMap {
+        let item = PublicItem::builder()
+            .name("Task".to_string())
+            .kind(ItemKind::Struct)
+            .file("core/src/lib.rs".to_string())
+            .line(1)
+            .visibility("pub".to_string())
+            .generics(String::new())
+            .attrs(ItemAttrs::default())
+            .build();
+        let module = ModuleInfo::builder()
+            .path("core".to_string())
+            .file("core/src/lib.rs".to_string())
+            .visibility("pub".to_string())
+            .public_items(vec![item])
+            .build();
+        let crate_info = CrateInfo::builder()
+            .name("core".to_string())
+            .root("core".to_string())
+            .package(PackageInfo::builder()
+                .name("core".to_string())
+                .version("0.1.0".to_string())
+                .edition("2021".to_string())
+                .crate_type(CrateType::Lib)
+                .build())
+            .modules(vec![module])
+            .deps(DepInfo::default())
+            .build();
+
+        let (symbols, name_index, files) = crate::indexes::derive_from_crates(&[crate_info.clone()]);
+
+        WorkspaceMap::builder()
+            .workspace(WorkspaceInfo::builder()
+                .root("/tmp/test".to_string())
+                .workspace_name("test".to_string())
+                .build())
+            .crates(vec![crate_info])
+            .cross_references(CrossReferences::default())
+            .symbols(symbols)
+            .name_index(name_index)
+            .files(files)
+            .workspace_root(std::path::PathBuf::from("/tmp/test"))
+            .build()
+    }
+
+    #[test]
+    fn lookup_symbol_found() {
+        let map = make_test_map();
+        let result = lookup_symbol(&map, "Task");
+        match result {
+            SymbolLookupResult::Found(entry) => {
+                assert_eq!(entry.crate_name, "core");
+                assert_eq!(entry.kind, ItemKind::Struct);
+            }
+            other => panic!("expected Found, got {other:?}"),
+        }
+    }
+
+    #[test]
+    fn lookup_symbol_not_found() {
+        let map = make_test_map();
+        let result = lookup_symbol(&map, "DoesNotExist");
+        assert!(matches!(result, SymbolLookupResult::NotFound));
+    }
+
+    #[test]
+    fn lookup_file_found() {
+        let map = make_test_map();
+        let result = lookup_file(&map, "core/src/lib.rs");
+        assert!(result.is_some(), "expected file to be found");
+    }
+
+    #[test]
+    fn lookup_file_not_found() {
+        let map = make_test_map();
+        let result = lookup_file(&map, "nonexistent.rs");
+        assert!(result.is_none());
+    }
+}
diff --git a/src/main.rs b/src/main.rs
index 0d9893c..7cd18b1 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,6 +1,32 @@
-use clap::Parser;
+use clap::{Parser, Subcommand};
 use std::path::PathBuf;
 
+#[derive(Subcommand)]
+enum Command {
+    /// Generate a JSON map of a Rust workspace's public API surface
+    Index {
+        /// Path to the workspace root or any directory within it
+        path: PathBuf,
+        /// Write JSON output to file instead of stdout
+        #[arg(short = 'o', long = "output", value_name = "FILE")]
+        output: Option<PathBuf>,
+        /// Run validation checks (orphan files, dead re-exports)
+        #[arg(long)]
+        validate: bool,
+    },
+    /// Look up a symbol or file in a previously generated workspace map
+    Lookup {
+        /// Path to the workspace root or any directory within it
+        path: PathBuf,
+        /// Look up by symbol name
+        #[arg(long, conflicts_with = "file")]
+        symbol: Option<String>,
+        /// Look up by file path
+        #[arg(long, conflicts_with = "symbol")]
+        file: Option<String>,
+    },
+}
+
 #[derive(Parser)]
 #[command(
     name = "rust-workspace-map",
@@ -8,34 +34,138 @@ use std::path::PathBuf;
     about = "Generate a JSON map of a Rust workspace's public API surface"
 )]
 struct Cli {
-    /// Path to the workspace root or any directory within it
-    #[arg(value_name = "PATH")]
-    path: PathBuf,
-
-    /// Write JSON output to file instead of stdout
-    #[arg(short = 'o', long = "output", value_name = "FILE")]
-    output: Option<PathBuf>,
+    #[command(subcommand)]
+    command: Command,
 }
 
-fn main() -> anyhow::Result<()> {
+fn main() {
     let cli = Cli::parse();
 
-    let workspace_path = std::path::absolute(&cli.path)
-        .map_err(|e| anyhow::anyhow!("invalid path {}: {}", cli.path.display(), e))?;
+    match cli.command {
+        Command::Index { path, output, validate } => {
+            let workspace_path = std::path::absolute(&path)
+                .unwrap_or_else(|e| {
+                    eprintln!("invalid path {}: {}", path.display(), e);
+                    std::process::exit(1);
+                });
 
-    let config = match cli.output {
-        Some(ref output) => {
-            rust_workspace_map::Config::builder()
-                .workspace_path(workspace_path)
-                .output_path(output.clone())
-                .build()
+            let config = if let Some(ref output) = output {
+                rust_workspace_map::Config::builder()
+                    .workspace_path(workspace_path)
+                    .output_path(output.clone())
+                    .validate(validate)
+                    .build()
+            } else {
+                rust_workspace_map::Config::builder()
+                    .workspace_path(workspace_path)
+                    .validate(validate)
+                    .build()
+            };
+
+            match rust_workspace_map::build_map(&config) {
+                Ok(map) => {
+                    let exit_code = if validate {
+                        map.errors.iter().any(|e| {
+                            matches!(
+                                e.kind,
+                                rust_workspace_map::schema::DiagnosticKind::OrphanFile
+                                    | rust_workspace_map::schema::DiagnosticKind::DeadReExport
+                            ) && e.severity == rust_workspace_map::schema::ErrorSeverity::Warning
+                        })
+                    } else {
+                        false
+                    };
+
+                    if let Some(ref output_path) = config.output_path {
+                        let file = match std::fs::File::create(output_path) {
+                            Ok(f) => f,
+                            Err(e) => {
+                                eprintln!("failed to create output file: {e}");
+                                std::process::exit(1);
+                            }
+                        };
+                        let writer = std::io::BufWriter::new(file);
+                        if rust_workspace_map::render::render_to_writer(&map, writer).is_err() {
+                            std::process::exit(1);
+                        }
+                    } else {
+                        let stdout = std::io::stdout();
+                        if rust_workspace_map::render::render_to_writer(&map, stdout.lock()).is_err() {
+                            std::process::exit(1);
+                        }
+                    }
+
+                    if exit_code {
+                        std::process::exit(2);
+                    }
+                }
+                Err(_) => {
+                    std::process::exit(1);
+                }
+            }
         }
-        None => {
-            rust_workspace_map::Config::builder()
+        Command::Lookup { path, symbol, file } => {
+            let workspace_path = std::path::absolute(&path)
+                .unwrap_or_else(|e| {
+                    eprintln!("invalid path {}: {}", path.display(), e);
+                    std::process::exit(1);
+                });
+
+            let config = rust_workspace_map::Config::builder()
                 .workspace_path(workspace_path)
-                .build()
-        }
-    };
+                .build();
 
-    rust_workspace_map::run(&config)
+            match rust_workspace_map::build_map(&config) {
+                Ok(map) => {
+                    match (symbol, file) {
+                        (Some(sym), None) => {
+                            let result = rust_workspace_map::lookup::lookup_symbol(&map, &sym);
+                            let json = match serde_json::to_string_pretty(&result) {
+                                Ok(j) => j,
+                                Err(_) => {
+                                    eprintln!("serialization error");
+                                    std::process::exit(1);
+                                }
+                            };
+                            println!("{json}");
+                            // Exit 1 on NotFound, 0 otherwise.
+                            if matches!(result, rust_workspace_map::lookup::SymbolLookupResult::NotFound) {
+                                std::process::exit(1);
+                            }
+                        }
+                        (None, Some(f)) => {
+                            match rust_workspace_map::lookup::lookup_file(&map, &f) {
+                                Some(result) => {
+                                    let json = match serde_json::to_string_pretty(&result) {
+                                        Ok(j) => j,
+                                        Err(_) => {
+                                            eprintln!("serialization error");
+                                            std::process::exit(1);
+                                        }
+                                    };
+                                    println!("{json}");
+                                }
+                                None => {
+                                    eprintln!("file not found: {f}");
+                                    std::process::exit(1);
+                                }
+                            }
+                        }
+                        (Some(_), Some(_)) => {
+                            // clap's conflicts_with handles this, but be defensive.
+                            eprintln!("cannot specify both --symbol and --file");
+                            std::process::exit(1);
+                        }
+                        (None, None) => {
+                            eprintln!("must specify either --symbol or --file");
+                            std::process::exit(1);
+                        }
+                    }
+                }
+                Err(_) => {
+                    std::process::exit(1);
+                }
+            }
+        }
+    }
 }
diff --git a/src/module_tree.rs b/src/module_tree.rs
index 0696c1f..78494a8 100644
--- a/src/module_tree.rs
+++ b/src/module_tree.rs
@@ -1,5 +1,5 @@
 use crate::file_parser;
-use crate::schema::{ErrorContext, ErrorEntry, ErrorSeverity, FileInfo, ModuleInfo, SubmoduleDecl};
+use crate::schema::{DiagnosticKind, ErrorContext, ErrorEntry, ErrorSeverity, FileInfo, ModuleInfo, SubmoduleDecl};
 use std::collections::HashSet;
 use std::path::{Path, PathBuf};
 
@@ -30,7 +30,7 @@ pub fn build_module_tree(
 ) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {
 
     let mut visited = HashSet::new();
-    let parent_dir = crate_root.parent().unwrap_or(crate_root);
+    let parent_dir = crate_root.parent().unwrap_or_else(|| Path::new("."));
 
     let parsed = file_parser::parse_file(crate_root);
     let mut errors: Vec<ErrorEntry> = Vec::new();
@@ -121,7 +121,7 @@ fn process_submodule(
             .file(String::new())
             .message(format!("orphaned module: {module_path}"))
             .severity(ErrorSeverity::Warning)
-            .kind("orphaned_module".to_string())
+            .kind(DiagnosticKind::OrphanedModule)
             .context(ErrorContext::builder()
                 .module_path(module_path.to_string())
                 .build())
@@ -158,7 +158,7 @@ fn process_submodule(
             .file(String::new())
             .message(format!("orphaned module: {module_path}"))
             .severity(ErrorSeverity::Warning)
-            .kind("orphaned_module".to_string())
+            .kind(DiagnosticKind::OrphanedModule)
             .context(ErrorContext::builder()
                 .module_path(module_path.to_string())
                 .build())
@@ -180,7 +180,7 @@ fn process_submodule(
     if let Some(ref err) = parsed.parse_error {
         errors.push(crate::file_parser::build_parse_error_entry(file_path, err));
     }
-    process_module_info(
+    let modules = process_module_info(
         module_path,
         file_path,
         visibility,
@@ -189,7 +189,8 @@ fn process_submodule(
         file_path.parent().unwrap_or(file_path),
         visited,
         &mut errors,
-    )
+    );
+    (modules, errors)
 }
 
 fn process_module_items(
@@ -207,7 +208,9 @@ fn process_module_items(
         submodules: file_parser::extract_submodules(items),
         impls: file_parser::extract_impls(items),
     };
-    process_module_info(module_path, file_path, visibility, &file_info, items, parent_dir, visited, &mut Vec::new())
+    let mut errs = Vec::new();
+    let modules = process_module_info(module_path, file_path, visibility, &file_info, items, parent_dir, visited, &mut errs);
+    (modules, errs)
 }
 
 #[allow(clippy::too_many_arguments)]
@@ -220,7 +223,7 @@ fn process_module_info(
     _parent_dir: &Path,
     visited: &mut HashSet<PathBuf>,
     errors: &mut Vec<ErrorEntry>,
-) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {
+) -> Vec<ModuleInfo> {
     let mut modules = vec![build_module_info(
         module_path,
         file_path,
@@ -237,7 +240,7 @@ fn process_module_info(
         }
         let child_path = format!("{}::{}", module_path, sub.name);
         let child_dir = file_path.parent().unwrap_or(file_path);
-        let (child_modules, child_errors) = process_submodule(
+        let child_modules = process_submodule(
             &child_path,
             &sub.name,
             items,
@@ -245,11 +248,11 @@ fn process_module_info(
             file_path,
             visited,
         );
-        errors.extend(child_errors);
-        modules.extend(child_modules);
+        errors.extend(child_modules.1);
+        modules.extend(child_modules.0);
     }
 
-    (modules, errors.clone())
+    modules
 }
 
 // ── Tests ───────────────────────────────────────────────────────────────
diff --git a/src/schema.rs b/src/schema.rs
index d6c1fe8..11edb66 100644
--- a/src/schema.rs
+++ b/src/schema.rs
@@ -1,6 +1,52 @@
 use std::collections::BTreeMap;
 use std::path::PathBuf;
 
+// ── Path newtypes for flat indexes ──────────────────────────────────────
+
+#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
+#[serde(transparent)]
+pub struct CanonicalPath(pub String);
+
+impl std::fmt::Display for CanonicalPath {
+    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
+        self.0.fmt(f)
+    }
+}
+
+impl AsRef<str> for CanonicalPath {
+    fn as_ref(&self) -> &str {
+        &self.0
+    }
+}
+
+impl From<String> for CanonicalPath {
+    fn from(s: String) -> Self {
+        Self(s)
+    }
+}
+
+#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
+#[serde(transparent)]
+pub struct WorkspaceRelativePath(pub String);
+
+impl std::fmt::Display for WorkspaceRelativePath {
+    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
+        self.0.fmt(f)
+    }
+}
+
+impl AsRef<str> for WorkspaceRelativePath {
+    fn as_ref(&self) -> &str {
+        &self.0
+    }
+}
+
+impl From<String> for WorkspaceRelativePath {
+    fn from(s: String) -> Self {
+        Self(s)
+    }
+}
+
 // ── Error type ──────────────────────────────────────────────────────────
 
 #[derive(Debug, thiserror::Error)]
@@ -47,6 +93,10 @@ pub struct Config {
 
     /// If Some, write JSON to this file instead of stdout.
     pub output_path: Option<PathBuf>,
+
+    /// When true, run validation checks (orphan files, dead re-exports).
+    #[builder(default)]
+    pub validate: bool,
 }
 
 // ── Crate type ──────────────────────────────────────────────────────────
@@ -69,6 +119,15 @@ pub struct WorkspaceMap {
     pub crates: Vec<CrateInfo>,
     pub cross_references: CrossReferences,
 
+    #[builder(default)]
+    pub symbols: BTreeMap<CanonicalPath, SymbolEntry>,
+
+    #[builder(default)]
+    pub name_index: BTreeMap<String, Vec<CanonicalPath>>,
+
+    #[builder(default)]
+    pub files: BTreeMap<WorkspaceRelativePath, FileEntry>,
+
     #[builder(default)]
     #[serde(skip_serializing_if = "Vec::is_empty")]
     pub errors: Vec<ErrorEntry>,
@@ -181,7 +240,7 @@ pub struct PublicItem {
     pub impls: Vec<ImplInfo>,
 }
 
-#[derive(Debug, Clone, serde::Serialize)]
+#[derive(Debug, Clone, PartialEq, serde::Serialize)]
 #[serde(rename_all = "camelCase")]
 pub enum ItemKind {
     Struct,
@@ -281,6 +340,23 @@ pub struct TypeRef {
     pub exported_by: Vec<String>,
 }
 
+// ── Diagnostic kind ─────────────────────────────────────────────────────
+
+#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
+#[serde(rename_all = "snake_case")]
+pub enum DiagnosticKind {
+    OrphanedModule,
+    TomlParseError,
+    MissingCrateRoots,
+    ModuleTreeError,
+    SynParseError,
+    MissingWorkspaceSection,
+    GlobPatternError,
+    MemberNotFound,
+    OrphanFile,
+    DeadReExport,
+}
+
 // ── Error severity ─────────────────────────────────────────────────────
 
 #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
@@ -331,6 +407,27 @@ pub struct SubmoduleDecl {
     pub is_test: bool,
 }
 
+// ── Flat index entry types ──────────────────────────────────────────────
+
+#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
+#[serde(rename_all = "camelCase")]
+pub struct SymbolEntry {
+    pub crate_name: String,
+    pub module: String,
+    pub file: String,
+    pub line: usize,
+    pub kind: ItemKind,
+}
+
+#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
+#[serde(rename_all = "camelCase")]
+pub struct FileEntry {
+    pub module_path: String,
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pub parent_module_file: Option<String>,
+    pub is_crate_root: bool,
+}
+
 // ── Error reporting ─────────────────────────────────────────────────────
 
 #[derive(Debug, Clone, serde::Serialize, bon::Builder)]
@@ -341,7 +438,7 @@ pub struct ErrorEntry {
     pub line: usize,
     pub message: String,
     pub severity: ErrorSeverity,
-    pub kind: String,
+    pub kind: DiagnosticKind,
     #[serde(skip_serializing_if = "Option::is_none")]
     pub context: Option<ErrorContext>,
     #[serde(skip_serializing_if = "Option::is_none")]
diff --git a/src/validate.rs b/src/validate.rs
new file mode 100644
index 0000000..353b122
--- /dev/null
+++ b/src/validate.rs
@@ -0,0 +1,242 @@
+use crate::schema::{
+    CanonicalPath, DiagnosticKind, ErrorEntry, ErrorSeverity, CrateInfo,
+};
+use std::collections::HashSet;
+use std::path::Path;
+use walkdir::WalkDir;
+
+/// Run validation checks on the crate set.
+///
+/// Checks performed:
+/// - **Orphan files**: `.rs` files on disk not declared in any module tree
+/// - **Dead re-exports**: `pub use` to symbols not found in the symbol index
+///
+/// Returns a list of `ErrorEntry` findings (severity: Warning).
+#[must_use]
+pub fn validate(
+    crates: &[CrateInfo],
+    symbols: &std::collections::BTreeMap<CanonicalPath, crate::schema::SymbolEntry>,
+    workspace_root: &Path,
+) -> Vec<ErrorEntry> {
+    let mut findings: Vec<ErrorEntry> = Vec::new();
+    let crate_names: std::collections::HashSet<&str> =
+        crates.iter().map(|c| c.name.as_str()).collect();
+
+    for crate_info in crates {
+        findings.extend(check_orphan_files(crate_info, workspace_root));
+        findings.extend(check_dead_reexports(crate_info, symbols, &crate_names));
+    }
+
+    findings
+}
+
+/// Find `.rs` files on disk that are not declared in the module tree.
+fn check_orphan_files(
+    crate_info: &CrateInfo,
+    workspace_root: &Path,
+) -> Vec<ErrorEntry> {
+    // crate_info.root is a relativized path to the crate root file (e.g., "src/lib.rs").
+    // We need the src/ directory, which is the parent of the root file.
+    let crate_root_path = workspace_root.join(&crate_info.root);
+    let src_dir = crate_root_path
+        .parent()
+        .unwrap_or(workspace_root)
+        .to_path_buf();
+    let crate_name = &crate_info.name;
+
+    // Collect all file paths declared in the module tree.
+    let declared_files: HashSet<String> = crate_info
+        .modules
+        .iter()
+        .filter(|m| m.file != "<unresolved>")
+        .map(|m| m.file.clone())
+        .collect();
+
+    // Walk the src/ directory recursively using walkdir.
+    let mut findings = Vec::new();
+
+    for entry in WalkDir::new(&src_dir).into_iter().filter_entry(|e| {
+        // Always yield the root; keep files (filtered later); skip hidden, bin/, tests/ dirs.
+        e.depth() == 0 || !e.file_type().is_dir() || {
+            let name = e.file_name().to_string_lossy();
+            !name.starts_with('.') && name != "bin" && name != "tests"
+        }
+    }) {
+        let Ok(entry) = entry else { continue };
+        let path = entry.path();
+
+        if !path.is_file() {
+            continue;
+        }
+        if path.extension().is_none_or(|e| e != "rs") {
+            continue;
+        }
+        let file_name = path.file_name().map_or_else(String::new, |f| f.to_string_lossy().into_owned());
+        if file_name == "lib.rs" || file_name == "main.rs" || file_name == "mod.rs" {
+            continue; // Crate roots and mod.rs — declared implicitly.
+        }
+
+        let ws_rel = path.strip_prefix(workspace_root)
+            .unwrap_or(path)
+            .to_string_lossy()
+            .to_string();
+
+        if declared_files.contains(&ws_rel) {
+            continue;
+        }
+
+        let parent_file = determine_parent_file(&ws_rel, crate_name, crate_info);
+        let stem = path.file_stem()
+            .map_or_else(String::new, |s| s.to_string_lossy().into_owned());
+        findings.push(ErrorEntry::builder()
+            .file(ws_rel.clone())
+            .message(format!(
+                "orphan file: '{file_name}' is not declared in the module tree. Add 'pub mod {stem};' to {parent_file}."
+            ))
+            .severity(ErrorSeverity::Warning)
+            .kind(DiagnosticKind::OrphanFile)
+            .build());
+    }
+
+    findings
+}
+
+/// Determine the parent file for an orphan file's fix hint.
+fn determine_parent_file(
+    orphan_file: &str,
+    crate_name: &str,
+    _crate_info: &CrateInfo,
+) -> String {
+    // Strip src/ prefix, resolve parent directory to a module file.
+    let stripped = orphan_file.strip_prefix("src/").unwrap_or(orphan_file);
+    let parent_dir = stripped.rsplit_once('/').map(|(dir, _)| dir);
+
+    match parent_dir {
+        Some("") | None => {
+            // File is directly in src/ — parent is lib.rs or main.rs.
+            format!("{crate_name}/src/lib.rs")
+        }
+        Some(dir) => {
+            // File is in a subdirectory — parent module file is dir/lib.rs or dir/mod.rs.
+            format!("{crate_name}/src/{dir}/mod.rs")
+        }
+    }
+}
+
+/// Find `pub use` re-exports that reference symbols not in the index.
+fn check_dead_reexports(
+    crate_info: &CrateInfo,
+    symbols: &std::collections::BTreeMap<CanonicalPath, crate::schema::SymbolEntry>,
+    crate_names: &HashSet<&str>,
+) -> Vec<ErrorEntry> {
+    let mut findings = Vec::new();
+    let my_name = crate_info.name.as_str();
+
+    for module in &crate_info.modules {
+        for re_export in &module.re_exports {
+            let path = &re_export.import_path;
+
+            // Skip glob re-exports.
+            if path.ends_with("::*") {
+                continue;
+            }
+
+            // Parse the import path. Resolve 'crate::' prefix.
+            let resolved = resolve_import_path(path, my_name, &module.path);
+
+            // If first segment is a workspace member crate name, skip (external re-export).
+            let first_seg = resolved.split("::").next().unwrap_or(&resolved);
+            if crate_names.contains(first_seg) && first_seg != my_name {
+                continue;
+            }
+
+            // Build the canonical path from the resolved import path.
+            let canonical = CanonicalPath::from(resolved.clone());
+
+            // Look up in the symbols map.
+            if !symbols.contains_key(&canonical) {
+                findings.push(ErrorEntry::builder()
+                    .file(module.file.clone())
+                    .line(re_export.line)
+                    .message(format!(
+                        "dead re-export: '{}' resolves to '{}' which is not found in the symbol index. Remove or fix the 'pub use' statement.",
+                        re_export.export_path,
+                        re_export.import_path
+                    ))
+                    .severity(ErrorSeverity::Warning)
+                    .kind(DiagnosticKind::DeadReExport)
+                    .build());
+            }
+        }
+    }
+
+    findings
+}
+
+/// Resolve an import path by expanding `crate::`, `self::`, `super::` prefixes.
+fn resolve_import_path(path: &str, crate_name: &str, module_path: &str) -> String {
+    if let Some(rest) = path.strip_prefix("crate::") {
+        format!("{crate_name}::{rest}")
+    } else if let Some(rest) = path.strip_prefix("self::") {
+        format!("{module_path}::{rest}")
+    } else if let Some(rest) = path.strip_prefix("super::") {
+        let parent = module_path.rsplit_once("::")
+            .map_or("", |(p, _)| p);
+        if parent.is_empty() {
+            format!("{crate_name}::{rest}")
+        } else {
+            format!("{parent}::{rest}")
+        }
+    } else {
+        // Bare path — assume it's relative to the crate root.
+        format!("{crate_name}::{path}")
+    }
+}
+
+// ── Tests ───────────────────────────────────────────────────────────────
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+
+    #[test]
+    fn resolve_import_path_crate_prefix() {
+        assert_eq!(
+            resolve_import_path("crate::foo::bar", "mycrate", "mycrate::sub"),
+            "mycrate::foo::bar"
+        );
+    }
+
+    #[test]
+    fn resolve_import_path_self_prefix() {
+        assert_eq!(
+            resolve_import_path("self::inner", "mycrate", "mycrate::sub"),
+            "mycrate::sub::inner"
+        );
+    }
+
+    #[test]
+    fn resolve_import_path_super_prefix() {
+        assert_eq!(
+            resolve_import_path("super::other", "mycrate", "mycrate::sub::deep"),
+            "mycrate::sub::other"
+        );
+    }
+
+    #[test]
+    fn resolve_import_path_super_root() {
+        // At crate root, super:: wraps to crate root.
+        assert_eq!(
+            resolve_import_path("super::helper", "mycrate", "mycrate"),
+            "mycrate::helper"
+        );
+    }
+
+    #[test]
+    fn resolve_import_path_bare() {
+        assert_eq!(
+            resolve_import_path("foo::Bar", "mycrate", "mycrate::sub"),
+            "mycrate::foo::Bar"
+        );
+    }
+}
diff --git a/tests/fixtures/bad-dead-reexport/Cargo.toml b/tests/fixtures/bad-dead-reexport/Cargo.toml
new file mode 100644
index 0000000..d325f03
--- /dev/null
+++ b/tests/fixtures/bad-dead-reexport/Cargo.toml
@@ -0,0 +1,7 @@
+[workspace]
+members = ["."]
+
+[package]
+name = "bad-dead-reexport"
+version = "0.1.0"
+edition = "2021"
diff --git a/tests/fixtures/bad-dead-reexport/src/lib.rs b/tests/fixtures/bad-dead-reexport/src/lib.rs
new file mode 100644
index 0000000..bd42bbf
--- /dev/null
+++ b/tests/fixtures/bad-dead-reexport/src/lib.rs
@@ -0,0 +1 @@
+pub use crate::DoesNotExist;
diff --git a/tests/fixtures/bad-orphan/Cargo.toml b/tests/fixtures/bad-orphan/Cargo.toml
new file mode 100644
index 0000000..eb0b155
--- /dev/null
+++ b/tests/fixtures/bad-orphan/Cargo.toml
@@ -0,0 +1,7 @@
+[workspace]
+members = ["."]
+
+[package]
+name = "bad-orphan"
+version = "0.1.0"
+edition = "2021"
diff --git a/tests/fixtures/bad-orphan/src/forgotten.rs b/tests/fixtures/bad-orphan/src/forgotten.rs
new file mode 100644
index 0000000..b74e137
--- /dev/null
+++ b/tests/fixtures/bad-orphan/src/forgotten.rs
@@ -0,0 +1 @@
+pub fn not_declared() {}
diff --git a/tests/fixtures/bad-orphan/src/lib.rs b/tests/fixtures/bad-orphan/src/lib.rs
new file mode 100644
index 0000000..a15afbc
--- /dev/null
+++ b/tests/fixtures/bad-orphan/src/lib.rs
@@ -0,0 +1 @@
+// root module — intentionally does NOT include 'mod forgotten;'
diff --git a/tests/integration_test.rs b/tests/integration_test.rs
index f1b8ae4..1e65521 100644
--- a/tests/integration_test.rs
+++ b/tests/integration_test.rs
@@ -12,11 +12,14 @@ fn extract_array<'a>(val: &'a serde_json::Value, key: &str) -> Vec<&'a serde_jso
         .unwrap_or_default()
 }
 
+// ── Existing tests (rewritten for index subcommand) ─────────────────────
+
 #[test]
 fn test_sample_workspace_output() {
     let fixture = std::path::Path::new("tests/fixtures/sample-workspace");
 
     let output = Command::new(&binary_path())
+        .arg("index")
         .arg(fixture)
         .output()
         .expect("failed to execute binary");
@@ -33,7 +36,7 @@ fn test_sample_workspace_output() {
         serde_json::from_str(&stdout).expect("output is not valid JSON");
 
     // Top-level structure.
-    assert_eq!(json["workspace"]["root"], ".");
+    assert!(!json["workspace"]["root"].as_str().unwrap().is_empty());
     assert!(!json["workspace"]["workspaceName"].as_str().unwrap().is_empty());
     assert!(json["crates"].is_array(), "crates must be an array");
 
@@ -77,11 +80,12 @@ fn test_sample_workspace_output() {
     };
     assert!(engine_imports_core, "engine should import from core");
 
-    // Cross-references should link Task to both crates.
+    // Cross-references: keys are now canonical paths (module.path::item.name).
     let cross_refs = &json["crossReferences"]["types"];
+    // Task is exported from the "core" module, so key is "core::Task".
     let task_ref = cross_refs
-        .get("Task")
-        .expect("Task should appear in crossReferences.types");
+        .get("core::Task")
+        .expect("core::Task should appear in crossReferences.types (canonical path)");
     assert!(
         task_ref["exportedBy"]
             .as_array()
@@ -96,12 +100,14 @@ fn test_deterministic_output() {
     let fixture = std::path::Path::new("tests/fixtures/sample-workspace");
 
     let output1 = Command::new(&binary_path())
+        .arg("index")
         .arg(fixture)
         .output()
         .expect("failed to execute binary (run 1)");
     assert!(output1.status.success());
 
     let output2 = Command::new(&binary_path())
+        .arg("index")
         .arg(fixture)
         .output()
         .expect("failed to execute binary (run 2)");
@@ -117,6 +123,7 @@ fn test_deterministic_output() {
 fn test_missing_path_exits_nonzero() {
     let bin = binary_path();
     let output = Command::new(&bin)
+        .arg("index")
         .arg("/tmp/nonexistent-path-12345")
         .output()
         .expect("failed to execute binary");
@@ -127,8 +134,9 @@ fn test_missing_path_exits_nonzero() {
     );
 }
 
-fn run_binary(path: &str) -> std::process::Output {
+fn run_index(path: &str) -> std::process::Output {
     Command::new(&binary_path())
+        .arg("index")
         .arg(path)
         .output()
         .expect("failed to execute binary")
@@ -172,7 +180,7 @@ members = ["good_crate", "bad_crate"]
     // Bad crate with invalid Rust syntax
     setup_crate(&root.join("bad_crate"), "pub struct { invalid rust syntax");
 
-    let output = run_binary(root.to_str().unwrap());
+    let output = run_index(root.to_str().unwrap());
     assert!(output.status.success());
 
     let json = parse_output(&output);
@@ -201,9 +209,13 @@ version = "0.1.0"
 edition = "2021"
 "#);
 
-    let output = run_binary(root.to_str().unwrap());
+    let output = Command::new(&binary_path())
+        .arg("index")
+        .arg(root.to_str().unwrap())
+        .output();
 
     // Should exit non-zero because workspace is missing
+    let output = output.expect("failed to execute binary");
     assert!(
         !output.status.success(),
         "should exit non-zero for missing workspace section"
@@ -224,7 +236,7 @@ members = ["crates/*"]
         setup_crate(&root.join("crates").join(name), format!("pub struct {name} {{}}").as_str());
     }
 
-    let output = run_binary(root.to_str().unwrap());
+    let output = run_index(root.to_str().unwrap());
     assert!(output.status.success());
 
     let json = parse_output(&output);
@@ -254,7 +266,7 @@ exclude = ["b"]
     setup_crate(&root.join("b"), "pub struct B {}");
     setup_crate(&root.join("c"), "pub struct C {}");
 
-    let output = run_binary(root.to_str().unwrap());
+    let output = run_index(root.to_str().unwrap());
     assert!(output.status.success());
 
     let json = parse_output(&output);
@@ -297,7 +309,7 @@ edition = "2021"
     // bar/baz.rs with a struct
     std::fs::write(bar.join("baz.rs"), "pub struct Deep {}").unwrap();
 
-    let output = run_binary(root.to_str().unwrap());
+    let output = run_index(root.to_str().unwrap());
     assert!(output.status.success());
 
     let json = parse_output(&output);
@@ -342,7 +354,7 @@ mod inner {
 pub use inner::Secret;
 ").unwrap();
 
-    let output = run_binary(root.to_str().unwrap());
+    let output = run_index(root.to_str().unwrap());
     assert!(output.status.success());
 
     let json = parse_output(&output);
@@ -368,6 +380,7 @@ fn test_output_via_flag() {
 
     // Run with -o flag
     let output1 = Command::new(&binary_path())
+        .arg("index")
         .arg(fixture)
         .arg("-o")
         .arg(output_path.clone())
@@ -377,6 +390,7 @@ fn test_output_via_flag() {
 
     // Run without -o, capture stdout
     let output2 = Command::new(&binary_path())
+        .arg("index")
         .arg(fixture)
         .output()
         .expect("failed to execute binary");
@@ -391,3 +405,203 @@ fn test_output_via_flag() {
         "file output should match stdout"
     );
 }
+
+// ── New validation tests ────────────────────────────────────────────────
+
+#[test]
+fn test_validate_orphan_file_exits_2() {
+    let fixture = std::path::Path::new("tests/fixtures/bad-orphan");
+
+    let output = Command::new(&binary_path())
+        .arg("index")
+        .arg("--validate")
+        .arg(fixture)
+        .output()
+        .expect("failed to execute binary");
+
+    assert_eq!(
+        output.status.code().unwrap(),
+        2,
+        "validation should exit 2 for orphan files"
+    );
+
+    let stdout = String::from_utf8_lossy(&output.stdout);
+    let json: serde_json::Value =
+        serde_json::from_str(&stdout).expect("output is not valid JSON");
+
+    let errors = extract_array(&json, "errors");
+    assert!(!errors.is_empty(), "should have validation errors");
+
+    let orphan_error = errors.iter().find(|e| e["kind"].as_str().unwrap() == "orphan_file")
+        .expect("should have orphan_file error");
+
+    assert_eq!(orphan_error["severity"].as_str().unwrap(), "warning");
+    let message = orphan_error["message"].as_str().unwrap();
+    assert!(message.contains("forgotten"), "message should mention the orphan file");
+    assert!(message.contains("pub mod"), "message should suggest fix hint");
+}
+
+#[test]
+fn test_validate_dead_reexport_exits_2() {
+    let fixture = std::path::Path::new("tests/fixtures/bad-dead-reexport");
+
+    let output = Command::new(&binary_path())
+        .arg("index")
+        .arg("--validate")
+        .arg(fixture)
+        .output()
+        .expect("failed to execute binary");
+
+    assert_eq!(
+        output.status.code().unwrap(),
+        2,
+        "validation should exit 2 for dead re-exports"
+    );
+
+    let stdout = String::from_utf8_lossy(&output.stdout);
+    let json: serde_json::Value =
+        serde_json::from_str(&stdout).expect("output is not valid JSON");
+
+    let errors = extract_array(&json, "errors");
+    let dead_reexport = errors.iter().find(|e| e["kind"].as_str().unwrap() == "dead_re_export")
+        .expect("should have dead_re_export error");
+
+    assert_eq!(dead_reexport["severity"].as_str().unwrap(), "warning");
+}
+
+#[test]
+fn test_index_no_validate_exits_0() {
+    let fixture = std::path::Path::new("tests/fixtures/bad-orphan");
+
+    let output = Command::new(&binary_path())
+        .arg("index")
+        .arg(fixture)
+        .output()
+        .expect("failed to execute binary");
+
+    assert_eq!(
+        output.status.code().unwrap(),
+        0,
+        "without --validate, should exit 0"
+    );
+
+    let stdout = String::from_utf8_lossy(&output.stdout);
+    let json: serde_json::Value =
+        serde_json::from_str(&stdout).expect("output is not valid JSON");
+
+    let errors = extract_array(&json, "errors");
+    let has_orphan = errors.iter().any(|e| {
+        e["kind"].as_str().unwrap() == "orphan_file"
+    });
+    assert!(!has_orphan, "without --validate, should not have OrphanFile/DeadReExport errors");
+}
+
+// ── New lookup tests ────────────────────────────────────────────────────
+
+#[test]
+fn test_lookup_symbol_found() {
+    let fixture = std::path::Path::new("tests/fixtures/sample-workspace");
+
+    let output = Command::new(&binary_path())
+        .arg("lookup")
+        .arg(fixture)
+        .arg("--symbol")
+        .arg("Task")
+        .output()
+        .expect("failed to execute binary");
+
+    assert!(output.status.success(), "should find Task symbol");
+
+    let stdout = String::from_utf8_lossy(&output.stdout);
+    let json: serde_json::Value =
+        serde_json::from_str(&stdout).expect("output is not valid JSON");
+
+    assert_eq!(json["status"].as_str().unwrap(), "found");
+    assert!(json["crateName"].as_str().unwrap().is_empty() == false);
+    assert_eq!(json["kind"].as_str().unwrap(), "struct");
+}
+
+#[test]
+fn test_lookup_symbol_not_found() {
+    let fixture = std::path::Path::new("tests/fixtures/sample-workspace");
+
+    let output = Command::new(&binary_path())
+        .arg("lookup")
+        .arg(fixture)
+        .arg("--symbol")
+        .arg("DoesNotExist")
+        .output()
+        .expect("failed to execute binary");
+
+    assert_eq!(
+        output.status.code().unwrap(),
+        1,
+        "should exit 1 for not found symbol"
+    );
+
+    let stdout = String::from_utf8_lossy(&output.stdout);
+    let json: serde_json::Value =
+        serde_json::from_str(&stdout).expect("output is not valid JSON");
+
+    assert_eq!(json["status"].as_str().unwrap(), "not_found");
+}
+
+#[test]
+fn test_lookup_file() {
+    let fixture = std::path::Path::new("tests/fixtures/sample-workspace");
+
+    let output = Command::new(&binary_path())
+        .arg("lookup")
+        .arg(fixture)
+        .arg("--file")
+        .arg("core/src/lib.rs")
+        .output()
+        .expect("failed to execute binary");
+
+    assert!(output.status.success(), "should find core/src/lib.rs");
+
+    let stdout = String::from_utf8_lossy(&output.stdout);
+    let json: serde_json::Value =
+        serde_json::from_str(&stdout).expect("output is not valid JSON");
+
+    assert!(json.get("fileEntry").is_some(), "should have fileEntry");
+    assert!(json.get("primaryModule").is_some(), "should have primaryModule");
+}
+
+// ── Serde regression test ───────────────────────────────────────────────
+
+#[test]
+fn test_orphaned_module_serde_regression() {
+    // Verify DiagnosticKind::OrphanedModule serializes as "orphaned_module".
+    let tmp = tempfile::tempdir().unwrap();
+    let root = tmp.path();
+
+    write_cargo_toml(root, r#"
+[workspace]
+members = ["."]
+
+[package]
+name = "orphan-mod-test"
+version = "0.1.0"
+edition = "2021"
+"#);
+
+    let src = root.join("src");
+    std::fs::create_dir_all(&src).unwrap();
+
+    // lib.rs with an undeclared module
+    std::fs::write(src.join("lib.rs"), "mod nonexistent;").unwrap();
+
+    let output = run_index(root.to_str().unwrap());
+    assert!(output.status.success());
+
+    let stdout = String::from_utf8_lossy(&output.stdout);
+    let json: serde_json::Value =
+        serde_json::from_str(&stdout).expect("output is not valid JSON");
+
+    let errors = extract_array(&json, "errors");
+    let orphaned = errors.iter().find(|e| e["kind"].as_str().unwrap() == "orphaned_module")
+        .expect("should have orphaned_module error (snake_case string)");
+
+    assert_eq!(orphaned["severity"].as_str().unwrap(), "warning");
+}

## File: CLAUDE.md
- Refer to [architecture note](./notes/architecture-current.md) whenever you
  want to explore the codebase for understanding.
## File: Cargo.lock
# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
version = 4

[[package]]
name = "anstream"
version = "1.0.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "824a212faf96e9acacdbd09febd34438f8f711fb84e09a8916013cd7815ca28d"
dependencies = [
 "anstyle",
 "anstyle-parse",
 "anstyle-query",
 "anstyle-wincon",
 "colorchoice",
 "is_terminal_polyfill",
 "utf8parse",
]

[[package]]
name = "anstyle"
version = "1.0.14"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "940b3a0ca603d1eade50a4846a2afffd5ef57a9feac2c0e2ec2e14f9ead76000"

[[package]]
name = "anstyle-parse"
version = "1.0.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "52ce7f38b242319f7cabaa6813055467063ecdc9d355bbb4ce0c68908cd8130e"
dependencies = [
 "utf8parse",
]

[[package]]
name = "anstyle-query"
version = "1.1.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "40c48f72fd53cd289104fc64099abca73db4166ad86ea0b4341abe65af83dadc"
dependencies = [
 "windows-sys",
]

[[package]]
name = "anstyle-wincon"
version = "3.0.11"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "291e6a250ff86cd4a820112fb8898808a366d8f9f58ce16d1f538353ad55747d"
dependencies = [
 "anstyle",
 "once_cell_polyfill",
 "windows-sys",
]

[[package]]
name = "anyhow"
version = "1.0.102"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7f202df86484c868dbad7eaa557ef785d5c66295e41b460ef922eca0723b842c"

[[package]]
name = "bitflags"
version = "2.11.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c4512299f36f043ab09a583e57bceb5a5aab7a73db1805848e8fef3c9e8c78b3"

[[package]]
name = "bon"
version = "3.9.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f47dbe92550676ee653353c310dfb9cf6ba17ee70396e1f7cf0a2020ad49b2fe"
dependencies = [
 "bon-macros",
 "rustversion",
]

[[package]]
name = "bon-macros"
version = "3.9.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "519bd3116aeeb42d5372c29d982d16d0170d3d4a5ed85fc7dd91642ffff3c67c"
dependencies = [
 "darling",
 "ident_case",
 "prettyplease",
 "proc-macro2",
 "quote",
 "rustversion",
 "syn",
]

[[package]]
name = "cfg-if"
version = "1.0.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9330f8b2ff13f34540b44e946ef35111825727b38d33286ef986142615121801"

[[package]]
name = "clap"
version = "4.6.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1ddb117e43bbf7dacf0a4190fef4d345b9bad68dfc649cb349e7d17d28428e51"
dependencies = [
 "clap_builder",
 "clap_derive",
]

[[package]]
name = "clap_builder"
version = "4.6.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "714a53001bf66416adb0e2ef5ac857140e7dc3a0c48fb28b2f10762fc4b5069f"
dependencies = [
 "anstream",
 "anstyle",
 "clap_lex",
 "strsim",
]

[[package]]
name = "clap_derive"
version = "4.6.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f2ce8604710f6733aa641a2b3731eaa1e8b3d9973d5e3565da11800813f997a9"
dependencies = [
 "heck",
 "proc-macro2",
 "quote",
 "syn",
]

[[package]]
name = "clap_lex"
version = "1.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c8d4a3bb8b1e0c1050499d1815f5ab16d04f0959b233085fb31653fbfc9d98f9"

[[package]]
name = "colorchoice"
version = "1.0.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1d07550c9036bf2ae0c684c4297d503f838287c83c53686d05370d0e139ae570"

[[package]]
name = "crossbeam-deque"
version = "0.8.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9dd111b7b7f7d55b72c0a6ae361660ee5853c9af73f70c3c2ef6858b950e2e51"
dependencies = [
 "crossbeam-epoch",
 "crossbeam-utils",
]

[[package]]
name = "crossbeam-epoch"
version = "0.9.18"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5b82ac4a3c2ca9c3460964f020e1402edd5753411d7737aa39c3714ad1b5420e"
dependencies = [
 "crossbeam-utils",
]

[[package]]
name = "crossbeam-utils"
version = "0.8.21"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d0a5c400df2834b80a4c3327b3aad3a4c4cd4de0629063962b03235697506a28"

[[package]]
name = "darling"
version = "0.23.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "25ae13da2f202d56bd7f91c25fba009e7717a1e4a1cc98a76d844b65ae912e9d"
dependencies = [
 "darling_core",
 "darling_macro",
]

[[package]]
name = "darling_core"
version = "0.23.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9865a50f7c335f53564bb694ef660825eb8610e0a53d3e11bf1b0d3df31e03b0"
dependencies = [
 "ident_case",
 "proc-macro2",
 "quote",
 "strsim",
 "syn",
]

[[package]]
name = "darling_macro"
version = "0.23.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ac3984ec7bd6cfa798e62b4a642426a5be0e68f9401cfc2a01e3fa9ea2fcdb8d"
dependencies = [
 "darling_core",
 "quote",
 "syn",
]

[[package]]
name = "either"
version = "1.15.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "48c757948c5ede0e46177b7add2e67155f70e33c07fea8284df6576da70b3719"

[[package]]
name = "equivalent"
version = "1.0.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "877a4ace8713b0bcf2a4e7eec82529c029f1d0619886d18145fea96c3ffe5c0f"

[[package]]
name = "errno"
version = "0.3.14"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "39cab71617ae0d63f51a36d69f866391735b51691dbda63cf6f96d042b63efeb"
dependencies = [
 "libc",
 "windows-sys",
]

[[package]]
name = "fastrand"
version = "2.4.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9f1f227452a390804cdb637b74a86990f2a7d7ba4b7d5693aac9b4dd6defd8d6"

[[package]]
name = "foldhash"
version = "0.1.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d9c4f5dac5e15c24eb999c26181a6ca40b39fe946cbe4c263c7209467bc83af2"

[[package]]
name = "getrandom"
version = "0.4.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0de51e6874e94e7bf76d726fc5d13ba782deca734ff60d5bb2fb2607c7406555"
dependencies = [
 "cfg-if",
 "libc",
 "r-efi",
 "wasip2",
 "wasip3",
]

[[package]]
name = "glob"
version = "0.3.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0cc23270f6e1808e30a928bdc84dea0b9b4136a8bc82338574f23baf47bbd280"

[[package]]
name = "hashbrown"
version = "0.15.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9229cfe53dfd69f0609a49f65461bd93001ea1ef889cd5529dd176593f5338a1"
dependencies = [
 "foldhash",
]

[[package]]
name = "hashbrown"
version = "0.17.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4f467dd6dccf739c208452f8014c75c18bb8301b050ad1cfb27153803edb0f51"

[[package]]
name = "heck"
version = "0.5.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "2304e00983f87ffb38b55b444b5e3b60a884b5d30c0fca7d82fe33449bbe55ea"

[[package]]
name = "id-arena"
version = "2.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3d3067d79b975e8844ca9eb072e16b31c3c1c36928edf9c6789548c524d0d954"

[[package]]
name = "ident_case"
version = "1.0.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b9e0384b61958566e926dc50660321d12159025e767c18e043daf26b70104c39"

[[package]]
name = "indexmap"
version = "2.14.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d466e9454f08e4a911e14806c24e16fba1b4c121d1ea474396f396069cf949d9"
dependencies = [
 "equivalent",
 "hashbrown 0.17.0",
 "serde",
 "serde_core",
]

[[package]]
name = "is_terminal_polyfill"
version = "1.70.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a6cb138bb79a146c1bd460005623e142ef0181e3d0219cb493e02f7d08a35695"

[[package]]
name = "itoa"
version = "1.0.18"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8f42a60cbdf9a97f5d2305f08a87dc4e09308d1276d28c869c684d7777685682"

[[package]]
name = "leb128fmt"
version = "0.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "09edd9e8b54e49e587e4f6295a7d29c3ea94d469cb40ab8ca70b288248a81db2"

[[package]]
name = "libc"
version = "0.2.186"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "68ab91017fe16c622486840e4c83c9a37afeff978bd239b5293d61ece587de66"

[[package]]
name = "linux-raw-sys"
version = "0.12.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "32a66949e030da00e8c7d4434b251670a91556f4144941d37452769c25d58a53"

[[package]]
name = "log"
version = "0.4.29"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5e5032e24019045c762d3c0f28f5b6b8bbf38563a65908389bf7978758920897"

[[package]]
name = "memchr"
version = "2.8.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f8ca58f447f06ed17d5fc4043ce1b10dd205e060fb3ce5b979b8ed8e59ff3f79"

[[package]]
name = "once_cell"
version = "1.21.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9f7c3e4beb33f85d45ae3e3a1792185706c8e16d043238c593331cc7cd313b50"

[[package]]
name = "once_cell_polyfill"
version = "1.70.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "384b8ab6d37215f3c5301a95a4accb5d64aa607f1fcb26a11b5303878451b4fe"

[[package]]
name = "prettyplease"
version = "0.2.37"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "479ca8adacdd7ce8f1fb39ce9ecccbfe93a3f1344b3d0d97f20bc0196208f62b"
dependencies = [
 "proc-macro2",
 "syn",
]

[[package]]
name = "proc-macro2"
version = "1.0.106"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8fd00f0bb2e90d81d1044c2b32617f68fcb9fa3bb7640c23e9c748e53fb30934"
dependencies = [
 "unicode-ident",
]

[[package]]
name = "quote"
version = "1.0.45"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "41f2619966050689382d2b44f664f4bc593e129785a36d6ee376ddf37259b924"
dependencies = [
 "proc-macro2",
]

[[package]]
name = "r-efi"
version = "6.0.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f8dcc9c7d52a811697d2151c701e0d08956f92b0e24136cf4cf27b57a6a0d9bf"

[[package]]
name = "rayon"
version = "1.12.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "fb39b166781f92d482534ef4b4b1b2568f42613b53e5b6c160e24cfbfa30926d"
dependencies = [
 "either",
 "rayon-core",
]

[[package]]
name = "rayon-core"
version = "1.13.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "22e18b0f0062d30d4230b2e85ff77fdfe4326feb054b9783a3460d8435c8ab91"
dependencies = [
 "crossbeam-deque",
 "crossbeam-utils",
]

[[package]]
name = "rust-workspace-map"
version = "0.1.0"
dependencies = [
 "anyhow",
 "bon",
 "clap",
 "glob",
 "proc-macro2",
 "rayon",
 "serde",
 "serde_json",
 "syn",
 "tempfile",
 "thiserror",
 "toml",
 "walkdir",
]

[[package]]
name = "rustix"
version = "1.1.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b6fe4565b9518b83ef4f91bb47ce29620ca828bd32cb7e408f0062e9930ba190"
dependencies = [
 "bitflags",
 "errno",
 "libc",
 "linux-raw-sys",
 "windows-sys",
]

[[package]]
name = "rustversion"
version = "1.0.22"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b39cdef0fa800fc44525c84ccb54a029961a8215f9619753635a9c0d2538d46d"

[[package]]
name = "same-file"
version = "1.0.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "93fc1dc3aaa9bfed95e02e6eadabb4baf7e3078b0bd1b4d7b6b0b68378900502"
dependencies = [
 "winapi-util",
]

[[package]]
name = "semver"
version = "1.0.28"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8a7852d02fc848982e0c167ef163aaff9cd91dc640ba85e263cb1ce46fae51cd"

[[package]]
name = "serde"
version = "1.0.228"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9a8e94ea7f378bd32cbbd37198a4a91436180c5bb472411e48b5ec2e2124ae9e"
dependencies = [
 "serde_core",
 "serde_derive",
]

[[package]]
name = "serde_core"
version = "1.0.228"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "41d385c7d4ca58e59fc732af25c3983b67ac852c1a25000afe1175de458b67ad"
dependencies = [
 "serde_derive",
]

[[package]]
name = "serde_derive"
version = "1.0.228"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d540f220d3187173da220f885ab66608367b6574e925011a9353e4badda91d79"
dependencies = [
 "proc-macro2",
 "quote",
 "syn",
]

[[package]]
name = "serde_json"
version = "1.0.149"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "83fc039473c5595ace860d8c4fafa220ff474b3fc6bfdb4293327f1a37e94d86"
dependencies = [
 "itoa",
 "memchr",
 "serde",
 "serde_core",
 "zmij",
]

[[package]]
name = "serde_spanned"
version = "0.6.9"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "bf41e0cfaf7226dca15e8197172c295a782857fcb97fad1808a166870dee75a3"
dependencies = [
 "serde",
]

[[package]]
name = "strsim"
version = "0.11.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7da8b5736845d9f2fcb837ea5d9e2628564b3b043a70948a3f0b778838c5fb4f"

[[package]]
name = "syn"
version = "2.0.117"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e665b8803e7b1d2a727f4023456bbbbe74da67099c585258af0ad9c5013b9b99"
dependencies = [
 "proc-macro2",
 "quote",
 "unicode-ident",
]

[[package]]
name = "tempfile"
version = "3.27.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "32497e9a4c7b38532efcdebeef879707aa9f794296a4f0244f6f69e9bc8574bd"
dependencies = [
 "fastrand",
 "getrandom",
 "once_cell",
 "rustix",
 "windows-sys",
]

[[package]]
name = "thiserror"
version = "2.0.18"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4288b5bcbc7920c07a1149a35cf9590a2aa808e0bc1eafaade0b80947865fbc4"
dependencies = [
 "thiserror-impl",
]

[[package]]
name = "thiserror-impl"
version = "2.0.18"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ebc4ee7f67670e9b64d05fa4253e753e016c6c95ff35b89b7941d6b856dec1d5"
dependencies = [
 "proc-macro2",
 "quote",
 "syn",
]

[[package]]
name = "toml"
version = "0.8.23"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "dc1beb996b9d83529a9e75c17a1686767d148d70663143c7854d8b4a09ced362"
dependencies = [
 "serde",
 "serde_spanned",
 "toml_datetime",
 "toml_edit",
]

[[package]]
name = "toml_datetime"
version = "0.6.11"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "22cddaf88f4fbc13c51aebbf5f8eceb5c7c5a9da2ac40a13519eb5b0a0e8f11c"
dependencies = [
 "serde",
]

[[package]]
name = "toml_edit"
version = "0.22.27"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "41fe8c660ae4257887cf66394862d21dbca4a6ddd26f04a3560410406a2f819a"
dependencies = [
 "indexmap",
 "serde",
 "serde_spanned",
 "toml_datetime",
 "toml_write",
 "winnow",
]

[[package]]
name = "toml_write"
version = "0.1.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5d99f8c9a7727884afe522e9bd5edbfc91a3312b36a77b5fb8926e4c31a41801"

[[package]]
name = "unicode-ident"
version = "1.0.24"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e6e4313cd5fcd3dad5cafa179702e2b244f760991f45397d14d4ebf38247da75"

[[package]]
name = "unicode-xid"
version = "0.2.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ebc1c04c71510c7f702b52b7c350734c9ff1295c464a03335b00bb84fc54f853"

[[package]]
name = "utf8parse"
version = "0.2.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "06abde3611657adf66d383f00b093d7faecc7fa57071cce2578660c9f1010821"

[[package]]
name = "walkdir"
version = "2.5.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "29790946404f91d9c5d06f9874efddea1dc06c5efe94541a7d6863108e3a5e4b"
dependencies = [
 "same-file",
 "winapi-util",
]

[[package]]
name = "wasip2"
version = "1.0.3+wasi-0.2.9"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "20064672db26d7cdc89c7798c48a0fdfac8213434a1186e5ef29fd560ae223d6"
dependencies = [
 "wit-bindgen 0.57.1",
]

[[package]]
name = "wasip3"
version = "0.4.0+wasi-0.3.0-rc-2026-01-06"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5428f8bf88ea5ddc08faddef2ac4a67e390b88186c703ce6dbd955e1c145aca5"
dependencies = [
 "wit-bindgen 0.51.0",
]

[[package]]
name = "wasm-encoder"
version = "0.244.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "990065f2fe63003fe337b932cfb5e3b80e0b4d0f5ff650e6985b1048f62c8319"
dependencies = [
 "leb128fmt",
 "wasmparser",
]

[[package]]
name = "wasm-metadata"
version = "0.244.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "bb0e353e6a2fbdc176932bbaab493762eb1255a7900fe0fea1a2f96c296cc909"
dependencies = [
 "anyhow",
 "indexmap",
 "wasm-encoder",
 "wasmparser",
]

[[package]]
name = "wasmparser"
version = "0.244.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "47b807c72e1bac69382b3a6fb3dbe8ea4c0ed87ff5629b8685ae6b9a611028fe"
dependencies = [
 "bitflags",
 "hashbrown 0.15.5",
 "indexmap",
 "semver",
]

[[package]]
name = "winapi-util"
version = "0.1.11"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c2a7b1c03c876122aa43f3020e6c3c3ee5c05081c9a00739faf7503aeba10d22"
dependencies = [
 "windows-sys",
]

[[package]]
name = "windows-link"
version = "0.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f0805222e57f7521d6a62e36fa9163bc891acd422f971defe97d64e70d0a4fe5"

[[package]]
name = "windows-sys"
version = "0.61.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ae137229bcbd6cdf0f7b80a31df61766145077ddf49416a728b02cb3921ff3fc"
dependencies = [
 "windows-link",
]

[[package]]
name = "winnow"
version = "0.7.15"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "df79d97927682d2fd8adb29682d1140b343be4ac0f08fd68b7765d9c059d3945"
dependencies = [
 "memchr",
]

[[package]]
name = "wit-bindgen"
version = "0.51.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d7249219f66ced02969388cf2bb044a09756a083d0fab1e566056b04d9fbcaa5"
dependencies = [
 "wit-bindgen-rust-macro",
]

[[package]]
name = "wit-bindgen"
version = "0.57.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1ebf944e87a7c253233ad6766e082e3cd714b5d03812acc24c318f549614536e"

[[package]]
name = "wit-bindgen-core"
version = "0.51.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ea61de684c3ea68cb082b7a88508a8b27fcc8b797d738bfc99a82facf1d752dc"
dependencies = [
 "anyhow",
 "heck",
 "wit-parser",
]

[[package]]
name = "wit-bindgen-rust"
version = "0.51.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b7c566e0f4b284dd6561c786d9cb0142da491f46a9fbed79ea69cdad5db17f21"
dependencies = [
 "anyhow",
 "heck",
 "indexmap",
 "prettyplease",
 "syn",
 "wasm-metadata",
 "wit-bindgen-core",
 "wit-component",
]

[[package]]
name = "wit-bindgen-rust-macro"
version = "0.51.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0c0f9bfd77e6a48eccf51359e3ae77140a7f50b1e2ebfe62422d8afdaffab17a"
dependencies = [
 "anyhow",
 "prettyplease",
 "proc-macro2",
 "quote",
 "syn",
 "wit-bindgen-core",
 "wit-bindgen-rust",
]

[[package]]
name = "wit-component"
version = "0.244.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9d66ea20e9553b30172b5e831994e35fbde2d165325bec84fc43dbf6f4eb9cb2"
dependencies = [
 "anyhow",
 "bitflags",
 "indexmap",
 "log",
 "serde",
 "serde_derive",
 "serde_json",
 "wasm-encoder",
 "wasm-metadata",
 "wasmparser",
 "wit-parser",
]

[[package]]
name = "wit-parser"
version = "0.244.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ecc8ac4bc1dc3381b7f59c34f00b67e18f910c2c0f50015669dde7def656a736"
dependencies = [
 "anyhow",
 "id-arena",
 "indexmap",
 "log",
 "semver",
 "serde",
 "serde_derive",
 "serde_json",
 "unicode-xid",
 "wasmparser",
]

[[package]]
name = "zmij"
version = "1.0.21"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b8848ee67ecc8aedbaf3e4122217aff892639231befc6a1b58d29fff4c2cabaa"
## File: Cargo.toml
[package]
name = "rust-workspace-map"
version = "0.1.0"
edition = "2024"

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
syn = { version = "2", features = ["full", "extra-traits"] }
toml = "0.8"
rayon = "1"
anyhow = "1"
clap = { version = "4", features = ["derive"] }
bon = "3"
thiserror = "2"
glob = "0.3"
proc-macro2 = { version = "1", features = ["span-locations"] }
walkdir = "2.5.0"

[dev-dependencies]
tempfile = "3"
## File: notes/architecture-current.md
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
## File: plans/mvp/MVP_PLAN.md
# Plan: `rust-workspace-map` MVP for `rust-development-pipeline` integration

## Context

`rust-workspace-map` already exists at `/Users/tony/programming/rust-workspace-map/` as a single-binary `syn`-based tool that emits a hierarchical JSON map of a Rust workspace's public API surface (`crates → modules → publicItems`). Two feature-request docs in this repo (`feature-requests/rust-workspace-map-agent-centric-design.md`, `feature-requests/rust-workspace-map-agent-centric-development-plan.md`) propose six new features at once: flat indexes, validation, structural diff, compact mode, scope filtering, plus broad pipeline integration.

The user clarified the motivation: this tool is **not a community-facing product**. It is internal infrastructure for `rust-development-pipeline` and exists to **smoothen the pipeline, accelerate it, and reduce token waste**. The two will evolve together.

The user also picked **Option C** in the prior comparison with `tirth8205/code-review-graph` (CRG): build a Rust-native tool independently and adopt CRG's *architectural* lessons (subcommand surface, edge-confidence labeling stance, conservative-recall posture) **without** importing CRG's heavyweight machinery (SQLite store, daemon, MCP server, watch hooks, vector search, eval harness — all explicitly out of MVP scope).

The measured failure data (`feature-requests/reduce_recurring_problems.md`) names three top recurring fix-task categories:
- **#1 (~40%)** — missing `pub mod` declarations
- **#2 (~20%)** — missing `pub use` re-exports
- **#3 (~20%)** — incomplete consumer updates

The MVP's job is to deterministically prevent #1 and #2 at plan-execution time, and structurally support #3 prevention at plan-decomposition time. Everything beyond that is out of MVP scope.

### Core Design Goal: O(1) lookup for LLM agents

This tool exists to answer structural questions about a Rust workspace in **constant time — one hashmap access, no tree traversal, no path guessing**. An LLM agent asking a question should get an answer the way an IDE answers: instant, precise, pre-computed.

The three flat indexes (`symbols`, `name_index`, `files`) pre-compute the relationships a human engineer builds through interactive IDE navigation:

| Agent question | IDE equivalent | Lookup | Cost |
|---|---|---|---|
| "Is `Task` a type? Where is it defined?" | Go-to-definition | `name_index["Task"]` → `symbols[path]` | O(1) |
| "What file contains module `foo::bar`?" | Go-to-file | iterate `files`, match `entry.module_path == "foo::bar"` (or in-memory reverse map built once at index time) | O(1) amortized |
| "What modules does `lib.rs` declare?" | File structure view | `files["core/src/lib.rs"].module_path` → `crates[].modules[]` join → `submodules` | O(1) hop, then O(1) join |
| "Is this file reachable from the module tree?" | Module graph view | `files[path].parent_module_file.is_some()` (or file is a crate root) | O(1) |
| "Which crate exports `Task`?" | (no direct IDE analog) | `cross_references.types["Task"].exported_by` | O(1) — already shipping in 0.1.0 |

This is the fundamental difference from the hierarchical `crates → modules → publicItems` tree alone. That tree requires the agent to traverse, filter, and guess module paths. The flat indexes eliminate traversal entirely — the agent asks one question, makes one lookup, gets one answer. The tool is not a JSON dump for humans to browse; it is a query engine where every query is O(1) and every answer fits in a few hundred tokens.

**Note on what's *not* O(1) in MVP:** "Who imports this symbol?" at file:line granularity is intentionally deferred — the existing `cross_references.types[name].imported_by` answers it at crate granularity, which covers most measured workflows. File:line precision adds extraction cost and per-symbol storage that no current pipeline integration cites. Add when a measured workflow demands it.

---

## Coding style

### Builder pattern
All new structs (`SymbolEntry`, `FileEntry`) use `#[derive(bon::Builder)]`, consistent with the existing codebase idiom.

### Newtype pattern
Two newtypes wrap the BTreeMap key strings to prevent passing the wrong key kind at compile time:

```rust
pub struct CanonicalPath(String);        // "crate::module::Name" — key for symbols, name_index values, cross_references.types
pub struct WorkspaceRelativePath(String); // "core/src/task.rs"   — key for files
```

Each implements `Display`, `AsRef<str>`, `From<String>`, `PartialOrd/Ord`, `PartialEq/Eq`, `Hash`, `Clone`, and serde `Serialize`/`Deserialize` (transparent). No other pass-through methods.

### Functional style — iterators over for-loops
New code in `indexes.rs`, `validate.rs`, and `lookup.rs` uses iterator pipelines (`flat_map`, `filter_map`, `fold`, `collect`) rather than `mut` accumulator loops. Existing code in `module_tree.rs` and `lib.rs` is touched only where the bug fixes require it — no opportunistic refactoring.

### `mem::take` for the O(n²) error-vec fix
The `errors.clone()` at `module_tree.rs:252` is replaced by passing `&mut Vec<ErrorEntry>` down the recursion and using `extend` in place, eliminating the per-level clone.

### `Option` as iterator
`Option<T>` fields (e.g. `parent_module_file`) are consumed via `.into_iter()` / `.extend(opt)` / `.chain(opt.iter())` in pipeline contexts rather than `if let Some` unwrapping.

---

## Architectural commitments (locked)

### KISS faithfulness — very faithful

The MVP ships only what closes a measured pipeline failure or directly enables a pipeline integration that closes one. Anything else is recorded as deferred and built only when measured pain re-surfaces it.

### Single binary with subcommands (not multiple binaries)

Decomposition into separate binaries (`rwm-index`, `rwm-validate`) is rejected. Pipeline skills make one tool call per step — they do not chain N tools. A single binary with subcommands (`cargo`, `git`, `gh`, `kubectl` idiom) gives one Cargo build, one install path, one `--help`. The Unix property holds *inside* the binary:

- Each subcommand has one job and writes self-contained JSON to stdout.
- Subcommands take a workspace path argument. **`--from-stdin` is deferred** — adding it would force `serde::Deserialize` on every schema type plus a JSON-roundtrip stability commitment, and no current pipeline integration pipes (`compile-plan` calls `validate <project>`, plan-decomposer calls `lookup --file`). Revisit when a piping consumer exists.
- Exit codes carry semantics: `0` clean, `1` tool error, `2` validation found issues (grep convention).

### CLI breaking change — accepted

`index` becomes mandatory in v0.2.0. The bare-path form `rust-workspace-map <PATH>` is removed entirely (not aliased, not deprecation-warned). Internal-only tool, no external users to migrate.

### Cache / MCP / daemon / eval harness — explicitly out of MVP scope

These are CRG-shaped maturity items. Built only when (a) the MVP has shipped, (b) the pipeline is using it for at least 3 phases, and (c) measured pain points justify each one individually. Recorded in §"Out of MVP scope" below.

---

## MVP surface

### CLI

```
rust-workspace-map index  [PATH] [-o FILE] [--validate]        # --validate runs OrphanFile + DeadReExport checks
rust-workspace-map lookup [PATH] (--symbol NAME | --file PATH)
```

### Schema additions

Field additions are additive (no struct fields removed). The `cross_references.types` map key format changes from short-name to fully-qualified path — this is a **breaking change** to the JSON output and is intentional (v0.2.0 breaking-change window, same commit as the `index` subcommand mandate).

Two principles drive the shape:

1. **Reuse `WorkspaceMap.errors: Vec<ErrorEntry>` for new diagnostics.** It already carries `severity: ErrorSeverity::{Error, Warning}`, `kind`, `file`, `line`, `message`, `context`, `cause`. A parallel `warnings: Vec<Warning>` channel duplicates that pipeline; new validate rules emit `ErrorEntry { severity: Warning, kind: DiagnosticKind::OrphanFile, … }` into the existing collection.
2. **Migrate `ErrorEntry.kind: String → kind: DiagnosticKind`.** Today the field is stringly-typed. The MVP introduces a typed enum covering both existing values (currently passed as strings — `OrphanedModule`, `TomlParseError`, `MissingCrateRoots`, `ModuleTreeError`) and new ones (`OrphanFile`, `DeadReExport`). Serde-rename keeps the JSON output a string for backward compatibility; the change is purely internal typing.

```rust
// in src/schema.rs
pub struct WorkspaceMap {
    pub workspace: WorkspaceInfo,
    pub crates: Vec<CrateInfo>,
    pub cross_references: CrossReferences,
    #[builder(default)]
    pub symbols:    BTreeMap<String, SymbolEntry>,   // NEW — canonical-path keyed ("crate::module::Name")
    #[builder(default)]
    pub name_index: BTreeMap<String, Vec<String>>,   // NEW — short name → list of canonical paths (collision support)
    #[builder(default)]
    pub files:      BTreeMap<String, FileEntry>,     // NEW — keyed on workspace-relative file path
    pub errors: Vec<ErrorEntry>,                     // existing — also receives validate findings
    pub workspace_root: PathBuf,                     // existing
}

pub struct SymbolEntry {     // map key = "crate::module::Name"
    pub crate_name: String,
    pub module: String,
    pub file: String,
    pub line: usize,
    pub kind: ItemKind,
    // Deferred until a measured workflow cites them:
    //   - re_exported_at: Vec<{file, line}>
    //   - imported_by:    Vec<{crate_name, file, line}>
    //   - confidence:     EXTRACTED | INFERRED | AMBIGUOUS  (no inference path exists in MVP)
    // Crate-granularity imported_by/exported_by already lives in CrossReferences.types.
    // Key format: root-module items = "crate::Name" (2 segments);
    // nested-module items = "crate::module::Name" (3+ segments).
}

// The `cross_references.types` map is re-keyed from short-name to fully-qualified path
// ("crate::module::Name") to avoid silent data loss on name collisions across crates.

pub struct FileEntry {
    pub module_path: String,                 // joins back to the matching ModuleInfo
    pub parent_module_file: Option<String>,  // critical for plan-decomposer wiring check
    pub is_crate_root: bool,
    // Deliberately *not* duplicating `crate_name`, `submodules`, or `exports` —
    // they already exist in the hierarchical view via `module_path → ModuleInfo`.
    // This keeps the `files` index small while still giving agents an O(1) hop
    // from file path to the structural facts only available through this index.
    // Inline modules (mod foo { ... }) share their parent's file.
    // They are excluded from the `files` index to avoid one-file-to-many-module key collisions.
}

// New typed kind for ErrorEntry. Variants serde-rename to existing string values
// where applicable so JSON output stays stable.
pub enum DiagnosticKind {
    // Existing string-valued kinds, now typed:
    OrphanedModule,         // serde rename: "orphaned_module"
    TomlParseError,         // serde rename: "toml_parse_error"
    MissingCrateRoots,      // serde rename: "missing_crate_roots"
    ModuleTreeError,        // serde rename: "module_tree_error"
    // New kinds emitted by `validate`:
    OrphanFile,             // .rs in src/ with no `pub mod` / `mod` in parent — fires on category #1
    DeadReExport,           // `pub use foo::Bar` where Bar isn't found — fires on category #2
}
```

**Note on flat indexes vs `lookup`.** The plan keeps both: indexes inside the JSON output *and* the `lookup` subcommand. They cover overlapping access patterns — direct map-load by an agent, and targeted CLI queries respectively. Primary access pattern is TBD; revisit at the 3-phase castep-cell-io review and drop whichever path proves redundant.

### `validate` rules — two, with two more deferred

| Rule | Fires when | Pipeline category |
|---|---|---|
| `OrphanFile` | after `build_module_tree` returns for a crate, collect all `ModuleInfo.file` paths (excluding `<unresolved>`); walk the crate's `src/` for all `*.rs` files; any `.rs` on disk absent from the module-tree file set is orphan. Excludes `build.rs`, `src/bin/*.rs`, and test/example directories. | #1 (~40%) |
| `DeadReExport` | a `pub use` path whose target cannot be resolved within the workspace. Algorithm: (1) resolve `crate::`, `self::`, `super::` prefixes; (2) if the first path segment is not a workspace member crate, skip (external re-export — not dead); (3) if the path is a glob (`pub use path::*`), skip (cannot evaluate); (4) for remaining intra-workspace paths, check whether the named item exists in any module reachable through the path prefix. Transitive re-export chains are not traced in MVP. | #2 (~20%) |

Each finding becomes an `ErrorEntry { severity: Warning, kind: DiagnosticKind::OrphanFile | DeadReExport, file, line, message, … }` in `WorkspaceMap.errors`. Message includes a fix hint (e.g., `add 'pub mod foo;' to core/src/lib.rs`). Conservative recall is policy: false negatives over false positives.

**Deferred from MVP:**

- `UnreachablePub` — a `pub` item with no path to a public crate root and no `pub use` rescuing it. The plan originally cited this as "partial #3," but #3 (incomplete consumer updates) and unreachable-pub describe negligibly overlapping problem shapes. Defer until a measured plan-execution failure traces to it.
- `UnsupportedLayout` warning — `mod foo { … }` and `#[path = ...]` are out of scope for AST analysis. **Document the limitation in `README.md`** under "What this tool does and doesn't analyze," rather than emitting a structured diagnostic the consumer cannot act on.

### `lookup` semantics

- `--symbol Task` → resolves through `name_index["Task"]`; if it points to one canonical path, emits the matching `SymbolEntry`. If many, emits the list of canonical paths plus disambiguation hints. Exit `0` on found, `1` on not-found.
- `--file core/src/task.rs` → looks up `FileEntry` and scans `crates[].modules[]` for **all** `ModuleInfo` entries whose `.file` matches the requested path. Returns `{ file_entry: FileEntry, primary_module: ModuleInfo, inline_modules: Vec<ModuleInfo> }` where `primary_module` is the entry whose `path` matches `FileEntry.module_path`, and `inline_modules` are all other entries sharing the same `.file` (e.g. `mod tests { … }` in `lib.rs`). The `files` index still excludes inline modules (avoids key collisions); the join step recovers them.

This is the *targeted query* form — the plan-decomposer asks one specific question and gets one specific answer, no JSON-dump-in-context.

`lookup` is a human CLI convenience for the MVP. Pipeline agents should embed flat indexes in context (single JSON load) rather than making per-symbol CLI calls — each `lookup` invocation performs a full workspace scan. Revisit `lookup` as a pipeline tool after the cache layer ships.

---

## Pipeline integrations shipped *with* the MVP

The user's reframing makes pipeline integration part of the MVP, not a follow-up phase. The MVP isn't done until the pipeline actually uses it.

| Pipeline file | Change |
|---|---|
| `skills/compile-plan/SKILL.md` | **Authoritative gate.** Add a pre-check step: run `rust-workspace-map index --validate <project>`. Block plan execution on any `OrphanFile` or `DeadReExport` finding (exit code 2). This is deterministic — no agent compliance required. |
| `agents/plan-decomposer.md` | **Soft suggestion only.** Add the **Module Wiring Check** guidance from `feature-requests/reduce_recurring_problems.md` §3.1, recommending — not requiring — that the agent call `rust-workspace-map lookup --file <parent>` when introducing a new file, to confirm intended siblings. Optionally call `lookup --symbol <name>` to check for name collisions on a planned re-export. The plan-decomposer's correctness is *not* gated on this; the deterministic gate lives in `compile-plan`. |

The `compile-plan` pre-check is what turns the binary from a CLI demo into a real failure-prevention tool. The plan-decomposer suggestion is upstream defense-in-depth: free if the agent follows it, harmless if it doesn't, never the sole gate.

The other integrations from the original design (`enrich-plan-gather` Step 2 replacement, `review-pr-gather` `diff` ground truth) are explicitly **deferred until after the MVP has been used through ≥3 phases of `castep-cell-io` work** — that gives concrete data on whether the cheaper integrations close enough of the failure rate to make the more ambitious ones worth building.

---

## Files to create / modify

### `rust-workspace-map/`

| File | Action |
|---|---|
| `src/schema.rs` | Add `SymbolEntry` (slim — see schema block), `FileEntry` (slim — three fields), `DiagnosticKind` enum. Extend `WorkspaceMap` with `symbols`, `name_index`, `files` BTreeMap fields. Migrate `ErrorEntry.kind: String → kind: DiagnosticKind` (serde-renamed to keep JSON output stable). **Do not add** `Confidence`, `Warning`, `WarningKind`, `ImportSite`, `ReExportSite`, or `warnings: Vec<Warning>`. |
| `src/lib.rs` | After existing `cross_refs::compute`, call `indexes::derive_from_crates(&crate_infos)` and pass results to `WorkspaceMap::builder()`. Run validation (if `--validate`) after construction, merging findings into `errors`. Update emit-sites in this file to use the new `DiagnosticKind` variants. Set `WorkspaceInfo.root` to the discovered workspace root path (currently hardcoded to `"."`). Fix dropped-errors bug at lines 132–137: collect `errs` from both `Some` and `None` branches of the crate-results drain loop (currently the `None` branch silently discards errors from failed crates). |
| `src/main.rs` | Switch to `clap` subcommands: `index` (with `--validate` flag), `lookup`. Bare-path form removed. Each subcommand takes a path. **No `--from-stdin`** — deferred. |
| `src/lookup.rs` | NEW — pure function over `&WorkspaceMap` implementing `--symbol` and `--file` filters. `--file` looks up `FileEntry` then scans all `ModuleInfo` entries sharing the same `.file`, returning `primary_module` + `inline_modules`. |
| `src/validate.rs` | Validation is merged into `index --validate`. No separate subcommand. The `OrphanFile` and `DeadReExport` logic lives in `src/validate.rs` as a function called by `lib.rs::run()` when `--validate` is set. |
| `src/indexes.rs` | NEW — pure function `derive_from_crates(&[CrateInfo]) -> (symbols, name_index, files)` in one pass over `crates[].modules[]`. Called before `WorkspaceMap` builder. No second AST traversal. |
| `src/module_tree.rs`, `src/file_parser.rs` | Update emit-sites to construct `ErrorEntry` with `kind: DiagnosticKind::OrphanedModule` etc. Fix error-vec cloning at line 252 (O(n²) memory waste) and fragile `unwrap_or` at line 33. |
| `src/cross_refs.rs` | No structural changes for the MVP. The crate-granularity `imported_by`/`exported_by` already collected stays as-is. File:line granularity per symbol is deferred until a measured workflow cites it. |
| `tests/fixtures/sample-workspace/` | Add `bad-orphan/` and `bad-dead-reexport/` sub-fixtures. |
| `tests/integration_test.rs` | (a) Rewrite **every** existing invocation from the bare-path form to `index <path>` (this is the breaking-change call-site sweep). (b) Add tests for `validate` exit code 2 on the new fixtures. (c) Add tests for `lookup --symbol` (single + ambiguous + not-found) and `lookup --file`. |
| `README.md` | Document subcommand surface; add a **"What this tool does and doesn't analyze"** section explicitly noting that inline `mod foo { … }` and `#[path = ...]` items are not analyzed. Add an "Inspiration" section crediting `tirth8205/code-review-graph` (MIT) and stating the differentiation: Rust-only via `syn`, native `pub`/`pub use`/`mod` semantics, one-shot CLI, no daemon/MCP/SQLite. |

### Existing utilities reused (do NOT reinvent)

- `src/cargo_info.rs::parse_cargo_toml` — existing dep + package + edition parser.
- `src/module_tree.rs::build_module_tree` — already resolves `mod foo;` to `foo.rs` / `foo/mod.rs`. **Phase-1 `OrphanFile` rule reuses this resolver in reverse**: walk every `.rs` under the crate source dir, check whether the resolver included it.
- `src/cross_refs.rs::compute` — already populates `crossReferences.types` with crate-granularity `imported_by`/`exported_by` keyed by short symbol name. The MVP **does not** extend its granularity (file:line per-symbol is deferred). The `name_index` flat index is largely a re-shaping of the same data.
- `src/render.rs::render_to_writer` — single rendering path, used by all subcommands.
- `src/file_parser.rs` — `syn` parsing wrapper, no changes.
- `bon::Builder` derive pattern — already idiomatic in the codebase; new types use it for consistency.

### `rust-development-pipeline/`

| File | Action |
|---|---|
| `skills/compile-plan/SKILL.md` | **Authoritative gate.** Add `validate` pre-check that blocks on `OrphanFile` / `DeadReExport` findings (exit code 2). |
| `agents/plan-decomposer.md` | **Soft suggestion only.** Add Module Wiring Check section recommending `rust-workspace-map lookup --file <parent>` and `lookup --symbol <name>` calls when introducing new files / re-exports. Phrase as guidance, not a procedural mandate. |

---

## Verification (MVP done = all of these green)

1. **Build green**: `cargo build --release` in `rust-workspace-map/`.
2. **Call-site sweep**: every invocation in `tests/integration_test.rs` rewritten from the bare-path form to `index <path>` in the same commit as the breaking change. README examples and any wrapper scripts in `rust-development-pipeline/` updated in lockstep. This is the flag-day checklist for D1.
3. **Existing tests green**: all 8 integration tests in `rust-workspace-map/tests/integration_test.rs` still pass after the rewrite, with the additive schema.
4. **DiagnosticKind migration green**: the existing `orphaned_module` warning emitted by `module_tree.rs` still serializes as the string `"orphaned_module"` in JSON (verified by an integration test on a known-orphan fixture). No JSON-output regression.
5. **New unit fixtures**: `bad-orphan/` triggers an `OrphanFile` finding naming the parent file; `bad-dead-reexport/` triggers a `DeadReExport` finding. `index --validate` exits `2` in both cases. Findings appear in `WorkspaceMap.errors` with `severity: Warning`.
6. **Lookup**: against the existing `sample-workspace`, `lookup --symbol Task` returns the matching `SymbolEntry`; `lookup --file core/src/task.rs` returns the joined `FileEntry`+`ModuleInfo`; `lookup --symbol DoesNotExist` exits `1`.
7. **Real-world dry-run + cache baseline**: `rust-workspace-map index --validate /Users/tony/programming/castep-cell-io` and `rust-workspace-map index --validate /Users/tony/programming/castep-cell-io` (303 files) run to completion. Record wall-time in `notes/` as the **0.1.0→0.2.0 cache baseline**. The cache layer (currently deferred) is allowed only if a future change pushes wall-time past 2× this baseline AND `compile-plan` runs index --validate ≥3× per phase. Estimated baseline: 100–400 ms; the original 2 s threshold is unlikely to ever trigger.
8. **Pipeline smoke**: in `rust-development-pipeline`, run `compile-plan` against a synthetic plan that creates a file without a `pub mod`. The pre-check blocks (via `index --validate`). Run another synthetic plan that introduces a `pub use` to a nonexistent symbol. The pre-check blocks (via `index --validate`). Both behaviors are deterministic — no LLM compliance involved.

---

## Out of MVP scope (explicitly deferred — built only when the pipeline measurably needs them)

These are recorded so they don't get rebuilt as ad-hoc additions. Each requires concrete pipeline pain to justify:

**Cut from this revision of the plan (after KISS review):**

- **`UnreachablePub` validate rule** — was originally framed as "partial #3," but #3 (incomplete consumer updates) and unreachable-pub describe negligibly overlapping problem shapes. Defer until at least one measured plan-execution failure traces to an unreachable-pub case the rule would have caught.
- **`UnsupportedLayout` warning** — covered by a README sentence ("What this tool does and doesn't analyze") rather than a structured diagnostic the consumer cannot act on.
- **`Confidence` enum (`EXTRACTED`/`INFERRED`/`AMBIGUOUS`)** — every value would be `EXTRACTED` in the MVP because no inference path exists. Add the day a non-AST data source is introduced.
- **Per-symbol `imported_by` / `re_exported_at` at file:line granularity** (`ImportSite`, `ReExportSite` types) — the three rules don't need them and crate-granularity already lives in `CrossReferences.types`. Add when a measured workflow cites "find references" with file:line precision.
- **`--from-stdin` on `validate` and `lookup`** — would force `serde::Deserialize` on every schema type plus a JSON-roundtrip stability commitment, with no current piping consumer. Add when a third caller requests it.
- **Parallel `Vec<Warning>` channel** — collapsed into the existing `WorkspaceMap.errors` with the typed `DiagnosticKind`. No reason to revive the parallel channel.
- **Hard-mandate plan-decomposer `lookup` calls** — soft suggestion is the chosen design. Hard mandates depending on LLM compliance fail silently; the deterministic gate is `compile-plan`'s pre-check.

**Originally deferred (carried forward unchanged):**

- **`diff <base-ref>` subcommand** — defer until `review-pr` regex parsing produces wrong results that an AST-based diff would have caught. The bar: at least one fix round attributable to a missed structural change in review.
- **`--compact` and scope filters (`--crates`, `--files`)** — defer until measured token use exceeds 50% of an agent's context window on `castep-cell-io`-size workspaces.
- **File-mtime-keyed cache** at `~/.cache/rust-workspace-map/<workspace-hash>.json` — defer until verification step 7 measures wall-time past 2× the recorded baseline, OR the `compile-plan` pre-check is observed running ≥3× per phase.
- **`serve` mode / MCP server** — defer until the pipeline grows its first MCP server. The pipeline currently has zero (`/Users/tony/programming/rust-development-pipeline/.mcp.json` does not exist).
- **Derive-macro expansion** (`bon::Builder`, `serde`, `thiserror`) — defer until a real plan-decomposer task fails because the symbol it needed was macro-generated and missing from `symbols`. Track misses by adding a "no symbol found, but file matches a `#[derive]` site" hint to `lookup`.
- **Eval harness (CRG `eval/`-style A/B replay)** — defer indefinitely. The MVP does not need a published reduction number; it needs the pipeline's fix-task ratio to drop. That metric already exists in `notes/pr-reviews/`.
- **NDJSON streaming output** — defer until a streaming consumer exists.
- **Multi-language Tree-sitter coverage** — out of scope permanently. Rust-only is the moat.

---

## CRG attribution

In `README.md` of `rust-workspace-map`, add a short "Inspiration" section:

> The subcommand surface and conservative-recall validation policy are inspired by [`tirth8205/code-review-graph`](https://github.com/tirth8205/code-review-graph) (MIT). This tool is differently shaped: Rust-only via `syn`, models `pub`/`pub use`/`mod` semantics natively, and is a one-shot CLI rather than a daemon-backed graph store. CRG's edge-confidence stance (`EXTRACTED`/`INFERRED`/`AMBIGUOUS`) is *not* yet adopted — the MVP performs only AST extraction, so every value would be `EXTRACTED`. Revisit if a non-AST data source is introduced.

---

## Locked decisions (recorded so they don't drift)

- **D1**: `index` mandatory immediately. Bare-path form removed in v0.2.0. No deprecation alias. Call-site sweep is part of the same commit (verification step 2). The `cross_references.types` re-keying (short-name → fully-qualified path) is also a breaking change and ships in this same commit.
- **D2**: MVP scope =
  - **subcommands**: `index` (with `--validate` flag), `lookup` (no `--from-stdin`)
  - **schema additions**: slim `SymbolEntry`, slim `FileEntry`, three flat indexes (`symbols`, `name_index`, `files`), `DiagnosticKind` enum (replacing `ErrorEntry.kind: String`), re-key `cross_references.types` from short-name to fully-qualified path
  - **schema *not* added**: `Confidence`, `Warning`, `WarningKind`, `ImportSite`, `ReExportSite`, parallel `warnings: Vec<Warning>`
  - **validate rules**: `OrphanFile`, `DeadReExport` (5-case algorithm; run via `index --validate`)
  - **pipeline integrations**: `compile-plan` pre-check (deterministic gate); `plan-decomposer` Module Wiring Check (soft suggestion only)
- **D3**: Cache layer deferred. Gate: wall-time on `castep-cell-io` exceeds 2× the verification-step-7 baseline AND `compile-plan` runs validate ≥3× per phase.
- **D4**: Pipeline integrations ship *with* the MVP, not after — the tool isn't done until the pipeline uses it.
- **D5**: Flat indexes and `lookup` are belt-and-suspenders. Primary access pattern (raw-map reading vs `lookup` calls) to be determined empirically over ≥3 castep-cell-io phases; the redundant path may be dropped at that review.
- **D6**: Per-symbol reverse data (file:line `imported_by`, `re_exported_at`) is gated on a measured workflow citation. Adding it speculatively would ship unread data and obligate maintenance forever.
## File: plans/mvp/MVP_PLAN_review.md
# Review of `MVP_PLAN.md` — KISS / first-principles critique

## Context

The MVP plan claims "very faithful" KISS and "ships only what closes a measured pipeline failure." Read end-to-end against the current state of `/Users/tony/programming/rust-workspace-map/`, parts of the proposed surface are speculative scaffolding that don't survive first-principles scrutiny on the right cost axis. The measured failure data — #1 `pub mod` (~40%), #2 `pub use` (~20%), #3 consumer updates (~20%) — supports a tighter MVP that earns the right to grow only if it doesn't close enough of the failure rate.

This document records the points where the plan over-builds and proposes a tighter cut. After review with the user (who clarified that the "O(1) lookup" framing was about token cost / agent reasoning load, not wall-clock latency), the critique was sharpened: flat indexes are accepted, but `Confidence`, `UnreachablePub`, parallel warning channel, `--from-stdin`, and other items remain over-built when the principle is applied uniformly.

## Ground truth that changes the calculus

Things the plan understates or omits about what already exists today:

- `schema::CrossReferences { types: BTreeMap<String, TypeRef> }` is **already** keyed by short symbol name, with `imported_by: Vec<String>` and `exported_by: Vec<String>` populated by `cross_refs::compute`. This is structurally what the plan calls `name_index` + part of `imported_by`. The plan presents flat indexes as new infrastructure when ~70% of one already ships in 0.1.0.
- `module_tree.rs` **already detects orphan modules in one direction**: a `mod foo;` declared but with no resolvable file emits an `orphaned_module` warning entry (`module_tree.rs:120–134`). The plan's `OrphanFile` is the reverse direction (file exists but no `mod foo;`), which is a real gap, but the plan doesn't note that the existing detector and the new one should share an `ErrorEntry` / warning emission pipeline rather than a parallel `Vec<Warning>` channel.
- `WorkspaceMap.errors: Vec<ErrorEntry>` already exists, with `severity: ErrorSeverity::{Error, Warning}`, `kind`, `context`, `cause`, `file`, `line`. The plan adds a parallel `warnings: Vec<Warning>` channel without explaining why the existing `ErrorEntry` with `Severity::Warning` won't do.
- All schema types derive `serde::Serialize` only — none derive `Deserialize`. The plan's `--from-stdin` requires `Deserialize` on every type plus a JSON-roundtrip stability commitment. This is a substantive cost the plan doesn't account for.
- `bon::Builder` is used pervasively; new types fitting that idiom is correct.
- `clap` derive is already wired; subcommand split is mechanical.

## The KISS critique, point by point

### 1. "O(1) lookup" — accepted on token-cost grounds, with a remaining tension

The plan justifies three pre-computed flat indexes (`symbols`, `name_index`, `files`) under "Core Design Goal: O(1) lookup for LLM agents." The intended cost axis is **token cost / agent reasoning load**, not wall-clock latency: a flat index lets an agent (or a `lookup` subcommand) jump straight to the answer rather than mentally tree-walking the hierarchical `crates[].modules[].public_items[]` shape — and prevents the agent's common fallback of running `rg`/`read` against the source when the JSON's shape isn't query-friendly. That argument is legitimate and KISS-aligned: pre-shaped answers are exactly the kind of thing measured "agent friction" should pay for.

The tension that remains, worth making explicit before building all three indexes:

- **If the dominant access pattern is `lookup --symbol/--file`** — i.e., the agent never sees the raw map — then the indexes only need to live as in-memory acceleration *inside* the `lookup` implementation. The JSON output of `index` does not need to carry them, because no agent reads the dump directly.
- **If the dominant access pattern is "load full map into agent context, reason globally"** — e.g., enrich-plan-gather, plan-review, rust-architect — then the indexes belong in the JSON output, because the JSON's shape *is* the agent's query interface. The duplication (each item appears in both the hierarchical view and the flat view) is the cost paid for query ergonomics.

The plan currently does both: indexes *and* `lookup`. That's a reasonable belt-and-suspenders position, but it's worth recording which access pattern is primary so future iteration knows what to trim. If `lookup` is the primary path, the indexes can be dropped from JSON output (kept in-memory only) once that's measured. If raw-map reading is primary, `lookup` becomes the redundant path.

**No change to recommendation on the indexes themselves.** Keep `symbols`, `name_index`, `files` in the MVP, but flag in the plan: *"primary access pattern TBD; revisit at the 3-phase review whether `lookup` or raw-map reading dominates, drop the redundant path then."*

The remaining first-principles concerns from the original critique survive on a different axis:

- `Confidence` is still YAGNI (every value will be `EXTRACTED`; no inference path exists in the MVP). This is independent of token cost — a constant-valued field neither shapes a query nor saves agent reasoning.
- `imported_by` / `re_exported_at` per-symbol, at file:line granularity, is justified *if* it's actually queried by a measured workflow. The MVP's three rules don't need it. `lookup --symbol` can return it cheaply — but if no agent prompt cites "who imports X," the data ships unread. Worth gating on a concrete pipeline citation.

### 2. `Confidence { EXTRACTED, INFERRED, AMBIGUOUS }` is pure YAGNI

The plan imports a CRG idiom into a tool that does only `syn` AST extraction. Every value will be `EXTRACTED`. There is no inference path, no ambiguity-resolver. Adding the field means consumers either ignore a constant field (noise) or branch on a value that never differs (dead code).

**Recommendation:** delete `Confidence` from the MVP. Add the day a non-AST data source is introduced, never before.

### 3. The new MVP rules don't need per-symbol reverse indexes

Walk through what each `validate` rule actually requires:

| Rule | Inputs needed | Already have? |
|---|---|---|
| `OrphanFile` | filesystem walk of crate `src/` + `module_tree::resolve_module_path` to check inclusion | Yes — `resolve_module_path` exists |
| `DeadReExport` | forward scan: for each `pub use a::b::C`, look for `C` in module `a` (or transitively) | Yes — `crates[].modules[].public_items[]` is the forward index |
| `UnreachablePub` | reachability over module tree | Tree exists; pass is new but cheap |

None of these require `imported_by` or `re_exported_at` per-symbol. The plan adds them to feed `lookup`, then mandates `lookup` in the plan-decomposer. That's circular justification — the indexes exist to support a subcommand that exists to motivate the indexes.

**Recommendation:** keep flat indexes on token-cost grounds (per §1), but defer the per-symbol reverse fields (`imported_by` at file:line, `re_exported_at` at file:line) until a measured workflow cites them. The crate-granularity `imported_by`/`exported_by` already in `CrossReferences.types` covers the common "which crate exports X" question.

### 4. `UnreachablePub` is scope creep mislabeled as "partial #3"

#3 is "incomplete consumer updates" — when an existing public item's signature/usage changes and callers don't get updated. `UnreachablePub` detects `pub` items with no public path from a crate root. These are different problems with negligible overlap. Calling it "partial #3" lets a speculative rule ride into the MVP on real-failure-data coattails.

The rule also adds graph-reachability logic for the smallest expected hit rate of the four rules.

**Recommendation:** drop `UnreachablePub` from MVP. Ship the two rules that close the measured 60%, observe for ≥3 phases per the plan's own "Out of scope" methodology, then decide.

### 5. `UnsupportedLayout` is not a rule, it's a README sentence

`mod foo { … }` and `#[path = ...]` exist in real code; emitting a structured warning that says "we didn't analyze this" produces noise on every run for workspaces that use either form. The user (the pipeline) has no action to take on it.

**Recommendation:** document the limitation in `README.md` under "What this tool does and doesn't analyze." If a real plan-execution failure ever traces to an unanalyzed inline mod, then add the warning.

### 6. `--from-stdin` adds composability with no consumer

The plan justifies `--from-stdin` with `index | validate --from-stdin | jq` as a "Unix property" worth preserving. But:

- Both pipeline integrations call subcommands directly (`validate <project>`, `lookup --file <parent>`). Neither pipes.
- No external user exists.
- Verification step #5 tests piping but the test exists to validate infrastructure with no caller.

The hidden cost: every schema type currently derives `serde::Serialize` only. `--from-stdin` requires `serde::Deserialize` everywhere plus a roundtrip-stability commitment (camelCase serialization is asymmetric on some types). That's ~30 derives plus a stability gate the plan doesn't acknowledge.

**Recommendation:** drop `--from-stdin` from MVP. Subcommands take a path. If a third caller asks for piping, add it then.

### 7. `FileEntry` duplicates `ModuleInfo` content — but the duplication is the point if the index is keyed on file path

Field-by-field:

| `FileEntry` field | Already in `ModuleInfo`? |
|---|---|
| `crate_name` | derivable from `path.split("::").first()` |
| `module` | yes — `path` |
| `submodules` | yes — `submodules` |
| `exports` | derivable — `public_items.iter().map(|p| &p.name)` |
| `is_crate_root` | derivable — `path` has no `::` |
| `parent_module_file` | new — needs a parent-of map built from `submodules` reverse |

Under the time-cost lens, this is wasteful denormalization. Under the token-cost lens (per §1 above), the `files: BTreeMap<String, FileEntry>` is *keyed on the file path string*, which the hierarchical `crates[].modules[]` view is not. An agent asking "what's in `core/src/task.rs`?" hits the file index directly; without it, the agent has to scan modules and match on `file`.

So the right framing isn't "delete `FileEntry`," it's:

- The genuinely new fields are `is_crate_root` and `parent_module_file`. Everything else is denormalization driven by the indexing-key choice (file path).
- Pick the slimmest `FileEntry` that pays for the new key. One viable shape: `{ module_path, parent_module_file, is_crate_root }` — three fields, with `lookup --file` re-joining to the matching `ModuleInfo` for the rest. Avoids re-serializing `submodules` and `exports` into the JSON when they're already in the hierarchical view.
- Or accept the duplication outright and document that the JSON is pre-denormalized for agent ergonomics.

**Recommendation:** keep `files` as a flat index, but slim `FileEntry` to the three fields the hierarchical view doesn't already cover (`module_path`, `parent_module_file`, `is_crate_root`). `lookup --file` reads the slim entry and joins to the corresponding `ModuleInfo`. JSON output stays compact; agents loading the raw map can still hashmap-lookup by file path.

### 8. Replace stringly-typed `ErrorEntry.kind` with an enum, don't add a parallel `Vec<Warning>` channel

Workspace already has `ErrorEntry { file, line, message, severity, kind: String, context, cause }` with `ErrorSeverity::Warning`. The plan's new `Vec<Warning>` channel duplicates infrastructure for no reason — the same row of data fits in `ErrorEntry` exactly. Per user preference, an enum is the right shape (over the current stringly-typed `kind`), so:

**Recommendation:**

1. Introduce a single `enum DiagnosticKind` (or `WarningKind` if you want to keep the name; "diagnostic" reads better given existing rows already cover both errors and warnings) with variants for the existing string values currently passed (`OrphanedModule`, `TomlParseError`, `MissingCrateRoots`, `ModuleTreeError`, plus the new `OrphanFile`, `DeadReExport`, etc.).
2. Migrate `ErrorEntry.kind: String → kind: DiagnosticKind`. This is a one-shot refactor: every emit-site in `lib.rs`, `module_tree.rs`, `file_parser.rs` becomes a typed variant; the JSON `kind` field stays a string via serde rename, so consumers don't see a breaking change.
3. The new validate rules emit `ErrorEntry { severity: Warning, kind: DiagnosticKind::OrphanFile, ... }` into the existing `WorkspaceMap.errors`. No `Vec<Warning>`. No parallel channel.

This is the disciplined version of the plan's intent — typed kinds — applied consistently rather than only to the new code path. It also eliminates the asymmetry where existing `orphaned_module` warnings and new `OrphanFile` warnings would have lived in different collections with different typing.

### 9. Plan-decomposer hard-mandate-to-call-lookup is fragile defense-in-depth

The plan requires the plan-decomposer LLM to call `lookup --file <parent>` before emitting any new-file `[[changes]]` entry, with the result cited in plan rationale.

- LLM compliance with hard procedural mandates is probabilistic. A skipped call can't be detected in-band.
- The downstream `compile-plan` pre-check already catches the issue deterministically.
- If the pre-check is the gate, the plan-decomposer's call is redundant work; if the pre-check misses, the plan-decomposer almost certainly missed the same case.

**Recommendation:** make the `compile-plan` pre-check the authoritative gate. Plan-decomposer integration becomes a soft suggestion ("when introducing a new file, consider `lookup --file <parent>` to confirm intended siblings"). Soft suggestions cost nothing if skipped; hard gates relying on agent compliance fail silently.

### 10. The verification matrix has the wrong cache trigger

Verification step #6 sets a 2-second wall-time trigger on `castep-cell-io` for the cache deferral. First-principles estimate: AST parse 303 small files in parallel (`rayon` is already used) on modern hardware ≈ 100–400 ms. The 2-second threshold is unlikely to ever trigger. That's fine, but the plan should record the **expected** baseline so the cache truly remains deferred rather than becoming a "we measured 1.8s, let's be safe" creep target.

**Recommendation:** measure current 0.1.0 wall-time on `castep-cell-io` once. Lock that as baseline. Cache layer is allowed only if a future change pushes wall-time past 2× baseline AND `compile-plan` runs validate ≥3× per phase.

### 11. The breaking-change mechanics are under-specified

`index` becomes mandatory in v0.2.0; bare-path form deleted entirely. Fine decision. But the plan doesn't enumerate:

- Every invocation in `tests/integration_test.rs` needs rewriting.
- Any wrapper scripts in `rust-development-pipeline/` need updating in the same commit.
- README examples need updating.
- Anything in the user's `notes/` that has copy-pasteable invocations.

This isn't an objection to the decision, just to the breeziness. A breaking change is a flag-day event; the plan should have a checkbox for the call-site sweep.

## The principle the plan violates

The plan articulates the right principle: *"ships only what closes a measured pipeline failure or directly enables a pipeline integration that closes one. Anything else is recorded as deferred."* It then applies that principle unevenly. Some features get the discipline:

- `diff` subcommand → deferred
- `--compact` / scope filters → deferred
- File-mtime cache → deferred
- `serve` mode / MCP → deferred
- Macro expansion → deferred
- Eval harness → deferred
- NDJSON streaming → deferred

Other features remain in scope. Of those, some are justified on the right axis (per §1 token-cost argument) and survive the principle:

- Flat indexes (`symbols`, `name_index`, slim `files`) → justified by token cost / agent reasoning load. **Keep.**

Others remain over-built when the principle is applied uniformly:

- `Confidence` enum → no inference path exists; constant-valued field. **Cut.**
- `imported_by` / `re_exported_at` per-symbol at file:line granularity → not required by the three rules; ships unread unless a measured workflow cites it. **Defer.**
- `UnreachablePub` rule → "partial #3" attribution is overstated; cheap to add later. **Defer.**
- `UnsupportedLayout` warning → no actionable response from the consumer. **Make a README sentence.**
- `--from-stdin` → no piping consumer; hidden `Deserialize` cost. **Defer.**
- Parallel `Vec<Warning>` channel → duplicates `WorkspaceMap.errors`. **Reuse `ErrorEntry` with typed `kind`.**
- Plan-decomposer hard-mandate-to-call-`lookup` → fragile compliance gate when `compile-plan` pre-check is the deterministic gate. **Soft suggestion only.**

## A KISS-cut MVP

What ships if the plan's stated principle is applied uniformly, with §1's token-cost reasoning preserved:

**Schema delta** (in `src/schema.rs`):
- Migrate `ErrorEntry.kind: String → kind: DiagnosticKind` (typed enum, serde-renamed to a string in JSON for backward-compatible output). Variants cover existing kinds (`OrphanedModule`, `TomlParseError`, `MissingCrateRoots`, `ModuleTreeError`) plus new (`OrphanFile`, `DeadReExport`).
- Add `SymbolEntry` (slim — `crate_name`, `module`, `file`, `line`, `kind: ItemKind`, no `confidence`, no `imported_by`/`re_exported_at` until a workflow cites them).
- Add `FileEntry` (slim — `module_path`, `parent_module_file`, `is_crate_root`).
- Add three `BTreeMap` fields on `WorkspaceMap`: `symbols`, `name_index`, `files`. Built in a single derivation pass after `cross_refs::compute`.
- Do **not** add: `Confidence`, `Warning`/`WarningKind` (use `ErrorEntry`), `ImportSite`/`ReExportSite`, parallel `warnings: Vec<Warning>`.

**CLI delta** (`src/main.rs`):
- Subcommands: `index` (current behavior), `validate <PATH>`, `lookup <PATH> --symbol|--file`. No `--from-stdin`.
- Bare-path form removed; `tests/integration_test.rs` rewrites in lockstep.
- Exit codes: `0` clean, `1` tool error, `2` validation findings.

**`validate` rules** (new file `src/validate.rs`, pure function over `&WorkspaceMap` returning `Vec<ErrorEntry>` to merge into the existing `errors` field):
- `OrphanFile` — fs walk of each crate's `src/` minus the set of files reachable through `module_tree::resolve_module_path`. Reuses the existing resolver.
- `DeadReExport` — for every `pub use a::b::C` across all modules, check whether any module under `a::*` has a `public_item` named `C`.
- **Not** `UnreachablePub` (defer until measured #3 cases attributable to it).
- **Not** `UnsupportedLayout` as a rule — document the limitation in `README.md`.

**Index derivation** (new file `src/indexes.rs`, pure function):
- `derive_indexes(&WorkspaceMap) -> (symbols, name_index, files)` in one pass over `crates[].modules[]`.

**Pipeline integration** (one only):
- `skills/compile-plan/SKILL.md` adds a pre-check step: run `validate`, block on any `OrphanFile` / `DeadReExport` finding.
- Plan-decomposer integration is a *soft suggestion* — "when introducing a new file, consider `lookup --file <parent>` to learn the intended siblings." No hard mandate.

**Out of MVP** (additive, defer until measured):
- `Confidence` enum → defer until a non-AST data source is introduced.
- Per-symbol `imported_by` / `re_exported_at` at file:line → defer until a measured workflow cites "find references."
- `UnreachablePub` rule → defer until measured #3 cases appear.
- `UnsupportedLayout` warning → README sentence.
- `--from-stdin` → defer until a piping consumer exists; carries hidden `Deserialize` cost.
- Plan-decomposer hard-call-`lookup` mandate → soft suggestion only.
- File-mtime cache → defer per the plan's existing methodology.
- Everything else from the plan's own "Out of MVP scope" stays out.

**Coverage projection:** ~60% of measured failure rate (#1 + #2 caught deterministically). Same as the bigger plan's hard-block coverage; the bigger plan's `UnreachablePub` adds at most a few percent on a partial-#3 axis with high false-negative tolerance.

**Effective trim vs the plan as written:** four schema types removed (`Confidence`, `Warning`, `ImportSite`, `ReExportSite`), one rule removed (`UnreachablePub`), one rule downgraded to documentation (`UnsupportedLayout`), one CLI flag removed (`--from-stdin`), one mandate softened (plan-decomposer), one channel collapsed (warnings → `errors` with typed kind). Indexes kept; `lookup` kept; subcommand split kept; breaking change kept.

## Critical files referenced

- `/Users/tony/programming/rust-workspace-map/src/schema.rs` — the existing `WorkspaceMap`, `ErrorEntry`, `ErrorSeverity`, `CrossReferences`, `Import`, `ReExport`. The plan's additive changes mostly belong here.
- `/Users/tony/programming/rust-workspace-map/src/module_tree.rs` — `resolve_module_path` and existing orphan detection. `OrphanFile` rule reuses both.
- `/Users/tony/programming/rust-workspace-map/src/cross_refs.rs` — already does name-keyed `imported_by`/`exported_by`. The plan's `name_index` is largely a rename.
- `/Users/tony/programming/rust-workspace-map/src/main.rs` — the bare-path CLI; subcommand split lands here.
- `/Users/tony/programming/rust-workspace-map/src/lib.rs` — the orchestrator; would call into `validate::run` after `cross_refs::compute`.
- `/Users/tony/programming/rust-workspace-map/Cargo.toml` — `clap`, `bon`, `syn`, `rayon`, `serde` already present. No new crates needed for the KISS-cut MVP.
- `/Users/tony/programming/rust-workspace-map/tests/integration_test.rs` — every existing invocation rewrites to `index <path>`.
- `/Users/tony/programming/rust-workspace-map/tests/fixtures/sample-workspace/` — add `bad-orphan/` and `bad-dead-reexport/`.
- `/Users/tony/programming/rust-development-pipeline/skills/compile-plan/SKILL.md` — single integration; pre-check step.

## Verification (for the KISS-cut MVP)

1. `cargo build --release` clean.
2. All existing integration tests pass after subcommand rewrite (`<path>` → `index <path>`).
3. `bad-orphan/` fixture → `validate` exits 2, warning names parent file.
4. `bad-dead-reexport/` fixture → `validate` exits 2, warning names the dead path.
5. `validate /Users/tony/programming/castep-cell-io` runs to completion. Wall-time recorded as cache baseline.
6. `compile-plan` against a synthetic plan that creates a file without `pub mod` blocks. Same for a synthetic `pub use` of a nonexistent symbol.

That's a real MVP. Ship it, run it through `castep-cell-io`, watch the fix-task ratio in `notes/pr-reviews/` for ≥3 phases. Then revisit `lookup`, `UnreachablePub`, flat indexes — whichever the *measured* gap demands, in priority order, one at a time.
## File: plans/mvp/MVP_PLAN_review2.md
# Second Review: MVP Plan — Structural Critique

## Context

The first review (`MVP_PLAN_review.md`) correctly trimmed scope: it killed `Confidence`, `UnreachablePub`, `--from-stdin`, the parallel `Vec<Warning>` channel, and the plan-decomposer hard mandate. All correct cuts.

This second review scrutinizes the **algorithms, data model integrity, and implementation sequencing** that the first review didn't cover. Grounded in the actual codebase at `/Users/tony/programming/rust-workspace-map/src/`.

---

## User walkthrough decisions

| # | Topic | Decision |
|---|---|---|
| 2 | `DeadReExport` false-positive risk | **Specify 5-case algorithm** before implementation |
| 7 | `validate` doubles parse cost | **Merge as `index --validate`** flag |
| 1 & 3 | `cross_references.types` collision + `files` inline-module | **Fix both in MVP** |
| 4 | `OrphanFile` algorithm underspecified | **Option C**: diff `ModuleInfo.file` set vs fs walk |
| 5 | `indexes::derive` sequencing | **Option B**: `derive_from_crates(&[CrateInfo])` before builder |
| 6 | "O(1) lookup" claim | **Resolved** — O(1) means one agent call, not algorithmic complexity |
| 8 | `lookup` parse-per-invocation cost | **Document as human tool only** until cache ships |
| 9 | `symbols` key format for root items | **Specify** `crate::Name` (2 segments) vs `crate::mod::Name` (3+) |
| 10 | `WorkspaceInfo.root` hardcoded to `"."` | **Fix**: set to actual workspace root |
| 11 | `module_tree.rs` existing bugs | **Fix** during DiagnosticKind refactor |

---

## Finding 2 (DECIDED: Specify 5-case algorithm)

**Severity: False positives — would block valid plans**

The plan originally said: *"for every `pub use a::b::C`, check whether any module under `a::*` has a `public_item` named `C`."*

This only works for intra-crate re-exports. Real code has five cases:

| Re-export form | Naive check result |
|---|---|
| `pub use crate::inner::Thing;` | Works (after prefix stripping) |
| `pub use super::thing;` | Fails — `super` is relative |
| `pub use self::detail::Type;` | Fails — `self` needs resolution |
| `pub use serde::Serialize;` | **False positive** — external dep, not in workspace |
| `pub use some::path::*;` | Cannot evaluate — glob |

Case 4 is critical: `pub use serde::Serialize;` is valid but the naive algorithm won't find `Serialize` in workspace modules → emits `DeadReExport` warning → `compile-plan` pre-check **blocks a valid plan**.

**Decision:** The algorithm now specified in `MVP_PLAN.md` must:
1. Resolve `crate::`, `self::`, `super::` prefixes
2. Skip re-exports where the first segment is NOT a workspace member (external crate)
3. Skip glob re-exports (conservative — false negatives preferred per policy)
4. Document that transitive re-export chains are not traced in MVP

**Amendment applied:** Yes — §"validate rules" table updated.

---

## Finding 7 (DECIDED: Merge as `index --validate`)

**Severity: Efficiency**

`index` and `validate` both do full AST parses. Sequential invocations double the cost for zero new information. The `validate` output is a superset of `index` output (same JSON + extra ErrorEntries).

**Decision:** `index` gets a `--validate` flag. Single invocation:
- `rust-workspace-map index <path>` — current behavior (no validation)
- `rust-workspace-map index <path> --validate` — runs OrphanFile + DeadReExport, findings appear in `errors`

**Amendments applied:**
- CLI section: removed `validate` subcommand, added `[--validate]` flag to `index`
- Files table: removed separate `src/validate.rs` row (merged into `index --validate` flow)
- Pipeline integrations: `compile-plan` pre-check calls `index --validate`
- Verification steps: all `validate` references → `index --validate`
- Locked decisions D2: updated

---

## Findings 1 & 3 (DECIDED: Fix both in MVP)

### 1. `cross_references.types` short-name collision corrupts data

`cross_refs.rs:30-36`: `TypeRef` is keyed by short name only. When two crates define different `Config` types, the first one processed wins — the second's `kind`, `crate_name` are silently lost (only `exported_by` gets appended). Any consumer trusting `TypeRef.kind` gets wrong data.

Names like `Config`, `Error`, `Result` appear across crates routinely — this is not an edge case.

**Fix:** Key `cross_references.types` by fully-qualified path (`crate::module::Name`).

### 3. `files` index silently drops inline modules

One `.rs` file can host multiple modules (`mod tests { ... }` in `lib.rs`). `files: BTreeMap<String, FileEntry>` stores only the last-inserted entry per file path — other modules silently dropped. Non-deterministic which wins.

When `plan-decomposer` calls `lookup --file core/src/lib.rs`, it gets whichever module happened to be processed last.

**Fix:** Exclude inline modules from `files` index (they have no independent file).

**Amendments applied:**
- Schema section: added comment documenting `cross_references.types` re-keying
- Schema section: added inline-module exclusion note to `FileEntry`
- Locked decisions D2: added "re-key `cross_references.types` from short-name to fully-qualified path"

---

## Finding 4 (DECIDED: Option C — diff ModuleInfo.file vs fs walk)

**Severity: Under-specification**

The plan said: *"walk every `.rs` under the crate source dir, check whether the resolver included it."* This is not specific enough to implement.

Three approaches exist:

| Approach | Correctness | Cost |
|---|---|---|
| **A:** Collect visited file set from `build_module_tree` return | Correct — uses actual parsed tree | Requires changing `build_module_tree`'s return signature |
| **B:** Use `resolve_module_path` in reverse for each `.rs` file | Partial — proves file *could* be included, not that it *was* | No refactoring; misses deeply nested orphans |
| **C:** Deduplicate `ModuleInfo.file` paths from already-returned modules | Correct — uses finalized module tree | No refactoring needed; path normalization required |

**Decision: Option C (simplest correct approach):**
1. After `build_module_tree` returns for a crate, collect all `ModuleInfo.file` values (excluding `<unresolved>`)
2. Walk the crate's `src/` directory for all `*.rs` files
3. Any `.rs` on disk absent from the file set = orphan → `OrphanFile` warning
4. Exclude known non-module files: `build.rs`, `src/bin/*.rs`, test/example directories

No refactoring of `build_module_tree`'s return type needed — the file set is already in the returned `Vec<ModuleInfo>`.

**Amendment applied:** Yes — §"validate rules" table updated for `OrphanFile`.

---

## Finding 5 (DECIDED: Option B — derive_from_crates before builder)

**Severity: Implementation friction**

The plan said `indexes::derive(&WorkspaceMap)` but the map is built via builder at the end of `run()`. The indexes need to go INTO the map.

**Options:**
- **Option A:** Build map first, mutate afterward — works mechanically (all fields are `pub`) but awkward
- **Option B (cleaner):** `derive_from_crates(&[CrateInfo])` before builder, pass results directly to builder

**Decision: Option B.** Matches how `cross_refs::compute` works today (takes `&[CrateInfo]` before the map exists).

**Amendments applied:**
- Files table: updated `src/indexes.rs` and `src/lib.rs` rows with new signature

---

## Finding 6 (RESOLVED — user clarification)

**Original critique:** "O(1) lookup" for `lookup --file` is O(1)+O(n) internally (hashmap hit + linear ModuleInfo scan).

**User clarification:** The O(1) stands for "the agent needs one call to get the correct, desired answer for its query." It's about **agent interaction cost** (one tool call → one answer), not internal algorithmic complexity. The internal O(n) scan is irrelevant to the agent interaction model.

**Resolution:** Design rationale is correct under this framing. Withdrawn — no plan changes needed.

---

## Finding 8 (DECIDED: Accept — document as human tool)

**Severity: Efficiency**

`lookup` pays full AST parse per invocation. If the plan-decomposer calls `lookup` 5 times, that's 5x parse cost for 5 hashmap lookups. Without a cache, this wastes time for pipeline use.

**Decision:** For the MVP, document `lookup` as a **human CLI convenience**. Pipeline agents should embed flat indexes in their context window (single JSON load) rather than making per-symbol CLI calls. Revisit `lookup` as a pipeline tool only after the cache layer ships.

**Amendment applied:** §"lookup semantics" — added note about human-convenience scope.

---

## Finding 9 (DECIDED: Specify key format)

**Severity: Under-specification**

The plan says `symbols` keyed by `"crate::module::Name"` but root-module items (in `lib.rs`) have no module segment.

**Decision:**
- Root-module items: `"crate::Name"` (2 segments)
- Nested-module items: `"crate::module::Name"` (3+ segments)
- Parsers split on `::`: first segment = crate name, last = item name, middle = module path

**Amendment applied:** §"Schema additions" — added key-format comment to `SymbolEntry`.

---

## Finding 10 (DECIDED: Fix)

**Severity: Latent bug**

`lib.rs:149-152`: `WorkspaceInfo.root` is always `"."`. With subcommands, input `PATH` can be a subdirectory (since `find_workspace_root` walks up), but file paths are relative to the **actual** workspace root. If a consumer tries to resolve file paths using `root` as base, they get wrong paths.

**Decision:** Set `root` to the discovered workspace root. One-line fix in `lib.rs` in the WorkspaceInfo builder call.

**Amendment applied:** Added to `src/lib.rs` row in files table.

---

## Finding 11 (DECIDED: Fix during refactor)

**Severity: Existing bugs**

- **Bug A** (`module_tree.rs:252`): `errors.clone()` at every recursion level duplicates errors O(n²) in memory. For a 4-level tree with errors at each level, waste compounds. `OrphanFile` will add more errors, making this worse.
- **Bug B** (`module_tree.rs:33`): `crate_root.parent().unwrap_or(crate_root)` — if `crate_root` somehow has no parent, `parent_dir` falls back to root itself, which is semantically wrong for submodule resolution.

**Decision:** Fix both during the planned `DiagnosticKind` mechanical refactor. The code is already being touched.

**Amendment applied:** `src/module_tree.rs` row in files table now includes "Fix error-vec cloning at line 252 and fragile `unwrap_or` at line 33."

---

## Summary of amendments applied to MVP_PLAN.md

| Amendment | Section | Change |
|---|---|---|
| A: `index --validate` | §CLI, §Files, §Pipeline, §Verification | Removed `validate` subcommand; merged as `index --validate` flag |
| B: 5-case DeadReExport | §Validate rules | Algorithm now handles 5 distinct re-export forms |
| C: OrphanFile Option C | §Validate rules | Algorithm now specified as ModuleInfo.file set diff vs fs walk |
| D: cross_references fix | §Schema additions | Documented re-key to fully-qualified path (collision fix) |
| D: FileEntry inline-module | §Schema additions | Added inline-module exclusion note |
| E: derive_from_crates | §Files (indexes.rs, lib.rs) | Changed signature from `derive(&WorkspaceMap)` to `derive_from_crates(&[CrateInfo])` |
| F: lookup as human tool | §Lookup semantics | Added paragraph about human-convenience scope |
| G: symbols key format | §Schema additions | Specified `crate::Name` (root) vs `crate::mod::Name` (nested) |
| H: WorkspaceInfo.root fix | §Files (lib.rs) | Added fix note for hardcoded `"."` root |
| I: module_tree.rs bugs | §Files (module_tree.rs) | Added error-cloning and `unwrap_or` fixes |
| J: Verification & D2 | §Verification, §Locked decisions | Updated all `validate` refs → `index --validate` |

**Outstanding items for future review rounds (from this review):**
- `cross_references.types` re-keying: implementation decision needed (fully-qualified key vs `Vec<TypeRef>` per short name)
- `files` inline-module handling: implementation decision needed (exclusion vs `Vec<FileEntry>`)
- `lookup` cache: revisit when a measured pipeline use case demands it
## File: src/cross_refs.rs
use crate::schema::{CrateInfo, CrossCrateImport, CrossReferences, TypeRef};
use std::collections::BTreeMap;

/// Compute cross-crate type references.
///
/// For every public item in every crate, matches it against imports from
/// other crates. Populates each `CrateInfo.cross_crate_imports` and returns
/// the global `CrossReferences` map (keyed by canonical path).
///
/// Takes `&mut [CrateInfo]` so it can write `cross_crate_imports` into each
/// crate while building the global cross-reference map.
pub fn compute(crates: &mut [CrateInfo]) -> CrossReferences {
    // Build a map: crate_name -> set of (canonical_path, kind) pairs.
    // canonical_path = "{module.path}::{item.name}" (e.g., "core::Task").
    let crate_exports: BTreeMap<String, Vec<(String, String)>> = crates
        .iter()
        .map(|c| {
            let items: Vec<(String, String)> = c
                .modules
                .iter()
                .flat_map(|m| {
                    m.public_items
                        .iter()
                        .map(move |item| (format!("{}::{}", m.path, item.name), item.kind_to_string()))
                })
                .collect();
            (c.name.clone(), items)
        })
        .collect();

    let mut types_map: BTreeMap<String, TypeRef> = BTreeMap::new();

    // Initialize TypeRef entries for every exported public item.
    for (crate_name, items) in &crate_exports {
        for (canonical_path, kind) in items {
            let entry = types_map.entry(canonical_path.clone()).or_insert_with(|| {
                TypeRef::builder()
                    .crate_name(crate_name.clone())
                    .kind(kind.clone())
                    .build()
            });
            if !entry.exported_by.contains(crate_name) {
                entry.exported_by.push(crate_name.clone());
            }
        }
    }

    // Build a reverse lookup: short_name -> Vec<canonical_path> for import matching.
    let mut reverse_lookup: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for canonical_path in types_map.keys() {
        if let Some(pos) = canonical_path.rfind("::") {
            let short_name = &canonical_path[pos + 2..];
            reverse_lookup.entry(short_name.to_string()).or_default().push(canonical_path.clone());
        }
    }

    // Scan each crate's imports to find cross-crate references.
    for crate_info in crates.iter_mut() {
        let my_name = crate_info.name.clone();
        let mut cross_imports: Vec<CrossCrateImport> = Vec::new();

        for module in &crate_info.modules {
            for import in &module.imports {
                // Extract first path segment as potential crate name.
                let first_seg = import
                    .path
                    .split("::")
                    .next()
                    .unwrap_or("")
                    .to_string();
                if first_seg.is_empty() || first_seg == "*" {
                    continue;
                }

                // Check if first segment matches any known crate.
                if crate_exports.contains_key(&first_seg) && first_seg != my_name {
                    let symbol = import
                        .path
                        .rsplit("::")
                        .next()
                        .unwrap_or("")
                        .to_string();

                    cross_imports.push(CrossCrateImport {
                        import_path: import.path.clone(),
                        target_crate: first_seg.clone(),
                        symbol: symbol.clone(),
                        line: import.line,
                    });

                    // Update global cross-references via reverse lookup.
                    if let Some(candidates) = reverse_lookup.get(&symbol) {
                        for canonical_path in candidates {
                            // Only match entries from the target crate.
                            if let Some(type_ref) = types_map.get_mut(canonical_path)
                                && type_ref.crate_name == first_seg
                            {
                                let importer_label = format!("{}:{}", my_name, module.path);
                                if !type_ref.imported_by.contains(&importer_label) {
                                    type_ref.imported_by.push(importer_label.clone());
                                }
                            }
                        }
                    }
                }
            }
        }

        cross_imports.sort_by(|a, b| a.import_path.cmp(&b.import_path));
        crate_info.cross_crate_imports = cross_imports;
    }

    CrossReferences { types: types_map }
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{CrateType, Import, ItemKind, ModuleInfo, PackageInfo, PublicItem};

    fn make_crate(name: &str, items: Vec<(String, ItemKind)>) -> CrateInfo {
        let public_items: Vec<PublicItem> = items
            .into_iter()
            .map(|(n, k)| {
                PublicItem::builder()
                    .kind(k)
                    .name(n)
                    .file(String::new())
                    .line(1)
                    .visibility("pub".to_string())
                    .generics(String::new())
                    .attrs(Default::default())
                    .build()
            })
            .collect();
        let module = ModuleInfo::builder()
            .path("".to_string())
            .file(String::new())
            .visibility("pub".to_string())
            .public_items(public_items)
            .build();
        CrateInfo::builder()
            .name(name.to_string())
            .root(String::new())
            .package(
                PackageInfo::builder()
                    .name(name.to_string())
                    .version("0.1.0".to_string())
                    .edition("2021".to_string())
                    .crate_type(CrateType::Lib)
                    .build(),
            )
            .modules(vec![module])
            .deps(Default::default())
            .build()
    }

    #[test]
    fn compute_finds_cross_crate_import() {
        let mut crates = vec![
            make_crate("core", vec![
                ("Task".to_string(), ItemKind::Struct),
            ]),
            make_crate("engine", vec![]),
        ];
        // Manually add an import in engine that references core::Task
        let engine_module = &mut crates[1].modules[0];
        engine_module.imports.push(Import {
            path: "core::Task".to_string(),
            line: 1,
        });
        let refs = compute(&mut crates);
        // Types map keys are now canonical paths (module.path::item.name).
        // With empty module path, key is "::Task".
        let task_key = "::Task".to_string();
        assert!(refs.types.contains_key(&task_key), "expected key {}", task_key);
        let task_ref = &refs.types[&task_key];
        assert_eq!(task_ref.crate_name, "core");
        // engine should have a cross_crate_import
        assert_eq!(crates[1].cross_crate_imports.len(), 1);
        assert_eq!(crates[1].cross_crate_imports[0].target_crate, "core");
    }

    #[test]
    fn compute_empty_for_no_cross_references() {
        let crates = vec![
            make_crate("a", vec![("Foo".to_string(), ItemKind::Struct)]),
            make_crate("b", vec![("Bar".to_string(), ItemKind::Struct)]),
        ];
        let mut crates_mut = crates;
        let refs = compute(&mut crates_mut);
        // No cross references since no crate imports from another
        assert!(refs.types.is_empty() || refs.types.values().all(|t| t.imported_by.is_empty()));
    }
}

// Helper: convert ItemKind to a short string for the TypeRef.kind field.
impl crate::schema::PublicItem {
    #[must_use]
    fn kind_to_string(&self) -> String {
        match self.kind {
            crate::schema::ItemKind::Struct => "struct".to_string(),
            crate::schema::ItemKind::Enum => "enum".to_string(),
            crate::schema::ItemKind::Trait => "trait".to_string(),
            crate::schema::ItemKind::Fn => "fn".to_string(),
            crate::schema::ItemKind::Type => "type".to_string(),
            crate::schema::ItemKind::Macro => "macro".to_string(),
        }
    }
}
## File: src/file_parser.rs
use crate::schema::{
    DiagnosticKind, ErrorEntry, ErrorSeverity, FileInfo, ImplInfo, ImplItem, ImplItemKind, Import,
    ItemAttrs, ItemKind, PublicItem, ReExport, SubmoduleDecl,
};
use std::path::Path;

// ── Internal parse result types ────────────────────────────────────────

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

/// Structured information about a parse failure.
pub struct SynParseError {
    pub message: String,
    pub line: usize,
}

// ── parse_file ──────────────────────────────────────────────────────────

/// Read and parse a Rust source file.
///
/// On parse failure, returns the original file content and a
/// `SynParseError` alongside an empty `FileInfo`. Callers use the
/// error to construct an `ErrorEntry`.
#[must_use]
pub fn parse_file(path: &Path) -> ParsedFile {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(source) => {
            let err = SynParseError {
                message: source.to_string(),
                line: 0,
            };
            return ParsedFile {
                ast: syn::File {
                    shebang: None,
                    attrs: vec![],
                    items: vec![],
                },
                file_info: FileInfo::default(),
                parse_error: Some(err),
            };
        }
    };

    match syn::parse_file(&content) {
        Ok(file) => {
            let file_info = FileInfo {
                public_items: extract_public_items(&file.items),
                imports: extract_imports(&file.items),
                re_exports: extract_re_exports(&file.items),
                submodules: extract_submodules(&file.items),
                impls: extract_impls(&file.items),
            };
            ParsedFile {
                ast: file,
                file_info,
                parse_error: None,
            }
        },
        Err(e) => {
            let line = e.span().start().line;
            let err = SynParseError {
                message: e.to_string(),
                line,
            };
            ParsedFile {
                ast: syn::File {
                    shebang: None,
                    attrs: vec![],
                    items: vec![],
                },
                file_info: FileInfo::default(),
                parse_error: Some(err),
            }
        }
    }
}

pub(crate) fn build_parse_error_entry(path: &Path, err: &SynParseError) -> ErrorEntry {
    ErrorEntry::builder()
        .file(path.to_string_lossy().to_string())
        .line(err.line)
        .message(err.message.clone())
        .severity(ErrorSeverity::Error)
        .kind(DiagnosticKind::SynParseError)
        .build()
}

// ── extract_public_items ────────────────────────────────────────────────

/// Extract all items with any form of `pub` visibility (excluding `Inherited`).
/// Results are sorted by name then line for deterministic output.
#[must_use]
pub fn extract_public_items(items: &[syn::Item]) -> Vec<PublicItem> {
    let mut result: Vec<PublicItem> = items
        .iter()
        .filter(|item| !is_visibility_inherited(item))
        .filter_map(into_public_item)
        .collect();
    result.sort_by(|a, b| a.name.cmp(&b.name).then(a.line.cmp(&b.line)));
    result
}

// ── extract_imports ─────────────────────────────────────────────────────

/// Extract all `use` statements. Braced imports are expanded to individual
/// entries. Results sorted by path for determinism.
#[must_use]
pub fn extract_imports(items: &[syn::Item]) -> Vec<Import> {
    let mut result: Vec<Import> = items
        .iter()
        .filter_map(|item| {
            if let syn::Item::Use(u) = item {
                Some(flatten_use_tree(&u.tree, "", line_of_item(item)))
            } else {
                None
            }
        })
        .flatten()
        .collect();
    result.sort_by(|a, b| a.path.cmp(&b.path));
    result
}

// ── extract_re_exports ──────────────────────────────────────────────────

/// Extract `pub use` re-exports. Results sorted by `export_path`.
#[must_use]
pub fn extract_re_exports(items: &[syn::Item]) -> Vec<ReExport> {
    let mut result: Vec<ReExport> = items
        .iter()
        .filter_map(|item| {
            if let syn::Item::Use(u) = item {
                if matches!(u.vis, syn::Visibility::Public(_)) {
                    Some(extract_re_exports_from_tree(
                        &u.tree,
                        String::new(),
                        line_of_item(item),
                    ))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .flatten()
        .collect();
    result.sort_by(|a, b| a.export_path.cmp(&b.export_path));
    result
}

// ── extract_submodules ──────────────────────────────────────────────────

/// Extract `mod` declarations. Detects `#[cfg(test)]` via literal token
/// matching. Results sorted by name.
#[must_use]
pub fn extract_submodules(items: &[syn::Item]) -> Vec<SubmoduleDecl> {
    let mut result: Vec<SubmoduleDecl> = items
        .iter()
        .filter_map(|item| {
            if let syn::Item::Mod(m) = item {
                let is_test = m.attrs.iter().any(|attr| {
                    if !attr.path().is_ident("cfg") {
                        return false;
                    }
                    if let syn::Meta::List(list) = &attr.meta {
                        let tokens = list.tokens.to_string();
                        tokens.trim() == "test"
                    } else {
                        false
                    }
                });
                Some(SubmoduleDecl {
                    name: m.ident.to_string(),
                    is_test,
                })
            } else {
                None
            }
        })
        .collect();
    result.sort_by(|a, b| a.name.cmp(&b.name));
    result
}

// ── extract_impls ───────────────────────────────────────────────────────

/// Extract `impl` blocks. Each `ImplInfo` records the target type name and
/// the impl items (fn, type, const).
#[must_use]
pub fn extract_impls(items: &[syn::Item]) -> Vec<ImplInfo> {
    items
        .iter()
        .filter_map(|item| {
            if let syn::Item::Impl(imp) = item {
                let type_name = match imp.self_ty.as_ref() {
                    syn::Type::Path(tp) => tp
                        .path
                        .segments
                        .last()
                        .map(|s| s.ident.to_string())
                        .unwrap_or_default(),
                    _ => String::new(),
                };
                if type_name.is_empty() {
                    return None;
                }
                let impl_items: Vec<ImplItem> = imp
                    .items
                    .iter()
                    .filter_map(|ii| match ii {
                        syn::ImplItem::Fn(f) => Some(ImplItem {
                            kind: ImplItemKind::Fn,
                            name: f.sig.ident.to_string(),
                            params: generics_to_string(&f.sig.generics),
                        }),
                        syn::ImplItem::Type(t) => Some(ImplItem {
                            kind: ImplItemKind::Type,
                            name: t.ident.to_string(),
                            params: String::new(),
                        }),
                        syn::ImplItem::Const(c) => Some(ImplItem {
                            kind: ImplItemKind::Const,
                            name: c.ident.to_string(),
                            params: String::new(),
                        }),
                        _ => None,
                    })
                    .collect();
                Some(ImplInfo {
                    type_: type_name,
                    items: impl_items,
                })
            } else {
                None
            }
        })
        .collect()
}

// ── Helpers ─────────────────────────────────────────────────────────────

fn is_visibility_inherited(item: &syn::Item) -> bool {
    matches!(item_vis(item), syn::Visibility::Inherited)
}

fn item_vis(item: &syn::Item) -> &syn::Visibility {
    match item {
        syn::Item::Const(i) => &i.vis,
        syn::Item::Enum(i) => &i.vis,
        syn::Item::ExternCrate(i) => &i.vis,
        syn::Item::Fn(i) => &i.vis,
        syn::Item::Mod(i) => &i.vis,
        syn::Item::Static(i) => &i.vis,
        syn::Item::Struct(i) => &i.vis,
        syn::Item::Trait(i) => &i.vis,
        syn::Item::TraitAlias(i) => &i.vis,
        syn::Item::Type(i) => &i.vis,
        syn::Item::Union(i) => &i.vis,
        syn::Item::Use(i) => &i.vis,
        _ => &syn::Visibility::Inherited,
    }
}

fn line_of_item(item: &syn::Item) -> usize {
    item_ident_span(item).unwrap_or(0)
}

fn item_ident_span(item: &syn::Item) -> Option<usize> {
    match item {
        syn::Item::Struct(s) => Some(s.ident.span().start().line),
        syn::Item::Enum(e) => Some(e.ident.span().start().line),
        syn::Item::Trait(t) => Some(t.ident.span().start().line),
        syn::Item::Fn(f) => Some(f.sig.ident.span().start().line),
        syn::Item::Type(t) => Some(t.ident.span().start().line),
        syn::Item::Mod(m) => Some(m.ident.span().start().line),
        syn::Item::Macro(m) => m.ident.as_ref().map(|i| i.span().start().line),
        _ => None,
    }
}

fn vis_to_string(vis: &syn::Visibility) -> String {
    match vis {
        syn::Visibility::Public(_) => "pub".to_string(),
        syn::Visibility::Restricted(r) => {
            let path = r
                .path
                .segments
                .iter()
                .map(|s| s.ident.to_string())
                .collect::<Vec<_>>()
                .join("::");
            if path.is_empty() {
                "pub(restricted)".to_string()
            } else {
                format!("pub({path})")
            }
        }
        syn::Visibility::Inherited => "private".to_string(),
    }
}

fn generics_to_string(generics: &syn::Generics) -> String {
    if generics.params.is_empty() {
        return String::new();
    }
    let params: Vec<String> = generics
        .params
        .iter()
        .map(|p| match p {
            syn::GenericParam::Type(t) => t.ident.to_string(),
            syn::GenericParam::Lifetime(l) => l.lifetime.ident.to_string(),
            syn::GenericParam::Const(c) => c.ident.to_string(),
        })
        .collect();
    params.join(", ")
}

fn type_to_string(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(tp) => {
            tp.path
                .segments
                .iter()
                .map(|s| {
                    let ident = s.ident.to_string();
                    match &s.arguments {
                        syn::PathArguments::AngleBracketed(args) => {
                            let inner: Vec<String> = args
                                .args
                                .iter()
                                .filter_map(|a| match a {
                                    syn::GenericArgument::Type(t) => Some(type_to_string(t)),
                                    syn::GenericArgument::Lifetime(l) => {
                                        Some(l.ident.to_string())
                                    }
                                    _ => None,
                                })
                                .collect();
                            format!("{}<{}>", ident, inner.join(", "))
                        }
                        _ => ident,
                    }
                })
                .collect::<Vec<_>>()
                .join("::")
        }
        syn::Type::Reference(tr) => {
            let mut s = String::from("&");
            if tr.lifetime.is_some() {
                s.push_str("'a ");
            }
            s.push_str(&type_to_string(&tr.elem));
            s
        }
        syn::Type::Tuple(tt) => {
            let inner: Vec<String> = tt.elems.iter().map(type_to_string).collect();
            format!("({})", inner.join(", "))
        }
        syn::Type::Slice(ts) => format!("[{}]", type_to_string(&ts.elem)),
        syn::Type::Array(ta) => format!("[{}; _]", type_to_string(&ta.elem)),
        syn::Type::Ptr(tp) => {
            let kw = if tp.const_token.is_some() {
                "const"
            } else {
                "mut"
            };
            format!("*{} {}", kw, type_to_string(&tp.elem))
        }
        syn::Type::BareFn(_) => "fn(...)".to_string(),
        syn::Type::Never(_) => "!".to_string(),
        syn::Type::TraitObject(to) => {
            let bounds: Vec<String> = to.bounds.iter().map(quote_bound).collect();
            bounds.join(" + ")
        }
        syn::Type::ImplTrait(ti) => {
            let bounds: Vec<String> = ti.bounds.iter().map(quote_bound).collect();
            format!("impl {}", bounds.join(" + "))
        }
        syn::Type::Paren(tp) => format!("({})", type_to_string(&tp.elem)),
        syn::Type::Group(tg) => type_to_string(&tg.elem),
        _ => "?".to_string(),
    }
}

fn quote_bound(bound: &syn::TypeParamBound) -> String {
    match bound {
        syn::TypeParamBound::Trait(tb) => tb
            .path
            .segments
            .last()
            .map(|s| s.ident.to_string())
            .unwrap_or_default(),
        syn::TypeParamBound::Lifetime(l) => l.ident.to_string(),
        _ => "?".to_string(),
    }
}

fn fields_to_strings(fields: &syn::Fields) -> Vec<String> {
    fields
        .iter()
        .map(|f| {
            let vis = match &f.vis {
                syn::Visibility::Public(_) => "pub ",
                _ => "",
            };
            match &f.ident {
                Some(name) => format!("{}{}: {}", vis, name, type_to_string(&f.ty)),
                None => format!("{}{}", vis, type_to_string(&f.ty)),
            }
        })
        .collect()
}

fn variants_to_strings(
    variants: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
) -> Vec<String> {
    variants
        .iter()
        .map(|v| {
            let name = v.ident.to_string();
            match &v.fields {
                syn::Fields::Named(fields) => {
                    let inner: Vec<String> = fields
                        .named
                        .iter()
                        .map(|f| match &f.ident {
                            Some(id) => format!("{}: {}", id, type_to_string(&f.ty)),
                            None => type_to_string(&f.ty),
                        })
                        .collect();
                    format!("{} {{ {} }}", name, inner.join(", "))
                }
                syn::Fields::Unnamed(fields) => {
                    let inner: Vec<String> =
                        fields.unnamed.iter().map(|f| type_to_string(&f.ty)).collect();
                    format!("{}({})", name, inner.join(", "))
                }
                syn::Fields::Unit => name,
            }
        })
        .collect()
}

fn extract_attrs(attrs: &[syn::Attribute]) -> ItemAttrs {
    let mut derive = Vec::new();
    let mut doc = Vec::new();

    for attr in attrs {
        if attr.path().is_ident("derive") {
            if let syn::Meta::List(list) = &attr.meta {
                let derives: Vec<String> = list
                    .tokens
                    .to_string()
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                derive.extend(derives);
            }
        } else if attr.path().is_ident("doc")
            && let syn::Meta::NameValue(nv) = &attr.meta
            && let syn::Expr::Lit(el) = &nv.value
            && let syn::Lit::Str(ls) = &el.lit
        {
            doc.push(ls.value());
        }
    }

    derive.sort();
    ItemAttrs { derive, doc }
}

fn into_public_item(item: &syn::Item) -> Option<PublicItem> {
    let (kind, name, fields, variants, generics, attrs_src) = match item {
        syn::Item::Struct(s) => (
            ItemKind::Struct,
            s.ident.to_string(),
            fields_to_strings(&s.fields),
            vec![],
            generics_to_string(&s.generics),
            &s.attrs,
        ),
        syn::Item::Enum(e) => (
            ItemKind::Enum,
            e.ident.to_string(),
            vec![],
            variants_to_strings(&e.variants),
            generics_to_string(&e.generics),
            &e.attrs,
        ),
        syn::Item::Trait(t) => (
            ItemKind::Trait,
            t.ident.to_string(),
            vec![],
            vec![],
            generics_to_string(&t.generics),
            &t.attrs,
        ),
        syn::Item::Fn(f) => (
            ItemKind::Fn,
            f.sig.ident.to_string(),
            vec![],
            vec![],
            generics_to_string(&f.sig.generics),
            &f.attrs,
        ),
        syn::Item::Type(t) => (
            ItemKind::Type,
            t.ident.to_string(),
            vec![],
            vec![],
            generics_to_string(&t.generics),
            &t.attrs,
        ),
        syn::Item::Macro(m) => {
            let name = m.ident.as_ref().map(ToString::to_string).unwrap_or_default();
            if name.is_empty() {
                return None;
            }
            (
                ItemKind::Macro,
                name,
                vec![],
                vec![],
                String::new(),
                &m.attrs,
            )
        }
        _ => return None,
    };

    let line = item_ident_span(item).unwrap_or(0);
    let attrs = extract_attrs(attrs_src);
    let visibility = vis_to_string(item_vis(item));

    Some(PublicItem {
        kind,
        name,
        file: String::new(),
        line,
        attrs,
        generics,
        visibility,
        fields,
        variants,
        impls: vec![],
    })
}

fn flatten_use_tree(tree: &syn::UseTree, prefix: &str, line: usize) -> Vec<Import> {
    match tree {
        syn::UseTree::Path(p) => {
            let new_prefix = if prefix.is_empty() {
                p.ident.to_string()
            } else {
                format!("{}::{}", prefix, p.ident)
            };
            flatten_use_tree(&p.tree, &new_prefix, line)
        }
        syn::UseTree::Name(n) => {
            let path = if prefix.is_empty() {
                n.ident.to_string()
            } else {
                format!("{}::{}", prefix, n.ident)
            };
            vec![Import { path, line }]
        }
        syn::UseTree::Rename(r) => {
            let path = if prefix.is_empty() {
                format!("{} as {}", r.ident, r.rename)
            } else {
                format!("{}::{} as {}", prefix, r.ident, r.rename)
            };
            vec![Import { path, line }]
        }
        syn::UseTree::Glob(_) => {
            let path = if prefix.is_empty() {
                "*".to_string()
            } else {
                format!("{prefix}::*")
            };
            vec![Import { path, line }]
        }
        syn::UseTree::Group(g) => g
            .items
            .iter()
            .flat_map(|t| flatten_use_tree(t, prefix, line))
            .collect(),
    }
}

fn extract_re_exports_from_tree(
    tree: &syn::UseTree,
    import_path: String,
    line: usize,
) -> Vec<ReExport> {
    match tree {
        syn::UseTree::Path(p) => {
            let new_import = if import_path.is_empty() {
                p.ident.to_string()
            } else {
                format!("{}::{}", import_path, p.ident)
            };
            extract_re_exports_from_tree(&p.tree, new_import, line)
        }
        syn::UseTree::Name(n) => {
            let full_path = if import_path.is_empty() {
                n.ident.to_string()
            } else {
                format!("{}::{}", import_path, n.ident)
            };
            vec![ReExport {
                import_path: full_path,
                export_path: n.ident.to_string(),
                line,
            }]
        }
        syn::UseTree::Rename(r) => {
            let full_path = if import_path.is_empty() {
                format!("{} as {}", r.ident, r.rename)
            } else {
                format!("{}::{} as {}", import_path, r.ident, r.rename)
            };
            vec![ReExport {
                import_path: full_path,
                export_path: r.rename.to_string(),
                line,
            }]
        }
        syn::UseTree::Glob(_) => {
            vec![ReExport {
                import_path,
                export_path: "*".to_string(),
                line,
            }]
        }
        syn::UseTree::Group(g) => g
            .items
            .iter()
            .flat_map(|t| extract_re_exports_from_tree(t, import_path.clone(), line))
            .collect(),
    }
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn parse_source(src: &str) -> ParsedFile {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let tmp = std::env::temp_dir().join(format!("parse_test_{id}.rs"));
        std::fs::write(&tmp, src).unwrap();
        let result = parse_file(&tmp);
        std::fs::remove_file(&tmp).ok();
        result
    }

    #[test]
    fn parse_file_returns_ast_for_valid_source() {
        let src = "pub struct Foo { x: i32 }";
        let result = parse_source(src);
        assert!(result.parse_error.is_none());
        assert_eq!(result.ast.items.len(), 1);
    }

    #[test]
    fn parse_file_returns_error_for_invalid_source() {
        let src = "pub struct { invalid rust }";
        let result = parse_source(src);
        assert!(result.parse_error.is_some());
        let err = result.parse_error.as_ref().unwrap();
        assert!(!err.message.is_empty());
        assert!(err.line > 0);
    }

    #[test]
    fn parse_file_returns_empty_for_empty_file() {
        let result = parse_source("");
        assert!(result.parse_error.is_none());
        assert!(result.file_info.public_items.is_empty());
    }

    #[test]
    fn extract_public_items_finds_struct_enum_trait_fn() {
        let src = "pub struct Foo {} pub enum Bar { A, B } pub trait Baz {} pub fn hello() {}";
        let result = parse_source(src);
        let items = extract_public_items(&result.ast.items);
        let names: Vec<_> = items.iter().map(|i| i.name.as_str()).collect();
        assert!(names.contains(&"Foo"));
        assert!(names.contains(&"Bar"));
        assert!(names.contains(&"Baz"));
        assert!(names.contains(&"hello"));
    }

    #[test]
    fn extract_public_items_empty_for_no_public_items() {
        let src = "struct Private {} fn private_fn() {}";
        let result = parse_source(src);
        let items = extract_public_items(&result.ast.items);
        assert!(items.is_empty());
    }

    #[test]
    fn extract_imports_finds_use_statements() {
        let src = "use std::collections::BTreeMap;";
        let result = parse_source(src);
        let imports = extract_imports(&result.ast.items);
        assert_eq!(imports.len(), 1);
        assert_eq!(imports[0].path, "std::collections::BTreeMap");
    }

    #[test]
    fn extract_re_exports_finds_pub_use() {
        let src = "pub use crate::foo;";
        let result = parse_source(src);
        let re_exports = extract_re_exports(&result.ast.items);
        assert_eq!(re_exports.len(), 1);
        assert_eq!(re_exports[0].import_path, "crate::foo");
        assert_eq!(re_exports[0].export_path, "foo");
    }

    #[test]
    fn extract_re_exports_finds_rename() {
        let src = "pub use crate::foo as bar;";
        let result = parse_source(src);
        let re_exports = extract_re_exports(&result.ast.items);
        assert_eq!(re_exports.len(), 1);
        assert_eq!(re_exports[0].import_path, "crate::foo as bar");
        assert_eq!(re_exports[0].export_path, "bar");
    }

    #[test]
    fn extract_submodules_finds_mod_declarations() {
        let src = "mod foo; mod bar;";
        let result = parse_source(src);
        let subs = extract_submodules(&result.ast.items);
        assert_eq!(subs.len(), 2);
        let names: Vec<_> = subs.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"bar"));
        assert!(names.contains(&"foo"));
    }

    #[test]
    fn extract_submodules_marks_cfg_test() {
        let src = "#[cfg(test)] mod inner;";
        let result = parse_source(src);
        let subs = extract_submodules(&result.ast.items);
        assert_eq!(subs.len(), 1);
        assert!(subs[0].is_test);
    }

    #[test]
    fn extract_impls_finds_fn_type_const() {
        let src = "impl MyType { pub fn foo(&self) {} pub type Alias = u32; pub const N: usize = 42; }";
        let result = parse_source(src);
        let impls = extract_impls(&result.ast.items);
        assert_eq!(impls.len(), 1);
        assert_eq!(impls[0].type_, "MyType");
        assert_eq!(impls[0].items.len(), 3);
    }

    #[test]
    fn build_parse_error_entry_constructs_error() {
        let path = PathBuf::from("test.rs");
        let err = SynParseError {
            message: "expected `;`".to_string(),
            line: 5,
        };
        let entry = build_parse_error_entry(&path, &err);
        assert_eq!(entry.file, "test.rs");
        assert_eq!(entry.line, 5);
        assert_eq!(entry.kind, crate::schema::DiagnosticKind::SynParseError);
        assert_eq!(entry.severity, ErrorSeverity::Error);
    }
}
## File: src/indexes.rs
use crate::schema::{
    CrateInfo, CanonicalPath, FileEntry, SymbolEntry, WorkspaceRelativePath,
};
use std::collections::BTreeMap;

type SymbolsIndex = BTreeMap<CanonicalPath, SymbolEntry>;
type NameIndex = BTreeMap<String, Vec<CanonicalPath>>;
type FilesIndex = BTreeMap<WorkspaceRelativePath, FileEntry>;

/// Build flat indexes from a slice of `CrateInfo`.
///
/// Returns three maps:
/// 1. `symbols` — canonical-path-keyed map of all public items
/// 2. `name_index` — short-name to canonical-path list (for disambiguation)
/// 3. `files` — workspace-relative-path-keyed map of all source files with module metadata
///
/// The symbols and `name_index` are constructed via an iterator pipeline;
/// the files map uses a for-loop due to the inline-detection accumulator.
///
/// # Inline module detection
///
/// This function relies on depth-first traversal order from `build_module_tree`.
/// The heuristic: the first module encountered per file path is the primary
/// (declared in a separate file); subsequent modules sharing the same file
/// are inline modules and are excluded from the files map.
#[allow(clippy::type_complexity)]
#[must_use]
pub fn derive_from_crates(
    crates: &[CrateInfo],
) -> (SymbolsIndex, NameIndex, FilesIndex) {
    // ── symbols & name_index via iterator pipeline ────────────────────

    let (syms, mut nidx): (
        BTreeMap<CanonicalPath, SymbolEntry>,
        BTreeMap<String, Vec<CanonicalPath>>,
    ) = crates
        .iter()
        .flat_map(|c| {
            c.modules
                .iter()
                .flat_map(move |m| {
                    m.public_items
                        .iter()
                        .map(move |item| (c.name.as_str(), m, item))
                })
        })
        .fold(
            (
                BTreeMap::<CanonicalPath, SymbolEntry>::new(),
                BTreeMap::<String, Vec<CanonicalPath>>::new(),
            ),
            |(mut syms, mut nidx), (crate_name, m, item)| {
                let canonical = CanonicalPath::from(format!("{}::{}", m.path, item.name));
                syms.insert(
                    canonical.clone(),
                    SymbolEntry::builder()
                        .crate_name(crate_name.to_string())
                        .module(m.path.clone())
                        .file(m.file.clone())
                        .line(item.line)
                        .kind(item.kind.clone())
                        .build(),
                );
                nidx.entry(item.name.clone()).or_default().push(canonical);
                (syms, nidx)
            },
        );

    // Sort name_index values for determinism.
    for val in nidx.values_mut() {
        val.sort();
    }

    // ── files via for-loop (inline detection requires accumulator) ───

    let mut files = BTreeMap::new();
    let mut seen_files: BTreeMap<String, String> = BTreeMap::new(); // file -> module_path of first owner

    for crate_info in crates {
        for module in &crate_info.modules {
            if module.file == "<unresolved>" {
                continue;
            }

            let file = &module.file;
            if seen_files.contains_key(file) {
                // Inline module — shares file with an earlier (parent) module.
                continue;
            }
            seen_files.insert(file.clone(), module.path.clone());

            let is_crate_root = module.path == crate_info.name;

            // Compute parent_module_file by walking the crate's modules.
            let parent_module_file = if module.path.is_empty() {
                None // Root module has no parent.
            } else {
                // Strip last ::name segment to get parent path.
                if let Some(pos) = module.path.rfind("::") {
                    let parent_path = &module.path[..pos];
                    // Find the module entry whose path matches the parent.
                    crate_info
                        .modules
                        .iter()
                        .find(|m| m.path == parent_path)
                        .map(|m| m.file.clone())
                } else {
                    None
                }
            };

            let files_entry = match parent_module_file {
                Some(pf) => FileEntry::builder()
                    .module_path(module.path.clone())
                    .parent_module_file(pf)
                    .is_crate_root(is_crate_root)
                    .build(),
                None => FileEntry::builder()
                    .module_path(module.path.clone())
                    .is_crate_root(is_crate_root)
                    .build(),
            };
            files.insert(
                WorkspaceRelativePath(file.clone()),
                files_entry,
            );
        }
    }

    (syms, nidx, files)
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{
        CrateType, DepInfo, ItemAttrs, ItemKind, ModuleInfo,
        PackageInfo, PublicItem,
    };

    fn make_crate(
        name: &str,
        modules: Vec<ModuleInfo>,
    ) -> CrateInfo {
        CrateInfo::builder()
            .name(name.to_string())
            .root("src/lib.rs".to_string())
            .package(
                PackageInfo::builder()
                    .name(name.to_string())
                    .version("0.1.0".to_string())
                    .edition("2021".to_string())
                    .crate_type(CrateType::Lib)
                    .build(),
            )
            .modules(modules)
            .deps(DepInfo::default())
            .build()
    }

    fn make_module(path: &str, file: &str, items: Vec<PublicItem>) -> ModuleInfo {
        ModuleInfo::builder()
            .path(path.to_string())
            .file(file.to_string())
            .visibility("pub".to_string())
            .public_items(items)
            .build()
    }

    fn make_item(name: &str, kind: ItemKind, line: usize) -> PublicItem {
        PublicItem::builder()
            .name(name.to_string())
            .kind(kind)
            .file("src/lib.rs".to_string())
            .line(line)
            .visibility("pub".to_string())
            .generics(String::new())
            .attrs(ItemAttrs::default())
            .build()
    }

    #[test]
    fn symbol_index_single_root_item() {
        let module = make_module(
            "mycrate",
            "src/lib.rs",
            vec![make_item("MyStruct", ItemKind::Struct, 1)],
        );
        let crates = vec![make_crate("mycrate", vec![module])];
        let (syms, _, _) = derive_from_crates(&crates);

        let key = CanonicalPath::from("mycrate::MyStruct".to_string());
        let entry = syms.get(&key).expect("expected mycrate::MyStruct");
        assert_eq!(entry.crate_name, "mycrate");
        assert_eq!(entry.module, "mycrate");
        assert_eq!(entry.kind, ItemKind::Struct);
    }

    #[test]
    fn symbol_index_nested_module_item() {
        let root = make_module("mycrate", "src/lib.rs", vec![]);
        let sub = make_module(
            "mycrate::sub",
            "src/sub.rs",
            vec![make_item("Helper", ItemKind::Fn, 5)],
        );
        let crates = vec![make_crate("mycrate", vec![root, sub])];
        let (syms, _, _) = derive_from_crates(&crates);

        let key = CanonicalPath::from("mycrate::sub::Helper".to_string());
        let entry = syms.get(&key).expect("expected mycrate::sub::Helper");
        assert_eq!(entry.module, "mycrate::sub");
        assert_eq!(entry.file, "src/sub.rs");
    }

    #[test]
    fn name_index_multi_crate_collision() {
        let root_a = make_module(
            "alpha",
            "src/lib.rs",
            vec![make_item("Foo", ItemKind::Struct, 1)],
        );
        let root_b = make_module(
            "beta",
            "src/lib.rs",
            vec![make_item("Foo", ItemKind::Fn, 10)],
        );
        let crates = vec![make_crate("alpha", vec![root_a]), make_crate("beta", vec![root_b])];
        let (_, nidx, _) = derive_from_crates(&crates);

        let candidates = nidx.get("Foo").expect("expected Foo in name_index");
        assert_eq!(candidates.len(), 2);
        assert!(candidates.iter().any(|p| p.as_ref() == "alpha::Foo"));
        assert!(candidates.iter().any(|p| p.as_ref() == "beta::Foo"));
    }

    #[test]
    fn files_excludes_inline_module() {
        // Root module in lib.rs, inline sub in lib.rs (same file).
        let root = make_module(
            "mycrate",
            "src/lib.rs",
            vec![make_item("Item", ItemKind::Struct, 1)],
        );
        let inline = make_module(
            "mycrate::inner",
            "src/lib.rs", // same file as root
            vec![make_item("InnerItem", ItemKind::Struct, 5)],
        );
        let crates = vec![make_crate("mycrate", vec![root, inline])];
        let (_, _, files) = derive_from_crates(&crates);

        // Only one file entry should exist (the root).
        assert_eq!(files.len(), 1);
        let key = WorkspaceRelativePath::from("src/lib.rs".to_string());
        let entry = files.get(&key).expect("expected src/lib.rs in files");
        assert!(entry.is_crate_root);
    }

    #[test]
    fn files_parent_module_file() {
        let root = make_module("mycrate", "src/lib.rs", vec![]);
        let sub = make_module(
            "mycrate::sub",
            "src/sub.rs",
            vec![make_item("X", ItemKind::Struct, 1)],
        );
        let crates = vec![make_crate("mycrate", vec![root, sub])];
        let (_, _, files) = derive_from_crates(&crates);

        let root_key = WorkspaceRelativePath::from("src/lib.rs".to_string());
        let root_entry = files.get(&root_key).expect("expected root");
        assert!(root_entry.parent_module_file.is_none(), "root should have no parent");

        let sub_key = WorkspaceRelativePath::from("src/sub.rs".to_string());
        let sub_entry = files.get(&sub_key).expect("expected sub");
        assert_eq!(
            sub_entry.parent_module_file,
            Some("src/lib.rs".to_string()),
            "sub should have lib.rs as parent"
        );
    }
}
## File: src/lib.rs
#![warn(clippy::pedantic)]

pub mod cargo_info;
pub mod cross_refs;
pub mod file_parser;
pub mod indexes;
pub mod lookup;
pub mod module_tree;
pub mod render;
pub mod schema;
pub mod validate;
pub mod workspace;

pub use schema::Config;

use anyhow::Context;
use rayon::prelude::*;
use schema::{
    CrateInfo, CrateType, DiagnosticKind, ErrorEntry, ErrorSeverity, ModuleInfo,
    WorkspaceInfo, WorkspaceMap,
};
use std::path::Path;

/// Build a `WorkspaceMap` from the given config, without rendering.
///
/// This function contains all pipeline logic up to and including
/// `WorkspaceMap` construction — workspace discovery, per-crate processing,
/// cross-refs computation, index derivation, optional validation, and map building.
///
/// # Errors
///
/// Returns an error if the workspace root cannot be found, the workspace
/// Cargo.toml is missing a `[workspace]` section, member crates cannot be
/// parsed, or the JSON output cannot be written.
#[allow(clippy::too_many_lines)]
pub fn build_map(config: &Config) -> anyhow::Result<WorkspaceMap> {
    let workspace_root = workspace::find_workspace_root(&config.workspace_path)?;
    let member_dirs = workspace::enumerate_members(&workspace_root)?;

    let mut crate_errors: Vec<ErrorEntry> = Vec::new();

    let results: Vec<(Option<CrateInfo>, Vec<ErrorEntry>)> = member_dirs
        .par_iter()
        .map(|dir| {
            let cargo_toml = dir.join("Cargo.toml");
            let mut crate_errors = Vec::new();

            let (pkg, deps) = match cargo_info::parse_cargo_toml(&cargo_toml) {
                Ok(v) => v,
                Err(e) => {
                    crate_errors.push(ErrorEntry::builder()
                        .file(cargo_toml.to_string_lossy().to_string())
                        .message(format!("failed to parse Cargo.toml: {e}"))
                        .severity(ErrorSeverity::Error)
                        .kind(DiagnosticKind::TomlParseError)
                        .cause(e.to_string())
                        .build());
                    return (None, crate_errors);
                }
            };

            let roots = workspace::resolve_crate_roots(dir);
            if roots.is_empty() {
                crate_errors.push(ErrorEntry::builder()
                    .file(dir.to_string_lossy().to_string())
                    .message("no crate entry points found".to_string())
                    .severity(ErrorSeverity::Warning)
                    .kind(DiagnosticKind::MissingCrateRoots)
                    .build());
                return (None, crate_errors);
            }

            let crate_type = if roots.iter().any(|(_, t)| *t == CrateType::Lib)
                && roots.iter().any(|(_, t)| *t == CrateType::Bin)
            {
                CrateType::LibAndBin
            } else {
                roots.first().map_or(CrateType::Lib, |(_, t)| *t)
            };

            let pkg_name = pkg.name.clone();
            let mut modules: Vec<ModuleInfo> = Vec::new();
            let mut collected_errors = Vec::new();
            for (root, _ty) in &roots {
                let (m, e) = module_tree::build_module_tree(root, &pkg_name);
                modules.extend(m);
                collected_errors.extend(e);
            }
            for err in collected_errors {
                crate_errors.push(err);
            }

            // Relativize all paths to the workspace root.
            for m in &mut modules {
                m.file = relativize_path(&m.file, &workspace_root);
                for item in &mut m.public_items {
                    item.file = relativize_path(&item.file, &workspace_root);
                }
            }

            let crate_root = roots
                .first()
                .map(|(r, _)| relativize_path(&r.to_string_lossy(), &workspace_root))
                .unwrap_or_default();

            let rebuilt_pkg = schema::PackageInfo::builder()
                .name(pkg.name)
                .version(pkg.version)
                .edition(pkg.edition)
                .crate_type(crate_type)
                .build();

            let crate_info = CrateInfo::builder()
                .name(pkg_name)
                .root(crate_root)
                .package(rebuilt_pkg)
                .modules(modules)
                .deps(deps)
                .build();

            (Some(crate_info), crate_errors)
        })
        .collect();

    let mut crate_infos: Vec<CrateInfo> = Vec::new();

    for (info, errs) in results {
        // Extend errors in both branches before checking info.
        // `errs` is moved by `extend` — this is fine because results is consumed by the for loop (move iteration).
        if let Some(ci) = info {
            crate_infos.push(ci);
        }
        crate_errors.extend(errs);
    }

    // Deterministic sort by crate name.
    crate_infos.sort_by(|a, b| a.name.cmp(&b.name));

    let cross_refs = cross_refs::compute(&mut crate_infos);

    // Derive flat indexes from crate info.
    let (symbols, name_index, files) = indexes::derive_from_crates(&crate_infos);

    // Run validation if enabled.
    if config.validate {
        let validate_findings = validate::validate(&crate_infos, &symbols, &workspace_root);
        crate_errors.extend(validate_findings);
    }

    let workspace_name = workspace_root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    let workspace_info = WorkspaceInfo::builder()
        .root(workspace_root.to_string_lossy().to_string())
        .workspace_name(workspace_name)
        .build();

    let map = WorkspaceMap::builder()
        .workspace(workspace_info)
        .crates(crate_infos)
        .cross_references(cross_refs)
        .symbols(symbols)
        .name_index(name_index)
        .files(files)
        .errors(crate_errors)
        .workspace_root(workspace_root.clone())
        .build();

    Ok(map)
}

/// Run the full workspace mapping pipeline.
///
/// 1. Discover workspace root and member crates.
/// 2. Process each crate in parallel (Cargo.toml parsing + module tree).
/// 3. Compute cross-crate references.
/// 4. Render JSON to stdout or the configured output file.
///
/// # Errors
///
/// Returns an error if the workspace root cannot be found, the workspace
/// Cargo.toml is missing a `[workspace]` section, member crates cannot be
/// parsed, or the JSON output cannot be written.
pub fn run(config: &Config) -> anyhow::Result<()> {
    let map = build_map(config)?;

    if let Some(ref output_path) = config.output_path {
        let file = std::fs::File::create(output_path)
            .with_context(|| format!("failed to create output file: {}", output_path.display()))?;
        let writer = std::io::BufWriter::new(file);
        render::render_to_writer(&map, writer)?;
    } else {
        let stdout = std::io::stdout();
        render::render_to_writer(&map, stdout.lock())?;
    }

    Ok(())
}

/// Strip the workspace root prefix from a path string, returning a
/// workspace-relative path. If the prefix doesn't match, returns the
/// original string unchanged.
fn relativize_path(path_str: &str, root: &Path) -> String {
    let p = Path::new(path_str);
    match p.strip_prefix(root) {
        Ok(rel) => rel.to_string_lossy().to_string(),
        Err(_) => path_str.to_string(),
    }
}
## File: src/lookup.rs
use crate::schema::{
    FileEntry, ItemKind, ModuleInfo, SymbolEntry, WorkspaceMap,
    WorkspaceRelativePath,
};

/// Result of a symbol lookup.
#[derive(Debug, Clone, serde::Serialize)]
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

/// Hint for disambiguating a symbol name collision.
#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct DisambiguationHint {
    pub canonical_path: String,
    pub crate_name: String,
    pub kind: ItemKind,
    pub file: String,
    pub line: usize,
}

/// Result of a file lookup.
#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct FileLookupResult {
    pub file_entry: FileEntry,
    pub primary_module: ModuleInfo,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub inline_modules: Vec<ModuleInfo>,
}

/// Look up a symbol by name in the workspace map's `name_index`.
///
/// Returns `Found` if exactly one canonical path is found,
/// `Ambiguous` if multiple candidates exist, or `NotFound` otherwise.
#[must_use]
pub fn lookup_symbol(map: &WorkspaceMap, name: &str) -> SymbolLookupResult {
    let Some(candidates) = map.name_index.get(name) else {
        return SymbolLookupResult::NotFound;
    };

    if candidates.len() == 1 {
        let canonical = &candidates[0];
        if let Some(entry) = map.symbols.get(canonical) {
            return SymbolLookupResult::Found(entry.clone());
        }
    }

    if candidates.len() > 1 {
        let mut hints = Vec::new();
        for canonical in candidates {
            if let Some(entry) = map.symbols.get(canonical) {
                hints.push(
                    DisambiguationHint::builder()
                        .canonical_path(canonical.0.clone())
                        .crate_name(entry.crate_name.clone())
                        .kind(entry.kind.clone())
                        .file(entry.file.clone())
                        .line(entry.line)
                        .build(),
                );
            }
        }
        return SymbolLookupResult::Ambiguous {
            name: name.to_string(),
            candidates: hints,
        };
    }

    SymbolLookupResult::NotFound
}

/// Look up a file by workspace-relative path.
///
/// Returns `Some` with the file entry and associated modules if found,
/// or `None` if the file is not in the files index.
#[must_use]
pub fn lookup_file(map: &WorkspaceMap, file: &str) -> Option<FileLookupResult> {
    let key = WorkspaceRelativePath(file.to_string());
    let file_entry = map.files.get(&key)?;

    let mut primary_module: Option<ModuleInfo> = None;
    let mut inline_modules: Vec<ModuleInfo> = Vec::new();

    for crate_info in &map.crates {
        for module in &crate_info.modules {
            if module.file != file {
                continue;
            }
            if module.path == file_entry.module_path {
                primary_module = Some(module.clone());
            } else {
                inline_modules.push(module.clone());
            }
        }
    }

    let primary_module = primary_module?;

    Some(FileLookupResult::builder()
        .file_entry(file_entry.clone())
        .primary_module(primary_module)
        .inline_modules(inline_modules)
        .build())
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{
        CrateInfo, CrateType, CrossReferences, DepInfo, ItemAttrs, ModuleInfo,
        PackageInfo, PublicItem, WorkspaceInfo,
    };

   fn make_test_map() -> WorkspaceMap {
        let item = PublicItem::builder()
            .name("Task".to_string())
            .kind(ItemKind::Struct)
            .file("core/src/lib.rs".to_string())
            .line(1)
            .visibility("pub".to_string())
            .generics(String::new())
            .attrs(ItemAttrs::default())
            .build();
        let module = ModuleInfo::builder()
            .path("core".to_string())
            .file("core/src/lib.rs".to_string())
            .visibility("pub".to_string())
            .public_items(vec![item])
            .build();
        let crate_info = CrateInfo::builder()
            .name("core".to_string())
            .root("core".to_string())
            .package(PackageInfo::builder()
                .name("core".to_string())
                .version("0.1.0".to_string())
                .edition("2021".to_string())
                .crate_type(CrateType::Lib)
                .build())
            .modules(vec![module])
            .deps(DepInfo::default())
            .build();

        let (symbols, name_index, files) = crate::indexes::derive_from_crates(&[crate_info.clone()]);

        WorkspaceMap::builder()
            .workspace(WorkspaceInfo::builder()
                .root("/tmp/test".to_string())
                .workspace_name("test".to_string())
                .build())
            .crates(vec![crate_info])
            .cross_references(CrossReferences::default())
            .symbols(symbols)
            .name_index(name_index)
            .files(files)
            .workspace_root(std::path::PathBuf::from("/tmp/test"))
            .build()
    }

    #[test]
    fn lookup_symbol_found() {
        let map = make_test_map();
        let result = lookup_symbol(&map, "Task");
        match result {
            SymbolLookupResult::Found(entry) => {
                assert_eq!(entry.crate_name, "core");
                assert_eq!(entry.kind, ItemKind::Struct);
            }
            other => panic!("expected Found, got {other:?}"),
        }
    }

    #[test]
    fn lookup_symbol_not_found() {
        let map = make_test_map();
        let result = lookup_symbol(&map, "DoesNotExist");
        assert!(matches!(result, SymbolLookupResult::NotFound));
    }

    #[test]
    fn lookup_file_found() {
        let map = make_test_map();
        let result = lookup_file(&map, "core/src/lib.rs");
        assert!(result.is_some(), "expected file to be found");
    }

    #[test]
    fn lookup_file_not_found() {
        let map = make_test_map();
        let result = lookup_file(&map, "nonexistent.rs");
        assert!(result.is_none());
    }
}
## File: src/main.rs
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Subcommand)]
enum Command {
    /// Generate a JSON map of a Rust workspace's public API surface
    Index {
        /// Path to the workspace root or any directory within it
        path: PathBuf,
        /// Write JSON output to file instead of stdout
        #[arg(short = 'o', long = "output", value_name = "FILE")]
        output: Option<PathBuf>,
        /// Run validation checks (orphan files, dead re-exports)
        #[arg(long)]
        validate: bool,
    },
    /// Look up a symbol or file in a previously generated workspace map
    Lookup {
        /// Path to the workspace root or any directory within it
        path: PathBuf,
        /// Look up by symbol name
        #[arg(long, conflicts_with = "file")]
        symbol: Option<String>,
        /// Look up by file path
        #[arg(long, conflicts_with = "symbol")]
        file: Option<String>,
    },
}

#[derive(Parser)]
#[command(
    name = "rust-workspace-map",
    version,
    about = "Generate a JSON map of a Rust workspace's public API surface"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Index { path, output, validate } => {
            let workspace_path = std::path::absolute(&path)
                .unwrap_or_else(|e| {
                    eprintln!("invalid path {}: {}", path.display(), e);
                    std::process::exit(1);
                });

            let config = if let Some(ref output) = output {
                rust_workspace_map::Config::builder()
                    .workspace_path(workspace_path)
                    .output_path(output.clone())
                    .validate(validate)
                    .build()
            } else {
                rust_workspace_map::Config::builder()
                    .workspace_path(workspace_path)
                    .validate(validate)
                    .build()
            };

            match rust_workspace_map::build_map(&config) {
                Ok(map) => {
                    let exit_code = if validate {
                        map.errors.iter().any(|e| {
                            matches!(
                                e.kind,
                                rust_workspace_map::schema::DiagnosticKind::OrphanFile
                                    | rust_workspace_map::schema::DiagnosticKind::DeadReExport
                            ) && e.severity == rust_workspace_map::schema::ErrorSeverity::Warning
                        })
                    } else {
                        false
                    };

                    if let Some(ref output_path) = config.output_path {
                        let file = match std::fs::File::create(output_path) {
                            Ok(f) => f,
                            Err(e) => {
                                eprintln!("failed to create output file: {e}");
                                std::process::exit(1);
                            }
                        };
                        let writer = std::io::BufWriter::new(file);
                        if rust_workspace_map::render::render_to_writer(&map, writer).is_err() {
                            std::process::exit(1);
                        }
                    } else {
                        let stdout = std::io::stdout();
                        if rust_workspace_map::render::render_to_writer(&map, stdout.lock()).is_err() {
                            std::process::exit(1);
                        }
                    }

                    if exit_code {
                        std::process::exit(2);
                    }
                }
                Err(_) => {
                    std::process::exit(1);
                }
            }
        }
        Command::Lookup { path, symbol, file } => {
            let workspace_path = std::path::absolute(&path)
                .unwrap_or_else(|e| {
                    eprintln!("invalid path {}: {}", path.display(), e);
                    std::process::exit(1);
                });

            let config = rust_workspace_map::Config::builder()
                .workspace_path(workspace_path)
                .build();

            match rust_workspace_map::build_map(&config) {
                Ok(map) => {
                    match (symbol, file) {
                        (Some(sym), None) => {
                            let result = rust_workspace_map::lookup::lookup_symbol(&map, &sym);
                            let json = match serde_json::to_string_pretty(&result) {
                                Ok(j) => j,
                                Err(_) => {
                                    eprintln!("serialization error");
                                    std::process::exit(1);
                                }
                            };
                            println!("{json}");
                            // Exit 1 on NotFound, 0 otherwise.
                            if matches!(result, rust_workspace_map::lookup::SymbolLookupResult::NotFound) {
                                std::process::exit(1);
                            }
                        }
                        (None, Some(f)) => {
                            match rust_workspace_map::lookup::lookup_file(&map, &f) {
                                Some(result) => {
                                    let json = match serde_json::to_string_pretty(&result) {
                                        Ok(j) => j,
                                        Err(_) => {
                                            eprintln!("serialization error");
                                            std::process::exit(1);
                                        }
                                    };
                                    println!("{json}");
                                }
                                None => {
                                    eprintln!("file not found: {f}");
                                    std::process::exit(1);
                                }
                            }
                        }
                        (Some(_), Some(_)) => {
                            // clap's conflicts_with handles this, but be defensive.
                            eprintln!("cannot specify both --symbol and --file");
                            std::process::exit(1);
                        }
                        (None, None) => {
                            eprintln!("must specify either --symbol or --file");
                            std::process::exit(1);
                        }
                    }
                }
                Err(_) => {
                    std::process::exit(1);
                }
            }
        }
    }
}
## File: src/module_tree.rs
use crate::file_parser;
use crate::schema::{DiagnosticKind, ErrorContext, ErrorEntry, ErrorSeverity, FileInfo, ModuleInfo, SubmoduleDecl};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Resolve a `mod name;` declaration to a file path.
/// Tries `{parent_dir}/{mod_name}.rs` first, then `{parent_dir}/{mod_name}/mod.rs`.
///
/// Returns `None` if neither path exists.
#[must_use]
pub fn resolve_module_path(parent_dir: &Path, mod_name: &str) -> Option<PathBuf> {
    let rs_file = parent_dir.join(format!("{mod_name}.rs"));
    if rs_file.exists() {
        return Some(rs_file);
    }
    let mod_dir = parent_dir.join(mod_name).join("mod.rs");
    if mod_dir.exists() {
        return Some(mod_dir);
    }
    None
}

/// Build the full module tree for a crate starting from its entry point
/// (e.g., `src/lib.rs`). Returns a tuple of module info and any errors
/// encountered during submodule parsing (including orphaned module warnings).
#[must_use]
pub fn build_module_tree(
    crate_root: &Path,
    crate_name: &str,
) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {

    let mut visited = HashSet::new();
    let parent_dir = crate_root.parent().unwrap_or_else(|| Path::new("."));

    let parsed = file_parser::parse_file(crate_root);
    let mut errors: Vec<ErrorEntry> = Vec::new();
    if let Some(ref err) = parsed.parse_error {
        errors.push(crate::file_parser::build_parse_error_entry(crate_root, err));
    }
    visited.insert(crate_root.to_path_buf());

    let root_module = build_module_info(
        crate_name,
        crate_root,
        "pub",
        &parsed.file_info.public_items,
        &parsed.file_info.imports,
        &parsed.file_info.re_exports,
        &parsed.file_info.submodules,
    );

    let mut modules = vec![root_module];

    for sub in &parsed.file_info.submodules {
        if sub.is_test {
            continue;
        }
        let sub_module_path = format!("{}::{}", crate_name, sub.name);
        let (child_modules, child_errors) = process_submodule(
            &sub_module_path,
            &sub.name,
            &parsed.ast.items,
            parent_dir,
            crate_root,
            &mut visited,
        );
        errors.extend(child_errors);
        modules.extend(child_modules);
    }

    (modules, errors)
}

// ── Internal helpers ────────────────────────────────────────────────────

fn build_module_info(
    path: &str,
    file: &Path,
    visibility: &str,
    public_items: &[crate::schema::PublicItem],
    imports: &[crate::schema::Import],
    re_exports: &[crate::schema::ReExport],
    submodules: &[SubmoduleDecl],
) -> ModuleInfo {
    ModuleInfo::builder()
        .path(path.to_string())
        .file(file.to_string_lossy().to_string())
        .visibility(visibility.to_string())
        .public_items(public_items.to_vec())
        .imports(imports.to_vec())
        .re_exports(re_exports.to_vec())
        .submodules(
            submodules
                .iter()
                .map(|s| s.name.clone())
                .collect::<Vec<_>>(),
        )
        .build()
}

fn process_submodule(
    module_path: &str,
    mod_name: &str,
    parent_items: &[syn::Item],
    parent_dir: &Path,
    parent_file: &Path,
    visited: &mut HashSet<PathBuf>,
) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {
    // Locate the `mod` item in the parent's AST.
    let mod_item = parent_items.iter().find_map(|item| {
        if let syn::Item::Mod(m) = item
            && m.ident == mod_name
        {
            return Some(m);
        }
        None
    });

    let Some(mod_item) = mod_item else {
        let err = ErrorEntry::builder()
            .file(String::new())
            .message(format!("orphaned module: {module_path}"))
            .severity(ErrorSeverity::Warning)
            .kind(DiagnosticKind::OrphanedModule)
            .context(ErrorContext::builder()
                .module_path(module_path.to_string())
                .build())
            .build();
        return (vec![ModuleInfo::builder()
            .path(module_path.to_string())
            .file("<unresolved>".to_string())
            .visibility("private".to_string())
            .build()], vec![err]);
    };

    let visibility = if matches!(mod_item.vis, syn::Visibility::Public(_)) {
        "pub"
    } else {
        "private"
    };

    if let Some((_, ref inline_items)) = mod_item.content {
        // Inline module: process its body items directly (no file lookup).
        let (modules, errs) = process_module_items(
            module_path,
            parent_file,
            visibility,
            inline_items,
            parent_dir,
            visited,
        );
        return (modules, errs);
    }
    // External module: resolve file path, parse, and recurse.
    let file_path = resolve_module_path(parent_dir, mod_name);
    let Some(ref file_path) = file_path else {
        let err = ErrorEntry::builder()
            .file(String::new())
            .message(format!("orphaned module: {module_path}"))
            .severity(ErrorSeverity::Warning)
            .kind(DiagnosticKind::OrphanedModule)
            .context(ErrorContext::builder()
                .module_path(module_path.to_string())
                .build())
            .build();
        return (vec![ModuleInfo::builder()
            .path(module_path.to_string())
            .file("<unresolved>".to_string())
            .visibility(visibility.to_string())
            .build()], vec![err]);
    };

    if visited.contains(file_path.as_path()) {
        return (vec![], vec![]); // cycle detected
    }
    visited.insert(file_path.clone());

    let parsed = file_parser::parse_file(file_path);
    let mut errors: Vec<ErrorEntry> = Vec::new();
    if let Some(ref err) = parsed.parse_error {
        errors.push(crate::file_parser::build_parse_error_entry(file_path, err));
    }
    let modules = process_module_info(
        module_path,
        file_path,
        visibility,
        &parsed.file_info,
        &parsed.ast.items,
        file_path.parent().unwrap_or(file_path),
        visited,
        &mut errors,
    );
    (modules, errors)
}

fn process_module_items(
    module_path: &str,
    file_path: &Path,
    visibility: &str,
    items: &[syn::Item],
    parent_dir: &Path,
    visited: &mut HashSet<PathBuf>,
) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {
    let file_info = FileInfo {
        public_items: file_parser::extract_public_items(items),
        imports: file_parser::extract_imports(items),
        re_exports: file_parser::extract_re_exports(items),
        submodules: file_parser::extract_submodules(items),
        impls: file_parser::extract_impls(items),
    };
    let mut errs = Vec::new();
    let modules = process_module_info(module_path, file_path, visibility, &file_info, items, parent_dir, visited, &mut errs);
    (modules, errs)
}

#[allow(clippy::too_many_arguments)]
fn process_module_info(
    module_path: &str,
    file_path: &Path,
    visibility: &str,
    file_info: &FileInfo,
    items: &[syn::Item],
    _parent_dir: &Path,
    visited: &mut HashSet<PathBuf>,
    errors: &mut Vec<ErrorEntry>,
) -> Vec<ModuleInfo> {
    let mut modules = vec![build_module_info(
        module_path,
        file_path,
        visibility,
        &file_info.public_items,
        &file_info.imports,
        &file_info.re_exports,
        &file_info.submodules,
    )];

    for sub in &file_info.submodules {
        if sub.is_test {
            continue;
        }
        let child_path = format!("{}::{}", module_path, sub.name);
        let child_dir = file_path.parent().unwrap_or(file_path);
        let child_modules = process_submodule(
            &child_path,
            &sub.name,
            items,
            child_dir,
            file_path,
            visited,
        );
        errors.extend(child_modules.1);
        modules.extend(child_modules.0);
    }

    modules
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_module_path_finds_rs_file() {
        let tmp = std::env::temp_dir().join("resolve_test");
        let _ = std::fs::create_dir_all(&tmp);
        let mod_file = tmp.join("foo.rs");
        std::fs::write(&mod_file, "").ok();
        let result = resolve_module_path(&tmp, "foo");
        assert_eq!(result, Some(mod_file));
        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn resolve_module_path_finds_mod_rs() {
        let tmp = std::env::temp_dir().join("resolve_test2");
        let _ = std::fs::create_dir_all(&tmp);
        let mod_dir = tmp.join("bar");
        let _ = std::fs::create_dir_all(&mod_dir);
        let mod_rs = mod_dir.join("mod.rs");
        std::fs::write(&mod_rs, "").ok();
        let result = resolve_module_path(&tmp, "bar");
        assert_eq!(result, Some(mod_rs));
        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn resolve_module_path_returns_none_for_missing() {
        let tmp = std::env::temp_dir().join("resolve_test3");
        let _ = std::fs::create_dir_all(&tmp);
        let result = resolve_module_path(&tmp, "nonexistent");
        assert!(result.is_none());
        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn build_module_tree_returns_empty_for_nonexistent() {
        let tmp = std::env::temp_dir().join("bmt_test");
        let _ = std::fs::create_dir_all(&tmp);
        let (modules, errors) = build_module_tree(&tmp, "test");
        assert!(!modules.is_empty());
        assert!(!errors.is_empty());
        std::fs::remove_dir_all(&tmp).ok();
    }
}
## File: src/schema.rs
use std::collections::BTreeMap;
use std::path::PathBuf;

// ── Path newtypes for flat indexes ──────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct CanonicalPath(pub String);

impl std::fmt::Display for CanonicalPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for CanonicalPath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for CanonicalPath {
    fn from(s: String) -> Self {
        Self(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct WorkspaceRelativePath(pub String);

impl std::fmt::Display for WorkspaceRelativePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for WorkspaceRelativePath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for WorkspaceRelativePath {
    fn from(s: String) -> Self {
        Self(s)
    }
}

// ── Error type ──────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no workspace root found starting from {0}")]
    WorkspaceRootNotFound(PathBuf),

    #[error("failed to read file {path}: {source}")]
    FileRead {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("failed to parse {path}: {source}")]
    TomlParse {
        path: PathBuf,
        source: toml::de::Error,
    },

    #[error("failed to parse Rust source {path}: {source}")]
    SynParse {
        path: PathBuf,
        source: syn::Error,
    },

    #[error("workspace member {0} does not exist")]
    MemberNotFound(PathBuf),

    #[error("glob pattern error: {0}")]
    GlobPattern(String),

    #[error("workspace Cargo.toml is missing the [workspace] section")]
    MissingWorkspaceSection,
}

pub type Result<T> = std::result::Result<T, Error>;

// ── Config ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, bon::Builder)]
pub struct Config {
    /// Absolute, canonical path to the workspace root (or a subdirectory within it).
    pub workspace_path: PathBuf,

    /// If Some, write JSON to this file instead of stdout.
    pub output_path: Option<PathBuf>,

    /// When true, run validation checks (orphan files, dead re-exports).
    #[builder(default)]
    pub validate: bool,
}

// ── Crate type ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CrateType {
    Lib,
    Bin,
    #[serde(rename = "lib_and_bin")]
    LibAndBin,
}

// ── Top-level output ────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceMap {
    pub workspace: WorkspaceInfo,
    pub crates: Vec<CrateInfo>,
    pub cross_references: CrossReferences,

    #[builder(default)]
    pub symbols: BTreeMap<CanonicalPath, SymbolEntry>,

    #[builder(default)]
    pub name_index: BTreeMap<String, Vec<CanonicalPath>>,

    #[builder(default)]
    pub files: BTreeMap<WorkspaceRelativePath, FileEntry>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<ErrorEntry>,

    /// Not serialized — used for path relativization during construction.
    #[builder(default)]
    #[serde(skip)]
    pub workspace_root: PathBuf,
}

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceInfo {
    pub root: String,
    pub workspace_name: String,
}

// ── Crate ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct CrateInfo {
    pub name: String,
    pub root: String,
    pub package: PackageInfo,
    pub modules: Vec<ModuleInfo>,
    pub deps: DepInfo,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cross_crate_imports: Vec<CrossCrateImport>,
}

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub edition: String,
    pub crate_type: CrateType,
}

// ── Dependencies ────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, Default, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct DepInfo {
    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub normal: Vec<String>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dev: Vec<String>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub workspace_members: Vec<String>,
}

// ── Module ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ModuleInfo {
    pub path: String,
    pub file: String,
    pub visibility: String,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub public_items: Vec<PublicItem>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub imports: Vec<Import>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub re_exports: Vec<ReExport>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub submodules: Vec<String>,
}

// ── Public items ────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct PublicItem {
    pub kind: ItemKind,
    pub name: String,
    pub file: String,
    pub line: usize,
    pub attrs: ItemAttrs,
    pub generics: String,
    pub visibility: String,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<String>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub variants: Vec<String>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub impls: Vec<ImplInfo>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ItemKind {
    Struct,
    Enum,
    Trait,
    Fn,
    Type,
    Macro,
}

#[derive(Debug, Clone, serde::Serialize, Default, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ItemAttrs {
    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub derive: Vec<String>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub doc: Vec<String>,
}

// ── Impl blocks ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ImplInfo {
    /// Serialized as "type" in JSON.
    #[serde(rename = "type")]
    pub type_: String,
    pub items: Vec<ImplItem>,
}

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ImplItem {
    pub kind: ImplItemKind,
    pub name: String,
    pub params: String,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ImplItemKind {
    Fn,
    Type,
    Const,
}

// ── Imports / Re-exports ────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct Import {
    pub path: String,
    pub line: usize,
}

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ReExport {
    pub import_path: String,
    pub export_path: String,
    pub line: usize,
}

// ── Cross-crate ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct CrossCrateImport {
    pub import_path: String,
    pub target_crate: String,
    pub symbol: String,
    pub line: usize,
}

#[derive(Debug, Clone, serde::Serialize, Default, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct CrossReferences {
    #[builder(default)]
    pub types: BTreeMap<String, TypeRef>,
}

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct TypeRef {
    pub crate_name: String,
    pub kind: String,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub imported_by: Vec<String>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub exported_by: Vec<String>,
}

// ── Diagnostic kind ─────────────────────────────────────────────────────

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

// ── Error severity ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorSeverity {
    Error,
    Warning,
}

// ── Error context ───────────────────────────────────────────────────────

/// Optional context attached to an error, providing additional location
/// and source information for diagnostics.
#[derive(Debug, Clone, Default, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ErrorContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crate_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_path: Option<String>,

    /// Line number in the source file where the error occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<usize>,

    /// A short source snippet near the error location (if available).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
}

// ── Internal types ──────────────────────────────────────────────────────

/// Internal intermediate type consumed by `module_tree`.
#[derive(Debug, Clone, Default)]
pub struct FileInfo {
    pub public_items: Vec<PublicItem>,
    pub imports: Vec<Import>,
    pub re_exports: Vec<ReExport>,
    pub submodules: Vec<SubmoduleDecl>,
    pub impls: Vec<ImplInfo>,
}

#[derive(Debug, Clone, bon::Builder)]
pub struct SubmoduleDecl {
    pub name: String,
    #[builder(default)]
    pub is_test: bool,
}

// ── Flat index entry types ──────────────────────────────────────────────

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

// ── Error reporting ─────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ErrorEntry {
    pub file: String,
    #[builder(default)]
    pub line: usize,
    pub message: String,
    pub severity: ErrorSeverity,
    pub kind: DiagnosticKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ErrorContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}
## File: src/validate.rs
use crate::schema::{
    CanonicalPath, DiagnosticKind, ErrorEntry, ErrorSeverity, CrateInfo,
};
use std::collections::HashSet;
use std::path::Path;
use walkdir::WalkDir;

/// Run validation checks on the crate set.
///
/// Checks performed:
/// - **Orphan files**: `.rs` files on disk not declared in any module tree
/// - **Dead re-exports**: `pub use` to symbols not found in the symbol index
///
/// Returns a list of `ErrorEntry` findings (severity: Warning).
#[must_use]
pub fn validate(
    crates: &[CrateInfo],
    symbols: &std::collections::BTreeMap<CanonicalPath, crate::schema::SymbolEntry>,
    workspace_root: &Path,
) -> Vec<ErrorEntry> {
    let mut findings: Vec<ErrorEntry> = Vec::new();
    let crate_names: std::collections::HashSet<&str> =
        crates.iter().map(|c| c.name.as_str()).collect();

    for crate_info in crates {
        findings.extend(check_orphan_files(crate_info, workspace_root));
        findings.extend(check_dead_reexports(crate_info, symbols, &crate_names));
    }

    findings
}

/// Find `.rs` files on disk that are not declared in the module tree.
fn check_orphan_files(
    crate_info: &CrateInfo,
    workspace_root: &Path,
) -> Vec<ErrorEntry> {
    // crate_info.root is a relativized path to the crate root file (e.g., "src/lib.rs").
    // We need the src/ directory, which is the parent of the root file.
    let crate_root_path = workspace_root.join(&crate_info.root);
    let src_dir = crate_root_path
        .parent()
        .unwrap_or(workspace_root)
        .to_path_buf();
    let crate_name = &crate_info.name;

    // Collect all file paths declared in the module tree.
    let declared_files: HashSet<String> = crate_info
        .modules
        .iter()
        .filter(|m| m.file != "<unresolved>")
        .map(|m| m.file.clone())
        .collect();

    // Walk the src/ directory recursively using walkdir.
    let mut findings = Vec::new();

    for entry in WalkDir::new(&src_dir).into_iter().filter_entry(|e| {
        // Always yield the root; keep files (filtered later); skip hidden, bin/, tests/ dirs.
        e.depth() == 0 || !e.file_type().is_dir() || {
            let name = e.file_name().to_string_lossy();
            !name.starts_with('.') && name != "bin" && name != "tests"
        }
    }) {
        let Ok(entry) = entry else { continue };
        let path = entry.path();

        if !path.is_file() {
            continue;
        }
        if path.extension().is_none_or(|e| e != "rs") {
            continue;
        }
        let file_name = path.file_name().map_or_else(String::new, |f| f.to_string_lossy().into_owned());
        if file_name == "lib.rs" || file_name == "main.rs" || file_name == "mod.rs" {
            continue; // Crate roots and mod.rs — declared implicitly.
        }

        let ws_rel = path.strip_prefix(workspace_root)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();

        if declared_files.contains(&ws_rel) {
            continue;
        }

        let parent_file = determine_parent_file(&ws_rel, crate_name, crate_info);
        let stem = path.file_stem()
            .map_or_else(String::new, |s| s.to_string_lossy().into_owned());
        findings.push(ErrorEntry::builder()
            .file(ws_rel.clone())
            .message(format!(
                "orphan file: '{file_name}' is not declared in the module tree. Add 'pub mod {stem};' to {parent_file}."
            ))
            .severity(ErrorSeverity::Warning)
            .kind(DiagnosticKind::OrphanFile)
            .build());
    }

    findings
}

/// Determine the parent file for an orphan file's fix hint.
fn determine_parent_file(
    orphan_file: &str,
    crate_name: &str,
    _crate_info: &CrateInfo,
) -> String {
    // Strip src/ prefix, resolve parent directory to a module file.
    let stripped = orphan_file.strip_prefix("src/").unwrap_or(orphan_file);
    let parent_dir = stripped.rsplit_once('/').map(|(dir, _)| dir);

    match parent_dir {
        Some("") | None => {
            // File is directly in src/ — parent is lib.rs or main.rs.
            format!("{crate_name}/src/lib.rs")
        }
        Some(dir) => {
            // File is in a subdirectory — parent module file is dir/lib.rs or dir/mod.rs.
            format!("{crate_name}/src/{dir}/mod.rs")
        }
    }
}

/// Find `pub use` re-exports that reference symbols not in the index.
fn check_dead_reexports(
    crate_info: &CrateInfo,
    symbols: &std::collections::BTreeMap<CanonicalPath, crate::schema::SymbolEntry>,
    crate_names: &HashSet<&str>,
) -> Vec<ErrorEntry> {
    let mut findings = Vec::new();
    let my_name = crate_info.name.as_str();

    for module in &crate_info.modules {
        for re_export in &module.re_exports {
            let path = &re_export.import_path;

            // Skip glob re-exports.
            if path.ends_with("::*") {
                continue;
            }

            // Parse the import path. Resolve 'crate::' prefix.
            let resolved = resolve_import_path(path, my_name, &module.path);

            // If first segment is a workspace member crate name, skip (external re-export).
            let first_seg = resolved.split("::").next().unwrap_or(&resolved);
            if crate_names.contains(first_seg) && first_seg != my_name {
                continue;
            }

            // Build the canonical path from the resolved import path.
            let canonical = CanonicalPath::from(resolved.clone());

            // Look up in the symbols map.
            if !symbols.contains_key(&canonical) {
                findings.push(ErrorEntry::builder()
                    .file(module.file.clone())
                    .line(re_export.line)
                    .message(format!(
                        "dead re-export: '{}' resolves to '{}' which is not found in the symbol index. Remove or fix the 'pub use' statement.",
                        re_export.export_path,
                        re_export.import_path
                    ))
                    .severity(ErrorSeverity::Warning)
                    .kind(DiagnosticKind::DeadReExport)
                    .build());
            }
        }
    }

    findings
}

/// Resolve an import path by expanding `crate::`, `self::`, `super::` prefixes.
fn resolve_import_path(path: &str, crate_name: &str, module_path: &str) -> String {
    if let Some(rest) = path.strip_prefix("crate::") {
        format!("{crate_name}::{rest}")
    } else if let Some(rest) = path.strip_prefix("self::") {
        format!("{module_path}::{rest}")
    } else if let Some(rest) = path.strip_prefix("super::") {
        let parent = module_path.rsplit_once("::")
            .map_or("", |(p, _)| p);
        if parent.is_empty() {
            format!("{crate_name}::{rest}")
        } else {
            format!("{parent}::{rest}")
        }
    } else {
        // Bare path — assume it's relative to the crate root.
        format!("{crate_name}::{path}")
    }
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_import_path_crate_prefix() {
        assert_eq!(
            resolve_import_path("crate::foo::bar", "mycrate", "mycrate::sub"),
            "mycrate::foo::bar"
        );
    }

    #[test]
    fn resolve_import_path_self_prefix() {
        assert_eq!(
            resolve_import_path("self::inner", "mycrate", "mycrate::sub"),
            "mycrate::sub::inner"
        );
    }

    #[test]
    fn resolve_import_path_super_prefix() {
        assert_eq!(
            resolve_import_path("super::other", "mycrate", "mycrate::sub::deep"),
            "mycrate::sub::other"
        );
    }

    #[test]
    fn resolve_import_path_super_root() {
        // At crate root, super:: wraps to crate root.
        assert_eq!(
            resolve_import_path("super::helper", "mycrate", "mycrate"),
            "mycrate::helper"
        );
    }

    #[test]
    fn resolve_import_path_bare() {
        assert_eq!(
            resolve_import_path("foo::Bar", "mycrate", "mycrate::sub"),
            "mycrate::foo::Bar"
        );
    }
}
## File: tests/fixtures/bad-dead-reexport/Cargo.toml
[workspace]
members = ["."]

[package]
name = "bad-dead-reexport"
version = "0.1.0"
edition = "2021"
## File: tests/fixtures/bad-dead-reexport/src/lib.rs
pub use crate::DoesNotExist;
## File: tests/fixtures/bad-orphan/Cargo.toml
[workspace]
members = ["."]

[package]
name = "bad-orphan"
version = "0.1.0"
edition = "2021"
## File: tests/fixtures/bad-orphan/src/forgotten.rs
pub fn not_declared() {}
## File: tests/fixtures/bad-orphan/src/lib.rs
// root module — intentionally does NOT include 'mod forgotten;'
## File: tests/integration_test.rs
use std::process::Command;

fn binary_path() -> String {
    let root = env!("CARGO_MANIFEST_DIR");
    format!("{}/target/debug/rust-workspace-map", root)
}

fn extract_array<'a>(val: &'a serde_json::Value, key: &str) -> Vec<&'a serde_json::Value> {
    val.get(key)
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().collect())
        .unwrap_or_default()
}

// ── Existing tests (rewritten for index subcommand) ─────────────────────

#[test]
fn test_sample_workspace_output() {
    let fixture = std::path::Path::new("tests/fixtures/sample-workspace");

    let output = Command::new(&binary_path())
        .arg("index")
        .arg(fixture)
        .output()
        .expect("failed to execute binary");

    assert!(
        output.status.success(),
        "binary exited with: {}\nstderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("output is not valid JSON");

    // Top-level structure.
    assert!(!json["workspace"]["root"].as_str().unwrap().is_empty());
    assert!(!json["workspace"]["workspaceName"].as_str().unwrap().is_empty());
    assert!(json["crates"].is_array(), "crates must be an array");

    let crates = json["crates"].as_array().unwrap();

    // Both crates should be present.
    let names: Vec<&str> = crates.iter().map(|c| c["name"].as_str().unwrap()).collect();
    assert!(names.contains(&"core"), "missing core crate");
    assert!(names.contains(&"engine"), "missing engine crate");

    // Find the core crate and verify its modules.
    let core_crate = crates.iter().find(|c| c["name"] == "core").unwrap();
    assert_eq!(core_crate["package"]["crateType"], "lib");
    assert!(!core_crate["modules"].as_array().unwrap().is_empty());

     // core should have a public Task struct.
    let has_task = {
        let items: Vec<_> = core_crate["modules"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|m| extract_array(m, "publicItems"))
            .collect();
        items.iter().any(|item| item["name"] == "Task" && item["kind"] == "struct")
    };
    assert!(has_task, "core should export pub struct Task");

    // Find the engine crate.
    let engine_crate = crates.iter().find(|c| c["name"] == "engine").unwrap();
    assert!(!engine_crate["modules"].as_array().unwrap().is_empty());

    // engine should import from core.
    let engine_imports_core = {
        let imports: Vec<_> = engine_crate["modules"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|m| extract_array(m, "imports"))
            .collect();
        imports.iter().any(|imp| imp["path"].as_str().unwrap().contains("core"))
    };
    assert!(engine_imports_core, "engine should import from core");

    // Cross-references: keys are now canonical paths (module.path::item.name).
    let cross_refs = &json["crossReferences"]["types"];
    // Task is exported from the "core" module, so key is "core::Task".
    let task_ref = cross_refs
        .get("core::Task")
        .expect("core::Task should appear in crossReferences.types (canonical path)");
    assert!(
        task_ref["exportedBy"]
            .as_array()
            .unwrap()
            .contains(&serde_json::Value::String("core".into())),
        "Task should be exported by core"
    );
}

#[test]
fn test_deterministic_output() {
    let fixture = std::path::Path::new("tests/fixtures/sample-workspace");

    let output1 = Command::new(&binary_path())
        .arg("index")
        .arg(fixture)
        .output()
        .expect("failed to execute binary (run 1)");
    assert!(output1.status.success());

    let output2 = Command::new(&binary_path())
        .arg("index")
        .arg(fixture)
        .output()
        .expect("failed to execute binary (run 2)");
    assert!(output2.status.success());

    assert_eq!(
        output1.stdout, output2.stdout,
        "output must be byte-identical across runs"
    );
}

#[test]
fn test_missing_path_exits_nonzero() {
    let bin = binary_path();
    let output = Command::new(&bin)
        .arg("index")
        .arg("/tmp/nonexistent-path-12345")
        .output()
        .expect("failed to execute binary");

    assert!(
        !output.status.success(),
        "should exit non-zero for invalid path"
    );
}

fn run_index(path: &str) -> std::process::Output {
    Command::new(&binary_path())
        .arg("index")
        .arg(path)
        .output()
        .expect("failed to execute binary")
}

fn parse_output(output: &std::process::Output) -> serde_json::Value {
    serde_json::from_str(&String::from_utf8_lossy(&output.stdout)).unwrap()
}

fn write_cargo_toml(dir: &std::path::Path, content: &str) {
    let mut f = std::fs::File::create(dir.join("Cargo.toml")).unwrap();
    use std::io::Write;
    f.write_all(content.as_bytes()).unwrap();
}

fn setup_crate(dir: &std::path::Path, lib_content: &str) {
    let src = dir.join("src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(src.join("lib.rs"), lib_content).unwrap();
    let name = dir.file_name().unwrap().to_string_lossy();
    let cargo = format!(
        "[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    );
    std::fs::write(dir.join("Cargo.toml"), cargo).unwrap();
}

#[test]
fn test_parse_failure_error_entry() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    // Create workspace Cargo.toml
    write_cargo_toml(root, r#"
[workspace]
members = ["good_crate", "bad_crate"]
"#);

    // Good crate with valid Rust
    setup_crate(&root.join("good_crate"), "pub struct Good {}");

    // Bad crate with invalid Rust syntax
    setup_crate(&root.join("bad_crate"), "pub struct { invalid rust syntax");

    let output = run_index(root.to_str().unwrap());
    assert!(output.status.success());

    let json = parse_output(&output);
    let errors: Vec<&serde_json::Value> = extract_array(&json, "errors");

    let parse_errors: Vec<_> = errors.iter()
        .filter(|e| {
            e["kind"].as_str().unwrap() == "syn_parse_error"
        })
        .collect();

    assert!(!parse_errors.is_empty(), "should have parse error entries");
    assert_eq!(parse_errors[0]["severity"].as_str().unwrap(), "error");
}

#[test]
fn test_missing_workspace_section() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    // Cargo.toml without [workspace] section
    write_cargo_toml(root, r#"
[package]
name = "standalone"
version = "0.1.0"
edition = "2021"
"#);

    let output = Command::new(&binary_path())
        .arg("index")
        .arg(root.to_str().unwrap())
        .output();

    // Should exit non-zero because workspace is missing
    let output = output.expect("failed to execute binary");
    assert!(
        !output.status.success(),
        "should exit non-zero for missing workspace section"
    );
}

#[test]
fn test_glob_member_patterns() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    write_cargo_toml(root, r#"
[workspace]
members = ["crates/*"]
"#);

    for name in &["alpha", "beta", "gamma"] {
        setup_crate(&root.join("crates").join(name), format!("pub struct {name} {{}}").as_str());
    }

    let output = run_index(root.to_str().unwrap());
    assert!(output.status.success());

    let json = parse_output(&output);
    let crates = extract_array(&json, "crates");
    let names: Vec<&str> = crates.iter()
        .map(|c| c["name"].as_str().unwrap())
        .collect();

    assert!(names.contains(&"alpha"));
    assert!(names.contains(&"beta"));
    assert!(names.contains(&"gamma"));
    assert_eq!(names.len(), 3);
}

#[test]
fn test_workspace_with_exclude() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    write_cargo_toml(root, r#"
[workspace]
members = ["a", "b", "c"]
exclude = ["b"]
"#);

    setup_crate(&root.join("a"), "pub struct A {}");
    setup_crate(&root.join("b"), "pub struct B {}");
    setup_crate(&root.join("c"), "pub struct C {}");

    let output = run_index(root.to_str().unwrap());
    assert!(output.status.success());

    let json = parse_output(&output);
    let crates = extract_array(&json, "crates");
    let names: Vec<&str> = crates.iter()
        .map(|c| c["name"].as_str().unwrap())
        .collect();

    assert!(names.contains(&"a"));
    assert!(!names.contains(&"b"));
    assert!(names.contains(&"c"));
}

#[test]
fn test_deeply_nested_modules() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    write_cargo_toml(root, r#"
[workspace]
members = ["."]

[package]
name = "nested"
version = "0.1.0"
edition = "2021"
"#);

    let src = root.join("src");
    let foo = src.join("foo");
    let bar = foo.join("bar");
    std::fs::create_dir_all(&bar).unwrap();

    // lib.rs declares mod foo (resolves to src/foo/mod.rs)
    std::fs::write(src.join("lib.rs"), "mod foo;").unwrap();
    // foo/mod.rs declares mod bar
    std::fs::write(foo.join("mod.rs"), "mod bar;").unwrap();
    // bar/mod.rs declares mod baz
    std::fs::write(bar.join("mod.rs"), "mod baz;").unwrap();
    // bar/baz.rs with a struct
    std::fs::write(bar.join("baz.rs"), "pub struct Deep {}").unwrap();

    let output = run_index(root.to_str().unwrap());
    assert!(output.status.success());

    let json = parse_output(&output);
    let crates = extract_array(&json, "crates");
    let nested_crate = crates.iter().find(|c| c["name"].as_str().unwrap() == "nested").unwrap();

    let modules = extract_array(&nested_crate, "modules");
    let module_paths: Vec<&str> = modules
        .iter()
        .map(|m| m["path"].as_str().unwrap())
        .collect();

    assert!(module_paths.iter().any(|p| *p == "nested"));
    assert!(module_paths.iter().any(|p| *p == "nested::foo"));
    assert!(module_paths.iter().any(|p| *p == "nested::foo::bar"));
    assert!(module_paths.iter().any(|p| *p == "nested::foo::bar::baz"));
}

#[test]
fn test_reexport_chains() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    write_cargo_toml(root, r#"
[workspace]
members = ["."]

[package]
name = "reexporter"
version = "0.1.0"
edition = "2021"
"#);

    let src = root.join("src");
    std::fs::create_dir_all(&src).unwrap();

    // lib.rs with re-export chain
    std::fs::write(src.join("lib.rs"), "
mod inner {
    pub struct Secret;
}
pub use inner::Secret;
").unwrap();

    let output = run_index(root.to_str().unwrap());
    assert!(output.status.success());

    let json = parse_output(&output);
    let crates = extract_array(&json, "crates");
    let reexporter = crates.iter().find(|c| c["name"].as_str().unwrap() == "reexporter").unwrap();

    let re_exports: Vec<&serde_json::Value> = extract_array(&reexporter, "modules")
        .iter()
        .flat_map(|m| extract_array(m, "reExports"))
        .collect();

    let has_secret = re_exports.iter().any(|re| {
        re["importPath"].as_str().unwrap().contains("Secret")
    });
    assert!(has_secret, "should have re-export for Secret");
}

#[test]
fn test_output_via_flag() {
    let tmp = tempfile::tempdir().unwrap();
    let fixture = std::path::Path::new("tests/fixtures/sample-workspace");
    let output_path = tmp.path().join("output.json");

    // Run with -o flag
    let output1 = Command::new(&binary_path())
        .arg("index")
        .arg(fixture)
        .arg("-o")
        .arg(output_path.clone())
        .output()
        .expect("failed to execute binary");
    assert!(output1.status.success());

    // Run without -o, capture stdout
    let output2 = Command::new(&binary_path())
        .arg("index")
        .arg(fixture)
        .output()
        .expect("failed to execute binary");
    assert!(output2.status.success());

    // Compare file content with stdout
    let file_content = std::fs::read_to_string(&output_path).unwrap();
    let stdout_content = String::from_utf8_lossy(&output2.stdout);
    assert_eq!(
        file_content.trim(),
        stdout_content.trim(),
        "file output should match stdout"
    );
}

// ── New validation tests ────────────────────────────────────────────────

#[test]
fn test_validate_orphan_file_exits_2() {
    let fixture = std::path::Path::new("tests/fixtures/bad-orphan");

    let output = Command::new(&binary_path())
        .arg("index")
        .arg("--validate")
        .arg(fixture)
        .output()
        .expect("failed to execute binary");

    assert_eq!(
        output.status.code().unwrap(),
        2,
        "validation should exit 2 for orphan files"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("output is not valid JSON");

    let errors = extract_array(&json, "errors");
    assert!(!errors.is_empty(), "should have validation errors");

    let orphan_error = errors.iter().find(|e| e["kind"].as_str().unwrap() == "orphan_file")
        .expect("should have orphan_file error");

    assert_eq!(orphan_error["severity"].as_str().unwrap(), "warning");
    let message = orphan_error["message"].as_str().unwrap();
    assert!(message.contains("forgotten"), "message should mention the orphan file");
    assert!(message.contains("pub mod"), "message should suggest fix hint");
}

#[test]
fn test_validate_dead_reexport_exits_2() {
    let fixture = std::path::Path::new("tests/fixtures/bad-dead-reexport");

    let output = Command::new(&binary_path())
        .arg("index")
        .arg("--validate")
        .arg(fixture)
        .output()
        .expect("failed to execute binary");

    assert_eq!(
        output.status.code().unwrap(),
        2,
        "validation should exit 2 for dead re-exports"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("output is not valid JSON");

    let errors = extract_array(&json, "errors");
    let dead_reexport = errors.iter().find(|e| e["kind"].as_str().unwrap() == "dead_re_export")
        .expect("should have dead_re_export error");

    assert_eq!(dead_reexport["severity"].as_str().unwrap(), "warning");
}

#[test]
fn test_index_no_validate_exits_0() {
    let fixture = std::path::Path::new("tests/fixtures/bad-orphan");

    let output = Command::new(&binary_path())
        .arg("index")
        .arg(fixture)
        .output()
        .expect("failed to execute binary");

    assert_eq!(
        output.status.code().unwrap(),
        0,
        "without --validate, should exit 0"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("output is not valid JSON");

    let errors = extract_array(&json, "errors");
    let has_orphan = errors.iter().any(|e| {
        e["kind"].as_str().unwrap() == "orphan_file"
    });
    assert!(!has_orphan, "without --validate, should not have OrphanFile/DeadReExport errors");
}

// ── New lookup tests ────────────────────────────────────────────────────

#[test]
fn test_lookup_symbol_found() {
    let fixture = std::path::Path::new("tests/fixtures/sample-workspace");

    let output = Command::new(&binary_path())
        .arg("lookup")
        .arg(fixture)
        .arg("--symbol")
        .arg("Task")
        .output()
        .expect("failed to execute binary");

    assert!(output.status.success(), "should find Task symbol");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("output is not valid JSON");

    assert_eq!(json["status"].as_str().unwrap(), "found");
    assert!(json["crateName"].as_str().unwrap().is_empty() == false);
    assert_eq!(json["kind"].as_str().unwrap(), "struct");
}

#[test]
fn test_lookup_symbol_not_found() {
    let fixture = std::path::Path::new("tests/fixtures/sample-workspace");

    let output = Command::new(&binary_path())
        .arg("lookup")
        .arg(fixture)
        .arg("--symbol")
        .arg("DoesNotExist")
        .output()
        .expect("failed to execute binary");

    assert_eq!(
        output.status.code().unwrap(),
        1,
        "should exit 1 for not found symbol"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("output is not valid JSON");

    assert_eq!(json["status"].as_str().unwrap(), "not_found");
}

#[test]
fn test_lookup_file() {
    let fixture = std::path::Path::new("tests/fixtures/sample-workspace");

    let output = Command::new(&binary_path())
        .arg("lookup")
        .arg(fixture)
        .arg("--file")
        .arg("core/src/lib.rs")
        .output()
        .expect("failed to execute binary");

    assert!(output.status.success(), "should find core/src/lib.rs");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("output is not valid JSON");

    assert!(json.get("fileEntry").is_some(), "should have fileEntry");
    assert!(json.get("primaryModule").is_some(), "should have primaryModule");
}

// ── Serde regression test ───────────────────────────────────────────────

#[test]
fn test_orphaned_module_serde_regression() {
    // Verify DiagnosticKind::OrphanedModule serializes as "orphaned_module".
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    write_cargo_toml(root, r#"
[workspace]
members = ["."]

[package]
name = "orphan-mod-test"
version = "0.1.0"
edition = "2021"
"#);

    let src = root.join("src");
    std::fs::create_dir_all(&src).unwrap();

    // lib.rs with an undeclared module
    std::fs::write(src.join("lib.rs"), "mod nonexistent;").unwrap();

    let output = run_index(root.to_str().unwrap());
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("output is not valid JSON");

    let errors = extract_array(&json, "errors");
    let orphaned = errors.iter().find(|e| e["kind"].as_str().unwrap() == "orphaned_module")
        .expect("should have orphaned_module error (snake_case string)");

    assert_eq!(orphaned["severity"].as_str().unwrap(), "warning");
}
