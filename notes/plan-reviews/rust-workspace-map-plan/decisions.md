## Plan Review Decisions — rust-workspace-map-plan — 2026-04-27

### Design Assessment

The design is structurally sound. Module decomposition is correct — `src/main.rs`, `src/lib.rs`, `schema.rs`, `workspace.rs`, `cargo_info.rs`, `file_parser.rs`, `module_tree.rs`, `cross_refs.rs`, `render.rs` — with clean SRP boundaries, no circular dependencies, and a well-chosen functional style over `syn::visit::Visit`. The `syn`-based parsing + functional traversal + JSON output approach is the right fit for a deterministic codebase map. Seven gaps were identified covering undefined types, crate root detection, error handling, cfg-test inconsistency, private module behavior, path semantics, and CLI specification. All have been addressed via the amendments below.

### Deferred Item Decisions

No deferred items — no `deferred.md` files exist in the repository.

### Plan Amendments Applied

All 8 amendments from the architectural review were applied:

1. **Add missing types** — `FileInfo`, `Import`, `DepInfo`, `Config`, and `ErrorEntry` added to Step 1b schema types
2. **Crate target resolution** — new section after Step 3 specifying `src/lib.rs`/`src/main.rs`/both detection, `crateType` as enum, root as entry file path
3. **Error handling strategy** — added to Steps 1b and 4b: parse failures emit stderr warnings and continue, optional `errors` field on `WorkspaceMap`
4. **cfg-test position refined** — `#[cfg(test)] mod tests { ... }` contents skipped, module existence recorded with test marker; all other `#[cfg(...)]` items extracted normally; literal `cfg(test)` text match heuristic, not cfg evaluation
5. **Private module behavior** — private modules traversed and included; `visibility` is `"pub"` or `"private"` on `ModuleInfo`; `pub(crate)`/`pub(super)` reported as `"pub"` at the crate level
6. **CLI arguments** — `PATH` positional (required), `--output`/`-o` optional, added to Step 1a
7. **Path semantics** — all file paths relative to workspace root, specified in Step 7
8. **Workspace exclude** — `enumerate_members` respects `[workspace] exclude` key, added to Step 2
