# Draft Elaboration — mvp-fp-cleanup

> Generated: 2026-04-30 | Source: PHASE_PLAN.md (reviewed), decisions.md, deferred-and-patterns.md, codebase-state.md, architecture-current.md
>
> This document elaborates the reviewed phase plan into precise architectural decisions,
> crate/module boundary assignments, type signatures, pattern requirements, pitfall
> enumerations, and task grouping rationale. It is the input to the directions.json
> generation step.

---

## 1. Crate Boundary Decisions

The project is a single crate (`rust-workspace-map`). No new crates are introduced.
Module assignments for all changes:

| Change | Module | Rationale |
|--------|--------|-----------|
| G1: prefix-aware external-crate check | `src/validate.rs` | Lives in `check_dead_reexports` — same function that iterates re-exports |
| G2: `derive_attrs` field on `SymbolEntry` | `src/schema.rs` | All data types reside here per the "one schema file" convention |
| G2: populate `derive_attrs` during index construction | `src/indexes.rs` | `derive_from_crates` already builds `SymbolEntry` from `PublicItem`; the copy is a one-liner in the fold closure |
| G2: prefix-decomposition heuristic | `src/validate.rs` | New helper function + call in `check_dead_reexports` DeadReExport branch |
| G3: `determine_parent_file` path fix | `src/validate.rs` | Direct replacement in the existing function |
| G4: deep-orphan fixture | `tests/fixtures/bad-orphan/` | Extends existing fixture; no new fixture directory needed |
| G4: recursive-orphan test | `tests/integration_test.rs` | Follows existing `test_validate_*` pattern |
| G2: README documentation | `README.md` | Under "Validation rules" section |

**No new modules.** All new logic fits in existing files. The validate.rs module grows by ~40 lines total (G1: 15-20 lines, G2: ~20 lines helper + call site, G3: ~5 lines replacement). Unit tests for G1/G2 add ~80 lines in the existing `#[cfg(test)] mod tests` block.

---

## 2. Design Decisions Per Goal

### 2.1 Goal 1 — Prefix-Aware External-Crate Check

**Decision:** Add a pre-check at the top of the per-re-export loop in `check_dead_reexports`, before `resolve_import_path` is called. The check inspects the **original** `re_export.import_path` as written in source code, not the resolved canonical form.

**Mechanism — three-case dispatch on path prefix:**

1. **Bare path** (no `::` prefix, e.g., `serde::Serialize`, `std::collections::HashMap`):
   - Extract the first `::`-delimited segment.
   - Check against `crate_names` (workspace member names) AND against top-level modules of the current crate.
   - Top-level modules are those in `crate_info.modules` where `module.path == format!("{crate_name}::{segment}")`.
   - If the segment matches a workspace member other than self → it's a cross-workspace re-export (not dead, skip).
   - If the segment matches a top-level module of self → it's internal (proceed to symbol lookup).
   - If neither → it's an external crate re-export (skip — not dead).
   - **Critical:** `crate_names` alone is insufficient. A bare path like `use utils::Helper` where `utils` is both a workspace member AND a top-level module of the current crate must be treated as internal (false positive if skipped). The module check covers this.

2. **`crate::` prefix** (e.g., `crate::sub::Type`, `crate::utils::Helper`):
   - Strip the `crate::` prefix.
   - The path is always internal — **never skip**.
   - Proceed directly to `resolve_import_path` (which already handles `crate::`).
   - Rationale: `crate::` is a compile-time guarantee of internal resolution. Checking further segments against module names is unnecessary because a `crate::nonexistent::Type` will fail the symbol-index lookup and correctly produce a DeadReExport finding.

3. **`self::` / `super::` prefix**:
   - Always internal — **skip the external-crate check entirely**.
   - Proceed to `resolve_import_path`.
   - Rationale: these prefixes can never resolve to an external crate.

**Placement in control flow:**

```
for re_export in &module.re_exports {
    let path = &re_export.import_path;

    // skip glob re-exports (existing)
    if path.ends_with("::*") { continue; }

    // ── NEW: G1 pre-check ──
    if is_external_crate_re_export(path, &crate_names, crate_info) {
        continue;
    }

    // existing: resolve and lookup
    let resolved = resolve_import_path(path, my_name, &module.path);
    let first_seg = resolved.split("::").next()...;  // existing cross-crate check
    // ...
}
```

