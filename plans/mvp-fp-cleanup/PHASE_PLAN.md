# Phase: MVP False-Positive Cleanup

**Date:** 2026-04-30
**Status:** Draft
**Branch:** mvp-dev

## Context

The MVP core (flat indexes, validate rules, CLI subcommands, lookup, fixtures) is implemented on `mvp-dev`. Two classes of DeadReExport false positive block `--validate` from being trustworthy on real workspaces:

1. **External-crate re-exports** — `pub use serde::Serialize;` resolves to `mycrate::serde::Serialize` and fails the symbol-index lookup, producing a false positive for every external-crate re-export.
2. **Derive-generated companion types** — `#[derive(bon::Builder)]` on `pub struct CellDocument` generates a `CellDocumentBuilder` type that is `pub use`-re-exported but absent from the syn-based symbol index. On `castep-cell-io`, this produces 283 DeadReExport findings.

Additionally, a path-calculation bug in `determine_parent_file` produces misleading fix hints for workspace members not at the workspace root, and the recursive orphan walk (the headline feature of the most recent fix) lacks targeted regression coverage for depth >1.

## Goals

### Goal 1 — Fix DeadReExport false positives on external crates (Small)

**What it achieves:** Before resolving an import path through `crate::`/`self::`/`super::` prefix handling, check the original (unresolved) path's first segment. If it is neither a workspace member crate name nor a module name in the current crate, skip the re-export — it targets an external crate and is not dead.

**Why now:** Affects every workspace that re-exports from external crates. The fix is small, purely additive, and unblocks trust in DeadReExport as a deterministic gate.

**Effort:** ~15-20 lines in `validate.rs::check_dead_reexports`, plus 2-3 unit tests.

### Goal 2 — Derive-generated companion-type awareness (Medium)

**What it achieves:** Eliminates false-positive DeadReExport findings for derive-generated types without hardcoding any derive name or suffix. The heuristic:

1. When DeadReExport fires on `crate::module::XxxSuffix` (not found in symbols):
2. Find the **longest prefix** of the name that exists as a symbol in the **same module** (e.g., `CellDocumentBuilder` → `CellDocument`).
3. Check whether that base symbol has any `#[derive(...)]` attributes.
4. If yes: the re-export targets a derive-generated companion type. Suppress (not dead).

**Design rationale (recorded in README.md):**

- Zero derive names hardcoded. Zero suffix list. Zero maintenance.
- The signal is structural: a `pub use` of a name that decomposes to a known base type in the same module, where the base type carries derives. The heuristic doesn't care which derive or what the companion is named.
- Conservative-recall policy aligns: the tool prefers false negatives (suppressing a genuinely-dead re-export where the base type coincidentally has derives) over false positives (flagging every derive-generated re-export). The genuinely-dead-with-derives-on-base case is vanishingly rare.
- `serde::Serialize`, `serde::Deserialize`, `thiserror::Error`, and most ecosystem derives never enter this code path — they generate trait/method impls only, not new named types.
- If the prefix lookup fails to decompose (no base type found in the same module), the DeadReExport stands — this handles cases where the name genuinely has no base in the crate.

**Why now:** The `--validate` gate on `castep-cell-io` produces 283 findings today, a mix of real dead re-exports and false positives. No one can triage 283 entries. This heuristic eliminates the derive-generated class entirely.

**Effort:** Medium. Touches:
- `src/indexes.rs` — during symbol construction, embed derive attributes on `SymbolEntry` (the `PublicItem.attrs` data is already captured, just needs to be carried through)
- `src/schema.rs` — add `derive_attrs: Vec<String>` to `SymbolEntry` (optional, default empty)
- `src/validate.rs` — prefix-decomposition + base-type derive check in the DeadReExport handler
- Unit tests: prefix-decompose cases (builder, error, no-match), derive-presence check, same-module scoping

### Goal 3 — Fix `determine_parent_file` for non-root crate locations (Trivial)

**What it achieves:** Replaces the hardcoded `strip_prefix("src/")` in `determine_parent_file` with a prefix derived from `crate_info.root`. The `_crate_info` parameter is already passed in and unused — this fix just uses it.

**Why now:** The fix hint is actively misleading for workspaces like `crates/mylib/src/subdir/file.rs`. The function is already open for Goals 1 and 2; fixing this adds ~5 lines.

**Effort:** Trivial. One path-calculation change in `src/validate.rs:105-124`.

### Goal 4 — Integration test for recursive subdirectory orphan detection (Small)

**What it achieves:** Extends the `bad-orphan` fixture with a deeply nested orphan file (e.g., `src/sub/deep/orphan.rs`) and adds an integration test asserting it is detected. Without this, a regression back to non-recursive `read_dir` would not be caught.

**Why now:** The recursive walk via `walkdir` is the headline feature of the most recent fix commit. It has no targeted regression coverage at depth >1.

