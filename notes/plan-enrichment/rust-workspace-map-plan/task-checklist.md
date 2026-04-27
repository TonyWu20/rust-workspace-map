# Task Checklist — rust-workspace-map Draft Plan

Review date: 2026-04-27  
Plan file: `/Users/tony/programming/rust-workspace-map/notes/plan-enrichment/rust-workspace-map-plan/draft-plan.toml`  
Codebase state: Greenfield (no Rust source files exist)

---

## TASK-1: Create Cargo.toml with all dependencies

Module wiring check (N/A — not a .rs file):
- pub mod in parent: Not applicable
- pub use re-export: Not applicable
- Consumer updates co-located: Not applicable

Known failure mode check:
- Missing pub mod risk: Low — not a .rs file
- Missing pub use risk: Low — not a .rs file
- Stale import risk: Low — no prior files to conflict with

Before-block check:
- Grep confirmed: Yes — no Cargo.toml exists in repo root
- Acceptance commands present: Yes — `test -f Cargo.toml`

Depends on: None

Notes:
- `walkdir = "2"` is declared in `[dependencies]` but never used anywhere in the plan. Not a clarity issue but wastes compile time.
- `cargo-llm-cov = "0.6"` is listed as a `[dev-dependency]` but `cargo-llm-cov` is a binary cargo subcommand, not a library crate. Putting it in dev-dependencies likely won't work (it will fail to link or silently do nothing). The acceptance command `test -f Cargo.toml` will pass regardless.
- Acceptance is minimal (file existence only, no content verification or `cargo check`).

---

## TASK-2: Create src/schema.rs with all data types, Error enum, and Result alias

Module wiring check (first .rs file):
- pub mod in parent: No — TASK-9 (lib.rs) will add `pub mod schema;`
- pub use re-export: No — TASK-9 adds `pub use schema::Config;`
- Consumer updates co-located: No — consumers are TASK-3, TASK-4, TASK-5, TASK-6, TASK-7, TASK-8, TASK-9

Known failure mode check:
- Missing pub mod risk: Medium — this file won't compile until TASK-9 provides `pub mod schema;` in lib.rs. If someone runs `cargo check` after this task but before TASK-9, it will fail.
- Missing pub use risk: Low — only `Config` is re-exported, and that's in TASK-9.
- Stale import risk: Low — greenfield project, no conflicting imports.

Before-block check:
- Grep confirmed: Yes — no src/schema.rs exists
- Acceptance commands present: Yes — `test -f src/schema.rs`

