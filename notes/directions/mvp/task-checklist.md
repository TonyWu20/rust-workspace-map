# MVP Directions Review: Task-by-Task Checklist

> Review of `notes/directions/mvp/draft-directions.json` against the six clarity/completeness criteria.
> Supporting documents consulted:
>   - `notes/directions/mvp/draft-elaboration.md`
>   - `notes/directions/mvp/deferred-and-patterns.md`
>   - `notes/architecture-current.md`
>   - Source files under `src/`
>
> Note: The file `notes/directions/mvp/codebase-state.md` referenced in the review request does not exist in the repository. The review relies on `architecture-current.md` and direct source inspection instead.

---

## T1: Add CanonicalPath and WorkspaceRelativePath newtypes

| Criterion | Assessment |
|---|---|
| Goal clear? | CLEAR -- "Add two newtype wrappers with all required trait implementations" |
| Files specified? | CLEAR -- `src/schema.rs` |
| Implementation detail sufficient? | CLEAR -- exact derive macros listed, trait impls enumerated (Display, AsRef, From, Serialize, Deserialize), placement specified (before the Error enum), and `#[serde(transparent)]` requirement stated |
| New types defined clearly? | CLEAR -- full type reference given in JSON: `#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)] pub struct CanonicalPath(#[serde(transparent)] pub String);` |
| Dependencies explicit? | CLEAR -- `depends_on: []` (none) |
| Verifiable? | CLEAR -- `cargo check` will verify the types compile |

**Verdict:** CLEAR

---

## T2: Add DiagnosticKind enum with 10 variants

| Criterion | Assessment |
|---|---|
| Goal clear? | CLEAR -- "Add DiagnosticKind enum with 10 variants replacing all existing string-error-kind values" |
| Files specified? | CLEAR -- `src/schema.rs` |
| Implementation detail sufficient? | CLEAR -- all 10 variants listed, serde `rename_all = "snake_case"` specified, placement near ErrorSeverity enum, instruction to NOT use per-variant renames |
| New types defined clearly? | CLEAR -- full type reference given |
| Dependencies explicit? | CLEAR -- `depends_on: []` (none) |
| Verifiable? | CLEAR -- `cargo check` |

**Verdict:** CLEAR

---

## T3: Add SymbolEntry and FileEntry structs

| Criterion | Assessment |
|---|---|
| Goal clear? | CLEAR -- "Add two new structs with bon::Builder derive" |
| Files specified? | CLEAR -- `src/schema.rs` |
| Implementation detail sufficient? | CLEAR -- exact fields, derives, serde attributes, and `skip_serializing_if` for `parent_module_file` all specified. Explicitly warns against adding `Confidence`, `ImportSite`, etc. |
| New types defined clearly? | CLEAR -- full type references given |
| Dependencies explicit? | CLEAR -- `depends_on: []` (none) |
| Verifiable? | CLEAR -- `cargo check` |

**Verdict:** CLEAR

---

## T4: Migrate ErrorEntry.kind, extend WorkspaceMap with flat index fields, extend Config with validate field

| Criterion | Assessment |
|---|---|
| Goal clear? | CLEAR -- three coordinated changes described |
| Files specified? | CLEAR -- `src/schema.rs` |
| Implementation detail sufficient? | CLEAR -- exact field types, `#[builder(default)]` placements, serde handling, and field ordering instructions given |
| New types defined clearly? | CLEAR -- type references for all four changed/added fields |
| Dependencies explicit? | CLEAR -- `depends_on: ["T1", "T2", "T3"]` |
| Verifiable? | CLEAR -- `cargo check` (schema-only changes) |

**Verdict:** CLEAR

---

## T5: Fix dropped-errors bug at lib.rs:132-137

| Criterion | Assessment |
|---|---|
| Goal clear? | CLEAR -- "Move crate_errors.extend(errs) before the if-let so errors are collected from both Some and None branches" |
| Files specified? | CLEAR -- `src/lib.rs` |
| Implementation detail sufficient? | CLEAR -- exact before/after code described, explains why no clone needed (move iteration) |
| New types defined? | N/A (no new types) |
| Dependencies explicit? | CLEAR -- `depends_on: []` (none) |
| Verifiable? | CLEAR -- `cargo check` + `cargo test --lib` |