**Effort:** Small. Extend fixture directory structure, add one integration test (~30 lines).

## Scope Boundaries

**In scope:**
- G1: External-crate first-segment check in `check_dead_reexports`
- G2: Prefix-decomposition heuristic with derive-attribute cross-check; `derive_attrs` on `SymbolEntry`
- G3: Fix `determine_parent_file` to use `crate_info.root`
- G4: Deep-orphan integration test
- README.md: document the prefix-decomposition design rationale in the validation rules section

**Out of scope:**
- Pipeline integration (V3 pipeline refactor deprecated `compile-plan`; no integration target exists yet)
- `pub mod` vs `mod` suggestion context-awareness (cosmetic, deferred)
- Any new validate rules (`UnreachablePub`, `UnsupportedLayout`)
- Any new output format, CLI flag, cache, MCP server, or diff subcommand
- Generalization beyond prefix-decomposition (e.g., fuzzy matching, type-alias tracing)

## Design Notes

### G1: External-crate check

In `check_dead_reexports`, the import path is currently resolved through `crate::`/`self::`/`super::` prefix expansion before checking the symbol index. The fix adds a pre-check on the **original path** (the `use_path` as written in source, before resolution):

```
if first_segment is not in (workspace_members ∪ current_crate_modules):
    skip  # external crate
```

Workspace member names are already available in scope (the function iterates over all crates). Module names for the current crate can be collected from `crate_info.modules[].name`.

### G2: Prefix-decomposition heuristic

**Data flow:**

1. `indexes.rs` already iterates all `PublicItem`s to build `SymbolEntry` values. During this pass, extract derive attributes from `item.attrs` (filter for `#[derive(...)]` lines, parse inner paths). Store on `SymbolEntry.derive_attrs: Vec<String>`.

2. `schema.rs`: add field to `SymbolEntry`:
   ```rust
   #[builder(default)]
   #[serde(skip_serializing_if = "Vec::is_empty")]
   pub derive_attrs: Vec<String>,
   ```

3. `validate.rs`: in the DeadReExport handler, when a symbol is not found:
   - Take the last segment of the canonical path (e.g., `CellDocumentBuilder`)
   - Iterate decreasing prefix lengths
   - At each step, check `symbols["crate::module::<prefix>"]` for the same module path
   - First match: check `symbol.derive_attrs.is_empty()`
   - If non-empty: suppress this DeadReExport (derive-generated companion)
   - If no prefix matches at all: the finding stands

**Same-module scoping:** The prefix lookup is constrained to the same module as the re-export target. A `pub use` of `FooBuilder` in module `bar` only checks symbols in module `bar` — not global. This prevents cross-module false matches.

**Performance:** The prefix iteration is O(name_length × hashmap_lookup) per DeadReExport finding. Name lengths are typically <30 characters. Even with hundreds of findings, the cost is negligible.

### G3: Path calculation fix

Current code (conceptual):
```rust
let parent = orphan_path.strip_prefix("src/")?;
let parent_file = parent.parent()?.join("mod.rs");
```

Fixed code (conceptual):
```rust
let crate_src = crate_info.root.join("src");
let parent = orphan_path.strip_prefix(crate_src.parent()?)?;
let parent_file = parent.parent()?.join("mod.rs");
```

## Files to Modify

| File | Goals | What changes |
|---|---|---|
| `src/validate.rs` | G1, G2, G3 | External-crate check, prefix-decomposition heuristic, determine_parent_file fix |
| `src/indexes.rs` | G2 | Extract derive_attrs during symbol construction |
| `src/schema.rs` | G2 | Add `derive_attrs: Vec<String>` to `SymbolEntry` |
| `tests/fixtures/bad-orphan/` | G4 | Add nested subdirectory with deep orphan file |
| `tests/integration_test.rs` | G4 | Add `test_recursive_orphan_detection` |
| `README.md` | G2 | Document prefix-decomposition design rationale |

## Execution Sequence

```
G1 + G3 + G4   (parallel — independent, touch different functions/files)
      │
      └──→ G2   (sequential — touches check_dead_reexports after G1 lands, avoids merge conflicts)
```

## Verification

1. **G1**: On a fixture with `pub use serde::Serialize;` — no DeadReExport finding. On a fixture with `pub use missing::Type;` where `missing` is not a workspace member — also skipped (correct).
2. **G2**: `rust-workspace-map index --validate` on `castep-cell-io` — derive-generated builder types do not appear in DeadReExport findings; genuinely-dead re-exports still do.
3. **G3**: On a multi-crate workspace with non-root member — fix hint names the correct parent file.
4. **G4**: `cargo test -p rust-workspace-map --test integration_test` — `test_recursive_orphan_detection` passes.
5. **Regression**: `cargo test -p rust-workspace-map` and `cargo test -p rust-workspace-map --test integration_test` green.
6. **Clippy**: `cargo clippy -p rust-workspace-map -- -D warnings` passes clean.
