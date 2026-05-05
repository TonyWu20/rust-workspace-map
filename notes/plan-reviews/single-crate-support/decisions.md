## Plan Review Decisions — just-running-our-binary-parsed-map — 2026-05-06

### Design Assessment

The design is sound. The two-function separation (`find_workspace_root` + `find_crate_root`) is architecturally required to avoid stopping at workspace member `[package]` Cargo.tomls instead of reaching the workspace root. Type conversions from `schema::Error` to `anyhow::Error` are valid (thiserror provides `std::error::Error`, and all internal fields are `Send + Sync + 'static`). Path semantics in single-crate mode are correct: `workspace_root == crate_dir` means `relativize_path` produces correctly relative paths (e.g., `src/lib.rs`). All edge cases resolve sensibly — standalone crate, workspace, root-as-member, subdirectory runs, no Cargo.toml anywhere, and Lookup against standalone crates all behave correctly.

### Deferred Item Decisions

#### Fix `determine_parent_file` path calculation (mvp)
**Decision:** Acknowledged — completed in mvp-fp-cleanup (G3)

#### Integration test for recursive orphan detection (mvp)
**Decision:** Acknowledged — completed in mvp-fp-cleanup (G4)

#### Make `pub mod` suggestion context-aware (mvp)
**Decision:** Defer again
**Rationale:** Requires threading crate-type into `validate::validate`, expanding scope beyond this hardening bug fix.
**Updated precondition:** A consumer of the validation output actively uses fix hints and reports incorrect `pub mod` suggestions for binary crates.

#### Document `src/tests/` directory pruning trade-off (mvp)
**Decision:** Close
**Rationale:** Already documented in the deferred list and architecture note. A code comment can be added next time that code path is touched. No functional value in a dedicated docs task.

#### Empty errors vector in run() (phase-0.1)
**Decision:** Acknowledged — already resolved in current codebase

#### Silent error swallowing in module tree construction (phase-0.1)
**Decision:** Close
**Rationale:** `lib.rs:84-88` now collects errors via `collected_errors.extend(e)` and surfaces them in JSON output. No remaining `unwrap_or_default()` on the error path. The precondition ("first consumer who needs per-crate health") is met.

#### Path fallback to "." in module_tree.rs (phase-0.1)
**Decision:** Close
**Rationale:** `.parent()` returning `None` only at filesystem root — unreachable for real Rust projects. Defensive code for an impossible state adds cognitive overhead with zero benefit.

#### Path fallback to "" in workspace.rs (phase-0.1)
**Decision:** Close
**Rationale:** Same class as above. Defaulting to empty vectors for unparseable workspace sections is reasonable graceful degradation.

#### No unit tests for 13 public API functions (phase-0.1)
**Decision:** Defer again
**Rationale:** Incremental coverage is better than a bulk testing pass. The current plan already adds 2 unit tests for `find_crate_root` plus 2 integration tests.
**Updated precondition:** When any of these functions is next modified, write unit tests alongside the change.

#### Eliminate `errors.clone()` in recursive `process_module_info` (phase-0.2)
**Decision:** Defer again
**Rationale:** O(depth × errors) cost is known but not yet a performance problem.
**Updated precondition:** When profiling demonstrates module_tree construction as a measurable bottleneck on crates with >50 modules, OR when a refactoring pass specifically targets module_tree.rs.

### Plan Amendments

1. Fixed "Five sites" → "Four sites" in Step 5 description
2. Added verification step for Lookup subcommand inheriting single-crate support
3. Added implementation note to verify `validate::validate` path safety with `workspace_root == crate_dir`
4. Clarified Step 6: `setup_crate` already writes Cargo.toml — the old `write_cargo_toml` call must be removed
