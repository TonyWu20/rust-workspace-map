## Gather Summary: rust-workspace-map-plan

**Tasks created:** 11
**Dependency chain:** TASK-1 → TASK-2 → TASK-3..8 ∥ → TASK-9 → TASK-10 → TASK-11 (sequential phases, parallelizable within phases)
**Deferred items absorbed:** 0 (none found)

**Gather completeness:**
- [x] deferred-and-patterns.md — saved
- [x] codebase-state.md — saved — Files documented: 11
- [x] draft-elaboration.md — saved
- [x] draft-plan.toml — saved
- [x] task-checklist.md — saved

**Before-block verification:** 0/0 confirmed (greenfield — no existing source to match)
**Unverified tasks:** none
**Wiring issues flagged:** 1 (TASK-11: `env!("CARGO_BIN_EXE_rust-workspace-map")` should use underscores: `rust_workspace_map`)

**Notable findings from checklist:**
- TASK-2 missing explicit `[dependencies]` edge from TASK-1 (safe sequentially, risky for parallel scheduler)
- TASK-1 through TASK-8 have weak acceptance (`test -f <filename>`) — syntax errors latent until TASK-9
- `walkdir = "2"` in Cargo.toml is declared but never used
- `cargo-llvm-cov = "0.6"` in dev-dependencies is a binary subcommand, not a library crate
- `FileInfo.impls` extracted by file_parser (TASK-5) but never consumed by module_tree (TASK-6) — data silently discarded
- Cross-crate detection in TASK-7 uses first-path-segment heuristic (potential false negatives)

**Confidence notes:**
4 items flagged uncertain by rust-architect:
- `glob` crate dependency not listed in plan but needed for workspace member glob resolution
- `src/bin/*.rs` binary targets deferred
- `crossReferences.types` vs `symbols` naming ambiguity
- `std::path::absolute` toolchain availability (MSRV concern)

**Questions for user:**
- Should the unused `walkdir` dependency be removed from Cargo.toml?
- Should `cargo-llvm-cov` be removed from dev-dependencies (it's a cargo subcommand, not a library)?
- Should the missed `glob` dependency for workspace member resolution be added?
- TASK-11 env var bug: confirm `rust_workspace_map` (underscores) is the correct fix?
- `FileInfo.impls` is extracted but never consumed — should it be removed from the schema or wired into module_tree?