**Verdict:** CLEAR

---

## T6: Fix O(n^2) errors.clone(), module-drop bug, and fragile unwrap_or

| Criterion | Assessment |
|---|---|
| Goal clear? | CLEAR -- four coordinated changes in module_tree.rs are individually described. However, this is the most complex single task: changing a return type, fixing a discarding bug, and fixing a path fallback. |
| Files specified? | CLEAR -- `src/module_tree.rs` |
| Implementation detail sufficient? | CLEAR -- each of the four changes has before/after logic. Line numbers (252, 183, 210, 33) are given but the descriptions are detailed enough that an implementer could find the right locations without exact line numbers. The `process_submodule` fix correctly identifies a pre-existing bug (returned modules were silently discarded). |
| New types defined? | N/A (signature change only) |
| Dependencies explicit? | CLEAR -- `depends_on: []` (none) |
| Verifiable? | CLEAR -- `cargo check` + `cargo test --lib` |

**Verdict:** CLEAR

---

## T7: Fix WorkspaceInfo.root from hardcoded "." to discovered workspace root

| Criterion | Assessment |
|---|---|
| Goal clear? | CLEAR -- "Change .root(\".\".to_string()) to use workspace_root.to_string_lossy().to_string()" |
| Files specified? | CLEAR -- `src/lib.rs` |
| Implementation detail sufficient? | CLEAR -- exact before/after, notes that `workspace_root` is already available in scope |
| New types defined? | N/A |
| Dependencies explicit? | CLEAR -- `depends_on: []` (none) |
| Verifiable? | CLEAR -- `cargo check` compiles. Behavioral verification (root is no longer ".") depends on T15 integration test updates, which is acknowledged in the guidance text |

**Verdict:** CLEAR

---

## T8: Migrate all .kind("string") emit-sites to DiagnosticKind enum variants

| Criterion | Assessment |
|---|---|
| Goal clear? | CLEAR -- "Replace all .kind(\"...\") string literals with .kind(DiagnosticKind::Variant)" |
| Files specified? | CLEAR -- `src/lib.rs`, `src/module_tree.rs`, `src/file_parser.rs` |
| Implementation detail sufficient? | CLEAR -- mapping tables given per file (e.g. "toml_parse_error" -> DiagnosticKind::TomlParseError, "orphaned_module" -> DiagnosticKind::OrphanedModule). Instructions to remove `.to_string()`. Import instructions given. |
| New types defined? | N/A (uses existing DiagnosticKind from T2) |
| Dependencies explicit? | CLEAR -- `depends_on: ["T4"]` (needs ErrorEntry.kind to be DiagnosticKind) |
| Verifiable? | CLEAR -- `cargo check` + `cargo test --lib` |

**Note:** The file `src/workspace.rs` is NOT in scope. This is correct -- source inspection confirms `workspace.rs` has no `.kind(...)` calls (it uses `anyhow::Error` propagation and `eprintln!`).

**Verdict:** CLEAR

---

## T9: Create src/indexes.rs with derive_from_crates() pure function

| Criterion | Assessment |
|---|---|
| Goal clear? | CLEAR -- "Create new module with derive_from_crates() that builds symbols, name_index, and files from CrateInfo" |
| Files specified? | CLEAR -- `src/indexes.rs` (new), `src/lib.rs` |
| Implementation detail sufficient? | CLEAR -- algorithm described for symbols/name_index (flat_map -> fold pipeline) and for files (for-loop with inline detection). Canonical path format specified: `format!("{}::{}", m.path, item.name)`. Inline module detection heuristic documented with dependency on depth-first ordering. Unit test cases enumerated. |
| New types defined clearly? | CLEAR -- exact function signature with type_reference: `#[must_use] pub fn derive_from_crates(crates: &[CrateInfo]) -> (BTreeMap<CanonicalPath, SymbolEntry>, BTreeMap<String, Vec<CanonicalPath>>, BTreeMap<WorkspaceRelativePath, FileEntry>)` |
| Dependencies explicit? | CLEAR -- `depends_on: ["T4"]` |
| Verifiable? | CLEAR -- `cargo check` + `cargo test --lib` (includes unit tests in the new module) |