Depends on: None (implicitly depends on TASK-1 for crate dependencies serde, bon, thiserror, toml, syn — but this is NOT listed in the plan's [dependencies] section)

Notes:
- The plan's `[dependencies]` section omits `TASK-2 = ["TASK-1"]`. This is a missing explicit dependency. Executing sequentially (TASK-1 then TASK-2) works, but a dependency-aware scheduler might reorder incorrectly.
- `walkdir` is not imported or used in schema.rs (consistent with being unused everywhere).
- All types/interfaces are fully specified in the `after` block — no ambiguity.

---

## TASK-3: Create src/workspace.rs with find_workspace_root, enumerate_members, resolve_crate_roots

Module wiring check:
- pub mod in parent: No — TASK-9 (lib.rs) will add `pub mod workspace;`
- pub use re-export: No — functions are called path-qualified as `workspace::find_workspace_root(...)`
- Consumer updates co-located: No — consumers are TASK-9 (lib.rs) and TASK-4 (uses CrateType)

Known failure mode check:
- Missing pub mod risk: Medium — file won't be compiled until TASK-9 wires it
- Missing pub use risk: Low — no re-export needed
- Stale import risk: Low — imports from `crate::schema` which is defined in TASK-2; no changes expected

Before-block check:
- Grep confirmed: Yes — no src/workspace.rs exists
- Acceptance commands present: Yes — `test -f src/workspace.rs`

Depends on: TASK-2 (explicit), TASK-1 (implicit for serde types used via schema)

Notes:
- `enumerate_members` uses `glob::glob` which requires `glob = "0.3"` from TASK-1.
- The function correctly handles member glob patterns and the `exclude` list.
- `resolve_crate_roots` only checks `src/lib.rs` and `src/main.rs`. It does not check for other binary entry points (e.g., `src/bin/*.rs`). This is a conscious scope decision per the plan.

---

## TASK-4: Create src/cargo_info.rs with parse_cargo_toml

Module wiring check:
- pub mod in parent: No — TASK-9 (lib.rs) will add `pub mod cargo_info;`
- pub use re-export: No
- Consumer updates co-located: No — consumer is TASK-9 (lib.rs)

Known failure mode check:
- Missing pub mod risk: Medium — file won't be compiled until TASK-9
- Missing pub use risk: Low — no re-export needed
- Stale import risk: Low — imports from `crate::schema`, no prior changes to schema expected

Before-block check:
- Grep confirmed: Yes — no src/cargo_info.rs exists
- Acceptance commands present: Yes — `test -f src/cargo_info.rs`

Depends on: TASK-2 (explicit), TASK-1 (implicit for toml crate)

Notes:
- The `extract_deps` inner function is defined inside `parse_cargo_toml`. It correctly distinguishes workspace vs non-workspace deps.
- `workspace_members` combines deps from both `[dependencies]` and `[dev-dependencies]`.
- Acceptable code, no ambiguity.

---

## TASK-5: Create src/file_parser.rs with parse_file and all extractor functions

Module wiring check:
- pub mod in parent: No — TASK-9 (lib.rs) will add `pub mod file_parser;`
- pub use re-export: No
- Consumer updates co-located: No — consumer is TASK-6 (module_tree.rs)

Known failure mode check:
- Missing pub mod risk: Medium — file won't be compiled until TASK-9
- Missing pub use risk: Low — no re-export needed
- Stale import risk: Low — imports from `crate::schema`, stable

Before-block check:
- Grep confirmed: Yes — no src/file_parser.rs exists
- Acceptance commands present: Yes — `test -f src/file_parser.rs`

Depends on: TASK-2 (explicit), TASK-1 (implicit for syn crate)

Notes:
- `parse_file` returns `Result<(syn::File, FileInfo)>` but does NOT propagate syn parse errors as `Err` — it catches them, prints a warning, and returns `Ok((empty, FileInfo::default()))`. The `Result` only propagates IO errors. This is intentional but could surprise the executor if they expect the `SynParse` variant to be used. The function signature is accurate.
- Helper functions (`flatten_use_tree`, `extract_re_exports_from_tree`, etc.) are comprehensive and handle all path forms (Name, Rename, Glob, Group).
- `type_to_string` handles ~15 type forms — very complete.
- 590+ lines of code, largest file in the plan. The `after` block is fully specified — exact code to write.

---

## TASK-6: Create src/module_tree.rs with build_module_tree and resolve_module_path

Module wiring check:
- pub mod in parent: No — TASK-9 (lib.rs) will add `pub mod module_tree;`
- pub use re-export: No
- Consumer updates co-located: No — consumer is TASK-9 (lib.rs)

Known failure mode check:
- Missing pub mod risk: Medium — file won't be compiled until TASK-9
- Missing pub use risk: Low — no re-export needed
- Stale import risk: Low — imports from `crate::file_parser` (TASK-5) and `crate::schema` (TASK-2). Both exist when executed in order.

Before-block check:
- Grep confirmed: Yes — no src/module_tree.rs exists
- Acceptance commands present: Yes — `test -f src/module_tree.rs`

Depends on: TASK-2, TASK-5 (both explicit), TASK-1 (implicit for syn)

Notes:
- The plan says TASK-6 depends on TASK-5 (file_parser), which is correct.
- Cycle detection uses a `HashSet<PathBuf>` of visited files — returns empty vec on cycle.
- Handles inline modules (`mod foo { ... }`) and external modules (`mod foo;`).
- `process_module_items` re-extracts items from inline module bodies — uses `file_parser::extract_*` functions directly.
- `FileInfo.impls` are extracted by file_parser but NEVER threaded into `ModuleInfo`. The `ModuleInfo` struct has no `impls` field. This means impl blocks are parsed but silently discarded. This is a consistency gap (code does work that is thrown away), but the plan specifies this exact behavior — the executor would just implement what's written.

---

## TASK-7: Create src/cross_refs.rs with compute function

Module wiring check:
- pub mod in parent: No — TASK-9 (lib.rs) will add `pub mod cross_refs;`
- pub use re-export: No
- Consumer updates co-located: No — consumer is TASK-9 (lib.rs)

Known failure mode check:
- Missing pub mod risk: Medium — file won't be compiled until TASK-9
- Missing pub use risk: Low — no re-export needed
- Stale import risk: Low — imports from `crate::schema`

Before-block check:
- Grep confirmed: Yes — no src/cross_refs.rs exists
- Acceptance commands present: Yes — `test -f src/cross_refs.rs`

Depends on: TASK-2 (explicit), TASK-1 (implicit)

Notes:
- `compute` takes `&mut [CrateInfo]` to mutate `cross_crate_imports` on each crate. This differs from the description in `codebase-state.md` which says `&[CrateInfo]`. The plan TOML `after` block is authoritative and shows `&mut [CrateInfo]`.
- The cross-reference matching heuristic assumes the first path segment of an import is a crate name and checks it against `crate_exports.keys()`. This works for direct crate imports like `use core::Task` but may produce false negatives for deeply re-exported paths or crate names that happen to match type names.
- Defines `kind_to_string()` on `PublicItem` via a local `impl` block outside the module — this is fine within the same crate.

---

## TASK-8: Create src/render.rs with render_json and render_to_writer

Module wiring check:
- pub mod in parent: No — TASK-9 (lib.rs) will add `pub mod render;`
- pub use re-export: No
- Consumer updates co-located: No — consumer is TASK-9 (lib.rs)

Known failure mode check:
- Missing pub mod risk: Medium — file won't be compiled until TASK-9
- Missing pub use risk: Low — no re-export needed
- Stale import risk: Low — imports from `crate::schema`, uses `std::io::Write`

Before-block check:
- Grep confirmed: Yes — no src/render.rs exists
- Acceptance commands present: Yes — `test -f src/render.rs`

Depends on: TASK-2 (explicit), TASK-1 (implicit for serde_json)

Notes:
- Clean, simple file. Two functions, no ambiguity.
- `render_to_writer` takes `impl Write` (not `impl Write + ?Sized`) which is the idiomatic signature.
- No custom serialization logic — delegates to serde_json.

---

## TASK-9: Create src/lib.rs with module declarations and run() orchestration

Module wiring check (this is the wiring module itself):
- pub mod in parent: Not applicable (lib.rs is crate root)
- pub use re-export: Yes — `pub use schema::Config;`
- Consumer updates co-located: Yes — TASK-10 (main.rs) imports `rust_workspace_map::Config` and `rust_workspace_map::run`

Known failure mode check:
- Missing pub mod risk: High for prior tasks — if any of TASK-3 through TASK-8 are missing, `cargo check` will fail. This is by design since TASK-9 depends on all of them.
- Missing pub use risk: Low — only `Config` is re-exported; if it's missing, main.rs will fail (TASK-10 catches this).
- Stale import risk: Medium — if any module file is renamed or its signatures change after TASK-9, compilation breaks. But this is a final wiring task.

Before-block check:
- Grep confirmed: Yes — no src/lib.rs exists
- Acceptance commands present: Yes — `cargo check` (stronger than file existence)

Depends on: TASK-1, TASK-2, TASK-3, TASK-4, TASK-5, TASK-6, TASK-7, TASK-8 (all explicit)

Notes:
- This is the first task with `cargo check` as acceptance. All prior tasks only check `test -f`. The real compilation verification starts here.
- `run()` returns `anyhow::Result<()>`, not `schema::Result<()>`. This is correct — it wraps multiple error sources.
- `relativize_path` is a private helper that strips the workspace root prefix from paths.
- The `run()` function processes members in parallel via `rayon::prelude::*`, collects into `crate_infos`, then sorts deterministically by name.
- Path relativization is applied inside the `par_iter().filter_map()` closure which takes `&member_dirs`. The closure borrows `&workspace_root` — this compiles because `workspace_root` is not mutated during iteration. Verified correct.

---

## TASK-10: Create src/main.rs with clap CLI and main() entry point

Module wiring check (binary crate, separate from library):
- pub mod in parent: Not applicable (main.rs is the binary entry point)
- pub use re-export: Not applicable
- Consumer updates co-located: Not applicable (no other module consumes main.rs)

Known failure mode check:
- Missing pub mod risk: Low — main.rs is a binary entry point, not a module
- Missing pub use risk: Low — no re-exports
- Stale import risk: Low — imports `rust_workspace_map::Config` and `rust_workspace_map::run` from TASK-9

Before-block check:
- Grep confirmed: Yes — no src/main.rs exists
- Acceptance commands present: Yes — `cargo check`

Depends on: TASK-9 (explicit), TASK-1 (implicit for clap, anyhow)

Notes:
- Uses `std::path::absolute` (stable since Rust 1.79).
- The binary is auto-discovered by Cargo because the package name in Cargo.toml matches the default binary name.
- Clean, minimal file. No ambiguity.

---

## TASK-11: Create integration test fixture workspace and integration test

Module wiring check (test files, not library modules):
- pub mod in parent: Not applicable — tests/ directory is a separate compilation unit
- pub use re-export: Not applicable
- Consumer updates co-located: Not applicable — these are standalone test files

Known failure mode check:
- Missing pub mod risk: Low — test files are discovered by Cargo automatically when in `tests/`
- Missing pub use risk: Low — no re-exports
- Stale import risk: Medium — the test calls the binary via `env!("CARGO_BIN_EXE_rust-workspace-map")`. This env var name uses hyphens, but Cargo converts hyphens to underscores in env var names. The env var would actually be `CARGO_BIN_EXE_rust_workspace_map`. **This is a confirmed bug** — the test will fail to compile.

Before-block check:
- Grep confirmed: Yes — no tests/ directory exists
- Acceptance commands present: Yes — `cargo test --test integration_test`

Depends on: TASK-9, TASK-10 (both explicit)

Notes:
- **CONFIRMED BUG**: The `env!("CARGO_BIN_EXE_rust-workspace-map")` call will fail at compile time. Cargo sets `CARGO_BIN_EXE_<name>` where hyphens in the binary name are replaced with underscores per Cargo documentation. The correct invocation should be `env!("CARGO_BIN_EXE_rust_workspace_map")`.
- The fixture workspace is well-structured: two crates (core and engine) with cross-crate dependencies, re-exports, `#[cfg(test)]` blocks, and a submodule.
- The integration tests cover three scenarios: basic output shape, deterministic output (byte-identical across runs), and nonzero exit on missing path.
- The fixture workspace Cargo.tomls use `edition = "2021"` while the main Cargo.toml uses `edition = "2024"` — this is fine since the tool and the analyzed crates can have different editions.
- The engine crate's `Cargo.toml` has `core = { path = "../core" }` which is a path dependency (not a workspace member dependency). The `resolve_crate_roots` and module tree building should still work correctly with path deps.

---

## Summary of issues found

### Missing explicit dependency:
- TASK-2 depends on TASK-1 (crate dependencies: serde, bon, thiserror, toml, syn) but this is not declared in the `[dependencies]` section. Sequential execution is safe, but dependency-aware scheduling could misorder.

### Weak acceptance criteria (TASK-1 through TASK-8):
- Only `test -f <filename>` — no compilation check until TASK-9. If a task produces a file with syntax errors, they won't be caught until TASK-9.

### Confirmed bug:
- **TASK-11**: `env!("CARGO_BIN_EXE_rust-workspace-map")` uses hyphens, but Cargo converts hyphens to underscores in env var names. Must be `CARGO_BIN_EXE_rust_workspace_map`. This will cause a compile-time error when running the integration tests.

### Non-critical observations:
- `walkdir = "2"` in TASK-1's `[dependencies]` is unused.
- `cargo-llm-cov = "0.6"` in `[dev-dependencies]` is a binary subcommand crate, likely not a valid library dependency.
- `FileInfo.impls` are extracted by file_parser (TASK-5) but never used by module_tree (TASK-6) — the data is silently discarded. The `ModuleInfo` struct has no `impls` field.
- TASK-7's `compute` function uses a heuristic (first path segment = crate name) for cross-crate import detection, which may have false negatives.

---

RESULT: task-checklist.md saved. Tasks checked: 11. Wiring issues flagged: 0. Before-block unverified: 0.