**Note on existing cross-crate check (lines 148-151):** The existing code already does `first_seg in crate_names && first_seg != my_name`. This check fires on resolved paths (e.g., `mycrate::serde::Serialize`), where `mycrate` matches `my_name` and the check is a no-op. After G1, external re-exports never reach this point. The existing check remains as a safety net for cross-workspace-member re-exports that pass the G1 pre-check (e.g., `use other_workspace_member::Type` without a prefix). **No code removal is needed for coexistence.**

**New function signature:**

```rust
/// Check if a re-export path targets an external crate (not a workspace member
/// or internal module). Returns true if the re-export should be skipped.
fn is_external_crate_re_export(
    import_path: &str,
    crate_names: &HashSet<&str>,
    crate_info: &CrateInfo,
) -> bool
```

**Determinism:** No impact — the check is a pure predicate, no collection mutation, no ordering dependency.

---

### 2.2 Goal 2 — Prefix-Decomposition Heuristic (Derive-Companion Awareness)

**Decision:** When DeadReExport fires on a name `X` (not found in the symbol index), attempt to decompose `X` into a longest prefix `P` such that `P` exists as a symbol in the **same module** and has non-empty `derive_attrs`. If found, suppress the finding.

**Design rationale (for README inclusion):**

- Zero hardcoded derive names (no `bon::Builder`, no `thiserror::Error`, no suffix list).
- Zero maintenance burden — any new ecosystem derive that generates named types is automatically handled.
- Signal is purely structural: a re-exported name that is a suffix-extension of a knowable base type in the same module, where that base type carries `#[derive(...)]`.
- Conservative-recall policy: the tool prefers a false negative (suppressing a genuinely-dead re-export whose base type coincidentally has derives) over a false positive (flagging every derive-generated companion). The genuinely-dead-with-derives-on-base case is vanishingly rare and less harmful than 283 false positives.
- `serde::Serialize`, `serde::Deserialize`, `thiserror::Error`, and most ecosystem derives never enter this code path — they generate trait/method impls only, not new named types. The heuristic is thus primarily triggered by `bon::Builder` and similar companion-type-generating proc macros.

**Mechanism:**

1. DeadReExport fires on name `N` (last `::` segment of the canonical path, e.g., `CellDocumentBuilder`).
2. Iterate decreasing prefix lengths of `N`: `len(N)`, `len(N)-1`, ..., `1`.
3. For each prefix `P`, construct a lookup key: `CanonicalPath(format!("{}::{}", module.path, P))`.
4. Look up in `symbols` (the `BTreeMap<CanonicalPath, SymbolEntry>`).
5. First hit: if `symbol.derive_attrs.is_empty()` → no suppression, continue to shorter prefixes. If `symbol.derive_attrs` is non-empty → suppress the DeadReExport (the re-export targets a derive-generated companion type).
6. If no prefix matches at all → the finding stands (genuinely dead re-export).

**Same-module scoping critical invariant:** The lookup key uses `module.path` of the **re-export's module** (e.g., `"mycrate::sub"`), not a global search. A `pub use` of `FooBuilder` in module `bar` only checks symbols in module `bar`. This prevents cross-module false matches where a `Foo` in a different module coincidentally shares a prefix with `FooBuilder`.

**Known limitation (private-base-type):** Private structs with `#[derive(bon::Builder)]` generating a public builder. The base type `CellDocument` (private) is absent from the public-only symbol index, so prefix-decomposition cannot find it. The DeadReExport finding on `pub use CellDocumentBuilder` will stand. This is an accepted false-negative edge case — documented in README.

**New function signature:**

```rust
/// Attempt to suppress a DeadReExport finding via prefix-decomposition.
///
/// Given a re-export target name (e.g., "CellDocumentBuilder"), iterates
/// decreasing prefixes of the name, looking for a base type in the same module
/// that has non-empty `derive_attrs`. Returns true if the finding should be
/// suppressed.
fn is_derive_companion(
    target_name: &str,
    module_path: &str,
    symbols: &BTreeMap<CanonicalPath, SymbolEntry>,
) -> bool
```

**Data flow changes:**

