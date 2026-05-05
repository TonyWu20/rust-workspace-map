# Plan: Add Single-Crate Support + Fix Silent Error Handling

## Context

Running `cargo run -- index .` from the `rust-workspace-map` project root produces no output. Two bugs cause this:

1. **Feature gap:** `find_workspace_root` in `workspace.rs:11` requires a `Cargo.toml` containing `[workspace]`. This project is a single crate (no `[workspace]` section), so it fails with `WorkspaceRootNotFound`. There is no fallback to treat a standalone crate as a "workspace of one."

2. **Error swallowing:** `main.rs:102-104` and `main.rs:165-167` discard errors silently: `Err(_) => { std::process::exit(1); }`. No message is printed to stderr, so even the failure is invisible.

## Design Approach

- **Auto-detect mode**: try workspace discovery first; on `WorkspaceRootNotFound`, fall back to single-crate discovery. No new CLI flags.
- **Two separate functions**: `find_workspace_root` (looks for `[workspace]`) + new `find_crate_root` (looks for `[package]`). This is necessary because a combined function would stop at a workspace member's `[package]` Cargo.toml instead of reaching the workspace root.
- **Fix all silent error exits**: both the pipeline error and the render errors in `main.rs`.

## Implementation Steps

### Step 1: Add `CrateRootNotFound` variant to `schema::Error`

**File:** `src/schema.rs`

Insert after line 54 (`WorkspaceRootNotFound` variant):

```rust
#[error("no Cargo.toml with [package] section found starting from {0}")]
CrateRootNotFound(PathBuf),
```

### Step 2: Add `find_crate_root` to `workspace.rs`

**File:** `src/workspace.rs`

Insert after line 25 (end of `find_workspace_root`, before `enumerate_members`):

```rust
/// Walk up the directory tree from `start_path` to find a `Cargo.toml`
/// containing a `[package]` section. Returns the directory containing it.
///
/// This is the fallback path when no workspace root is found — it treats
/// a single crate as a "workspace of one".
///
/// # Errors
///
/// Returns `Error::CrateRootNotFound` if no `Cargo.toml` with a
/// `[package]` section is found in any ancestor directory.
pub fn find_crate_root(start_path: &Path) -> Result<PathBuf> {
    for ancestor in start_path.ancestors() {
        let cargo_toml = ancestor.join("Cargo.toml");
        if cargo_toml.exists() {
            let content = std::fs::read_to_string(&cargo_toml).map_err(|source| Error::FileRead {
                path: cargo_toml.clone(),
                source,
            })?;
            if content.contains("[package]") {
                return Ok(ancestor.to_path_buf());
            }
        }
    }
    Err(Error::CrateRootNotFound(start_path.to_path_buf()))
}
```

### Step 3: Add unit tests for `find_crate_root`

**File:** `src/workspace.rs`, in the `#[cfg(test)] mod tests` block (after line 143)

Two tests:
- `find_crate_root_finds_package_section` — walks up from nested subdir to find Cargo.toml with `[package]`
- `find_crate_root_returns_err_for_no_package` — no Cargo.toml at all → `CrateRootNotFound`

### Step 4: Modify `build_map` in `lib.rs`

**File:** `src/lib.rs`

**4a.** Add `Error` to the `use schema::` import (line 18-21).

**4b.** Replace lines 37-38:
```rust
let workspace_root = workspace::find_workspace_root(&config.workspace_path)?;
let member_dirs = workspace::enumerate_members(&workspace_root)?;
```

With:
```rust
let (workspace_root, member_dirs) = match workspace::find_workspace_root(&config.workspace_path) {
    Ok(root) => {
        let members = workspace::enumerate_members(&root)?;
        (root, members)
    }
    Err(Error::WorkspaceRootNotFound(_)) => {
        let crate_dir = workspace::find_crate_root(&config.workspace_path)?;
        (crate_dir.clone(), vec![crate_dir])
    }
    Err(other) => return Err(other.into()),
};
```

The rest of `build_map` works unchanged — it uses `workspace_root` and `member_dirs` the same way regardless of source.

### Step 5: Fix silent error exits in `main.rs`

**File:** `src/main.rs`

Four sites to fix:

| Lines | Current | New |
|-------|---------|-----|
| 88-90 | `.is_err()` → `exit(1)` | `if let Err(e)` → `eprintln!("error writing JSON to file: {e}"); exit(1);` |
| 93-95 | `.is_err()` → `exit(1)` | `if let Err(e)` → `eprintln!("error writing JSON to stdout: {e}"); exit(1);` |
| 102-104 | `Err(_) => exit(1)` | `Err(e) => { eprintln!("error: {e:#}"); exit(1); }` |
| 165-167 | `Err(_) => exit(1)` | `Err(e) => { eprintln!("error: {e:#}"); exit(1); }` |

### Step 6: Update existing integration test

**File:** `tests/integration_test.rs`

Rename `test_missing_workspace_section` → `test_single_crate_without_workspace` (lines 199-223). The current test expects a non-zero exit for a crate without `[workspace]`. After the fix, a valid single crate should succeed.

Change to: remove the old `write_cargo_toml(root, ...)` call (line 205-210), then use `setup_crate(root, "pub struct Standalone { pub x: i32 }")` which writes both the Cargo.toml and src/lib.rs. Assert exit 0, assert one crate named `"standalone"` in output.

### Step 7: Add new integration test for single-crate module tree

**File:** `tests/integration_test.rs`

`test_single_crate_module_tree` — creates a standalone crate with `lib.rs` declaring `pub mod helpers;` and `helpers.rs` with `pub fn assist()`. Verifies exit 0, both modules present, and the `EntryPoint` struct in public items.

## Verification

1. `cargo check` — compiles cleanly after each step
2. `cargo test` — all 30 unit + 18 integration tests pass (17 existing + 2 new - 1 renamed)
3. `cargo run -- index .` from project root — produces valid JSON with one crate (`rust-workspace-map`) and all its modules
4. `cargo run -- index . | jq '.crates[0].name'` — outputs `"rust-workspace-map"`
5. `cargo run -- index /tmp/nonexistent` — exits 1 with a clear error message on stderr
6. `cargo clippy -- -D warnings` — clean (respects the existing `#![warn(clippy::pedantic)]`)
7. `cargo run -- lookup . --symbol <some_public_item>` from project root — produces valid JSON (confirms Lookup inherited single-crate support)

### Implementation Note: `validate::validate` path safety

When in single-crate mode, `workspace_root` equals the crate directory (not a parent-of-crates directory). Verify during implementation that `validate::validate(&crate_infos, &symbols, &workspace_root)` does not assume `workspace_root` is a parent-of-crates directory (e.g., it does not join `workspace_root` with a crate subdirectory name that doesn't exist in single-crate mode). Read `validate.rs:check_orphan_files` to confirm it uses `crate_info` paths directly, not composed from `workspace_root`.

## Files Modified

- `src/schema.rs` — add `CrateRootNotFound` error variant
- `src/workspace.rs` — add `find_crate_root` function + 2 unit tests
- `src/lib.rs` — add `Error` import, modify `build_map` fallback logic
- `src/main.rs` — fix 4 silent error exits
- `tests/integration_test.rs` — update 1 test, add 1 test