**Note:** There is a contradiction between the directions.json and the enrichment document (`draft-elaboration.md`) on the BTreeMap key types. Directions.json (T4 and T9) consistently uses `CanonicalPath`/`WorkspaceRelativePath` as map keys. The elaboration.md (section A2) says maps use `String` keys. The directions.json is the authoritative implementation contract and is self-consistent. This contradiction does not block implementation but could confuse an implementer who reads both documents.

**Verdict:** CLEAR (with the documentation-consistency note above)

---

## T10: Re-key cross_references.types map to canonical paths

| Criterion | Assessment |
|---|---|
| Goal clear? | CLEAR -- "Change crate_exports keys from short names to canonical paths ({module.path}::{item.name}), update types_map initialization and imported_by detection" |
| Files specified? | CLEAR -- `src/cross_refs.rs` |
| Implementation detail sufficient? | CLEAR -- before/after code for crate_exports construction, types_map key change, imported_by suffix matching described. Test update guidance given (though specific assertion changes are left to the implementer's judgment -- the tests will fail and need fixing). |
| New types defined? | N/A |
| Dependencies explicit? | CLEAR -- `depends_on: []` (none) |
| Verifiable? | CLEAR -- `cargo check` + `cargo test --lib` (existing tests must pass after updates) |

**Potential issue:** T10 has no explicit dependencies and is in its own group (`group-cross-refs`) with no `depends_on_groups`. It can be done at any time. However, T15 (integration test rewrite) directly touches assertions about `crossReferences.types` keys, and those assertions must match whatever format T10 produces. **T10 is not listed as a dependency of T15.** If T15 runs before T10, the integration tests will assert against the wrong key format. The serial group ordering (group-cross-refs at index 8, group-integration-tests at index 13) makes this unlikely in practice, but the explicit dependency edge is missing.

**Verdict:** CLEAR (with the missing-dependency-edge note above)

---

## T11: Create src/validate.rs with OrphanFile and DeadReExport validation rules

| Criterion | Assessment |
|---|---|
| Goal clear? | CLEAR -- "Create validate() function with check_orphan_files and check_dead_reexports private helpers" |
| Files specified? | CLEAR -- `src/validate.rs` (new), `src/lib.rs` |
| Implementation detail sufficient? | CLEAR -- detailed algorithms for both rules. Orphan file detection: collect file set, walk src/, find undeclared .rs files. DeadReExport: parse import_path, resolve prefixes, skip external/glob, look up in symbols map. Conservative recall policy documented. |
| New types defined clearly? | CLEAR -- three function signatures given with type_reference: `validate()`, `check_orphan_files()`, `check_dead_reexports()` |
| Dependencies explicit? | CLEAR -- `depends_on: ["T9"]` (needs symbols map from derive_from_crates) |
| Verifiable? | CLEAR -- `cargo check` + `cargo test --lib` (includes unit tests with temp dirs) |

**Note:** The `check_dead_reexports` guidance says "parse the import_path" and "resolve prefixes (crate::, self::, super::)" but does not provide a precise algorithm for `self::` and `super::` resolution. The implementer needs to understand module path semantics to implement this correctly. The guidance is directionally correct but leaves some implementation discretion.

**Verdict:** CLEAR

---

## T12: Refactor main.rs to clap subcommands, remove bare-path form, extract build_map()

| Criterion | Assessment |
|---|---|
| Goal clear? | CLEAR -- "Replace flat CLI with subcommands, extract build_map() from run(), implement exit code logic" |
| Files specified? | CLEAR -- `src/main.rs`, `src/lib.rs` |
| Implementation detail sufficient? | CLEAR -- exact CLI struct definitions (Index, Lookup subcommands), build_map() extraction described (all pipeline logic up to WorkspaceMap builder, run() becomes thin wrapper), exit code precedence documented (error > validation > success) |
| New types defined clearly? | CLEAR -- Cli, Command, build_map signatures in type_reference |
| Dependencies explicit? | PARTIALLY CLEAR -- `depends_on: ["T9", "T11", "T5", "T7", "T8"]`. However, **T6 is missing** from the individual task dependency list, even though T6 is in the same task group (`group-bugfixes`) as T5 and T7. The group dependency (`group-cli-refactor` depends on `group-bugfixes`) is correct, so at the group level everything works. But an implementer looking only at individual task `depends_on` fields would see T5 and T7 listed but not T6, which is inconsistent. |
| Verifiable? | CLEAR -- `cargo check` + `cargo test --lib`. The known pitfalls section correctly notes that integration tests are deferred to T15. |

**Note:** The exit code 2 check in main.rs says "check the returned WorkspaceMap for any ErrorEntry where severity == Warning AND kind is OrphanFile or DeadReExport." This matches the known pitfalls section's precedence rules. However, note that the validate::validate() function already only produces Warning-severity entries with OrphanFile/DeadReExport kinds, so filtering in main.rs is a belt-and-suspenders approach. This is fine.

**Verdict:** CLEAR (with minor T6 dependency inconsistency)

---

## T13: Create src/lookup.rs with lookup_symbol() and lookup_file()

| Criterion | Assessment |
|---|---|
| Goal clear? | CLEAR -- "Create lookup module with two pure functions and wire into the lookup subcommand" |
| Files specified? | CLEAR -- `src/lookup.rs` (new), `src/main.rs`, `src/lib.rs` |
| Implementation detail sufficient? | CLEAR -- algorithms for both functions described. lookup_symbol: check name_index, dispatch to Found/Ambiguous/NotFound. lookup_file: check files index, scan modules for primary/inline partition. Result types defined with exact fields and serde attributes. |
| New types defined clearly? | CLEAR -- SymbolLookupResult (tagged enum), FileLookupResult, DisambiguationHint all defined with exact fields, derives, and serde attributes in type_reference |
| Dependencies explicit? | CLEAR -- `depends_on: ["T12", "T9"]` (needs CLI structure and flat indexes in WorkspaceMap) |
| Verifiable? | CLEAR -- `cargo check` + `cargo test --lib` (includes unit tests) |

**Verdict:** CLEAR

---

## T14: Create test fixtures (bad-orphan, bad-dead-reexport)

| Criterion | Assessment |
|---|---|
| Goal clear? | CLEAR -- "Create static test fixture directories with specific file contents" |
| Files specified? | CLEAR -- all 5 files listed with exact paths |
| Implementation detail sufficient? | CLEAR -- exact file contents described: Cargo.toml contents, lib.rs contents, forgotten.rs contents |
| New types defined? | N/A |
| Dependencies explicit? | CLEAR -- `depends_on: []` (none) |
| Verifiable? | CLEAR -- `ls` commands to check files exist |

**Verdict:** CLEAR

---

## T15: Rewrite integration tests for flag day, add new tests

| Criterion | Assessment |
|---|---|
| Goal clear? | CLEAR -- "Rewrite all 10 existing tests for index subcommand, add 7 new validate/lookup/regression tests" |
| Files specified? | CLEAR -- `tests/integration_test.rs` |
| Implementation detail sufficient? | CLEAR -- exact changes to existing tests described (prefix `"index"` before path, update root assertion, update crossReferences.types assertion). All 7 new test functions described with invocation, assertions, and expected behavior. |
| New types defined? | N/A |
| Dependencies explicit? | PARTIALLY CLEAR -- `depends_on: ["T12", "T13", "T14"]`. However, **T10 is not listed** as a dependency, even though T15 modifies the `crossReferences.types` assertion to account for T10's re-keying. The transitively through T12 does not include T10 (T12 depends on T9, T11, T5, T7, T8 -- not T10). The group-level ordering (T10 in group 8, T15 in group 13) makes the issue unlikely in practice, but the explicit dependency edge from T15 to T10 is absent. |
| Verifiable? | CLEAR -- multiple `cargo test` commands listed for individual tests |

**Verdict:** CLEAR (with missing T10 dependency note)

---

## T16: Update pipeline integration files (compile-plan, plan-decomposer)

| Criterion | Assessment |
|---|---|
| Goal clear? | CLEAR -- "Add authoritative gate to compile-plan SKILL.md and soft suggestion to plan-decomposer.md" |
| Files specified? | CLEAR -- `../rust-development-pipeline/skills/compile-plan/SKILL.md`, `../rust-development-pipeline/agents/plan-decomposer.md` |
| Implementation detail sufficient? | CLEAR -- exact content descriptions for both files: pre-check steps, exit code handling, soft recommendation language. |
| New types defined? | N/A |
| Dependencies explicit? | CLEAR -- `depends_on: ["T15"]` |
| Verifiable? | CLEAR -- `grep` commands to verify additions exist |

**Verdict:** CLEAR

---

## T17: Build release, dry-run against castep-cell-io, update README

| Criterion | Assessment |
|---|---|
| Goal clear? | CLEAR -- "Build release binary, run against real project, record baseline, update README" |
| Files specified? | CLEAR -- `README.md`, `notes/dry-run-baseline.md` |
| Implementation detail sufficient? | CLEAR -- README additions specified (usage update, scope documentation, inspiration section). Dry-run baseline content structure described (command, wall-time, file count, exit code, findings). |
| New types defined? | N/A |
| Dependencies explicit? | CLEAR -- `depends_on: ["T15"]` |
| Verifiable? | CLEAR -- `cargo build --release`, `time` command, `cat` to review output |

**Verdict:** CLEAR

---

## Summary of Issues Found

### Contradiction between directions.json and elaboration.md

The directions.json (T4, T9) consistently defines WorkspaceMap fields using newtype keys:
```
symbols: BTreeMap<CanonicalPath, SymbolEntry>
name_index: BTreeMap<String, Vec<CanonicalPath>>
files: BTreeMap<WorkspaceRelativePath, FileEntry>
```

The elaboration.md (section A2) says the opposite: "BTreeMaps in WorkspaceMap use String keys" and "They are NOT used in WorkspaceMap field types (which stay BTreeMap<String, ...>)."

This does not block implementation because the directions.json is the authoritative implementation contract and is internally self-consistent. However, an implementer who reads the elaboration.md for context will encounter conflicting information and may be confused about which to follow.

### Missing dependency edges (non-blocking)

1. **T15 depends on T10** (cross-refs re-keying): T15 explicitly modifies the `crossReferences.types` assertion to use canonical paths, but T10 is not listed in T15's `depends_on`. The group ordering makes this safe in serial execution, but the dependency is not explicitly recorded.

2. **T12 individual depends_on missing T6**: T12's `depends_on` lists `["T9", "T11", "T5", "T7", "T8"]` but not T6, even though T6 modifies `process_module_info`'s return type which affects the pipeline that `build_map()` orchestrates. The group-level dependency is correct (`group-cli-refactor` depends on `group-bugfixes` which includes T6).

### Line number fragility in T6

T6 references specific line numbers (252, 183, 210, 33) in `src/module_tree.rs`. While the descriptive guidance is detailed enough to find the right locations without the numbers, the line number references will drift if module_tree.rs is modified between the plan creation and implementation.

### T11 import path resolution detail

T11's `check_dead_reexports` guidance says "Parse the import_path. Resolve prefixes: 'crate::' -> this crate's name; 'self::' -> current module path; 'super::' -> parent module path." It does not specify how to handle multiple `super::` prefixes (e.g., `super::super::foo::Bar`) or mixed paths. The implementer will need to implement this resolution logic. The guidance is directionally correct but leaves implementation discretion.

### T8 scope correctly excludes workspace.rs

Source verification confirms `workspace.rs` has no `ErrorEntry.kind()` calls -- workplace errors propagate via `anyhow::Error` or `eprintln!` -- so the three-file scope (lib.rs, module_tree.rs, file_parser.rs) is correct.

---

## Overall Assessment

**Ready to Implement**

The `draft-directions.json` is exceptionally well-specified. All 17 tasks have clear goals, well-defined file scopes, detailed implementation guidance, and verifiable acceptance criteria. The task grouping, dependency ordering, and known pitfalls documentation are thorough.

The issues identified are:
- One documentation contradiction (elaboration.md vs directions.json on key types) that does not affect implementation correctness
- Two missing explicit dependency edges at the individual task level (T15 -> T10, T12 -> T6) that are handled correctly at the group level
- Minor implementation discretion areas (T6 line numbers, T11 import path resolution)

None of these are blockers. An implementer following the directions.json will produce a correct implementation.