1. `src/schema.rs` — add field to `SymbolEntry`:
   ```rust
   pub struct SymbolEntry {
       pub crate_name: String,
       pub module: String,
       pub file: String,
       pub line: usize,
       pub kind: ItemKind,
       // ── NEW ──
       #[builder(default)]
       #[serde(skip_serializing_if = "Vec::is_empty")]
       pub derive_attrs: Vec<String>,
   }
   ```

2. `src/indexes.rs` — populate in the fold closure (line 56-62):
   ```rust
   SymbolEntry::builder()
       .crate_name(crate_name.to_string())
       .module(m.path.clone())
       .file(m.file.clone())
       .line(item.line)
       .kind(item.kind.clone())
       .derive_attrs(item.attrs.derive.clone())  // NEW: straight copy
       .build(),
   ```

   **Verification:** `item.attrs.derive` is already `Vec<String>` (extracted by `extract_attrs` in `file_parser.rs:456-483`). No AST re-extraction needed.

3. `src/validate.rs` — call `is_derive_companion` in the DeadReExport branch:
   ```rust
   if !symbols.contains_key(&canonical) {
       // NEW: G2 prefix-decomposition check
       let target_name = canonical.0.rsplit("::").next().unwrap_or(&canonical.0);
       if is_derive_companion(target_name, &module.path, symbols) {
           continue; // suppress — derive-generated companion type
       }
       // existing: push DeadReExport ErrorEntry
   }
   ```

**Performance:** O(name_length * hashmap lookup) per DeadReExport. Name lengths are typically <30 chars. Even with 283 findings, total operations are <10,000 hash lookups — negligible.

**Edge case — name equals prefix (len 0):** The iteration starts at `len(name)` and wraps-around to `1`. If a type is literally named `Builder` with no longer base, the prefix `Builder` itself is checked. If `Builder` has derives, the finding is suppressed (correct — `pub use Builder` where `Builder` has derives is itself a derive-generated type re-exporting itself). If `Builder` has no derives, the prefix `Buil`, `Bui`, ... down to `B` are checked. Unlikely to match. The finding stands (correct).

---

### 2.3 Goal 3 — `determine_parent_file` Path Fix

**Decision:** Replace the hardcoded `strip_prefix("src/")` with a prefix derived from `crate_info.root.join("src").parent()`.

**Why `crate_info.root` is the correct anchor:** `crate_info.root` is a workspace-relative path (e.g., `crates/mylib/src/lib.rs` or `src/lib.rs`). The `src/` directory is `crate_info.root`'s parent directory. The `orphan_file` parameter is also workspace-relative (stripped by `path.strip_prefix(workspace_root)` in `check_orphan_files`). Stripping the `src/` directory prefix from the orphan file yields the crate-relative path within `src/`.

**Before (broken for non-root members):**
```rust
let stripped = orphan_file.strip_prefix("src/").unwrap_or(orphan_file);
```
For `crates/mylib/src/sub/deep.rs`, this fails to strip because `orphan_file` is `crates/mylib/src/sub/deep.rs`, which does not start with `src/`. Falls through to `unwrap_or(orphan_file)`, so `stripped = "crates/mylib/src/sub/deep.rs"`. Then `rsplit_once('/')` yields `dir = "crates/mylib/src/sub"`, producing parent file `"mylib/src/crates/mylib/src/sub/mod.rs"` — nonsensical.

**After (correct for all crate roots):**
```rust
let crate_src = crate_info.root.parent().unwrap_or(Path::new(""));
// crate_info.root = "crates/mylib/src/lib.rs"
// crate_src = "crates/mylib/src"
let parent_path = orphan_file.strip_prefix(crate_src.join("")).unwrap_or(orphan_file);
// Strips "crates/mylib/src/" prefix → "sub/deep.rs"
```

**Actual implementation — use `Path` operations for portability:**
```rust
fn determine_parent_file(
    orphan_file: &str,
    crate_name: &str,
    crate_info: &CrateInfo,
) -> String {
    let orphan_path = Path::new(orphan_file);
    let root_path = Path::new(&crate_info.root);
    // Derive src/ directory from crate root file
    let crate_src = root_path.parent().unwrap_or(Path::new(""));
    let relative = orphan_path.strip_prefix(crate_src).unwrap_or(orphan_path);
    let parent_dir = relative.parent();
    let file_stem = relative.file_stem();
    // ... rest of logic adapted from Path operations
}
```

**Wait — closer inspection of existing logic:** The current code uses string operations (`strip_prefix`, `rsplit_once`). Switching to `Path` operations is more robust but changes the implementation style. The simpler fix is:

```rust
// Derive the src/ prefix from crate_info.root
let root_path = Path::new(&crate_info.root);
let src_dir = root_path.parent().unwrap_or(Path::new(""));
let src_prefix = format!("{}/", src_dir.display());
let stripped = orphan_file.strip_prefix(&src_prefix).unwrap_or(orphan_file);
```

This preserves the string-based approach while making the prefix crate-relative. The `unwrap_or` fallback to `orphan_file` handles the case where the orphan file isn't under the expected `src/` directory (defensive, should not occur in practice for valid crate structures).

**Why not use `crate_info.root.join("src")`:** The phase plan suggested `crate_info.root.join("src")`, but `crate_info.root` already points to a file inside `src/` (e.g., `src/lib.rs`). Joining `src` would produce `src/lib.rs/src`. The correct derivation is `crate_info.root.parent()` to get the `src/` directory itself.

---

### 2.4 Goal 4 — Recursive Orphan Integration Test

**Decision:** Extend `tests/fixtures/bad-orphan/` with a deeply nested orphan file and add an integration test that verifies it is detected.

**Fixture structure after G4:**
```
tests/fixtures/bad-orphan/
  Cargo.toml              (unchanged)
  src/
    lib.rs                (unchanged — declares no submodules)
    forgotten.rs          (existing — shallow orphan)
    sub/
      mod.rs              (NEW — declared in lib.rs: `pub mod sub;`)
      deeper_orphan.rs    (NEW — NOT declared, orphan at depth 1)
      deep/
        orphan.rs         (NEW — NOT declared, orphan at depth 2)
```

Wait — the existing `bad-orphan` fixture has `lib.rs` that declares no submodules (that's what makes `forgotten.rs` an orphan). To test depth > 1, we need a declared module at depth 0 that has further subdirectories. So:

**Corrected fixture design:**
```
tests/fixtures/bad-orphan/
  Cargo.toml              (unchanged)
  src/
    lib.rs                (MODIFIED: add `pub mod sub;`)
    forgotten.rs          (existing — shallow orphan, still detected)
    sub/
      mod.rs              (NEW: declares nothing beyond its own items)
      legal.rs            (NEW: used and declared — not an orphan)
      deep/
        orphan.rs         (NEW: file at depth 2, NOT declared anywhere)
```

**lib.rs changes:**
```rust
// Original lib.rs content preserved, plus:
pub mod sub;
```

**sub/mod.rs:**
```rust
// This module is declared — legal
pub mod legal;
pub fn helper() -> u32 { 42 }
```

**sub/legal.rs:**
```rust
pub fn do_thing() -> bool { true }
```

**sub/deep/orphan.rs:**
```rust
// This file is NOT declared in any mod statement — orphan at depth 2
pub fn orphan_func() -> &'static str { "I am an orphan" }
```

**Integration test:**
```rust
#[test]
fn test_recursive_orphan_detection() {
    let fixture = std::path::Path::new("tests/fixtures/bad-orphan");
    let output = Command::new(&binary_path())
        .arg("index")
        .arg("--validate")
        .arg(fixture)
        .output()
        .expect("failed to execute binary");

    assert_eq!(output.status.code().unwrap(), 2, "validation should exit 2");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("output is not valid JSON");
    let errors = extract_array(&json, "errors");

    // Should detect both shallow and deep orphans
    let orphan_errors: Vec<_> = errors.iter()
        .filter(|e| e["kind"].as_str().unwrap() == "orphan_file")
        .collect();

    assert!(!orphan_errors.is_empty(), "should find orphan files");

    // Verify the deep orphan is detected
    let has_deep_orphan = orphan_errors.iter().any(|e| {
        e["file"].as_str().unwrap().contains("deep/orphan.rs")
    });
    assert!(has_deep_orphan, "should detect orphan at depth 2");
}
```

**Why this tests depth > 1:** Without `walkdir` (i.e., with the pre-fix `read_dir` approach), only `src/` would be scanned. The `sub/` directory would be entered only if `sub/mod.rs` explicitly declared it (which it doesn't for `deep/`). With `walkdir`, the recursive walk descends into `sub/deep/` regardless and finds `orphan.rs`. This test proves the recursive walk works at depth 2+.

**Important:** Modifying `lib.rs` to add `pub mod sub;` means the existing `forgotten.rs` still shows as an orphan, but now `sub/` is legitimately entered. The existing test that asserts `forgotten.rs` is an orphan should still pass — verify during implementation.

---

## 3. Pattern Requirements

All changes must conform to existing conventions observed in the codebase:

### 3.1 Deterministic Output

- All `BTreeMap`-based collections are sorted by key.
- Unit test assertions must not depend on HashMap iteration order.
- New `derive_attrs` field: `#[serde(skip_serializing_if = "Vec::is_empty")]` suppresses it when empty.
- `derive_attrs` is already sorted by `extract_attrs` (line 481: `derive.sort()`). No additional sorting needed.

### 3.2 Serde Conventions

- `#[serde(rename_all = "camelCase")]` on all serialized structs. `SymbolEntry` does NOT have this attribute — it uses the default Rust field naming (snake_case). Verify that `derive_attrs` serializes as `derive_attrs` and `deriveAttrs` is NOT needed. (Check: the existing fields `crate_name`, `module`, `file`, `line`, `kind` all serialize as-is — snake_case.)
- `#[serde(skip_serializing_if = "Vec::is_empty")]` on all `Vec` fields.
- `#[serde(skip_serializing_if = "Option::is_none")]` on all `Option` fields.

### 3.3 Builder Pattern

- `SymbolEntry` uses `bon::Builder`. The new field needs `#[builder(default)]`.
- All `ErrorEntry` construction in `check_orphan_files` and `check_dead_reexports` uses the builder pattern with `.builder()...build()`.

### 3.4 Unit Test Patterns

- Unit tests live in `#[cfg(test)] mod tests` within the same file.
- Test functions are annotated `#[test]` (not `#[tokio::test]` — no async in this crate).
- Helper functions for constructing test data inline (minimal fixtures, no file I/O for validate.rs unit tests).
- Amendment B specifies 5 categories of unit tests for G1 and G2:
  1. bare-path external skip
  2. `crate::` path proceed
  3. companion-type suppressed
  4. companion-type NOT suppressed when base has no derives
  5. cross-module no-false-suppress

### 3.5 Integration Test Patterns

- All integration tests invoke the compiled binary via `std::process::Command`.
- Tests use `binary_path()` helper, `extract_array()` for JSON traversal.
- Assertions check exit codes AND JSON content.
- Tests targeting `--validate` assert exit code 2 (validation findings present).

### 3.6 Error Handling Conventions

- `#[must_use]` on all public functions that return `Vec<ErrorEntry>` or `Result`.
- New helper functions (`is_external_crate_re_export`, `is_derive_companion`) are pure predicates returning `bool` — no `#[must_use]` needed.
- No panics in new code. Use early returns and conditionals.

### 3.7 Naming Conventions

- Functions: `snake_case`, verb-led for predicates (`is_*`, `check_*`).
- New fields: `snake_case`, match existing naming in struct.
- Test functions: `test_<scenario_description>`.
- Variables: descriptive, no single-letter names except loop indices.

---

## 4. Key Type Signatures

### 4.1 New types / type changes

**`SymbolEntry` — added field:**
```rust
#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
pub struct SymbolEntry {
    pub crate_name: String,
    pub module: String,
    pub file: String,
    pub line: usize,
    pub kind: ItemKind,
    // NEW
    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub derive_attrs: Vec<String>,
}
```

### 4.2 New functions

**G1:**
```rust
/// Check if `import_path` targets an external crate.
///
/// Examines the original (unresolved) path prefix:
/// - Bare paths: check first segment against workspace members + top-level modules
/// - `crate::` paths: always internal
/// - `self::` / `super::` paths: always internal
fn is_external_crate_re_export(
    import_path: &str,
    crate_names: &HashSet<&str>,
    crate_info: &CrateInfo,
) -> bool
```

**G2:**
```rust
/// Check if `target_name` (last segment of a canonical path) is a derive-generated
/// companion type by decomposing it into prefixes and checking for base types with
/// non-empty `derive_attrs` in the same module.
fn is_derive_companion(
    target_name: &str,
    module_path: &str,
    symbols: &BTreeMap<CanonicalPath, SymbolEntry>,
) -> bool
```

### 4.3 Modified functions

**G3 — `determine_parent_file`:**
```rust
/// Before:
fn determine_parent_file(
    orphan_file: &str,
    crate_name: &str,
    _crate_info: &CrateInfo,  // unused
) -> String

/// After:
fn determine_parent_file(
    orphan_file: &str,
    crate_name: &str,
    crate_info: &CrateInfo,   // now used — underscore removed
) -> String
```
Signature unchanged other than `_crate_info` becoming `crate_info`. Internal implementation changes (replaces hardcoded `"src/"` with `crate_info.root`-derived prefix).

**G1/G2 — `check_dead_reexports`:** Signature unchanged. Internal logic gains G1 pre-check call + G2 prefix-decomposition call in the `!symbols.contains_key` branch.

---

## 5. Known Pitfalls and Constraints

### 5.1 Structural Invariants (DO NOT BREAK)

1. **Deterministic output:** All collections in output must be sorted. `derive_attrs` is pre-sorted by `extract_attrs`. No new unsorted structure.
2. **Paths are workspace-relative strings.** G3's fix must ensure output remains workspace-relative — the `orphan_file` parameter is already workspace-relative from `check_orphan_files`, and G3 strips a workspace-relative `src/` prefix. No absolute paths introduced.
3. **`skip_serializing_if = "Vec::is_empty"`** on `derive_attrs`. Omission will bloat JSON output with empty arrays on every symbol.
4. **`SymbolEntry` does not use `#[serde(rename_all = "camelCase")]`**. The field name `derive_attrs` will serialize as `"derive_attrs"` in JSON. This is correct.

### 5.2 G1-Specific Pitfalls

5. **`crate_names` alone is insufficient for bare-path check.** If a bare path's first segment matches a workspace member name, the re-export could still be internal (e.g., a crate named `utils` with a top-level module also named `utils`). Must also check `crate_info.modules` for top-level modules of the current crate.
6. **Bare path with single segment** (e.g., `use Foo`). First segment is the entire path. Should be checked as internal (single-segment bare paths resolve to the crate root scope).
7. **The existing `crate_names` cross-crate check (lines 148-151) must NOT be removed.** It operates on resolved paths and handles cross-workspace-member re-exports that pass G1. Coexistence is safe because G1 runs first and short-circuits via `continue`.

### 5.3 G2-Specific Pitfalls

8. **`derive_attrs` is a straight copy, not a re-extraction.** `item.attrs.derive` is already `Vec<String>` from `extract_attrs`. Do not re-parse the AST or call `extract_attrs` again. The code change is `.derive_attrs(item.attrs.derive.clone())` in the fold closure.
9. **Same-module scoping is critical.** The lookup key must use the re-export's module path, not a global search. A `pub use FooBuilder` in module `bar::sub` should look up `bar::sub::Foo`, not `qux::Foo`.
10. **Lookup key uses `module.path` (the module's canonical path), not `module.file`.** The `symbols` map is keyed by `CanonicalPath("crate::module::Name")`. Module path format is `"crate::module"`.
11. **Empty prefix edge case.** If `target_name` is empty (should not happen for valid Rust identifiers, but defensively), the loop should immediately return false.
12. **Prefix iteration order matters.** Start from longest prefix (`len(name)`) and decrease. Shorter prefixes are less specific — a match on `Cell` (prefix of `CellDocumentBuilder`) could be a different type entirely. Longest-prefix-first minimizes false suppression.
13. **Private-base-type limitation is a known false negative, not a bug.** Document in README. Do not attempt to populate the symbol index with private types — that's a separate project with broad implications (changes the contract of `--validate` from "public API surface" to "all types").

### 5.4 G3-Specific Pitfalls

14. **`crate_info.root` points to a file (e.g., `src/lib.rs`), not a directory.** Use `.parent()` to derive the `src/` directory. Do NOT use `.join("src")` — that would produce `src/lib.rs/src`.
15. **Windows path separators.** The code currently uses string-based `strip_prefix` and `rsplit_once('/')`. Switching to `Path` operations would be more robust but changes the style. If sticking with string operations, use `std::path::MAIN_SEPARATOR` or `Path::new(...).display()` to construct the prefix.

### 5.5 G4-Specific Pitfalls

16. **Modifying `bad-orphan/lib.rs` could break the existing shallow-orphan test.** The existing `forgotten.rs` must remain an orphan. Adding `pub mod sub;` is fine — `forgotten.rs` is still undeclared. Verify existing tests pass after fixture changes.
17. **The deep orphan must NOT be declared in any `mod` statement.** `sub/mod.rs` should declare `pub mod legal;` but NOT `pub mod deep;` or `pub mod deep::orphan;`. The file `sub/deep/orphan.rs` must have no corresponding declaration.

### 5.6 Testing Pitfalls

18. **Unit tests for `is_external_crate_re_export` and `is_derive_companion` need test-only data.** Construct minimal `CrateInfo`, `SymbolEntry`, and `HashSet` values inline. Do not parse real files. Follow existing validate.rs test patterns.
19. **Integration test binary must be compiled before running.** `cargo test --test integration_test` handles this. The test uses `env!("CARGO_MANIFEST_DIR")` to find the binary.
20. **`cargo clippy -D warnings` must remain clean.** No new `#[allow(...)]` annotations. Fix all warnings, don't suppress them.

### 5.7 Cross-Cutting Pitfalls

21. **G2 depends on G1 landing first** to avoid merge conflicts in `check_dead_reexports`. Both touch the same loop. G1 adds a pre-check at the top; G2 adds logic in the `!symbols.contains_key` branch. Sequential execution avoids conflict.
22. **`serde::Serialize` re-export is the canonical G1 false positive.** Verify: `pub use serde::Serialize;` in any module should NOT produce a DeadReExport finding after G1. The bare path `serde::Serialize` has first segment `serde`, which is neither a workspace member nor a top-level module → skipped.
23. **`bon::Builder` re-exports are the canonical G2 false positive.** Verify: on `castep-cell-io`, builder types like `CellDocumentBuilder` do not appear in DeadReExport findings after G2.

---

## 6. Suggested Task Grouping Rationale

### Group 1 — G1 + G3 + G4 (parallel, independent)

**Rationale for grouping:**
- All three touch different physical locations with zero overlap:
  - G1: `validate.rs` — new function `is_external_crate_re_export` + call site in `check_dead_reexports` top-of-loop
  - G3: `validate.rs` — body of `determine_parent_file` (different function, different line range)
  - G4: `tests/fixtures/bad-orphan/` and `tests/integration_test.rs` (different files entirely)
- No merge conflicts: G1 changes lines 136-142 (inside the for-loop), G3 changes lines 106-123 (different function), G4 touches test files.
- All three can be implemented, tested, and committed independently.

**Task decomposition for Group 1:**

| Task | File(s) | Type | Effort |
|------|---------|------|--------|
| G1-a | `src/validate.rs` | Implement `is_external_crate_re_export` | 15-20 lines |
| G1-b | `src/validate.rs` | Call it in `check_dead_reexports` | ~5 lines |
| G1-c | `src/validate.rs` | Unit tests for G1 (3 cases: bare external, `crate::` internal, `self::`/`super::` internal) | ~50 lines |
| G3-a | `src/validate.rs` | Fix `determine_parent_file` path derivation | ~5 lines |
| G4-a | `tests/fixtures/bad-orphan/src/sub/` | Create nested orphan fixture files | 3 new files |
| G4-b | `tests/fixtures/bad-orphan/src/lib.rs` | Add `pub mod sub;` | 1 line edit |
| G4-c | `tests/integration_test.rs` | Add `test_recursive_orphan_detection` | ~40 lines |

### Group 2 — G2 (sequential after Group 1)

**Rationale for sequential placement:**
- G2 modifies `check_dead_reexports` in the same function that G1 touches. If G1 and G2 were parallel, they'd produce a merge conflict on the for-loop body.
- G1 runs first, lands cleanly. Then G2 adds logic in the `!symbols.contains_key` branch — editing lines that are now stable post-G1.
- G2 also adds the `derive_attrs` field to `SymbolEntry` and populates it in `indexes.rs` — these are independent of G1/G3/G4 but benefit from the cleaner codebase state after Group 1 lands.

**Task decomposition for Group 2:**

| Task | File(s) | Type | Effort |
|------|---------|------|--------|
| G2-a | `src/schema.rs` | Add `derive_attrs: Vec<String>` to `SymbolEntry` | ~4 lines |
| G2-b | `src/indexes.rs` | Copy `item.attrs.derive.clone()` in fold closure | ~1 line |
| G2-c | `src/validate.rs` | Implement `is_derive_companion` | ~20 lines |
| G2-d | `src/validate.rs` | Call in `check_dead_reexports` DeadReExport branch | ~5 lines |
| G2-e | `src/validate.rs` | Unit tests for G2 (5 cases per Amendment B) | ~70 lines |
| G2-f | `README.md` | Document prefix-decomposition design rationale + private-base-type limitation | ~15 lines |

### Total file impact summary:

| File | Group 1 | Group 2 | Total lines (est.) |
|------|---------|---------|---------------------|
| `src/schema.rs` | — | G2-a (+4) | +4 |
| `src/indexes.rs` | — | G2-b (+1) | +1 |
| `src/validate.rs` | G1-a,b (+20), G3-a (+5) | G2-c,d (+25) | +50 |
| `src/validate.rs` tests | G1-c (+50) | G2-e (+70) | +120 |
| `tests/fixtures/bad-orphan/` | G4-a,b (+3 files, 1 edit) | — | +3 files |
| `tests/integration_test.rs` | G4-c (+40) | — | +40 |
| `README.md` | — | G2-f (+15) | +15 |

---

## 7. Implementation Order Within Each Group

### Group 1 implementation order:

1. **G3 first** (simplest, no dependencies): Fix `determine_parent_file` — purely local change, immediately verifiable with existing tests.
2. **G1 second** (adds function + hooks into flow): Implement `is_external_crate_re_export`, hook into `check_dead_reexports`, write unit tests.
3. **G4 third** (requires binary compilation for integration test): Create fixture files, modify `lib.rs`, add integration test.
4. **Verify:** `cargo test`, `cargo clippy -- -D warnings`.

### Group 2 implementation order:

1. **G2-a and G2-b first** (data plumbing, no logic): Add `derive_attrs` to `SymbolEntry`, populate in `indexes.rs`. Compile-check: `cargo check`.
2. **G2-c and G2-d second** (logic): Implement `is_derive_companion`, hook into `check_dead_reexports`. Write unit tests.
3. **G2-f third** (documentation): Update README.
4. **Verify:** `cargo test`, `cargo clippy -- -D warnings`, manual verification on `castep-cell-io` if available.

---

## 8. Verification Checklist (Implementation-Gate)

Before declaring each group complete:

### Group 1 verification:
- [ ] `cargo test -p rust-workspace-map` — all unit tests pass
- [ ] `cargo test -p rust-workspace-map --test integration_test` — all integration tests pass (including new G4 test)
- [ ] `cargo clippy -p rust-workspace-map -- -D warnings` — clean
- [ ] G1 unit test: `pub use serde::Serialize;` → `is_external_crate_re_export` returns true
- [ ] G1 unit test: `pub use crate::sub::missing;` → `is_external_crate_re_export` returns false
- [ ] G1 unit test: `pub use self::inner::Type;` → `is_external_crate_re_export` returns false
- [ ] G1 unit test: bare path `pub use other_member::Type;` (where `other_member` is in `crate_names`) → `is_external_crate_re_export` returns true (cross-workspace-member, handled by existing check downstream)
- [ ] G4: `test_recursive_orphan_detection` detects `sub/deep/orphan.rs`

### Group 2 verification:
- [ ] `cargo test -p rust-workspace-map` — all unit tests pass (G1 + G2 tests)
- [ ] `cargo test -p rust-workspace-map --test integration_test` — all integration tests pass (regression)
- [ ] `cargo clippy -p rust-workspace-map -- -D warnings` — clean
- [ ] G2 unit test: `CellDocumentBuilder` with `CellDocument` in same module having `derive_attrs = ["bon::Builder"]` → suppressed
- [ ] G2 unit test: `CellDocumentBuilder` with `CellDocument` in same module having empty `derive_attrs` → NOT suppressed
- [ ] G2 unit test: `CellDocumentBuilder` with `CellDocument` in DIFFERENT module having non-empty derives → NOT suppressed (cross-module scoping)
- [ ] G2 unit test: genuinely dead name `TotallyMissing` with no prefix match → NOT suppressed
- [ ] G2 unit test: `crate::`-prefixed path proceeds through G1+G2 combined check
- [ ] Manual: `cargo run -- index --validate` on a workspace with `bon::Builder` derive — builder types absent from DeadReExport findings
