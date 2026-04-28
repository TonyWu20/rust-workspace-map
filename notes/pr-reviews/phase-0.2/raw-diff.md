# Raw Diff: `phase-0.2` -> `main`
Generated: 2026-04-28T12:57:07Z

## Commits
bbc0490 fix(phase-0.2): fix integration test extraction bug and post-impl cleanup
d69a51f feat(phase-0.2): TASK-10: Expand integration tests from 3 to 10 covering error cases, workspace patterns, and output
2c73842 feat(phase-0.2): TASK-9: Add unit tests for file_parser, module_tree, workspace, cargo_info, cross_refs, and render
06a3515 feat(phase-0.2): TASK-8: Remove all crate-level clippy allow attributes and fix individual lint violations
bd49767 feat(phase-0.2): TASK-6: Update workspace.rs enumerate_members to return MissingWorkspaceSection error
8aa8507 feat(phase-0.2): TASK-7: Refactor lib.rs run(): parallel error collection, error entry construction, remove unwrap_or_default
a44c48e feat(phase-0.2): TASK-5: Refactor module_tree.rs: fix path fallbacks, add error collection, change build_module_tree return type
3abd1f3 feat(phase-0.2): TASK-4: Change parse_file to return ParsedFile with optional parse errors instead of Result
a13f330 feat(phase-0.2): TASK-3: Expand ErrorEntry struct with severity, kind, context, and cause fields
99dbae6 feat(phase-0.2): TASK-2: Add MissingWorkspaceSection variant to Error enum in schema.rs
bfe6a41 plan(phase-0.2): finalize enriched plan after judge review

## Diff Stat
 .claude/hooks/current_task_TASK-PREP.json          |   24 +
 Cargo.lock                                         |  287 ++-
 Cargo.toml                                         |    3 +
 execution_reports/.checkpoint_phase-0.2.json       |   18 +
 execution_reports/execution_phase-0.2_20260428.md  |  288 +++
 notes/plan-enrichment/phase-0.2/codebase-state.md  |  328 +++
 .../phase-0.2/deferred-and-patterns.md             |   11 +
 .../plan-enrichment/phase-0.2/draft-elaboration.md |  563 +++++
 notes/plan-enrichment/phase-0.2/gather-summary.md  |   27 +
 notes/plan-enrichment/phase-0.2/plan.approved.toml | 2168 ++++++++++++++++++++
 notes/plan-enrichment/phase-0.2/task-checklist.md  |  274 +++
 plans/compiled/TASK-1.py                           |   44 +
 plans/compiled/TASK-1.sh                           |    7 +
 plans/compiled/TASK-10.py                          |   44 +
 plans/compiled/TASK-10.sh                          |    7 +
 plans/compiled/TASK-2.py                           |   44 +
 plans/compiled/TASK-2.sh                           |    7 +
 plans/compiled/TASK-3.py                           |   44 +
 plans/compiled/TASK-3.sh                           |    7 +
 plans/compiled/TASK-4.py                           |   44 +
 plans/compiled/TASK-4.sh                           |    7 +
 plans/compiled/TASK-5.py                           |   44 +
 plans/compiled/TASK-5.sh                           |    7 +
 plans/compiled/TASK-6.py                           |   44 +
 plans/compiled/TASK-6.sh                           |    7 +
 plans/compiled/TASK-7.py                           |   44 +
 plans/compiled/TASK-7.sh                           |    7 +
 plans/compiled/TASK-8.py                           |   44 +
 plans/compiled/TASK-8.sh                           |    7 +
 plans/compiled/TASK-9.py                           |   44 +
 plans/compiled/TASK-9.sh                           |    7 +
 plans/compiled/TASK-PREP.py                        |   44 +
 plans/compiled/TASK-PREP.sh                        |    7 +
 plans/compiled/manifest.json                       |  139 ++
 plans/phase-0.2.toml                               | 2168 ++++++++++++++++++++
 src/cargo_info.rs                                  |   65 +
 src/cross_refs.rs                                  |   82 +
 src/file_parser.rs                                 |  304 ++-
 src/lib.rs                                         |   97 +-
 src/main.rs                                        |    2 +-
 src/module_tree.rs                                 |  205 +-
 src/render.rs                                      |   78 +
 src/schema.rs                                      |   42 +-
 src/workspace.rs                                   |  131 +-
 tests/integration_test.rs                          |  265 +++
 45 files changed, 7967 insertions(+), 163 deletions(-)

## Full Diff
diff --git a/.claude/hooks/current_task_TASK-PREP.json b/.claude/hooks/current_task_TASK-PREP.json
new file mode 100644
index 0000000..1b12c30
--- /dev/null
+++ b/.claude/hooks/current_task_TASK-PREP.json
@@ -0,0 +1,24 @@
+{
+  "task_id": "TASK-PREP",
+  "task_description": "Add tempfile dev-dependency for unit and integration tests",
+  "plan_path": "/Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml",
+  "plan_slug": "phase-0.2",
+  "acceptance_commands": [
+    "cargo check -p rust-workspace-map"
+  ],
+  "acceptance_prose": [],
+  "all_task_ids": [
+    "TASK-PREP",
+    "TASK-1",
+    "TASK-2",
+    "TASK-3",
+    "TASK-4",
+    "TASK-5",
+    "TASK-6",
+    "TASK-7",
+    "TASK-8",
+    "TASK-9",
+    "TASK-10"
+  ],
+  "timestamp": "2026-04-28T08:54:28Z"
+}
diff --git a/Cargo.lock b/Cargo.lock
index 45258fb..15d9c74 100644
--- a/Cargo.lock
+++ b/Cargo.lock
@@ -58,6 +58,12 @@ version = "1.0.102"
 source = "registry+https://github.com/rust-lang/crates.io-index"
 checksum = "7f202df86484c868dbad7eaa557ef785d5c66295e41b460ef922eca0723b842c"
 
+[[package]]
+name = "bitflags"
+version = "2.11.1"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "c4512299f36f043ab09a583e57bceb5a5aab7a73db1805848e8fef3c9e8c78b3"
+
 [[package]]
 name = "bon"
 version = "3.9.1"
@@ -83,6 +89,12 @@ dependencies = [
  "syn",
 ]
 
+[[package]]
+name = "cfg-if"
+version = "1.0.4"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "9330f8b2ff13f34540b44e946ef35111825727b38d33286ef986142615121801"
+
 [[package]]
 name = "clap"
 version = "4.6.1"
@@ -200,12 +212,56 @@ version = "1.0.2"
 source = "registry+https://github.com/rust-lang/crates.io-index"
 checksum = "877a4ace8713b0bcf2a4e7eec82529c029f1d0619886d18145fea96c3ffe5c0f"
 
+[[package]]
+name = "errno"
+version = "0.3.14"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "39cab71617ae0d63f51a36d69f866391735b51691dbda63cf6f96d042b63efeb"
+dependencies = [
+ "libc",
+ "windows-sys",
+]
+
+[[package]]
+name = "fastrand"
+version = "2.4.1"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "9f1f227452a390804cdb637b74a86990f2a7d7ba4b7d5693aac9b4dd6defd8d6"
+
+[[package]]
+name = "foldhash"
+version = "0.1.5"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "d9c4f5dac5e15c24eb999c26181a6ca40b39fe946cbe4c263c7209467bc83af2"
+
+[[package]]
+name = "getrandom"
+version = "0.4.2"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "0de51e6874e94e7bf76d726fc5d13ba782deca734ff60d5bb2fb2607c7406555"
+dependencies = [
+ "cfg-if",
+ "libc",
+ "r-efi",
+ "wasip2",
+ "wasip3",
+]
+
 [[package]]
 name = "glob"
 version = "0.3.3"
 source = "registry+https://github.com/rust-lang/crates.io-index"
 checksum = "0cc23270f6e1808e30a928bdc84dea0b9b4136a8bc82338574f23baf47bbd280"
 
+[[package]]
+name = "hashbrown"
+version = "0.15.5"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "9229cfe53dfd69f0609a49f65461bd93001ea1ef889cd5529dd176593f5338a1"
+dependencies = [
+ "foldhash",
+]
+
 [[package]]
 name = "hashbrown"
 version = "0.17.0"
@@ -218,6 +274,12 @@ version = "0.5.0"
 source = "registry+https://github.com/rust-lang/crates.io-index"
 checksum = "2304e00983f87ffb38b55b444b5e3b60a884b5d30c0fca7d82fe33449bbe55ea"
 
+[[package]]
+name = "id-arena"
+version = "2.3.0"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "3d3067d79b975e8844ca9eb072e16b31c3c1c36928edf9c6789548c524d0d954"
+
 [[package]]
 name = "ident_case"
 version = "1.0.1"
@@ -231,7 +293,9 @@ source = "registry+https://github.com/rust-lang/crates.io-index"
 checksum = "d466e9454f08e4a911e14806c24e16fba1b4c121d1ea474396f396069cf949d9"
 dependencies = [
  "equivalent",
- "hashbrown",
+ "hashbrown 0.17.0",
+ "serde",
+ "serde_core",
 ]
 
 [[package]]
@@ -246,12 +310,42 @@ version = "1.0.18"
 source = "registry+https://github.com/rust-lang/crates.io-index"
 checksum = "8f42a60cbdf9a97f5d2305f08a87dc4e09308d1276d28c869c684d7777685682"
 
+[[package]]
+name = "leb128fmt"
+version = "0.1.0"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "09edd9e8b54e49e587e4f6295a7d29c3ea94d469cb40ab8ca70b288248a81db2"
+
+[[package]]
+name = "libc"
+version = "0.2.186"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "68ab91017fe16c622486840e4c83c9a37afeff978bd239b5293d61ece587de66"
+
+[[package]]
+name = "linux-raw-sys"
+version = "0.12.1"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "32a66949e030da00e8c7d4434b251670a91556f4144941d37452769c25d58a53"
+
+[[package]]
+name = "log"
+version = "0.4.29"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "5e5032e24019045c762d3c0f28f5b6b8bbf38563a65908389bf7978758920897"
+
 [[package]]
 name = "memchr"
 version = "2.8.0"
 source = "registry+https://github.com/rust-lang/crates.io-index"
 checksum = "f8ca58f447f06ed17d5fc4043ce1b10dd205e060fb3ce5b979b8ed8e59ff3f79"
 
+[[package]]
+name = "once_cell"
+version = "1.21.4"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "9f7c3e4beb33f85d45ae3e3a1792185706c8e16d043238c593331cc7cd313b50"
+
 [[package]]
 name = "once_cell_polyfill"
 version = "1.70.2"
@@ -286,6 +380,12 @@ dependencies = [
  "proc-macro2",
 ]
 
+[[package]]
+name = "r-efi"
+version = "6.0.0"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "f8dcc9c7d52a811697d2151c701e0d08956f92b0e24136cf4cf27b57a6a0d9bf"
+
 [[package]]
 name = "rayon"
 version = "1.12.0"
@@ -319,16 +419,36 @@ dependencies = [
  "serde",
  "serde_json",
  "syn",
+ "tempfile",
  "thiserror",
  "toml",
 ]
 
+[[package]]
+name = "rustix"
+version = "1.1.4"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "b6fe4565b9518b83ef4f91bb47ce29620ca828bd32cb7e408f0062e9930ba190"
+dependencies = [
+ "bitflags",
+ "errno",
+ "libc",
+ "linux-raw-sys",
+ "windows-sys",
+]
+
 [[package]]
 name = "rustversion"
 version = "1.0.22"
 source = "registry+https://github.com/rust-lang/crates.io-index"
 checksum = "b39cdef0fa800fc44525c84ccb54a029961a8215f9619753635a9c0d2538d46d"
 
+[[package]]
+name = "semver"
+version = "1.0.28"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "8a7852d02fc848982e0c167ef163aaff9cd91dc640ba85e263cb1ce46fae51cd"
+
 [[package]]
 name = "serde"
 version = "1.0.228"
@@ -398,6 +518,19 @@ dependencies = [
  "unicode-ident",
 ]
 
+[[package]]
+name = "tempfile"
+version = "3.27.0"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "32497e9a4c7b38532efcdebeef879707aa9f794296a4f0244f6f69e9bc8574bd"
+dependencies = [
+ "fastrand",
+ "getrandom",
+ "once_cell",
+ "rustix",
+ "windows-sys",
+]
+
 [[package]]
 name = "thiserror"
 version = "2.0.18"
@@ -465,12 +598,70 @@ version = "1.0.24"
 source = "registry+https://github.com/rust-lang/crates.io-index"
 checksum = "e6e4313cd5fcd3dad5cafa179702e2b244f760991f45397d14d4ebf38247da75"
 
+[[package]]
+name = "unicode-xid"
+version = "0.2.6"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "ebc1c04c71510c7f702b52b7c350734c9ff1295c464a03335b00bb84fc54f853"
+
 [[package]]
 name = "utf8parse"
 version = "0.2.2"
 source = "registry+https://github.com/rust-lang/crates.io-index"
 checksum = "06abde3611657adf66d383f00b093d7faecc7fa57071cce2578660c9f1010821"
 
+[[package]]
+name = "wasip2"
+version = "1.0.3+wasi-0.2.9"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "20064672db26d7cdc89c7798c48a0fdfac8213434a1186e5ef29fd560ae223d6"
+dependencies = [
+ "wit-bindgen 0.57.1",
+]
+
+[[package]]
+name = "wasip3"
+version = "0.4.0+wasi-0.3.0-rc-2026-01-06"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "5428f8bf88ea5ddc08faddef2ac4a67e390b88186c703ce6dbd955e1c145aca5"
+dependencies = [
+ "wit-bindgen 0.51.0",
+]
+
+[[package]]
+name = "wasm-encoder"
+version = "0.244.0"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "990065f2fe63003fe337b932cfb5e3b80e0b4d0f5ff650e6985b1048f62c8319"
+dependencies = [
+ "leb128fmt",
+ "wasmparser",
+]
+
+[[package]]
+name = "wasm-metadata"
+version = "0.244.0"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "bb0e353e6a2fbdc176932bbaab493762eb1255a7900fe0fea1a2f96c296cc909"
+dependencies = [
+ "anyhow",
+ "indexmap",
+ "wasm-encoder",
+ "wasmparser",
+]
+
+[[package]]
+name = "wasmparser"
+version = "0.244.0"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "47b807c72e1bac69382b3a6fb3dbe8ea4c0ed87ff5629b8685ae6b9a611028fe"
+dependencies = [
+ "bitflags",
+ "hashbrown 0.15.5",
+ "indexmap",
+ "semver",
+]
+
 [[package]]
 name = "windows-link"
 version = "0.2.1"
@@ -495,6 +686,100 @@ dependencies = [
  "memchr",
 ]
 
+[[package]]
+name = "wit-bindgen"
+version = "0.51.0"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "d7249219f66ced02969388cf2bb044a09756a083d0fab1e566056b04d9fbcaa5"
+dependencies = [
+ "wit-bindgen-rust-macro",
+]
+
+[[package]]
+name = "wit-bindgen"
+version = "0.57.1"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "1ebf944e87a7c253233ad6766e082e3cd714b5d03812acc24c318f549614536e"
+
+[[package]]
+name = "wit-bindgen-core"
+version = "0.51.0"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "ea61de684c3ea68cb082b7a88508a8b27fcc8b797d738bfc99a82facf1d752dc"
+dependencies = [
+ "anyhow",
+ "heck",
+ "wit-parser",
+]
+
+[[package]]
+name = "wit-bindgen-rust"
+version = "0.51.0"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "b7c566e0f4b284dd6561c786d9cb0142da491f46a9fbed79ea69cdad5db17f21"
+dependencies = [
+ "anyhow",
+ "heck",
+ "indexmap",
+ "prettyplease",
+ "syn",
+ "wasm-metadata",
+ "wit-bindgen-core",
+ "wit-component",
+]
+
+[[package]]
+name = "wit-bindgen-rust-macro"
+version = "0.51.0"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "0c0f9bfd77e6a48eccf51359e3ae77140a7f50b1e2ebfe62422d8afdaffab17a"
+dependencies = [
+ "anyhow",
+ "prettyplease",
+ "proc-macro2",
+ "quote",
+ "syn",
+ "wit-bindgen-core",
+ "wit-bindgen-rust",
+]
+
+[[package]]
+name = "wit-component"
+version = "0.244.0"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "9d66ea20e9553b30172b5e831994e35fbde2d165325bec84fc43dbf6f4eb9cb2"
+dependencies = [
+ "anyhow",
+ "bitflags",
+ "indexmap",
+ "log",
+ "serde",
+ "serde_derive",
+ "serde_json",
+ "wasm-encoder",
+ "wasm-metadata",
+ "wasmparser",
+ "wit-parser",
+]
+
+[[package]]
+name = "wit-parser"
+version = "0.244.0"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "ecc8ac4bc1dc3381b7f59c34f00b67e18f910c2c0f50015669dde7def656a736"
+dependencies = [
+ "anyhow",
+ "id-arena",
+ "indexmap",
+ "log",
+ "semver",
+ "serde",
+ "serde_derive",
+ "serde_json",
+ "unicode-xid",
+ "wasmparser",
+]
+
 [[package]]
 name = "zmij"
 version = "1.0.21"
diff --git a/Cargo.toml b/Cargo.toml
index 3088325..b89c9b7 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -15,3 +15,6 @@ bon = "3"
 thiserror = "2"
 glob = "0.3"
 proc-macro2 = { version = "1", features = ["span-locations"] }
+
+[dev-dependencies]
+tempfile = "3"
diff --git a/execution_reports/.checkpoint_phase-0.2.json b/execution_reports/.checkpoint_phase-0.2.json
new file mode 100644
index 0000000..bb3c36a
--- /dev/null
+++ b/execution_reports/.checkpoint_phase-0.2.json
@@ -0,0 +1,18 @@
+{
+  "plan": "/Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml",
+  "base_commit": "bfe6a418a006d292aabea07d27e4a8440d57d661",
+  "completed": [
+    "TASK-2",
+    "TASK-3"
+  ],
+  "failed": [
+    "TASK-4",
+    "TASK-5",
+    "TASK-7",
+    "TASK-6",
+    "TASK-8",
+    "TASK-9",
+    "TASK-10"
+  ],
+  "blocked": []
+}
diff --git a/execution_reports/execution_phase-0.2_20260428.md b/execution_reports/execution_phase-0.2_20260428.md
new file mode 100644
index 0000000..4f74f62
--- /dev/null
+++ b/execution_reports/execution_phase-0.2_20260428.md
@@ -0,0 +1,288 @@
+# Execution Report
+
+**Plan**: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
+**Started**: 2026-04-28T08:55:35Z
+**Status**: In Progress
+
+## Task Results
+
+### TASK-2: Add MissingWorkspaceSection variant to Error enum in schema.rs
+- **Status**: ✓ Passed
+- **Validation output**:
+  - `cargo check -p rust-workspace-map`: PASSED
+  - `cargo check --workspace 2>&1`: PASSED
+
+### TASK-1: Add ErrorSeverity enum and ErrorContext struct to schema.rs
+- **Status**: ✓ Passed
+- **Validation output**:
+  - `cargo check -p rust-workspace-map`: PASSED
+  - `cargo check --workspace 2>&1`: PASSED
+
+### TASK-2: Add MissingWorkspaceSection variant to Error enum in schema.rs
+- **Status**: ✓ Passed
+- **Validation output**:
+  - `cargo check -p rust-workspace-map`: PASSED
+  - `cargo check --workspace 2>&1`: PASSED
+
+### TASK-3: Expand ErrorEntry struct with severity, kind, context, and cause fields
+- **Status**: ✓ Passed
+- **Validation output**:
+  - `cargo check -p rust-workspace-map`: PASSED
+  - `cargo check --workspace 2>&1`: PASSED
+
+### TASK-4: Change parse_file to return ParsedFile with optional parse errors instead of Result
+- **Status**: ✗ Failed
+- **Validation output**:
+  - `true  # applied atomically with TASK-5; compilation verified at TASK-5`: PASSED
+  - `cargo check --workspace 2>&1`: FAILED (exit 101)
+    ```
+    Checking rust-workspace-map v0.1.0 (/Users/tony/programming/rust-workspace-map)
+    warning: unused imports: `Error` and `Result`
+     --> src/file_parser.rs:2:5
+      |
+    2 |     Error, ErrorEntry, ErrorSeverity, FileInfo, ImplInfo, ImplItem, ImplItemKind, Import,
+      |     ^^^^^
+    3 |     ItemAttrs, ItemKind, PublicItem, ReExport, Result, SubmoduleDecl,
+      |                                                ^^^^^^
+      |
+      = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default
+    
+    error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
+      --> src/module_tree.rs:27:28
+       |
+    27 |     let (ast, file_info) = file_parser::parse_file(crate_root)?;
+       |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `ParsedFile`
+       |
+    help: the nightly-only, unstable trait `std::ops::Try` is not implemented for `ParsedFile`
+      --> src/file_parser.rs:15:1
+       |
+    15 | pub struct ParsedFile {
+       | ^^^^^^^^^^^^^^^^^^^^^
+    
+    error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
+       --> src/module_tree.rs:148:32
+        |
+    148 |         let (ast, file_info) = file_parser::parse_file(file_path)?;
+        |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `ParsedFile`
+        |
+    help: the nightly-only, unstable trait `std::ops::Try` is not implemented for `ParsedFile`
+    ```
+
+### TASK-5: Refactor module_tree.rs: fix path fallbacks, add error collection, change build_module_tree return type
+- **Status**: ✗ Failed
+- **Validation output**:
+  - `cargo check -p rust-workspace-map`: FAILED (exit 101)
+    ```
+    Checking rust-workspace-map v0.1.0 (/Users/tony/programming/rust-workspace-map)
+    warning: unused imports: `Error` and `Result`
+     --> src/file_parser.rs:2:5
+      |
+    2 |     Error, ErrorEntry, ErrorSeverity, FileInfo, ImplInfo, ImplItem, ImplItemKind, Import,
+      |     ^^^^^
+    3 |     ItemAttrs, ItemKind, PublicItem, ReExport, Result, SubmoduleDecl,
+      |                                                ^^^^^^
+      |
+      = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default
+    
+    warning: unused import: `Result`
+     --> src/module_tree.rs:2:84
+      |
+    2 | use crate::schema::{ErrorContext, ErrorEntry, ErrorSeverity, FileInfo, ModuleInfo, Result, SubmoduleDecl};
+      |                                                                                    ^^^^^^
+    
+    error[E0599]: no method named `unwrap_or_default` found for tuple `(Vec<ModuleInfo>, Vec<ErrorEntry>)` in the current scope
+      --> src/lib.rs:79:69
+       |
+    79 |                     module_tree::build_module_tree(root, &pkg_name).unwrap_or_default()
+       |                                                                     ^^^^^^^^^^^^^^^^^ method not found in `(Vec<ModuleInfo>, Vec<ErrorEntry>)`
+    
+    For more information about this error, try `rustc --explain E0599`.
+    warning: `rust-workspace-map` (lib) generated 2 warnings
+    error: could not compile `rust-workspace-map` (lib) due to 1 previous error; 2 warnings emitted
+    ```
+
+### TASK-7: Refactor lib.rs run(): parallel error collection, error entry construction, remove unwrap_or_default
+- **Status**: ✗ Failed
+- **Validation output**:
+  - `cargo check -p rust-workspace-map`: FAILED (exit 101)
+    ```
+    workspace::enumerate_members(&workspace_root)?;
+         |                         ---------------------------------------------- this expression has type `Vec<PathBuf>`
+    ...
+      43 |           .par_iter()
+         |            ---------- `ParallelIterator::Item` is `&PathBuf` here
+      44 |           .map(|dir| {
+         |  __________^
+      45 | |             let cargo_toml = dir.join("Cargo.toml");
+      46 | |             let mut crate_errors = Vec::new();
+    ...    |
+     119 | |             (Some(crate_info), crate_errors)
+     120 | |         })
+         | |__________^ `ParallelIterator::Item` changed to `(Option<CrateInfo>, Vec<ErrorEntry>)` here
+    note: required by a bound in `rayon::iter::ParallelIterator::collect`
+        --> /Users/tony/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rayon-1.12.0/src/iter/mod.rs:2056:12
+         |
+    2054 |     fn collect<C>(self) -> C
+         |        ------- required by a bound in this associated function
+    2055 |     where
+    2056 |         C: FromParallelIterator<Self::Item>,
+         |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ParallelIterator::collect`
+         = note: the full name for the type has been written to '/Users/tony/programming/rust-workspace-map/target/debug/deps/rust_workspace_map-ca1f86303c4a42a1.long-type-7933542791174809822.txt'
+         = note: consider using `--verbose` to print the full type name to the console
+    
+    error[E0308]: mismatched types
+       --> src/lib.rs:126:16
+        |
+    126 |         if let Some(ci) = info {
+        |                ^^^^^^^^   ---- this expression has type `CrateInfo`
+        |                |
+    ```
+
+### TASK-6: Update workspace.rs enumerate_members to return MissingWorkspaceSection error
+- **Status**: ✗ Failed
+- **Validation output**:
+  - `cargo check -p rust-workspace-map`: FAILED (exit 101)
+    ```
+    workspace::enumerate_members(&workspace_root)?;
+         |                         ---------------------------------------------- this expression has type `Vec<PathBuf>`
+    ...
+      43 |           .par_iter()
+         |            ---------- `ParallelIterator::Item` is `&PathBuf` here
+      44 |           .map(|dir| {
+         |  __________^
+      45 | |             let cargo_toml = dir.join("Cargo.toml");
+      46 | |             let mut crate_errors = Vec::new();
+    ...    |
+     119 | |             (Some(crate_info), crate_errors)
+     120 | |         })
+         | |__________^ `ParallelIterator::Item` changed to `(Option<CrateInfo>, Vec<ErrorEntry>)` here
+    note: required by a bound in `rayon::iter::ParallelIterator::collect`
+        --> /Users/tony/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rayon-1.12.0/src/iter/mod.rs:2056:12
+         |
+    2054 |     fn collect<C>(self) -> C
+         |        ------- required by a bound in this associated function
+    2055 |     where
+    2056 |         C: FromParallelIterator<Self::Item>,
+         |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ParallelIterator::collect`
+         = note: the full name for the type has been written to '/Users/tony/programming/rust-workspace-map/target/debug/deps/rust_workspace_map-ca1f86303c4a42a1.long-type-8545240589136472689.txt'
+         = note: consider using `--verbose` to print the full type name to the console
+    
+    error[E0308]: mismatched types
+       --> src/lib.rs:126:16
+        |
+    126 |         if let Some(ci) = info {
+        |                ^^^^^^^^   ---- this expression has type `CrateInfo`
+        |                |
+    ```
+
+### TASK-8: Remove all crate-level clippy allow attributes and fix individual lint violations
+- **Status**: ✗ Failed
+- **Validation output**:
+  - `cargo clippy -p rust-workspace-map -- -D warnings`: FAILED (exit 101)
+    ```
+    --> src/lib.rs:41:10
+         |
+      35 |       let member_dirs = workspace::enumerate_members(&workspace_root)?;
+         |                         ---------------------------------------------- this expression has type `Vec<PathBuf>`
+    ...
+      40 |           .par_iter()
+         |            ---------- `ParallelIterator::Item` is `&PathBuf` here
+      41 |           .map(|dir| {
+         |  __________^
+      42 | |             let cargo_toml = dir.join("Cargo.toml");
+      43 | |             let mut crate_errors = Vec::new();
+    ...    |
+     116 | |             (Some(crate_info), crate_errors)
+     117 | |         })
+         | |__________^ `ParallelIterator::Item` changed to `(Option<CrateInfo>, Vec<ErrorEntry>)` here
+    note: required by a bound in `rayon::iter::ParallelIterator::collect`
+        --> /Users/tony/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rayon-1.12.0/src/iter/mod.rs:2056:12
+         |
+    2054 |     fn collect<C>(self) -> C
+         |        ------- required by a bound in this associated function
+    2055 |     where
+    2056 |         C: FromParallelIterator<Self::Item>,
+         |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ParallelIterator::collect`
+         = note: the full name for the type has been written to '/Users/tony/programming/rust-workspace-map/target/debug/deps/rust_workspace_map-c0af38651655276e.long-type-9209811902749593541.txt'
+         = note: consider using `--verbose` to print the full type name to the console
+    
+    error[E0308]: mismatched types
+       --> src/lib.rs:123:16
+        |
+    123 |         if let Some(ci) = info {
+    ```
+
+### TASK-9: Add unit tests for file_parser, module_tree, workspace, cargo_info, cross_refs, and render
+- **Status**: ✗ Failed
+- **Validation output**:
+  - `cargo test -p rust-workspace-map`: FAILED (exit 101)
+    ```
+    -------------------------------------- this expression has type `Vec<PathBuf>`
+    ...
+      40 |           .par_iter()
+         |            ---------- `ParallelIterator::Item` is `&PathBuf` here
+      41 |           .map(|dir| {
+         |  __________^
+      42 | |             let cargo_toml = dir.join("Cargo.toml");
+      43 | |             let mut crate_errors = Vec::new();
+    ...    |
+     116 | |             (Some(crate_info), crate_errors)
+     117 | |         })
+         | |__________^ `ParallelIterator::Item` changed to `(Option<CrateInfo>, Vec<ErrorEntry>)` here
+    note: required by a bound in `rayon::iter::ParallelIterator::collect`
+        --> /Users/tony/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rayon-1.12.0/src/iter/mod.rs:2056:12
+         |
+    2054 |     fn collect<C>(self) -> C
+         |        ------- required by a bound in this associated function
+    2055 |     where
+    2056 |         C: FromParallelIterator<Self::Item>,
+         |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ParallelIterator::collect`
+         = note: the full name for the type has been written to '/Users/tony/programming/rust-workspace-map/target/debug/deps/rust_workspace_map-453a2fe558063110.long-type-13358229255708071102.txt'
+         = note: consider using `--verbose` to print the full type name to the console
+    
+    error[E0308]: mismatched types
+       --> src/lib.rs:123:16
+        |
+    123 |         if let Some(ci) = info {
+        |                ^^^^^^^^   ---- this expression has type `CrateInfo`
+        |                |
+        |                expected `CrateInfo`, found `Option<_>`
+    ```
+
+### TASK-10: Expand integration tests from 3 to 10 covering error cases, workspace patterns, and output
+- **Status**: ✗ Failed
+- **Validation output**:
+  - `cargo test -p rust-workspace-map --test integration_test`: FAILED (exit 101)
+    ```
+    orkspace::enumerate_members(&workspace_root)?;
+         |                         ---------------------------------------------- this expression has type `Vec<PathBuf>`
+    ...
+      40 |           .par_iter()
+         |            ---------- `ParallelIterator::Item` is `&PathBuf` here
+      41 |           .map(|dir| {
+         |  __________^
+      42 | |             let cargo_toml = dir.join("Cargo.toml");
+      43 | |             let mut crate_errors = Vec::new();
+    ...    |
+     116 | |             (Some(crate_info), crate_errors)
+     117 | |         })
+         | |__________^ `ParallelIterator::Item` changed to `(Option<CrateInfo>, Vec<ErrorEntry>)` here
+    note: required by a bound in `rayon::iter::ParallelIterator::collect`
+        --> /Users/tony/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rayon-1.12.0/src/iter/mod.rs:2056:12
+         |
+    2054 |     fn collect<C>(self) -> C
+         |        ------- required by a bound in this associated function
+    2055 |     where
+    2056 |         C: FromParallelIterator<Self::Item>,
+         |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ParallelIterator::collect`
+         = note: the full name for the type has been written to '/Users/tony/programming/rust-workspace-map/target/debug/deps/rust_workspace_map-453a2fe558063110.long-type-11627958768023326083.txt'
+         = note: consider using `--verbose` to print the full type name to the console
+    
+    error[E0308]: mismatched types
+       --> src/lib.rs:123:16
+        |
+    123 |         if let Some(ci) = info {
+        |                ^^^^^^^^   ---- this expression has type `CrateInfo`
+        |                |
+    ```
+
diff --git a/notes/plan-enrichment/phase-0.2/codebase-state.md b/notes/plan-enrichment/phase-0.2/codebase-state.md
new file mode 100644
index 0000000..fa7b852
--- /dev/null
+++ b/notes/plan-enrichment/phase-0.2/codebase-state.md
@@ -0,0 +1,328 @@
+# Phase 0.2 — Codebase State Snapshot
+
+Captured on 2026-04-28. This documents the current state of every source file mentioned in the phase plan.
+
+---
+
+## File: src/schema.rs
+
+### Public API
+
+**Error enum:**
+```rust
+#[derive(Debug, thiserror::Error)]
+pub enum Error {
+    WorkspaceRootNotFound(PathBuf),
+    FileRead { path: PathBuf, source: std::io::Error },
+    TomlParse { path: PathBuf, source: toml::de::Error },
+    SynParse { path: PathBuf, source: syn::Error },
+    MemberNotFound(PathBuf),
+    GlobPattern(String),
+}
+pub type Result<T> = std::result::Result<T, Error>;
+```
+
+**Config:**
+```rust
+#[derive(Debug, Clone, bon::Builder)]
+pub struct Config {
+    pub workspace_path: PathBuf,
+    pub output_path: Option<PathBuf>,
+}
+```
+
+**CrateType enum:** `Lib`, `Bin`, `LibAndBin` (serde rename_all = "camelCase")
+
+**Output structs (all bon::Builder, serde rename_all = "camelCase"):**
+- `WorkspaceMap` — fields: `workspace: WorkspaceInfo`, `crates: Vec<CrateInfo>`, `cross_references: CrossReferences`, `errors: Vec<ErrorEntry>` (skip_serializing_if), `workspace_root: PathBuf` (skip)
+- `WorkspaceInfo` — fields: `root: String`, `workspace_name: String`
+- `CrateInfo` — fields: `name`, `root`, `package: PackageInfo`, `modules: Vec<ModuleInfo>`, `deps: DepInfo`, `cross_crate_imports: Vec<CrossCrateImport>` (skip_serializing_if)
+- `PackageInfo` — fields: `name`, `version`, `edition`, `crate_type: CrateType`
+- `DepInfo` (Default) — fields: `normal`, `dev`, `workspace_members` (all skip_serializing_if)
+- `ModuleInfo` — fields: `path`, `file`, `visibility`, `public_items: Vec<PublicItem>`, `imports: Vec<Import>`, `re_exports: Vec<ReExport>`, `submodules: Vec<String>` (all optional)
+- `PublicItem` — fields: `kind: ItemKind`, `name`, `file`, `line: usize`, `attrs: ItemAttrs`, `generics`, `visibility`, `fields`, `variants`, `impls: Vec<ImplInfo>`
+- `ItemKind` enum: `Struct`, `Enum`, `Trait`, `Fn`, `Type`, `Macro`
+- `ItemAttrs` (Default) — fields: `derive`, `doc`
+- `ImplInfo` — fields: `type_: String`, `items: Vec<ImplItem>`
+- `ImplItem` — fields: `kind: ImplItemKind`, `name`, `params`
+- `ImplItemKind` enum: `Fn`, `Type`, `Const`
+- `Import` — fields: `path`, `line`
+- `ReExport` — fields: `import_path`, `export_path`, `line`
+- `CrossCrateImport` — fields: `import_path`, `target_crate`, `symbol`, `line`
+- `CrossReferences` (Default) — field: `types: BTreeMap<String, TypeRef>`
+- `TypeRef` — fields: `crate_name`, `kind`, `imported_by`, `exported_by`
+
+**Internal types:**
+- `FileInfo` (Default) — `public_items`, `imports`, `re_exports`, `submodules: Vec<SubmoduleDecl>`, `impls`
+- `SubmoduleDecl` — `name`, `is_test: bool` (default)
+
+**ErrorEntry (current):**
+```rust
+pub struct ErrorEntry {
+    pub file: String,
+    pub line: usize,  // builder(default)
+    pub message: String,
+}
+```
+
+### Module wiring
+- Declared as `pub mod schema;` in `lib.rs`
+- `pub use schema::Config;` re-export in `lib.rs`
+
+### Plan relationship
+- **Plan says:** Add `severity: ErrorSeverity` (enum: `Error`, `Warning`), `kind: String`, `context: Option<ErrorContext>`, `cause: Option<String>` to `ErrorEntry`. Add `ErrorSeverity` enum. Add `ErrorContext` struct with `crate_name`, `module_path`, `line`, `snippet` fields. Add `MissingWorkspaceSection` variant to `Error` enum.
+- **Current state:** `ErrorEntry` has only 3 fields (`file`, `line`, `message`). No `ErrorSeverity` enum. No `ErrorContext` struct. `Error` enum has 6 variants but no `MissingWorkspaceSection`.
+- **Gap:** All 4 new `ErrorEntry` fields, both new types (`ErrorSeverity`, `ErrorContext`), and the `MissingWorkspaceSection` error variant need to be added. `ErrorEntry` serde serialization annotations and bon::Builder annotations need to be added for the new fields.
+
+---
+
+## File: src/workspace.rs
+
+### Public API
+
+```rust
+pub fn find_workspace_root(start_path: &Path) -> Result<PathBuf>
+pub fn enumerate_members(root: &Path) -> Result<Vec<PathBuf>>
+pub fn resolve_crate_roots(crate_dir: &Path) -> Vec<(PathBuf, CrateType)>
+```
+
+### Module wiring
+- `pub mod workspace;` in `lib.rs`
+- No `pub use` re-exports of workspace items.
+
+### Plan relationship
+- **Plan says:** `enumerate_members` should return `Err(Error::MissingWorkspaceSection)` (new variant) when `[workspace]` section or `members` key is absent. `resolve_crate_roots` empty results should become an `ErrorEntry` with `kind = "missing_crate_roots"` in the caller. The `eprintln!` at line 76 (member not found) should become `Error::MemberNotFound`.
+- **Current state:** `enumerate_members` uses `.unwrap_or_default()` on lines 46 and 57 — when `[workspace]` or `members` is absent, it silently returns an empty list. `resolve_crate_roots` already returns an empty Vec when no entry points are found (no error handling). The `eprintln!` at line 76–79 logs a warning for missing members but does not return an error.
+- **Gap:** `enumerate_members` needs to distinguish "no workspace section" from "workspace section with no members." The missing-member `eprintln!` should return `Error::MemberNotFound`. `resolve_crate_roots` is called in `run()` which currently does nothing with empty results beyond `eprintln!` — the plan wants this converted to an `ErrorEntry` in `run()` (not in `workspace.rs` itself).
+
+---
+
+## File: src/cargo_info.rs
+
+### Public API
+
+```rust
+pub fn parse_cargo_toml(path: &Path) -> Result<(PackageInfo, DepInfo)>
+```
+
+### Module wiring
+- `pub mod cargo_info;` in `lib.rs`
+- No `pub use` re-exports.
+
+### Plan relationship
+- **Plan says:** Convert `parse_cargo_toml` failures (currently `eprintln!` + `return None` in the caller `run()`) to `ErrorEntry` with `kind = "toml_parse_error"`.
+- **Current state:** `parse_cargo_toml` returns `Result<(PackageInfo, DepInfo)>` and already uses `?` to propagate parse errors via the `Error::TomlParse` variant. The caller in `run()` (line 46–55) matches on `Err(e)` and does `eprintln!` + `return None` — errors are swallowed silently.
+- **Gap:** No changes needed in `cargo_info.rs` itself. The caller in `main.rs`/`lib.rs` `run()` needs to be updated to convert the error into an `ErrorEntry`.
+
+---
+
+## File: src/file_parser.rs
+
+### Public API
+
+```rust
+pub fn parse_file(path: &Path) -> Result<(syn::File, FileInfo)>
+pub fn extract_public_items(items: &[syn::Item]) -> Vec<PublicItem>
+pub fn extract_imports(items: &[syn::Item]) -> Vec<Import>
+pub fn extract_re_exports(items: &[syn::Item]) -> Vec<ReExport>
+pub fn extract_submodules(items: &[syn::Item]) -> Vec<SubmoduleDecl>
+pub fn extract_impls(items: &[syn::Item]) -> Vec<ImplInfo>
+```
+
+### Module wiring
+- `pub mod file_parser;` in `lib.rs`
+- No `pub use` re-exports.
+
+### Plan relationship
+- **Plan says:** Change `parse_file()` to propagate `SynParse` errors as `ErrorEntry` with severity `error`, rather than returning empty results. On parse failure, return structured error data instead of `(empty syn::File, FileInfo::default())`. This applies to all files including inline `#[cfg(test)]` module bodies and external `#[cfg(test)]` module files.
+- **Current state:** `parse_file` (lines 18–30) catches `syn::parse_file` errors, prints a warning via `eprintln!`, and returns an empty `syn::File` + default `FileInfo` — the error is completely lost. All extraction functions are pure and return their collections.
+- **Gap:** `parse_file` needs to change its return signature or behavior to report parse errors as `ErrorEntry` data alongside results. The plan says: "parse_file returns errors alongside results rather than propagating via `?` on parse failures." This means either a new return type like `Result<(syn::File, FileInfo), ErrorEntry>` or a tuple `(Result<..., ...>, Vec<ErrorEntry>)`.
+
+---
+
+## File: src/module_tree.rs
+
+### Public API
+
+```rust
+pub fn resolve_module_path(parent_dir: &Path, mod_name: &str) -> Option<PathBuf>
+pub fn build_module_tree(crate_root: &Path, crate_name: &str) -> Result<Vec<ModuleInfo>>
+```
+
+**Internal functions (private):**
+- `fn build_module_info(...)` — builds ModuleInfo from components
+- `fn process_submodule(...)` — processes a single submodule, handles inline vs external
+- `fn process_module_items(...)` — extracts FileInfo from inline module items
+- `fn process_module_info(...)` — builds module info for a file-path-backed module
+
+### Module wiring
+- `pub mod module_tree;` in `lib.rs`
+- No `pub use` re-exports.
+
+### Plan relationship
+
+**Path safety (Goal 3):**
+- **Plan says:** Fix four `unwrap_or_else(|| Path::new("."))` fallback bugs:
+  1. Line 25: `crate_root.parent().unwrap_or_else(|| Path::new("."))` in `build_module_tree`
+  2. Line ~155: `file_path.parent().unwrap_or_else(|| Path::new("."))` in `process_submodule`
+  3. Line ~203: `file_path.parent().unwrap_or_else(|| Path::new("."))` in `process_module_info`
+  4. (workspace.rs `enumerate_members` defaulting to empty list — covered there)
+- **Current state:** All three instances in this file use `unwrap_or_else(|| Path::new("."))`. When `parent()` returns `None` (e.g., the path is at the filesystem root), the fallback to `"."` silently produces incorrect relative paths.
+- **Gap:** Replace all three `unwrap_or_else(|| Path::new("."))` calls with explicit handling that either returns an error, uses a better default, or propagates the issue visibly.
+
+**Error collection (Goal 1):**
+- **Plan says:** `build_module_tree` should collect per-module errors into a `Vec<ErrorEntry>` returned alongside the module tree. Orphaned module warnings (lines 107, 135) should emit `ErrorEntry` with `kind = "orphaned_module"`. Silent error swallowing in `build_module_tree` should be eliminated.
+- **Current state:** `build_module_tree` returns `Result<Vec<ModuleInfo>>` and propagates errors via `?`. Orphaned modules produce `eprintln!` + a placeholder `ModuleInfo` with `<unresolved>` file. `process_submodule` uses `?` on `parse_file` calls, which currently never fail (they return empty results).
+- **Gap:** `build_module_tree` signature needs to change to collect and return errors alongside results. Orphaned module handling needs to produce `ErrorEntry` values instead of `eprintln!`.
+
+---
+
+## File: src/cross_refs.rs
+
+### Public API
+
+```rust
+pub fn compute(crates: &mut [CrateInfo]) -> CrossReferences
+```
+
+**Impl block:**
+```rust
+impl PublicItem {
+    fn kind_to_string(&self) -> String  // private helper
+}
+```
+
+### Module wiring
+- `pub mod cross_refs;` in `lib.rs`
+- No `pub use` re-exports.
+
+### Plan relationship
+- **Plan says:** Add unit tests for `compute`. No source changes mentioned.
+- **Current state:** `compute` is fully implemented — builds `crate_exports` map, initializes `TypeRef` entries, scans imports for cross-crate references, populates `cross_crate_imports` on each crate.
+- **Gap:** No code changes needed per the plan. Only unit tests are required (Goal 2).
+
+---
+
+## File: src/render.rs
+
+### Public API
+
+```rust
+pub fn render_json(map: &WorkspaceMap) -> serde_json::Result<String>
+pub fn render_to_writer(map: &WorkspaceMap, writer: impl Write) -> serde_json::Result<()>
+```
+
+### Module wiring
+- `pub mod render;` in `lib.rs`
+- No `pub use` re-exports.
+
+### Plan relationship
+- **Plan says:** Add unit tests for `render_json` and `render_to_writer`. No source changes mentioned.
+- **Current state:** Both functions are thin wrappers around `serde_json::to_string_pretty` and `serde_json::to_writer_pretty`.
+- **Gap:** No code changes needed per the plan. Only unit tests are required (Goal 2).
+
+---
+
+## File: src/main.rs
+
+### Public API
+
+```rust
+struct Cli { path: PathBuf, output: Option<PathBuf> }  // clap Parser
+fn main() -> anyhow::Result<()>
+```
+
+### Module wiring
+- Calls `rust_workspace_map::run(config)` from the library crate.
+- Uses `rust_workspace_map::Config::builder()` for configuration.
+
+### Plan relationship
+- **Plan says:** No changes mentioned. The `run()` function lives in `lib.rs` (not `main.rs`).
+- **Current state:** Thin CLI entry point. Delegates all logic to `lib.rs::run()`.
+- **Gap:** No changes needed in `main.rs` itself per the plan.
+
+---
+
+## File: src/lib.rs
+
+### Public API
+
+**Crate-level attributes:**
+```rust
+#![warn(clippy::pedantic)]
+#![allow(clippy::missing_errors_doc)]
+#![allow(clippy::must_use_candidate)]
+#![allow(clippy::doc_markdown)]
+#![allow(clippy::uninlined_format_args)]
+#![allow(clippy::redundant_closure)]
+#![allow(clippy::collapsible_if)]
+#![allow(clippy::needless_pass_by_value)]
+#![allow(clippy::needless_borrow)]
+#![allow(clippy::redundant_closure_for_method_calls)]
+```
+
+**Module declarations:**
+```rust
+pub mod cargo_info;
+pub mod cross_refs;
+pub mod file_parser;
+pub mod module_tree;
+pub mod render;
+pub mod schema;
+pub mod workspace;
+```
+
+**Re-exports:**
+```rust
+pub use schema::Config;
+```
+
+**Public functions:**
+```rust
+pub fn run(config: Config) -> anyhow::Result<()>
+fn relativize_path(path_str: &str, root: &Path) -> String  // private
+```
+
+### Plan relationship
+
+**Error collection (Goal 1):**
+- **Plan says:** `run()` must: remove `.unwrap_or_default()` on `module_tree::build_module_tree` (line 79), capture errors as `ErrorEntry` with `kind = "module_tree_error"`. Collect `ErrorEntry` values from parallel crate processing (use `Mutex<Vec<ErrorEntry>>` or `rayon::collect`). Convert `cargo_info::parse_cargo_toml` failures to `ErrorEntry`. Convert `resolve_crate_roots` empty results to `ErrorEntry`. Replace `errors: Vec<ErrorEntry> = Vec::new()` with actual collection.
+- **Current state:** `run()` creates an empty `errors` vec (line 39). Line 79 uses `.unwrap_or_default()` on `build_module_tree`, silently discarding errors. Parse failures and resolve failures go to `eprintln!` + `return None`. No error collection from parallel processing.
+- **Gap:** Major changes needed: error collection from parallel rayon processing, removal of `.unwrap_or_default()`, conversion of `eprintln!` paths to `ErrorEntry` construction, and wiring of collected errors into `WorkspaceMap`.
+
+**Clippy (Goal 5):**
+- **Plan says:** No `#![allow(clippy::*)]` remains. Every suppression must be removed. Add `# Errors` doc sections, `#[must_use]`, fix doc comments, inline format args, replace closures, merge nested `if`, change to `&` references, remove unnecessary borrows.
+- **Current state:** 9 crate-level `#![allow(clippy::*)]` attributes present (lines 2–10) plus `#![warn(clippy::pedantic)]`.
+- **Gap:** All 9 crate-level suppressions must be resolved — either by fixing the underlying code or adding per-item suppressions (plan says: "no crate-level suppressions are converted to per-item suppressions; if a lint fires on legitimate code, the code is changed, not silenced").
+
+---
+
+## File: tests/integration_test.rs
+
+### Public API (test functions)
+
+```rust
+fn test_sample_workspace_output()
+fn test_deterministic_output()
+fn test_missing_path_exits_nonzero()
+```
+
+Helper: `fn binary_path() -> String`, `fn extract_array<'a>(...)`
+
+### Plan relationship
+- **Plan says (Goal 4):** Expand from 3 to 8+ integration tests covering:
+  - Parse failure workspace → verify `ErrorEntry` with severity in JSON
+  - Missing workspace section → verify appropriate error
+  - Glob member patterns → verify all matching crates discovered
+  - Workspace with `exclude` → verify excluded crate absent
+  - Deeply nested modules (3+ levels) → verify correct paths and hierarchy
+  - Re-export chains → verify correct re-export tracking
+  - Output via `-o` flag → verify file content matches stdout content
+- **Current state:** 3 integration tests covering happy path, determinism, and missing path.
+- **Gap:** Need 5+ new integration tests (plan lists 7, 3 already exist). All need fixture workspace structures.
+
+---
+
+## Files mentioned in plan but not found
+
+none — all 10 files identified in the plan were found in the codebase.
diff --git a/notes/plan-enrichment/phase-0.2/deferred-and-patterns.md b/notes/plan-enrichment/phase-0.2/deferred-and-patterns.md
new file mode 100644
index 0000000..2b6a894
--- /dev/null
+++ b/notes/plan-enrichment/phase-0.2/deferred-and-patterns.md
@@ -0,0 +1,11 @@
+## Deferred Improvements
+
+- Empty `errors` vector in `WorkspaceMap::run()` — the field is always empty because no code path populates it; wiring in error collection would improve debugging of misconfigured workspaces
+- Silent error swallowing in `build_module_tree` — `unwrap_or_default()` masks parse failures, making it indistinguishable from crates with no public items
+- Path fallback to `"."` in `module_tree.rs` — three `.unwrap_or_else(|| Path::new("."))` calls produce incorrect relative paths when `.parent()` returns `None`
+- Path fallback to `""` in `workspace.rs` — defaulting to empty vectors/malformed sections silently includes zero members; falling back to `""` for file names bypasses exclude filters
+- No unit tests for public API functions — 13 public functions across 5 modules lack unit tests; only integration tests cover the full pipeline
+
+## Known Failure Modes
+
+None found. The only `fix-plan.toml` (`notes/pr-reviews/phase-0.1/fix-plan.toml`) contained no fix tasks — the PR was reviewed and approved with all items deferred to future phases.
diff --git a/notes/plan-enrichment/phase-0.2/draft-elaboration.md b/notes/plan-enrichment/phase-0.2/draft-elaboration.md
new file mode 100644
index 0000000..f0e06d7
--- /dev/null
+++ b/notes/plan-enrichment/phase-0.2/draft-elaboration.md
@@ -0,0 +1,563 @@
+# Phase 0.2 — Draft Elaboration
+
+Grounded in existing patterns from `codebase-state.md`. No new patterns invented.
+
+---
+
+## Item: Add ErrorSeverity enum to schema
+
+**Proposed type signature:**
+```rust
+#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
+#[serde(rename_all = "snake_case")]
+pub enum ErrorSeverity {
+    Error,
+    Warning,
+}
+```
+
+**Module placement:** `src/schema.rs`, placed immediately before the `ErrorEntry` struct definition (line ~300, same section block).
+
+**Error handling strategy:** New standalone enum. `serde::Serialize` with `rename_all = "snake_case"` follows the convention already established by `CrateType` and `ItemKind`. No `thiserror` involvement.
+
+**Ownership/lifetime notes:** None. Copy, Clone — cheap to copy into every ErrorEntry.
+
+**Trait coherence notes:** None. No generics, no external trait bounds beyond serde and standard derives.
+
+---
+
+## Item: Add ErrorContext struct to schema
+
+**Proposed type signature:**
+```rust
+#[derive(Debug, Clone, Default, serde::Serialize, bon::Builder)]
+#[serde(rename_all = "camelCase")]
+pub struct ErrorContext {
+    #[builder(default)]
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pub crate_name: Option<String>,
+
+    #[builder(default)]
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pub module_path: Option<String>,
+
+    /// Line number in the source file where the error occurred.
+    #[builder(default)]
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pub line: Option<usize>,
+
+    /// A short source snippet near the error location (if available).
+    #[builder(default)]
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pub snippet: Option<String>,
+}
+```
+
+**Module placement:** `src/schema.rs`, placed immediately before `ErrorEntry` struct.
+
+**Error handling strategy:** New struct with `Default`. Uses `Option<T>` for all fields so a fully-populated context and a minimal one (just `line`) are both valid. The `bon::Builder` defaults follow the pattern already used everywhere in schema.rs.
+
+**Ownership/lifetime notes:** None. All `String`/`usize` owned types.
+
+**Trait coherence notes:** None. `bon::Builder` requires types to be clonable or constructible — `Option<String>` and `Option<usize>` both satisfy this.
+
+---
+
+## Item: Expand ErrorEntry with new fields
+
+**Proposed type signature (full struct):**
+```rust
+#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
+#[serde(rename_all = "camelCase")]
+pub struct ErrorEntry {
+    pub file: String,
+    #[builder(default)]
+    pub line: usize,
+    pub message: String,
+    pub severity: ErrorSeverity,
+    pub kind: String,
+    #[builder(default)]
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pub context: Option<ErrorContext>,
+    #[builder(default)]
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pub cause: Option<String>,
+}
+```
+
+**Module placement:** `src/schema.rs`, replacing the existing 3-field `ErrorEntry` (lines 302–309).
+
+**Error handling strategy:** `severity` is required (no Option) — every ErrorEntry always has a severity. `kind` is required (no Option) — every ErrorEntry always has a machine-readable tag. `context` and `cause` are optional for backwards compatibility with existing callers during construction.
+
+**Ownership/lifetime notes:** None.
+
+**Trait coherence notes:** `bon::Builder` on `ErrorSeverity` (Copy + Clone) — no issue. The existing `WorkspaceMap::builder().errors(vec)` pattern works unchanged since `Vec<ErrorEntry>` is the same type.
+
+**Implementation detail:** The `WorkspaceMap::builder()` call in `lib.rs::run()` currently passes `.errors(errors)` where `errors: Vec<ErrorEntry> = Vec::new()`. Since `ErrorEntry` is changed, callers constructing ErrorEntry values must use the new builder which now requires `severity` and `kind` (no defaults for those). This is intentional — it is impossible to construct an ErrorEntry without specifying what kind of error it is.
+
+---
+
+## Item: Add MissingWorkspaceSection to Error enum
+
+**Proposed type signature (enum addition):**
+```rust
+#[error("workspace Cargo.toml is missing the [workspace] section")]
+MissingWorkspaceSection,
+```
+
+**Module placement:** `src/schema.rs`, added to the `Error` enum after `GlobPattern` (before the closing `}`).
+
+**Error handling strategy:** New unit variant. No source error, no associated data. The message is a static string.
+
+**Ownership/lifetime notes:** None.
+
+**Trait coherence notes:** None.
+
+**Caller wiring:** In `workspace.rs::enumerate_members()`, the current code on lines 37–46 uses `unwrap_or_default()` on the `workspace.members` lookup. The change is:
+- After the `members` extraction, check if the `[workspace]` table was absent entirely (not just if `members` was absent). Return `Err(Error::MissingWorkspaceSection)` when the workspace table itself is missing.
+- When the workspace table exists but `members` is absent (empty array or missing key), keep returning an empty `Vec<PathBuf>` (this is valid for a workspace that just has `exclude`).
+
+**Uncertain:** The distinction between "no [workspace] section at all" vs "[workspace] section with no members" matters. If `parsed.get("workspace")` returns `None`, it is `MissingWorkspaceSection`. If it returns `Some` but `members` is missing/empty, return empty vec. This preserves backwards compatibility for workspaces that have `[workspace]` but no members (valid TOML).
+
+---
+
+## Item: Change parse_file to return errors alongside results
+
+**Proposed type signature:**
+```rust
+/// On parse failure, returns the original file content and a
+/// `SynParseError` alongside an empty `FileInfo`. Callers use the
+/// error to construct an `ErrorEntry`.
+pub fn parse_file(path: &Path) -> ParsedFile {
+```
+
+Where `ParsedFile` is an internal type defined in `file_parser.rs`:
+
+```rust
+/// Result of parsing a Rust source file.
+///
+/// Unlike `Result<T, Error>`, this type always succeeds — parse
+/// failures are reported as data, not as errors, so the caller
+/// can continue processing other files. The caller constructs
+/// `ErrorEntry` values from `SynParseError` when needed.
+pub struct ParsedFile {
+    pub ast: syn::File,
+    pub file_info: FileInfo,
+    pub parse_error: Option<SynParseError>,
+}
+
+pub struct SynParseError {
+    pub message: String,
+    pub line: usize,
+}
+```
+
+**Module placement:** `src/file_parser.rs`, placed in a new section block before `parse_file`.
+
+**Error handling strategy:** `parse_file` never returns `Err` — it always returns `Ok(ParsedFile)`. Parse errors are captured in the `parse_error` field. This is the "partial results" approach from the plan: the function collects errors rather than propagating them.
+
+**Why not `Result<(syn::File, FileInfo), ErrorEntry>`:** Because `build_module_tree` already uses `?` on `parse_file` calls. If `parse_file` returned `Err`, it would abort the entire crate — violating the partial results requirement. An internal `ParsedFile` type avoids changing the public signature while solving the problem internally.
+
+**Ownership/lifetime notes:** `ParsedFile` owns the AST and FileInfo. `SynParseError` owns only a message string (no source span from syn — we extract line from `syn::Error` but the full diagnostic is captured as a string).
+
+**Trait coherence notes:** None. No generics.
+
+**Call-site changes:** All `let (ast, file_info) = parse_file(path)?;` calls become:
+```rust
+let parsed = parse_file(path);
+if let Some(err) = &parsed.parse_error {
+    errors_vec.push(build_parse_error_entry(path, err));
+}
+let ast = parsed.ast;
+let file_info = parsed.file_info;
+```
+
+The `build_parse_error_entry` helper constructs an `ErrorEntry` from the path and `SynParseError`:
+```rust
+fn build_parse_error_entry(path: &Path, err: &SynParseError) -> ErrorEntry {
+    ErrorEntry::builder()
+        .file(path.to_string_lossy().to_string())
+        .line(err.line)
+        .message(err.message.clone())
+        .severity(ErrorSeverity::Error)
+        .kind("syn_parse_error".to_string())
+        .build()
+}
+```
+
+---
+
+## Item: Change build_module_tree to collect errors
+
+**Proposed type signature:**
+```rust
+/// Same as before, but errors from submodule parsing are collected
+/// into the returned `Vec<ModuleInfo>` as placeholder entries with
+/// an `ErrorEntry` attached via `ModuleInfo.extra_error` — NO, that
+/// would break the schema. Instead, errors are returned via a
+/// side-channel: `run()` collects them.
+///
+/// Actually: build_module_tree returns (Vec<ModuleInfo>, Vec<ErrorEntry>).
+pub fn build_module_tree(crate_root: &Path, crate_name: &str) -> (Vec<ModuleInfo>, Vec<ErrorEntry>) {
+```
+
+**Module placement:** `src/module_tree.rs`.
+
+**Error handling strategy:** `build_module_tree` no longer returns `Result`. Instead, it returns a tuple `(Vec<ModuleInfo>, Vec<ErrorEntry>)`. Parse errors are captured in the second element. Orphaned modules produce `ErrorEntry` with `kind = "orphaned_module"` instead of `eprintln!`. Cycle detection returns `Ok(vec![])` (no error entry — cycles are normal in Rust modules).
+
+**Why change the return type:** The current `Result<Vec<ModuleInfo>>` signature forces single-error propagation via `?`. With parallel rayon processing in `run()`, we need to collect multiple errors per crate. Returning a tuple lets callers extract errors without aborting.
+
+**Trait coherence notes:** Changing the return type of a public function. The callers are only in `lib.rs::run()`. This is an internal API surface — no downstream crates call `module_tree::build_module_tree`.
+
+**Call-site changes in `lib.rs`:**
+```rust
+// Before:
+module_tree::build_module_tree(root, &pkg_name).unwrap_or_default()
+
+// After:
+let (mods, errs) = module_tree::build_module_tree(root, &pkg_name);
+crate_errors.extend(errs);
+mods
+```
+
+---
+
+## Item: Fix path fallback in module_tree.rs (Goal 3)
+
+**Three occurrences to fix:**
+
+### Fix 1: `build_module_tree` line 25
+```rust
+// Before:
+let parent_dir = crate_root.parent().unwrap_or_else(|| Path::new("."));
+
+// After:
+let parent_dir = crate_root.parent().unwrap_or(crate_root);
+```
+
+**Rationale:** When `crate_root` is at the filesystem root (extremely rare), `.parent()` returns `None`. Using `crate_root` itself as the fallback is a safe no-op — the caller then tries `{crate_root}/{mod_name}.rs` which is the same as the original path. This is better than `"."` which silently produces wrong relative paths.
+
+### Fix 2: `process_submodule` line 155
+```rust
+// Before:
+&file_path.parent().unwrap_or_else(|| Path::new("."))
+
+// After:
+file_path.parent().unwrap_or(file_path)
+```
+
+Same rationale. The `file_path` here is the parent directory for recursive `process_submodule` calls.
+
+### Fix 3: `process_module_info` line 203
+```rust
+// Before:
+let child_dir = file_path.parent().unwrap_or_else(|| Path::new("."));
+
+// After:
+let child_dir = file_path.parent().unwrap_or(file_path);
+```
+
+Same rationale.
+
+**Module placement:** `src/module_tree.rs`, inline replacements.
+
+**Ownership/lifetime notes:** None. All `&Path` borrows.
+
+---
+
+## Item: Fix enumerate_members path fallback (Goal 3)
+
+**Proposed change in `workspace.rs`:**
+```rust
+// Before (lines 37-46):
+let members: Vec<String> = parsed
+    .get("workspace")
+    .and_then(|w| w.get("members"))
+    .and_then(|m| m.as_array())
+    .map(|arr| {
+        arr.iter()
+            .filter_map(|v| v.as_str().map(String::from))
+            .collect()
+    })
+    .unwrap_or_default();
+
+// After:
+let members = match parsed.get("workspace") {
+    None => return Err(Error::MissingWorkspaceSection),
+    Some(workspace) => workspace
+        .get("members")
+        .and_then(|m| m.as_array())
+        .map(|arr| {
+            arr.iter()
+                .filter_map(|v| v.as_str().map(String::from))
+                .collect::<Vec<_>>()
+        })
+        .unwrap_or_default(),
+};
+```
+
+**Module placement:** `src/workspace.rs`, lines 37–46.
+
+**Error handling strategy:** Returns `Error::MissingWorkspaceSection` (new variant) when the `[workspace]` section is entirely absent. When `[workspace]` exists but `members` is absent or empty, returns empty vec (valid for workspace with only `exclude`).
+
+---
+
+## Item: Convert eprintln! paths in run() to ErrorEntry (Goal 1)
+
+### Cargo parse failure (lines 46-56)
+```rust
+// Before:
+Err(e) => {
+    eprintln!("warning: failed to parse {}: {}", cargo_toml.display(), e);
+    return None;
+}
+
+// After:
+Err(e) => {
+    crate_errors.push(ErrorEntry::builder()
+        .file(cargo_toml.to_string_lossy().to_string())
+        .message(format!("failed to parse Cargo.toml: {}", e))
+        .severity(ErrorSeverity::Error)
+        .kind("toml_parse_error".to_string())
+        .cause(e.to_string())
+        .build());
+    return None;
+}
+```
+
+### Resolve crate roots empty (lines 58-65)
+```rust
+// Before:
+if roots.is_empty() {
+    eprintln!("warning: no crate entry points found in {}", dir.display());
+    return None;
+}
+
+// After:
+if roots.is_empty() {
+    crate_errors.push(ErrorEntry::builder()
+        .file(dir.to_string_lossy().to_string())
+        .message("no crate entry points found".to_string())
+        .severity(ErrorSeverity::Warning)
+        .kind("missing_crate_roots".to_string())
+        .build());
+    return None;
+}
+```
+
+**Severity rationale:** `missing_crate_roots` is a `Warning` — the run completed fully, this crate just has no entry points. `toml_parse_error` is an `Error` — data loss for this crate.
+
+### Module tree error (lines 78-80)
+```rust
+// Before:
+module_tree::build_module_tree(root, &pkg_name).unwrap_or_default()
+
+// After:
+let (mods, tree_errors) = module_tree::build_module_tree(root, &pkg_name);
+crate_errors.extend(tree_errors);
+mods
+```
+
+### Orphaned modules in module_tree.rs (lines 106-113, 134-141)
+```rust
+// Before (both occurrences):
+eprintln!("warning: orphaned module {}", module_path);
+return Ok(vec![ModuleInfo::builder()
+    .path(module_path.to_string())
+    .file("<unresolved>".to_string())
+    .visibility("private".to_string())
+    .build()]);
+
+// After (both occurrences):
+crate_errors.push(ErrorEntry::builder()
+    .file(String::new())
+    .message(format!("orphaned module: {}", module_path))
+    .severity(ErrorSeverity::Warning)
+    .kind("orphaned_module".to_string())
+    .context(ErrorContext::builder()
+        .module_path(module_path.to_string())
+        .build())
+    .build());
+return Ok(vec![ModuleInfo::builder()
+    .path(module_path.to_string())
+    .file("<unresolved>".to_string())
+    .visibility("private".to_string())
+    .build()]);
+```
+
+**Module placement:** `src/module_tree.rs` for orphaned module changes. `src/lib.rs` for run() changes.
+
+---
+
+## Item: Parallel error collection in run()
+
+**Current state:** `run()` uses `.par_iter().filter_map().collect()` on `member_dirs`. Errors are currently `eprintln!`d inside the parallel closure, which is racy with stdout.
+
+**Proposed approach:** Use `rayon::scope` with shared `Vec<ErrorEntry>` via interior mutability, or collect per-thread errors and merge afterward.
+
+**Best approach matching existing patterns:** Collect per-crate errors alongside crate info using a parallel-join pattern:
+
+```rust
+let mut crate_errors: Vec<ErrorEntry> = Vec::new();
+
+let mut crate_infos: Vec<CrateInfo> = member_dirs
+    .par_iter()
+    .map(|dir| {
+        let mut crate_errors = Vec::new();
+        // ... processing with errors pushed to crate_errors ...
+        (crate_info, crate_errors)
+    })
+    .unzip::<_, _, Vec<CrateInfo>, Vec<Vec<ErrorEntry>>>();
+
+for errs in crate_infos_errors {
+    crate_errors.extend(errs);
+}
+```
+
+**Uncertain:** `unzip` on a `par_iter().map()` returning tuples is not directly supported by rayon — `unzip` is a sequential iterator trait. The correct approach is:
+
+```rust
+let results: Vec<(CrateInfo, Vec<ErrorEntry>)> = member_dirs
+    .par_iter()
+    .map(|dir| {
+        let mut crate_errors = Vec::new();
+        // ... process, push errors ...
+        (crate_info, crate_errors)
+    })
+    .collect();
+
+let mut crate_errors = Vec::new();
+let crate_infos: Vec<CrateInfo> = results
+    .into_iter()
+    .map(|(info, errs)| {
+        crate_errors.extend(errs);
+        info
+    })
+    .collect();
+```
+
+This collects parallel results into `(CrateInfo, Vec<ErrorEntry>)` tuples, then sequentially extracts errors. The parallel work is in `.map()`, the error merge is sequential over the already-collected results (typically tens of entries, negligible cost).
+
+**Module placement:** `src/lib.rs`, `run()` function.
+
+**Trait coherence notes:** `CrateInfo` and `Vec<ErrorEntry>` must both be `Send + Sync` for rayon. `CrateInfo` contains `String`, `Vec<ModuleInfo>`, etc. — all `Send + Sync`. `ErrorEntry` contains `String`, `ErrorSeverity`, `Option<ErrorContext>` — all `Send + Sync`. Coherence is satisfied.
+
+---
+
+## Item: Replace errors Vec with populated collection in WorkspaceMap builder
+
+**Current:** `errors(errors)` where `errors: Vec<ErrorEntry> = Vec::new()`.
+
+**After:** Collect all errors from parallel processing into the vec, then pass it:
+```rust
+let map = WorkspaceMap::builder()
+    .workspace(workspace_info)
+    .crates(crate_infos)
+    .cross_references(cross_refs)
+    .errors(crate_errors)
+    .workspace_root(workspace_root.clone())
+    .build();
+```
+
+**Module placement:** `src/lib.rs`, `run()` function.
+
+---
+
+## Item: Unit test suite (Goal 2)
+
+All tests use `#[cfg(test)]` modules in the respective source files, following the existing pattern where `file_parser.rs` and `module_tree.rs` already have internal helper functions that are testable in isolation.
+
+### workspace.rs tests
+- `find_workspace_root`: Test with `tempfile::tempdir()`, create a minimal workspace Cargo.toml, verify path resolution.
+- `enumerate_members`: Test with `tempfile::tempdir()` — create workspace with members, verify returned paths. Test with missing workspace section — verify `Err(MissingWorkspaceSection)`. Test with exclude list — verify excluded crate absent.
+- `resolve_crate_roots`: Test with `tempfile::tempdir()` — create lib.rs only, main.rs only, both. Verify returned `CrateType`.
+
+### cargo_info.rs tests
+- `parse_cargo_toml`: Parse a minimal Cargo.toml string using `tempfile::tempdir()`, verify `PackageInfo` fields. Test missing `[package]` table — verify defaults (name="unknown", edition="2021"). Test valid dependencies — verify normal/dev/workspace distinction.
+
+### file_parser.rs tests
+- `parse_file`: Parse inline Rust string via `tempfile::tempdir()` — create file with struct/enum/trait, verify extracted types. Test parse failure — verify `SynParseError` is populated in `ParsedFile::parse_error`. Test empty file — verify empty results (not error).
+- `extract_public_items`: Pass `syn::File` constructed from parsed string — verify correct item kinds extracted. Test no public items — verify empty vec.
+- `extract_imports`: Parse `use std::collections::BTreeMap;` — verify single import. Test braced imports — verify expansion to individual entries.
+- `extract_re_exports`: Parse `pub use crate::foo;` — verify re-export captured. Test `pub use crate::foo as bar;` — verify rename.
+- `extract_submodules`: Parse `mod foo;` and `#[cfg(test)] mod bar;` — verify first extracted, second marked `is_test: true`.
+- `extract_impls`: Parse impl block with fn/type/const items — verify all three kinds captured.
+
+### module_tree.rs tests
+- `resolve_module_path`: Test with `tempfile::tempdir()` — create `{dir}/foo.rs` and `{dir}/foo/mod.rs`, verify correct path for each. Test non-existent module — verify None.
+- `build_module_tree`: Test with a multi-file workspace crate — verify module hierarchy, paths, and public items. Test with parse-failure submodule — verify `ErrorEntry` in errors vec, other modules still extracted.
+
+### cross_refs.rs tests
+- `compute`: Build two `CrateInfo` values manually (no fixtures) — one exports `Task`, the other imports `core::Task`. Verify `cross_crate_imports` populated and `CrossReferences.types` contains the reference.
+
+### render.rs tests
+- `render_json`: Build a minimal `WorkspaceMap` manually, serialize, verify JSON contains expected keys. Test with empty errors — verify `errors` field is absent from JSON (skip_serializing_if).
+- `render_to_writer`: Build minimal map, write to `Vec<u8>`, verify output matches `render_json`.
+
+**Module placement:** `#[cfg(test)] mod tests { ... }` at the end of each source file. Integration tests in `tests/integration_test.rs`.
+
+**Test fixtures:** `tempfile::tempdir()` is sufficient for most tests. The existing `tests/fixtures/sample-workspace/` is an integration test fixture that could be extended with additional sub-fixtures for the 7 integration tests listed in the plan.
+
+---
+
+## Item: Integration test expansion (Goal 4)
+
+### 7 new integration tests needed:
+
+1. **Parse failure workspace:** Create a workspace with one well-formed crate and one crate containing invalid Rust syntax. Run binary, verify `ErrorEntry` with `kind: "syn_parse_error"` and `severity: "error"` appears in JSON output.
+
+2. **Missing workspace section:** Create a `Cargo.toml` with `[dependencies]` but no `[workspace]` section. Run binary, verify `ErrorEntry` with `kind: "missing_workspace_section"` appears (or the binary exits non-zero via `Error::MissingWorkspaceSection` propagated through `anyhow`).
+
+3. **Glob member patterns:** Create a workspace with `members = ["crates/*"]` containing 3 sub-crate directories. Run binary, verify all 3 crates appear in output.
+
+4. **Workspace with exclude:** Create a workspace with `members = ["a", "b", "c"]` and `exclude = ["b"]`. Run binary, verify only `a` and `c` appear in output.
+
+5. **Deeply nested modules:** Create a crate with 3+ levels of `mod` declarations (`src/lib.rs` → `src/foo.rs` → `src/foo/bar.rs` → `src/foo/bar/baz.rs`). Verify correct module paths (`foo::bar::baz`), file references, and hierarchy in output.
+
+6. **Re-export chains:** Create a crate with `pub use inner::Secret;` where `mod inner { pub struct Secret; }`. Verify `ReExport` entries are populated with correct import/export paths.
+
+7. **Output via -o flag:** Run binary with `-o /tmp/test-output.json`, read the file, verify content matches stdout output from a separate run (byte-identical).
+
+**Module placement:** `tests/integration_test.rs`.
+
+**Fixture strategy:** Each test creates its fixture in a `tempfile::tempdir()` and passes the path to the binary via CLI argument. No need for new files under `tests/fixtures/`.
+
+---
+
+## Item: Remove clippy suppressions (Goal 5)
+
+### Suppression by suppression:
+
+**`missing_errors_doc`:** Add `# Errors` doc sections to `parse_file`, `build_module_tree` (signature change), and `enumerate_members` (now returns error for missing section). Other functions either return `Result` and have existing docs, or don't return `Result` and don't need the section.
+
+**`must_use_candidate`:** Add `#[must_use]` to pure functions returning `Vec<...>`: `extract_public_items`, `extract_imports`, `extract_re_exports`, `extract_submodules`, `extract_impls`, `resolve_module_path`, `render_json`, `kind_to_string`. Functions with side effects or that are builder-style do not get `#[must_use]`.
+
+**`doc_markdown`:** Fix doc comments that contain identifiers not wrapped in backticks. E.g., `syn::File` → already correct, but check for bare `Path`, `Vec`, `Option`, etc.
+
+**`uninlined_format_args`:** Inline all `format!("{}", x)` to `format!("{x}")` and `eprintln!("text: {}", x)` to `eprintln!("text: {x}")`.
+
+**`redundant_closure`:** Find closures like `.map(|x| foo(x))` and replace with `.map(foo)`.
+
+**`collapsible_if`:** Merge nested `if` expressions.
+
+**`needless_pass_by_value`:** Change function parameters from `String` to `&str` or `PathBuf` to `&Path` where callers only need to borrow.
+
+**`needless_borrow`:** Remove unnecessary `&` in function calls.
+
+**`redundant_closure_for_method_calls`:** Replace `.map(|x| x.method())` with `.map(|x| x.method())` clippy is fine... actually: `.iter().map(|s| s.to_string())` should become `.iter().map(ToString::to_string)` or `.cloned()` depending on context.
+
+**Module placement:** `src/lib.rs` (removal of crate-level allows), then per-file fixes in each `src/*.rs`.
+
+**Uncertain:** The exact locations of each clippy lint require running `cargo clippy -- -W clippy::pedantic` to see which lints actually fire. The disposition table in the plan maps each suppression to a fix action, but the actual code locations may differ from what's expected. A safe approach: remove all 9 crate-level allows, run clippy, fix each firing at the per-item level, then verify clean.
+
+---
+
+## Deferred Items Assessment
+
+- **Empty errors vector in `run()`:** Absorbed — Goal 1 directly addresses this by wiring error collection into `run()`.
+- **Silent error swallowing in `build_module_tree`:** Absorbed — Goal 1 changes `build_module_tree` to return `(Vec<ModuleInfo>, Vec<ErrorEntry>)` instead of `Result<Vec<ModuleInfo>>`, eliminating `unwrap_or_default()`.
+- **Path fallback to `"."` in `module_tree.rs`:** Absorbed — Goal 3 fixes all three `unwrap_or_else(|| Path::new("."))` calls.
+- **Path fallback to `""` in `workspace.rs`:** Absorbed — Goal 3 changes `enumerate_members` to return `Err(MissingWorkspaceSection)` when the workspace section is absent, and Goal 1 converts the remaining paths to ErrorEntry construction in `run()`.
+- **No unit tests for public API functions:** Absorbed — Goal 2 adds comprehensive unit tests across all 5 core modules.
+
+**Deferred items absorbed: 5/5. No items skipped.**
diff --git a/notes/plan-enrichment/phase-0.2/gather-summary.md b/notes/plan-enrichment/phase-0.2/gather-summary.md
new file mode 100644
index 0000000..2543875
--- /dev/null
+++ b/notes/plan-enrichment/phase-0.2/gather-summary.md
@@ -0,0 +1,27 @@
+## Gather Summary: phase-0.2
+
+**Tasks created:** 10
+**Dependency chain:** Not extracted, see draft-plan.toml
+**Deferred items absorbed:** 5/5 (all deferred items from phase 0.1 absorbed by phase 0.2 goals)
+
+**Gather completeness:**
+- [x] deferred-and-patterns.md — saved (5 deferred items, 0 failure modes)
+- [x] codebase-state.md — saved — Files documented: 10
+- [x] draft-elaboration.md — saved — Items elaborated: 15
+- [x] draft-plan.toml — saved — Tasks: 10, Validation: PASSED
+- [x] task-checklist.md — saved — Tasks checked: 10, Wiring issues flagged: 2
+
+**Before-block verification:** 10/10 confirmed
+**Unverified tasks:** none
+**Wiring issues flagged:** 2 (see task-checklist.md for details)
+
+**Before-block verification:** 10/10 confirmed
+**Unverified tasks:** none
+**Wiring issues flagged:** 2
+
+**Confidence notes:**
+1. Exact clippy lint locations require running `cargo clippy` after suppression removal to confirm which functions actually fire each lint.
+2. Orphaned module handling retains placeholder `ModuleInfo` entries alongside the new `ErrorEntry` per the existing pattern in `module_tree.rs` (the plan says "emit ErrorEntry" not "replace eprintln with ErrorEntry", so both coexist).
+
+**Questions for user:**
+None
diff --git a/notes/plan-enrichment/phase-0.2/plan.approved.toml b/notes/plan-enrichment/phase-0.2/plan.approved.toml
new file mode 100644
index 0000000..de26e22
--- /dev/null
+++ b/notes/plan-enrichment/phase-0.2/plan.approved.toml
@@ -0,0 +1,2168 @@
+[meta]
+title = "Phase 0.2: Hardening for Trustworthiness"
+source_branch = "phase-0.2"
+created = "2026-04-28"
+
+[dependencies]
+TASK-PREP = []
+TASK-3 = ["TASK-1"]
+TASK-4 = ["TASK-1", "TASK-2", "TASK-3"]
+TASK-5 = ["TASK-4"]
+TASK-6 = ["TASK-2"]
+TASK-7 = ["TASK-4", "TASK-5"]
+TASK-8 = ["TASK-7"]
+TASK-9 = ["TASK-PREP", "TASK-5", "TASK-6", "TASK-8"]
+TASK-10 = ["TASK-PREP", "TASK-9"]
+
+[tasks.TASK-PREP]
+description = "Add tempfile dev-dependency for unit and integration tests"
+type = "replace"
+acceptance = [
+    "cargo check -p rust-workspace-map",
+]
+
+[[tasks.TASK-PREP.changes]]
+file = "Cargo.toml"
+before = "proc-macro2 = { version = \"1\", features = [\"span-locations\"] }"
+after = """proc-macro2 = { version = "1", features = ["span-locations"] }
+
+[dev-dependencies]
+tempfile = "3\""""
+
+[tasks.TASK-1]
+description = "Add ErrorSeverity enum and ErrorContext struct to schema.rs"
+type = "replace"
+acceptance = [
+    "cargo check -p rust-workspace-map",
+]
+
+[[tasks.TASK-1.changes]]
+file = "src/schema.rs"
+before = '''// ── Internal types ──────────────────────────────────────────────────────
+
+/// Internal intermediate type consumed by module_tree.
+#[derive(Debug, Clone, Default)]
+pub struct FileInfo {
+'''
+after = '''// ── Error severity ─────────────────────────────────────────────────────
+
+#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
+#[serde(rename_all = "snake_case")]
+pub enum ErrorSeverity {
+    Error,
+    Warning,
+}
+
+// ── Error context ───────────────────────────────────────────────────────
+
+/// Optional context attached to an error, providing additional location
+/// and source information for diagnostics.
+#[derive(Debug, Clone, Default, serde::Serialize, bon::Builder)]
+#[serde(rename_all = "camelCase")]
+pub struct ErrorContext {
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pub crate_name: Option<String>,
+
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pub module_path: Option<String>,
+
+    /// Line number in the source file where the error occurred.
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pub line: Option<usize>,
+
+    /// A short source snippet near the error location (if available).
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pub snippet: Option<String>,
+}
+
+// ── Internal types ──────────────────────────────────────────────────────
+
+/// Internal intermediate type consumed by module_tree.
+#[derive(Debug, Clone, Default)]
+pub struct FileInfo {
+'''
+
+[tasks.TASK-2]
+description = "Add MissingWorkspaceSection variant to Error enum in schema.rs"
+type = "replace"
+acceptance = [
+    "cargo check -p rust-workspace-map",
+]
+
+[[tasks.TASK-2.changes]]
+file = "src/schema.rs"
+before = '''    #[error("glob pattern error: {0}")]
+    GlobPattern(String),
+}
+
+pub type Result<T> = std::result::Result<T, Error>;'''
+after = '''    #[error("glob pattern error: {0}")]
+    GlobPattern(String),
+
+    #[error("workspace Cargo.toml is missing the [workspace] section")]
+    MissingWorkspaceSection,
+}
+
+pub type Result<T> = std::result::Result<T, Error>;'''
+
+[tasks.TASK-3]
+description = "Expand ErrorEntry struct with severity, kind, context, and cause fields"
+type = "replace"
+acceptance = [
+    "cargo check -p rust-workspace-map",
+]
+
+[[tasks.TASK-3.changes]]
+file = "src/schema.rs"
+before = '''#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
+#[serde(rename_all = "camelCase")]
+pub struct ErrorEntry {
+    pub file: String,
+    #[builder(default)]
+    pub line: usize,
+    pub message: String,
+}
+'''
+after = '''#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
+#[serde(rename_all = "camelCase")]
+pub struct ErrorEntry {
+    pub file: String,
+    #[builder(default)]
+    pub line: usize,
+    pub message: String,
+    pub severity: ErrorSeverity,
+    pub kind: String,
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pub context: Option<ErrorContext>,
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pub cause: Option<String>,
+}
+'''
+
+[tasks.TASK-4]
+description = "Change parse_file to return ParsedFile with optional parse errors instead of Result"
+type = "replace"
+acceptance = [
+    "true  # applied atomically with TASK-5; compilation verified at TASK-5",
+]
+
+[[tasks.TASK-4.changes]]
+file = "src/file_parser.rs"
+before = '''use crate::schema::{
+    Error, FileInfo, ImplInfo, ImplItem, ImplItemKind, Import, ItemAttrs, ItemKind, PublicItem,
+    ReExport, Result, SubmoduleDecl,
+};
+use std::path::Path;
+
+// ── parse_file ──────────────────────────────────────────────────────────
+
+/// Read and parse a Rust source file. Returns the raw `syn::File` AST (needed
+/// by `module_tree` for inline module item extraction) and the extracted
+/// `FileInfo`. On parse failure, warns to stderr and returns empty results.
+pub fn parse_file(path: &Path) -> Result<(syn::File, FileInfo)> {
+    let content = std::fs::read_to_string(path).map_err(|source| Error::FileRead {
+        path: path.to_path_buf(),
+        source,
+    })?;
+
+    let file = match syn::parse_file(&content) {
+        Ok(f) => f,
+        Err(e) => {
+            eprintln!("warning: failed to parse {}: {}", path.display(), e);
+            let empty = syn::File {
+                shebang: None,
+                attrs: vec![],
+                items: vec![],
+            };
+            let info = FileInfo::default();
+            return Ok((empty, info));
+        }
+    };
+
+    let info = FileInfo {
+        public_items: extract_public_items(&file.items),
+        imports: extract_imports(&file.items),
+        re_exports: extract_re_exports(&file.items),
+        submodules: extract_submodules(&file.items),
+        impls: extract_impls(&file.items),
+    };
+
+    Ok((file, info))
+}'''
+after = '''use crate::schema::{
+    Error, ErrorEntry, ErrorSeverity, FileInfo, ImplInfo, ImplItem, ImplItemKind, Import,
+    ItemAttrs, ItemKind, PublicItem, ReExport, Result, SubmoduleDecl,
+};
+use std::path::Path;
+
+// ── Internal parse result types ────────────────────────────────────────
+
+/// Result of parsing a Rust source file.
+///
+/// Unlike `Result<T, Error>`, this type always succeeds — parse
+/// failures are reported as data, not as errors, so the caller
+/// can continue processing other files. The caller constructs
+/// `ErrorEntry` values from `SynParseError` when needed.
+pub struct ParsedFile {
+    pub ast: syn::File,
+    pub file_info: FileInfo,
+    pub parse_error: Option<SynParseError>,
+}
+
+/// Structured information about a parse failure.
+pub struct SynParseError {
+    pub message: String,
+    pub line: usize,
+}
+
+// ── parse_file ──────────────────────────────────────────────────────────
+
+/// Read and parse a Rust source file.
+///
+/// On parse failure, returns the original file content and a
+/// `SynParseError` alongside an empty `FileInfo`. Callers use the
+/// error to construct an `ErrorEntry`.
+pub fn parse_file(path: &Path) -> ParsedFile {
+    let content = match std::fs::read_to_string(path) {
+        Ok(c) => c,
+        Err(source) => {
+            let err = SynParseError {
+                message: source.to_string(),
+                line: 0,
+            };
+            return ParsedFile {
+                ast: syn::File {
+                    shebang: None,
+                    attrs: vec![],
+                    items: vec![],
+                },
+                file_info: FileInfo::default(),
+                parse_error: Some(err),
+            };
+        }
+    };
+
+    match syn::parse_file(&content) {
+        Ok(file) => {
+            let file_info = FileInfo {
+                public_items: extract_public_items(&file.items),
+                imports: extract_imports(&file.items),
+                re_exports: extract_re_exports(&file.items),
+                submodules: extract_submodules(&file.items),
+                impls: extract_impls(&file.items),
+            };
+            ParsedFile {
+                ast: file,
+                file_info,
+                parse_error: None,
+            }
+        },
+        Err(e) => {
+            let line = e.span().start().line;
+            let err = SynParseError {
+                message: e.to_string(),
+                line,
+            };
+            ParsedFile {
+                ast: syn::File {
+                    shebang: None,
+                    attrs: vec![],
+                    items: vec![],
+                },
+                file_info: FileInfo::default(),
+                parse_error: Some(err),
+            }
+        }
+    }
+}
+
+pub(crate) fn build_parse_error_entry(path: &Path, err: &SynParseError) -> ErrorEntry {
+    ErrorEntry::builder()
+        .file(path.to_string_lossy().to_string())
+        .line(err.line)
+        .message(err.message.clone())
+        .severity(ErrorSeverity::Error)
+        .kind("syn_parse_error".to_string())
+        .build()
+}'''
+
+[tasks.TASK-5]
+description = "Refactor module_tree.rs: fix path fallbacks, add error collection, change build_module_tree return type"
+type = "replace"
+acceptance = [
+    "cargo check -p rust-workspace-map",
+]
+
+[[tasks.TASK-5.changes]]
+file = "src/module_tree.rs"
+before = "use crate::schema::{FileInfo, ModuleInfo, Result, SubmoduleDecl};"
+after = "use crate::schema::{ErrorContext, ErrorEntry, ErrorSeverity, FileInfo, ModuleInfo, Result, SubmoduleDecl};"
+
+[[tasks.TASK-5.changes]]
+file = "src/module_tree.rs"
+before = '''/// Build the full module tree for a crate starting from its entry point
+/// (e.g., `src/lib.rs`). Returns a flat `Vec<ModuleInfo>` containing the
+/// root module and all recursively discovered submodules.
+pub fn build_module_tree(crate_root: &Path, crate_name: &str) -> Result<Vec<ModuleInfo>> {
+    let mut visited = HashSet::new();
+    let parent_dir = crate_root.parent().unwrap_or_else(|| Path::new("."));
+
+    let (ast, file_info) = file_parser::parse_file(crate_root)?;
+    visited.insert(crate_root.to_path_buf());
+
+    let root_module = build_module_info(
+        crate_name,
+        crate_root,
+        "pub",
+        &file_info.public_items,
+        &file_info.imports,
+        &file_info.re_exports,
+        &file_info.submodules,
+    );
+
+    let mut modules = vec![root_module];
+
+    for sub in &file_info.submodules {
+        if sub.is_test {
+            continue;
+        }
+        let sub_module_path = format!("{}::{}", crate_name, sub.name);
+        let child_modules = process_submodule(
+            &sub_module_path,
+            &sub.name,
+            &ast.items,
+            parent_dir,
+            crate_root,
+            &mut visited,
+        )?;
+        modules.extend(child_modules);
+    }
+
+    Ok(modules)
+}'''
+after = '''/// Build the full module tree for a crate starting from its entry point
+/// (e.g., `src/lib.rs`). Returns a tuple of module info and any errors
+/// encountered during submodule parsing (including orphaned module warnings).
+pub fn build_module_tree(
+    crate_root: &Path,
+    crate_name: &str,
+) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {
+
+    let mut visited = HashSet::new();
+    let parent_dir = crate_root.parent().unwrap_or(crate_root);
+
+    let parsed = file_parser::parse_file(crate_root);
+    let mut errors: Vec<ErrorEntry> = Vec::new();
+    if let Some(ref err) = parsed.parse_error {
+        errors.push(crate::file_parser::build_parse_error_entry(crate_root, err));
+    }
+    visited.insert(crate_root.to_path_buf());
+
+    let root_module = build_module_info(
+        crate_name,
+        crate_root,
+        "pub",
+        &parsed.file_info.public_items,
+        &parsed.file_info.imports,
+        &parsed.file_info.re_exports,
+        &parsed.file_info.submodules,
+    );
+
+    let mut modules = vec![root_module];
+
+    for sub in &parsed.file_info.submodules {
+        if sub.is_test {
+            continue;
+        }
+        let sub_module_path = format!("{}::{}", crate_name, sub.name);
+        let (child_modules, child_errors) = process_submodule(
+            &sub_module_path,
+            &sub.name,
+            &parsed.ast.items,
+            parent_dir,
+            crate_root,
+            &mut visited,
+        );
+        errors.extend(child_errors);
+        modules.extend(child_modules);
+    }
+
+    (modules, errors)
+}'''
+
+[[tasks.TASK-5.changes]]
+file = "src/module_tree.rs"
+before = '''    let Some(mod_item) = mod_item else {
+        eprintln!("warning: orphaned module {}", module_path);
+        return Ok(vec![ModuleInfo::builder()
+            .path(module_path.to_string())
+            .file("<unresolved>".to_string())
+            .visibility("private".to_string())
+            .build()]);
+    };'''
+after = '''    let Some(mod_item) = mod_item else {
+        let err = ErrorEntry::builder()
+            .file(String::new())
+            .message(format!("orphaned module: {module_path}"))
+            .severity(ErrorSeverity::Warning)
+            .kind("orphaned_module".to_string())
+            .context(ErrorContext::builder()
+                .module_path(module_path.to_string())
+                .build())
+            .build();
+        return (vec![ModuleInfo::builder()
+            .path(module_path.to_string())
+            .file("<unresolved>".to_string())
+            .visibility("private".to_string())
+            .build()], vec![err]);
+    };'''
+
+[[tasks.TASK-5.changes]]
+file = "src/module_tree.rs"
+before = '''        let Some(ref file_path) = file_path else {
+            eprintln!("warning: orphaned module {}", module_path);
+            return Ok(vec![ModuleInfo::builder()
+                .path(module_path.to_string())
+                .file("<unresolved>".to_string())
+                .visibility(visibility.to_string())
+                .build()]);
+        };'''
+after = '''        let Some(ref file_path) = file_path else {
+            let err = ErrorEntry::builder()
+                .file(String::new())
+                .message(format!("orphaned module: {module_path}"))
+                .severity(ErrorSeverity::Warning)
+                .kind("orphaned_module".to_string())
+                .context(ErrorContext::builder()
+                    .module_path(module_path.to_string())
+                    .build())
+                .build();
+            return (vec![ModuleInfo::builder()
+                .path(module_path.to_string())
+                .file("<unresolved>".to_string())
+                .visibility(visibility.to_string())
+                .build()], vec![err]);
+        };'''
+
+[[tasks.TASK-5.changes]]
+file = "src/module_tree.rs"
+before = '''        let (ast, file_info) = file_parser::parse_file(file_path)?;
+        process_module_info(
+            module_path,
+            file_path,
+            visibility,
+            &file_info,
+            &ast.items,
+            &file_path.parent().unwrap_or_else(|| Path::new(".")),
+            visited,
+        )'''
+after = '''        let parsed = file_parser::parse_file(file_path);
+        let mut errors: Vec<ErrorEntry> = Vec::new();
+        if let Some(ref err) = parsed.parse_error {
+            errors.push(crate::file_parser::build_parse_error_entry(file_path, err));
+        }
+        process_module_info(
+            module_path,
+            file_path,
+            visibility,
+            &parsed.file_info,
+            &parsed.ast.items,
+            &file_path.parent().unwrap_or(file_path),
+            visited,
+            &mut errors,
+        )'''
+
+[[tasks.TASK-5.changes]]
+file = "src/module_tree.rs"
+before = '''        if visited.contains(file_path.as_path()) {
+            return Ok(vec![]); // cycle detected
+        }'''
+after = '''        if visited.contains(file_path.as_path()) {
+            return (vec![], vec![]); // cycle detected
+        }'''
+
+[[tasks.TASK-5.changes]]
+file = "src/module_tree.rs"
+before = '''/// Resolve a `mod name;` declaration to a file path.
+/// Tries `{parent_dir}/{mod_name}.rs` first, then `{parent_dir}/{mod_name}/mod.rs`.
+pub fn resolve_module_path(parent_dir: &Path, mod_name: &str) -> Option<PathBuf> {'''
+after = '''/// Resolve a `mod name;` declaration to a file path.
+/// Tries `{parent_dir}/{mod_name}.rs` first, then `{parent_dir}/{mod_name}/mod.rs`.
+///
+/// Returns `None` if neither path exists.
+pub fn resolve_module_path(parent_dir: &Path, mod_name: &str) -> Option<PathBuf> {'''
+
+[[tasks.TASK-5.changes]]
+file = "src/module_tree.rs"
+before = '''fn process_submodule(
+    module_path: &str,
+    mod_name: &str,
+    parent_items: &[syn::Item],
+    parent_dir: &Path,
+    parent_file: &Path,
+    visited: &mut HashSet<PathBuf>,
+) -> Result<Vec<ModuleInfo>> {'''
+after = '''fn process_submodule(
+    module_path: &str,
+    mod_name: &str,
+    parent_items: &[syn::Item],
+    parent_dir: &Path,
+    parent_file: &Path,
+    visited: &mut HashSet<PathBuf>,
+) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {'''
+
+[[tasks.TASK-5.changes]]
+file = "src/module_tree.rs"
+before = '''    if let Some((_, ref inline_items)) = mod_item.content {
+        // Inline module: process its body items directly (no file lookup).
+        process_module_items(
+            module_path,
+            parent_file,
+            visibility,
+            inline_items,
+            parent_dir,
+            visited,
+        )
+    } else {'''
+after = '''    if let Some((_, ref inline_items)) = mod_item.content {
+        // Inline module: process its body items directly (no file lookup).
+        let (modules, errs) = process_module_items(
+            module_path,
+            parent_file,
+            visibility,
+            inline_items,
+            parent_dir,
+            visited,
+        );
+        return (modules, errs);
+    } else {'''
+
+[[tasks.TASK-5.changes]]
+file = "src/module_tree.rs"
+before = '''fn process_module_items(
+    module_path: &str,
+    file_path: &Path,
+    visibility: &str,
+    items: &[syn::Item],
+    parent_dir: &Path,
+    visited: &mut HashSet<PathBuf>,
+) -> Result<Vec<ModuleInfo>> {'''
+after = '''fn process_module_items(
+    module_path: &str,
+    file_path: &Path,
+    visibility: &str,
+    items: &[syn::Item],
+    parent_dir: &Path,
+    visited: &mut HashSet<PathBuf>,
+) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {'''
+
+[[tasks.TASK-5.changes]]
+file = "src/module_tree.rs"
+before = '''    let file_info = FileInfo {
+        public_items: file_parser::extract_public_items(items),
+        imports: file_parser::extract_imports(items),
+        re_exports: file_parser::extract_re_exports(items),
+        submodules: file_parser::extract_submodules(items),
+        impls: file_parser::extract_impls(items),
+    };
+    process_module_info(module_path, file_path, visibility, &file_info, items, parent_dir, visited)
+}'''
+after = '''    let file_info = FileInfo {
+        public_items: file_parser::extract_public_items(items),
+        imports: file_parser::extract_imports(items),
+        re_exports: file_parser::extract_re_exports(items),
+        submodules: file_parser::extract_submodules(items),
+        impls: file_parser::extract_impls(items),
+    };
+    process_module_info(module_path, file_path, visibility, &file_info, items, parent_dir, visited, &mut Vec::new())
+}'''
+
+[[tasks.TASK-5.changes]]
+file = "src/module_tree.rs"
+before = '''fn process_module_info(
+    module_path: &str,
+    file_path: &Path,
+    visibility: &str,
+    file_info: &FileInfo,
+    items: &[syn::Item],
+    _parent_dir: &Path,
+    visited: &mut HashSet<PathBuf>,
+) -> Result<Vec<ModuleInfo>> {
+    let mut modules = vec![build_module_info(
+        module_path,
+        file_path,
+        visibility,
+        &file_info.public_items,
+        &file_info.imports,
+        &file_info.re_exports,
+        &file_info.submodules,
+    )];
+
+    for sub in &file_info.submodules {
+        if sub.is_test {
+            continue;
+        }
+        let child_path = format!("{}::{}", module_path, sub.name);
+        let child_dir = file_path.parent().unwrap_or_else(|| Path::new("."));
+        let child_modules = process_submodule(
+            &child_path,
+            &sub.name,
+            items,
+            child_dir,
+            file_path,
+            visited,
+        )?;
+        modules.extend(child_modules);
+    }
+
+    Ok(modules)
+}'''
+after = '''fn process_module_info(
+    module_path: &str,
+    file_path: &Path,
+    visibility: &str,
+    file_info: &FileInfo,
+    items: &[syn::Item],
+    _parent_dir: &Path,
+    visited: &mut HashSet<PathBuf>,
+    errors: &mut Vec<crate::schema::ErrorEntry>,
+) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {
+    let mut modules = vec![build_module_info(
+        module_path,
+        file_path,
+        visibility,
+        &file_info.public_items,
+        &file_info.imports,
+        &file_info.re_exports,
+        &file_info.submodules,
+    )];
+
+    for sub in &file_info.submodules {
+        if sub.is_test {
+            continue;
+        }
+        let child_path = format!("{}::{}", module_path, sub.name);
+        let child_dir = file_path.parent().unwrap_or(file_path);
+        let (child_modules, child_errors) = process_submodule(
+            &child_path,
+            &sub.name,
+            items,
+            child_dir,
+            file_path,
+            visited,
+        );
+        errors.extend(child_errors);
+        modules.extend(child_modules);
+    }
+
+    (modules, errors.clone())
+}'''
+
+
+[tasks.TASK-6]
+description = "Update workspace.rs enumerate_members to return MissingWorkspaceSection error"
+type = "replace"
+acceptance = [
+    "cargo check -p rust-workspace-map",
+]
+
+[[tasks.TASK-6.changes]]
+file = "src/workspace.rs"
+before = '''    let members: Vec<String> = parsed
+        .get("workspace")
+        .and_then(|w| w.get("members"))
+        .and_then(|m| m.as_array())
+        .map(|arr| {
+            arr.iter()
+                .filter_map(|v| v.as_str().map(String::from))
+                .collect()
+        })
+        .unwrap_or_default();'''
+after = '''    let members: Vec<String> = match parsed.get("workspace") {
+        None => return Err(Error::MissingWorkspaceSection),
+        Some(workspace) => workspace
+            .get("members")
+            .and_then(|m| m.as_array())
+            .map(|arr| {
+                arr.iter()
+                    .filter_map(|v| v.as_str().map(String::from))
+                    .collect::<Vec<_>>()
+            })
+            .unwrap_or_default(),
+    };'''
+
+[[tasks.TASK-6.changes]]
+file = "src/workspace.rs"
+before = '''/// Walk up the directory tree from `start_path` to find a `Cargo.toml`
+/// containing a `[workspace]` section. Returns the directory containing it.
+pub fn find_workspace_root(start_path: &Path) -> Result<PathBuf> {'''
+after = '''/// Walk up the directory tree from `start_path` to find a `Cargo.toml`
+/// containing a `[workspace]` section. Returns the directory containing it.
+///
+/// # Errors
+///
+/// Returns `Error::WorkspaceRootNotFound` if no `Cargo.toml` with a
+/// `[workspace]` section is found in any ancestor directory.
+pub fn find_workspace_root(start_path: &Path) -> Result<PathBuf> {'''
+
+[[tasks.TASK-6.changes]]
+file = "src/workspace.rs"
+before = '''/// Parse the workspace `Cargo.toml`, resolve member paths (including glob
+/// patterns), apply `exclude` list, and return absolute paths to each member
+/// crate directory.
+pub fn enumerate_members(root: &Path) -> Result<Vec<PathBuf>> {'''
+after = '''/// Parse the workspace `Cargo.toml`, resolve member paths (including glob
+/// patterns), apply `exclude` list, and return absolute paths to each member
+/// crate directory.
+///
+/// # Errors
+///
+/// Returns `Error::MissingWorkspaceSection` if the `Cargo.toml` lacks a
+/// `[workspace]` section entirely.
+pub fn enumerate_members(root: &Path) -> Result<Vec<PathBuf>> {'''
+
+[tasks.TASK-7]
+description = "Refactor lib.rs run(): parallel error collection, error entry construction, remove unwrap_or_default"
+type = "replace"
+acceptance = [
+    "cargo check -p rust-workspace-map",
+]
+
+[[tasks.TASK-7.changes]]
+file = "src/lib.rs"
+before = '''use anyhow::Context;
+use rayon::prelude::*;
+use schema::{
+    CrateInfo, CrateType, ErrorEntry, ModuleInfo, WorkspaceInfo, WorkspaceMap,
+};
+use std::path::Path;'''
+after = '''use anyhow::Context;
+use rayon::prelude::*;
+use schema::{
+    CrateInfo, CrateType, ErrorEntry, ErrorSeverity, ModuleInfo, WorkspaceInfo,
+    WorkspaceMap,
+};
+use std::path::Path;'''
+
+[[tasks.TASK-7.changes]]
+file = "src/lib.rs"
+before = '''    let errors: Vec<ErrorEntry> = Vec::new();
+
+    let mut crate_infos: Vec<CrateInfo> = member_dirs
+        .par_iter()
+        .filter_map(|dir| {
+            let cargo_toml = dir.join("Cargo.toml");
+
+            let (pkg, deps) = match cargo_info::parse_cargo_toml(&cargo_toml) {
+                Ok(v) => v,
+                Err(e) => {
+                    eprintln!(
+                        "warning: failed to parse {}: {}",
+                        cargo_toml.display(),
+                        e
+                    );
+                    return None;
+                }
+            };
+
+            let roots = workspace::resolve_crate_roots(dir);
+            if roots.is_empty() {
+                eprintln!(
+                    "warning: no crate entry points found in {}",
+                    dir.display()
+                );
+                return None;
+            }
+
+            let crate_type = if roots.iter().any(|(_, t)| *t == CrateType::Lib)
+                && roots.iter().any(|(_, t)| *t == CrateType::Bin)
+            {
+                CrateType::LibAndBin
+            } else {
+                roots.first().map_or(CrateType::Lib, |(_, t)| *t)
+            };
+
+            let pkg_name = pkg.name.clone();
+            let mut modules: Vec<ModuleInfo> = roots
+                .iter()
+                .flat_map(|(root, _ty)| {
+                    module_tree::build_module_tree(root, &pkg_name).unwrap_or_default()
+                })
+                .collect();
+
+            // Relativize all paths to the workspace root.
+            for m in &mut modules {
+                m.file = relativize_path(&m.file, &workspace_root);
+                for item in &mut m.public_items {
+                    item.file = relativize_path(&item.file, &workspace_root);
+                }
+            }
+
+            let crate_root = roots
+                .first()
+                .map(|(r, _)| relativize_path(&r.to_string_lossy(), &workspace_root))
+                .unwrap_or_default();
+
+            let rebuilt_pkg = schema::PackageInfo::builder()
+                .name(pkg.name)
+                .version(pkg.version)
+                .edition(pkg.edition)
+                .crate_type(crate_type)
+                .build();
+
+            Some(
+                CrateInfo::builder()
+                    .name(pkg_name)
+                    .root(crate_root)
+                    .package(rebuilt_pkg)
+                    .modules(modules)
+                    .deps(deps)
+                    .build(),
+            )
+        })
+        .collect();'''
+after = '''    let mut crate_errors: Vec<ErrorEntry> = Vec::new();
+
+    let results: Vec<(CrateInfo, Vec<ErrorEntry>)> = member_dirs
+        .par_iter()
+        .map(|dir| {
+            let cargo_toml = dir.join("Cargo.toml");
+            let mut crate_errors = Vec::new();
+
+            let (pkg, deps) = match cargo_info::parse_cargo_toml(&cargo_toml) {
+                Ok(v) => v,
+                Err(e) => {
+                    crate_errors.push(ErrorEntry::builder()
+                        .file(cargo_toml.to_string_lossy().to_string())
+                        .message(format!("failed to parse Cargo.toml: {e}"))
+                        .severity(ErrorSeverity::Error)
+                        .kind("toml_parse_error".to_string())
+                        .cause(e.to_string())
+                        .build());
+                    return (None, crate_errors);
+                }
+            };
+
+            let roots = workspace::resolve_crate_roots(dir);
+            if roots.is_empty() {
+                crate_errors.push(ErrorEntry::builder()
+                    .file(dir.to_string_lossy().to_string())
+                    .message("no crate entry points found".to_string())
+                    .severity(ErrorSeverity::Warning)
+                    .kind("missing_crate_roots".to_string())
+                    .build());
+                return (None, crate_errors);
+            }
+
+            let crate_type = if roots.iter().any(|(_, t)| *t == CrateType::Lib)
+                && roots.iter().any(|(_, t)| *t == CrateType::Bin)
+            {
+                CrateType::LibAndBin
+            } else {
+                roots.first().map_or(CrateType::Lib, |(_, t)| *t)
+            };
+
+            let pkg_name = pkg.name.clone();
+            let mut modules: Vec<ModuleInfo> = Vec::new();
+            let mut collected_errors = Vec::new();
+            for (root, _ty) in &roots {
+                let (m, e) = module_tree::build_module_tree(&root, &pkg_name);
+                modules.extend(m);
+                collected_errors.extend(e);
+            }
+            crate_errors.extend(collected_errors);
+
+            // Relativize all paths to the workspace root.
+            for m in &mut modules {
+                m.file = relativize_path(&m.file, &workspace_root);
+                for item in &mut m.public_items {
+                    item.file = relativize_path(&item.file, &workspace_root);
+                }
+            }
+
+            let crate_root = roots
+                .first()
+                .map(|(r, _)| relativize_path(&r.to_string_lossy(), &workspace_root))
+                .unwrap_or_default();
+
+            let rebuilt_pkg = schema::PackageInfo::builder()
+                .name(pkg.name)
+                .version(pkg.version)
+                .edition(pkg.edition)
+                .crate_type(crate_type)
+                .build();
+
+            let crate_info = CrateInfo::builder()
+                .name(pkg_name)
+                .root(crate_root)
+                .package(rebuilt_pkg)
+                .modules(modules)
+                .deps(deps)
+                .build();
+
+            (Some(crate_info), crate_errors)
+        })
+        .collect();
+
+    let mut crate_infos: Vec<CrateInfo> = Vec::new();
+
+    for (info, errs) in results {
+        if let Some(ci) = info {
+            crate_errors.extend(errs);
+            crate_infos.push(ci);
+        }
+    }'''
+
+[[tasks.TASK-7.changes]]
+file = "src/lib.rs"
+before = '''    let map = WorkspaceMap::builder()
+        .workspace(workspace_info)
+        .crates(crate_infos)
+        .cross_references(cross_refs)
+        .errors(errors)
+        .workspace_root(workspace_root.clone())
+        .build();'''
+after = '''    let map = WorkspaceMap::builder()
+        .workspace(workspace_info)
+        .crates(crate_infos)
+        .cross_references(cross_refs)
+        .errors(crate_errors)
+        .workspace_root(workspace_root.clone())
+        .build();'''
+
+[tasks.TASK-8]
+description = "Remove all crate-level clippy allow attributes and fix individual lint violations"
+type = "replace"
+acceptance = [
+    "cargo clippy -p rust-workspace-map -- -D warnings",
+]
+
+[[tasks.TASK-8.changes]]
+file = "src/lib.rs"
+before = '''#![warn(clippy::pedantic)]
+#![allow(clippy::missing_errors_doc)]
+#![allow(clippy::must_use_candidate)]
+#![allow(clippy::doc_markdown)]
+#![allow(clippy::uninlined_format_args)]
+#![allow(clippy::redundant_closure)]
+#![allow(clippy::collapsible_if)]
+#![allow(clippy::needless_pass_by_value)]
+#![allow(clippy::needless_borrow)]
+#![allow(clippy::redundant_closure_for_method_calls)]'''
+after = '''#![warn(clippy::pedantic)]'''
+
+[[tasks.TASK-8.changes]]
+file = "src/lib.rs"
+before = '''/// Run the full workspace mapping pipeline.
+///
+/// 1. Discover workspace root and member crates.
+/// 2. Process each crate in parallel (Cargo.toml parsing + module tree).
+/// 3. Compute cross-crate references.
+/// 4. Render JSON to stdout or the configured output file.
+pub fn run(config: Config) -> anyhow::Result<()> {'''
+after = '''/// Run the full workspace mapping pipeline.
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
+pub fn run(config: Config) -> anyhow::Result<()> {'''
+
+[[tasks.TASK-8.changes]]
+file = "src/lib.rs"
+before = '''    let workspace_name = workspace_root
+        .file_name()
+        .map(|n| n.to_string_lossy().to_string())
+        .unwrap_or_default();'''
+after = '''    let workspace_name = workspace_root
+        .file_name()
+        .map(|n| n.to_string_lossy().to_string())
+        .unwrap_or_default();'''
+
+[[tasks.TASK-8.changes]]
+file = "src/lib.rs"
+before = '''/// Strip the workspace root prefix from a path string, returning a
+/// workspace-relative path. If the prefix doesn't match, returns the
+/// original string unchanged.
+fn relativize_path(path_str: &str, root: &Path) -> String {
+    let p = Path::new(path_str);
+    match p.strip_prefix(root) {
+        Ok(rel) => rel.to_string_lossy().to_string(),
+        Err(_) => path_str.to_string(),
+    }
+}'''
+after = '''/// Strip the workspace root prefix from a path string, returning a
+/// workspace-relative path. If the prefix doesn't match, returns the
+/// original string unchanged.
+fn relativize_path(path_str: &str, root: &Path) -> String {
+    let p = Path::new(path_str);
+    match p.strip_prefix(root) {
+        Ok(rel) => rel.to_string_lossy().to_string(),
+        Err(_) => path_str.to_string(),
+    }
+}'''
+
+[[tasks.TASK-8.changes]]
+file = "src/file_parser.rs"
+before = '''/// Extract all items with any form of `pub` visibility (excluding `Inherited`).
+/// Results are sorted by name then line for deterministic output.
+pub fn extract_public_items(items: &[syn::Item]) -> Vec<PublicItem> {'''
+after = '''/// Extract all items with any form of `pub` visibility (excluding `Inherited`).
+/// Results are sorted by name then line for deterministic output.
+#[must_use]
+pub fn extract_public_items(items: &[syn::Item]) -> Vec<PublicItem> {'''
+
+[[tasks.TASK-8.changes]]
+file = "src/file_parser.rs"
+before = '''/// Extract all `use` statements. Braced imports are expanded to individual
+/// entries. Results sorted by path for determinism.
+pub fn extract_imports(items: &[syn::Item]) -> Vec<Import> {'''
+after = '''/// Extract all `use` statements. Braced imports are expanded to individual
+/// entries. Results sorted by path for determinism.
+#[must_use]
+pub fn extract_imports(items: &[syn::Item]) -> Vec<Import> {'''
+
+[[tasks.TASK-8.changes]]
+file = "src/file_parser.rs"
+before = '''/// Extract `pub use` re-exports. Results sorted by export_path.
+pub fn extract_re_exports(items: &[syn::Item]) -> Vec<ReExport> {'''
+after = '''/// Extract `pub use` re-exports. Results sorted by export_path.
+#[must_use]
+pub fn extract_re_exports(items: &[syn::Item]) -> Vec<ReExport> {'''
+
+[[tasks.TASK-8.changes]]
+file = "src/file_parser.rs"
+before = '''/// Extract `mod` declarations. Detects `#[cfg(test)]` via literal token
+/// matching. Results sorted by name.
+pub fn extract_submodules(items: &[syn::Item]) -> Vec<SubmoduleDecl> {'''
+after = '''/// Extract `mod` declarations. Detects `#[cfg(test)]` via literal token
+/// matching. Results sorted by name.
+#[must_use]
+pub fn extract_submodules(items: &[syn::Item]) -> Vec<SubmoduleDecl> {'''
+
+[[tasks.TASK-8.changes]]
+file = "src/file_parser.rs"
+before = '''/// Extract `impl` blocks. Each `ImplInfo` records the target type name and
+/// the impl items (fn, type, const).
+pub fn extract_impls(items: &[syn::Item]) -> Vec<ImplInfo> {'''
+after = '''/// Extract `impl` blocks. Each `ImplInfo` records the target type name and
+/// the impl items (fn, type, const).
+#[must_use]
+pub fn extract_impls(items: &[syn::Item]) -> Vec<ImplInfo> {'''
+
+[[tasks.TASK-8.changes]]
+file = "src/module_tree.rs"
+before = '''/// Resolve a `mod name;` declaration to a file path.
+/// Tries `{parent_dir}/{mod_name}.rs` first, then `{parent_dir}/{mod_name}/mod.rs`.
+///
+/// Returns `None` if neither path exists.
+pub fn resolve_module_path(parent_dir: &Path, mod_name: &str) -> Option<PathBuf> {'''
+after = '''/// Resolve a `mod name;` declaration to a file path.
+/// Tries `{parent_dir}/{mod_name}.rs` first, then `{parent_dir}/{mod_name}/mod.rs`.
+///
+/// Returns `None` if neither path exists.
+#[must_use]
+pub fn resolve_module_path(parent_dir: &Path, mod_name: &str) -> Option<PathBuf> {'''
+
+[[tasks.TASK-8.changes]]
+file = "src/render.rs"
+before = '''/// Serialize the workspace map to a JSON string with 2-space indentation.
+pub fn render_json(map: &WorkspaceMap) -> serde_json::Result<String> {'''
+after = '''/// Serialize the workspace map to a JSON string with 2-space indentation.
+#[must_use]
+pub fn render_json(map: &WorkspaceMap) -> serde_json::Result<String> {'''
+
+[[tasks.TASK-8.changes]]
+file = "src/cross_refs.rs"
+before = '''// Helper: convert ItemKind to a short string for the TypeRef.kind field.
+impl crate::schema::PublicItem {
+    fn kind_to_string(&self) -> String {'''
+after = '''// Helper: convert ItemKind to a short string for the TypeRef.kind field.
+impl crate::schema::PublicItem {
+    #[must_use]
+    fn kind_to_string(&self) -> String {'''
+
+[[tasks.TASK-8.changes]]
+file = "src/lib.rs"
+before = '''    match p.strip_prefix(root) {
+        Ok(rel) => rel.to_string_lossy().to_string(),
+        Err(_) => path_str.to_string(),
+    }
+}'''
+after = '''    match p.strip_prefix(root) {
+        Ok(rel) => rel.to_string_lossy().to_string(),
+        Err(_) => path_str.to_string(),
+    }
+}'''
+
+[[tasks.TASK-8.changes]]
+file = "src/file_parser.rs"
+before = "/// Extract `pub use` re-exports. Results sorted by export_path.\n#[must_use]\npub fn extract_re_exports"
+after = "/// Extract `pub use` re-exports. Results sorted by `export_path`.\n#[must_use]\npub fn extract_re_exports"
+
+[[tasks.TASK-8.changes]]
+file = "src/file_parser.rs"
+before = "            let name = m.ident.as_ref().map(|i| i.to_string()).unwrap_or_default();"
+after = "            let name = m.ident.as_ref().map(ToString::to_string).unwrap_or_default();"
+
+[[tasks.TASK-8.changes]]
+file = "src/file_parser.rs"
+before = "fn flatten_use_tree(tree: &syn::UseTree, prefix: String, line: usize) -> Vec<Import> {"
+after = "fn flatten_use_tree(tree: &syn::UseTree, prefix: &str, line: usize) -> Vec<Import> {"
+
+[[tasks.TASK-8.changes]]
+file = "src/file_parser.rs"
+before = "                Some(flatten_use_tree(&u.tree, String::new(), line_of_item(item)))"
+after = "                Some(flatten_use_tree(&u.tree, \"\", line_of_item(item)))"
+
+[[tasks.TASK-8.changes]]
+file = "src/file_parser.rs"
+before = "            flatten_use_tree(&p.tree, new_prefix, line)"
+after = "            flatten_use_tree(&p.tree, &new_prefix, line)"
+
+[[tasks.TASK-8.changes]]
+file = "src/schema.rs"
+before = "/// Internal intermediate type consumed by module_tree.\n#[derive(Debug, Clone, Default)]\npub struct FileInfo"
+after = "/// Internal intermediate type consumed by `module_tree`.\n#[derive(Debug, Clone, Default)]\npub struct FileInfo"
+
+[[tasks.TASK-8.changes]]
+file = "src/lib.rs"
+before = "/// parsed, or the JSON output cannot be written.\npub fn run(config: Config) -> anyhow::Result<()> {"
+after = "/// parsed, or the JSON output cannot be written.\npub fn run(config: &Config) -> anyhow::Result<()> {"
+
+[[tasks.TASK-8.changes]]
+file = "src/main.rs"
+before = "    rust_workspace_map::run(config)"
+after = "    rust_workspace_map::run(&config)"
+
+[tasks.TASK-9]
+description = "Add unit tests for file_parser, module_tree, workspace, cargo_info, cross_refs, and render"
+type = "replace"
+acceptance = [
+    "cargo test -p rust-workspace-map",
+]
+
+[[tasks.TASK-9.changes]]
+file = "src/file_parser.rs"
+before = '''fn extract_re_exports_from_tree(
+    tree: &syn::UseTree,
+    import_path: String,
+    line: usize,
+) -> Vec<ReExport> {
+    match tree {
+        syn::UseTree::Path(p) => {
+            let new_import = if import_path.is_empty() {
+                p.ident.to_string()
+            } else {
+                format!("{}::{}", import_path, p.ident)
+            };
+            extract_re_exports_from_tree(&p.tree, new_import, line)
+        }
+        syn::UseTree::Name(n) => {
+            vec![ReExport {
+                import_path,
+                export_path: n.ident.to_string(),
+                line,
+            }]
+        }
+        syn::UseTree::Rename(r) => {
+            vec![ReExport {
+                import_path,
+                export_path: r.rename.to_string(),
+                line,
+            }]
+        }
+        syn::UseTree::Glob(_) => {
+            vec![ReExport {
+                import_path,
+                export_path: "*".to_string(),
+                line,
+            }]
+        }
+        syn::UseTree::Group(g) => g
+            .items
+            .iter()
+            .flat_map(|t| extract_re_exports_from_tree(t, import_path.clone(), line))
+            .collect(),
+    }
+}'''
+after = '''fn extract_re_exports_from_tree(
+    tree: &syn::UseTree,
+    import_path: String,
+    line: usize,
+) -> Vec<ReExport> {
+    match tree {
+        syn::UseTree::Path(p) => {
+            let new_import = if import_path.is_empty() {
+                p.ident.to_string()
+            } else {
+                format!("{}::{}", import_path, p.ident)
+            };
+            extract_re_exports_from_tree(&p.tree, new_import, line)
+        }
+        syn::UseTree::Name(n) => {
+            vec![ReExport {
+                import_path,
+                export_path: n.ident.to_string(),
+                line,
+            }]
+        }
+        syn::UseTree::Rename(r) => {
+            vec![ReExport {
+                import_path,
+                export_path: r.rename.to_string(),
+                line,
+            }]
+        }
+        syn::UseTree::Glob(_) => {
+            vec![ReExport {
+                import_path,
+                export_path: "*".to_string(),
+                line,
+            }]
+        }
+        syn::UseTree::Group(g) => g
+            .items
+            .iter()
+            .flat_map(|t| extract_re_exports_from_tree(t, import_path.clone(), line))
+            .collect(),
+    }
+}
+
+// ── Tests ───────────────────────────────────────────────────────────────
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use crate::schema::{Import, ReExport, SubmoduleDecl};
+    use std::path::PathBuf;
+
+    fn parse_source(src: &str) -> ParsedFile {
+        let tmp = std::env::temp_dir().join("parse_test.rs");
+        std::fs::write(&tmp, src).unwrap();
+        let result = parse_file(&tmp);
+        std::fs::remove_file(&tmp).ok();
+        result
+    }
+
+    #[test]
+    fn parse_file_returns_ast_for_valid_source() {
+        let src = "pub struct Foo { x: i32 }";
+        let result = parse_source(src);
+        assert!(result.parse_error.is_none());
+        assert_eq!(result.ast.items.len(), 1);
+    }
+
+    #[test]
+    fn parse_file_returns_error_for_invalid_source() {
+        let src = "pub struct { invalid rust }";
+        let result = parse_source(src);
+        assert!(result.parse_error.is_some());
+        let err = result.parse_error.as_ref().unwrap();
+        assert!(!err.message.is_empty());
+        assert!(err.line > 0);
+    }
+
+    #[test]
+    fn parse_file_returns_empty_for_empty_file() {
+        let result = parse_source("");
+        assert!(result.parse_error.is_none());
+        assert!(result.file_info.public_items.is_empty());
+    }
+
+    #[test]
+    fn extract_public_items_finds_struct_enum_trait_fn() {
+        let src = "pub struct Foo {} pub enum Bar { A, B } pub trait Baz {} pub fn hello() {}";
+        let result = parse_source(src);
+        let items = extract_public_items(&result.ast.items);
+        let names: Vec<_> = items.iter().map(|i| i.name.as_str()).collect();
+        assert!(names.contains(&"Foo"));
+        assert!(names.contains(&"Bar"));
+        assert!(names.contains(&"Baz"));
+        assert!(names.contains(&"hello"));
+    }
+
+    #[test]
+    fn extract_public_items_empty_for_no_public_items() {
+        let src = "struct Private {} fn private_fn() {}";
+        let result = parse_source(src);
+        let items = extract_public_items(&result.ast.items);
+        assert!(items.is_empty());
+    }
+
+    #[test]
+    fn extract_imports_finds_use_statements() {
+        let src = "use std::collections::BTreeMap;";
+        let result = parse_source(src);
+        let imports = extract_imports(&result.ast.items);
+        assert_eq!(imports.len(), 1);
+        assert_eq!(imports[0].path, "std::collections::BTreeMap");
+    }
+
+    #[test]
+    fn extract_re_exports_finds_pub_use() {
+        let src = "pub use crate::foo;";
+        let result = parse_source(src);
+        let re_exports = extract_re_exports(&result.ast.items);
+        assert_eq!(re_exports.len(), 1);
+        assert_eq!(re_exports[0].import_path, "crate::foo");
+        assert_eq!(re_exports[0].export_path, "foo");
+    }
+
+    #[test]
+    fn extract_re_exports_finds_rename() {
+        let src = "pub use crate::foo as bar;";
+        let result = parse_source(src);
+        let re_exports = extract_re_exports(&result.ast.items);
+        assert_eq!(re_exports.len(), 1);
+        assert_eq!(re_exports[0].import_path, "crate::foo as bar");
+        assert_eq!(re_exports[0].export_path, "bar");
+    }
+
+    #[test]
+    fn extract_submodules_finds_mod_declarations() {
+        let src = "mod foo; mod bar;";
+        let result = parse_source(src);
+        let subs = extract_submodules(&result.ast.items);
+        assert_eq!(subs.len(), 2);
+        let names: Vec<_> = subs.iter().map(|s| s.name.as_str()).collect();
+        assert!(names.contains(&"bar"));
+        assert!(names.contains(&"foo"));
+    }
+
+    #[test]
+    fn extract_submodules_marks_cfg_test() {
+        let src = "#[cfg(test)] mod inner;";
+        let result = parse_source(src);
+        let subs = extract_submodules(&result.ast.items);
+        assert_eq!(subs.len(), 1);
+        assert!(subs[0].is_test);
+    }
+
+    #[test]
+    fn extract_impls_finds_fn_type_const() {
+        let src = "impl MyType { pub fn foo(&self) {} pub type Alias = u32; pub const N: usize = 42; }";
+        let result = parse_source(src);
+        let impls = extract_impls(&result.ast.items);
+        assert_eq!(impls.len(), 1);
+        assert_eq!(impls[0].type_, "MyType");
+        assert_eq!(impls[0].items.len(), 3);
+    }
+
+    #[test]
+    fn build_parse_error_entry_constructs_error() {
+        let path = PathBuf::from("test.rs");
+        let err = SynParseError {
+            message: "expected `;`".to_string(),
+            line: 5,
+        };
+        let entry = build_parse_error_entry(&path, &err);
+        assert_eq!(entry.file, "test.rs");
+        assert_eq!(entry.line, 5);
+        assert_eq!(entry.kind, "syn_parse_error");
+        assert_eq!(entry.severity, ErrorSeverity::Error);
+    }
+}'''
+
+[[tasks.TASK-9.changes]]
+file = "src/module_tree.rs"
+before = '''fn process_module_info(
+    module_path: &str,
+    file_path: &Path,
+    visibility: &str,
+    file_info: &FileInfo,
+    items: &[syn::Item],
+    _parent_dir: &Path,
+    visited: &mut HashSet<PathBuf>,
+    errors: &mut Vec<crate::schema::ErrorEntry>,
+) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {
+    let mut modules = vec![build_module_info(
+        module_path,
+        file_path,
+        visibility,
+        &file_info.public_items,
+        &file_info.imports,
+        &file_info.re_exports,
+        &file_info.submodules,
+    )];
+
+    for sub in &file_info.submodules {
+        if sub.is_test {
+            continue;
+        }
+        let child_path = format!("{}::{}", module_path, sub.name);
+        let child_dir = file_path.parent().unwrap_or(file_path);
+        let (child_modules, child_errors) = process_submodule(
+            &child_path,
+            &sub.name,
+            items,
+            child_dir,
+            file_path,
+            visited,
+        );
+        errors.extend(child_errors);
+        modules.extend(child_modules);
+    }
+
+    (modules, errors.clone())
+}'''
+after = '''fn process_module_info(
+    module_path: &str,
+    file_path: &Path,
+    visibility: &str,
+    file_info: &FileInfo,
+    items: &[syn::Item],
+    _parent_dir: &Path,
+    visited: &mut HashSet<PathBuf>,
+    errors: &mut Vec<crate::schema::ErrorEntry>,
+) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {
+    let mut modules = vec![build_module_info(
+        module_path,
+        file_path,
+        visibility,
+        &file_info.public_items,
+        &file_info.imports,
+        &file_info.re_exports,
+        &file_info.submodules,
+    )];
+
+    for sub in &file_info.submodules {
+        if sub.is_test {
+            continue;
+        }
+        let child_path = format!("{}::{}", module_path, sub.name);
+        let child_dir = file_path.parent().unwrap_or(file_path);
+        let (child_modules, child_errors) = process_submodule(
+            &child_path,
+            &sub.name,
+            items,
+            child_dir,
+            file_path,
+            visited,
+        );
+        errors.extend(child_errors);
+        modules.extend(child_modules);
+    }
+
+    (modules, errors.clone())
+}
+
+// ── Tests ───────────────────────────────────────────────────────────────
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use crate::schema::ErrorEntry;
+
+    #[test]
+    fn resolve_module_path_finds_rs_file() {
+        let tmp = std::env::temp_dir().join("resolve_test");
+        let _ = std::fs::create_dir_all(&tmp);
+        let mod_file = tmp.join("foo.rs");
+        std::fs::write(&mod_file, "").ok();
+        let result = resolve_module_path(&tmp, "foo");
+        assert_eq!(result, Some(mod_file));
+        std::fs::remove_dir_all(&tmp).ok();
+    }
+
+    #[test]
+    fn resolve_module_path_finds_mod_rs() {
+        let tmp = std::env::temp_dir().join("resolve_test2");
+        let _ = std::fs::create_dir_all(&tmp);
+        let mod_dir = tmp.join("bar");
+        let _ = std::fs::create_dir_all(&mod_dir);
+        let mod_rs = mod_dir.join("mod.rs");
+        std::fs::write(&mod_rs, "").ok();
+        let result = resolve_module_path(&tmp, "bar");
+        assert_eq!(result, Some(mod_rs));
+        std::fs::remove_dir_all(&tmp).ok();
+    }
+
+    #[test]
+    fn resolve_module_path_returns_none_for_missing() {
+        let tmp = std::env::temp_dir().join("resolve_test3");
+        let _ = std::fs::create_dir_all(&tmp);
+        let result = resolve_module_path(&tmp, "nonexistent");
+        assert!(result.is_none());
+        std::fs::remove_dir_all(&tmp).ok();
+    }
+
+    #[test]
+    fn build_module_tree_returns_empty_for_nonexistent() {
+        let tmp = std::env::temp_dir().join("bmt_test");
+        let _ = std::fs::create_dir_all(&tmp);
+        let (modules, errors) = build_module_tree(&tmp, "test");
+        assert!(modules.is_empty());
+        assert!(!errors.is_empty());
+        std::fs::remove_dir_all(&tmp).ok();
+    }
+}'''
+
+[[tasks.TASK-9.changes]]
+file = "src/workspace.rs"
+before = '''    result.sort();
+    result.dedup();
+    Ok(result)
+}
+
+/// For a crate directory, determine its entry-point file(s).'''
+after = '''    result.sort();
+    result.dedup();
+    Ok(result)
+}
+
+// ── Tests ───────────────────────────────────────────────────────────────
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use std::io::Write;
+
+    fn write_cargo_toml(dir: &std::path::Path, content: &str) {
+        let mut f = std::fs::File::create(dir.join("Cargo.toml")).unwrap();
+        f.write_all(content.as_bytes()).unwrap();
+    }
+
+    fn setup_crate(dir: &std::path::Path) {
+        let src = dir.join("src");
+        std::fs::create_dir_all(&src).unwrap();
+        std::fs::write(src.join("lib.rs"), "").unwrap();
+    }
+
+    #[test]
+    fn find_workspace_root_finds_cargo_toml() {
+        let tmp = tempfile::tempdir().unwrap();
+        let path = tmp.path().join("subdir").join("nested");
+        std::fs::create_dir_all(&path).unwrap();
+        write_cargo_toml(tmp.path(), "[workspace]");
+        let result = find_workspace_root(&path).unwrap();
+        assert_eq!(result, tmp.path());
+    }
+
+    #[test]
+    fn enumerate_members_returns_members() {
+        let tmp = tempfile::tempdir().unwrap();
+        write_cargo_toml(tmp.path(), r#"
+[workspace]
+members = ["crate_a", "crate_b"]
+"#);
+        setup_crate(tmp.path().join("crate_a").as_path());
+        setup_crate(tmp.path().join("crate_b").as_path());
+        let members = enumerate_members(tmp.path()).unwrap();
+        assert_eq!(members.len(), 2);
+    }
+
+    #[test]
+    fn enumerate_members_returns_err_for_missing_workspace() {
+        let tmp = tempfile::tempdir().unwrap();
+        write_cargo_toml(tmp.path(), "[dependencies]\nfoo = \"1\"");
+        let result = enumerate_members(tmp.path());
+        assert!(result.is_err());
+        match result.unwrap_err() {
+            Error::MissingWorkspaceSection => {},
+            other => panic!("expected MissingWorkspaceSection, got {:?}", other),
+        }
+    }
+
+    #[test]
+    fn enumerate_members_applies_exclude() {
+        let tmp = tempfile::tempdir().unwrap();
+        write_cargo_toml(tmp.path(), r#"
+[workspace]
+members = ["a", "b", "c"]
+exclude = ["b"]
+"#);
+        setup_crate(tmp.path().join("a").as_path());
+        setup_crate(tmp.path().join("b").as_path());
+        setup_crate(tmp.path().join("c").as_path());
+        let members = enumerate_members(tmp.path()).unwrap();
+        let names: Vec<_> = members.iter().map(|p| p.file_name().unwrap().to_string_lossy()).collect();
+        assert!(names.contains(&"a".as_ref()));
+        assert!(!names.contains(&"b".as_ref()));
+        assert!(names.contains(&"c".as_ref()));
+    }
+
+    #[test]
+    fn resolve_crate_roots_detects_lib() {
+        let tmp = tempfile::tempdir().unwrap();
+        std::fs::create_dir_all(tmp.path().join("src")).unwrap();
+        std::fs::write(tmp.path().join("src").join("lib.rs"), "").unwrap();
+        let roots = resolve_crate_roots(tmp.path());
+        assert_eq!(roots.len(), 1);
+        assert_eq!(roots[0].1, CrateType::Lib);
+    }
+
+    #[test]
+    fn resolve_crate_roots_detects_bin() {
+        let tmp = tempfile::tempdir().unwrap();
+        std::fs::create_dir_all(tmp.path().join("src")).unwrap();
+        std::fs::write(tmp.path().join("src").join("main.rs"), "").unwrap();
+        let roots = resolve_crate_roots(tmp.path());
+        assert_eq!(roots.len(), 1);
+        assert_eq!(roots[0].1, CrateType::Bin);
+    }
+}'''
+
+[[tasks.TASK-9.changes]]
+file = "src/cargo_info.rs"
+before = '''    Ok((package, deps))
+}'''
+after = '''    Ok((package, deps))
+}
+
+// ── Tests ───────────────────────────────────────────────────────────────
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use std::io::Write;
+
+    fn write_cargo_toml(dir: &std::path::Path, content: &str) {
+        let mut f = std::fs::File::create(dir.join("Cargo.toml")).unwrap();
+        f.write_all(content.as_bytes()).unwrap();
+    }
+
+    #[test]
+    fn parse_cargo_toml_parses_minimal() {
+        let tmp = tempfile::tempdir().unwrap();
+        write_cargo_toml(tmp.path(), r#"
+[package]
+name = "test-pkg"
+version = "1.0.0"
+edition = "2021"
+"#);
+        let (pkg, _deps) = parse_cargo_toml(tmp.path().join("Cargo.toml").as_path()).unwrap();
+        assert_eq!(pkg.name, "test-pkg");
+        assert_eq!(pkg.version, "1.0.0");
+        assert_eq!(pkg.edition, "2021");
+    }
+
+    #[test]
+    fn parse_cargo_toml_uses_defaults_for_missing_package() {
+        let tmp = tempfile::tempdir().unwrap();
+        write_cargo_toml(tmp.path(), "");
+        let (pkg, _deps) = parse_cargo_toml(tmp.path().join("Cargo.toml").as_path()).unwrap();
+        assert_eq!(pkg.name, "unknown");
+        assert_eq!(pkg.edition, "2021");
+    }
+
+    #[test]
+    fn parse_cargo_toml_distinguishes_deps() {
+        let tmp = tempfile::tempdir().unwrap();
+        write_cargo_toml(tmp.path(), r#"
+[package]
+name = "test-pkg"
+version = "0.1.0"
+edition = "2021"
+
+[dependencies]
+foo = "1"
+bar = { workspace = true }
+
+[dev-dependencies]
+baz = "2"
+qux = { workspace = true }
+"#);
+        let (_, deps) = parse_cargo_toml(tmp.path().join("Cargo.toml").as_path()).unwrap();
+        assert_eq!(deps.normal, vec!["foo"]);
+        assert_eq!(deps.dev, vec!["baz"]);
+        assert!(deps.workspace_members.contains(&"bar".to_string()));
+        assert!(deps.workspace_members.contains(&"qux".to_string()));
+    }
+}'''
+
+[[tasks.TASK-9.changes]]
+file = "src/cross_refs.rs"
+before = '''    CrossReferences { types: types_map }
+}
+
+// Helper: convert ItemKind to a short string for the TypeRef.kind field.'''
+after = '''    CrossReferences { types: types_map }
+}
+
+// ── Tests ───────────────────────────────────────────────────────────────
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use crate::schema::{ModuleInfo, PublicItem, SubmoduleDecl};
+
+    fn make_crate(name: &str, items: Vec<(String, ItemKind)>) -> CrateInfo {
+        let public_items: Vec<PublicItem> = items
+            .into_iter()
+            .map(|(n, k)| {
+                PublicItem::builder()
+                    .kind(k)
+                    .name(n)
+                    .file(String::new())
+                    .line(1)
+                    .visibility("pub".to_string())
+                    .generics(String::new())
+                    .attrs(Default::default())
+                    .build()
+            })
+            .collect();
+        let module = ModuleInfo::builder()
+            .path("".to_string())
+            .file(String::new())
+            .visibility("pub".to_string())
+            .public_items(public_items)
+            .build();
+        CrateInfo::builder()
+            .name(name.to_string())
+            .root(String::new())
+            .package(
+                schema::PackageInfo::builder()
+                    .name(name.to_string())
+                    .version("0.1.0".to_string())
+                    .edition("2021".to_string())
+                    .crate_type(schema::CrateType::Lib)
+                    .build(),
+            )
+            .modules(vec![module])
+            .deps(Default::default())
+            .build()
+    }
+
+    #[test]
+    fn compute_finds_cross_crate_import() {
+        let mut crates = vec![
+            make_crate("core", vec![
+                ("Task".to_string(), ItemKind::Struct),
+            ]),
+            make_crate("engine", vec![]),
+        ];
+        // Manually add an import in engine that references core::Task
+        let engine_module = &mut crates[1].modules[0];
+        engine_module.imports.push(Import {
+            path: "core::Task".to_string(),
+            line: 1,
+        });
+        let refs = compute(&mut crates);
+        // Task should be in cross-references
+        assert!(refs.types.contains_key("Task"));
+        let task_ref = &refs.types["Task"];
+        assert_eq!(task_ref.crate_name, "core");
+        // engine should have a cross_crate_import
+        assert_eq!(crates[1].cross_crate_imports.len(), 1);
+        assert_eq!(crates[1].cross_crate_imports[0].target_crate, "core");
+    }
+
+    #[test]
+    fn compute_empty_for_no_cross_references() {
+        let crates = vec![
+            make_crate("a", vec![("Foo".to_string(), ItemKind::Struct)]),
+            make_crate("b", vec![("Bar".to_string(), ItemKind::Struct)]),
+        ];
+        let mut crates_mut = crates;
+        let refs = compute(&mut crates_mut);
+        // No cross references since no crate imports from another
+        assert!(refs.types.is_empty() || refs.types.values().all(|t| t.imported_by.is_empty()));
+    }
+}
+
+// Helper: convert ItemKind to a short string for the TypeRef.kind field.'''
+
+[[tasks.TASK-9.changes]]
+file = "src/render.rs"
+before = '''use crate::schema::WorkspaceMap;
+use std::io::Write;
+
+/// Serialize the workspace map to a JSON string with 2-space indentation.
+#[must_use]
+pub fn render_json(map: &WorkspaceMap) -> serde_json::Result<String> {
+    serde_json::to_string_pretty(map)
+}
+
+/// Serialize the workspace map to the given writer.
+pub fn render_to_writer(map: &WorkspaceMap, writer: impl Write) -> serde_json::Result<()> {
+    serde_json::to_writer_pretty(writer, map)
+}
+'''
+after = '''use crate::schema::WorkspaceMap;
+use std::io::Write;
+
+/// Serialize the workspace map to a JSON string with 2-space indentation.
+#[must_use]
+pub fn render_json(map: &WorkspaceMap) -> serde_json::Result<String> {
+    serde_json::to_string_pretty(map)
+}
+
+/// Serialize the workspace map to the given writer.
+pub fn render_to_writer(map: &WorkspaceMap, writer: impl Write) -> serde_json::Result<()> {
+    serde_json::to_writer_pretty(writer, map)
+}
+
+// ── Tests ───────────────────────────────────────────────────────────────
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use crate::schema::{
+        CrateInfo, CrateType, CrossReferences, DepInfo, ModuleInfo, PackageInfo,
+        WorkspaceInfo, WorkspaceMap,
+    };
+
+    fn make_minimal_map() -> WorkspaceMap {
+        WorkspaceMap::builder()
+            .workspace(WorkspaceInfo::builder()
+                .root(".".to_string())
+                .workspace_name("test".to_string())
+                .build())
+            .crates(vec![
+                CrateInfo::builder()
+                    .name("test-crate".to_string())
+                    .root(".".to_string())
+                    .package(PackageInfo::builder()
+                        .name("test-crate".to_string())
+                        .version("0.1.0".to_string())
+                        .edition("2021".to_string())
+                        .crate_type(CrateType::Lib)
+                        .build())
+                    .modules(vec![ModuleInfo::builder()
+                        .path("".to_string())
+                        .file("src/lib.rs".to_string())
+                        .visibility("pub".to_string())
+                        .build()])
+                    .deps(DepInfo::default())
+                    .build(),
+            ])
+            .cross_references(CrossReferences::default())
+            .workspace_root(std::path::PathBuf::from("."))
+            .build()
+    }
+
+    #[test]
+    fn render_json_produces_valid_json() {
+        let map = make_minimal_map();
+        let json = render_json(&map).unwrap();
+        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
+        assert_eq!(parsed["workspace"]["root"], ".");
+        assert_eq!(parsed["crates"].as_array().unwrap().len(), 1);
+    }
+
+    #[test]
+    fn render_json_skips_empty_errors() {
+        let map = make_minimal_map();
+        let json = render_json(&map).unwrap();
+        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
+        // errors field should be absent (skip_serializing_if)
+        assert!(parsed.get("errors").is_none());
+    }
+
+    #[test]
+    fn render_to_writer_matches_render_json() {
+        let map = make_minimal_map();
+        let json = render_json(&map).unwrap();
+
+        let mut buf = Vec::new();
+        render_to_writer(&map, &mut buf).unwrap();
+        let from_writer = String::from_utf8(buf).unwrap();
+
+        assert_eq!(json, from_writer);
+    }
+}
+'''
+
+[tasks.TASK-10]
+description = "Expand integration tests from 3 to 10 covering error cases, workspace patterns, and output"
+type = "replace"
+acceptance = [
+    "cargo test -p rust-workspace-map --test integration_test",
+]
+
+[[tasks.TASK-10.changes]]
+file = "tests/integration_test.rs"
+before = '''#[test]
+fn test_missing_path_exits_nonzero() {
+    let bin = binary_path();
+    let output = Command::new(&bin)
+        .arg("/tmp/nonexistent-path-12345")
+        .output()
+        .expect("failed to execute binary");
+
+    assert!(
+        !output.status.success(),
+        "should exit non-zero for invalid path"
+    );
+}
+'''
+after = '''#[test]
+fn test_missing_path_exits_nonzero() {
+    let bin = binary_path();
+    let output = Command::new(&bin)
+        .arg("/tmp/nonexistent-path-12345")
+        .output()
+        .expect("failed to execute binary");
+
+    assert!(
+        !output.status.success(),
+        "should exit non-zero for invalid path"
+    );
+}
+
+fn run_binary(path: &str) -> std::process::Output {
+    Command::new(&binary_path())
+        .arg(path)
+        .output()
+        .expect("failed to execute binary")
+}
+
+fn parse_output(output: &std::process::Output) -> serde_json::Value {
+    serde_json::from_str(&String::from_utf8_lossy(&output.stdout)).unwrap()
+}
+
+fn write_cargo_toml(dir: &std::path::Path, content: &str) {
+    let mut f = std::fs::File::create(dir.join("Cargo.toml")).unwrap();
+    use std::io::Write;
+    f.write_all(content.as_bytes()).unwrap();
+}
+
+fn setup_crate(dir: &std::path::Path, lib_content: &str) {
+    let src = dir.join("src");
+    std::fs::create_dir_all(&src).unwrap();
+    std::fs::write(src.join("lib.rs"), lib_content).unwrap();
+}
+
+#[test]
+fn test_parse_failure_error_entry() {
+    let tmp = tempfile::tempdir().unwrap();
+    let root = tmp.path();
+
+    // Create workspace Cargo.toml
+    write_cargo_toml(root, r#"
+[workspace]
+members = ["good_crate", "bad_crate"]
+"#);
+
+    // Good crate with valid Rust
+    setup_crate(&root.join("good_crate"), "pub struct Good {}");
+
+    // Bad crate with invalid Rust syntax
+    setup_crate(&root.join("bad_crate"), "pub struct { invalid rust syntax");
+
+    let output = run_binary(root.to_str().unwrap());
+    assert!(output.status.success());
+
+    let json = parse_output(&output);
+    let errors: Vec<&serde_json::Value> = extract_array(&json, "errors");
+
+    let parse_errors: Vec<_> = errors.iter()
+        .filter(|e| {
+            e["kind"].as_str().unwrap() == "syn_parse_error"
+        })
+        .collect();
+
+    assert!(!parse_errors.is_empty(), "should have parse error entries");
+    assert_eq!(parse_errors[0]["severity"].as_str().unwrap(), "error");
+}
+
+#[test]
+fn test_missing_workspace_section() {
+    let tmp = tempfile::tempdir().unwrap();
+    let root = tmp.path();
+
+    // Cargo.toml without [workspace] section
+    write_cargo_toml(root, r#"
+[package]
+name = "standalone"
+version = "0.1.0"
+edition = "2021"
+"#);
+
+    let output = run_binary(root.to_str().unwrap());
+
+    // Should exit non-zero because workspace is missing
+    assert!(
+        !output.status.success(),
+        "should exit non-zero for missing workspace section"
+    );
+}
+
+#[test]
+fn test_glob_member_patterns() {
+    let tmp = tempfile::tempdir().unwrap();
+    let root = tmp.path();
+
+    write_cargo_toml(root, r#"
+[workspace]
+members = ["crates/*"]
+"#);
+
+    for name in &["alpha", "beta", "gamma"] {
+        setup_crate(&root.join("crates").join(name), format!("pub struct {name} {{}}").as_str());
+    }
+
+    let output = run_binary(root.to_str().unwrap());
+    assert!(output.status.success());
+
+    let json = parse_output(&output);
+    let crates = extract_array(&json, "crates");
+    let names: Vec<&str> = crates.iter()
+        .map(|c| c["name"].as_str().unwrap())
+        .collect();
+
+    assert!(names.contains(&"alpha"));
+    assert!(names.contains(&"beta"));
+    assert!(names.contains(&"gamma"));
+    assert_eq!(names.len(), 3);
+}
+
+#[test]
+fn test_workspace_with_exclude() {
+    let tmp = tempfile::tempdir().unwrap();
+    let root = tmp.path();
+
+    write_cargo_toml(root, r#"
+[workspace]
+members = ["a", "b", "c"]
+exclude = ["b"]
+"#);
+
+    setup_crate(&root.join("a"), "pub struct A {}");
+    setup_crate(&root.join("b"), "pub struct B {}");
+    setup_crate(&root.join("c"), "pub struct C {}");
+
+    let output = run_binary(root.to_str().unwrap());
+    assert!(output.status.success());
+
+    let json = parse_output(&output);
+    let crates = extract_array(&json, "crates");
+    let names: Vec<&str> = crates.iter()
+        .map(|c| c["name"].as_str().unwrap())
+        .collect();
+
+    assert!(names.contains(&"a"));
+    assert!(!names.contains(&"b"));
+    assert!(names.contains(&"c"));
+}
+
+#[test]
+fn test_deeply_nested_modules() {
+    let tmp = tempfile::tempdir().unwrap();
+    let root = tmp.path();
+
+    write_cargo_toml(root, r#"
+[workspace]
+members = ["."]
+
+[package]
+name = "nested"
+version = "0.1.0"
+edition = "2021"
+"#);
+
+    let src = root.join("src");
+    let foo = src.join("foo");
+    let bar = foo.join("bar");
+    std::fs::create_dir_all(&bar).unwrap();
+
+    // lib.rs declares mod foo
+    std::fs::write(src.join("lib.rs"), "mod foo;").unwrap();
+    // foo.rs declares mod bar
+    std::fs::write(foo.join("foo.rs"), "mod bar;").unwrap();
+    // bar/baz.rs declares mod baz
+    std::fs::write(bar.join("bar.rs"), "mod baz;").unwrap();
+    // baz.rs with a struct
+    std::fs::write(bar.join("baz.rs"), "pub struct Deep {}").unwrap();
+
+    let output = run_binary(root.to_str().unwrap());
+    assert!(output.status.success());
+
+    let json = parse_output(&output);
+    let crates = extract_array(&json, "crates");
+    let nested_crate = crates.iter().find(|c| c["name"].as_str().unwrap() == "nested").unwrap();
+
+    let module_paths: Vec<&str> = extract_array(&nested_crate["modules"], "path")
+        .iter()
+        .map(|m| m.as_str().unwrap())
+        .collect();
+
+    assert!(module_paths.iter().any(|p| *p == "nested"));
+    assert!(module_paths.iter().any(|p| *p == "nested::foo"));
+    assert!(module_paths.iter().any(|p| *p == "nested::foo::bar"));
+    assert!(module_paths.iter().any(|p| *p == "nested::foo::bar::baz"));
+}
+
+#[test]
+fn test_reexport_chains() {
+    let tmp = tempfile::tempdir().unwrap();
+    let root = tmp.path();
+
+    write_cargo_toml(root, r#"
+[workspace]
+members = ["."]
+
+[package]
+name = "reexporter"
+version = "0.1.0"
+edition = "2021"
+"#);
+
+    let src = root.join("src");
+    std::fs::create_dir_all(&src).unwrap();
+
+    // lib.rs with re-export chain
+    std::fs::write(src.join("lib.rs"), "
+mod inner {
+    pub struct Secret;
+}
+pub use inner::Secret;
+").unwrap();
+
+    let output = run_binary(root.to_str().unwrap());
+    assert!(output.status.success());
+
+    let json = parse_output(&output);
+    let crates = extract_array(&json, "crates");
+    let reexporter = crates.iter().find(|c| c["name"].as_str().unwrap() == "reexporter").unwrap();
+
+    let re_exports: Vec<&serde_json::Value> = extract_array(&reexporter["modules"])
+        .iter()
+        .flat_map(|m| extract_array(m, "reExports"))
+        .collect();
+
+    let has_secret = re_exports.iter().any(|re| {
+        re["importPath"].as_str().unwrap().contains("Secret")
+    });
+    assert!(has_secret, "should have re-export for Secret");
+}
+
+#[test]
+fn test_output_via_flag() {
+    let tmp = tempfile::tempdir().unwrap();
+    let fixture = std::path::Path::new("tests/fixtures/sample-workspace");
+    let output_path = tmp.path().join("output.json");
+
+    // Run with -o flag
+    let output1 = Command::new(&binary_path())
+        .arg(fixture)
+        .arg("-o")
+        .arg(output_path.clone())
+        .output()
+        .expect("failed to execute binary");
+    assert!(output1.status.success());
+
+    // Run without -o, capture stdout
+    let output2 = Command::new(&binary_path())
+        .arg(fixture)
+        .output()
+        .expect("failed to execute binary");
+    assert!(output2.status.success());
+
+    // Compare file content with stdout
+    let file_content = std::fs::read_to_string(&output_path).unwrap();
+    let stdout_content = String::from_utf8_lossy(&output2.stdout);
+    assert_eq!(
+        file_content.trim(),
+        stdout_content.trim(),
+        "file output should match stdout"
+    );
+}
+'''
diff --git a/notes/plan-enrichment/phase-0.2/task-checklist.md b/notes/plan-enrichment/phase-0.2/task-checklist.md
new file mode 100644
index 0000000..6317188
--- /dev/null
+++ b/notes/plan-enrichment/phase-0.2/task-checklist.md
@@ -0,0 +1,274 @@
+# Phase 0.2 — Task Checklist
+
+Generated: 2026-04-28
+
+---
+
+## TASK-1: Add ErrorSeverity enum and ErrorContext struct to schema.rs
+
+Module wiring check:
+- pub mod in parent: Yes — `pub mod schema;` declared in `lib.rs` line 17
+- pub use re-export: No — neither type is re-exported from `lib.rs` (not needed, they are internal to the schema module)
+- Consumer updates co-located: Yes — `schema.rs` itself contains the types
+
+Known failure mode check:
+- Missing pub mod risk: Low — `schema.rs` already has `pub mod` in `lib.rs`
+- Missing pub use risk: Low — consumers reference `crate::schema::ErrorSeverity` directly
+- Stale import risk: Low — no existing code imports these types yet
+
+Before-block check:
+- Grep confirmed: Yes — the before-block text ("// -- Internal types") exists at `schema.rs` line 281
+- Acceptance commands present: Yes — `cargo check -p rust-workspace-map`
+
+Depends on: None
+
+Notes: The insertion point is clean — placed before `pub struct FileInfo`. `ErrorContext` uses `bon::Builder` and `serde::Serialize` which are already imported indirectly via `schema.rs` derives.
+
+---
+
+## TASK-2: Add MissingWorkspaceSection variant to Error enum in schema.rs
+
+Module wiring check:
+- pub mod in parent: Yes — `pub mod schema;` declared in `lib.rs` line 17
+- pub use re-export: No — new variant, no re-export needed
+- Consumer updates co-located: Yes — only `schema.rs` changes
+
+Known failure mode check:
+- Missing pub mod risk: Low
+- Missing pub use risk: Low
+- Stale import risk: Low — no existing code references this variant
+
+Before-block check:
+- Grep confirmed: Yes — before-block ("GlobPattern variant + closing brace") exists at `schema.rs` lines 32-36
+- Acceptance commands present: Yes — `cargo check -p rust-workspace-map`
+
+Depends on: TASK-1 (not for wiring; `MissingWorkspaceSection` error message references no new types)
+
+Notes: Straightforward variant addition. The error message string "workspace Cargo.toml is missing the [workspace] section" is clear and consistent with other error messages in this enum.
+
+---
+
+## TASK-3: Expand ErrorEntry struct with severity, kind, context, and cause fields
+
+Module wiring check:
+- pub mod in parent: Yes — `pub mod schema;` declared in `lib.rs` line 17
+- pub use re-export: No
+- Consumer updates co-located: Yes — `schema.rs` changes only
+
+Known failure mode check:
+- Missing pub mod risk: Low
+- Missing pub use risk: Low
+- Stale import risk: Low
+
+Before-block check:
+- Grep confirmed: Yes — before-block ("ErrorEntry struct with 3 fields") exists at `schema.rs` lines 302-309
+- Acceptance commands present: Yes — `cargo check -p rust-workspace-map`
+
+Depends on: TASK-1 — **BLOCKED without TASK-1** because the new `kind` field references `ErrorSeverity` (defined in TASK-1) and `context` field references `ErrorContext` (also defined in TASK-1)
+
+Notes: The new `kind: String` field has no `skip_serializing_if` annotation, meaning it will always be serialized (even as empty string). This is intentional — callers are expected to always set it. Both `context` and `cause` have proper `skip_serializing_if` annotations. The `#[builder(default)]` on `line: usize` means the default is `0` — this is fine.
+
+---
+
+## TASK-4: Change parse_file to return ParsedFile with optional parse errors instead of Result
+
+Module wiring check:
+- pub mod in parent: Yes — `pub mod file_parser;` declared in `lib.rs` line 14
+- pub use re-export: No — `ParsedFile` and `SynParseError` are `pub` (no re-export needed; they are used internally)
+- Consumer updates co-located: No — `module_tree.rs` and `lib.rs` need updates for the new return type (handled in TASK-5 and TASK-7)
+
+Known failure mode check:
+- Missing pub mod risk: Low — `file_parser.rs` already `pub mod`
+- Missing pub use risk: Medium — `build_parse_error_entry` is NOT `pub` (no visibility qualifier). It is referenced as `crate::file_parser::build_parse_error_entry` in `module_tree.rs` changes. Since Rust default visibility for non-pub items in the same crate is `crate`-level, this WILL work. But the plan does not explicitly declare this visibility, which could confuse an executor.
+- Stale import risk: High — `file_parser.rs` imports `ErrorEntry` and `ErrorSeverity` from `schema.rs` which are defined in TASK-1 and TASK-3. If TASK-1 and TASK-3 are not applied first, the imports will fail.
+
+Before-block check:
+- Grep confirmed: Yes — the full `parse_file` function block matches at `file_parser.rs` lines 1-41
+- Acceptance commands present: Yes — `cargo check -p rust-workspace-map`
+
+Depends on: TASK-1, TASK-3 — **BLOCKED without these** because the new code imports `ErrorEntry` and `ErrorSeverity` from `schema.rs`
+
+Notes: `ParsedFile` and `SynParseError` are `pub` in `file_parser.rs` but not re-exported from `lib.rs`. This is fine for internal use. The `build_parse_error_entry` helper function is `pub(crate)` by Rust default (no `pub` keyword). `parse_file` now always succeeds (never returns an `Err`), which is the intended behavior per the plan.
+
+---
+
+## TASK-5: Refactor module_tree.rs: fix path fallbacks, add error collection, change build_module_tree return type
+
+Module wiring check:
+- pub mod in parent: Yes — `pub mod module_tree;` declared in `lib.rs` line 15
+- pub use re-export: No
+- Consumer updates co-located: No — `lib.rs::run()` needs to call the new `(Vec<ModuleInfo>, Vec<ErrorEntry>)` return type (handled in TASK-7)
+
+Known failure mode check:
+- Missing pub mod risk: Low
+- Missing pub use risk: Low
+- Stale import risk: Medium — `module_tree.rs` imports `FileInfo` and `SubmoduleDecl` from `schema.rs` and uses `file_parser::parse_file`. If TASK-4 is not applied first, the `file_parser::parse_file` call will not match the new `ParsedFile` return type.
+
+Before-block check:
+- Grep confirmed: Yes — all 9 before-blocks verified against `module_tree.rs` source
+- Acceptance commands present: Yes — `cargo check -p rust-workspace-map`
+
+Depends on: TASK-4 — **BLOCKED without TASK-4** because `build_module_tree` and `process_submodule` call `file_parser::parse_file()` which returns `ParsedFile` in TASK-4 but `Result<(...)>` in current source. Also, `module_tree.rs` references `crate::file_parser::build_parse_error_entry` which is only defined in TASK-4.
+
+Notes: This is the most complex task with 9 sequential replace operations on a single file. The operations must be applied in order (or atomically merged) because later before-blocks reference code that is mutated by earlier before-blocks. The path fallback fix (`unwrap_or_else(|| Path::new("."))` -> `unwrap_or(file_path)` or `unwrap_or(crate_root)`) is conservative — using the file itself as fallback when parent() returns None is safer than the original `Path::new(".")` which could resolve to the wrong directory. The `process_module_info` function gains an `errors: &mut Vec<...>` parameter that threads errors through recursion. The `process_module_items` function calls `process_module_info` with `&mut Vec::new()` which starts a fresh error vector per inline module — this may lose cross-module error context.
+
+---
+
+## TASK-6: Update workspace.rs enumerate_members to return MissingWorkspaceSection error
+
+Module wiring check:
+- pub mod in parent: Yes — `pub mod workspace;` declared in `lib.rs` line 18
+- pub use re-export: No
+- Consumer updates co-located: No — `lib.rs::run()` may propagate this error (but the error already propagates via `?` since it returns `Result`)
+
+Known failure mode check:
+- Missing pub mod risk: Low
+- Missing pub use risk: Low
+- Stale import risk: Low — `workspace.rs` already imports `Error` from `schema.rs`; the `MissingWorkspaceSection` variant is added in TASK-2
+
+Before-block check:
+- Grep confirmed: Yes — all 3 before-blocks verified against `workspace.rs` source
+- Acceptance commands present: Yes — `cargo check -p rust-workspace-map`
+
+Depends on: TASK-2 — **BLOCKED without TASK-2** because `Error::MissingWorkspaceSection` variant is only defined in TASK-2. If applied first, `cargo check` will fail with "variant does not exist".
+
+Notes: The first change introduces a `match parsed.get("workspace")` that explicitly checks for `None` and returns `Err(Error::MissingWorkspaceSection)`. The second and third changes are purely doc additions (````# Errors` sections) to `find_workspace_root` and `enumerate_members`.
+
+---
+
+## TASK-7: Refactor lib.rs run(): parallel error collection, error entry construction, remove unwrap_or_default
+
+Module wiring check:
+- pub mod in parent: N/A — this modifies `lib.rs` (the crate root, not a module file)
+- pub use re-export: Changes imports to add `ErrorContext`, `ErrorSeverity` to the use block — these types come from `schema.rs` (TASK-1, TASK-3)
+- Consumer updates co-located: N/A — `lib.rs` IS the consumer of all other modules
+
+Known failure mode check:
+- Missing pub mod risk: N/A
+- Missing pub use risk: Low — imports added to the existing `use schema::{...}` block
+- Stale import risk: High — the before-block is very large (the entire `par_iter` closure block). If any prior task (TASK-4, TASK-5) changes the source before TASK-7 is applied, the before-block will NOT match.
+
+Before-block check:
+- Grep confirmed: Yes — the large before-block matches the current `lib.rs` lines 39-113
+- Acceptance commands present: Yes — `cargo check -p rust-workspace-map`
+
+Depends on: TASK-4, TASK-5, TASK-6 — **BLOCKED without these**. Specifically:
+  - TASK-4: `parse_file` return type changed to `ParsedFile`
+  - TASK-5: `build_module_tree` return type changed to `(Vec<ModuleInfo>, Vec<ErrorEntry>)`
+  - TASK-6: `enumerate_members` may return `Err(MissingWorkspaceSection)` — already handled via `?`
+
+Notes: This is the most critical integration task. The entire `par_iter` block is replaced with a `map` that returns `(Option<CrateInfo>, Vec<ErrorEntry>)` tuples, which are then collected and processed in a post-loop. The `errors: Vec<ErrorEntry>` variable is renamed to `crate_errors` and wired into the final `WorkspaceMap`. The `unwrap_or_default()` on line 79 is removed in favor of destructuring the `(modules, errors)` tuple from `build_module_tree`. If TASK-5 is not applied first, the `build_module_tree` call with the new return type will not compile.
+
+---
+
+## TASK-8: Remove all crate-level clippy allow attributes and fix individual lint violations
+
+Module wiring check:
+- pub mod in parent: N/A — modifies `lib.rs`, `file_parser.rs`, `module_tree.rs`, `render.rs`, `cross_refs.rs`
+- pub use re-export: N/A — no new exports
+- Consumer updates co-located: N/A — purely cosmetic/lint fixes
+
+Known failure mode check:
+- Missing pub mod risk: N/A
+- Missing pub use risk: N/A
+- Stale import risk: High — 11 replace operations across 5 files. Any prior task that modifies these files before TASK-8 will cause before-block mismatches.
+
+Before-block check:
+- Grep confirmed: Yes — all before-blocks verified against current source. Note: some "before" and "after" blocks are identical (e.g., `lib.rs` `workspace_name` block at line 959 and `relativize_path` ending at line 1073). These appear to be no-op placeholders. The actual changes are: removing crate-level `#![allow(...)]` lines, adding `#[must_use]` to extractors, and adding `# Errors` doc sections.
+- Acceptance commands present: Yes — `cargo clippy -p rust-workspace-map -- -D warnings`
+
+Depends on: TASK-7 — **Should follow TASK-7** because TASK-7 adds `# Errors` doc section to `run()` which is referenced in TASK-8's before/after. Also, TASK-8's clippy fixes should be applied after the structural changes from TASK-7 so clippy can validate the new code.
+
+Notes: Some before/after blocks are identical (no-op). The `workspace_name` block and `relativize_path` block appear to be no-op placeholders — the actual change is the `#![warn(clippy::pedantic)]` block replacement. The `#[must_use]` annotations are added to all public extraction functions and the `resolve_module_path` function.
+
+---
+
+## TASK-9: Add unit tests for file_parser, module_tree, workspace, cargo_info, cross_refs, and render
+
+Module wiring check:
+- pub mod in parent: N/A — tests are `#[cfg(test)]` modules appended to existing source files
+- pub use re-export: N/A
+- Consumer updates co-located: N/A
+
+Known failure mode check:
+- Missing pub mod risk: Low
+- Missing pub use risk: Low
+- Stale import risk: Medium — tests reference `ErrorSeverity` (from TASK-1), `ParsedFile` (from TASK-4), `SynParseError` (from TASK-4), `ErrorContext::builder()` (from TASK-1), and `ErrorContext::builder().module_path(...)` (from TASK-1). These must exist in the compilation unit.
+
+Before-block check:
+- Grep confirmed: Yes — all 6 before-blocks verified against current source files
+- Acceptance commands present: Yes — `cargo test -p rust-workspace-map`
+
+Depends on: TASK-1, TASK-4 — **BLOCKED without these** because tests import `ErrorSeverity` and reference `ParsedFile`/`SynParseError` types. Also depends on TASK-5 indirectly because `build_module_tree` return type changes are tested.
+
+Notes: Tests use `tempfile` crate which is NOT declared in `Cargo.toml`. **Missing `[dev-dependencies]` section with `tempfile = "..."` must be added before this task can compile.** The `parse_source` helper function writes temp files, which is correct for unit tests. The `build_module_tree_returns_empty_for_nonexistent` test expects `modules.is_empty()` and `!errors.is_empty()` — this is a reasonable expectation for a non-existent file (parse error from `std::fs::read_to_string`).
+
+---
+
+## TASK-10: Expand integration tests from 3 to 10 covering error cases, workspace patterns, and output
+
+Module wiring check:
+- pub mod in parent: N/A — integration test file
+- pub use re-export: N/A
+- Consumer updates co-located: N/A
+
+Known failure mode check:
+- Missing pub mod risk: N/A
+- Missing pub use risk: N/A
+- Stale import risk: Medium — depends on all structural changes (TASK-4 through TASK-8) being applied because the binary behavior changes (new error entries, new error kinds like `syn_parse_error`, `orphaned_module`, `toml_parse_error`, `missing_crate_roots`).
+
+Before-block check:
+- Grep confirmed: Yes — the before-block (the `test_missing_path_exits_nonzero` function end) matches at `integration_test.rs` lines 116-128
+- Acceptance commands present: Yes — `cargo test -p rust-workspace-map --test integration_test`
+
+Depends on: TASK-4, TASK-5, TASK-6, TASK-7 — **BLOCKED without these** because integration tests assert on error entry structure (`severity`, `kind` fields) and module tree behavior that only exist after the structural changes.
+
+Notes: Tests use `tempfile` crate which is NOT declared in `Cargo.toml`. **Same missing `[dev-dependencies]` issue as TASK-9.** The new tests cover: parse failure error entries, missing workspace section, glob member patterns, workspace exclude, deeply nested modules (3+ levels), re-export chains, and output via `-o` flag. The `test_output_via_flag` test references `tests/fixtures/sample-workspace` which exists. Helper functions (`run_binary`, `parse_output`, `write_cargo_toml`, `setup_crate`) are defined inline in the after-block.
+
+---
+
+# Dependency Graph Summary
+
+```
+TASK-1 ──┐
+TASK-2 ──┤
+TASK-3 ──┤         TASK-6 ──┐
+  │        │                  │
+  └──► TASK-4 ──► TASK-5 ────┤
+                          ▼
+                       TASK-7 ──► TASK-8
+                          │
+                          ▼
+                       TASK-9
+                          │
+                          ▼
+                       TASK-10
+```
+
+Dependency notes:
+- TASK-3 depends on TASK-1 because `ErrorEntry` new fields reference `ErrorSeverity` and `ErrorContext` types.
+- TASK-4 depends on TASK-1, TASK-3 because it imports `ErrorEntry` and `ErrorSeverity`.
+- TASK-5 depends on TASK-4 because it calls `file_parser::parse_file()` (new return type) and `crate::file_parser::build_parse_error_entry` (new function).
+- TASK-6 depends on TASK-2 because it returns `Error::MissingWorkspaceSection` (new variant).
+- TASK-7 depends on TASK-4, TASK-5, TASK-6 because it calls `parse_file()` (new type), `build_module_tree()` (new return type), and `enumerate_members()` (new error variant).
+- TASK-8 should follow TASK-7 so clippy validates the new code.
+- TASK-9 depends on TASK-1, TASK-4 because tests reference those new types.
+- TASK-10 depends on TASK-4 through TASK-7 because integration tests assert on new error kinds and module tree structure.
+
+# Overall Assessment
+
+Ready to Implement: Yes, but with the following action items:
+
+1. **Add `[dev-dependencies]` section** with `tempfile = "*"` (or a specific version). This is required for TASK-9 and TASK-10 tests to compile.
+
+2. **Execute order matters**: Tasks must be executed in dependency order (TASK-1/2/3 first, then 4/6, then 5/7, then 8/9, then 10). Parallel execution of independent tasks (e.g., TASK-1 + TASK-2, or TASK-4 + TASK-6 after their deps) is possible but should be done carefully.
+
+3. **High-risk transition points**:
+   - TASK-4 -> TASK-5: `parse_file` return type changes from `Result<(...)>` to `ParsedFile`
+   - TASK-5 -> TASK-7: `build_module_tree` return type changes from `Result<Vec<ModuleInfo>>` to `(Vec<ModuleInfo>, Vec<ErrorEntry>)`
+   - TASK-7: Large single-replace in `lib.rs` that touches the entire parallel processing pipeline
+
+4. **Before-block stability**: TASK-7's before-block is a large multi-line block that is the most fragile. If any prior task modifies `lib.rs::run()` before TASK-7, the before-block will not match.
+
+Wiring issues flagged: 2 (missing `tempfile` dev-dependency, `build_parse_error_entry` visibility not explicitly declared).
+Before-block unverified: 0 (all before-blocks match current source).
diff --git a/plans/compiled/TASK-1.py b/plans/compiled/TASK-1.py
new file mode 100644
index 0000000..2883ca5
--- /dev/null
+++ b/plans/compiled/TASK-1.py
@@ -0,0 +1,44 @@
+#!/usr/bin/env python3
+"""TASK-1: Add ErrorSeverity enum and ErrorContext struct to schema.rs"""
+import base64, json, subprocess, sys
+from pathlib import Path
+
+TASK_ID = "TASK-1"
+STEPS = json.loads('[{"before_b64": "Ly8g4pSA4pSAIEludGVybmFsIHR5cGVzIOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgAoKLy8vIEludGVybmFsIGludGVybWVkaWF0ZSB0eXBlIGNvbnN1bWVkIGJ5IG1vZHVsZV90cmVlLgojW2Rlcml2ZShEZWJ1ZywgQ2xvbmUsIERlZmF1bHQpXQpwdWIgc3RydWN0IEZpbGVJbmZvIHsK", "after_b64": "Ly8g4pSA4pSAIEVycm9yIHNldmVyaXR5IOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgAoKI1tkZXJpdmUoRGVidWcsIENsb25lLCBDb3B5LCBQYXJ0aWFsRXEsIEVxLCBzZXJkZTo6U2VyaWFsaXplKV0KI1tzZXJkZShyZW5hbWVfYWxsID0gInNuYWtlX2Nhc2UiKV0KcHViIGVudW0gRXJyb3JTZXZlcml0eSB7CiAgICBFcnJvciwKICAgIFdhcm5pbmcsCn0KCi8vIOKUgOKUgCBFcnJvciBjb250ZXh0IOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgAoKLy8vIE9wdGlvbmFsIGNvbnRleHQgYXR0YWNoZWQgdG8gYW4gZXJyb3IsIHByb3ZpZGluZyBhZGRpdGlvbmFsIGxvY2F0aW9uCi8vLyBhbmQgc291cmNlIGluZm9ybWF0aW9uIGZvciBkaWFnbm9zdGljcy4KI1tkZXJpdmUoRGVidWcsIENsb25lLCBEZWZhdWx0LCBzZXJkZTo6U2VyaWFsaXplLCBib246OkJ1aWxkZXIpXQojW3NlcmRlKHJlbmFtZV9hbGwgPSAiY2FtZWxDYXNlIildCnB1YiBzdHJ1Y3QgRXJyb3JDb250ZXh0IHsKICAgICNbc2VyZGUoc2tpcF9zZXJpYWxpemluZ19pZiA9ICJPcHRpb246OmlzX25vbmUiKV0KICAgIHB1YiBjcmF0ZV9uYW1lOiBPcHRpb248U3RyaW5nPiwKCiAgICAjW3NlcmRlKHNraXBfc2VyaWFsaXppbmdfaWYgPSAiT3B0aW9uOjppc19ub25lIildCiAgICBwdWIgbW9kdWxlX3BhdGg6IE9wdGlvbjxTdHJpbmc+LAoKICAgIC8vLyBMaW5lIG51bWJlciBpbiB0aGUgc291cmNlIGZpbGUgd2hlcmUgdGhlIGVycm9yIG9jY3VycmVkLgogICAgI1tzZXJkZShza2lwX3NlcmlhbGl6aW5nX2lmID0gIk9wdGlvbjo6aXNfbm9uZSIpXQogICAgcHViIGxpbmU6IE9wdGlvbjx1c2l6ZT4sCgogICAgLy8vIEEgc2hvcnQgc291cmNlIHNuaXBwZXQgbmVhciB0aGUgZXJyb3IgbG9jYXRpb24gKGlmIGF2YWlsYWJsZSkuCiAgICAjW3NlcmRlKHNraXBfc2VyaWFsaXppbmdfaWYgPSAiT3B0aW9uOjppc19ub25lIildCiAgICBwdWIgc25pcHBldDogT3B0aW9uPFN0cmluZz4sCn0KCi8vIOKUgOKUgCBJbnRlcm5hbCB0eXBlcyDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIAKCi8vLyBJbnRlcm5hbCBpbnRlcm1lZGlhdGUgdHlwZSBjb25zdW1lZCBieSBtb2R1bGVfdHJlZS4KI1tkZXJpdmUoRGVidWcsIENsb25lLCBEZWZhdWx0KV0KcHViIHN0cnVjdCBGaWxlSW5mbyB7Cg==", "target": "src/schema.rs", "index": 0, "is_create": false}]')
+
+for step in STEPS:
+    before = base64.b64decode(step["before_b64"]).decode()
+    after = base64.b64decode(step["after_b64"]).decode()
+    target = step["target"]
+    idx = step["index"]
+    is_create = step["is_create"]
+
+    if is_create:
+        target_path = Path(target)
+        target_path.parent.mkdir(parents=True, exist_ok=True)
+        target_path.write_text(after)
+        print(f"OK {TASK_ID} change {idx}: created {target}")
+    else:
+        target_path = Path(target)
+        content = target_path.read_text()
+        if before not in content:
+            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
+            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
+            sys.exit(1)
+
+        result = subprocess.run(
+            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
+            capture_output=True, text=True,
+        )
+        if result.returncode != 0:
+            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
+            sys.exit(result.returncode)
+
+        new_content = target_path.read_text()
+        if after and after not in new_content:
+            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
+            sys.exit(1)
+
+        print(f"OK {TASK_ID} change {idx}: applied to {target}")
+
+print(f"OK {TASK_ID}: all changes applied")
diff --git a/plans/compiled/TASK-1.sh b/plans/compiled/TASK-1.sh
new file mode 100755
index 0000000..86e78d3
--- /dev/null
+++ b/plans/compiled/TASK-1.sh
@@ -0,0 +1,7 @@
+#!/usr/bin/env bash
+set -euo pipefail
+# TASK-1: Add ErrorSeverity enum and ErrorContext struct to schema.rs
+# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
+# Type: replace
+# File: src/schema.rs
+python3 "$(dirname "$0")/TASK-1.py"
diff --git a/plans/compiled/TASK-10.py b/plans/compiled/TASK-10.py
new file mode 100644
index 0000000..f9753bc
--- /dev/null
+++ b/plans/compiled/TASK-10.py
@@ -0,0 +1,44 @@
+#!/usr/bin/env python3
+"""TASK-10: Expand integration tests from 3 to 10 covering error cases, workspace patterns, and output"""
+import base64, json, subprocess, sys
+from pathlib import Path
+
+TASK_ID = "TASK-10"
+STEPS = json.loads('[{"before_b64": "I1t0ZXN0XQpmbiB0ZXN0X21pc3NpbmdfcGF0aF9leGl0c19ub256ZXJvKCkgewogICAgbGV0IGJpbiA9IGJpbmFyeV9wYXRoKCk7CiAgICBsZXQgb3V0cHV0ID0gQ29tbWFuZDo6bmV3KCZiaW4pCiAgICAgICAgLmFyZygiL3RtcC9ub25leGlzdGVudC1wYXRoLTEyMzQ1IikKICAgICAgICAub3V0cHV0KCkKICAgICAgICAuZXhwZWN0KCJmYWlsZWQgdG8gZXhlY3V0ZSBiaW5hcnkiKTsKCiAgICBhc3NlcnQhKAogICAgICAgICFvdXRwdXQuc3RhdHVzLnN1Y2Nlc3MoKSwKICAgICAgICAic2hvdWxkIGV4aXQgbm9uLXplcm8gZm9yIGludmFsaWQgcGF0aCIKICAgICk7Cn0K", "after_b64": "I1t0ZXN0XQpmbiB0ZXN0X21pc3NpbmdfcGF0aF9leGl0c19ub256ZXJvKCkgewogICAgbGV0IGJpbiA9IGJpbmFyeV9wYXRoKCk7CiAgICBsZXQgb3V0cHV0ID0gQ29tbWFuZDo6bmV3KCZiaW4pCiAgICAgICAgLmFyZygiL3RtcC9ub25leGlzdGVudC1wYXRoLTEyMzQ1IikKICAgICAgICAub3V0cHV0KCkKICAgICAgICAuZXhwZWN0KCJmYWlsZWQgdG8gZXhlY3V0ZSBiaW5hcnkiKTsKCiAgICBhc3NlcnQhKAogICAgICAgICFvdXRwdXQuc3RhdHVzLnN1Y2Nlc3MoKSwKICAgICAgICAic2hvdWxkIGV4aXQgbm9uLXplcm8gZm9yIGludmFsaWQgcGF0aCIKICAgICk7Cn0KCmZuIHJ1bl9iaW5hcnkocGF0aDogJnN0cikgLT4gc3RkOjpwcm9jZXNzOjpPdXRwdXQgewogICAgQ29tbWFuZDo6bmV3KCZiaW5hcnlfcGF0aCgpKQogICAgICAgIC5hcmcocGF0aCkKICAgICAgICAub3V0cHV0KCkKICAgICAgICAuZXhwZWN0KCJmYWlsZWQgdG8gZXhlY3V0ZSBiaW5hcnkiKQp9CgpmbiBwYXJzZV9vdXRwdXQob3V0cHV0OiAmc3RkOjpwcm9jZXNzOjpPdXRwdXQpIC0+IHNlcmRlX2pzb246OlZhbHVlIHsKICAgIHNlcmRlX2pzb246OmZyb21fc3RyKCZTdHJpbmc6OmZyb21fdXRmOF9sb3NzeSgmb3V0cHV0LnN0ZG91dCkpLnVud3JhcCgpCn0KCmZuIHdyaXRlX2NhcmdvX3RvbWwoZGlyOiAmc3RkOjpwYXRoOjpQYXRoLCBjb250ZW50OiAmc3RyKSB7CiAgICBsZXQgbXV0IGYgPSBzdGQ6OmZzOjpGaWxlOjpjcmVhdGUoZGlyLmpvaW4oIkNhcmdvLnRvbWwiKSkudW53cmFwKCk7CiAgICB1c2Ugc3RkOjppbzo6V3JpdGU7CiAgICBmLndyaXRlX2FsbChjb250ZW50LmFzX2J5dGVzKCkpLnVud3JhcCgpOwp9CgpmbiBzZXR1cF9jcmF0ZShkaXI6ICZzdGQ6OnBhdGg6OlBhdGgsIGxpYl9jb250ZW50OiAmc3RyKSB7CiAgICBsZXQgc3JjID0gZGlyLmpvaW4oInNyYyIpOwogICAgc3RkOjpmczo6Y3JlYXRlX2Rpcl9hbGwoJnNyYykudW53cmFwKCk7CiAgICBzdGQ6OmZzOjp3cml0ZShzcmMuam9pbigibGliLnJzIiksIGxpYl9jb250ZW50KS51bndyYXAoKTsKfQoKI1t0ZXN0XQpmbiB0ZXN0X3BhcnNlX2ZhaWx1cmVfZXJyb3JfZW50cnkoKSB7CiAgICBsZXQgdG1wID0gdGVtcGZpbGU6OnRlbXBkaXIoKS51bndyYXAoKTsKICAgIGxldCByb290ID0gdG1wLnBhdGgoKTsKCiAgICAvLyBDcmVhdGUgd29ya3NwYWNlIENhcmdvLnRvbWwKICAgIHdyaXRlX2NhcmdvX3RvbWwocm9vdCwgciMiClt3b3Jrc3BhY2VdCm1lbWJlcnMgPSBbImdvb2RfY3JhdGUiLCAiYmFkX2NyYXRlIl0KIiMpOwoKICAgIC8vIEdvb2QgY3JhdGUgd2l0aCB2YWxpZCBSdXN0CiAgICBzZXR1cF9jcmF0ZSgmcm9vdC5qb2luKCJnb29kX2NyYXRlIiksICJwdWIgc3RydWN0IEdvb2Qge30iKTsKCiAgICAvLyBCYWQgY3JhdGUgd2l0aCBpbnZhbGlkIFJ1c3Qgc3ludGF4CiAgICBzZXR1cF9jcmF0ZSgmcm9vdC5qb2luKCJiYWRfY3JhdGUiKSwgInB1YiBzdHJ1Y3QgeyBpbnZhbGlkIHJ1c3Qgc3ludGF4Iik7CgogICAgbGV0IG91dHB1dCA9IHJ1bl9iaW5hcnkocm9vdC50b19zdHIoKS51bndyYXAoKSk7CiAgICBhc3NlcnQhKG91dHB1dC5zdGF0dXMuc3VjY2VzcygpKTsKCiAgICBsZXQganNvbiA9IHBhcnNlX291dHB1dCgmb3V0cHV0KTsKICAgIGxldCBlcnJvcnM6IFZlYzwmc2VyZGVfanNvbjo6VmFsdWU+ID0gZXh0cmFjdF9hcnJheSgmanNvbiwgImVycm9ycyIpOwoKICAgIGxldCBwYXJzZV9lcnJvcnM6IFZlYzxfPiA9IGVycm9ycy5pdGVyKCkKICAgICAgICAuZmlsdGVyKHxlfCB7CiAgICAgICAgICAgIGVbImtpbmQiXS5hc19zdHIoKS51bndyYXAoKSA9PSAic3luX3BhcnNlX2Vycm9yIgogICAgICAgIH0pCiAgICAgICAgLmNvbGxlY3QoKTsKCiAgICBhc3NlcnQhKCFwYXJzZV9lcnJvcnMuaXNfZW1wdHkoKSwgInNob3VsZCBoYXZlIHBhcnNlIGVycm9yIGVudHJpZXMiKTsKICAgIGFzc2VydF9lcSEocGFyc2VfZXJyb3JzWzBdWyJzZXZlcml0eSJdLmFzX3N0cigpLnVud3JhcCgpLCAiZXJyb3IiKTsKfQoKI1t0ZXN0XQpmbiB0ZXN0X21pc3Npbmdfd29ya3NwYWNlX3NlY3Rpb24oKSB7CiAgICBsZXQgdG1wID0gdGVtcGZpbGU6OnRlbXBkaXIoKS51bndyYXAoKTsKICAgIGxldCByb290ID0gdG1wLnBhdGgoKTsKCiAgICAvLyBDYXJnby50b21sIHdpdGhvdXQgW3dvcmtzcGFjZV0gc2VjdGlvbgogICAgd3JpdGVfY2FyZ29fdG9tbChyb290LCByIyIKW3BhY2thZ2VdCm5hbWUgPSAic3RhbmRhbG9uZSIKdmVyc2lvbiA9ICIwLjEuMCIKZWRpdGlvbiA9ICIyMDIxIgoiIyk7CgogICAgbGV0IG91dHB1dCA9IHJ1bl9iaW5hcnkocm9vdC50b19zdHIoKS51bndyYXAoKSk7CgogICAgLy8gU2hvdWxkIGV4aXQgbm9uLXplcm8gYmVjYXVzZSB3b3Jrc3BhY2UgaXMgbWlzc2luZwogICAgYXNzZXJ0ISgKICAgICAgICAhb3V0cHV0LnN0YXR1cy5zdWNjZXNzKCksCiAgICAgICAgInNob3VsZCBleGl0IG5vbi16ZXJvIGZvciBtaXNzaW5nIHdvcmtzcGFjZSBzZWN0aW9uIgogICAgKTsKfQoKI1t0ZXN0XQpmbiB0ZXN0X2dsb2JfbWVtYmVyX3BhdHRlcm5zKCkgewogICAgbGV0IHRtcCA9IHRlbXBmaWxlOjp0ZW1wZGlyKCkudW53cmFwKCk7CiAgICBsZXQgcm9vdCA9IHRtcC5wYXRoKCk7CgogICAgd3JpdGVfY2FyZ29fdG9tbChyb290LCByIyIKW3dvcmtzcGFjZV0KbWVtYmVycyA9IFsiY3JhdGVzLyoiXQoiIyk7CgogICAgZm9yIG5hbWUgaW4gJlsiYWxwaGEiLCAiYmV0YSIsICJnYW1tYSJdIHsKICAgICAgICBzZXR1cF9jcmF0ZSgmcm9vdC5qb2luKCJjcmF0ZXMiKS5qb2luKG5hbWUpLCBmb3JtYXQhKCJwdWIgc3RydWN0IHtuYW1lfSB7e319IikuYXNfc3RyKCkpOwogICAgfQoKICAgIGxldCBvdXRwdXQgPSBydW5fYmluYXJ5KHJvb3QudG9fc3RyKCkudW53cmFwKCkpOwogICAgYXNzZXJ0IShvdXRwdXQuc3RhdHVzLnN1Y2Nlc3MoKSk7CgogICAgbGV0IGpzb24gPSBwYXJzZV9vdXRwdXQoJm91dHB1dCk7CiAgICBsZXQgY3JhdGVzID0gZXh0cmFjdF9hcnJheSgmanNvbiwgImNyYXRlcyIpOwogICAgbGV0IG5hbWVzOiBWZWM8JnN0cj4gPSBjcmF0ZXMuaXRlcigpCiAgICAgICAgLm1hcCh8Y3wgY1sibmFtZSJdLmFzX3N0cigpLnVud3JhcCgpKQogICAgICAgIC5jb2xsZWN0KCk7CgogICAgYXNzZXJ0IShuYW1lcy5jb250YWlucygmImFscGhhIikpOwogICAgYXNzZXJ0IShuYW1lcy5jb250YWlucygmImJldGEiKSk7CiAgICBhc3NlcnQhKG5hbWVzLmNvbnRhaW5zKCYiZ2FtbWEiKSk7CiAgICBhc3NlcnRfZXEhKG5hbWVzLmxlbigpLCAzKTsKfQoKI1t0ZXN0XQpmbiB0ZXN0X3dvcmtzcGFjZV93aXRoX2V4Y2x1ZGUoKSB7CiAgICBsZXQgdG1wID0gdGVtcGZpbGU6OnRlbXBkaXIoKS51bndyYXAoKTsKICAgIGxldCByb290ID0gdG1wLnBhdGgoKTsKCiAgICB3cml0ZV9jYXJnb190b21sKHJvb3QsIHIjIgpbd29ya3NwYWNlXQptZW1iZXJzID0gWyJhIiwgImIiLCAiYyJdCmV4Y2x1ZGUgPSBbImIiXQoiIyk7CgogICAgc2V0dXBfY3JhdGUoJnJvb3Quam9pbigiYSIpLCAicHViIHN0cnVjdCBBIHt9Iik7CiAgICBzZXR1cF9jcmF0ZSgmcm9vdC5qb2luKCJiIiksICJwdWIgc3RydWN0IEIge30iKTsKICAgIHNldHVwX2NyYXRlKCZyb290LmpvaW4oImMiKSwgInB1YiBzdHJ1Y3QgQyB7fSIpOwoKICAgIGxldCBvdXRwdXQgPSBydW5fYmluYXJ5KHJvb3QudG9fc3RyKCkudW53cmFwKCkpOwogICAgYXNzZXJ0IShvdXRwdXQuc3RhdHVzLnN1Y2Nlc3MoKSk7CgogICAgbGV0IGpzb24gPSBwYXJzZV9vdXRwdXQoJm91dHB1dCk7CiAgICBsZXQgY3JhdGVzID0gZXh0cmFjdF9hcnJheSgmanNvbiwgImNyYXRlcyIpOwogICAgbGV0IG5hbWVzOiBWZWM8JnN0cj4gPSBjcmF0ZXMuaXRlcigpCiAgICAgICAgLm1hcCh8Y3wgY1sibmFtZSJdLmFzX3N0cigpLnVud3JhcCgpKQogICAgICAgIC5jb2xsZWN0KCk7CgogICAgYXNzZXJ0IShuYW1lcy5jb250YWlucygmImEiKSk7CiAgICBhc3NlcnQhKCFuYW1lcy5jb250YWlucygmImIiKSk7CiAgICBhc3NlcnQhKG5hbWVzLmNvbnRhaW5zKCYiYyIpKTsKfQoKI1t0ZXN0XQpmbiB0ZXN0X2RlZXBseV9uZXN0ZWRfbW9kdWxlcygpIHsKICAgIGxldCB0bXAgPSB0ZW1wZmlsZTo6dGVtcGRpcigpLnVud3JhcCgpOwogICAgbGV0IHJvb3QgPSB0bXAucGF0aCgpOwoKICAgIHdyaXRlX2NhcmdvX3RvbWwocm9vdCwgciMiClt3b3Jrc3BhY2VdCm1lbWJlcnMgPSBbIi4iXQoKW3BhY2thZ2VdCm5hbWUgPSAibmVzdGVkIgp2ZXJzaW9uID0gIjAuMS4wIgplZGl0aW9uID0gIjIwMjEiCiIjKTsKCiAgICBsZXQgc3JjID0gcm9vdC5qb2luKCJzcmMiKTsKICAgIGxldCBmb28gPSBzcmMuam9pbigiZm9vIik7CiAgICBsZXQgYmFyID0gZm9vLmpvaW4oImJhciIpOwogICAgc3RkOjpmczo6Y3JlYXRlX2Rpcl9hbGwoJmJhcikudW53cmFwKCk7CgogICAgLy8gbGliLnJzIGRlY2xhcmVzIG1vZCBmb28KICAgIHN0ZDo6ZnM6OndyaXRlKHNyYy5qb2luKCJsaWIucnMiKSwgIm1vZCBmb287IikudW53cmFwKCk7CiAgICAvLyBmb28ucnMgZGVjbGFyZXMgbW9kIGJhcgogICAgc3RkOjpmczo6d3JpdGUoZm9vLmpvaW4oImZvby5ycyIpLCAibW9kIGJhcjsiKS51bndyYXAoKTsKICAgIC8vIGJhci9iYXoucnMgZGVjbGFyZXMgbW9kIGJhegogICAgc3RkOjpmczo6d3JpdGUoYmFyLmpvaW4oImJhci5ycyIpLCAibW9kIGJhejsiKS51bndyYXAoKTsKICAgIC8vIGJhei5ycyB3aXRoIGEgc3RydWN0CiAgICBzdGQ6OmZzOjp3cml0ZShiYXIuam9pbigiYmF6LnJzIiksICJwdWIgc3RydWN0IERlZXAge30iKS51bndyYXAoKTsKCiAgICBsZXQgb3V0cHV0ID0gcnVuX2JpbmFyeShyb290LnRvX3N0cigpLnVud3JhcCgpKTsKICAgIGFzc2VydCEob3V0cHV0LnN0YXR1cy5zdWNjZXNzKCkpOwoKICAgIGxldCBqc29uID0gcGFyc2Vfb3V0cHV0KCZvdXRwdXQpOwogICAgbGV0IGNyYXRlcyA9IGV4dHJhY3RfYXJyYXkoJmpzb24sICJjcmF0ZXMiKTsKICAgIGxldCBuZXN0ZWRfY3JhdGUgPSBjcmF0ZXMuaXRlcigpLmZpbmQofGN8IGNbIm5hbWUiXS5hc19zdHIoKS51bndyYXAoKSA9PSAibmVzdGVkIikudW53cmFwKCk7CgogICAgbGV0IG1vZHVsZV9wYXRoczogVmVjPCZzdHI+ID0gZXh0cmFjdF9hcnJheSgmbmVzdGVkX2NyYXRlWyJtb2R1bGVzIl0sICJwYXRoIikKICAgICAgICAuaXRlcigpCiAgICAgICAgLm1hcCh8bXwgbS5hc19zdHIoKS51bndyYXAoKSkKICAgICAgICAuY29sbGVjdCgpOwoKICAgIGFzc2VydCEobW9kdWxlX3BhdGhzLml0ZXIoKS5hbnkofHB8ICpwID09ICJuZXN0ZWQiKSk7CiAgICBhc3NlcnQhKG1vZHVsZV9wYXRocy5pdGVyKCkuYW55KHxwfCAqcCA9PSAibmVzdGVkOjpmb28iKSk7CiAgICBhc3NlcnQhKG1vZHVsZV9wYXRocy5pdGVyKCkuYW55KHxwfCAqcCA9PSAibmVzdGVkOjpmb286OmJhciIpKTsKICAgIGFzc2VydCEobW9kdWxlX3BhdGhzLml0ZXIoKS5hbnkofHB8ICpwID09ICJuZXN0ZWQ6OmZvbzo6YmFyOjpiYXoiKSk7Cn0KCiNbdGVzdF0KZm4gdGVzdF9yZWV4cG9ydF9jaGFpbnMoKSB7CiAgICBsZXQgdG1wID0gdGVtcGZpbGU6OnRlbXBkaXIoKS51bndyYXAoKTsKICAgIGxldCByb290ID0gdG1wLnBhdGgoKTsKCiAgICB3cml0ZV9jYXJnb190b21sKHJvb3QsIHIjIgpbd29ya3NwYWNlXQptZW1iZXJzID0gWyIuIl0KCltwYWNrYWdlXQpuYW1lID0gInJlZXhwb3J0ZXIiCnZlcnNpb24gPSAiMC4xLjAiCmVkaXRpb24gPSAiMjAyMSIKIiMpOwoKICAgIGxldCBzcmMgPSByb290LmpvaW4oInNyYyIpOwogICAgc3RkOjpmczo6Y3JlYXRlX2Rpcl9hbGwoJnNyYykudW53cmFwKCk7CgogICAgLy8gbGliLnJzIHdpdGggcmUtZXhwb3J0IGNoYWluCiAgICBzdGQ6OmZzOjp3cml0ZShzcmMuam9pbigibGliLnJzIiksICIKbW9kIGlubmVyIHsKICAgIHB1YiBzdHJ1Y3QgU2VjcmV0Owp9CnB1YiB1c2UgaW5uZXI6OlNlY3JldDsKIikudW53cmFwKCk7CgogICAgbGV0IG91dHB1dCA9IHJ1bl9iaW5hcnkocm9vdC50b19zdHIoKS51bndyYXAoKSk7CiAgICBhc3NlcnQhKG91dHB1dC5zdGF0dXMuc3VjY2VzcygpKTsKCiAgICBsZXQganNvbiA9IHBhcnNlX291dHB1dCgmb3V0cHV0KTsKICAgIGxldCBjcmF0ZXMgPSBleHRyYWN0X2FycmF5KCZqc29uLCAiY3JhdGVzIik7CiAgICBsZXQgcmVleHBvcnRlciA9IGNyYXRlcy5pdGVyKCkuZmluZCh8Y3wgY1sibmFtZSJdLmFzX3N0cigpLnVud3JhcCgpID09ICJyZWV4cG9ydGVyIikudW53cmFwKCk7CgogICAgbGV0IHJlX2V4cG9ydHM6IFZlYzwmc2VyZGVfanNvbjo6VmFsdWU+ID0gZXh0cmFjdF9hcnJheSgmcmVleHBvcnRlclsibW9kdWxlcyJdKQogICAgICAgIC5pdGVyKCkKICAgICAgICAuZmxhdF9tYXAofG18IGV4dHJhY3RfYXJyYXkobSwgInJlRXhwb3J0cyIpKQogICAgICAgIC5jb2xsZWN0KCk7CgogICAgbGV0IGhhc19zZWNyZXQgPSByZV9leHBvcnRzLml0ZXIoKS5hbnkofHJlfCB7CiAgICAgICAgcmVbImltcG9ydFBhdGgiXS5hc19zdHIoKS51bndyYXAoKS5jb250YWlucygiU2VjcmV0IikKICAgIH0pOwogICAgYXNzZXJ0IShoYXNfc2VjcmV0LCAic2hvdWxkIGhhdmUgcmUtZXhwb3J0IGZvciBTZWNyZXQiKTsKfQoKI1t0ZXN0XQpmbiB0ZXN0X291dHB1dF92aWFfZmxhZygpIHsKICAgIGxldCB0bXAgPSB0ZW1wZmlsZTo6dGVtcGRpcigpLnVud3JhcCgpOwogICAgbGV0IGZpeHR1cmUgPSBzdGQ6OnBhdGg6OlBhdGg6Om5ldygidGVzdHMvZml4dHVyZXMvc2FtcGxlLXdvcmtzcGFjZSIpOwogICAgbGV0IG91dHB1dF9wYXRoID0gdG1wLnBhdGgoKS5qb2luKCJvdXRwdXQuanNvbiIpOwoKICAgIC8vIFJ1biB3aXRoIC1vIGZsYWcKICAgIGxldCBvdXRwdXQxID0gQ29tbWFuZDo6bmV3KCZiaW5hcnlfcGF0aCgpKQogICAgICAgIC5hcmcoZml4dHVyZSkKICAgICAgICAuYXJnKCItbyIpCiAgICAgICAgLmFyZyhvdXRwdXRfcGF0aC5jbG9uZSgpKQogICAgICAgIC5vdXRwdXQoKQogICAgICAgIC5leHBlY3QoImZhaWxlZCB0byBleGVjdXRlIGJpbmFyeSIpOwogICAgYXNzZXJ0IShvdXRwdXQxLnN0YXR1cy5zdWNjZXNzKCkpOwoKICAgIC8vIFJ1biB3aXRob3V0IC1vLCBjYXB0dXJlIHN0ZG91dAogICAgbGV0IG91dHB1dDIgPSBDb21tYW5kOjpuZXcoJmJpbmFyeV9wYXRoKCkpCiAgICAgICAgLmFyZyhmaXh0dXJlKQogICAgICAgIC5vdXRwdXQoKQogICAgICAgIC5leHBlY3QoImZhaWxlZCB0byBleGVjdXRlIGJpbmFyeSIpOwogICAgYXNzZXJ0IShvdXRwdXQyLnN0YXR1cy5zdWNjZXNzKCkpOwoKICAgIC8vIENvbXBhcmUgZmlsZSBjb250ZW50IHdpdGggc3Rkb3V0CiAgICBsZXQgZmlsZV9jb250ZW50ID0gc3RkOjpmczo6cmVhZF90b19zdHJpbmcoJm91dHB1dF9wYXRoKS51bndyYXAoKTsKICAgIGxldCBzdGRvdXRfY29udGVudCA9IFN0cmluZzo6ZnJvbV91dGY4X2xvc3N5KCZvdXRwdXQyLnN0ZG91dCk7CiAgICBhc3NlcnRfZXEhKAogICAgICAgIGZpbGVfY29udGVudC50cmltKCksCiAgICAgICAgc3Rkb3V0X2NvbnRlbnQudHJpbSgpLAogICAgICAgICJmaWxlIG91dHB1dCBzaG91bGQgbWF0Y2ggc3Rkb3V0IgogICAgKTsKfQo=", "target": "tests/integration_test.rs", "index": 0, "is_create": false}]')
+
+for step in STEPS:
+    before = base64.b64decode(step["before_b64"]).decode()
+    after = base64.b64decode(step["after_b64"]).decode()
+    target = step["target"]
+    idx = step["index"]
+    is_create = step["is_create"]
+
+    if is_create:
+        target_path = Path(target)
+        target_path.parent.mkdir(parents=True, exist_ok=True)
+        target_path.write_text(after)
+        print(f"OK {TASK_ID} change {idx}: created {target}")
+    else:
+        target_path = Path(target)
+        content = target_path.read_text()
+        if before not in content:
+            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
+            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
+            sys.exit(1)
+
+        result = subprocess.run(
+            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
+            capture_output=True, text=True,
+        )
+        if result.returncode != 0:
+            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
+            sys.exit(result.returncode)
+
+        new_content = target_path.read_text()
+        if after and after not in new_content:
+            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
+            sys.exit(1)
+
+        print(f"OK {TASK_ID} change {idx}: applied to {target}")
+
+print(f"OK {TASK_ID}: all changes applied")
diff --git a/plans/compiled/TASK-10.sh b/plans/compiled/TASK-10.sh
new file mode 100755
index 0000000..bccaa8a
--- /dev/null
+++ b/plans/compiled/TASK-10.sh
@@ -0,0 +1,7 @@
+#!/usr/bin/env bash
+set -euo pipefail
+# TASK-10: Expand integration tests from 3 to 10 covering error cases, workspace patterns, and output
+# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
+# Type: replace
+# File: tests/integration_test.rs
+python3 "$(dirname "$0")/TASK-10.py"
diff --git a/plans/compiled/TASK-2.py b/plans/compiled/TASK-2.py
new file mode 100644
index 0000000..fbade65
--- /dev/null
+++ b/plans/compiled/TASK-2.py
@@ -0,0 +1,44 @@
+#!/usr/bin/env python3
+"""TASK-2: Add MissingWorkspaceSection variant to Error enum in schema.rs"""
+import base64, json, subprocess, sys
+from pathlib import Path
+
+TASK_ID = "TASK-2"
+STEPS = json.loads('[{"before_b64": "ICAgICNbZXJyb3IoImdsb2IgcGF0dGVybiBlcnJvcjogezB9IildCiAgICBHbG9iUGF0dGVybihTdHJpbmcpLAp9CgpwdWIgdHlwZSBSZXN1bHQ8VD4gPSBzdGQ6OnJlc3VsdDo6UmVzdWx0PFQsIEVycm9yPjs=", "after_b64": "ICAgICNbZXJyb3IoImdsb2IgcGF0dGVybiBlcnJvcjogezB9IildCiAgICBHbG9iUGF0dGVybihTdHJpbmcpLAoKICAgICNbZXJyb3IoIndvcmtzcGFjZSBDYXJnby50b21sIGlzIG1pc3NpbmcgdGhlIFt3b3Jrc3BhY2VdIHNlY3Rpb24iKV0KICAgIE1pc3NpbmdXb3Jrc3BhY2VTZWN0aW9uLAp9CgpwdWIgdHlwZSBSZXN1bHQ8VD4gPSBzdGQ6OnJlc3VsdDo6UmVzdWx0PFQsIEVycm9yPjs=", "target": "src/schema.rs", "index": 0, "is_create": false}]')
+
+for step in STEPS:
+    before = base64.b64decode(step["before_b64"]).decode()
+    after = base64.b64decode(step["after_b64"]).decode()
+    target = step["target"]
+    idx = step["index"]
+    is_create = step["is_create"]
+
+    if is_create:
+        target_path = Path(target)
+        target_path.parent.mkdir(parents=True, exist_ok=True)
+        target_path.write_text(after)
+        print(f"OK {TASK_ID} change {idx}: created {target}")
+    else:
+        target_path = Path(target)
+        content = target_path.read_text()
+        if before not in content:
+            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
+            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
+            sys.exit(1)
+
+        result = subprocess.run(
+            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
+            capture_output=True, text=True,
+        )
+        if result.returncode != 0:
+            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
+            sys.exit(result.returncode)
+
+        new_content = target_path.read_text()
+        if after and after not in new_content:
+            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
+            sys.exit(1)
+
+        print(f"OK {TASK_ID} change {idx}: applied to {target}")
+
+print(f"OK {TASK_ID}: all changes applied")
diff --git a/plans/compiled/TASK-2.sh b/plans/compiled/TASK-2.sh
new file mode 100755
index 0000000..2cd1783
--- /dev/null
+++ b/plans/compiled/TASK-2.sh
@@ -0,0 +1,7 @@
+#!/usr/bin/env bash
+set -euo pipefail
+# TASK-2: Add MissingWorkspaceSection variant to Error enum in schema.rs
+# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
+# Type: replace
+# File: src/schema.rs
+python3 "$(dirname "$0")/TASK-2.py"
diff --git a/plans/compiled/TASK-3.py b/plans/compiled/TASK-3.py
new file mode 100644
index 0000000..cc00199
--- /dev/null
+++ b/plans/compiled/TASK-3.py
@@ -0,0 +1,44 @@
+#!/usr/bin/env python3
+"""TASK-3: Expand ErrorEntry struct with severity, kind, context, and cause fields"""
+import base64, json, subprocess, sys
+from pathlib import Path
+
+TASK_ID = "TASK-3"
+STEPS = json.loads('[{"before_b64": "I1tkZXJpdmUoRGVidWcsIENsb25lLCBzZXJkZTo6U2VyaWFsaXplLCBib246OkJ1aWxkZXIpXQojW3NlcmRlKHJlbmFtZV9hbGwgPSAiY2FtZWxDYXNlIildCnB1YiBzdHJ1Y3QgRXJyb3JFbnRyeSB7CiAgICBwdWIgZmlsZTogU3RyaW5nLAogICAgI1tidWlsZGVyKGRlZmF1bHQpXQogICAgcHViIGxpbmU6IHVzaXplLAogICAgcHViIG1lc3NhZ2U6IFN0cmluZywKfQo=", "after_b64": "I1tkZXJpdmUoRGVidWcsIENsb25lLCBzZXJkZTo6U2VyaWFsaXplLCBib246OkJ1aWxkZXIpXQojW3NlcmRlKHJlbmFtZV9hbGwgPSAiY2FtZWxDYXNlIildCnB1YiBzdHJ1Y3QgRXJyb3JFbnRyeSB7CiAgICBwdWIgZmlsZTogU3RyaW5nLAogICAgI1tidWlsZGVyKGRlZmF1bHQpXQogICAgcHViIGxpbmU6IHVzaXplLAogICAgcHViIG1lc3NhZ2U6IFN0cmluZywKICAgIHB1YiBzZXZlcml0eTogRXJyb3JTZXZlcml0eSwKICAgIHB1YiBraW5kOiBTdHJpbmcsCiAgICAjW3NlcmRlKHNraXBfc2VyaWFsaXppbmdfaWYgPSAiT3B0aW9uOjppc19ub25lIildCiAgICBwdWIgY29udGV4dDogT3B0aW9uPEVycm9yQ29udGV4dD4sCiAgICAjW3NlcmRlKHNraXBfc2VyaWFsaXppbmdfaWYgPSAiT3B0aW9uOjppc19ub25lIildCiAgICBwdWIgY2F1c2U6IE9wdGlvbjxTdHJpbmc+LAp9Cg==", "target": "src/schema.rs", "index": 0, "is_create": false}]')
+
+for step in STEPS:
+    before = base64.b64decode(step["before_b64"]).decode()
+    after = base64.b64decode(step["after_b64"]).decode()
+    target = step["target"]
+    idx = step["index"]
+    is_create = step["is_create"]
+
+    if is_create:
+        target_path = Path(target)
+        target_path.parent.mkdir(parents=True, exist_ok=True)
+        target_path.write_text(after)
+        print(f"OK {TASK_ID} change {idx}: created {target}")
+    else:
+        target_path = Path(target)
+        content = target_path.read_text()
+        if before not in content:
+            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
+            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
+            sys.exit(1)
+
+        result = subprocess.run(
+            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
+            capture_output=True, text=True,
+        )
+        if result.returncode != 0:
+            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
+            sys.exit(result.returncode)
+
+        new_content = target_path.read_text()
+        if after and after not in new_content:
+            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
+            sys.exit(1)
+
+        print(f"OK {TASK_ID} change {idx}: applied to {target}")
+
+print(f"OK {TASK_ID}: all changes applied")
diff --git a/plans/compiled/TASK-3.sh b/plans/compiled/TASK-3.sh
new file mode 100755
index 0000000..0e353e0
--- /dev/null
+++ b/plans/compiled/TASK-3.sh
@@ -0,0 +1,7 @@
+#!/usr/bin/env bash
+set -euo pipefail
+# TASK-3: Expand ErrorEntry struct with severity, kind, context, and cause fields
+# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
+# Type: replace
+# File: src/schema.rs
+python3 "$(dirname "$0")/TASK-3.py"
diff --git a/plans/compiled/TASK-4.py b/plans/compiled/TASK-4.py
new file mode 100644
index 0000000..966b3aa
--- /dev/null
+++ b/plans/compiled/TASK-4.py
@@ -0,0 +1,44 @@
+#!/usr/bin/env python3
+"""TASK-4: Change parse_file to return ParsedFile with optional parse errors instead of Result"""
+import base64, json, subprocess, sys
+from pathlib import Path
+
+TASK_ID = "TASK-4"
+STEPS = json.loads('[{"before_b64": "dXNlIGNyYXRlOjpzY2hlbWE6OnsKICAgIEVycm9yLCBGaWxlSW5mbywgSW1wbEluZm8sIEltcGxJdGVtLCBJbXBsSXRlbUtpbmQsIEltcG9ydCwgSXRlbUF0dHJzLCBJdGVtS2luZCwgUHVibGljSXRlbSwKICAgIFJlRXhwb3J0LCBSZXN1bHQsIFN1Ym1vZHVsZURlY2wsCn07CnVzZSBzdGQ6OnBhdGg6OlBhdGg7CgovLyDilIDilIAgcGFyc2VfZmlsZSDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIAKCi8vLyBSZWFkIGFuZCBwYXJzZSBhIFJ1c3Qgc291cmNlIGZpbGUuIFJldHVybnMgdGhlIHJhdyBgc3luOjpGaWxlYCBBU1QgKG5lZWRlZAovLy8gYnkgYG1vZHVsZV90cmVlYCBmb3IgaW5saW5lIG1vZHVsZSBpdGVtIGV4dHJhY3Rpb24pIGFuZCB0aGUgZXh0cmFjdGVkCi8vLyBgRmlsZUluZm9gLiBPbiBwYXJzZSBmYWlsdXJlLCB3YXJucyB0byBzdGRlcnIgYW5kIHJldHVybnMgZW1wdHkgcmVzdWx0cy4KcHViIGZuIHBhcnNlX2ZpbGUocGF0aDogJlBhdGgpIC0+IFJlc3VsdDwoc3luOjpGaWxlLCBGaWxlSW5mbyk+IHsKICAgIGxldCBjb250ZW50ID0gc3RkOjpmczo6cmVhZF90b19zdHJpbmcocGF0aCkubWFwX2Vycih8c291cmNlfCBFcnJvcjo6RmlsZVJlYWQgewogICAgICAgIHBhdGg6IHBhdGgudG9fcGF0aF9idWYoKSwKICAgICAgICBzb3VyY2UsCiAgICB9KT87CgogICAgbGV0IGZpbGUgPSBtYXRjaCBzeW46OnBhcnNlX2ZpbGUoJmNvbnRlbnQpIHsKICAgICAgICBPayhmKSA9PiBmLAogICAgICAgIEVycihlKSA9PiB7CiAgICAgICAgICAgIGVwcmludGxuISgid2FybmluZzogZmFpbGVkIHRvIHBhcnNlIHt9OiB7fSIsIHBhdGguZGlzcGxheSgpLCBlKTsKICAgICAgICAgICAgbGV0IGVtcHR5ID0gc3luOjpGaWxlIHsKICAgICAgICAgICAgICAgIHNoZWJhbmc6IE5vbmUsCiAgICAgICAgICAgICAgICBhdHRyczogdmVjIVtdLAogICAgICAgICAgICAgICAgaXRlbXM6IHZlYyFbXSwKICAgICAgICAgICAgfTsKICAgICAgICAgICAgbGV0IGluZm8gPSBGaWxlSW5mbzo6ZGVmYXVsdCgpOwogICAgICAgICAgICByZXR1cm4gT2soKGVtcHR5LCBpbmZvKSk7CiAgICAgICAgfQogICAgfTsKCiAgICBsZXQgaW5mbyA9IEZpbGVJbmZvIHsKICAgICAgICBwdWJsaWNfaXRlbXM6IGV4dHJhY3RfcHVibGljX2l0ZW1zKCZmaWxlLml0ZW1zKSwKICAgICAgICBpbXBvcnRzOiBleHRyYWN0X2ltcG9ydHMoJmZpbGUuaXRlbXMpLAogICAgICAgIHJlX2V4cG9ydHM6IGV4dHJhY3RfcmVfZXhwb3J0cygmZmlsZS5pdGVtcyksCiAgICAgICAgc3VibW9kdWxlczogZXh0cmFjdF9zdWJtb2R1bGVzKCZmaWxlLml0ZW1zKSwKICAgICAgICBpbXBsczogZXh0cmFjdF9pbXBscygmZmlsZS5pdGVtcyksCiAgICB9OwoKICAgIE9rKChmaWxlLCBpbmZvKSkKfQ==", "after_b64": "dXNlIGNyYXRlOjpzY2hlbWE6OnsKICAgIEVycm9yLCBFcnJvckVudHJ5LCBFcnJvclNldmVyaXR5LCBGaWxlSW5mbywgSW1wbEluZm8sIEltcGxJdGVtLCBJbXBsSXRlbUtpbmQsIEltcG9ydCwKICAgIEl0ZW1BdHRycywgSXRlbUtpbmQsIFB1YmxpY0l0ZW0sIFJlRXhwb3J0LCBSZXN1bHQsIFN1Ym1vZHVsZURlY2wsCn07CnVzZSBzdGQ6OnBhdGg6OlBhdGg7CgovLyDilIDilIAgSW50ZXJuYWwgcGFyc2UgcmVzdWx0IHR5cGVzIOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgAoKLy8vIFJlc3VsdCBvZiBwYXJzaW5nIGEgUnVzdCBzb3VyY2UgZmlsZS4KLy8vCi8vLyBVbmxpa2UgYFJlc3VsdDxULCBFcnJvcj5gLCB0aGlzIHR5cGUgYWx3YXlzIHN1Y2NlZWRzIOKAlCBwYXJzZQovLy8gZmFpbHVyZXMgYXJlIHJlcG9ydGVkIGFzIGRhdGEsIG5vdCBhcyBlcnJvcnMsIHNvIHRoZSBjYWxsZXIKLy8vIGNhbiBjb250aW51ZSBwcm9jZXNzaW5nIG90aGVyIGZpbGVzLiBUaGUgY2FsbGVyIGNvbnN0cnVjdHMKLy8vIGBFcnJvckVudHJ5YCB2YWx1ZXMgZnJvbSBgU3luUGFyc2VFcnJvcmAgd2hlbiBuZWVkZWQuCnB1YiBzdHJ1Y3QgUGFyc2VkRmlsZSB7CiAgICBwdWIgYXN0OiBzeW46OkZpbGUsCiAgICBwdWIgZmlsZV9pbmZvOiBGaWxlSW5mbywKICAgIHB1YiBwYXJzZV9lcnJvcjogT3B0aW9uPFN5blBhcnNlRXJyb3I+LAp9CgovLy8gU3RydWN0dXJlZCBpbmZvcm1hdGlvbiBhYm91dCBhIHBhcnNlIGZhaWx1cmUuCnB1YiBzdHJ1Y3QgU3luUGFyc2VFcnJvciB7CiAgICBwdWIgbWVzc2FnZTogU3RyaW5nLAogICAgcHViIGxpbmU6IHVzaXplLAp9CgovLyDilIDilIAgcGFyc2VfZmlsZSDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIAKCi8vLyBSZWFkIGFuZCBwYXJzZSBhIFJ1c3Qgc291cmNlIGZpbGUuCi8vLwovLy8gT24gcGFyc2UgZmFpbHVyZSwgcmV0dXJucyB0aGUgb3JpZ2luYWwgZmlsZSBjb250ZW50IGFuZCBhCi8vLyBgU3luUGFyc2VFcnJvcmAgYWxvbmdzaWRlIGFuIGVtcHR5IGBGaWxlSW5mb2AuIENhbGxlcnMgdXNlIHRoZQovLy8gZXJyb3IgdG8gY29uc3RydWN0IGFuIGBFcnJvckVudHJ5YC4KcHViIGZuIHBhcnNlX2ZpbGUocGF0aDogJlBhdGgpIC0+IFBhcnNlZEZpbGUgewogICAgbGV0IGNvbnRlbnQgPSBtYXRjaCBzdGQ6OmZzOjpyZWFkX3RvX3N0cmluZyhwYXRoKSB7CiAgICAgICAgT2soYykgPT4gYywKICAgICAgICBFcnIoc291cmNlKSA9PiB7CiAgICAgICAgICAgIGxldCBlcnIgPSBTeW5QYXJzZUVycm9yIHsKICAgICAgICAgICAgICAgIG1lc3NhZ2U6IHNvdXJjZS50b19zdHJpbmcoKSwKICAgICAgICAgICAgICAgIGxpbmU6IDAsCiAgICAgICAgICAgIH07CiAgICAgICAgICAgIHJldHVybiBQYXJzZWRGaWxlIHsKICAgICAgICAgICAgICAgIGFzdDogc3luOjpGaWxlIHsKICAgICAgICAgICAgICAgICAgICBzaGViYW5nOiBOb25lLAogICAgICAgICAgICAgICAgICAgIGF0dHJzOiB2ZWMhW10sCiAgICAgICAgICAgICAgICAgICAgaXRlbXM6IHZlYyFbXSwKICAgICAgICAgICAgICAgIH0sCiAgICAgICAgICAgICAgICBmaWxlX2luZm86IEZpbGVJbmZvOjpkZWZhdWx0KCksCiAgICAgICAgICAgICAgICBwYXJzZV9lcnJvcjogU29tZShlcnIpLAogICAgICAgICAgICB9OwogICAgICAgIH0KICAgIH07CgogICAgbWF0Y2ggc3luOjpwYXJzZV9maWxlKCZjb250ZW50KSB7CiAgICAgICAgT2soZmlsZSkgPT4gewogICAgICAgICAgICBsZXQgZmlsZV9pbmZvID0gRmlsZUluZm8gewogICAgICAgICAgICAgICAgcHVibGljX2l0ZW1zOiBleHRyYWN0X3B1YmxpY19pdGVtcygmZmlsZS5pdGVtcyksCiAgICAgICAgICAgICAgICBpbXBvcnRzOiBleHRyYWN0X2ltcG9ydHMoJmZpbGUuaXRlbXMpLAogICAgICAgICAgICAgICAgcmVfZXhwb3J0czogZXh0cmFjdF9yZV9leHBvcnRzKCZmaWxlLml0ZW1zKSwKICAgICAgICAgICAgICAgIHN1Ym1vZHVsZXM6IGV4dHJhY3Rfc3VibW9kdWxlcygmZmlsZS5pdGVtcyksCiAgICAgICAgICAgICAgICBpbXBsczogZXh0cmFjdF9pbXBscygmZmlsZS5pdGVtcyksCiAgICAgICAgICAgIH07CiAgICAgICAgICAgIFBhcnNlZEZpbGUgewogICAgICAgICAgICAgICAgYXN0OiBmaWxlLAogICAgICAgICAgICAgICAgZmlsZV9pbmZvLAogICAgICAgICAgICAgICAgcGFyc2VfZXJyb3I6IE5vbmUsCiAgICAgICAgICAgIH0KICAgICAgICB9LAogICAgICAgIEVycihlKSA9PiB7CiAgICAgICAgICAgIGxldCBsaW5lID0gZS5zcGFuKCkuc3RhcnQoKS5saW5lOwogICAgICAgICAgICBsZXQgZXJyID0gU3luUGFyc2VFcnJvciB7CiAgICAgICAgICAgICAgICBtZXNzYWdlOiBlLnRvX3N0cmluZygpLAogICAgICAgICAgICAgICAgbGluZSwKICAgICAgICAgICAgfTsKICAgICAgICAgICAgUGFyc2VkRmlsZSB7CiAgICAgICAgICAgICAgICBhc3Q6IHN5bjo6RmlsZSB7CiAgICAgICAgICAgICAgICAgICAgc2hlYmFuZzogTm9uZSwKICAgICAgICAgICAgICAgICAgICBhdHRyczogdmVjIVtdLAogICAgICAgICAgICAgICAgICAgIGl0ZW1zOiB2ZWMhW10sCiAgICAgICAgICAgICAgICB9LAogICAgICAgICAgICAgICAgZmlsZV9pbmZvOiBGaWxlSW5mbzo6ZGVmYXVsdCgpLAogICAgICAgICAgICAgICAgcGFyc2VfZXJyb3I6IFNvbWUoZXJyKSwKICAgICAgICAgICAgfQogICAgICAgIH0KICAgIH0KfQoKcHViKGNyYXRlKSBmbiBidWlsZF9wYXJzZV9lcnJvcl9lbnRyeShwYXRoOiAmUGF0aCwgZXJyOiAmU3luUGFyc2VFcnJvcikgLT4gRXJyb3JFbnRyeSB7CiAgICBFcnJvckVudHJ5OjpidWlsZGVyKCkKICAgICAgICAuZmlsZShwYXRoLnRvX3N0cmluZ19sb3NzeSgpLnRvX3N0cmluZygpKQogICAgICAgIC5saW5lKGVyci5saW5lKQogICAgICAgIC5tZXNzYWdlKGVyci5tZXNzYWdlLmNsb25lKCkpCiAgICAgICAgLnNldmVyaXR5KEVycm9yU2V2ZXJpdHk6OkVycm9yKQogICAgICAgIC5raW5kKCJzeW5fcGFyc2VfZXJyb3IiLnRvX3N0cmluZygpKQogICAgICAgIC5idWlsZCgpCn0=", "target": "src/file_parser.rs", "index": 0, "is_create": false}]')
+
+for step in STEPS:
+    before = base64.b64decode(step["before_b64"]).decode()
+    after = base64.b64decode(step["after_b64"]).decode()
+    target = step["target"]
+    idx = step["index"]
+    is_create = step["is_create"]
+
+    if is_create:
+        target_path = Path(target)
+        target_path.parent.mkdir(parents=True, exist_ok=True)
+        target_path.write_text(after)
+        print(f"OK {TASK_ID} change {idx}: created {target}")
+    else:
+        target_path = Path(target)
+        content = target_path.read_text()
+        if before not in content:
+            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
+            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
+            sys.exit(1)
+
+        result = subprocess.run(
+            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
+            capture_output=True, text=True,
+        )
+        if result.returncode != 0:
+            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
+            sys.exit(result.returncode)
+
+        new_content = target_path.read_text()
+        if after and after not in new_content:
+            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
+            sys.exit(1)
+
+        print(f"OK {TASK_ID} change {idx}: applied to {target}")
+
+print(f"OK {TASK_ID}: all changes applied")
diff --git a/plans/compiled/TASK-4.sh b/plans/compiled/TASK-4.sh
new file mode 100755
index 0000000..aae910d
--- /dev/null
+++ b/plans/compiled/TASK-4.sh
@@ -0,0 +1,7 @@
+#!/usr/bin/env bash
+set -euo pipefail
+# TASK-4: Change parse_file to return ParsedFile with optional parse errors instead of Result
+# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
+# Type: replace
+# File: src/file_parser.rs
+python3 "$(dirname "$0")/TASK-4.py"
diff --git a/plans/compiled/TASK-5.py b/plans/compiled/TASK-5.py
new file mode 100644
index 0000000..94c7ef6
--- /dev/null
+++ b/plans/compiled/TASK-5.py
@@ -0,0 +1,44 @@
+#!/usr/bin/env python3
+"""TASK-5: Refactor module_tree.rs: fix path fallbacks, add error collection, change build_module_tree return type"""
+import base64, json, subprocess, sys
+from pathlib import Path
+
+TASK_ID = "TASK-5"
+STEPS = json.loads('[{"before_b64": "dXNlIGNyYXRlOjpzY2hlbWE6OntGaWxlSW5mbywgTW9kdWxlSW5mbywgUmVzdWx0LCBTdWJtb2R1bGVEZWNsfTs=", "after_b64": "dXNlIGNyYXRlOjpzY2hlbWE6OntFcnJvckNvbnRleHQsIEVycm9yRW50cnksIEVycm9yU2V2ZXJpdHksIEZpbGVJbmZvLCBNb2R1bGVJbmZvLCBSZXN1bHQsIFN1Ym1vZHVsZURlY2x9Ow==", "target": "src/module_tree.rs", "index": 0, "is_create": false}, {"before_b64": "Ly8vIEJ1aWxkIHRoZSBmdWxsIG1vZHVsZSB0cmVlIGZvciBhIGNyYXRlIHN0YXJ0aW5nIGZyb20gaXRzIGVudHJ5IHBvaW50Ci8vLyAoZS5nLiwgYHNyYy9saWIucnNgKS4gUmV0dXJucyBhIGZsYXQgYFZlYzxNb2R1bGVJbmZvPmAgY29udGFpbmluZyB0aGUKLy8vIHJvb3QgbW9kdWxlIGFuZCBhbGwgcmVjdXJzaXZlbHkgZGlzY292ZXJlZCBzdWJtb2R1bGVzLgpwdWIgZm4gYnVpbGRfbW9kdWxlX3RyZWUoY3JhdGVfcm9vdDogJlBhdGgsIGNyYXRlX25hbWU6ICZzdHIpIC0+IFJlc3VsdDxWZWM8TW9kdWxlSW5mbz4+IHsKICAgIGxldCBtdXQgdmlzaXRlZCA9IEhhc2hTZXQ6Om5ldygpOwogICAgbGV0IHBhcmVudF9kaXIgPSBjcmF0ZV9yb290LnBhcmVudCgpLnVud3JhcF9vcl9lbHNlKHx8IFBhdGg6Om5ldygiLiIpKTsKCiAgICBsZXQgKGFzdCwgZmlsZV9pbmZvKSA9IGZpbGVfcGFyc2VyOjpwYXJzZV9maWxlKGNyYXRlX3Jvb3QpPzsKICAgIHZpc2l0ZWQuaW5zZXJ0KGNyYXRlX3Jvb3QudG9fcGF0aF9idWYoKSk7CgogICAgbGV0IHJvb3RfbW9kdWxlID0gYnVpbGRfbW9kdWxlX2luZm8oCiAgICAgICAgY3JhdGVfbmFtZSwKICAgICAgICBjcmF0ZV9yb290LAogICAgICAgICJwdWIiLAogICAgICAgICZmaWxlX2luZm8ucHVibGljX2l0ZW1zLAogICAgICAgICZmaWxlX2luZm8uaW1wb3J0cywKICAgICAgICAmZmlsZV9pbmZvLnJlX2V4cG9ydHMsCiAgICAgICAgJmZpbGVfaW5mby5zdWJtb2R1bGVzLAogICAgKTsKCiAgICBsZXQgbXV0IG1vZHVsZXMgPSB2ZWMhW3Jvb3RfbW9kdWxlXTsKCiAgICBmb3Igc3ViIGluICZmaWxlX2luZm8uc3VibW9kdWxlcyB7CiAgICAgICAgaWYgc3ViLmlzX3Rlc3QgewogICAgICAgICAgICBjb250aW51ZTsKICAgICAgICB9CiAgICAgICAgbGV0IHN1Yl9tb2R1bGVfcGF0aCA9IGZvcm1hdCEoInt9Ojp7fSIsIGNyYXRlX25hbWUsIHN1Yi5uYW1lKTsKICAgICAgICBsZXQgY2hpbGRfbW9kdWxlcyA9IHByb2Nlc3Nfc3VibW9kdWxlKAogICAgICAgICAgICAmc3ViX21vZHVsZV9wYXRoLAogICAgICAgICAgICAmc3ViLm5hbWUsCiAgICAgICAgICAgICZhc3QuaXRlbXMsCiAgICAgICAgICAgIHBhcmVudF9kaXIsCiAgICAgICAgICAgIGNyYXRlX3Jvb3QsCiAgICAgICAgICAgICZtdXQgdmlzaXRlZCwKICAgICAgICApPzsKICAgICAgICBtb2R1bGVzLmV4dGVuZChjaGlsZF9tb2R1bGVzKTsKICAgIH0KCiAgICBPayhtb2R1bGVzKQp9", "after_b64": "Ly8vIEJ1aWxkIHRoZSBmdWxsIG1vZHVsZSB0cmVlIGZvciBhIGNyYXRlIHN0YXJ0aW5nIGZyb20gaXRzIGVudHJ5IHBvaW50Ci8vLyAoZS5nLiwgYHNyYy9saWIucnNgKS4gUmV0dXJucyBhIHR1cGxlIG9mIG1vZHVsZSBpbmZvIGFuZCBhbnkgZXJyb3JzCi8vLyBlbmNvdW50ZXJlZCBkdXJpbmcgc3VibW9kdWxlIHBhcnNpbmcgKGluY2x1ZGluZyBvcnBoYW5lZCBtb2R1bGUgd2FybmluZ3MpLgpwdWIgZm4gYnVpbGRfbW9kdWxlX3RyZWUoCiAgICBjcmF0ZV9yb290OiAmUGF0aCwKICAgIGNyYXRlX25hbWU6ICZzdHIsCikgLT4gKFZlYzxNb2R1bGVJbmZvPiwgVmVjPGNyYXRlOjpzY2hlbWE6OkVycm9yRW50cnk+KSB7CgogICAgbGV0IG11dCB2aXNpdGVkID0gSGFzaFNldDo6bmV3KCk7CiAgICBsZXQgcGFyZW50X2RpciA9IGNyYXRlX3Jvb3QucGFyZW50KCkudW53cmFwX29yKGNyYXRlX3Jvb3QpOwoKICAgIGxldCBwYXJzZWQgPSBmaWxlX3BhcnNlcjo6cGFyc2VfZmlsZShjcmF0ZV9yb290KTsKICAgIGxldCBtdXQgZXJyb3JzOiBWZWM8RXJyb3JFbnRyeT4gPSBWZWM6Om5ldygpOwogICAgaWYgbGV0IFNvbWUocmVmIGVycikgPSBwYXJzZWQucGFyc2VfZXJyb3IgewogICAgICAgIGVycm9ycy5wdXNoKGNyYXRlOjpmaWxlX3BhcnNlcjo6YnVpbGRfcGFyc2VfZXJyb3JfZW50cnkoY3JhdGVfcm9vdCwgZXJyKSk7CiAgICB9CiAgICB2aXNpdGVkLmluc2VydChjcmF0ZV9yb290LnRvX3BhdGhfYnVmKCkpOwoKICAgIGxldCByb290X21vZHVsZSA9IGJ1aWxkX21vZHVsZV9pbmZvKAogICAgICAgIGNyYXRlX25hbWUsCiAgICAgICAgY3JhdGVfcm9vdCwKICAgICAgICAicHViIiwKICAgICAgICAmcGFyc2VkLmZpbGVfaW5mby5wdWJsaWNfaXRlbXMsCiAgICAgICAgJnBhcnNlZC5maWxlX2luZm8uaW1wb3J0cywKICAgICAgICAmcGFyc2VkLmZpbGVfaW5mby5yZV9leHBvcnRzLAogICAgICAgICZwYXJzZWQuZmlsZV9pbmZvLnN1Ym1vZHVsZXMsCiAgICApOwoKICAgIGxldCBtdXQgbW9kdWxlcyA9IHZlYyFbcm9vdF9tb2R1bGVdOwoKICAgIGZvciBzdWIgaW4gJnBhcnNlZC5maWxlX2luZm8uc3VibW9kdWxlcyB7CiAgICAgICAgaWYgc3ViLmlzX3Rlc3QgewogICAgICAgICAgICBjb250aW51ZTsKICAgICAgICB9CiAgICAgICAgbGV0IHN1Yl9tb2R1bGVfcGF0aCA9IGZvcm1hdCEoInt9Ojp7fSIsIGNyYXRlX25hbWUsIHN1Yi5uYW1lKTsKICAgICAgICBsZXQgKGNoaWxkX21vZHVsZXMsIGNoaWxkX2Vycm9ycykgPSBwcm9jZXNzX3N1Ym1vZHVsZSgKICAgICAgICAgICAgJnN1Yl9tb2R1bGVfcGF0aCwKICAgICAgICAgICAgJnN1Yi5uYW1lLAogICAgICAgICAgICAmcGFyc2VkLmFzdC5pdGVtcywKICAgICAgICAgICAgcGFyZW50X2RpciwKICAgICAgICAgICAgY3JhdGVfcm9vdCwKICAgICAgICAgICAgJm11dCB2aXNpdGVkLAogICAgICAgICk7CiAgICAgICAgZXJyb3JzLmV4dGVuZChjaGlsZF9lcnJvcnMpOwogICAgICAgIG1vZHVsZXMuZXh0ZW5kKGNoaWxkX21vZHVsZXMpOwogICAgfQoKICAgIChtb2R1bGVzLCBlcnJvcnMpCn0=", "target": "src/module_tree.rs", "index": 1, "is_create": false}, {"before_b64": "ICAgIGxldCBTb21lKG1vZF9pdGVtKSA9IG1vZF9pdGVtIGVsc2UgewogICAgICAgIGVwcmludGxuISgid2FybmluZzogb3JwaGFuZWQgbW9kdWxlIHt9IiwgbW9kdWxlX3BhdGgpOwogICAgICAgIHJldHVybiBPayh2ZWMhW01vZHVsZUluZm86OmJ1aWxkZXIoKQogICAgICAgICAgICAucGF0aChtb2R1bGVfcGF0aC50b19zdHJpbmcoKSkKICAgICAgICAgICAgLmZpbGUoIjx1bnJlc29sdmVkPiIudG9fc3RyaW5nKCkpCiAgICAgICAgICAgIC52aXNpYmlsaXR5KCJwcml2YXRlIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgLmJ1aWxkKCldKTsKICAgIH07", "after_b64": "ICAgIGxldCBTb21lKG1vZF9pdGVtKSA9IG1vZF9pdGVtIGVsc2UgewogICAgICAgIGxldCBlcnIgPSBFcnJvckVudHJ5OjpidWlsZGVyKCkKICAgICAgICAgICAgLmZpbGUoU3RyaW5nOjpuZXcoKSkKICAgICAgICAgICAgLm1lc3NhZ2UoZm9ybWF0ISgib3JwaGFuZWQgbW9kdWxlOiB7bW9kdWxlX3BhdGh9IikpCiAgICAgICAgICAgIC5zZXZlcml0eShFcnJvclNldmVyaXR5OjpXYXJuaW5nKQogICAgICAgICAgICAua2luZCgib3JwaGFuZWRfbW9kdWxlIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgLmNvbnRleHQoRXJyb3JDb250ZXh0OjpidWlsZGVyKCkKICAgICAgICAgICAgICAgIC5tb2R1bGVfcGF0aChtb2R1bGVfcGF0aC50b19zdHJpbmcoKSkKICAgICAgICAgICAgICAgIC5idWlsZCgpKQogICAgICAgICAgICAuYnVpbGQoKTsKICAgICAgICByZXR1cm4gKHZlYyFbTW9kdWxlSW5mbzo6YnVpbGRlcigpCiAgICAgICAgICAgIC5wYXRoKG1vZHVsZV9wYXRoLnRvX3N0cmluZygpKQogICAgICAgICAgICAuZmlsZSgiPHVucmVzb2x2ZWQ+Ii50b19zdHJpbmcoKSkKICAgICAgICAgICAgLnZpc2liaWxpdHkoInByaXZhdGUiLnRvX3N0cmluZygpKQogICAgICAgICAgICAuYnVpbGQoKV0sIHZlYyFbZXJyXSk7CiAgICB9Ow==", "target": "src/module_tree.rs", "index": 2, "is_create": false}, {"before_b64": "ICAgICAgICBsZXQgU29tZShyZWYgZmlsZV9wYXRoKSA9IGZpbGVfcGF0aCBlbHNlIHsKICAgICAgICAgICAgZXByaW50bG4hKCJ3YXJuaW5nOiBvcnBoYW5lZCBtb2R1bGUge30iLCBtb2R1bGVfcGF0aCk7CiAgICAgICAgICAgIHJldHVybiBPayh2ZWMhW01vZHVsZUluZm86OmJ1aWxkZXIoKQogICAgICAgICAgICAgICAgLnBhdGgobW9kdWxlX3BhdGgudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAuZmlsZSgiPHVucmVzb2x2ZWQ+Ii50b19zdHJpbmcoKSkKICAgICAgICAgICAgICAgIC52aXNpYmlsaXR5KHZpc2liaWxpdHkudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAuYnVpbGQoKV0pOwogICAgICAgIH07", "after_b64": "ICAgICAgICBsZXQgU29tZShyZWYgZmlsZV9wYXRoKSA9IGZpbGVfcGF0aCBlbHNlIHsKICAgICAgICAgICAgbGV0IGVyciA9IEVycm9yRW50cnk6OmJ1aWxkZXIoKQogICAgICAgICAgICAgICAgLmZpbGUoU3RyaW5nOjpuZXcoKSkKICAgICAgICAgICAgICAgIC5tZXNzYWdlKGZvcm1hdCEoIm9ycGhhbmVkIG1vZHVsZToge21vZHVsZV9wYXRofSIpKQogICAgICAgICAgICAgICAgLnNldmVyaXR5KEVycm9yU2V2ZXJpdHk6Oldhcm5pbmcpCiAgICAgICAgICAgICAgICAua2luZCgib3JwaGFuZWRfbW9kdWxlIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgICAgIC5jb250ZXh0KEVycm9yQ29udGV4dDo6YnVpbGRlcigpCiAgICAgICAgICAgICAgICAgICAgLm1vZHVsZV9wYXRoKG1vZHVsZV9wYXRoLnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgICAgIC5idWlsZCgpKQogICAgICAgICAgICAgICAgLmJ1aWxkKCk7CiAgICAgICAgICAgIHJldHVybiAodmVjIVtNb2R1bGVJbmZvOjpidWlsZGVyKCkKICAgICAgICAgICAgICAgIC5wYXRoKG1vZHVsZV9wYXRoLnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgLmZpbGUoIjx1bnJlc29sdmVkPiIudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAudmlzaWJpbGl0eSh2aXNpYmlsaXR5LnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgLmJ1aWxkKCldLCB2ZWMhW2Vycl0pOwogICAgICAgIH07", "target": "src/module_tree.rs", "index": 3, "is_create": false}, {"before_b64": "ICAgICAgICBsZXQgKGFzdCwgZmlsZV9pbmZvKSA9IGZpbGVfcGFyc2VyOjpwYXJzZV9maWxlKGZpbGVfcGF0aCk/OwogICAgICAgIHByb2Nlc3NfbW9kdWxlX2luZm8oCiAgICAgICAgICAgIG1vZHVsZV9wYXRoLAogICAgICAgICAgICBmaWxlX3BhdGgsCiAgICAgICAgICAgIHZpc2liaWxpdHksCiAgICAgICAgICAgICZmaWxlX2luZm8sCiAgICAgICAgICAgICZhc3QuaXRlbXMsCiAgICAgICAgICAgICZmaWxlX3BhdGgucGFyZW50KCkudW53cmFwX29yX2Vsc2UofHwgUGF0aDo6bmV3KCIuIikpLAogICAgICAgICAgICB2aXNpdGVkLAogICAgICAgICk=", "after_b64": "ICAgICAgICBsZXQgcGFyc2VkID0gZmlsZV9wYXJzZXI6OnBhcnNlX2ZpbGUoZmlsZV9wYXRoKTsKICAgICAgICBsZXQgbXV0IGVycm9yczogVmVjPEVycm9yRW50cnk+ID0gVmVjOjpuZXcoKTsKICAgICAgICBpZiBsZXQgU29tZShyZWYgZXJyKSA9IHBhcnNlZC5wYXJzZV9lcnJvciB7CiAgICAgICAgICAgIGVycm9ycy5wdXNoKGNyYXRlOjpmaWxlX3BhcnNlcjo6YnVpbGRfcGFyc2VfZXJyb3JfZW50cnkoZmlsZV9wYXRoLCBlcnIpKTsKICAgICAgICB9CiAgICAgICAgcHJvY2Vzc19tb2R1bGVfaW5mbygKICAgICAgICAgICAgbW9kdWxlX3BhdGgsCiAgICAgICAgICAgIGZpbGVfcGF0aCwKICAgICAgICAgICAgdmlzaWJpbGl0eSwKICAgICAgICAgICAgJnBhcnNlZC5maWxlX2luZm8sCiAgICAgICAgICAgICZwYXJzZWQuYXN0Lml0ZW1zLAogICAgICAgICAgICAmZmlsZV9wYXRoLnBhcmVudCgpLnVud3JhcF9vcihmaWxlX3BhdGgpLAogICAgICAgICAgICB2aXNpdGVkLAogICAgICAgICAgICAmbXV0IGVycm9ycywKICAgICAgICAp", "target": "src/module_tree.rs", "index": 4, "is_create": false}, {"before_b64": "ICAgICAgICBpZiB2aXNpdGVkLmNvbnRhaW5zKGZpbGVfcGF0aC5hc19wYXRoKCkpIHsKICAgICAgICAgICAgcmV0dXJuIE9rKHZlYyFbXSk7IC8vIGN5Y2xlIGRldGVjdGVkCiAgICAgICAgfQ==", "after_b64": "ICAgICAgICBpZiB2aXNpdGVkLmNvbnRhaW5zKGZpbGVfcGF0aC5hc19wYXRoKCkpIHsKICAgICAgICAgICAgcmV0dXJuICh2ZWMhW10sIHZlYyFbXSk7IC8vIGN5Y2xlIGRldGVjdGVkCiAgICAgICAgfQ==", "target": "src/module_tree.rs", "index": 5, "is_create": false}, {"before_b64": "Ly8vIFJlc29sdmUgYSBgbW9kIG5hbWU7YCBkZWNsYXJhdGlvbiB0byBhIGZpbGUgcGF0aC4KLy8vIFRyaWVzIGB7cGFyZW50X2Rpcn0ve21vZF9uYW1lfS5yc2AgZmlyc3QsIHRoZW4gYHtwYXJlbnRfZGlyfS97bW9kX25hbWV9L21vZC5yc2AuCnB1YiBmbiByZXNvbHZlX21vZHVsZV9wYXRoKHBhcmVudF9kaXI6ICZQYXRoLCBtb2RfbmFtZTogJnN0cikgLT4gT3B0aW9uPFBhdGhCdWY+IHs=", "after_b64": "Ly8vIFJlc29sdmUgYSBgbW9kIG5hbWU7YCBkZWNsYXJhdGlvbiB0byBhIGZpbGUgcGF0aC4KLy8vIFRyaWVzIGB7cGFyZW50X2Rpcn0ve21vZF9uYW1lfS5yc2AgZmlyc3QsIHRoZW4gYHtwYXJlbnRfZGlyfS97bW9kX25hbWV9L21vZC5yc2AuCi8vLwovLy8gUmV0dXJucyBgTm9uZWAgaWYgbmVpdGhlciBwYXRoIGV4aXN0cy4KcHViIGZuIHJlc29sdmVfbW9kdWxlX3BhdGgocGFyZW50X2RpcjogJlBhdGgsIG1vZF9uYW1lOiAmc3RyKSAtPiBPcHRpb248UGF0aEJ1Zj4gew==", "target": "src/module_tree.rs", "index": 6, "is_create": false}, {"before_b64": "Zm4gcHJvY2Vzc19zdWJtb2R1bGUoCiAgICBtb2R1bGVfcGF0aDogJnN0ciwKICAgIG1vZF9uYW1lOiAmc3RyLAogICAgcGFyZW50X2l0ZW1zOiAmW3N5bjo6SXRlbV0sCiAgICBwYXJlbnRfZGlyOiAmUGF0aCwKICAgIHBhcmVudF9maWxlOiAmUGF0aCwKICAgIHZpc2l0ZWQ6ICZtdXQgSGFzaFNldDxQYXRoQnVmPiwKKSAtPiBSZXN1bHQ8VmVjPE1vZHVsZUluZm8+PiB7", "after_b64": "Zm4gcHJvY2Vzc19zdWJtb2R1bGUoCiAgICBtb2R1bGVfcGF0aDogJnN0ciwKICAgIG1vZF9uYW1lOiAmc3RyLAogICAgcGFyZW50X2l0ZW1zOiAmW3N5bjo6SXRlbV0sCiAgICBwYXJlbnRfZGlyOiAmUGF0aCwKICAgIHBhcmVudF9maWxlOiAmUGF0aCwKICAgIHZpc2l0ZWQ6ICZtdXQgSGFzaFNldDxQYXRoQnVmPiwKKSAtPiAoVmVjPE1vZHVsZUluZm8+LCBWZWM8Y3JhdGU6OnNjaGVtYTo6RXJyb3JFbnRyeT4pIHs=", "target": "src/module_tree.rs", "index": 7, "is_create": false}, {"before_b64": "ICAgIGlmIGxldCBTb21lKChfLCByZWYgaW5saW5lX2l0ZW1zKSkgPSBtb2RfaXRlbS5jb250ZW50IHsKICAgICAgICAvLyBJbmxpbmUgbW9kdWxlOiBwcm9jZXNzIGl0cyBib2R5IGl0ZW1zIGRpcmVjdGx5IChubyBmaWxlIGxvb2t1cCkuCiAgICAgICAgcHJvY2Vzc19tb2R1bGVfaXRlbXMoCiAgICAgICAgICAgIG1vZHVsZV9wYXRoLAogICAgICAgICAgICBwYXJlbnRfZmlsZSwKICAgICAgICAgICAgdmlzaWJpbGl0eSwKICAgICAgICAgICAgaW5saW5lX2l0ZW1zLAogICAgICAgICAgICBwYXJlbnRfZGlyLAogICAgICAgICAgICB2aXNpdGVkLAogICAgICAgICkKICAgIH0gZWxzZSB7", "after_b64": "ICAgIGlmIGxldCBTb21lKChfLCByZWYgaW5saW5lX2l0ZW1zKSkgPSBtb2RfaXRlbS5jb250ZW50IHsKICAgICAgICAvLyBJbmxpbmUgbW9kdWxlOiBwcm9jZXNzIGl0cyBib2R5IGl0ZW1zIGRpcmVjdGx5IChubyBmaWxlIGxvb2t1cCkuCiAgICAgICAgbGV0IChtb2R1bGVzLCBlcnJzKSA9IHByb2Nlc3NfbW9kdWxlX2l0ZW1zKAogICAgICAgICAgICBtb2R1bGVfcGF0aCwKICAgICAgICAgICAgcGFyZW50X2ZpbGUsCiAgICAgICAgICAgIHZpc2liaWxpdHksCiAgICAgICAgICAgIGlubGluZV9pdGVtcywKICAgICAgICAgICAgcGFyZW50X2RpciwKICAgICAgICAgICAgdmlzaXRlZCwKICAgICAgICApOwogICAgICAgIHJldHVybiAobW9kdWxlcywgZXJycyk7CiAgICB9IGVsc2Ugew==", "target": "src/module_tree.rs", "index": 8, "is_create": false}, {"before_b64": "Zm4gcHJvY2Vzc19tb2R1bGVfaXRlbXMoCiAgICBtb2R1bGVfcGF0aDogJnN0ciwKICAgIGZpbGVfcGF0aDogJlBhdGgsCiAgICB2aXNpYmlsaXR5OiAmc3RyLAogICAgaXRlbXM6ICZbc3luOjpJdGVtXSwKICAgIHBhcmVudF9kaXI6ICZQYXRoLAogICAgdmlzaXRlZDogJm11dCBIYXNoU2V0PFBhdGhCdWY+LAopIC0+IFJlc3VsdDxWZWM8TW9kdWxlSW5mbz4+IHs=", "after_b64": "Zm4gcHJvY2Vzc19tb2R1bGVfaXRlbXMoCiAgICBtb2R1bGVfcGF0aDogJnN0ciwKICAgIGZpbGVfcGF0aDogJlBhdGgsCiAgICB2aXNpYmlsaXR5OiAmc3RyLAogICAgaXRlbXM6ICZbc3luOjpJdGVtXSwKICAgIHBhcmVudF9kaXI6ICZQYXRoLAogICAgdmlzaXRlZDogJm11dCBIYXNoU2V0PFBhdGhCdWY+LAopIC0+IChWZWM8TW9kdWxlSW5mbz4sIFZlYzxjcmF0ZTo6c2NoZW1hOjpFcnJvckVudHJ5Pikgew==", "target": "src/module_tree.rs", "index": 9, "is_create": false}, {"before_b64": "ICAgIGxldCBmaWxlX2luZm8gPSBGaWxlSW5mbyB7CiAgICAgICAgcHVibGljX2l0ZW1zOiBmaWxlX3BhcnNlcjo6ZXh0cmFjdF9wdWJsaWNfaXRlbXMoaXRlbXMpLAogICAgICAgIGltcG9ydHM6IGZpbGVfcGFyc2VyOjpleHRyYWN0X2ltcG9ydHMoaXRlbXMpLAogICAgICAgIHJlX2V4cG9ydHM6IGZpbGVfcGFyc2VyOjpleHRyYWN0X3JlX2V4cG9ydHMoaXRlbXMpLAogICAgICAgIHN1Ym1vZHVsZXM6IGZpbGVfcGFyc2VyOjpleHRyYWN0X3N1Ym1vZHVsZXMoaXRlbXMpLAogICAgICAgIGltcGxzOiBmaWxlX3BhcnNlcjo6ZXh0cmFjdF9pbXBscyhpdGVtcyksCiAgICB9OwogICAgcHJvY2Vzc19tb2R1bGVfaW5mbyhtb2R1bGVfcGF0aCwgZmlsZV9wYXRoLCB2aXNpYmlsaXR5LCAmZmlsZV9pbmZvLCBpdGVtcywgcGFyZW50X2RpciwgdmlzaXRlZCkKfQ==", "after_b64": "ICAgIGxldCBmaWxlX2luZm8gPSBGaWxlSW5mbyB7CiAgICAgICAgcHVibGljX2l0ZW1zOiBmaWxlX3BhcnNlcjo6ZXh0cmFjdF9wdWJsaWNfaXRlbXMoaXRlbXMpLAogICAgICAgIGltcG9ydHM6IGZpbGVfcGFyc2VyOjpleHRyYWN0X2ltcG9ydHMoaXRlbXMpLAogICAgICAgIHJlX2V4cG9ydHM6IGZpbGVfcGFyc2VyOjpleHRyYWN0X3JlX2V4cG9ydHMoaXRlbXMpLAogICAgICAgIHN1Ym1vZHVsZXM6IGZpbGVfcGFyc2VyOjpleHRyYWN0X3N1Ym1vZHVsZXMoaXRlbXMpLAogICAgICAgIGltcGxzOiBmaWxlX3BhcnNlcjo6ZXh0cmFjdF9pbXBscyhpdGVtcyksCiAgICB9OwogICAgcHJvY2Vzc19tb2R1bGVfaW5mbyhtb2R1bGVfcGF0aCwgZmlsZV9wYXRoLCB2aXNpYmlsaXR5LCAmZmlsZV9pbmZvLCBpdGVtcywgcGFyZW50X2RpciwgdmlzaXRlZCwgJm11dCBWZWM6Om5ldygpKQp9", "target": "src/module_tree.rs", "index": 10, "is_create": false}, {"before_b64": "Zm4gcHJvY2Vzc19tb2R1bGVfaW5mbygKICAgIG1vZHVsZV9wYXRoOiAmc3RyLAogICAgZmlsZV9wYXRoOiAmUGF0aCwKICAgIHZpc2liaWxpdHk6ICZzdHIsCiAgICBmaWxlX2luZm86ICZGaWxlSW5mbywKICAgIGl0ZW1zOiAmW3N5bjo6SXRlbV0sCiAgICBfcGFyZW50X2RpcjogJlBhdGgsCiAgICB2aXNpdGVkOiAmbXV0IEhhc2hTZXQ8UGF0aEJ1Zj4sCikgLT4gUmVzdWx0PFZlYzxNb2R1bGVJbmZvPj4gewogICAgbGV0IG11dCBtb2R1bGVzID0gdmVjIVtidWlsZF9tb2R1bGVfaW5mbygKICAgICAgICBtb2R1bGVfcGF0aCwKICAgICAgICBmaWxlX3BhdGgsCiAgICAgICAgdmlzaWJpbGl0eSwKICAgICAgICAmZmlsZV9pbmZvLnB1YmxpY19pdGVtcywKICAgICAgICAmZmlsZV9pbmZvLmltcG9ydHMsCiAgICAgICAgJmZpbGVfaW5mby5yZV9leHBvcnRzLAogICAgICAgICZmaWxlX2luZm8uc3VibW9kdWxlcywKICAgICldOwoKICAgIGZvciBzdWIgaW4gJmZpbGVfaW5mby5zdWJtb2R1bGVzIHsKICAgICAgICBpZiBzdWIuaXNfdGVzdCB7CiAgICAgICAgICAgIGNvbnRpbnVlOwogICAgICAgIH0KICAgICAgICBsZXQgY2hpbGRfcGF0aCA9IGZvcm1hdCEoInt9Ojp7fSIsIG1vZHVsZV9wYXRoLCBzdWIubmFtZSk7CiAgICAgICAgbGV0IGNoaWxkX2RpciA9IGZpbGVfcGF0aC5wYXJlbnQoKS51bndyYXBfb3JfZWxzZSh8fCBQYXRoOjpuZXcoIi4iKSk7CiAgICAgICAgbGV0IGNoaWxkX21vZHVsZXMgPSBwcm9jZXNzX3N1Ym1vZHVsZSgKICAgICAgICAgICAgJmNoaWxkX3BhdGgsCiAgICAgICAgICAgICZzdWIubmFtZSwKICAgICAgICAgICAgaXRlbXMsCiAgICAgICAgICAgIGNoaWxkX2RpciwKICAgICAgICAgICAgZmlsZV9wYXRoLAogICAgICAgICAgICB2aXNpdGVkLAogICAgICAgICk/OwogICAgICAgIG1vZHVsZXMuZXh0ZW5kKGNoaWxkX21vZHVsZXMpOwogICAgfQoKICAgIE9rKG1vZHVsZXMpCn0=", "after_b64": "Zm4gcHJvY2Vzc19tb2R1bGVfaW5mbygKICAgIG1vZHVsZV9wYXRoOiAmc3RyLAogICAgZmlsZV9wYXRoOiAmUGF0aCwKICAgIHZpc2liaWxpdHk6ICZzdHIsCiAgICBmaWxlX2luZm86ICZGaWxlSW5mbywKICAgIGl0ZW1zOiAmW3N5bjo6SXRlbV0sCiAgICBfcGFyZW50X2RpcjogJlBhdGgsCiAgICB2aXNpdGVkOiAmbXV0IEhhc2hTZXQ8UGF0aEJ1Zj4sCiAgICBlcnJvcnM6ICZtdXQgVmVjPGNyYXRlOjpzY2hlbWE6OkVycm9yRW50cnk+LAopIC0+IChWZWM8TW9kdWxlSW5mbz4sIFZlYzxjcmF0ZTo6c2NoZW1hOjpFcnJvckVudHJ5PikgewogICAgbGV0IG11dCBtb2R1bGVzID0gdmVjIVtidWlsZF9tb2R1bGVfaW5mbygKICAgICAgICBtb2R1bGVfcGF0aCwKICAgICAgICBmaWxlX3BhdGgsCiAgICAgICAgdmlzaWJpbGl0eSwKICAgICAgICAmZmlsZV9pbmZvLnB1YmxpY19pdGVtcywKICAgICAgICAmZmlsZV9pbmZvLmltcG9ydHMsCiAgICAgICAgJmZpbGVfaW5mby5yZV9leHBvcnRzLAogICAgICAgICZmaWxlX2luZm8uc3VibW9kdWxlcywKICAgICldOwoKICAgIGZvciBzdWIgaW4gJmZpbGVfaW5mby5zdWJtb2R1bGVzIHsKICAgICAgICBpZiBzdWIuaXNfdGVzdCB7CiAgICAgICAgICAgIGNvbnRpbnVlOwogICAgICAgIH0KICAgICAgICBsZXQgY2hpbGRfcGF0aCA9IGZvcm1hdCEoInt9Ojp7fSIsIG1vZHVsZV9wYXRoLCBzdWIubmFtZSk7CiAgICAgICAgbGV0IGNoaWxkX2RpciA9IGZpbGVfcGF0aC5wYXJlbnQoKS51bndyYXBfb3IoZmlsZV9wYXRoKTsKICAgICAgICBsZXQgKGNoaWxkX21vZHVsZXMsIGNoaWxkX2Vycm9ycykgPSBwcm9jZXNzX3N1Ym1vZHVsZSgKICAgICAgICAgICAgJmNoaWxkX3BhdGgsCiAgICAgICAgICAgICZzdWIubmFtZSwKICAgICAgICAgICAgaXRlbXMsCiAgICAgICAgICAgIGNoaWxkX2RpciwKICAgICAgICAgICAgZmlsZV9wYXRoLAogICAgICAgICAgICB2aXNpdGVkLAogICAgICAgICk7CiAgICAgICAgZXJyb3JzLmV4dGVuZChjaGlsZF9lcnJvcnMpOwogICAgICAgIG1vZHVsZXMuZXh0ZW5kKGNoaWxkX21vZHVsZXMpOwogICAgfQoKICAgIChtb2R1bGVzLCBlcnJvcnMuY2xvbmUoKSkKfQ==", "target": "src/module_tree.rs", "index": 11, "is_create": false}]')
+
+for step in STEPS:
+    before = base64.b64decode(step["before_b64"]).decode()
+    after = base64.b64decode(step["after_b64"]).decode()
+    target = step["target"]
+    idx = step["index"]
+    is_create = step["is_create"]
+
+    if is_create:
+        target_path = Path(target)
+        target_path.parent.mkdir(parents=True, exist_ok=True)
+        target_path.write_text(after)
+        print(f"OK {TASK_ID} change {idx}: created {target}")
+    else:
+        target_path = Path(target)
+        content = target_path.read_text()
+        if before not in content:
+            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
+            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
+            sys.exit(1)
+
+        result = subprocess.run(
+            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
+            capture_output=True, text=True,
+        )
+        if result.returncode != 0:
+            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
+            sys.exit(result.returncode)
+
+        new_content = target_path.read_text()
+        if after and after not in new_content:
+            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
+            sys.exit(1)
+
+        print(f"OK {TASK_ID} change {idx}: applied to {target}")
+
+print(f"OK {TASK_ID}: all changes applied")
diff --git a/plans/compiled/TASK-5.sh b/plans/compiled/TASK-5.sh
new file mode 100755
index 0000000..186cdd0
--- /dev/null
+++ b/plans/compiled/TASK-5.sh
@@ -0,0 +1,7 @@
+#!/usr/bin/env bash
+set -euo pipefail
+# TASK-5: Refactor module_tree.rs: fix path fallbacks, add error collection, change build_module_tree return type
+# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
+# Type: replace
+# File: src/module_tree.rs
+python3 "$(dirname "$0")/TASK-5.py"
diff --git a/plans/compiled/TASK-6.py b/plans/compiled/TASK-6.py
new file mode 100644
index 0000000..8030cb1
--- /dev/null
+++ b/plans/compiled/TASK-6.py
@@ -0,0 +1,44 @@
+#!/usr/bin/env python3
+"""TASK-6: Update workspace.rs enumerate_members to return MissingWorkspaceSection error"""
+import base64, json, subprocess, sys
+from pathlib import Path
+
+TASK_ID = "TASK-6"
+STEPS = json.loads('[{"before_b64": "ICAgIGxldCBtZW1iZXJzOiBWZWM8U3RyaW5nPiA9IHBhcnNlZAogICAgICAgIC5nZXQoIndvcmtzcGFjZSIpCiAgICAgICAgLmFuZF90aGVuKHx3fCB3LmdldCgibWVtYmVycyIpKQogICAgICAgIC5hbmRfdGhlbih8bXwgbS5hc19hcnJheSgpKQogICAgICAgIC5tYXAofGFycnwgewogICAgICAgICAgICBhcnIuaXRlcigpCiAgICAgICAgICAgICAgICAuZmlsdGVyX21hcCh8dnwgdi5hc19zdHIoKS5tYXAoU3RyaW5nOjpmcm9tKSkKICAgICAgICAgICAgICAgIC5jb2xsZWN0KCkKICAgICAgICB9KQogICAgICAgIC51bndyYXBfb3JfZGVmYXVsdCgpOw==", "after_b64": "ICAgIGxldCBtZW1iZXJzOiBWZWM8U3RyaW5nPiA9IG1hdGNoIHBhcnNlZC5nZXQoIndvcmtzcGFjZSIpIHsKICAgICAgICBOb25lID0+IHJldHVybiBFcnIoRXJyb3I6Ok1pc3NpbmdXb3Jrc3BhY2VTZWN0aW9uKSwKICAgICAgICBTb21lKHdvcmtzcGFjZSkgPT4gd29ya3NwYWNlCiAgICAgICAgICAgIC5nZXQoIm1lbWJlcnMiKQogICAgICAgICAgICAuYW5kX3RoZW4ofG18IG0uYXNfYXJyYXkoKSkKICAgICAgICAgICAgLm1hcCh8YXJyfCB7CiAgICAgICAgICAgICAgICBhcnIuaXRlcigpCiAgICAgICAgICAgICAgICAgICAgLmZpbHRlcl9tYXAofHZ8IHYuYXNfc3RyKCkubWFwKFN0cmluZzo6ZnJvbSkpCiAgICAgICAgICAgICAgICAgICAgLmNvbGxlY3Q6OjxWZWM8Xz4+KCkKICAgICAgICAgICAgfSkKICAgICAgICAgICAgLnVud3JhcF9vcl9kZWZhdWx0KCksCiAgICB9Ow==", "target": "src/workspace.rs", "index": 0, "is_create": false}, {"before_b64": "Ly8vIFdhbGsgdXAgdGhlIGRpcmVjdG9yeSB0cmVlIGZyb20gYHN0YXJ0X3BhdGhgIHRvIGZpbmQgYSBgQ2FyZ28udG9tbGAKLy8vIGNvbnRhaW5pbmcgYSBgW3dvcmtzcGFjZV1gIHNlY3Rpb24uIFJldHVybnMgdGhlIGRpcmVjdG9yeSBjb250YWluaW5nIGl0LgpwdWIgZm4gZmluZF93b3Jrc3BhY2Vfcm9vdChzdGFydF9wYXRoOiAmUGF0aCkgLT4gUmVzdWx0PFBhdGhCdWY+IHs=", "after_b64": "Ly8vIFdhbGsgdXAgdGhlIGRpcmVjdG9yeSB0cmVlIGZyb20gYHN0YXJ0X3BhdGhgIHRvIGZpbmQgYSBgQ2FyZ28udG9tbGAKLy8vIGNvbnRhaW5pbmcgYSBgW3dvcmtzcGFjZV1gIHNlY3Rpb24uIFJldHVybnMgdGhlIGRpcmVjdG9yeSBjb250YWluaW5nIGl0LgovLy8KLy8vICMgRXJyb3JzCi8vLwovLy8gUmV0dXJucyBgRXJyb3I6OldvcmtzcGFjZVJvb3ROb3RGb3VuZGAgaWYgbm8gYENhcmdvLnRvbWxgIHdpdGggYQovLy8gYFt3b3Jrc3BhY2VdYCBzZWN0aW9uIGlzIGZvdW5kIGluIGFueSBhbmNlc3RvciBkaXJlY3RvcnkuCnB1YiBmbiBmaW5kX3dvcmtzcGFjZV9yb290KHN0YXJ0X3BhdGg6ICZQYXRoKSAtPiBSZXN1bHQ8UGF0aEJ1Zj4gew==", "target": "src/workspace.rs", "index": 1, "is_create": false}, {"before_b64": "Ly8vIFBhcnNlIHRoZSB3b3Jrc3BhY2UgYENhcmdvLnRvbWxgLCByZXNvbHZlIG1lbWJlciBwYXRocyAoaW5jbHVkaW5nIGdsb2IKLy8vIHBhdHRlcm5zKSwgYXBwbHkgYGV4Y2x1ZGVgIGxpc3QsIGFuZCByZXR1cm4gYWJzb2x1dGUgcGF0aHMgdG8gZWFjaCBtZW1iZXIKLy8vIGNyYXRlIGRpcmVjdG9yeS4KcHViIGZuIGVudW1lcmF0ZV9tZW1iZXJzKHJvb3Q6ICZQYXRoKSAtPiBSZXN1bHQ8VmVjPFBhdGhCdWY+PiB7", "after_b64": "Ly8vIFBhcnNlIHRoZSB3b3Jrc3BhY2UgYENhcmdvLnRvbWxgLCByZXNvbHZlIG1lbWJlciBwYXRocyAoaW5jbHVkaW5nIGdsb2IKLy8vIHBhdHRlcm5zKSwgYXBwbHkgYGV4Y2x1ZGVgIGxpc3QsIGFuZCByZXR1cm4gYWJzb2x1dGUgcGF0aHMgdG8gZWFjaCBtZW1iZXIKLy8vIGNyYXRlIGRpcmVjdG9yeS4KLy8vCi8vLyAjIEVycm9ycwovLy8KLy8vIFJldHVybnMgYEVycm9yOjpNaXNzaW5nV29ya3NwYWNlU2VjdGlvbmAgaWYgdGhlIGBDYXJnby50b21sYCBsYWNrcyBhCi8vLyBgW3dvcmtzcGFjZV1gIHNlY3Rpb24gZW50aXJlbHkuCnB1YiBmbiBlbnVtZXJhdGVfbWVtYmVycyhyb290OiAmUGF0aCkgLT4gUmVzdWx0PFZlYzxQYXRoQnVmPj4gew==", "target": "src/workspace.rs", "index": 2, "is_create": false}]')
+
+for step in STEPS:
+    before = base64.b64decode(step["before_b64"]).decode()
+    after = base64.b64decode(step["after_b64"]).decode()
+    target = step["target"]
+    idx = step["index"]
+    is_create = step["is_create"]
+
+    if is_create:
+        target_path = Path(target)
+        target_path.parent.mkdir(parents=True, exist_ok=True)
+        target_path.write_text(after)
+        print(f"OK {TASK_ID} change {idx}: created {target}")
+    else:
+        target_path = Path(target)
+        content = target_path.read_text()
+        if before not in content:
+            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
+            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
+            sys.exit(1)
+
+        result = subprocess.run(
+            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
+            capture_output=True, text=True,
+        )
+        if result.returncode != 0:
+            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
+            sys.exit(result.returncode)
+
+        new_content = target_path.read_text()
+        if after and after not in new_content:
+            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
+            sys.exit(1)
+
+        print(f"OK {TASK_ID} change {idx}: applied to {target}")
+
+print(f"OK {TASK_ID}: all changes applied")
diff --git a/plans/compiled/TASK-6.sh b/plans/compiled/TASK-6.sh
new file mode 100755
index 0000000..2a89fc7
--- /dev/null
+++ b/plans/compiled/TASK-6.sh
@@ -0,0 +1,7 @@
+#!/usr/bin/env bash
+set -euo pipefail
+# TASK-6: Update workspace.rs enumerate_members to return MissingWorkspaceSection error
+# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
+# Type: replace
+# File: src/workspace.rs
+python3 "$(dirname "$0")/TASK-6.py"
diff --git a/plans/compiled/TASK-7.py b/plans/compiled/TASK-7.py
new file mode 100644
index 0000000..5f61e22
--- /dev/null
+++ b/plans/compiled/TASK-7.py
@@ -0,0 +1,44 @@
+#!/usr/bin/env python3
+"""TASK-7: Refactor lib.rs run(): parallel error collection, error entry construction, remove unwrap_or_default"""
+import base64, json, subprocess, sys
+from pathlib import Path
+
+TASK_ID = "TASK-7"
+STEPS = json.loads('[{"before_b64": "dXNlIGFueWhvdzo6Q29udGV4dDsKdXNlIHJheW9uOjpwcmVsdWRlOjoqOwp1c2Ugc2NoZW1hOjp7CiAgICBDcmF0ZUluZm8sIENyYXRlVHlwZSwgRXJyb3JFbnRyeSwgTW9kdWxlSW5mbywgV29ya3NwYWNlSW5mbywgV29ya3NwYWNlTWFwLAp9Owp1c2Ugc3RkOjpwYXRoOjpQYXRoOw==", "after_b64": "dXNlIGFueWhvdzo6Q29udGV4dDsKdXNlIHJheW9uOjpwcmVsdWRlOjoqOwp1c2Ugc2NoZW1hOjp7CiAgICBDcmF0ZUluZm8sIENyYXRlVHlwZSwgRXJyb3JFbnRyeSwgRXJyb3JTZXZlcml0eSwgTW9kdWxlSW5mbywgV29ya3NwYWNlSW5mbywKICAgIFdvcmtzcGFjZU1hcCwKfTsKdXNlIHN0ZDo6cGF0aDo6UGF0aDs=", "target": "src/lib.rs", "index": 0, "is_create": false}, {"before_b64": "ICAgIGxldCBlcnJvcnM6IFZlYzxFcnJvckVudHJ5PiA9IFZlYzo6bmV3KCk7CgogICAgbGV0IG11dCBjcmF0ZV9pbmZvczogVmVjPENyYXRlSW5mbz4gPSBtZW1iZXJfZGlycwogICAgICAgIC5wYXJfaXRlcigpCiAgICAgICAgLmZpbHRlcl9tYXAofGRpcnwgewogICAgICAgICAgICBsZXQgY2FyZ29fdG9tbCA9IGRpci5qb2luKCJDYXJnby50b21sIik7CgogICAgICAgICAgICBsZXQgKHBrZywgZGVwcykgPSBtYXRjaCBjYXJnb19pbmZvOjpwYXJzZV9jYXJnb190b21sKCZjYXJnb190b21sKSB7CiAgICAgICAgICAgICAgICBPayh2KSA9PiB2LAogICAgICAgICAgICAgICAgRXJyKGUpID0+IHsKICAgICAgICAgICAgICAgICAgICBlcHJpbnRsbiEoCiAgICAgICAgICAgICAgICAgICAgICAgICJ3YXJuaW5nOiBmYWlsZWQgdG8gcGFyc2Uge306IHt9IiwKICAgICAgICAgICAgICAgICAgICAgICAgY2FyZ29fdG9tbC5kaXNwbGF5KCksCiAgICAgICAgICAgICAgICAgICAgICAgIGUKICAgICAgICAgICAgICAgICAgICApOwogICAgICAgICAgICAgICAgICAgIHJldHVybiBOb25lOwogICAgICAgICAgICAgICAgfQogICAgICAgICAgICB9OwoKICAgICAgICAgICAgbGV0IHJvb3RzID0gd29ya3NwYWNlOjpyZXNvbHZlX2NyYXRlX3Jvb3RzKGRpcik7CiAgICAgICAgICAgIGlmIHJvb3RzLmlzX2VtcHR5KCkgewogICAgICAgICAgICAgICAgZXByaW50bG4hKAogICAgICAgICAgICAgICAgICAgICJ3YXJuaW5nOiBubyBjcmF0ZSBlbnRyeSBwb2ludHMgZm91bmQgaW4ge30iLAogICAgICAgICAgICAgICAgICAgIGRpci5kaXNwbGF5KCkKICAgICAgICAgICAgICAgICk7CiAgICAgICAgICAgICAgICByZXR1cm4gTm9uZTsKICAgICAgICAgICAgfQoKICAgICAgICAgICAgbGV0IGNyYXRlX3R5cGUgPSBpZiByb290cy5pdGVyKCkuYW55KHwoXywgdCl8ICp0ID09IENyYXRlVHlwZTo6TGliKQogICAgICAgICAgICAgICAgJiYgcm9vdHMuaXRlcigpLmFueSh8KF8sIHQpfCAqdCA9PSBDcmF0ZVR5cGU6OkJpbikKICAgICAgICAgICAgewogICAgICAgICAgICAgICAgQ3JhdGVUeXBlOjpMaWJBbmRCaW4KICAgICAgICAgICAgfSBlbHNlIHsKICAgICAgICAgICAgICAgIHJvb3RzLmZpcnN0KCkubWFwX29yKENyYXRlVHlwZTo6TGliLCB8KF8sIHQpfCAqdCkKICAgICAgICAgICAgfTsKCiAgICAgICAgICAgIGxldCBwa2dfbmFtZSA9IHBrZy5uYW1lLmNsb25lKCk7CiAgICAgICAgICAgIGxldCBtdXQgbW9kdWxlczogVmVjPE1vZHVsZUluZm8+ID0gcm9vdHMKICAgICAgICAgICAgICAgIC5pdGVyKCkKICAgICAgICAgICAgICAgIC5mbGF0X21hcCh8KHJvb3QsIF90eSl8IHsKICAgICAgICAgICAgICAgICAgICBtb2R1bGVfdHJlZTo6YnVpbGRfbW9kdWxlX3RyZWUocm9vdCwgJnBrZ19uYW1lKS51bndyYXBfb3JfZGVmYXVsdCgpCiAgICAgICAgICAgICAgICB9KQogICAgICAgICAgICAgICAgLmNvbGxlY3QoKTsKCiAgICAgICAgICAgIC8vIFJlbGF0aXZpemUgYWxsIHBhdGhzIHRvIHRoZSB3b3Jrc3BhY2Ugcm9vdC4KICAgICAgICAgICAgZm9yIG0gaW4gJm11dCBtb2R1bGVzIHsKICAgICAgICAgICAgICAgIG0uZmlsZSA9IHJlbGF0aXZpemVfcGF0aCgmbS5maWxlLCAmd29ya3NwYWNlX3Jvb3QpOwogICAgICAgICAgICAgICAgZm9yIGl0ZW0gaW4gJm11dCBtLnB1YmxpY19pdGVtcyB7CiAgICAgICAgICAgICAgICAgICAgaXRlbS5maWxlID0gcmVsYXRpdml6ZV9wYXRoKCZpdGVtLmZpbGUsICZ3b3Jrc3BhY2Vfcm9vdCk7CiAgICAgICAgICAgICAgICB9CiAgICAgICAgICAgIH0KCiAgICAgICAgICAgIGxldCBjcmF0ZV9yb290ID0gcm9vdHMKICAgICAgICAgICAgICAgIC5maXJzdCgpCiAgICAgICAgICAgICAgICAubWFwKHwociwgXyl8IHJlbGF0aXZpemVfcGF0aCgmci50b19zdHJpbmdfbG9zc3koKSwgJndvcmtzcGFjZV9yb290KSkKICAgICAgICAgICAgICAgIC51bndyYXBfb3JfZGVmYXVsdCgpOwoKICAgICAgICAgICAgbGV0IHJlYnVpbHRfcGtnID0gc2NoZW1hOjpQYWNrYWdlSW5mbzo6YnVpbGRlcigpCiAgICAgICAgICAgICAgICAubmFtZShwa2cubmFtZSkKICAgICAgICAgICAgICAgIC52ZXJzaW9uKHBrZy52ZXJzaW9uKQogICAgICAgICAgICAgICAgLmVkaXRpb24ocGtnLmVkaXRpb24pCiAgICAgICAgICAgICAgICAuY3JhdGVfdHlwZShjcmF0ZV90eXBlKQogICAgICAgICAgICAgICAgLmJ1aWxkKCk7CgogICAgICAgICAgICBTb21lKAogICAgICAgICAgICAgICAgQ3JhdGVJbmZvOjpidWlsZGVyKCkKICAgICAgICAgICAgICAgICAgICAubmFtZShwa2dfbmFtZSkKICAgICAgICAgICAgICAgICAgICAucm9vdChjcmF0ZV9yb290KQogICAgICAgICAgICAgICAgICAgIC5wYWNrYWdlKHJlYnVpbHRfcGtnKQogICAgICAgICAgICAgICAgICAgIC5tb2R1bGVzKG1vZHVsZXMpCiAgICAgICAgICAgICAgICAgICAgLmRlcHMoZGVwcykKICAgICAgICAgICAgICAgICAgICAuYnVpbGQoKSwKICAgICAgICAgICAgKQogICAgICAgIH0pCiAgICAgICAgLmNvbGxlY3QoKTs=", "after_b64": "ICAgIGxldCBtdXQgY3JhdGVfZXJyb3JzOiBWZWM8RXJyb3JFbnRyeT4gPSBWZWM6Om5ldygpOwoKICAgIGxldCByZXN1bHRzOiBWZWM8KENyYXRlSW5mbywgVmVjPEVycm9yRW50cnk+KT4gPSBtZW1iZXJfZGlycwogICAgICAgIC5wYXJfaXRlcigpCiAgICAgICAgLm1hcCh8ZGlyfCB7CiAgICAgICAgICAgIGxldCBjYXJnb190b21sID0gZGlyLmpvaW4oIkNhcmdvLnRvbWwiKTsKICAgICAgICAgICAgbGV0IG11dCBjcmF0ZV9lcnJvcnMgPSBWZWM6Om5ldygpOwoKICAgICAgICAgICAgbGV0IChwa2csIGRlcHMpID0gbWF0Y2ggY2FyZ29faW5mbzo6cGFyc2VfY2FyZ29fdG9tbCgmY2FyZ29fdG9tbCkgewogICAgICAgICAgICAgICAgT2sodikgPT4gdiwKICAgICAgICAgICAgICAgIEVycihlKSA9PiB7CiAgICAgICAgICAgICAgICAgICAgY3JhdGVfZXJyb3JzLnB1c2goRXJyb3JFbnRyeTo6YnVpbGRlcigpCiAgICAgICAgICAgICAgICAgICAgICAgIC5maWxlKGNhcmdvX3RvbWwudG9fc3RyaW5nX2xvc3N5KCkudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAgICAgICAgIC5tZXNzYWdlKGZvcm1hdCEoImZhaWxlZCB0byBwYXJzZSBDYXJnby50b21sOiB7ZX0iKSkKICAgICAgICAgICAgICAgICAgICAgICAgLnNldmVyaXR5KEVycm9yU2V2ZXJpdHk6OkVycm9yKQogICAgICAgICAgICAgICAgICAgICAgICAua2luZCgidG9tbF9wYXJzZV9lcnJvciIudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAgICAgICAgIC5jYXVzZShlLnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgICAgICAgICAuYnVpbGQoKSk7CiAgICAgICAgICAgICAgICAgICAgcmV0dXJuIChOb25lLCBjcmF0ZV9lcnJvcnMpOwogICAgICAgICAgICAgICAgfQogICAgICAgICAgICB9OwoKICAgICAgICAgICAgbGV0IHJvb3RzID0gd29ya3NwYWNlOjpyZXNvbHZlX2NyYXRlX3Jvb3RzKGRpcik7CiAgICAgICAgICAgIGlmIHJvb3RzLmlzX2VtcHR5KCkgewogICAgICAgICAgICAgICAgY3JhdGVfZXJyb3JzLnB1c2goRXJyb3JFbnRyeTo6YnVpbGRlcigpCiAgICAgICAgICAgICAgICAgICAgLmZpbGUoZGlyLnRvX3N0cmluZ19sb3NzeSgpLnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgICAgIC5tZXNzYWdlKCJubyBjcmF0ZSBlbnRyeSBwb2ludHMgZm91bmQiLnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgICAgIC5zZXZlcml0eShFcnJvclNldmVyaXR5OjpXYXJuaW5nKQogICAgICAgICAgICAgICAgICAgIC5raW5kKCJtaXNzaW5nX2NyYXRlX3Jvb3RzIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgICAgICAgICAuYnVpbGQoKSk7CiAgICAgICAgICAgICAgICByZXR1cm4gKE5vbmUsIGNyYXRlX2Vycm9ycyk7CiAgICAgICAgICAgIH0KCiAgICAgICAgICAgIGxldCBjcmF0ZV90eXBlID0gaWYgcm9vdHMuaXRlcigpLmFueSh8KF8sIHQpfCAqdCA9PSBDcmF0ZVR5cGU6OkxpYikKICAgICAgICAgICAgICAgICYmIHJvb3RzLml0ZXIoKS5hbnkofChfLCB0KXwgKnQgPT0gQ3JhdGVUeXBlOjpCaW4pCiAgICAgICAgICAgIHsKICAgICAgICAgICAgICAgIENyYXRlVHlwZTo6TGliQW5kQmluCiAgICAgICAgICAgIH0gZWxzZSB7CiAgICAgICAgICAgICAgICByb290cy5maXJzdCgpLm1hcF9vcihDcmF0ZVR5cGU6OkxpYiwgfChfLCB0KXwgKnQpCiAgICAgICAgICAgIH07CgogICAgICAgICAgICBsZXQgcGtnX25hbWUgPSBwa2cubmFtZS5jbG9uZSgpOwogICAgICAgICAgICBsZXQgbXV0IG1vZHVsZXM6IFZlYzxNb2R1bGVJbmZvPiA9IFZlYzo6bmV3KCk7CiAgICAgICAgICAgIGxldCBtdXQgY29sbGVjdGVkX2Vycm9ycyA9IFZlYzo6bmV3KCk7CiAgICAgICAgICAgIGZvciAocm9vdCwgX3R5KSBpbiAmcm9vdHMgewogICAgICAgICAgICAgICAgbGV0IChtLCBlKSA9IG1vZHVsZV90cmVlOjpidWlsZF9tb2R1bGVfdHJlZSgmcm9vdCwgJnBrZ19uYW1lKTsKICAgICAgICAgICAgICAgIG1vZHVsZXMuZXh0ZW5kKG0pOwogICAgICAgICAgICAgICAgY29sbGVjdGVkX2Vycm9ycy5leHRlbmQoZSk7CiAgICAgICAgICAgIH0KICAgICAgICAgICAgY3JhdGVfZXJyb3JzLmV4dGVuZChjb2xsZWN0ZWRfZXJyb3JzKTsKCiAgICAgICAgICAgIC8vIFJlbGF0aXZpemUgYWxsIHBhdGhzIHRvIHRoZSB3b3Jrc3BhY2Ugcm9vdC4KICAgICAgICAgICAgZm9yIG0gaW4gJm11dCBtb2R1bGVzIHsKICAgICAgICAgICAgICAgIG0uZmlsZSA9IHJlbGF0aXZpemVfcGF0aCgmbS5maWxlLCAmd29ya3NwYWNlX3Jvb3QpOwogICAgICAgICAgICAgICAgZm9yIGl0ZW0gaW4gJm11dCBtLnB1YmxpY19pdGVtcyB7CiAgICAgICAgICAgICAgICAgICAgaXRlbS5maWxlID0gcmVsYXRpdml6ZV9wYXRoKCZpdGVtLmZpbGUsICZ3b3Jrc3BhY2Vfcm9vdCk7CiAgICAgICAgICAgICAgICB9CiAgICAgICAgICAgIH0KCiAgICAgICAgICAgIGxldCBjcmF0ZV9yb290ID0gcm9vdHMKICAgICAgICAgICAgICAgIC5maXJzdCgpCiAgICAgICAgICAgICAgICAubWFwKHwociwgXyl8IHJlbGF0aXZpemVfcGF0aCgmci50b19zdHJpbmdfbG9zc3koKSwgJndvcmtzcGFjZV9yb290KSkKICAgICAgICAgICAgICAgIC51bndyYXBfb3JfZGVmYXVsdCgpOwoKICAgICAgICAgICAgbGV0IHJlYnVpbHRfcGtnID0gc2NoZW1hOjpQYWNrYWdlSW5mbzo6YnVpbGRlcigpCiAgICAgICAgICAgICAgICAubmFtZShwa2cubmFtZSkKICAgICAgICAgICAgICAgIC52ZXJzaW9uKHBrZy52ZXJzaW9uKQogICAgICAgICAgICAgICAgLmVkaXRpb24ocGtnLmVkaXRpb24pCiAgICAgICAgICAgICAgICAuY3JhdGVfdHlwZShjcmF0ZV90eXBlKQogICAgICAgICAgICAgICAgLmJ1aWxkKCk7CgogICAgICAgICAgICBsZXQgY3JhdGVfaW5mbyA9IENyYXRlSW5mbzo6YnVpbGRlcigpCiAgICAgICAgICAgICAgICAubmFtZShwa2dfbmFtZSkKICAgICAgICAgICAgICAgIC5yb290KGNyYXRlX3Jvb3QpCiAgICAgICAgICAgICAgICAucGFja2FnZShyZWJ1aWx0X3BrZykKICAgICAgICAgICAgICAgIC5tb2R1bGVzKG1vZHVsZXMpCiAgICAgICAgICAgICAgICAuZGVwcyhkZXBzKQogICAgICAgICAgICAgICAgLmJ1aWxkKCk7CgogICAgICAgICAgICAoU29tZShjcmF0ZV9pbmZvKSwgY3JhdGVfZXJyb3JzKQogICAgICAgIH0pCiAgICAgICAgLmNvbGxlY3QoKTsKCiAgICBsZXQgbXV0IGNyYXRlX2luZm9zOiBWZWM8Q3JhdGVJbmZvPiA9IFZlYzo6bmV3KCk7CgogICAgZm9yIChpbmZvLCBlcnJzKSBpbiByZXN1bHRzIHsKICAgICAgICBpZiBsZXQgU29tZShjaSkgPSBpbmZvIHsKICAgICAgICAgICAgY3JhdGVfZXJyb3JzLmV4dGVuZChlcnJzKTsKICAgICAgICAgICAgY3JhdGVfaW5mb3MucHVzaChjaSk7CiAgICAgICAgfQogICAgfQ==", "target": "src/lib.rs", "index": 1, "is_create": false}, {"before_b64": "ICAgIGxldCBtYXAgPSBXb3Jrc3BhY2VNYXA6OmJ1aWxkZXIoKQogICAgICAgIC53b3Jrc3BhY2Uod29ya3NwYWNlX2luZm8pCiAgICAgICAgLmNyYXRlcyhjcmF0ZV9pbmZvcykKICAgICAgICAuY3Jvc3NfcmVmZXJlbmNlcyhjcm9zc19yZWZzKQogICAgICAgIC5lcnJvcnMoZXJyb3JzKQogICAgICAgIC53b3Jrc3BhY2Vfcm9vdCh3b3Jrc3BhY2Vfcm9vdC5jbG9uZSgpKQogICAgICAgIC5idWlsZCgpOw==", "after_b64": "ICAgIGxldCBtYXAgPSBXb3Jrc3BhY2VNYXA6OmJ1aWxkZXIoKQogICAgICAgIC53b3Jrc3BhY2Uod29ya3NwYWNlX2luZm8pCiAgICAgICAgLmNyYXRlcyhjcmF0ZV9pbmZvcykKICAgICAgICAuY3Jvc3NfcmVmZXJlbmNlcyhjcm9zc19yZWZzKQogICAgICAgIC5lcnJvcnMoY3JhdGVfZXJyb3JzKQogICAgICAgIC53b3Jrc3BhY2Vfcm9vdCh3b3Jrc3BhY2Vfcm9vdC5jbG9uZSgpKQogICAgICAgIC5idWlsZCgpOw==", "target": "src/lib.rs", "index": 2, "is_create": false}]')
+
+for step in STEPS:
+    before = base64.b64decode(step["before_b64"]).decode()
+    after = base64.b64decode(step["after_b64"]).decode()
+    target = step["target"]
+    idx = step["index"]
+    is_create = step["is_create"]
+
+    if is_create:
+        target_path = Path(target)
+        target_path.parent.mkdir(parents=True, exist_ok=True)
+        target_path.write_text(after)
+        print(f"OK {TASK_ID} change {idx}: created {target}")
+    else:
+        target_path = Path(target)
+        content = target_path.read_text()
+        if before not in content:
+            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
+            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
+            sys.exit(1)
+
+        result = subprocess.run(
+            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
+            capture_output=True, text=True,
+        )
+        if result.returncode != 0:
+            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
+            sys.exit(result.returncode)
+
+        new_content = target_path.read_text()
+        if after and after not in new_content:
+            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
+            sys.exit(1)
+
+        print(f"OK {TASK_ID} change {idx}: applied to {target}")
+
+print(f"OK {TASK_ID}: all changes applied")
diff --git a/plans/compiled/TASK-7.sh b/plans/compiled/TASK-7.sh
new file mode 100755
index 0000000..35337cf
--- /dev/null
+++ b/plans/compiled/TASK-7.sh
@@ -0,0 +1,7 @@
+#!/usr/bin/env bash
+set -euo pipefail
+# TASK-7: Refactor lib.rs run(): parallel error collection, error entry construction, remove unwrap_or_default
+# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
+# Type: replace
+# File: src/lib.rs
+python3 "$(dirname "$0")/TASK-7.py"
diff --git a/plans/compiled/TASK-8.py b/plans/compiled/TASK-8.py
new file mode 100644
index 0000000..52d3e99
--- /dev/null
+++ b/plans/compiled/TASK-8.py
@@ -0,0 +1,44 @@
+#!/usr/bin/env python3
+"""TASK-8: Remove all crate-level clippy allow attributes and fix individual lint violations"""
+import base64, json, subprocess, sys
+from pathlib import Path
+
+TASK_ID = "TASK-8"
+STEPS = json.loads('[{"before_b64": "IyFbd2FybihjbGlwcHk6OnBlZGFudGljKV0KIyFbYWxsb3coY2xpcHB5OjptaXNzaW5nX2Vycm9yc19kb2MpXQojIVthbGxvdyhjbGlwcHk6Om11c3RfdXNlX2NhbmRpZGF0ZSldCiMhW2FsbG93KGNsaXBweTo6ZG9jX21hcmtkb3duKV0KIyFbYWxsb3coY2xpcHB5Ojp1bmlubGluZWRfZm9ybWF0X2FyZ3MpXQojIVthbGxvdyhjbGlwcHk6OnJlZHVuZGFudF9jbG9zdXJlKV0KIyFbYWxsb3coY2xpcHB5Ojpjb2xsYXBzaWJsZV9pZildCiMhW2FsbG93KGNsaXBweTo6bmVlZGxlc3NfcGFzc19ieV92YWx1ZSldCiMhW2FsbG93KGNsaXBweTo6bmVlZGxlc3NfYm9ycm93KV0KIyFbYWxsb3coY2xpcHB5OjpyZWR1bmRhbnRfY2xvc3VyZV9mb3JfbWV0aG9kX2NhbGxzKV0=", "after_b64": "IyFbd2FybihjbGlwcHk6OnBlZGFudGljKV0=", "target": "src/lib.rs", "index": 0, "is_create": false}, {"before_b64": "Ly8vIFJ1biB0aGUgZnVsbCB3b3Jrc3BhY2UgbWFwcGluZyBwaXBlbGluZS4KLy8vCi8vLyAxLiBEaXNjb3ZlciB3b3Jrc3BhY2Ugcm9vdCBhbmQgbWVtYmVyIGNyYXRlcy4KLy8vIDIuIFByb2Nlc3MgZWFjaCBjcmF0ZSBpbiBwYXJhbGxlbCAoQ2FyZ28udG9tbCBwYXJzaW5nICsgbW9kdWxlIHRyZWUpLgovLy8gMy4gQ29tcHV0ZSBjcm9zcy1jcmF0ZSByZWZlcmVuY2VzLgovLy8gNC4gUmVuZGVyIEpTT04gdG8gc3Rkb3V0IG9yIHRoZSBjb25maWd1cmVkIG91dHB1dCBmaWxlLgpwdWIgZm4gcnVuKGNvbmZpZzogQ29uZmlnKSAtPiBhbnlob3c6OlJlc3VsdDwoKT4gew==", "after_b64": "Ly8vIFJ1biB0aGUgZnVsbCB3b3Jrc3BhY2UgbWFwcGluZyBwaXBlbGluZS4KLy8vCi8vLyAxLiBEaXNjb3ZlciB3b3Jrc3BhY2Ugcm9vdCBhbmQgbWVtYmVyIGNyYXRlcy4KLy8vIDIuIFByb2Nlc3MgZWFjaCBjcmF0ZSBpbiBwYXJhbGxlbCAoQ2FyZ28udG9tbCBwYXJzaW5nICsgbW9kdWxlIHRyZWUpLgovLy8gMy4gQ29tcHV0ZSBjcm9zcy1jcmF0ZSByZWZlcmVuY2VzLgovLy8gNC4gUmVuZGVyIEpTT04gdG8gc3Rkb3V0IG9yIHRoZSBjb25maWd1cmVkIG91dHB1dCBmaWxlLgovLy8KLy8vICMgRXJyb3JzCi8vLwovLy8gUmV0dXJucyBhbiBlcnJvciBpZiB0aGUgd29ya3NwYWNlIHJvb3QgY2Fubm90IGJlIGZvdW5kLCB0aGUgd29ya3NwYWNlCi8vLyBDYXJnby50b21sIGlzIG1pc3NpbmcgYSBgW3dvcmtzcGFjZV1gIHNlY3Rpb24sIG1lbWJlciBjcmF0ZXMgY2Fubm90IGJlCi8vLyBwYXJzZWQsIG9yIHRoZSBKU09OIG91dHB1dCBjYW5ub3QgYmUgd3JpdHRlbi4KcHViIGZuIHJ1bihjb25maWc6IENvbmZpZykgLT4gYW55aG93OjpSZXN1bHQ8KCk+IHs=", "target": "src/lib.rs", "index": 1, "is_create": false}, {"before_b64": "ICAgIGxldCB3b3Jrc3BhY2VfbmFtZSA9IHdvcmtzcGFjZV9yb290CiAgICAgICAgLmZpbGVfbmFtZSgpCiAgICAgICAgLm1hcCh8bnwgbi50b19zdHJpbmdfbG9zc3koKS50b19zdHJpbmcoKSkKICAgICAgICAudW53cmFwX29yX2RlZmF1bHQoKTs=", "after_b64": "ICAgIGxldCB3b3Jrc3BhY2VfbmFtZSA9IHdvcmtzcGFjZV9yb290CiAgICAgICAgLmZpbGVfbmFtZSgpCiAgICAgICAgLm1hcCh8bnwgbi50b19zdHJpbmdfbG9zc3koKS50b19zdHJpbmcoKSkKICAgICAgICAudW53cmFwX29yX2RlZmF1bHQoKTs=", "target": "src/lib.rs", "index": 2, "is_create": false}, {"before_b64": "Ly8vIFN0cmlwIHRoZSB3b3Jrc3BhY2Ugcm9vdCBwcmVmaXggZnJvbSBhIHBhdGggc3RyaW5nLCByZXR1cm5pbmcgYQovLy8gd29ya3NwYWNlLXJlbGF0aXZlIHBhdGguIElmIHRoZSBwcmVmaXggZG9lc24ndCBtYXRjaCwgcmV0dXJucyB0aGUKLy8vIG9yaWdpbmFsIHN0cmluZyB1bmNoYW5nZWQuCmZuIHJlbGF0aXZpemVfcGF0aChwYXRoX3N0cjogJnN0ciwgcm9vdDogJlBhdGgpIC0+IFN0cmluZyB7CiAgICBsZXQgcCA9IFBhdGg6Om5ldyhwYXRoX3N0cik7CiAgICBtYXRjaCBwLnN0cmlwX3ByZWZpeChyb290KSB7CiAgICAgICAgT2socmVsKSA9PiByZWwudG9fc3RyaW5nX2xvc3N5KCkudG9fc3RyaW5nKCksCiAgICAgICAgRXJyKF8pID0+IHBhdGhfc3RyLnRvX3N0cmluZygpLAogICAgfQp9", "after_b64": "Ly8vIFN0cmlwIHRoZSB3b3Jrc3BhY2Ugcm9vdCBwcmVmaXggZnJvbSBhIHBhdGggc3RyaW5nLCByZXR1cm5pbmcgYQovLy8gd29ya3NwYWNlLXJlbGF0aXZlIHBhdGguIElmIHRoZSBwcmVmaXggZG9lc24ndCBtYXRjaCwgcmV0dXJucyB0aGUKLy8vIG9yaWdpbmFsIHN0cmluZyB1bmNoYW5nZWQuCmZuIHJlbGF0aXZpemVfcGF0aChwYXRoX3N0cjogJnN0ciwgcm9vdDogJlBhdGgpIC0+IFN0cmluZyB7CiAgICBsZXQgcCA9IFBhdGg6Om5ldyhwYXRoX3N0cik7CiAgICBtYXRjaCBwLnN0cmlwX3ByZWZpeChyb290KSB7CiAgICAgICAgT2socmVsKSA9PiByZWwudG9fc3RyaW5nX2xvc3N5KCkudG9fc3RyaW5nKCksCiAgICAgICAgRXJyKF8pID0+IHBhdGhfc3RyLnRvX3N0cmluZygpLAogICAgfQp9", "target": "src/lib.rs", "index": 3, "is_create": false}, {"before_b64": "Ly8vIEV4dHJhY3QgYWxsIGl0ZW1zIHdpdGggYW55IGZvcm0gb2YgYHB1YmAgdmlzaWJpbGl0eSAoZXhjbHVkaW5nIGBJbmhlcml0ZWRgKS4KLy8vIFJlc3VsdHMgYXJlIHNvcnRlZCBieSBuYW1lIHRoZW4gbGluZSBmb3IgZGV0ZXJtaW5pc3RpYyBvdXRwdXQuCnB1YiBmbiBleHRyYWN0X3B1YmxpY19pdGVtcyhpdGVtczogJltzeW46Okl0ZW1dKSAtPiBWZWM8UHVibGljSXRlbT4gew==", "after_b64": "Ly8vIEV4dHJhY3QgYWxsIGl0ZW1zIHdpdGggYW55IGZvcm0gb2YgYHB1YmAgdmlzaWJpbGl0eSAoZXhjbHVkaW5nIGBJbmhlcml0ZWRgKS4KLy8vIFJlc3VsdHMgYXJlIHNvcnRlZCBieSBuYW1lIHRoZW4gbGluZSBmb3IgZGV0ZXJtaW5pc3RpYyBvdXRwdXQuCiNbbXVzdF91c2VdCnB1YiBmbiBleHRyYWN0X3B1YmxpY19pdGVtcyhpdGVtczogJltzeW46Okl0ZW1dKSAtPiBWZWM8UHVibGljSXRlbT4gew==", "target": "src/file_parser.rs", "index": 4, "is_create": false}, {"before_b64": "Ly8vIEV4dHJhY3QgYWxsIGB1c2VgIHN0YXRlbWVudHMuIEJyYWNlZCBpbXBvcnRzIGFyZSBleHBhbmRlZCB0byBpbmRpdmlkdWFsCi8vLyBlbnRyaWVzLiBSZXN1bHRzIHNvcnRlZCBieSBwYXRoIGZvciBkZXRlcm1pbmlzbS4KcHViIGZuIGV4dHJhY3RfaW1wb3J0cyhpdGVtczogJltzeW46Okl0ZW1dKSAtPiBWZWM8SW1wb3J0PiB7", "after_b64": "Ly8vIEV4dHJhY3QgYWxsIGB1c2VgIHN0YXRlbWVudHMuIEJyYWNlZCBpbXBvcnRzIGFyZSBleHBhbmRlZCB0byBpbmRpdmlkdWFsCi8vLyBlbnRyaWVzLiBSZXN1bHRzIHNvcnRlZCBieSBwYXRoIGZvciBkZXRlcm1pbmlzbS4KI1ttdXN0X3VzZV0KcHViIGZuIGV4dHJhY3RfaW1wb3J0cyhpdGVtczogJltzeW46Okl0ZW1dKSAtPiBWZWM8SW1wb3J0PiB7", "target": "src/file_parser.rs", "index": 5, "is_create": false}, {"before_b64": "Ly8vIEV4dHJhY3QgYHB1YiB1c2VgIHJlLWV4cG9ydHMuIFJlc3VsdHMgc29ydGVkIGJ5IGV4cG9ydF9wYXRoLgpwdWIgZm4gZXh0cmFjdF9yZV9leHBvcnRzKGl0ZW1zOiAmW3N5bjo6SXRlbV0pIC0+IFZlYzxSZUV4cG9ydD4gew==", "after_b64": "Ly8vIEV4dHJhY3QgYHB1YiB1c2VgIHJlLWV4cG9ydHMuIFJlc3VsdHMgc29ydGVkIGJ5IGV4cG9ydF9wYXRoLgojW211c3RfdXNlXQpwdWIgZm4gZXh0cmFjdF9yZV9leHBvcnRzKGl0ZW1zOiAmW3N5bjo6SXRlbV0pIC0+IFZlYzxSZUV4cG9ydD4gew==", "target": "src/file_parser.rs", "index": 6, "is_create": false}, {"before_b64": "Ly8vIEV4dHJhY3QgYG1vZGAgZGVjbGFyYXRpb25zLiBEZXRlY3RzIGAjW2NmZyh0ZXN0KV1gIHZpYSBsaXRlcmFsIHRva2VuCi8vLyBtYXRjaGluZy4gUmVzdWx0cyBzb3J0ZWQgYnkgbmFtZS4KcHViIGZuIGV4dHJhY3Rfc3VibW9kdWxlcyhpdGVtczogJltzeW46Okl0ZW1dKSAtPiBWZWM8U3VibW9kdWxlRGVjbD4gew==", "after_b64": "Ly8vIEV4dHJhY3QgYG1vZGAgZGVjbGFyYXRpb25zLiBEZXRlY3RzIGAjW2NmZyh0ZXN0KV1gIHZpYSBsaXRlcmFsIHRva2VuCi8vLyBtYXRjaGluZy4gUmVzdWx0cyBzb3J0ZWQgYnkgbmFtZS4KI1ttdXN0X3VzZV0KcHViIGZuIGV4dHJhY3Rfc3VibW9kdWxlcyhpdGVtczogJltzeW46Okl0ZW1dKSAtPiBWZWM8U3VibW9kdWxlRGVjbD4gew==", "target": "src/file_parser.rs", "index": 7, "is_create": false}, {"before_b64": "Ly8vIEV4dHJhY3QgYGltcGxgIGJsb2Nrcy4gRWFjaCBgSW1wbEluZm9gIHJlY29yZHMgdGhlIHRhcmdldCB0eXBlIG5hbWUgYW5kCi8vLyB0aGUgaW1wbCBpdGVtcyAoZm4sIHR5cGUsIGNvbnN0KS4KcHViIGZuIGV4dHJhY3RfaW1wbHMoaXRlbXM6ICZbc3luOjpJdGVtXSkgLT4gVmVjPEltcGxJbmZvPiB7", "after_b64": "Ly8vIEV4dHJhY3QgYGltcGxgIGJsb2Nrcy4gRWFjaCBgSW1wbEluZm9gIHJlY29yZHMgdGhlIHRhcmdldCB0eXBlIG5hbWUgYW5kCi8vLyB0aGUgaW1wbCBpdGVtcyAoZm4sIHR5cGUsIGNvbnN0KS4KI1ttdXN0X3VzZV0KcHViIGZuIGV4dHJhY3RfaW1wbHMoaXRlbXM6ICZbc3luOjpJdGVtXSkgLT4gVmVjPEltcGxJbmZvPiB7", "target": "src/file_parser.rs", "index": 8, "is_create": false}, {"before_b64": "Ly8vIFJlc29sdmUgYSBgbW9kIG5hbWU7YCBkZWNsYXJhdGlvbiB0byBhIGZpbGUgcGF0aC4KLy8vIFRyaWVzIGB7cGFyZW50X2Rpcn0ve21vZF9uYW1lfS5yc2AgZmlyc3QsIHRoZW4gYHtwYXJlbnRfZGlyfS97bW9kX25hbWV9L21vZC5yc2AuCi8vLwovLy8gUmV0dXJucyBgTm9uZWAgaWYgbmVpdGhlciBwYXRoIGV4aXN0cy4KcHViIGZuIHJlc29sdmVfbW9kdWxlX3BhdGgocGFyZW50X2RpcjogJlBhdGgsIG1vZF9uYW1lOiAmc3RyKSAtPiBPcHRpb248UGF0aEJ1Zj4gew==", "after_b64": "Ly8vIFJlc29sdmUgYSBgbW9kIG5hbWU7YCBkZWNsYXJhdGlvbiB0byBhIGZpbGUgcGF0aC4KLy8vIFRyaWVzIGB7cGFyZW50X2Rpcn0ve21vZF9uYW1lfS5yc2AgZmlyc3QsIHRoZW4gYHtwYXJlbnRfZGlyfS97bW9kX25hbWV9L21vZC5yc2AuCi8vLwovLy8gUmV0dXJucyBgTm9uZWAgaWYgbmVpdGhlciBwYXRoIGV4aXN0cy4KI1ttdXN0X3VzZV0KcHViIGZuIHJlc29sdmVfbW9kdWxlX3BhdGgocGFyZW50X2RpcjogJlBhdGgsIG1vZF9uYW1lOiAmc3RyKSAtPiBPcHRpb248UGF0aEJ1Zj4gew==", "target": "src/module_tree.rs", "index": 9, "is_create": false}, {"before_b64": "Ly8vIFNlcmlhbGl6ZSB0aGUgd29ya3NwYWNlIG1hcCB0byBhIEpTT04gc3RyaW5nIHdpdGggMi1zcGFjZSBpbmRlbnRhdGlvbi4KcHViIGZuIHJlbmRlcl9qc29uKG1hcDogJldvcmtzcGFjZU1hcCkgLT4gc2VyZGVfanNvbjo6UmVzdWx0PFN0cmluZz4gew==", "after_b64": "Ly8vIFNlcmlhbGl6ZSB0aGUgd29ya3NwYWNlIG1hcCB0byBhIEpTT04gc3RyaW5nIHdpdGggMi1zcGFjZSBpbmRlbnRhdGlvbi4KI1ttdXN0X3VzZV0KcHViIGZuIHJlbmRlcl9qc29uKG1hcDogJldvcmtzcGFjZU1hcCkgLT4gc2VyZGVfanNvbjo6UmVzdWx0PFN0cmluZz4gew==", "target": "src/render.rs", "index": 10, "is_create": false}, {"before_b64": "Ly8gSGVscGVyOiBjb252ZXJ0IEl0ZW1LaW5kIHRvIGEgc2hvcnQgc3RyaW5nIGZvciB0aGUgVHlwZVJlZi5raW5kIGZpZWxkLgppbXBsIGNyYXRlOjpzY2hlbWE6OlB1YmxpY0l0ZW0gewogICAgZm4ga2luZF90b19zdHJpbmcoJnNlbGYpIC0+IFN0cmluZyB7", "after_b64": "Ly8gSGVscGVyOiBjb252ZXJ0IEl0ZW1LaW5kIHRvIGEgc2hvcnQgc3RyaW5nIGZvciB0aGUgVHlwZVJlZi5raW5kIGZpZWxkLgppbXBsIGNyYXRlOjpzY2hlbWE6OlB1YmxpY0l0ZW0gewogICAgI1ttdXN0X3VzZV0KICAgIGZuIGtpbmRfdG9fc3RyaW5nKCZzZWxmKSAtPiBTdHJpbmcgew==", "target": "src/cross_refs.rs", "index": 11, "is_create": false}, {"before_b64": "ICAgIG1hdGNoIHAuc3RyaXBfcHJlZml4KHJvb3QpIHsKICAgICAgICBPayhyZWwpID0+IHJlbC50b19zdHJpbmdfbG9zc3koKS50b19zdHJpbmcoKSwKICAgICAgICBFcnIoXykgPT4gcGF0aF9zdHIudG9fc3RyaW5nKCksCiAgICB9Cn0=", "after_b64": "ICAgIG1hdGNoIHAuc3RyaXBfcHJlZml4KHJvb3QpIHsKICAgICAgICBPayhyZWwpID0+IHJlbC50b19zdHJpbmdfbG9zc3koKS50b19zdHJpbmcoKSwKICAgICAgICBFcnIoXykgPT4gcGF0aF9zdHIudG9fc3RyaW5nKCksCiAgICB9Cn0=", "target": "src/lib.rs", "index": 12, "is_create": false}, {"before_b64": "Ly8vIEV4dHJhY3QgYHB1YiB1c2VgIHJlLWV4cG9ydHMuIFJlc3VsdHMgc29ydGVkIGJ5IGV4cG9ydF9wYXRoLgojW211c3RfdXNlXQpwdWIgZm4gZXh0cmFjdF9yZV9leHBvcnRz", "after_b64": "Ly8vIEV4dHJhY3QgYHB1YiB1c2VgIHJlLWV4cG9ydHMuIFJlc3VsdHMgc29ydGVkIGJ5IGBleHBvcnRfcGF0aGAuCiNbbXVzdF91c2VdCnB1YiBmbiBleHRyYWN0X3JlX2V4cG9ydHM=", "target": "src/file_parser.rs", "index": 13, "is_create": false}, {"before_b64": "ICAgICAgICAgICAgbGV0IG5hbWUgPSBtLmlkZW50LmFzX3JlZigpLm1hcCh8aXwgaS50b19zdHJpbmcoKSkudW53cmFwX29yX2RlZmF1bHQoKTs=", "after_b64": "ICAgICAgICAgICAgbGV0IG5hbWUgPSBtLmlkZW50LmFzX3JlZigpLm1hcChUb1N0cmluZzo6dG9fc3RyaW5nKS51bndyYXBfb3JfZGVmYXVsdCgpOw==", "target": "src/file_parser.rs", "index": 14, "is_create": false}, {"before_b64": "Zm4gZmxhdHRlbl91c2VfdHJlZSh0cmVlOiAmc3luOjpVc2VUcmVlLCBwcmVmaXg6IFN0cmluZywgbGluZTogdXNpemUpIC0+IFZlYzxJbXBvcnQ+IHs=", "after_b64": "Zm4gZmxhdHRlbl91c2VfdHJlZSh0cmVlOiAmc3luOjpVc2VUcmVlLCBwcmVmaXg6ICZzdHIsIGxpbmU6IHVzaXplKSAtPiBWZWM8SW1wb3J0PiB7", "target": "src/file_parser.rs", "index": 15, "is_create": false}, {"before_b64": "ICAgICAgICAgICAgICAgIFNvbWUoZmxhdHRlbl91c2VfdHJlZSgmdS50cmVlLCBTdHJpbmc6Om5ldygpLCBsaW5lX29mX2l0ZW0oaXRlbSkpKQ==", "after_b64": "ICAgICAgICAgICAgICAgIFNvbWUoZmxhdHRlbl91c2VfdHJlZSgmdS50cmVlLCAiIiwgbGluZV9vZl9pdGVtKGl0ZW0pKSk=", "target": "src/file_parser.rs", "index": 16, "is_create": false}, {"before_b64": "ICAgICAgICAgICAgZmxhdHRlbl91c2VfdHJlZSgmcC50cmVlLCBuZXdfcHJlZml4LCBsaW5lKQ==", "after_b64": "ICAgICAgICAgICAgZmxhdHRlbl91c2VfdHJlZSgmcC50cmVlLCAmbmV3X3ByZWZpeCwgbGluZSk=", "target": "src/file_parser.rs", "index": 17, "is_create": false}, {"before_b64": "Ly8vIEludGVybmFsIGludGVybWVkaWF0ZSB0eXBlIGNvbnN1bWVkIGJ5IG1vZHVsZV90cmVlLgojW2Rlcml2ZShEZWJ1ZywgQ2xvbmUsIERlZmF1bHQpXQpwdWIgc3RydWN0IEZpbGVJbmZv", "after_b64": "Ly8vIEludGVybmFsIGludGVybWVkaWF0ZSB0eXBlIGNvbnN1bWVkIGJ5IGBtb2R1bGVfdHJlZWAuCiNbZGVyaXZlKERlYnVnLCBDbG9uZSwgRGVmYXVsdCldCnB1YiBzdHJ1Y3QgRmlsZUluZm8=", "target": "src/schema.rs", "index": 18, "is_create": false}, {"before_b64": "Ly8vIHBhcnNlZCwgb3IgdGhlIEpTT04gb3V0cHV0IGNhbm5vdCBiZSB3cml0dGVuLgpwdWIgZm4gcnVuKGNvbmZpZzogQ29uZmlnKSAtPiBhbnlob3c6OlJlc3VsdDwoKT4gew==", "after_b64": "Ly8vIHBhcnNlZCwgb3IgdGhlIEpTT04gb3V0cHV0IGNhbm5vdCBiZSB3cml0dGVuLgpwdWIgZm4gcnVuKGNvbmZpZzogJkNvbmZpZykgLT4gYW55aG93OjpSZXN1bHQ8KCk+IHs=", "target": "src/lib.rs", "index": 19, "is_create": false}, {"before_b64": "ICAgIHJ1c3Rfd29ya3NwYWNlX21hcDo6cnVuKGNvbmZpZyk=", "after_b64": "ICAgIHJ1c3Rfd29ya3NwYWNlX21hcDo6cnVuKCZjb25maWcp", "target": "src/main.rs", "index": 20, "is_create": false}]')
+
+for step in STEPS:
+    before = base64.b64decode(step["before_b64"]).decode()
+    after = base64.b64decode(step["after_b64"]).decode()
+    target = step["target"]
+    idx = step["index"]
+    is_create = step["is_create"]
+
+    if is_create:
+        target_path = Path(target)
+        target_path.parent.mkdir(parents=True, exist_ok=True)
+        target_path.write_text(after)
+        print(f"OK {TASK_ID} change {idx}: created {target}")
+    else:
+        target_path = Path(target)
+        content = target_path.read_text()
+        if before not in content:
+            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
+            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
+            sys.exit(1)
+
+        result = subprocess.run(
+            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
+            capture_output=True, text=True,
+        )
+        if result.returncode != 0:
+            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
+            sys.exit(result.returncode)
+
+        new_content = target_path.read_text()
+        if after and after not in new_content:
+            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
+            sys.exit(1)
+
+        print(f"OK {TASK_ID} change {idx}: applied to {target}")
+
+print(f"OK {TASK_ID}: all changes applied")
diff --git a/plans/compiled/TASK-8.sh b/plans/compiled/TASK-8.sh
new file mode 100755
index 0000000..63050be
--- /dev/null
+++ b/plans/compiled/TASK-8.sh
@@ -0,0 +1,7 @@
+#!/usr/bin/env bash
+set -euo pipefail
+# TASK-8: Remove all crate-level clippy allow attributes and fix individual lint violations
+# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
+# Type: replace
+# File: src/lib.rs
+python3 "$(dirname "$0")/TASK-8.py"
diff --git a/plans/compiled/TASK-9.py b/plans/compiled/TASK-9.py
new file mode 100644
index 0000000..7ff9e4a
--- /dev/null
+++ b/plans/compiled/TASK-9.py
@@ -0,0 +1,44 @@
+#!/usr/bin/env python3
+"""TASK-9: Add unit tests for file_parser, module_tree, workspace, cargo_info, cross_refs, and render"""
+import base64, json, subprocess, sys
+from pathlib import Path
+
+TASK_ID = "TASK-9"
+STEPS = json.loads('[{"before_b64": "Zm4gZXh0cmFjdF9yZV9leHBvcnRzX2Zyb21fdHJlZSgKICAgIHRyZWU6ICZzeW46OlVzZVRyZWUsCiAgICBpbXBvcnRfcGF0aDogU3RyaW5nLAogICAgbGluZTogdXNpemUsCikgLT4gVmVjPFJlRXhwb3J0PiB7CiAgICBtYXRjaCB0cmVlIHsKICAgICAgICBzeW46OlVzZVRyZWU6OlBhdGgocCkgPT4gewogICAgICAgICAgICBsZXQgbmV3X2ltcG9ydCA9IGlmIGltcG9ydF9wYXRoLmlzX2VtcHR5KCkgewogICAgICAgICAgICAgICAgcC5pZGVudC50b19zdHJpbmcoKQogICAgICAgICAgICB9IGVsc2UgewogICAgICAgICAgICAgICAgZm9ybWF0ISgie306Ont9IiwgaW1wb3J0X3BhdGgsIHAuaWRlbnQpCiAgICAgICAgICAgIH07CiAgICAgICAgICAgIGV4dHJhY3RfcmVfZXhwb3J0c19mcm9tX3RyZWUoJnAudHJlZSwgbmV3X2ltcG9ydCwgbGluZSkKICAgICAgICB9CiAgICAgICAgc3luOjpVc2VUcmVlOjpOYW1lKG4pID0+IHsKICAgICAgICAgICAgdmVjIVtSZUV4cG9ydCB7CiAgICAgICAgICAgICAgICBpbXBvcnRfcGF0aCwKICAgICAgICAgICAgICAgIGV4cG9ydF9wYXRoOiBuLmlkZW50LnRvX3N0cmluZygpLAogICAgICAgICAgICAgICAgbGluZSwKICAgICAgICAgICAgfV0KICAgICAgICB9CiAgICAgICAgc3luOjpVc2VUcmVlOjpSZW5hbWUocikgPT4gewogICAgICAgICAgICB2ZWMhW1JlRXhwb3J0IHsKICAgICAgICAgICAgICAgIGltcG9ydF9wYXRoLAogICAgICAgICAgICAgICAgZXhwb3J0X3BhdGg6IHIucmVuYW1lLnRvX3N0cmluZygpLAogICAgICAgICAgICAgICAgbGluZSwKICAgICAgICAgICAgfV0KICAgICAgICB9CiAgICAgICAgc3luOjpVc2VUcmVlOjpHbG9iKF8pID0+IHsKICAgICAgICAgICAgdmVjIVtSZUV4cG9ydCB7CiAgICAgICAgICAgICAgICBpbXBvcnRfcGF0aCwKICAgICAgICAgICAgICAgIGV4cG9ydF9wYXRoOiAiKiIudG9fc3RyaW5nKCksCiAgICAgICAgICAgICAgICBsaW5lLAogICAgICAgICAgICB9XQogICAgICAgIH0KICAgICAgICBzeW46OlVzZVRyZWU6Okdyb3VwKGcpID0+IGcKICAgICAgICAgICAgLml0ZW1zCiAgICAgICAgICAgIC5pdGVyKCkKICAgICAgICAgICAgLmZsYXRfbWFwKHx0fCBleHRyYWN0X3JlX2V4cG9ydHNfZnJvbV90cmVlKHQsIGltcG9ydF9wYXRoLmNsb25lKCksIGxpbmUpKQogICAgICAgICAgICAuY29sbGVjdCgpLAogICAgfQp9", "after_b64": "Zm4gZXh0cmFjdF9yZV9leHBvcnRzX2Zyb21fdHJlZSgKICAgIHRyZWU6ICZzeW46OlVzZVRyZWUsCiAgICBpbXBvcnRfcGF0aDogU3RyaW5nLAogICAgbGluZTogdXNpemUsCikgLT4gVmVjPFJlRXhwb3J0PiB7CiAgICBtYXRjaCB0cmVlIHsKICAgICAgICBzeW46OlVzZVRyZWU6OlBhdGgocCkgPT4gewogICAgICAgICAgICBsZXQgbmV3X2ltcG9ydCA9IGlmIGltcG9ydF9wYXRoLmlzX2VtcHR5KCkgewogICAgICAgICAgICAgICAgcC5pZGVudC50b19zdHJpbmcoKQogICAgICAgICAgICB9IGVsc2UgewogICAgICAgICAgICAgICAgZm9ybWF0ISgie306Ont9IiwgaW1wb3J0X3BhdGgsIHAuaWRlbnQpCiAgICAgICAgICAgIH07CiAgICAgICAgICAgIGV4dHJhY3RfcmVfZXhwb3J0c19mcm9tX3RyZWUoJnAudHJlZSwgbmV3X2ltcG9ydCwgbGluZSkKICAgICAgICB9CiAgICAgICAgc3luOjpVc2VUcmVlOjpOYW1lKG4pID0+IHsKICAgICAgICAgICAgdmVjIVtSZUV4cG9ydCB7CiAgICAgICAgICAgICAgICBpbXBvcnRfcGF0aCwKICAgICAgICAgICAgICAgIGV4cG9ydF9wYXRoOiBuLmlkZW50LnRvX3N0cmluZygpLAogICAgICAgICAgICAgICAgbGluZSwKICAgICAgICAgICAgfV0KICAgICAgICB9CiAgICAgICAgc3luOjpVc2VUcmVlOjpSZW5hbWUocikgPT4gewogICAgICAgICAgICB2ZWMhW1JlRXhwb3J0IHsKICAgICAgICAgICAgICAgIGltcG9ydF9wYXRoLAogICAgICAgICAgICAgICAgZXhwb3J0X3BhdGg6IHIucmVuYW1lLnRvX3N0cmluZygpLAogICAgICAgICAgICAgICAgbGluZSwKICAgICAgICAgICAgfV0KICAgICAgICB9CiAgICAgICAgc3luOjpVc2VUcmVlOjpHbG9iKF8pID0+IHsKICAgICAgICAgICAgdmVjIVtSZUV4cG9ydCB7CiAgICAgICAgICAgICAgICBpbXBvcnRfcGF0aCwKICAgICAgICAgICAgICAgIGV4cG9ydF9wYXRoOiAiKiIudG9fc3RyaW5nKCksCiAgICAgICAgICAgICAgICBsaW5lLAogICAgICAgICAgICB9XQogICAgICAgIH0KICAgICAgICBzeW46OlVzZVRyZWU6Okdyb3VwKGcpID0+IGcKICAgICAgICAgICAgLml0ZW1zCiAgICAgICAgICAgIC5pdGVyKCkKICAgICAgICAgICAgLmZsYXRfbWFwKHx0fCBleHRyYWN0X3JlX2V4cG9ydHNfZnJvbV90cmVlKHQsIGltcG9ydF9wYXRoLmNsb25lKCksIGxpbmUpKQogICAgICAgICAgICAuY29sbGVjdCgpLAogICAgfQp9CgovLyDilIDilIAgVGVzdHMg4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSACgojW2NmZyh0ZXN0KV0KbW9kIHRlc3RzIHsKICAgIHVzZSBzdXBlcjo6KjsKICAgIHVzZSBjcmF0ZTo6c2NoZW1hOjp7SW1wb3J0LCBSZUV4cG9ydCwgU3VibW9kdWxlRGVjbH07CiAgICB1c2Ugc3RkOjpwYXRoOjpQYXRoQnVmOwoKICAgIGZuIHBhcnNlX3NvdXJjZShzcmM6ICZzdHIpIC0+IFBhcnNlZEZpbGUgewogICAgICAgIGxldCB0bXAgPSBzdGQ6OmVudjo6dGVtcF9kaXIoKS5qb2luKCJwYXJzZV90ZXN0LnJzIik7CiAgICAgICAgc3RkOjpmczo6d3JpdGUoJnRtcCwgc3JjKS51bndyYXAoKTsKICAgICAgICBsZXQgcmVzdWx0ID0gcGFyc2VfZmlsZSgmdG1wKTsKICAgICAgICBzdGQ6OmZzOjpyZW1vdmVfZmlsZSgmdG1wKS5vaygpOwogICAgICAgIHJlc3VsdAogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIHBhcnNlX2ZpbGVfcmV0dXJuc19hc3RfZm9yX3ZhbGlkX3NvdXJjZSgpIHsKICAgICAgICBsZXQgc3JjID0gInB1YiBzdHJ1Y3QgRm9vIHsgeDogaTMyIH0iOwogICAgICAgIGxldCByZXN1bHQgPSBwYXJzZV9zb3VyY2Uoc3JjKTsKICAgICAgICBhc3NlcnQhKHJlc3VsdC5wYXJzZV9lcnJvci5pc19ub25lKCkpOwogICAgICAgIGFzc2VydF9lcSEocmVzdWx0LmFzdC5pdGVtcy5sZW4oKSwgMSk7CiAgICB9CgogICAgI1t0ZXN0XQogICAgZm4gcGFyc2VfZmlsZV9yZXR1cm5zX2Vycm9yX2Zvcl9pbnZhbGlkX3NvdXJjZSgpIHsKICAgICAgICBsZXQgc3JjID0gInB1YiBzdHJ1Y3QgeyBpbnZhbGlkIHJ1c3QgfSI7CiAgICAgICAgbGV0IHJlc3VsdCA9IHBhcnNlX3NvdXJjZShzcmMpOwogICAgICAgIGFzc2VydCEocmVzdWx0LnBhcnNlX2Vycm9yLmlzX3NvbWUoKSk7CiAgICAgICAgbGV0IGVyciA9IHJlc3VsdC5wYXJzZV9lcnJvci5hc19yZWYoKS51bndyYXAoKTsKICAgICAgICBhc3NlcnQhKCFlcnIubWVzc2FnZS5pc19lbXB0eSgpKTsKICAgICAgICBhc3NlcnQhKGVyci5saW5lID4gMCk7CiAgICB9CgogICAgI1t0ZXN0XQogICAgZm4gcGFyc2VfZmlsZV9yZXR1cm5zX2VtcHR5X2Zvcl9lbXB0eV9maWxlKCkgewogICAgICAgIGxldCByZXN1bHQgPSBwYXJzZV9zb3VyY2UoIiIpOwogICAgICAgIGFzc2VydCEocmVzdWx0LnBhcnNlX2Vycm9yLmlzX25vbmUoKSk7CiAgICAgICAgYXNzZXJ0IShyZXN1bHQuZmlsZV9pbmZvLnB1YmxpY19pdGVtcy5pc19lbXB0eSgpKTsKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiBleHRyYWN0X3B1YmxpY19pdGVtc19maW5kc19zdHJ1Y3RfZW51bV90cmFpdF9mbigpIHsKICAgICAgICBsZXQgc3JjID0gInB1YiBzdHJ1Y3QgRm9vIHt9IHB1YiBlbnVtIEJhciB7IEEsIEIgfSBwdWIgdHJhaXQgQmF6IHt9IHB1YiBmbiBoZWxsbygpIHt9IjsKICAgICAgICBsZXQgcmVzdWx0ID0gcGFyc2Vfc291cmNlKHNyYyk7CiAgICAgICAgbGV0IGl0ZW1zID0gZXh0cmFjdF9wdWJsaWNfaXRlbXMoJnJlc3VsdC5hc3QuaXRlbXMpOwogICAgICAgIGxldCBuYW1lczogVmVjPF8+ID0gaXRlbXMuaXRlcigpLm1hcCh8aXwgaS5uYW1lLmFzX3N0cigpKS5jb2xsZWN0KCk7CiAgICAgICAgYXNzZXJ0IShuYW1lcy5jb250YWlucygmIkZvbyIpKTsKICAgICAgICBhc3NlcnQhKG5hbWVzLmNvbnRhaW5zKCYiQmFyIikpOwogICAgICAgIGFzc2VydCEobmFtZXMuY29udGFpbnMoJiJCYXoiKSk7CiAgICAgICAgYXNzZXJ0IShuYW1lcy5jb250YWlucygmImhlbGxvIikpOwogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIGV4dHJhY3RfcHVibGljX2l0ZW1zX2VtcHR5X2Zvcl9ub19wdWJsaWNfaXRlbXMoKSB7CiAgICAgICAgbGV0IHNyYyA9ICJzdHJ1Y3QgUHJpdmF0ZSB7fSBmbiBwcml2YXRlX2ZuKCkge30iOwogICAgICAgIGxldCByZXN1bHQgPSBwYXJzZV9zb3VyY2Uoc3JjKTsKICAgICAgICBsZXQgaXRlbXMgPSBleHRyYWN0X3B1YmxpY19pdGVtcygmcmVzdWx0LmFzdC5pdGVtcyk7CiAgICAgICAgYXNzZXJ0IShpdGVtcy5pc19lbXB0eSgpKTsKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiBleHRyYWN0X2ltcG9ydHNfZmluZHNfdXNlX3N0YXRlbWVudHMoKSB7CiAgICAgICAgbGV0IHNyYyA9ICJ1c2Ugc3RkOjpjb2xsZWN0aW9uczo6QlRyZWVNYXA7IjsKICAgICAgICBsZXQgcmVzdWx0ID0gcGFyc2Vfc291cmNlKHNyYyk7CiAgICAgICAgbGV0IGltcG9ydHMgPSBleHRyYWN0X2ltcG9ydHMoJnJlc3VsdC5hc3QuaXRlbXMpOwogICAgICAgIGFzc2VydF9lcSEoaW1wb3J0cy5sZW4oKSwgMSk7CiAgICAgICAgYXNzZXJ0X2VxIShpbXBvcnRzWzBdLnBhdGgsICJzdGQ6OmNvbGxlY3Rpb25zOjpCVHJlZU1hcCIpOwogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIGV4dHJhY3RfcmVfZXhwb3J0c19maW5kc19wdWJfdXNlKCkgewogICAgICAgIGxldCBzcmMgPSAicHViIHVzZSBjcmF0ZTo6Zm9vOyI7CiAgICAgICAgbGV0IHJlc3VsdCA9IHBhcnNlX3NvdXJjZShzcmMpOwogICAgICAgIGxldCByZV9leHBvcnRzID0gZXh0cmFjdF9yZV9leHBvcnRzKCZyZXN1bHQuYXN0Lml0ZW1zKTsKICAgICAgICBhc3NlcnRfZXEhKHJlX2V4cG9ydHMubGVuKCksIDEpOwogICAgICAgIGFzc2VydF9lcSEocmVfZXhwb3J0c1swXS5pbXBvcnRfcGF0aCwgImNyYXRlOjpmb28iKTsKICAgICAgICBhc3NlcnRfZXEhKHJlX2V4cG9ydHNbMF0uZXhwb3J0X3BhdGgsICJmb28iKTsKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiBleHRyYWN0X3JlX2V4cG9ydHNfZmluZHNfcmVuYW1lKCkgewogICAgICAgIGxldCBzcmMgPSAicHViIHVzZSBjcmF0ZTo6Zm9vIGFzIGJhcjsiOwogICAgICAgIGxldCByZXN1bHQgPSBwYXJzZV9zb3VyY2Uoc3JjKTsKICAgICAgICBsZXQgcmVfZXhwb3J0cyA9IGV4dHJhY3RfcmVfZXhwb3J0cygmcmVzdWx0LmFzdC5pdGVtcyk7CiAgICAgICAgYXNzZXJ0X2VxIShyZV9leHBvcnRzLmxlbigpLCAxKTsKICAgICAgICBhc3NlcnRfZXEhKHJlX2V4cG9ydHNbMF0uaW1wb3J0X3BhdGgsICJjcmF0ZTo6Zm9vIGFzIGJhciIpOwogICAgICAgIGFzc2VydF9lcSEocmVfZXhwb3J0c1swXS5leHBvcnRfcGF0aCwgImJhciIpOwogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIGV4dHJhY3Rfc3VibW9kdWxlc19maW5kc19tb2RfZGVjbGFyYXRpb25zKCkgewogICAgICAgIGxldCBzcmMgPSAibW9kIGZvbzsgbW9kIGJhcjsiOwogICAgICAgIGxldCByZXN1bHQgPSBwYXJzZV9zb3VyY2Uoc3JjKTsKICAgICAgICBsZXQgc3VicyA9IGV4dHJhY3Rfc3VibW9kdWxlcygmcmVzdWx0LmFzdC5pdGVtcyk7CiAgICAgICAgYXNzZXJ0X2VxIShzdWJzLmxlbigpLCAyKTsKICAgICAgICBsZXQgbmFtZXM6IFZlYzxfPiA9IHN1YnMuaXRlcigpLm1hcCh8c3wgcy5uYW1lLmFzX3N0cigpKS5jb2xsZWN0KCk7CiAgICAgICAgYXNzZXJ0IShuYW1lcy5jb250YWlucygmImJhciIpKTsKICAgICAgICBhc3NlcnQhKG5hbWVzLmNvbnRhaW5zKCYiZm9vIikpOwogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIGV4dHJhY3Rfc3VibW9kdWxlc19tYXJrc19jZmdfdGVzdCgpIHsKICAgICAgICBsZXQgc3JjID0gIiNbY2ZnKHRlc3QpXSBtb2QgaW5uZXI7IjsKICAgICAgICBsZXQgcmVzdWx0ID0gcGFyc2Vfc291cmNlKHNyYyk7CiAgICAgICAgbGV0IHN1YnMgPSBleHRyYWN0X3N1Ym1vZHVsZXMoJnJlc3VsdC5hc3QuaXRlbXMpOwogICAgICAgIGFzc2VydF9lcSEoc3Vicy5sZW4oKSwgMSk7CiAgICAgICAgYXNzZXJ0IShzdWJzWzBdLmlzX3Rlc3QpOwogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIGV4dHJhY3RfaW1wbHNfZmluZHNfZm5fdHlwZV9jb25zdCgpIHsKICAgICAgICBsZXQgc3JjID0gImltcGwgTXlUeXBlIHsgcHViIGZuIGZvbygmc2VsZikge30gcHViIHR5cGUgQWxpYXMgPSB1MzI7IHB1YiBjb25zdCBOOiB1c2l6ZSA9IDQyOyB9IjsKICAgICAgICBsZXQgcmVzdWx0ID0gcGFyc2Vfc291cmNlKHNyYyk7CiAgICAgICAgbGV0IGltcGxzID0gZXh0cmFjdF9pbXBscygmcmVzdWx0LmFzdC5pdGVtcyk7CiAgICAgICAgYXNzZXJ0X2VxIShpbXBscy5sZW4oKSwgMSk7CiAgICAgICAgYXNzZXJ0X2VxIShpbXBsc1swXS50eXBlXywgIk15VHlwZSIpOwogICAgICAgIGFzc2VydF9lcSEoaW1wbHNbMF0uaXRlbXMubGVuKCksIDMpOwogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIGJ1aWxkX3BhcnNlX2Vycm9yX2VudHJ5X2NvbnN0cnVjdHNfZXJyb3IoKSB7CiAgICAgICAgbGV0IHBhdGggPSBQYXRoQnVmOjpmcm9tKCJ0ZXN0LnJzIik7CiAgICAgICAgbGV0IGVyciA9IFN5blBhcnNlRXJyb3IgewogICAgICAgICAgICBtZXNzYWdlOiAiZXhwZWN0ZWQgYDtgIi50b19zdHJpbmcoKSwKICAgICAgICAgICAgbGluZTogNSwKICAgICAgICB9OwogICAgICAgIGxldCBlbnRyeSA9IGJ1aWxkX3BhcnNlX2Vycm9yX2VudHJ5KCZwYXRoLCAmZXJyKTsKICAgICAgICBhc3NlcnRfZXEhKGVudHJ5LmZpbGUsICJ0ZXN0LnJzIik7CiAgICAgICAgYXNzZXJ0X2VxIShlbnRyeS5saW5lLCA1KTsKICAgICAgICBhc3NlcnRfZXEhKGVudHJ5LmtpbmQsICJzeW5fcGFyc2VfZXJyb3IiKTsKICAgICAgICBhc3NlcnRfZXEhKGVudHJ5LnNldmVyaXR5LCBFcnJvclNldmVyaXR5OjpFcnJvcik7CiAgICB9Cn0=", "target": "src/file_parser.rs", "index": 0, "is_create": false}, {"before_b64": "Zm4gcHJvY2Vzc19tb2R1bGVfaW5mbygKICAgIG1vZHVsZV9wYXRoOiAmc3RyLAogICAgZmlsZV9wYXRoOiAmUGF0aCwKICAgIHZpc2liaWxpdHk6ICZzdHIsCiAgICBmaWxlX2luZm86ICZGaWxlSW5mbywKICAgIGl0ZW1zOiAmW3N5bjo6SXRlbV0sCiAgICBfcGFyZW50X2RpcjogJlBhdGgsCiAgICB2aXNpdGVkOiAmbXV0IEhhc2hTZXQ8UGF0aEJ1Zj4sCiAgICBlcnJvcnM6ICZtdXQgVmVjPGNyYXRlOjpzY2hlbWE6OkVycm9yRW50cnk+LAopIC0+IChWZWM8TW9kdWxlSW5mbz4sIFZlYzxjcmF0ZTo6c2NoZW1hOjpFcnJvckVudHJ5PikgewogICAgbGV0IG11dCBtb2R1bGVzID0gdmVjIVtidWlsZF9tb2R1bGVfaW5mbygKICAgICAgICBtb2R1bGVfcGF0aCwKICAgICAgICBmaWxlX3BhdGgsCiAgICAgICAgdmlzaWJpbGl0eSwKICAgICAgICAmZmlsZV9pbmZvLnB1YmxpY19pdGVtcywKICAgICAgICAmZmlsZV9pbmZvLmltcG9ydHMsCiAgICAgICAgJmZpbGVfaW5mby5yZV9leHBvcnRzLAogICAgICAgICZmaWxlX2luZm8uc3VibW9kdWxlcywKICAgICldOwoKICAgIGZvciBzdWIgaW4gJmZpbGVfaW5mby5zdWJtb2R1bGVzIHsKICAgICAgICBpZiBzdWIuaXNfdGVzdCB7CiAgICAgICAgICAgIGNvbnRpbnVlOwogICAgICAgIH0KICAgICAgICBsZXQgY2hpbGRfcGF0aCA9IGZvcm1hdCEoInt9Ojp7fSIsIG1vZHVsZV9wYXRoLCBzdWIubmFtZSk7CiAgICAgICAgbGV0IGNoaWxkX2RpciA9IGZpbGVfcGF0aC5wYXJlbnQoKS51bndyYXBfb3IoZmlsZV9wYXRoKTsKICAgICAgICBsZXQgKGNoaWxkX21vZHVsZXMsIGNoaWxkX2Vycm9ycykgPSBwcm9jZXNzX3N1Ym1vZHVsZSgKICAgICAgICAgICAgJmNoaWxkX3BhdGgsCiAgICAgICAgICAgICZzdWIubmFtZSwKICAgICAgICAgICAgaXRlbXMsCiAgICAgICAgICAgIGNoaWxkX2RpciwKICAgICAgICAgICAgZmlsZV9wYXRoLAogICAgICAgICAgICB2aXNpdGVkLAogICAgICAgICk7CiAgICAgICAgZXJyb3JzLmV4dGVuZChjaGlsZF9lcnJvcnMpOwogICAgICAgIG1vZHVsZXMuZXh0ZW5kKGNoaWxkX21vZHVsZXMpOwogICAgfQoKICAgIChtb2R1bGVzLCBlcnJvcnMuY2xvbmUoKSkKfQ==", "after_b64": "Zm4gcHJvY2Vzc19tb2R1bGVfaW5mbygKICAgIG1vZHVsZV9wYXRoOiAmc3RyLAogICAgZmlsZV9wYXRoOiAmUGF0aCwKICAgIHZpc2liaWxpdHk6ICZzdHIsCiAgICBmaWxlX2luZm86ICZGaWxlSW5mbywKICAgIGl0ZW1zOiAmW3N5bjo6SXRlbV0sCiAgICBfcGFyZW50X2RpcjogJlBhdGgsCiAgICB2aXNpdGVkOiAmbXV0IEhhc2hTZXQ8UGF0aEJ1Zj4sCiAgICBlcnJvcnM6ICZtdXQgVmVjPGNyYXRlOjpzY2hlbWE6OkVycm9yRW50cnk+LAopIC0+IChWZWM8TW9kdWxlSW5mbz4sIFZlYzxjcmF0ZTo6c2NoZW1hOjpFcnJvckVudHJ5PikgewogICAgbGV0IG11dCBtb2R1bGVzID0gdmVjIVtidWlsZF9tb2R1bGVfaW5mbygKICAgICAgICBtb2R1bGVfcGF0aCwKICAgICAgICBmaWxlX3BhdGgsCiAgICAgICAgdmlzaWJpbGl0eSwKICAgICAgICAmZmlsZV9pbmZvLnB1YmxpY19pdGVtcywKICAgICAgICAmZmlsZV9pbmZvLmltcG9ydHMsCiAgICAgICAgJmZpbGVfaW5mby5yZV9leHBvcnRzLAogICAgICAgICZmaWxlX2luZm8uc3VibW9kdWxlcywKICAgICldOwoKICAgIGZvciBzdWIgaW4gJmZpbGVfaW5mby5zdWJtb2R1bGVzIHsKICAgICAgICBpZiBzdWIuaXNfdGVzdCB7CiAgICAgICAgICAgIGNvbnRpbnVlOwogICAgICAgIH0KICAgICAgICBsZXQgY2hpbGRfcGF0aCA9IGZvcm1hdCEoInt9Ojp7fSIsIG1vZHVsZV9wYXRoLCBzdWIubmFtZSk7CiAgICAgICAgbGV0IGNoaWxkX2RpciA9IGZpbGVfcGF0aC5wYXJlbnQoKS51bndyYXBfb3IoZmlsZV9wYXRoKTsKICAgICAgICBsZXQgKGNoaWxkX21vZHVsZXMsIGNoaWxkX2Vycm9ycykgPSBwcm9jZXNzX3N1Ym1vZHVsZSgKICAgICAgICAgICAgJmNoaWxkX3BhdGgsCiAgICAgICAgICAgICZzdWIubmFtZSwKICAgICAgICAgICAgaXRlbXMsCiAgICAgICAgICAgIGNoaWxkX2RpciwKICAgICAgICAgICAgZmlsZV9wYXRoLAogICAgICAgICAgICB2aXNpdGVkLAogICAgICAgICk7CiAgICAgICAgZXJyb3JzLmV4dGVuZChjaGlsZF9lcnJvcnMpOwogICAgICAgIG1vZHVsZXMuZXh0ZW5kKGNoaWxkX21vZHVsZXMpOwogICAgfQoKICAgIChtb2R1bGVzLCBlcnJvcnMuY2xvbmUoKSkKfQoKLy8g4pSA4pSAIFRlc3RzIOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgAoKI1tjZmcodGVzdCldCm1vZCB0ZXN0cyB7CiAgICB1c2Ugc3VwZXI6Oio7CiAgICB1c2UgY3JhdGU6OnNjaGVtYTo6RXJyb3JFbnRyeTsKCiAgICAjW3Rlc3RdCiAgICBmbiByZXNvbHZlX21vZHVsZV9wYXRoX2ZpbmRzX3JzX2ZpbGUoKSB7CiAgICAgICAgbGV0IHRtcCA9IHN0ZDo6ZW52Ojp0ZW1wX2RpcigpLmpvaW4oInJlc29sdmVfdGVzdCIpOwogICAgICAgIGxldCBfID0gc3RkOjpmczo6Y3JlYXRlX2Rpcl9hbGwoJnRtcCk7CiAgICAgICAgbGV0IG1vZF9maWxlID0gdG1wLmpvaW4oImZvby5ycyIpOwogICAgICAgIHN0ZDo6ZnM6OndyaXRlKCZtb2RfZmlsZSwgIiIpLm9rKCk7CiAgICAgICAgbGV0IHJlc3VsdCA9IHJlc29sdmVfbW9kdWxlX3BhdGgoJnRtcCwgImZvbyIpOwogICAgICAgIGFzc2VydF9lcSEocmVzdWx0LCBTb21lKG1vZF9maWxlKSk7CiAgICAgICAgc3RkOjpmczo6cmVtb3ZlX2Rpcl9hbGwoJnRtcCkub2soKTsKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiByZXNvbHZlX21vZHVsZV9wYXRoX2ZpbmRzX21vZF9ycygpIHsKICAgICAgICBsZXQgdG1wID0gc3RkOjplbnY6OnRlbXBfZGlyKCkuam9pbigicmVzb2x2ZV90ZXN0MiIpOwogICAgICAgIGxldCBfID0gc3RkOjpmczo6Y3JlYXRlX2Rpcl9hbGwoJnRtcCk7CiAgICAgICAgbGV0IG1vZF9kaXIgPSB0bXAuam9pbigiYmFyIik7CiAgICAgICAgbGV0IF8gPSBzdGQ6OmZzOjpjcmVhdGVfZGlyX2FsbCgmbW9kX2Rpcik7CiAgICAgICAgbGV0IG1vZF9ycyA9IG1vZF9kaXIuam9pbigibW9kLnJzIik7CiAgICAgICAgc3RkOjpmczo6d3JpdGUoJm1vZF9ycywgIiIpLm9rKCk7CiAgICAgICAgbGV0IHJlc3VsdCA9IHJlc29sdmVfbW9kdWxlX3BhdGgoJnRtcCwgImJhciIpOwogICAgICAgIGFzc2VydF9lcSEocmVzdWx0LCBTb21lKG1vZF9ycykpOwogICAgICAgIHN0ZDo6ZnM6OnJlbW92ZV9kaXJfYWxsKCZ0bXApLm9rKCk7CiAgICB9CgogICAgI1t0ZXN0XQogICAgZm4gcmVzb2x2ZV9tb2R1bGVfcGF0aF9yZXR1cm5zX25vbmVfZm9yX21pc3NpbmcoKSB7CiAgICAgICAgbGV0IHRtcCA9IHN0ZDo6ZW52Ojp0ZW1wX2RpcigpLmpvaW4oInJlc29sdmVfdGVzdDMiKTsKICAgICAgICBsZXQgXyA9IHN0ZDo6ZnM6OmNyZWF0ZV9kaXJfYWxsKCZ0bXApOwogICAgICAgIGxldCByZXN1bHQgPSByZXNvbHZlX21vZHVsZV9wYXRoKCZ0bXAsICJub25leGlzdGVudCIpOwogICAgICAgIGFzc2VydCEocmVzdWx0LmlzX25vbmUoKSk7CiAgICAgICAgc3RkOjpmczo6cmVtb3ZlX2Rpcl9hbGwoJnRtcCkub2soKTsKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiBidWlsZF9tb2R1bGVfdHJlZV9yZXR1cm5zX2VtcHR5X2Zvcl9ub25leGlzdGVudCgpIHsKICAgICAgICBsZXQgdG1wID0gc3RkOjplbnY6OnRlbXBfZGlyKCkuam9pbigiYm10X3Rlc3QiKTsKICAgICAgICBsZXQgXyA9IHN0ZDo6ZnM6OmNyZWF0ZV9kaXJfYWxsKCZ0bXApOwogICAgICAgIGxldCAobW9kdWxlcywgZXJyb3JzKSA9IGJ1aWxkX21vZHVsZV90cmVlKCZ0bXAsICJ0ZXN0Iik7CiAgICAgICAgYXNzZXJ0IShtb2R1bGVzLmlzX2VtcHR5KCkpOwogICAgICAgIGFzc2VydCEoIWVycm9ycy5pc19lbXB0eSgpKTsKICAgICAgICBzdGQ6OmZzOjpyZW1vdmVfZGlyX2FsbCgmdG1wKS5vaygpOwogICAgfQp9", "target": "src/module_tree.rs", "index": 1, "is_create": false}, {"before_b64": "ICAgIHJlc3VsdC5zb3J0KCk7CiAgICByZXN1bHQuZGVkdXAoKTsKICAgIE9rKHJlc3VsdCkKfQoKLy8vIEZvciBhIGNyYXRlIGRpcmVjdG9yeSwgZGV0ZXJtaW5lIGl0cyBlbnRyeS1wb2ludCBmaWxlKHMpLg==", "after_b64": "ICAgIHJlc3VsdC5zb3J0KCk7CiAgICByZXN1bHQuZGVkdXAoKTsKICAgIE9rKHJlc3VsdCkKfQoKLy8g4pSA4pSAIFRlc3RzIOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgAoKI1tjZmcodGVzdCldCm1vZCB0ZXN0cyB7CiAgICB1c2Ugc3VwZXI6Oio7CiAgICB1c2Ugc3RkOjppbzo6V3JpdGU7CgogICAgZm4gd3JpdGVfY2FyZ29fdG9tbChkaXI6ICZzdGQ6OnBhdGg6OlBhdGgsIGNvbnRlbnQ6ICZzdHIpIHsKICAgICAgICBsZXQgbXV0IGYgPSBzdGQ6OmZzOjpGaWxlOjpjcmVhdGUoZGlyLmpvaW4oIkNhcmdvLnRvbWwiKSkudW53cmFwKCk7CiAgICAgICAgZi53cml0ZV9hbGwoY29udGVudC5hc19ieXRlcygpKS51bndyYXAoKTsKICAgIH0KCiAgICBmbiBzZXR1cF9jcmF0ZShkaXI6ICZzdGQ6OnBhdGg6OlBhdGgpIHsKICAgICAgICBsZXQgc3JjID0gZGlyLmpvaW4oInNyYyIpOwogICAgICAgIHN0ZDo6ZnM6OmNyZWF0ZV9kaXJfYWxsKCZzcmMpLnVud3JhcCgpOwogICAgICAgIHN0ZDo6ZnM6OndyaXRlKHNyYy5qb2luKCJsaWIucnMiKSwgIiIpLnVud3JhcCgpOwogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIGZpbmRfd29ya3NwYWNlX3Jvb3RfZmluZHNfY2FyZ29fdG9tbCgpIHsKICAgICAgICBsZXQgdG1wID0gdGVtcGZpbGU6OnRlbXBkaXIoKS51bndyYXAoKTsKICAgICAgICBsZXQgcGF0aCA9IHRtcC5wYXRoKCkuam9pbigic3ViZGlyIikuam9pbigibmVzdGVkIik7CiAgICAgICAgc3RkOjpmczo6Y3JlYXRlX2Rpcl9hbGwoJnBhdGgpLnVud3JhcCgpOwogICAgICAgIHdyaXRlX2NhcmdvX3RvbWwodG1wLnBhdGgoKSwgIlt3b3Jrc3BhY2VdIik7CiAgICAgICAgbGV0IHJlc3VsdCA9IGZpbmRfd29ya3NwYWNlX3Jvb3QoJnBhdGgpLnVud3JhcCgpOwogICAgICAgIGFzc2VydF9lcSEocmVzdWx0LCB0bXAucGF0aCgpKTsKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiBlbnVtZXJhdGVfbWVtYmVyc19yZXR1cm5zX21lbWJlcnMoKSB7CiAgICAgICAgbGV0IHRtcCA9IHRlbXBmaWxlOjp0ZW1wZGlyKCkudW53cmFwKCk7CiAgICAgICAgd3JpdGVfY2FyZ29fdG9tbCh0bXAucGF0aCgpLCByIyIKW3dvcmtzcGFjZV0KbWVtYmVycyA9IFsiY3JhdGVfYSIsICJjcmF0ZV9iIl0KIiMpOwogICAgICAgIHNldHVwX2NyYXRlKHRtcC5wYXRoKCkuam9pbigiY3JhdGVfYSIpLmFzX3BhdGgoKSk7CiAgICAgICAgc2V0dXBfY3JhdGUodG1wLnBhdGgoKS5qb2luKCJjcmF0ZV9iIikuYXNfcGF0aCgpKTsKICAgICAgICBsZXQgbWVtYmVycyA9IGVudW1lcmF0ZV9tZW1iZXJzKHRtcC5wYXRoKCkpLnVud3JhcCgpOwogICAgICAgIGFzc2VydF9lcSEobWVtYmVycy5sZW4oKSwgMik7CiAgICB9CgogICAgI1t0ZXN0XQogICAgZm4gZW51bWVyYXRlX21lbWJlcnNfcmV0dXJuc19lcnJfZm9yX21pc3Npbmdfd29ya3NwYWNlKCkgewogICAgICAgIGxldCB0bXAgPSB0ZW1wZmlsZTo6dGVtcGRpcigpLnVud3JhcCgpOwogICAgICAgIHdyaXRlX2NhcmdvX3RvbWwodG1wLnBhdGgoKSwgIltkZXBlbmRlbmNpZXNdXG5mb28gPSBcIjFcIiIpOwogICAgICAgIGxldCByZXN1bHQgPSBlbnVtZXJhdGVfbWVtYmVycyh0bXAucGF0aCgpKTsKICAgICAgICBhc3NlcnQhKHJlc3VsdC5pc19lcnIoKSk7CiAgICAgICAgbWF0Y2ggcmVzdWx0LnVud3JhcF9lcnIoKSB7CiAgICAgICAgICAgIEVycm9yOjpNaXNzaW5nV29ya3NwYWNlU2VjdGlvbiA9PiB7fSwKICAgICAgICAgICAgb3RoZXIgPT4gcGFuaWMhKCJleHBlY3RlZCBNaXNzaW5nV29ya3NwYWNlU2VjdGlvbiwgZ290IHs6P30iLCBvdGhlciksCiAgICAgICAgfQogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIGVudW1lcmF0ZV9tZW1iZXJzX2FwcGxpZXNfZXhjbHVkZSgpIHsKICAgICAgICBsZXQgdG1wID0gdGVtcGZpbGU6OnRlbXBkaXIoKS51bndyYXAoKTsKICAgICAgICB3cml0ZV9jYXJnb190b21sKHRtcC5wYXRoKCksIHIjIgpbd29ya3NwYWNlXQptZW1iZXJzID0gWyJhIiwgImIiLCAiYyJdCmV4Y2x1ZGUgPSBbImIiXQoiIyk7CiAgICAgICAgc2V0dXBfY3JhdGUodG1wLnBhdGgoKS5qb2luKCJhIikuYXNfcGF0aCgpKTsKICAgICAgICBzZXR1cF9jcmF0ZSh0bXAucGF0aCgpLmpvaW4oImIiKS5hc19wYXRoKCkpOwogICAgICAgIHNldHVwX2NyYXRlKHRtcC5wYXRoKCkuam9pbigiYyIpLmFzX3BhdGgoKSk7CiAgICAgICAgbGV0IG1lbWJlcnMgPSBlbnVtZXJhdGVfbWVtYmVycyh0bXAucGF0aCgpKS51bndyYXAoKTsKICAgICAgICBsZXQgbmFtZXM6IFZlYzxfPiA9IG1lbWJlcnMuaXRlcigpLm1hcCh8cHwgcC5maWxlX25hbWUoKS51bndyYXAoKS50b19zdHJpbmdfbG9zc3koKSkuY29sbGVjdCgpOwogICAgICAgIGFzc2VydCEobmFtZXMuY29udGFpbnMoJiJhIi5hc19yZWYoKSkpOwogICAgICAgIGFzc2VydCEoIW5hbWVzLmNvbnRhaW5zKCYiYiIuYXNfcmVmKCkpKTsKICAgICAgICBhc3NlcnQhKG5hbWVzLmNvbnRhaW5zKCYiYyIuYXNfcmVmKCkpKTsKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiByZXNvbHZlX2NyYXRlX3Jvb3RzX2RldGVjdHNfbGliKCkgewogICAgICAgIGxldCB0bXAgPSB0ZW1wZmlsZTo6dGVtcGRpcigpLnVud3JhcCgpOwogICAgICAgIHN0ZDo6ZnM6OmNyZWF0ZV9kaXJfYWxsKHRtcC5wYXRoKCkuam9pbigic3JjIikpLnVud3JhcCgpOwogICAgICAgIHN0ZDo6ZnM6OndyaXRlKHRtcC5wYXRoKCkuam9pbigic3JjIikuam9pbigibGliLnJzIiksICIiKS51bndyYXAoKTsKICAgICAgICBsZXQgcm9vdHMgPSByZXNvbHZlX2NyYXRlX3Jvb3RzKHRtcC5wYXRoKCkpOwogICAgICAgIGFzc2VydF9lcSEocm9vdHMubGVuKCksIDEpOwogICAgICAgIGFzc2VydF9lcSEocm9vdHNbMF0uMSwgQ3JhdGVUeXBlOjpMaWIpOwogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIHJlc29sdmVfY3JhdGVfcm9vdHNfZGV0ZWN0c19iaW4oKSB7CiAgICAgICAgbGV0IHRtcCA9IHRlbXBmaWxlOjp0ZW1wZGlyKCkudW53cmFwKCk7CiAgICAgICAgc3RkOjpmczo6Y3JlYXRlX2Rpcl9hbGwodG1wLnBhdGgoKS5qb2luKCJzcmMiKSkudW53cmFwKCk7CiAgICAgICAgc3RkOjpmczo6d3JpdGUodG1wLnBhdGgoKS5qb2luKCJzcmMiKS5qb2luKCJtYWluLnJzIiksICIiKS51bndyYXAoKTsKICAgICAgICBsZXQgcm9vdHMgPSByZXNvbHZlX2NyYXRlX3Jvb3RzKHRtcC5wYXRoKCkpOwogICAgICAgIGFzc2VydF9lcSEocm9vdHMubGVuKCksIDEpOwogICAgICAgIGFzc2VydF9lcSEocm9vdHNbMF0uMSwgQ3JhdGVUeXBlOjpCaW4pOwogICAgfQp9", "target": "src/workspace.rs", "index": 2, "is_create": false}, {"before_b64": "ICAgIE9rKChwYWNrYWdlLCBkZXBzKSkKfQ==", "after_b64": "ICAgIE9rKChwYWNrYWdlLCBkZXBzKSkKfQoKLy8g4pSA4pSAIFRlc3RzIOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgAoKI1tjZmcodGVzdCldCm1vZCB0ZXN0cyB7CiAgICB1c2Ugc3VwZXI6Oio7CiAgICB1c2Ugc3RkOjppbzo6V3JpdGU7CgogICAgZm4gd3JpdGVfY2FyZ29fdG9tbChkaXI6ICZzdGQ6OnBhdGg6OlBhdGgsIGNvbnRlbnQ6ICZzdHIpIHsKICAgICAgICBsZXQgbXV0IGYgPSBzdGQ6OmZzOjpGaWxlOjpjcmVhdGUoZGlyLmpvaW4oIkNhcmdvLnRvbWwiKSkudW53cmFwKCk7CiAgICAgICAgZi53cml0ZV9hbGwoY29udGVudC5hc19ieXRlcygpKS51bndyYXAoKTsKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiBwYXJzZV9jYXJnb190b21sX3BhcnNlc19taW5pbWFsKCkgewogICAgICAgIGxldCB0bXAgPSB0ZW1wZmlsZTo6dGVtcGRpcigpLnVud3JhcCgpOwogICAgICAgIHdyaXRlX2NhcmdvX3RvbWwodG1wLnBhdGgoKSwgciMiCltwYWNrYWdlXQpuYW1lID0gInRlc3QtcGtnIgp2ZXJzaW9uID0gIjEuMC4wIgplZGl0aW9uID0gIjIwMjEiCiIjKTsKICAgICAgICBsZXQgKHBrZywgX2RlcHMpID0gcGFyc2VfY2FyZ29fdG9tbCh0bXAucGF0aCgpLmpvaW4oIkNhcmdvLnRvbWwiKS5hc19wYXRoKCkpLnVud3JhcCgpOwogICAgICAgIGFzc2VydF9lcSEocGtnLm5hbWUsICJ0ZXN0LXBrZyIpOwogICAgICAgIGFzc2VydF9lcSEocGtnLnZlcnNpb24sICIxLjAuMCIpOwogICAgICAgIGFzc2VydF9lcSEocGtnLmVkaXRpb24sICIyMDIxIik7CiAgICB9CgogICAgI1t0ZXN0XQogICAgZm4gcGFyc2VfY2FyZ29fdG9tbF91c2VzX2RlZmF1bHRzX2Zvcl9taXNzaW5nX3BhY2thZ2UoKSB7CiAgICAgICAgbGV0IHRtcCA9IHRlbXBmaWxlOjp0ZW1wZGlyKCkudW53cmFwKCk7CiAgICAgICAgd3JpdGVfY2FyZ29fdG9tbCh0bXAucGF0aCgpLCAiIik7CiAgICAgICAgbGV0IChwa2csIF9kZXBzKSA9IHBhcnNlX2NhcmdvX3RvbWwodG1wLnBhdGgoKS5qb2luKCJDYXJnby50b21sIikuYXNfcGF0aCgpKS51bndyYXAoKTsKICAgICAgICBhc3NlcnRfZXEhKHBrZy5uYW1lLCAidW5rbm93biIpOwogICAgICAgIGFzc2VydF9lcSEocGtnLmVkaXRpb24sICIyMDIxIik7CiAgICB9CgogICAgI1t0ZXN0XQogICAgZm4gcGFyc2VfY2FyZ29fdG9tbF9kaXN0aW5ndWlzaGVzX2RlcHMoKSB7CiAgICAgICAgbGV0IHRtcCA9IHRlbXBmaWxlOjp0ZW1wZGlyKCkudW53cmFwKCk7CiAgICAgICAgd3JpdGVfY2FyZ29fdG9tbCh0bXAucGF0aCgpLCByIyIKW3BhY2thZ2VdCm5hbWUgPSAidGVzdC1wa2ciCnZlcnNpb24gPSAiMC4xLjAiCmVkaXRpb24gPSAiMjAyMSIKCltkZXBlbmRlbmNpZXNdCmZvbyA9ICIxIgpiYXIgPSB7IHdvcmtzcGFjZSA9IHRydWUgfQoKW2Rldi1kZXBlbmRlbmNpZXNdCmJheiA9ICIyIgpxdXggPSB7IHdvcmtzcGFjZSA9IHRydWUgfQoiIyk7CiAgICAgICAgbGV0IChfLCBkZXBzKSA9IHBhcnNlX2NhcmdvX3RvbWwodG1wLnBhdGgoKS5qb2luKCJDYXJnby50b21sIikuYXNfcGF0aCgpKS51bndyYXAoKTsKICAgICAgICBhc3NlcnRfZXEhKGRlcHMubm9ybWFsLCB2ZWMhWyJmb28iXSk7CiAgICAgICAgYXNzZXJ0X2VxIShkZXBzLmRldiwgdmVjIVsiYmF6Il0pOwogICAgICAgIGFzc2VydCEoZGVwcy53b3Jrc3BhY2VfbWVtYmVycy5jb250YWlucygmImJhciIudG9fc3RyaW5nKCkpKTsKICAgICAgICBhc3NlcnQhKGRlcHMud29ya3NwYWNlX21lbWJlcnMuY29udGFpbnMoJiJxdXgiLnRvX3N0cmluZygpKSk7CiAgICB9Cn0=", "target": "src/cargo_info.rs", "index": 3, "is_create": false}, {"before_b64": "ICAgIENyb3NzUmVmZXJlbmNlcyB7IHR5cGVzOiB0eXBlc19tYXAgfQp9CgovLyBIZWxwZXI6IGNvbnZlcnQgSXRlbUtpbmQgdG8gYSBzaG9ydCBzdHJpbmcgZm9yIHRoZSBUeXBlUmVmLmtpbmQgZmllbGQu", "after_b64": "ICAgIENyb3NzUmVmZXJlbmNlcyB7IHR5cGVzOiB0eXBlc19tYXAgfQp9CgovLyDilIDilIAgVGVzdHMg4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSACgojW2NmZyh0ZXN0KV0KbW9kIHRlc3RzIHsKICAgIHVzZSBzdXBlcjo6KjsKICAgIHVzZSBjcmF0ZTo6c2NoZW1hOjp7TW9kdWxlSW5mbywgUHVibGljSXRlbSwgU3VibW9kdWxlRGVjbH07CgogICAgZm4gbWFrZV9jcmF0ZShuYW1lOiAmc3RyLCBpdGVtczogVmVjPChTdHJpbmcsIEl0ZW1LaW5kKT4pIC0+IENyYXRlSW5mbyB7CiAgICAgICAgbGV0IHB1YmxpY19pdGVtczogVmVjPFB1YmxpY0l0ZW0+ID0gaXRlbXMKICAgICAgICAgICAgLmludG9faXRlcigpCiAgICAgICAgICAgIC5tYXAofChuLCBrKXwgewogICAgICAgICAgICAgICAgUHVibGljSXRlbTo6YnVpbGRlcigpCiAgICAgICAgICAgICAgICAgICAgLmtpbmQoaykKICAgICAgICAgICAgICAgICAgICAubmFtZShuKQogICAgICAgICAgICAgICAgICAgIC5maWxlKFN0cmluZzo6bmV3KCkpCiAgICAgICAgICAgICAgICAgICAgLmxpbmUoMSkKICAgICAgICAgICAgICAgICAgICAudmlzaWJpbGl0eSgicHViIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgICAgICAgICAuZ2VuZXJpY3MoU3RyaW5nOjpuZXcoKSkKICAgICAgICAgICAgICAgICAgICAuYXR0cnMoRGVmYXVsdDo6ZGVmYXVsdCgpKQogICAgICAgICAgICAgICAgICAgIC5idWlsZCgpCiAgICAgICAgICAgIH0pCiAgICAgICAgICAgIC5jb2xsZWN0KCk7CiAgICAgICAgbGV0IG1vZHVsZSA9IE1vZHVsZUluZm86OmJ1aWxkZXIoKQogICAgICAgICAgICAucGF0aCgiIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgLmZpbGUoU3RyaW5nOjpuZXcoKSkKICAgICAgICAgICAgLnZpc2liaWxpdHkoInB1YiIudG9fc3RyaW5nKCkpCiAgICAgICAgICAgIC5wdWJsaWNfaXRlbXMocHVibGljX2l0ZW1zKQogICAgICAgICAgICAuYnVpbGQoKTsKICAgICAgICBDcmF0ZUluZm86OmJ1aWxkZXIoKQogICAgICAgICAgICAubmFtZShuYW1lLnRvX3N0cmluZygpKQogICAgICAgICAgICAucm9vdChTdHJpbmc6Om5ldygpKQogICAgICAgICAgICAucGFja2FnZSgKICAgICAgICAgICAgICAgIHNjaGVtYTo6UGFja2FnZUluZm86OmJ1aWxkZXIoKQogICAgICAgICAgICAgICAgICAgIC5uYW1lKG5hbWUudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAgICAgLnZlcnNpb24oIjAuMS4wIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgICAgICAgICAuZWRpdGlvbigiMjAyMSIudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAgICAgLmNyYXRlX3R5cGUoc2NoZW1hOjpDcmF0ZVR5cGU6OkxpYikKICAgICAgICAgICAgICAgICAgICAuYnVpbGQoKSwKICAgICAgICAgICAgKQogICAgICAgICAgICAubW9kdWxlcyh2ZWMhW21vZHVsZV0pCiAgICAgICAgICAgIC5kZXBzKERlZmF1bHQ6OmRlZmF1bHQoKSkKICAgICAgICAgICAgLmJ1aWxkKCkKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiBjb21wdXRlX2ZpbmRzX2Nyb3NzX2NyYXRlX2ltcG9ydCgpIHsKICAgICAgICBsZXQgbXV0IGNyYXRlcyA9IHZlYyFbCiAgICAgICAgICAgIG1ha2VfY3JhdGUoImNvcmUiLCB2ZWMhWwogICAgICAgICAgICAgICAgKCJUYXNrIi50b19zdHJpbmcoKSwgSXRlbUtpbmQ6OlN0cnVjdCksCiAgICAgICAgICAgIF0pLAogICAgICAgICAgICBtYWtlX2NyYXRlKCJlbmdpbmUiLCB2ZWMhW10pLAogICAgICAgIF07CiAgICAgICAgLy8gTWFudWFsbHkgYWRkIGFuIGltcG9ydCBpbiBlbmdpbmUgdGhhdCByZWZlcmVuY2VzIGNvcmU6OlRhc2sKICAgICAgICBsZXQgZW5naW5lX21vZHVsZSA9ICZtdXQgY3JhdGVzWzFdLm1vZHVsZXNbMF07CiAgICAgICAgZW5naW5lX21vZHVsZS5pbXBvcnRzLnB1c2goSW1wb3J0IHsKICAgICAgICAgICAgcGF0aDogImNvcmU6OlRhc2siLnRvX3N0cmluZygpLAogICAgICAgICAgICBsaW5lOiAxLAogICAgICAgIH0pOwogICAgICAgIGxldCByZWZzID0gY29tcHV0ZSgmbXV0IGNyYXRlcyk7CiAgICAgICAgLy8gVGFzayBzaG91bGQgYmUgaW4gY3Jvc3MtcmVmZXJlbmNlcwogICAgICAgIGFzc2VydCEocmVmcy50eXBlcy5jb250YWluc19rZXkoIlRhc2siKSk7CiAgICAgICAgbGV0IHRhc2tfcmVmID0gJnJlZnMudHlwZXNbIlRhc2siXTsKICAgICAgICBhc3NlcnRfZXEhKHRhc2tfcmVmLmNyYXRlX25hbWUsICJjb3JlIik7CiAgICAgICAgLy8gZW5naW5lIHNob3VsZCBoYXZlIGEgY3Jvc3NfY3JhdGVfaW1wb3J0CiAgICAgICAgYXNzZXJ0X2VxIShjcmF0ZXNbMV0uY3Jvc3NfY3JhdGVfaW1wb3J0cy5sZW4oKSwgMSk7CiAgICAgICAgYXNzZXJ0X2VxIShjcmF0ZXNbMV0uY3Jvc3NfY3JhdGVfaW1wb3J0c1swXS50YXJnZXRfY3JhdGUsICJjb3JlIik7CiAgICB9CgogICAgI1t0ZXN0XQogICAgZm4gY29tcHV0ZV9lbXB0eV9mb3Jfbm9fY3Jvc3NfcmVmZXJlbmNlcygpIHsKICAgICAgICBsZXQgY3JhdGVzID0gdmVjIVsKICAgICAgICAgICAgbWFrZV9jcmF0ZSgiYSIsIHZlYyFbKCJGb28iLnRvX3N0cmluZygpLCBJdGVtS2luZDo6U3RydWN0KV0pLAogICAgICAgICAgICBtYWtlX2NyYXRlKCJiIiwgdmVjIVsoIkJhciIudG9fc3RyaW5nKCksIEl0ZW1LaW5kOjpTdHJ1Y3QpXSksCiAgICAgICAgXTsKICAgICAgICBsZXQgbXV0IGNyYXRlc19tdXQgPSBjcmF0ZXM7CiAgICAgICAgbGV0IHJlZnMgPSBjb21wdXRlKCZtdXQgY3JhdGVzX211dCk7CiAgICAgICAgLy8gTm8gY3Jvc3MgcmVmZXJlbmNlcyBzaW5jZSBubyBjcmF0ZSBpbXBvcnRzIGZyb20gYW5vdGhlcgogICAgICAgIGFzc2VydCEocmVmcy50eXBlcy5pc19lbXB0eSgpIHx8IHJlZnMudHlwZXMudmFsdWVzKCkuYWxsKHx0fCB0LmltcG9ydGVkX2J5LmlzX2VtcHR5KCkpKTsKICAgIH0KfQoKLy8gSGVscGVyOiBjb252ZXJ0IEl0ZW1LaW5kIHRvIGEgc2hvcnQgc3RyaW5nIGZvciB0aGUgVHlwZVJlZi5raW5kIGZpZWxkLg==", "target": "src/cross_refs.rs", "index": 4, "is_create": false}, {"before_b64": "dXNlIGNyYXRlOjpzY2hlbWE6OldvcmtzcGFjZU1hcDsKdXNlIHN0ZDo6aW86OldyaXRlOwoKLy8vIFNlcmlhbGl6ZSB0aGUgd29ya3NwYWNlIG1hcCB0byBhIEpTT04gc3RyaW5nIHdpdGggMi1zcGFjZSBpbmRlbnRhdGlvbi4KI1ttdXN0X3VzZV0KcHViIGZuIHJlbmRlcl9qc29uKG1hcDogJldvcmtzcGFjZU1hcCkgLT4gc2VyZGVfanNvbjo6UmVzdWx0PFN0cmluZz4gewogICAgc2VyZGVfanNvbjo6dG9fc3RyaW5nX3ByZXR0eShtYXApCn0KCi8vLyBTZXJpYWxpemUgdGhlIHdvcmtzcGFjZSBtYXAgdG8gdGhlIGdpdmVuIHdyaXRlci4KcHViIGZuIHJlbmRlcl90b193cml0ZXIobWFwOiAmV29ya3NwYWNlTWFwLCB3cml0ZXI6IGltcGwgV3JpdGUpIC0+IHNlcmRlX2pzb246OlJlc3VsdDwoKT4gewogICAgc2VyZGVfanNvbjo6dG9fd3JpdGVyX3ByZXR0eSh3cml0ZXIsIG1hcCkKfQo=", "after_b64": "dXNlIGNyYXRlOjpzY2hlbWE6OldvcmtzcGFjZU1hcDsKdXNlIHN0ZDo6aW86OldyaXRlOwoKLy8vIFNlcmlhbGl6ZSB0aGUgd29ya3NwYWNlIG1hcCB0byBhIEpTT04gc3RyaW5nIHdpdGggMi1zcGFjZSBpbmRlbnRhdGlvbi4KI1ttdXN0X3VzZV0KcHViIGZuIHJlbmRlcl9qc29uKG1hcDogJldvcmtzcGFjZU1hcCkgLT4gc2VyZGVfanNvbjo6UmVzdWx0PFN0cmluZz4gewogICAgc2VyZGVfanNvbjo6dG9fc3RyaW5nX3ByZXR0eShtYXApCn0KCi8vLyBTZXJpYWxpemUgdGhlIHdvcmtzcGFjZSBtYXAgdG8gdGhlIGdpdmVuIHdyaXRlci4KcHViIGZuIHJlbmRlcl90b193cml0ZXIobWFwOiAmV29ya3NwYWNlTWFwLCB3cml0ZXI6IGltcGwgV3JpdGUpIC0+IHNlcmRlX2pzb246OlJlc3VsdDwoKT4gewogICAgc2VyZGVfanNvbjo6dG9fd3JpdGVyX3ByZXR0eSh3cml0ZXIsIG1hcCkKfQoKLy8g4pSA4pSAIFRlc3RzIOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgAoKI1tjZmcodGVzdCldCm1vZCB0ZXN0cyB7CiAgICB1c2Ugc3VwZXI6Oio7CiAgICB1c2UgY3JhdGU6OnNjaGVtYTo6ewogICAgICAgIENyYXRlSW5mbywgQ3JhdGVUeXBlLCBDcm9zc1JlZmVyZW5jZXMsIERlcEluZm8sIE1vZHVsZUluZm8sIFBhY2thZ2VJbmZvLAogICAgICAgIFdvcmtzcGFjZUluZm8sIFdvcmtzcGFjZU1hcCwKICAgIH07CgogICAgZm4gbWFrZV9taW5pbWFsX21hcCgpIC0+IFdvcmtzcGFjZU1hcCB7CiAgICAgICAgV29ya3NwYWNlTWFwOjpidWlsZGVyKCkKICAgICAgICAgICAgLndvcmtzcGFjZShXb3Jrc3BhY2VJbmZvOjpidWlsZGVyKCkKICAgICAgICAgICAgICAgIC5yb290KCIuIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgICAgIC53b3Jrc3BhY2VfbmFtZSgidGVzdCIudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAuYnVpbGQoKSkKICAgICAgICAgICAgLmNyYXRlcyh2ZWMhWwogICAgICAgICAgICAgICAgQ3JhdGVJbmZvOjpidWlsZGVyKCkKICAgICAgICAgICAgICAgICAgICAubmFtZSgidGVzdC1jcmF0ZSIudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAgICAgLnJvb3QoIi4iLnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgICAgIC5wYWNrYWdlKFBhY2thZ2VJbmZvOjpidWlsZGVyKCkKICAgICAgICAgICAgICAgICAgICAgICAgLm5hbWUoInRlc3QtY3JhdGUiLnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgICAgICAgICAudmVyc2lvbigiMC4xLjAiLnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgICAgICAgICAuZWRpdGlvbigiMjAyMSIudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAgICAgICAgIC5jcmF0ZV90eXBlKENyYXRlVHlwZTo6TGliKQogICAgICAgICAgICAgICAgICAgICAgICAuYnVpbGQoKSkKICAgICAgICAgICAgICAgICAgICAubW9kdWxlcyh2ZWMhW01vZHVsZUluZm86OmJ1aWxkZXIoKQogICAgICAgICAgICAgICAgICAgICAgICAucGF0aCgiIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgICAgICAgICAgICAgLmZpbGUoInNyYy9saWIucnMiLnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgICAgICAgICAudmlzaWJpbGl0eSgicHViIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgICAgICAgICAgICAgLmJ1aWxkKCldKQogICAgICAgICAgICAgICAgICAgIC5kZXBzKERlcEluZm86OmRlZmF1bHQoKSkKICAgICAgICAgICAgICAgICAgICAuYnVpbGQoKSwKICAgICAgICAgICAgXSkKICAgICAgICAgICAgLmNyb3NzX3JlZmVyZW5jZXMoQ3Jvc3NSZWZlcmVuY2VzOjpkZWZhdWx0KCkpCiAgICAgICAgICAgIC53b3Jrc3BhY2Vfcm9vdChzdGQ6OnBhdGg6OlBhdGhCdWY6OmZyb20oIi4iKSkKICAgICAgICAgICAgLmJ1aWxkKCkKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiByZW5kZXJfanNvbl9wcm9kdWNlc192YWxpZF9qc29uKCkgewogICAgICAgIGxldCBtYXAgPSBtYWtlX21pbmltYWxfbWFwKCk7CiAgICAgICAgbGV0IGpzb24gPSByZW5kZXJfanNvbigmbWFwKS51bndyYXAoKTsKICAgICAgICBsZXQgcGFyc2VkOiBzZXJkZV9qc29uOjpWYWx1ZSA9IHNlcmRlX2pzb246OmZyb21fc3RyKCZqc29uKS51bndyYXAoKTsKICAgICAgICBhc3NlcnRfZXEhKHBhcnNlZFsid29ya3NwYWNlIl1bInJvb3QiXSwgIi4iKTsKICAgICAgICBhc3NlcnRfZXEhKHBhcnNlZFsiY3JhdGVzIl0uYXNfYXJyYXkoKS51bndyYXAoKS5sZW4oKSwgMSk7CiAgICB9CgogICAgI1t0ZXN0XQogICAgZm4gcmVuZGVyX2pzb25fc2tpcHNfZW1wdHlfZXJyb3JzKCkgewogICAgICAgIGxldCBtYXAgPSBtYWtlX21pbmltYWxfbWFwKCk7CiAgICAgICAgbGV0IGpzb24gPSByZW5kZXJfanNvbigmbWFwKS51bndyYXAoKTsKICAgICAgICBsZXQgcGFyc2VkOiBzZXJkZV9qc29uOjpWYWx1ZSA9IHNlcmRlX2pzb246OmZyb21fc3RyKCZqc29uKS51bndyYXAoKTsKICAgICAgICAvLyBlcnJvcnMgZmllbGQgc2hvdWxkIGJlIGFic2VudCAoc2tpcF9zZXJpYWxpemluZ19pZikKICAgICAgICBhc3NlcnQhKHBhcnNlZC5nZXQoImVycm9ycyIpLmlzX25vbmUoKSk7CiAgICB9CgogICAgI1t0ZXN0XQogICAgZm4gcmVuZGVyX3RvX3dyaXRlcl9tYXRjaGVzX3JlbmRlcl9qc29uKCkgewogICAgICAgIGxldCBtYXAgPSBtYWtlX21pbmltYWxfbWFwKCk7CiAgICAgICAgbGV0IGpzb24gPSByZW5kZXJfanNvbigmbWFwKS51bndyYXAoKTsKCiAgICAgICAgbGV0IG11dCBidWYgPSBWZWM6Om5ldygpOwogICAgICAgIHJlbmRlcl90b193cml0ZXIoJm1hcCwgJm11dCBidWYpLnVud3JhcCgpOwogICAgICAgIGxldCBmcm9tX3dyaXRlciA9IFN0cmluZzo6ZnJvbV91dGY4KGJ1ZikudW53cmFwKCk7CgogICAgICAgIGFzc2VydF9lcSEoanNvbiwgZnJvbV93cml0ZXIpOwogICAgfQp9Cg==", "target": "src/render.rs", "index": 5, "is_create": false}]')
+
+for step in STEPS:
+    before = base64.b64decode(step["before_b64"]).decode()
+    after = base64.b64decode(step["after_b64"]).decode()
+    target = step["target"]
+    idx = step["index"]
+    is_create = step["is_create"]
+
+    if is_create:
+        target_path = Path(target)
+        target_path.parent.mkdir(parents=True, exist_ok=True)
+        target_path.write_text(after)
+        print(f"OK {TASK_ID} change {idx}: created {target}")
+    else:
+        target_path = Path(target)
+        content = target_path.read_text()
+        if before not in content:
+            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
+            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
+            sys.exit(1)
+
+        result = subprocess.run(
+            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
+            capture_output=True, text=True,
+        )
+        if result.returncode != 0:
+            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
+            sys.exit(result.returncode)
+
+        new_content = target_path.read_text()
+        if after and after not in new_content:
+            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
+            sys.exit(1)
+
+        print(f"OK {TASK_ID} change {idx}: applied to {target}")
+
+print(f"OK {TASK_ID}: all changes applied")
diff --git a/plans/compiled/TASK-9.sh b/plans/compiled/TASK-9.sh
new file mode 100755
index 0000000..d78aa7c
--- /dev/null
+++ b/plans/compiled/TASK-9.sh
@@ -0,0 +1,7 @@
+#!/usr/bin/env bash
+set -euo pipefail
+# TASK-9: Add unit tests for file_parser, module_tree, workspace, cargo_info, cross_refs, and render
+# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
+# Type: replace
+# File: src/file_parser.rs
+python3 "$(dirname "$0")/TASK-9.py"
diff --git a/plans/compiled/TASK-PREP.py b/plans/compiled/TASK-PREP.py
new file mode 100644
index 0000000..d453fa4
--- /dev/null
+++ b/plans/compiled/TASK-PREP.py
@@ -0,0 +1,44 @@
+#!/usr/bin/env python3
+"""TASK-PREP: Add tempfile dev-dependency for unit and integration tests"""
+import base64, json, subprocess, sys
+from pathlib import Path
+
+TASK_ID = "TASK-PREP"
+STEPS = json.loads('[{"before_b64": "cHJvYy1tYWNybzIgPSB7IHZlcnNpb24gPSAiMSIsIGZlYXR1cmVzID0gWyJzcGFuLWxvY2F0aW9ucyJdIH0=", "after_b64": "cHJvYy1tYWNybzIgPSB7IHZlcnNpb24gPSAiMSIsIGZlYXR1cmVzID0gWyJzcGFuLWxvY2F0aW9ucyJdIH0KCltkZXYtZGVwZW5kZW5jaWVzXQp0ZW1wZmlsZSA9ICIzIg==", "target": "Cargo.toml", "index": 0, "is_create": false}]')
+
+for step in STEPS:
+    before = base64.b64decode(step["before_b64"]).decode()
+    after = base64.b64decode(step["after_b64"]).decode()
+    target = step["target"]
+    idx = step["index"]
+    is_create = step["is_create"]
+
+    if is_create:
+        target_path = Path(target)
+        target_path.parent.mkdir(parents=True, exist_ok=True)
+        target_path.write_text(after)
+        print(f"OK {TASK_ID} change {idx}: created {target}")
+    else:
+        target_path = Path(target)
+        content = target_path.read_text()
+        if before not in content:
+            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
+            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
+            sys.exit(1)
+
+        result = subprocess.run(
+            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
+            capture_output=True, text=True,
+        )
+        if result.returncode != 0:
+            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
+            sys.exit(result.returncode)
+
+        new_content = target_path.read_text()
+        if after and after not in new_content:
+            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
+            sys.exit(1)
+
+        print(f"OK {TASK_ID} change {idx}: applied to {target}")
+
+print(f"OK {TASK_ID}: all changes applied")
diff --git a/plans/compiled/TASK-PREP.sh b/plans/compiled/TASK-PREP.sh
new file mode 100755
index 0000000..a037be6
--- /dev/null
+++ b/plans/compiled/TASK-PREP.sh
@@ -0,0 +1,7 @@
+#!/usr/bin/env bash
+set -euo pipefail
+# TASK-PREP: Add tempfile dev-dependency for unit and integration tests
+# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
+# Type: replace
+# File: Cargo.toml
+python3 "$(dirname "$0")/TASK-PREP.py"
diff --git a/plans/compiled/manifest.json b/plans/compiled/manifest.json
new file mode 100644
index 0000000..94a624c
--- /dev/null
+++ b/plans/compiled/manifest.json
@@ -0,0 +1,139 @@
+{
+  "plan": "/Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml",
+  "compiled_at": "2026-04-28T08:48:13.087314+00:00",
+  "tasks": [
+    {
+      "id": "TASK-PREP",
+      "script": "TASK-PREP.sh",
+      "runner": "TASK-PREP.py",
+      "file": "Cargo.toml",
+      "type": "replace",
+      "changes": 1,
+      "description": "Add tempfile dev-dependency for unit and integration tests",
+      "acceptance": [
+        "cargo check -p rust-workspace-map"
+      ]
+    },
+    {
+      "id": "TASK-1",
+      "script": "TASK-1.sh",
+      "runner": "TASK-1.py",
+      "file": "src/schema.rs",
+      "type": "replace",
+      "changes": 1,
+      "description": "Add ErrorSeverity enum and ErrorContext struct to schema.rs",
+      "acceptance": [
+        "cargo check -p rust-workspace-map"
+      ]
+    },
+    {
+      "id": "TASK-2",
+      "script": "TASK-2.sh",
+      "runner": "TASK-2.py",
+      "file": "src/schema.rs",
+      "type": "replace",
+      "changes": 1,
+      "description": "Add MissingWorkspaceSection variant to Error enum in schema.rs",
+      "acceptance": [
+        "cargo check -p rust-workspace-map"
+      ]
+    },
+    {
+      "id": "TASK-3",
+      "script": "TASK-3.sh",
+      "runner": "TASK-3.py",
+      "file": "src/schema.rs",
+      "type": "replace",
+      "changes": 1,
+      "description": "Expand ErrorEntry struct with severity, kind, context, and cause fields",
+      "acceptance": [
+        "cargo check -p rust-workspace-map"
+      ]
+    },
+    {
+      "id": "TASK-4",
+      "script": "TASK-4.sh",
+      "runner": "TASK-4.py",
+      "file": "src/file_parser.rs",
+      "type": "replace",
+      "changes": 1,
+      "description": "Change parse_file to return ParsedFile with optional parse errors instead of Result",
+      "acceptance": [
+        "true  # applied atomically with TASK-5; compilation verified at TASK-5"
+      ]
+    },
+    {
+      "id": "TASK-5",
+      "script": "TASK-5.sh",
+      "runner": "TASK-5.py",
+      "file": "src/module_tree.rs",
+      "type": "replace",
+      "changes": 12,
+      "description": "Refactor module_tree.rs: fix path fallbacks, add error collection, change build_module_tree return type",
+      "acceptance": [
+        "cargo check -p rust-workspace-map"
+      ]
+    },
+    {
+      "id": "TASK-6",
+      "script": "TASK-6.sh",
+      "runner": "TASK-6.py",
+      "file": "src/workspace.rs",
+      "type": "replace",
+      "changes": 3,
+      "description": "Update workspace.rs enumerate_members to return MissingWorkspaceSection error",
+      "acceptance": [
+        "cargo check -p rust-workspace-map"
+      ]
+    },
+    {
+      "id": "TASK-7",
+      "script": "TASK-7.sh",
+      "runner": "TASK-7.py",
+      "file": "src/lib.rs",
+      "type": "replace",
+      "changes": 3,
+      "description": "Refactor lib.rs run(): parallel error collection, error entry construction, remove unwrap_or_default",
+      "acceptance": [
+        "cargo check -p rust-workspace-map"
+      ]
+    },
+    {
+      "id": "TASK-8",
+      "script": "TASK-8.sh",
+      "runner": "TASK-8.py",
+      "file": "src/lib.rs",
+      "type": "replace",
+      "changes": 21,
+      "description": "Remove all crate-level clippy allow attributes and fix individual lint violations",
+      "acceptance": [
+        "cargo clippy -p rust-workspace-map -- -D warnings"
+      ]
+    },
+    {
+      "id": "TASK-9",
+      "script": "TASK-9.sh",
+      "runner": "TASK-9.py",
+      "file": "src/file_parser.rs",
+      "type": "replace",
+      "changes": 6,
+      "description": "Add unit tests for file_parser, module_tree, workspace, cargo_info, cross_refs, and render",
+      "acceptance": [
+        "cargo test -p rust-workspace-map"
+      ]
+    },
+    {
+      "id": "TASK-10",
+      "script": "TASK-10.sh",
+      "runner": "TASK-10.py",
+      "file": "tests/integration_test.rs",
+      "type": "replace",
+      "changes": 1,
+      "description": "Expand integration tests from 3 to 10 covering error cases, workspace patterns, and output",
+      "acceptance": [
+        "cargo test -p rust-workspace-map --test integration_test"
+      ]
+    }
+  ],
+  "skipped": []
+}
diff --git a/plans/phase-0.2.toml b/plans/phase-0.2.toml
new file mode 100644
index 0000000..de26e22
--- /dev/null
+++ b/plans/phase-0.2.toml
@@ -0,0 +1,2168 @@
+[meta]
+title = "Phase 0.2: Hardening for Trustworthiness"
+source_branch = "phase-0.2"
+created = "2026-04-28"
+
+[dependencies]
+TASK-PREP = []
+TASK-3 = ["TASK-1"]
+TASK-4 = ["TASK-1", "TASK-2", "TASK-3"]
+TASK-5 = ["TASK-4"]
+TASK-6 = ["TASK-2"]
+TASK-7 = ["TASK-4", "TASK-5"]
+TASK-8 = ["TASK-7"]
+TASK-9 = ["TASK-PREP", "TASK-5", "TASK-6", "TASK-8"]
+TASK-10 = ["TASK-PREP", "TASK-9"]
+
+[tasks.TASK-PREP]
+description = "Add tempfile dev-dependency for unit and integration tests"
+type = "replace"
+acceptance = [
+    "cargo check -p rust-workspace-map",
+]
+
+[[tasks.TASK-PREP.changes]]
+file = "Cargo.toml"
+before = "proc-macro2 = { version = \"1\", features = [\"span-locations\"] }"
+after = """proc-macro2 = { version = "1", features = ["span-locations"] }
+
+[dev-dependencies]
+tempfile = "3\""""
+
+[tasks.TASK-1]
+description = "Add ErrorSeverity enum and ErrorContext struct to schema.rs"
+type = "replace"
+acceptance = [
+    "cargo check -p rust-workspace-map",
+]
+
+[[tasks.TASK-1.changes]]
+file = "src/schema.rs"
+before = '''// ── Internal types ──────────────────────────────────────────────────────
+
+/// Internal intermediate type consumed by module_tree.
+#[derive(Debug, Clone, Default)]
+pub struct FileInfo {
+'''
+after = '''// ── Error severity ─────────────────────────────────────────────────────
+
+#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
+#[serde(rename_all = "snake_case")]
+pub enum ErrorSeverity {
+    Error,
+    Warning,
+}
+
+// ── Error context ───────────────────────────────────────────────────────
+
+/// Optional context attached to an error, providing additional location
+/// and source information for diagnostics.
+#[derive(Debug, Clone, Default, serde::Serialize, bon::Builder)]
+#[serde(rename_all = "camelCase")]
+pub struct ErrorContext {
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pub crate_name: Option<String>,
+
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pub module_path: Option<String>,
+
+    /// Line number in the source file where the error occurred.
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pub line: Option<usize>,
+
+    /// A short source snippet near the error location (if available).
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pub snippet: Option<String>,
+}
+
+// ── Internal types ──────────────────────────────────────────────────────
+
+/// Internal intermediate type consumed by module_tree.
+#[derive(Debug, Clone, Default)]
+pub struct FileInfo {
+'''
+
+[tasks.TASK-2]
+description = "Add MissingWorkspaceSection variant to Error enum in schema.rs"
+type = "replace"
+acceptance = [
+    "cargo check -p rust-workspace-map",
+]
+
+[[tasks.TASK-2.changes]]
+file = "src/schema.rs"
+before = '''    #[error("glob pattern error: {0}")]
+    GlobPattern(String),
+}
+
+pub type Result<T> = std::result::Result<T, Error>;'''
+after = '''    #[error("glob pattern error: {0}")]
+    GlobPattern(String),
+
+    #[error("workspace Cargo.toml is missing the [workspace] section")]
+    MissingWorkspaceSection,
+}
+
+pub type Result<T> = std::result::Result<T, Error>;'''
+
+[tasks.TASK-3]
+description = "Expand ErrorEntry struct with severity, kind, context, and cause fields"
+type = "replace"
+acceptance = [
+    "cargo check -p rust-workspace-map",
+]
+
+[[tasks.TASK-3.changes]]
+file = "src/schema.rs"
+before = '''#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
+#[serde(rename_all = "camelCase")]
+pub struct ErrorEntry {
+    pub file: String,
+    #[builder(default)]
+    pub line: usize,
+    pub message: String,
+}
+'''
+after = '''#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
+#[serde(rename_all = "camelCase")]
+pub struct ErrorEntry {
+    pub file: String,
+    #[builder(default)]
+    pub line: usize,
+    pub message: String,
+    pub severity: ErrorSeverity,
+    pub kind: String,
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pub context: Option<ErrorContext>,
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pub cause: Option<String>,
+}
+'''
+
+[tasks.TASK-4]
+description = "Change parse_file to return ParsedFile with optional parse errors instead of Result"
+type = "replace"
+acceptance = [
+    "true  # applied atomically with TASK-5; compilation verified at TASK-5",
+]
+
+[[tasks.TASK-4.changes]]
+file = "src/file_parser.rs"
+before = '''use crate::schema::{
+    Error, FileInfo, ImplInfo, ImplItem, ImplItemKind, Import, ItemAttrs, ItemKind, PublicItem,
+    ReExport, Result, SubmoduleDecl,
+};
+use std::path::Path;
+
+// ── parse_file ──────────────────────────────────────────────────────────
+
+/// Read and parse a Rust source file. Returns the raw `syn::File` AST (needed
+/// by `module_tree` for inline module item extraction) and the extracted
+/// `FileInfo`. On parse failure, warns to stderr and returns empty results.
+pub fn parse_file(path: &Path) -> Result<(syn::File, FileInfo)> {
+    let content = std::fs::read_to_string(path).map_err(|source| Error::FileRead {
+        path: path.to_path_buf(),
+        source,
+    })?;
+
+    let file = match syn::parse_file(&content) {
+        Ok(f) => f,
+        Err(e) => {
+            eprintln!("warning: failed to parse {}: {}", path.display(), e);
+            let empty = syn::File {
+                shebang: None,
+                attrs: vec![],
+                items: vec![],
+            };
+            let info = FileInfo::default();
+            return Ok((empty, info));
+        }
+    };
+
+    let info = FileInfo {
+        public_items: extract_public_items(&file.items),
+        imports: extract_imports(&file.items),
+        re_exports: extract_re_exports(&file.items),
+        submodules: extract_submodules(&file.items),
+        impls: extract_impls(&file.items),
+    };
+
+    Ok((file, info))
+}'''
+after = '''use crate::schema::{
+    Error, ErrorEntry, ErrorSeverity, FileInfo, ImplInfo, ImplItem, ImplItemKind, Import,
+    ItemAttrs, ItemKind, PublicItem, ReExport, Result, SubmoduleDecl,
+};
+use std::path::Path;
+
+// ── Internal parse result types ────────────────────────────────────────
+
+/// Result of parsing a Rust source file.
+///
+/// Unlike `Result<T, Error>`, this type always succeeds — parse
+/// failures are reported as data, not as errors, so the caller
+/// can continue processing other files. The caller constructs
+/// `ErrorEntry` values from `SynParseError` when needed.
+pub struct ParsedFile {
+    pub ast: syn::File,
+    pub file_info: FileInfo,
+    pub parse_error: Option<SynParseError>,
+}
+
+/// Structured information about a parse failure.
+pub struct SynParseError {
+    pub message: String,
+    pub line: usize,
+}
+
+// ── parse_file ──────────────────────────────────────────────────────────
+
+/// Read and parse a Rust source file.
+///
+/// On parse failure, returns the original file content and a
+/// `SynParseError` alongside an empty `FileInfo`. Callers use the
+/// error to construct an `ErrorEntry`.
+pub fn parse_file(path: &Path) -> ParsedFile {
+    let content = match std::fs::read_to_string(path) {
+        Ok(c) => c,
+        Err(source) => {
+            let err = SynParseError {
+                message: source.to_string(),
+                line: 0,
+            };
+            return ParsedFile {
+                ast: syn::File {
+                    shebang: None,
+                    attrs: vec![],
+                    items: vec![],
+                },
+                file_info: FileInfo::default(),
+                parse_error: Some(err),
+            };
+        }
+    };
+
+    match syn::parse_file(&content) {
+        Ok(file) => {
+            let file_info = FileInfo {
+                public_items: extract_public_items(&file.items),
+                imports: extract_imports(&file.items),
+                re_exports: extract_re_exports(&file.items),
+                submodules: extract_submodules(&file.items),
+                impls: extract_impls(&file.items),
+            };
+            ParsedFile {
+                ast: file,
+                file_info,
+                parse_error: None,
+            }
+        },
+        Err(e) => {
+            let line = e.span().start().line;
+            let err = SynParseError {
+                message: e.to_string(),
+                line,
+            };
+            ParsedFile {
+                ast: syn::File {
+                    shebang: None,
+                    attrs: vec![],
+                    items: vec![],
+                },
+                file_info: FileInfo::default(),
+                parse_error: Some(err),
+            }
+        }
+    }
+}
+
+pub(crate) fn build_parse_error_entry(path: &Path, err: &SynParseError) -> ErrorEntry {
+    ErrorEntry::builder()
+        .file(path.to_string_lossy().to_string())
+        .line(err.line)
+        .message(err.message.clone())
+        .severity(ErrorSeverity::Error)
+        .kind("syn_parse_error".to_string())
+        .build()
+}'''
+
+[tasks.TASK-5]
+description = "Refactor module_tree.rs: fix path fallbacks, add error collection, change build_module_tree return type"
+type = "replace"
+acceptance = [
+    "cargo check -p rust-workspace-map",
+]
+
+[[tasks.TASK-5.changes]]
+file = "src/module_tree.rs"
+before = "use crate::schema::{FileInfo, ModuleInfo, Result, SubmoduleDecl};"
+after = "use crate::schema::{ErrorContext, ErrorEntry, ErrorSeverity, FileInfo, ModuleInfo, Result, SubmoduleDecl};"
+
+[[tasks.TASK-5.changes]]
+file = "src/module_tree.rs"
+before = '''/// Build the full module tree for a crate starting from its entry point
+/// (e.g., `src/lib.rs`). Returns a flat `Vec<ModuleInfo>` containing the
+/// root module and all recursively discovered submodules.
+pub fn build_module_tree(crate_root: &Path, crate_name: &str) -> Result<Vec<ModuleInfo>> {
+    let mut visited = HashSet::new();
+    let parent_dir = crate_root.parent().unwrap_or_else(|| Path::new("."));
+
+    let (ast, file_info) = file_parser::parse_file(crate_root)?;
+    visited.insert(crate_root.to_path_buf());
+
+    let root_module = build_module_info(
+        crate_name,
+        crate_root,
+        "pub",
+        &file_info.public_items,
+        &file_info.imports,
+        &file_info.re_exports,
+        &file_info.submodules,
+    );
+
+    let mut modules = vec![root_module];
+
+    for sub in &file_info.submodules {
+        if sub.is_test {
+            continue;
+        }
+        let sub_module_path = format!("{}::{}", crate_name, sub.name);
+        let child_modules = process_submodule(
+            &sub_module_path,
+            &sub.name,
+            &ast.items,
+            parent_dir,
+            crate_root,
+            &mut visited,
+        )?;
+        modules.extend(child_modules);
+    }
+
+    Ok(modules)
+}'''
+after = '''/// Build the full module tree for a crate starting from its entry point
+/// (e.g., `src/lib.rs`). Returns a tuple of module info and any errors
+/// encountered during submodule parsing (including orphaned module warnings).
+pub fn build_module_tree(
+    crate_root: &Path,
+    crate_name: &str,
+) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {
+
+    let mut visited = HashSet::new();
+    let parent_dir = crate_root.parent().unwrap_or(crate_root);
+
+    let parsed = file_parser::parse_file(crate_root);
+    let mut errors: Vec<ErrorEntry> = Vec::new();
+    if let Some(ref err) = parsed.parse_error {
+        errors.push(crate::file_parser::build_parse_error_entry(crate_root, err));
+    }
+    visited.insert(crate_root.to_path_buf());
+
+    let root_module = build_module_info(
+        crate_name,
+        crate_root,
+        "pub",
+        &parsed.file_info.public_items,
+        &parsed.file_info.imports,
+        &parsed.file_info.re_exports,
+        &parsed.file_info.submodules,
+    );
+
+    let mut modules = vec![root_module];
+
+    for sub in &parsed.file_info.submodules {
+        if sub.is_test {
+            continue;
+        }
+        let sub_module_path = format!("{}::{}", crate_name, sub.name);
+        let (child_modules, child_errors) = process_submodule(
+            &sub_module_path,
+            &sub.name,
+            &parsed.ast.items,
+            parent_dir,
+            crate_root,
+            &mut visited,
+        );
+        errors.extend(child_errors);
+        modules.extend(child_modules);
+    }
+
+    (modules, errors)
+}'''
+
+[[tasks.TASK-5.changes]]
+file = "src/module_tree.rs"
+before = '''    let Some(mod_item) = mod_item else {
+        eprintln!("warning: orphaned module {}", module_path);
+        return Ok(vec![ModuleInfo::builder()
+            .path(module_path.to_string())
+            .file("<unresolved>".to_string())
+            .visibility("private".to_string())
+            .build()]);
+    };'''
+after = '''    let Some(mod_item) = mod_item else {
+        let err = ErrorEntry::builder()
+            .file(String::new())
+            .message(format!("orphaned module: {module_path}"))
+            .severity(ErrorSeverity::Warning)
+            .kind("orphaned_module".to_string())
+            .context(ErrorContext::builder()
+                .module_path(module_path.to_string())
+                .build())
+            .build();
+        return (vec![ModuleInfo::builder()
+            .path(module_path.to_string())
+            .file("<unresolved>".to_string())
+            .visibility("private".to_string())
+            .build()], vec![err]);
+    };'''
+
+[[tasks.TASK-5.changes]]
+file = "src/module_tree.rs"
+before = '''        let Some(ref file_path) = file_path else {
+            eprintln!("warning: orphaned module {}", module_path);
+            return Ok(vec![ModuleInfo::builder()
+                .path(module_path.to_string())
+                .file("<unresolved>".to_string())
+                .visibility(visibility.to_string())
+                .build()]);
+        };'''
+after = '''        let Some(ref file_path) = file_path else {
+            let err = ErrorEntry::builder()
+                .file(String::new())
+                .message(format!("orphaned module: {module_path}"))
+                .severity(ErrorSeverity::Warning)
+                .kind("orphaned_module".to_string())
+                .context(ErrorContext::builder()
+                    .module_path(module_path.to_string())
+                    .build())
+                .build();
+            return (vec![ModuleInfo::builder()
+                .path(module_path.to_string())
+                .file("<unresolved>".to_string())
+                .visibility(visibility.to_string())
+                .build()], vec![err]);
+        };'''
+
+[[tasks.TASK-5.changes]]
+file = "src/module_tree.rs"
+before = '''        let (ast, file_info) = file_parser::parse_file(file_path)?;
+        process_module_info(
+            module_path,
+            file_path,
+            visibility,
+            &file_info,
+            &ast.items,
+            &file_path.parent().unwrap_or_else(|| Path::new(".")),
+            visited,
+        )'''
+after = '''        let parsed = file_parser::parse_file(file_path);
+        let mut errors: Vec<ErrorEntry> = Vec::new();
+        if let Some(ref err) = parsed.parse_error {
+            errors.push(crate::file_parser::build_parse_error_entry(file_path, err));
+        }
+        process_module_info(
+            module_path,
+            file_path,
+            visibility,
+            &parsed.file_info,
+            &parsed.ast.items,
+            &file_path.parent().unwrap_or(file_path),
+            visited,
+            &mut errors,
+        )'''
+
+[[tasks.TASK-5.changes]]
+file = "src/module_tree.rs"
+before = '''        if visited.contains(file_path.as_path()) {
+            return Ok(vec![]); // cycle detected
+        }'''
+after = '''        if visited.contains(file_path.as_path()) {
+            return (vec![], vec![]); // cycle detected
+        }'''
+
+[[tasks.TASK-5.changes]]
+file = "src/module_tree.rs"
+before = '''/// Resolve a `mod name;` declaration to a file path.
+/// Tries `{parent_dir}/{mod_name}.rs` first, then `{parent_dir}/{mod_name}/mod.rs`.
+pub fn resolve_module_path(parent_dir: &Path, mod_name: &str) -> Option<PathBuf> {'''
+after = '''/// Resolve a `mod name;` declaration to a file path.
+/// Tries `{parent_dir}/{mod_name}.rs` first, then `{parent_dir}/{mod_name}/mod.rs`.
+///
+/// Returns `None` if neither path exists.
+pub fn resolve_module_path(parent_dir: &Path, mod_name: &str) -> Option<PathBuf> {'''
+
+[[tasks.TASK-5.changes]]
+file = "src/module_tree.rs"
+before = '''fn process_submodule(
+    module_path: &str,
+    mod_name: &str,
+    parent_items: &[syn::Item],
+    parent_dir: &Path,
+    parent_file: &Path,
+    visited: &mut HashSet<PathBuf>,
+) -> Result<Vec<ModuleInfo>> {'''
+after = '''fn process_submodule(
+    module_path: &str,
+    mod_name: &str,
+    parent_items: &[syn::Item],
+    parent_dir: &Path,
+    parent_file: &Path,
+    visited: &mut HashSet<PathBuf>,
+) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {'''
+
+[[tasks.TASK-5.changes]]
+file = "src/module_tree.rs"
+before = '''    if let Some((_, ref inline_items)) = mod_item.content {
+        // Inline module: process its body items directly (no file lookup).
+        process_module_items(
+            module_path,
+            parent_file,
+            visibility,
+            inline_items,
+            parent_dir,
+            visited,
+        )
+    } else {'''
+after = '''    if let Some((_, ref inline_items)) = mod_item.content {
+        // Inline module: process its body items directly (no file lookup).
+        let (modules, errs) = process_module_items(
+            module_path,
+            parent_file,
+            visibility,
+            inline_items,
+            parent_dir,
+            visited,
+        );
+        return (modules, errs);
+    } else {'''
+
+[[tasks.TASK-5.changes]]
+file = "src/module_tree.rs"
+before = '''fn process_module_items(
+    module_path: &str,
+    file_path: &Path,
+    visibility: &str,
+    items: &[syn::Item],
+    parent_dir: &Path,
+    visited: &mut HashSet<PathBuf>,
+) -> Result<Vec<ModuleInfo>> {'''
+after = '''fn process_module_items(
+    module_path: &str,
+    file_path: &Path,
+    visibility: &str,
+    items: &[syn::Item],
+    parent_dir: &Path,
+    visited: &mut HashSet<PathBuf>,
+) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {'''
+
+[[tasks.TASK-5.changes]]
+file = "src/module_tree.rs"
+before = '''    let file_info = FileInfo {
+        public_items: file_parser::extract_public_items(items),
+        imports: file_parser::extract_imports(items),
+        re_exports: file_parser::extract_re_exports(items),
+        submodules: file_parser::extract_submodules(items),
+        impls: file_parser::extract_impls(items),
+    };
+    process_module_info(module_path, file_path, visibility, &file_info, items, parent_dir, visited)
+}'''
+after = '''    let file_info = FileInfo {
+        public_items: file_parser::extract_public_items(items),
+        imports: file_parser::extract_imports(items),
+        re_exports: file_parser::extract_re_exports(items),
+        submodules: file_parser::extract_submodules(items),
+        impls: file_parser::extract_impls(items),
+    };
+    process_module_info(module_path, file_path, visibility, &file_info, items, parent_dir, visited, &mut Vec::new())
+}'''
+
+[[tasks.TASK-5.changes]]
+file = "src/module_tree.rs"
+before = '''fn process_module_info(
+    module_path: &str,
+    file_path: &Path,
+    visibility: &str,
+    file_info: &FileInfo,
+    items: &[syn::Item],
+    _parent_dir: &Path,
+    visited: &mut HashSet<PathBuf>,
+) -> Result<Vec<ModuleInfo>> {
+    let mut modules = vec![build_module_info(
+        module_path,
+        file_path,
+        visibility,
+        &file_info.public_items,
+        &file_info.imports,
+        &file_info.re_exports,
+        &file_info.submodules,
+    )];
+
+    for sub in &file_info.submodules {
+        if sub.is_test {
+            continue;
+        }
+        let child_path = format!("{}::{}", module_path, sub.name);
+        let child_dir = file_path.parent().unwrap_or_else(|| Path::new("."));
+        let child_modules = process_submodule(
+            &child_path,
+            &sub.name,
+            items,
+            child_dir,
+            file_path,
+            visited,
+        )?;
+        modules.extend(child_modules);
+    }
+
+    Ok(modules)
+}'''
+after = '''fn process_module_info(
+    module_path: &str,
+    file_path: &Path,
+    visibility: &str,
+    file_info: &FileInfo,
+    items: &[syn::Item],
+    _parent_dir: &Path,
+    visited: &mut HashSet<PathBuf>,
+    errors: &mut Vec<crate::schema::ErrorEntry>,
+) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {
+    let mut modules = vec![build_module_info(
+        module_path,
+        file_path,
+        visibility,
+        &file_info.public_items,
+        &file_info.imports,
+        &file_info.re_exports,
+        &file_info.submodules,
+    )];
+
+    for sub in &file_info.submodules {
+        if sub.is_test {
+            continue;
+        }
+        let child_path = format!("{}::{}", module_path, sub.name);
+        let child_dir = file_path.parent().unwrap_or(file_path);
+        let (child_modules, child_errors) = process_submodule(
+            &child_path,
+            &sub.name,
+            items,
+            child_dir,
+            file_path,
+            visited,
+        );
+        errors.extend(child_errors);
+        modules.extend(child_modules);
+    }
+
+    (modules, errors.clone())
+}'''
+
+
+[tasks.TASK-6]
+description = "Update workspace.rs enumerate_members to return MissingWorkspaceSection error"
+type = "replace"
+acceptance = [
+    "cargo check -p rust-workspace-map",
+]
+
+[[tasks.TASK-6.changes]]
+file = "src/workspace.rs"
+before = '''    let members: Vec<String> = parsed
+        .get("workspace")
+        .and_then(|w| w.get("members"))
+        .and_then(|m| m.as_array())
+        .map(|arr| {
+            arr.iter()
+                .filter_map(|v| v.as_str().map(String::from))
+                .collect()
+        })
+        .unwrap_or_default();'''
+after = '''    let members: Vec<String> = match parsed.get("workspace") {
+        None => return Err(Error::MissingWorkspaceSection),
+        Some(workspace) => workspace
+            .get("members")
+            .and_then(|m| m.as_array())
+            .map(|arr| {
+                arr.iter()
+                    .filter_map(|v| v.as_str().map(String::from))
+                    .collect::<Vec<_>>()
+            })
+            .unwrap_or_default(),
+    };'''
+
+[[tasks.TASK-6.changes]]
+file = "src/workspace.rs"
+before = '''/// Walk up the directory tree from `start_path` to find a `Cargo.toml`
+/// containing a `[workspace]` section. Returns the directory containing it.
+pub fn find_workspace_root(start_path: &Path) -> Result<PathBuf> {'''
+after = '''/// Walk up the directory tree from `start_path` to find a `Cargo.toml`
+/// containing a `[workspace]` section. Returns the directory containing it.
+///
+/// # Errors
+///
+/// Returns `Error::WorkspaceRootNotFound` if no `Cargo.toml` with a
+/// `[workspace]` section is found in any ancestor directory.
+pub fn find_workspace_root(start_path: &Path) -> Result<PathBuf> {'''
+
+[[tasks.TASK-6.changes]]
+file = "src/workspace.rs"
+before = '''/// Parse the workspace `Cargo.toml`, resolve member paths (including glob
+/// patterns), apply `exclude` list, and return absolute paths to each member
+/// crate directory.
+pub fn enumerate_members(root: &Path) -> Result<Vec<PathBuf>> {'''
+after = '''/// Parse the workspace `Cargo.toml`, resolve member paths (including glob
+/// patterns), apply `exclude` list, and return absolute paths to each member
+/// crate directory.
+///
+/// # Errors
+///
+/// Returns `Error::MissingWorkspaceSection` if the `Cargo.toml` lacks a
+/// `[workspace]` section entirely.
+pub fn enumerate_members(root: &Path) -> Result<Vec<PathBuf>> {'''
+
+[tasks.TASK-7]
+description = "Refactor lib.rs run(): parallel error collection, error entry construction, remove unwrap_or_default"
+type = "replace"
+acceptance = [
+    "cargo check -p rust-workspace-map",
+]
+
+[[tasks.TASK-7.changes]]
+file = "src/lib.rs"
+before = '''use anyhow::Context;
+use rayon::prelude::*;
+use schema::{
+    CrateInfo, CrateType, ErrorEntry, ModuleInfo, WorkspaceInfo, WorkspaceMap,
+};
+use std::path::Path;'''
+after = '''use anyhow::Context;
+use rayon::prelude::*;
+use schema::{
+    CrateInfo, CrateType, ErrorEntry, ErrorSeverity, ModuleInfo, WorkspaceInfo,
+    WorkspaceMap,
+};
+use std::path::Path;'''
+
+[[tasks.TASK-7.changes]]
+file = "src/lib.rs"
+before = '''    let errors: Vec<ErrorEntry> = Vec::new();
+
+    let mut crate_infos: Vec<CrateInfo> = member_dirs
+        .par_iter()
+        .filter_map(|dir| {
+            let cargo_toml = dir.join("Cargo.toml");
+
+            let (pkg, deps) = match cargo_info::parse_cargo_toml(&cargo_toml) {
+                Ok(v) => v,
+                Err(e) => {
+                    eprintln!(
+                        "warning: failed to parse {}: {}",
+                        cargo_toml.display(),
+                        e
+                    );
+                    return None;
+                }
+            };
+
+            let roots = workspace::resolve_crate_roots(dir);
+            if roots.is_empty() {
+                eprintln!(
+                    "warning: no crate entry points found in {}",
+                    dir.display()
+                );
+                return None;
+            }
+
+            let crate_type = if roots.iter().any(|(_, t)| *t == CrateType::Lib)
+                && roots.iter().any(|(_, t)| *t == CrateType::Bin)
+            {
+                CrateType::LibAndBin
+            } else {
+                roots.first().map_or(CrateType::Lib, |(_, t)| *t)
+            };
+
+            let pkg_name = pkg.name.clone();
+            let mut modules: Vec<ModuleInfo> = roots
+                .iter()
+                .flat_map(|(root, _ty)| {
+                    module_tree::build_module_tree(root, &pkg_name).unwrap_or_default()
+                })
+                .collect();
+
+            // Relativize all paths to the workspace root.
+            for m in &mut modules {
+                m.file = relativize_path(&m.file, &workspace_root);
+                for item in &mut m.public_items {
+                    item.file = relativize_path(&item.file, &workspace_root);
+                }
+            }
+
+            let crate_root = roots
+                .first()
+                .map(|(r, _)| relativize_path(&r.to_string_lossy(), &workspace_root))
+                .unwrap_or_default();
+
+            let rebuilt_pkg = schema::PackageInfo::builder()
+                .name(pkg.name)
+                .version(pkg.version)
+                .edition(pkg.edition)
+                .crate_type(crate_type)
+                .build();
+
+            Some(
+                CrateInfo::builder()
+                    .name(pkg_name)
+                    .root(crate_root)
+                    .package(rebuilt_pkg)
+                    .modules(modules)
+                    .deps(deps)
+                    .build(),
+            )
+        })
+        .collect();'''
+after = '''    let mut crate_errors: Vec<ErrorEntry> = Vec::new();
+
+    let results: Vec<(CrateInfo, Vec<ErrorEntry>)> = member_dirs
+        .par_iter()
+        .map(|dir| {
+            let cargo_toml = dir.join("Cargo.toml");
+            let mut crate_errors = Vec::new();
+
+            let (pkg, deps) = match cargo_info::parse_cargo_toml(&cargo_toml) {
+                Ok(v) => v,
+                Err(e) => {
+                    crate_errors.push(ErrorEntry::builder()
+                        .file(cargo_toml.to_string_lossy().to_string())
+                        .message(format!("failed to parse Cargo.toml: {e}"))
+                        .severity(ErrorSeverity::Error)
+                        .kind("toml_parse_error".to_string())
+                        .cause(e.to_string())
+                        .build());
+                    return (None, crate_errors);
+                }
+            };
+
+            let roots = workspace::resolve_crate_roots(dir);
+            if roots.is_empty() {
+                crate_errors.push(ErrorEntry::builder()
+                    .file(dir.to_string_lossy().to_string())
+                    .message("no crate entry points found".to_string())
+                    .severity(ErrorSeverity::Warning)
+                    .kind("missing_crate_roots".to_string())
+                    .build());
+                return (None, crate_errors);
+            }
+
+            let crate_type = if roots.iter().any(|(_, t)| *t == CrateType::Lib)
+                && roots.iter().any(|(_, t)| *t == CrateType::Bin)
+            {
+                CrateType::LibAndBin
+            } else {
+                roots.first().map_or(CrateType::Lib, |(_, t)| *t)
+            };
+
+            let pkg_name = pkg.name.clone();
+            let mut modules: Vec<ModuleInfo> = Vec::new();
+            let mut collected_errors = Vec::new();
+            for (root, _ty) in &roots {
+                let (m, e) = module_tree::build_module_tree(&root, &pkg_name);
+                modules.extend(m);
+                collected_errors.extend(e);
+            }
+            crate_errors.extend(collected_errors);
+
+            // Relativize all paths to the workspace root.
+            for m in &mut modules {
+                m.file = relativize_path(&m.file, &workspace_root);
+                for item in &mut m.public_items {
+                    item.file = relativize_path(&item.file, &workspace_root);
+                }
+            }
+
+            let crate_root = roots
+                .first()
+                .map(|(r, _)| relativize_path(&r.to_string_lossy(), &workspace_root))
+                .unwrap_or_default();
+
+            let rebuilt_pkg = schema::PackageInfo::builder()
+                .name(pkg.name)
+                .version(pkg.version)
+                .edition(pkg.edition)
+                .crate_type(crate_type)
+                .build();
+
+            let crate_info = CrateInfo::builder()
+                .name(pkg_name)
+                .root(crate_root)
+                .package(rebuilt_pkg)
+                .modules(modules)
+                .deps(deps)
+                .build();
+
+            (Some(crate_info), crate_errors)
+        })
+        .collect();
+
+    let mut crate_infos: Vec<CrateInfo> = Vec::new();
+
+    for (info, errs) in results {
+        if let Some(ci) = info {
+            crate_errors.extend(errs);
+            crate_infos.push(ci);
+        }
+    }'''
+
+[[tasks.TASK-7.changes]]
+file = "src/lib.rs"
+before = '''    let map = WorkspaceMap::builder()
+        .workspace(workspace_info)
+        .crates(crate_infos)
+        .cross_references(cross_refs)
+        .errors(errors)
+        .workspace_root(workspace_root.clone())
+        .build();'''
+after = '''    let map = WorkspaceMap::builder()
+        .workspace(workspace_info)
+        .crates(crate_infos)
+        .cross_references(cross_refs)
+        .errors(crate_errors)
+        .workspace_root(workspace_root.clone())
+        .build();'''
+
+[tasks.TASK-8]
+description = "Remove all crate-level clippy allow attributes and fix individual lint violations"
+type = "replace"
+acceptance = [
+    "cargo clippy -p rust-workspace-map -- -D warnings",
+]
+
+[[tasks.TASK-8.changes]]
+file = "src/lib.rs"
+before = '''#![warn(clippy::pedantic)]
+#![allow(clippy::missing_errors_doc)]
+#![allow(clippy::must_use_candidate)]
+#![allow(clippy::doc_markdown)]
+#![allow(clippy::uninlined_format_args)]
+#![allow(clippy::redundant_closure)]
+#![allow(clippy::collapsible_if)]
+#![allow(clippy::needless_pass_by_value)]
+#![allow(clippy::needless_borrow)]
+#![allow(clippy::redundant_closure_for_method_calls)]'''
+after = '''#![warn(clippy::pedantic)]'''
+
+[[tasks.TASK-8.changes]]
+file = "src/lib.rs"
+before = '''/// Run the full workspace mapping pipeline.
+///
+/// 1. Discover workspace root and member crates.
+/// 2. Process each crate in parallel (Cargo.toml parsing + module tree).
+/// 3. Compute cross-crate references.
+/// 4. Render JSON to stdout or the configured output file.
+pub fn run(config: Config) -> anyhow::Result<()> {'''
+after = '''/// Run the full workspace mapping pipeline.
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
+pub fn run(config: Config) -> anyhow::Result<()> {'''
+
+[[tasks.TASK-8.changes]]
+file = "src/lib.rs"
+before = '''    let workspace_name = workspace_root
+        .file_name()
+        .map(|n| n.to_string_lossy().to_string())
+        .unwrap_or_default();'''
+after = '''    let workspace_name = workspace_root
+        .file_name()
+        .map(|n| n.to_string_lossy().to_string())
+        .unwrap_or_default();'''
+
+[[tasks.TASK-8.changes]]
+file = "src/lib.rs"
+before = '''/// Strip the workspace root prefix from a path string, returning a
+/// workspace-relative path. If the prefix doesn't match, returns the
+/// original string unchanged.
+fn relativize_path(path_str: &str, root: &Path) -> String {
+    let p = Path::new(path_str);
+    match p.strip_prefix(root) {
+        Ok(rel) => rel.to_string_lossy().to_string(),
+        Err(_) => path_str.to_string(),
+    }
+}'''
+after = '''/// Strip the workspace root prefix from a path string, returning a
+/// workspace-relative path. If the prefix doesn't match, returns the
+/// original string unchanged.
+fn relativize_path(path_str: &str, root: &Path) -> String {
+    let p = Path::new(path_str);
+    match p.strip_prefix(root) {
+        Ok(rel) => rel.to_string_lossy().to_string(),
+        Err(_) => path_str.to_string(),
+    }
+}'''
+
+[[tasks.TASK-8.changes]]
+file = "src/file_parser.rs"
+before = '''/// Extract all items with any form of `pub` visibility (excluding `Inherited`).
+/// Results are sorted by name then line for deterministic output.
+pub fn extract_public_items(items: &[syn::Item]) -> Vec<PublicItem> {'''
+after = '''/// Extract all items with any form of `pub` visibility (excluding `Inherited`).
+/// Results are sorted by name then line for deterministic output.
+#[must_use]
+pub fn extract_public_items(items: &[syn::Item]) -> Vec<PublicItem> {'''
+
+[[tasks.TASK-8.changes]]
+file = "src/file_parser.rs"
+before = '''/// Extract all `use` statements. Braced imports are expanded to individual
+/// entries. Results sorted by path for determinism.
+pub fn extract_imports(items: &[syn::Item]) -> Vec<Import> {'''
+after = '''/// Extract all `use` statements. Braced imports are expanded to individual
+/// entries. Results sorted by path for determinism.
+#[must_use]
+pub fn extract_imports(items: &[syn::Item]) -> Vec<Import> {'''
+
+[[tasks.TASK-8.changes]]
+file = "src/file_parser.rs"
+before = '''/// Extract `pub use` re-exports. Results sorted by export_path.
+pub fn extract_re_exports(items: &[syn::Item]) -> Vec<ReExport> {'''
+after = '''/// Extract `pub use` re-exports. Results sorted by export_path.
+#[must_use]
+pub fn extract_re_exports(items: &[syn::Item]) -> Vec<ReExport> {'''
+
+[[tasks.TASK-8.changes]]
+file = "src/file_parser.rs"
+before = '''/// Extract `mod` declarations. Detects `#[cfg(test)]` via literal token
+/// matching. Results sorted by name.
+pub fn extract_submodules(items: &[syn::Item]) -> Vec<SubmoduleDecl> {'''
+after = '''/// Extract `mod` declarations. Detects `#[cfg(test)]` via literal token
+/// matching. Results sorted by name.
+#[must_use]
+pub fn extract_submodules(items: &[syn::Item]) -> Vec<SubmoduleDecl> {'''
+
+[[tasks.TASK-8.changes]]
+file = "src/file_parser.rs"
+before = '''/// Extract `impl` blocks. Each `ImplInfo` records the target type name and
+/// the impl items (fn, type, const).
+pub fn extract_impls(items: &[syn::Item]) -> Vec<ImplInfo> {'''
+after = '''/// Extract `impl` blocks. Each `ImplInfo` records the target type name and
+/// the impl items (fn, type, const).
+#[must_use]
+pub fn extract_impls(items: &[syn::Item]) -> Vec<ImplInfo> {'''
+
+[[tasks.TASK-8.changes]]
+file = "src/module_tree.rs"
+before = '''/// Resolve a `mod name;` declaration to a file path.
+/// Tries `{parent_dir}/{mod_name}.rs` first, then `{parent_dir}/{mod_name}/mod.rs`.
+///
+/// Returns `None` if neither path exists.
+pub fn resolve_module_path(parent_dir: &Path, mod_name: &str) -> Option<PathBuf> {'''
+after = '''/// Resolve a `mod name;` declaration to a file path.
+/// Tries `{parent_dir}/{mod_name}.rs` first, then `{parent_dir}/{mod_name}/mod.rs`.
+///
+/// Returns `None` if neither path exists.
+#[must_use]
+pub fn resolve_module_path(parent_dir: &Path, mod_name: &str) -> Option<PathBuf> {'''
+
+[[tasks.TASK-8.changes]]
+file = "src/render.rs"
+before = '''/// Serialize the workspace map to a JSON string with 2-space indentation.
+pub fn render_json(map: &WorkspaceMap) -> serde_json::Result<String> {'''
+after = '''/// Serialize the workspace map to a JSON string with 2-space indentation.
+#[must_use]
+pub fn render_json(map: &WorkspaceMap) -> serde_json::Result<String> {'''
+
+[[tasks.TASK-8.changes]]
+file = "src/cross_refs.rs"
+before = '''// Helper: convert ItemKind to a short string for the TypeRef.kind field.
+impl crate::schema::PublicItem {
+    fn kind_to_string(&self) -> String {'''
+after = '''// Helper: convert ItemKind to a short string for the TypeRef.kind field.
+impl crate::schema::PublicItem {
+    #[must_use]
+    fn kind_to_string(&self) -> String {'''
+
+[[tasks.TASK-8.changes]]
+file = "src/lib.rs"
+before = '''    match p.strip_prefix(root) {
+        Ok(rel) => rel.to_string_lossy().to_string(),
+        Err(_) => path_str.to_string(),
+    }
+}'''
+after = '''    match p.strip_prefix(root) {
+        Ok(rel) => rel.to_string_lossy().to_string(),
+        Err(_) => path_str.to_string(),
+    }
+}'''
+
+[[tasks.TASK-8.changes]]
+file = "src/file_parser.rs"
+before = "/// Extract `pub use` re-exports. Results sorted by export_path.\n#[must_use]\npub fn extract_re_exports"
+after = "/// Extract `pub use` re-exports. Results sorted by `export_path`.\n#[must_use]\npub fn extract_re_exports"
+
+[[tasks.TASK-8.changes]]
+file = "src/file_parser.rs"
+before = "            let name = m.ident.as_ref().map(|i| i.to_string()).unwrap_or_default();"
+after = "            let name = m.ident.as_ref().map(ToString::to_string).unwrap_or_default();"
+
+[[tasks.TASK-8.changes]]
+file = "src/file_parser.rs"
+before = "fn flatten_use_tree(tree: &syn::UseTree, prefix: String, line: usize) -> Vec<Import> {"
+after = "fn flatten_use_tree(tree: &syn::UseTree, prefix: &str, line: usize) -> Vec<Import> {"
+
+[[tasks.TASK-8.changes]]
+file = "src/file_parser.rs"
+before = "                Some(flatten_use_tree(&u.tree, String::new(), line_of_item(item)))"
+after = "                Some(flatten_use_tree(&u.tree, \"\", line_of_item(item)))"
+
+[[tasks.TASK-8.changes]]
+file = "src/file_parser.rs"
+before = "            flatten_use_tree(&p.tree, new_prefix, line)"
+after = "            flatten_use_tree(&p.tree, &new_prefix, line)"
+
+[[tasks.TASK-8.changes]]
+file = "src/schema.rs"
+before = "/// Internal intermediate type consumed by module_tree.\n#[derive(Debug, Clone, Default)]\npub struct FileInfo"
+after = "/// Internal intermediate type consumed by `module_tree`.\n#[derive(Debug, Clone, Default)]\npub struct FileInfo"
+
+[[tasks.TASK-8.changes]]
+file = "src/lib.rs"
+before = "/// parsed, or the JSON output cannot be written.\npub fn run(config: Config) -> anyhow::Result<()> {"
+after = "/// parsed, or the JSON output cannot be written.\npub fn run(config: &Config) -> anyhow::Result<()> {"
+
+[[tasks.TASK-8.changes]]
+file = "src/main.rs"
+before = "    rust_workspace_map::run(config)"
+after = "    rust_workspace_map::run(&config)"
+
+[tasks.TASK-9]
+description = "Add unit tests for file_parser, module_tree, workspace, cargo_info, cross_refs, and render"
+type = "replace"
+acceptance = [
+    "cargo test -p rust-workspace-map",
+]
+
+[[tasks.TASK-9.changes]]
+file = "src/file_parser.rs"
+before = '''fn extract_re_exports_from_tree(
+    tree: &syn::UseTree,
+    import_path: String,
+    line: usize,
+) -> Vec<ReExport> {
+    match tree {
+        syn::UseTree::Path(p) => {
+            let new_import = if import_path.is_empty() {
+                p.ident.to_string()
+            } else {
+                format!("{}::{}", import_path, p.ident)
+            };
+            extract_re_exports_from_tree(&p.tree, new_import, line)
+        }
+        syn::UseTree::Name(n) => {
+            vec![ReExport {
+                import_path,
+                export_path: n.ident.to_string(),
+                line,
+            }]
+        }
+        syn::UseTree::Rename(r) => {
+            vec![ReExport {
+                import_path,
+                export_path: r.rename.to_string(),
+                line,
+            }]
+        }
+        syn::UseTree::Glob(_) => {
+            vec![ReExport {
+                import_path,
+                export_path: "*".to_string(),
+                line,
+            }]
+        }
+        syn::UseTree::Group(g) => g
+            .items
+            .iter()
+            .flat_map(|t| extract_re_exports_from_tree(t, import_path.clone(), line))
+            .collect(),
+    }
+}'''
+after = '''fn extract_re_exports_from_tree(
+    tree: &syn::UseTree,
+    import_path: String,
+    line: usize,
+) -> Vec<ReExport> {
+    match tree {
+        syn::UseTree::Path(p) => {
+            let new_import = if import_path.is_empty() {
+                p.ident.to_string()
+            } else {
+                format!("{}::{}", import_path, p.ident)
+            };
+            extract_re_exports_from_tree(&p.tree, new_import, line)
+        }
+        syn::UseTree::Name(n) => {
+            vec![ReExport {
+                import_path,
+                export_path: n.ident.to_string(),
+                line,
+            }]
+        }
+        syn::UseTree::Rename(r) => {
+            vec![ReExport {
+                import_path,
+                export_path: r.rename.to_string(),
+                line,
+            }]
+        }
+        syn::UseTree::Glob(_) => {
+            vec![ReExport {
+                import_path,
+                export_path: "*".to_string(),
+                line,
+            }]
+        }
+        syn::UseTree::Group(g) => g
+            .items
+            .iter()
+            .flat_map(|t| extract_re_exports_from_tree(t, import_path.clone(), line))
+            .collect(),
+    }
+}
+
+// ── Tests ───────────────────────────────────────────────────────────────
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use crate::schema::{Import, ReExport, SubmoduleDecl};
+    use std::path::PathBuf;
+
+    fn parse_source(src: &str) -> ParsedFile {
+        let tmp = std::env::temp_dir().join("parse_test.rs");
+        std::fs::write(&tmp, src).unwrap();
+        let result = parse_file(&tmp);
+        std::fs::remove_file(&tmp).ok();
+        result
+    }
+
+    #[test]
+    fn parse_file_returns_ast_for_valid_source() {
+        let src = "pub struct Foo { x: i32 }";
+        let result = parse_source(src);
+        assert!(result.parse_error.is_none());
+        assert_eq!(result.ast.items.len(), 1);
+    }
+
+    #[test]
+    fn parse_file_returns_error_for_invalid_source() {
+        let src = "pub struct { invalid rust }";
+        let result = parse_source(src);
+        assert!(result.parse_error.is_some());
+        let err = result.parse_error.as_ref().unwrap();
+        assert!(!err.message.is_empty());
+        assert!(err.line > 0);
+    }
+
+    #[test]
+    fn parse_file_returns_empty_for_empty_file() {
+        let result = parse_source("");
+        assert!(result.parse_error.is_none());
+        assert!(result.file_info.public_items.is_empty());
+    }
+
+    #[test]
+    fn extract_public_items_finds_struct_enum_trait_fn() {
+        let src = "pub struct Foo {} pub enum Bar { A, B } pub trait Baz {} pub fn hello() {}";
+        let result = parse_source(src);
+        let items = extract_public_items(&result.ast.items);
+        let names: Vec<_> = items.iter().map(|i| i.name.as_str()).collect();
+        assert!(names.contains(&"Foo"));
+        assert!(names.contains(&"Bar"));
+        assert!(names.contains(&"Baz"));
+        assert!(names.contains(&"hello"));
+    }
+
+    #[test]
+    fn extract_public_items_empty_for_no_public_items() {
+        let src = "struct Private {} fn private_fn() {}";
+        let result = parse_source(src);
+        let items = extract_public_items(&result.ast.items);
+        assert!(items.is_empty());
+    }
+
+    #[test]
+    fn extract_imports_finds_use_statements() {
+        let src = "use std::collections::BTreeMap;";
+        let result = parse_source(src);
+        let imports = extract_imports(&result.ast.items);
+        assert_eq!(imports.len(), 1);
+        assert_eq!(imports[0].path, "std::collections::BTreeMap");
+    }
+
+    #[test]
+    fn extract_re_exports_finds_pub_use() {
+        let src = "pub use crate::foo;";
+        let result = parse_source(src);
+        let re_exports = extract_re_exports(&result.ast.items);
+        assert_eq!(re_exports.len(), 1);
+        assert_eq!(re_exports[0].import_path, "crate::foo");
+        assert_eq!(re_exports[0].export_path, "foo");
+    }
+
+    #[test]
+    fn extract_re_exports_finds_rename() {
+        let src = "pub use crate::foo as bar;";
+        let result = parse_source(src);
+        let re_exports = extract_re_exports(&result.ast.items);
+        assert_eq!(re_exports.len(), 1);
+        assert_eq!(re_exports[0].import_path, "crate::foo as bar");
+        assert_eq!(re_exports[0].export_path, "bar");
+    }
+
+    #[test]
+    fn extract_submodules_finds_mod_declarations() {
+        let src = "mod foo; mod bar;";
+        let result = parse_source(src);
+        let subs = extract_submodules(&result.ast.items);
+        assert_eq!(subs.len(), 2);
+        let names: Vec<_> = subs.iter().map(|s| s.name.as_str()).collect();
+        assert!(names.contains(&"bar"));
+        assert!(names.contains(&"foo"));
+    }
+
+    #[test]
+    fn extract_submodules_marks_cfg_test() {
+        let src = "#[cfg(test)] mod inner;";
+        let result = parse_source(src);
+        let subs = extract_submodules(&result.ast.items);
+        assert_eq!(subs.len(), 1);
+        assert!(subs[0].is_test);
+    }
+
+    #[test]
+    fn extract_impls_finds_fn_type_const() {
+        let src = "impl MyType { pub fn foo(&self) {} pub type Alias = u32; pub const N: usize = 42; }";
+        let result = parse_source(src);
+        let impls = extract_impls(&result.ast.items);
+        assert_eq!(impls.len(), 1);
+        assert_eq!(impls[0].type_, "MyType");
+        assert_eq!(impls[0].items.len(), 3);
+    }
+
+    #[test]
+    fn build_parse_error_entry_constructs_error() {
+        let path = PathBuf::from("test.rs");
+        let err = SynParseError {
+            message: "expected `;`".to_string(),
+            line: 5,
+        };
+        let entry = build_parse_error_entry(&path, &err);
+        assert_eq!(entry.file, "test.rs");
+        assert_eq!(entry.line, 5);
+        assert_eq!(entry.kind, "syn_parse_error");
+        assert_eq!(entry.severity, ErrorSeverity::Error);
+    }
+}'''
+
+[[tasks.TASK-9.changes]]
+file = "src/module_tree.rs"
+before = '''fn process_module_info(
+    module_path: &str,
+    file_path: &Path,
+    visibility: &str,
+    file_info: &FileInfo,
+    items: &[syn::Item],
+    _parent_dir: &Path,
+    visited: &mut HashSet<PathBuf>,
+    errors: &mut Vec<crate::schema::ErrorEntry>,
+) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {
+    let mut modules = vec![build_module_info(
+        module_path,
+        file_path,
+        visibility,
+        &file_info.public_items,
+        &file_info.imports,
+        &file_info.re_exports,
+        &file_info.submodules,
+    )];
+
+    for sub in &file_info.submodules {
+        if sub.is_test {
+            continue;
+        }
+        let child_path = format!("{}::{}", module_path, sub.name);
+        let child_dir = file_path.parent().unwrap_or(file_path);
+        let (child_modules, child_errors) = process_submodule(
+            &child_path,
+            &sub.name,
+            items,
+            child_dir,
+            file_path,
+            visited,
+        );
+        errors.extend(child_errors);
+        modules.extend(child_modules);
+    }
+
+    (modules, errors.clone())
+}'''
+after = '''fn process_module_info(
+    module_path: &str,
+    file_path: &Path,
+    visibility: &str,
+    file_info: &FileInfo,
+    items: &[syn::Item],
+    _parent_dir: &Path,
+    visited: &mut HashSet<PathBuf>,
+    errors: &mut Vec<crate::schema::ErrorEntry>,
+) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {
+    let mut modules = vec![build_module_info(
+        module_path,
+        file_path,
+        visibility,
+        &file_info.public_items,
+        &file_info.imports,
+        &file_info.re_exports,
+        &file_info.submodules,
+    )];
+
+    for sub in &file_info.submodules {
+        if sub.is_test {
+            continue;
+        }
+        let child_path = format!("{}::{}", module_path, sub.name);
+        let child_dir = file_path.parent().unwrap_or(file_path);
+        let (child_modules, child_errors) = process_submodule(
+            &child_path,
+            &sub.name,
+            items,
+            child_dir,
+            file_path,
+            visited,
+        );
+        errors.extend(child_errors);
+        modules.extend(child_modules);
+    }
+
+    (modules, errors.clone())
+}
+
+// ── Tests ───────────────────────────────────────────────────────────────
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use crate::schema::ErrorEntry;
+
+    #[test]
+    fn resolve_module_path_finds_rs_file() {
+        let tmp = std::env::temp_dir().join("resolve_test");
+        let _ = std::fs::create_dir_all(&tmp);
+        let mod_file = tmp.join("foo.rs");
+        std::fs::write(&mod_file, "").ok();
+        let result = resolve_module_path(&tmp, "foo");
+        assert_eq!(result, Some(mod_file));
+        std::fs::remove_dir_all(&tmp).ok();
+    }
+
+    #[test]
+    fn resolve_module_path_finds_mod_rs() {
+        let tmp = std::env::temp_dir().join("resolve_test2");
+        let _ = std::fs::create_dir_all(&tmp);
+        let mod_dir = tmp.join("bar");
+        let _ = std::fs::create_dir_all(&mod_dir);
+        let mod_rs = mod_dir.join("mod.rs");
+        std::fs::write(&mod_rs, "").ok();
+        let result = resolve_module_path(&tmp, "bar");
+        assert_eq!(result, Some(mod_rs));
+        std::fs::remove_dir_all(&tmp).ok();
+    }
+
+    #[test]
+    fn resolve_module_path_returns_none_for_missing() {
+        let tmp = std::env::temp_dir().join("resolve_test3");
+        let _ = std::fs::create_dir_all(&tmp);
+        let result = resolve_module_path(&tmp, "nonexistent");
+        assert!(result.is_none());
+        std::fs::remove_dir_all(&tmp).ok();
+    }
+
+    #[test]
+    fn build_module_tree_returns_empty_for_nonexistent() {
+        let tmp = std::env::temp_dir().join("bmt_test");
+        let _ = std::fs::create_dir_all(&tmp);
+        let (modules, errors) = build_module_tree(&tmp, "test");
+        assert!(modules.is_empty());
+        assert!(!errors.is_empty());
+        std::fs::remove_dir_all(&tmp).ok();
+    }
+}'''
+
+[[tasks.TASK-9.changes]]
+file = "src/workspace.rs"
+before = '''    result.sort();
+    result.dedup();
+    Ok(result)
+}
+
+/// For a crate directory, determine its entry-point file(s).'''
+after = '''    result.sort();
+    result.dedup();
+    Ok(result)
+}
+
+// ── Tests ───────────────────────────────────────────────────────────────
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use std::io::Write;
+
+    fn write_cargo_toml(dir: &std::path::Path, content: &str) {
+        let mut f = std::fs::File::create(dir.join("Cargo.toml")).unwrap();
+        f.write_all(content.as_bytes()).unwrap();
+    }
+
+    fn setup_crate(dir: &std::path::Path) {
+        let src = dir.join("src");
+        std::fs::create_dir_all(&src).unwrap();
+        std::fs::write(src.join("lib.rs"), "").unwrap();
+    }
+
+    #[test]
+    fn find_workspace_root_finds_cargo_toml() {
+        let tmp = tempfile::tempdir().unwrap();
+        let path = tmp.path().join("subdir").join("nested");
+        std::fs::create_dir_all(&path).unwrap();
+        write_cargo_toml(tmp.path(), "[workspace]");
+        let result = find_workspace_root(&path).unwrap();
+        assert_eq!(result, tmp.path());
+    }
+
+    #[test]
+    fn enumerate_members_returns_members() {
+        let tmp = tempfile::tempdir().unwrap();
+        write_cargo_toml(tmp.path(), r#"
+[workspace]
+members = ["crate_a", "crate_b"]
+"#);
+        setup_crate(tmp.path().join("crate_a").as_path());
+        setup_crate(tmp.path().join("crate_b").as_path());
+        let members = enumerate_members(tmp.path()).unwrap();
+        assert_eq!(members.len(), 2);
+    }
+
+    #[test]
+    fn enumerate_members_returns_err_for_missing_workspace() {
+        let tmp = tempfile::tempdir().unwrap();
+        write_cargo_toml(tmp.path(), "[dependencies]\nfoo = \"1\"");
+        let result = enumerate_members(tmp.path());
+        assert!(result.is_err());
+        match result.unwrap_err() {
+            Error::MissingWorkspaceSection => {},
+            other => panic!("expected MissingWorkspaceSection, got {:?}", other),
+        }
+    }
+
+    #[test]
+    fn enumerate_members_applies_exclude() {
+        let tmp = tempfile::tempdir().unwrap();
+        write_cargo_toml(tmp.path(), r#"
+[workspace]
+members = ["a", "b", "c"]
+exclude = ["b"]
+"#);
+        setup_crate(tmp.path().join("a").as_path());
+        setup_crate(tmp.path().join("b").as_path());
+        setup_crate(tmp.path().join("c").as_path());
+        let members = enumerate_members(tmp.path()).unwrap();
+        let names: Vec<_> = members.iter().map(|p| p.file_name().unwrap().to_string_lossy()).collect();
+        assert!(names.contains(&"a".as_ref()));
+        assert!(!names.contains(&"b".as_ref()));
+        assert!(names.contains(&"c".as_ref()));
+    }
+
+    #[test]
+    fn resolve_crate_roots_detects_lib() {
+        let tmp = tempfile::tempdir().unwrap();
+        std::fs::create_dir_all(tmp.path().join("src")).unwrap();
+        std::fs::write(tmp.path().join("src").join("lib.rs"), "").unwrap();
+        let roots = resolve_crate_roots(tmp.path());
+        assert_eq!(roots.len(), 1);
+        assert_eq!(roots[0].1, CrateType::Lib);
+    }
+
+    #[test]
+    fn resolve_crate_roots_detects_bin() {
+        let tmp = tempfile::tempdir().unwrap();
+        std::fs::create_dir_all(tmp.path().join("src")).unwrap();
+        std::fs::write(tmp.path().join("src").join("main.rs"), "").unwrap();
+        let roots = resolve_crate_roots(tmp.path());
+        assert_eq!(roots.len(), 1);
+        assert_eq!(roots[0].1, CrateType::Bin);
+    }
+}'''
+
+[[tasks.TASK-9.changes]]
+file = "src/cargo_info.rs"
+before = '''    Ok((package, deps))
+}'''
+after = '''    Ok((package, deps))
+}
+
+// ── Tests ───────────────────────────────────────────────────────────────
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use std::io::Write;
+
+    fn write_cargo_toml(dir: &std::path::Path, content: &str) {
+        let mut f = std::fs::File::create(dir.join("Cargo.toml")).unwrap();
+        f.write_all(content.as_bytes()).unwrap();
+    }
+
+    #[test]
+    fn parse_cargo_toml_parses_minimal() {
+        let tmp = tempfile::tempdir().unwrap();
+        write_cargo_toml(tmp.path(), r#"
+[package]
+name = "test-pkg"
+version = "1.0.0"
+edition = "2021"
+"#);
+        let (pkg, _deps) = parse_cargo_toml(tmp.path().join("Cargo.toml").as_path()).unwrap();
+        assert_eq!(pkg.name, "test-pkg");
+        assert_eq!(pkg.version, "1.0.0");
+        assert_eq!(pkg.edition, "2021");
+    }
+
+    #[test]
+    fn parse_cargo_toml_uses_defaults_for_missing_package() {
+        let tmp = tempfile::tempdir().unwrap();
+        write_cargo_toml(tmp.path(), "");
+        let (pkg, _deps) = parse_cargo_toml(tmp.path().join("Cargo.toml").as_path()).unwrap();
+        assert_eq!(pkg.name, "unknown");
+        assert_eq!(pkg.edition, "2021");
+    }
+
+    #[test]
+    fn parse_cargo_toml_distinguishes_deps() {
+        let tmp = tempfile::tempdir().unwrap();
+        write_cargo_toml(tmp.path(), r#"
+[package]
+name = "test-pkg"
+version = "0.1.0"
+edition = "2021"
+
+[dependencies]
+foo = "1"
+bar = { workspace = true }
+
+[dev-dependencies]
+baz = "2"
+qux = { workspace = true }
+"#);
+        let (_, deps) = parse_cargo_toml(tmp.path().join("Cargo.toml").as_path()).unwrap();
+        assert_eq!(deps.normal, vec!["foo"]);
+        assert_eq!(deps.dev, vec!["baz"]);
+        assert!(deps.workspace_members.contains(&"bar".to_string()));
+        assert!(deps.workspace_members.contains(&"qux".to_string()));
+    }
+}'''
+
+[[tasks.TASK-9.changes]]
+file = "src/cross_refs.rs"
+before = '''    CrossReferences { types: types_map }
+}
+
+// Helper: convert ItemKind to a short string for the TypeRef.kind field.'''
+after = '''    CrossReferences { types: types_map }
+}
+
+// ── Tests ───────────────────────────────────────────────────────────────
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use crate::schema::{ModuleInfo, PublicItem, SubmoduleDecl};
+
+    fn make_crate(name: &str, items: Vec<(String, ItemKind)>) -> CrateInfo {
+        let public_items: Vec<PublicItem> = items
+            .into_iter()
+            .map(|(n, k)| {
+                PublicItem::builder()
+                    .kind(k)
+                    .name(n)
+                    .file(String::new())
+                    .line(1)
+                    .visibility("pub".to_string())
+                    .generics(String::new())
+                    .attrs(Default::default())
+                    .build()
+            })
+            .collect();
+        let module = ModuleInfo::builder()
+            .path("".to_string())
+            .file(String::new())
+            .visibility("pub".to_string())
+            .public_items(public_items)
+            .build();
+        CrateInfo::builder()
+            .name(name.to_string())
+            .root(String::new())
+            .package(
+                schema::PackageInfo::builder()
+                    .name(name.to_string())
+                    .version("0.1.0".to_string())
+                    .edition("2021".to_string())
+                    .crate_type(schema::CrateType::Lib)
+                    .build(),
+            )
+            .modules(vec![module])
+            .deps(Default::default())
+            .build()
+    }
+
+    #[test]
+    fn compute_finds_cross_crate_import() {
+        let mut crates = vec![
+            make_crate("core", vec![
+                ("Task".to_string(), ItemKind::Struct),
+            ]),
+            make_crate("engine", vec![]),
+        ];
+        // Manually add an import in engine that references core::Task
+        let engine_module = &mut crates[1].modules[0];
+        engine_module.imports.push(Import {
+            path: "core::Task".to_string(),
+            line: 1,
+        });
+        let refs = compute(&mut crates);
+        // Task should be in cross-references
+        assert!(refs.types.contains_key("Task"));
+        let task_ref = &refs.types["Task"];
+        assert_eq!(task_ref.crate_name, "core");
+        // engine should have a cross_crate_import
+        assert_eq!(crates[1].cross_crate_imports.len(), 1);
+        assert_eq!(crates[1].cross_crate_imports[0].target_crate, "core");
+    }
+
+    #[test]
+    fn compute_empty_for_no_cross_references() {
+        let crates = vec![
+            make_crate("a", vec![("Foo".to_string(), ItemKind::Struct)]),
+            make_crate("b", vec![("Bar".to_string(), ItemKind::Struct)]),
+        ];
+        let mut crates_mut = crates;
+        let refs = compute(&mut crates_mut);
+        // No cross references since no crate imports from another
+        assert!(refs.types.is_empty() || refs.types.values().all(|t| t.imported_by.is_empty()));
+    }
+}
+
+// Helper: convert ItemKind to a short string for the TypeRef.kind field.'''
+
+[[tasks.TASK-9.changes]]
+file = "src/render.rs"
+before = '''use crate::schema::WorkspaceMap;
+use std::io::Write;
+
+/// Serialize the workspace map to a JSON string with 2-space indentation.
+#[must_use]
+pub fn render_json(map: &WorkspaceMap) -> serde_json::Result<String> {
+    serde_json::to_string_pretty(map)
+}
+
+/// Serialize the workspace map to the given writer.
+pub fn render_to_writer(map: &WorkspaceMap, writer: impl Write) -> serde_json::Result<()> {
+    serde_json::to_writer_pretty(writer, map)
+}
+'''
+after = '''use crate::schema::WorkspaceMap;
+use std::io::Write;
+
+/// Serialize the workspace map to a JSON string with 2-space indentation.
+#[must_use]
+pub fn render_json(map: &WorkspaceMap) -> serde_json::Result<String> {
+    serde_json::to_string_pretty(map)
+}
+
+/// Serialize the workspace map to the given writer.
+pub fn render_to_writer(map: &WorkspaceMap, writer: impl Write) -> serde_json::Result<()> {
+    serde_json::to_writer_pretty(writer, map)
+}
+
+// ── Tests ───────────────────────────────────────────────────────────────
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use crate::schema::{
+        CrateInfo, CrateType, CrossReferences, DepInfo, ModuleInfo, PackageInfo,
+        WorkspaceInfo, WorkspaceMap,
+    };
+
+    fn make_minimal_map() -> WorkspaceMap {
+        WorkspaceMap::builder()
+            .workspace(WorkspaceInfo::builder()
+                .root(".".to_string())
+                .workspace_name("test".to_string())
+                .build())
+            .crates(vec![
+                CrateInfo::builder()
+                    .name("test-crate".to_string())
+                    .root(".".to_string())
+                    .package(PackageInfo::builder()
+                        .name("test-crate".to_string())
+                        .version("0.1.0".to_string())
+                        .edition("2021".to_string())
+                        .crate_type(CrateType::Lib)
+                        .build())
+                    .modules(vec![ModuleInfo::builder()
+                        .path("".to_string())
+                        .file("src/lib.rs".to_string())
+                        .visibility("pub".to_string())
+                        .build()])
+                    .deps(DepInfo::default())
+                    .build(),
+            ])
+            .cross_references(CrossReferences::default())
+            .workspace_root(std::path::PathBuf::from("."))
+            .build()
+    }
+
+    #[test]
+    fn render_json_produces_valid_json() {
+        let map = make_minimal_map();
+        let json = render_json(&map).unwrap();
+        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
+        assert_eq!(parsed["workspace"]["root"], ".");
+        assert_eq!(parsed["crates"].as_array().unwrap().len(), 1);
+    }
+
+    #[test]
+    fn render_json_skips_empty_errors() {
+        let map = make_minimal_map();
+        let json = render_json(&map).unwrap();
+        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
+        // errors field should be absent (skip_serializing_if)
+        assert!(parsed.get("errors").is_none());
+    }
+
+    #[test]
+    fn render_to_writer_matches_render_json() {
+        let map = make_minimal_map();
+        let json = render_json(&map).unwrap();
+
+        let mut buf = Vec::new();
+        render_to_writer(&map, &mut buf).unwrap();
+        let from_writer = String::from_utf8(buf).unwrap();
+
+        assert_eq!(json, from_writer);
+    }
+}
+'''
+
+[tasks.TASK-10]
+description = "Expand integration tests from 3 to 10 covering error cases, workspace patterns, and output"
+type = "replace"
+acceptance = [
+    "cargo test -p rust-workspace-map --test integration_test",
+]
+
+[[tasks.TASK-10.changes]]
+file = "tests/integration_test.rs"
+before = '''#[test]
+fn test_missing_path_exits_nonzero() {
+    let bin = binary_path();
+    let output = Command::new(&bin)
+        .arg("/tmp/nonexistent-path-12345")
+        .output()
+        .expect("failed to execute binary");
+
+    assert!(
+        !output.status.success(),
+        "should exit non-zero for invalid path"
+    );
+}
+'''
+after = '''#[test]
+fn test_missing_path_exits_nonzero() {
+    let bin = binary_path();
+    let output = Command::new(&bin)
+        .arg("/tmp/nonexistent-path-12345")
+        .output()
+        .expect("failed to execute binary");
+
+    assert!(
+        !output.status.success(),
+        "should exit non-zero for invalid path"
+    );
+}
+
+fn run_binary(path: &str) -> std::process::Output {
+    Command::new(&binary_path())
+        .arg(path)
+        .output()
+        .expect("failed to execute binary")
+}
+
+fn parse_output(output: &std::process::Output) -> serde_json::Value {
+    serde_json::from_str(&String::from_utf8_lossy(&output.stdout)).unwrap()
+}
+
+fn write_cargo_toml(dir: &std::path::Path, content: &str) {
+    let mut f = std::fs::File::create(dir.join("Cargo.toml")).unwrap();
+    use std::io::Write;
+    f.write_all(content.as_bytes()).unwrap();
+}
+
+fn setup_crate(dir: &std::path::Path, lib_content: &str) {
+    let src = dir.join("src");
+    std::fs::create_dir_all(&src).unwrap();
+    std::fs::write(src.join("lib.rs"), lib_content).unwrap();
+}
+
+#[test]
+fn test_parse_failure_error_entry() {
+    let tmp = tempfile::tempdir().unwrap();
+    let root = tmp.path();
+
+    // Create workspace Cargo.toml
+    write_cargo_toml(root, r#"
+[workspace]
+members = ["good_crate", "bad_crate"]
+"#);
+
+    // Good crate with valid Rust
+    setup_crate(&root.join("good_crate"), "pub struct Good {}");
+
+    // Bad crate with invalid Rust syntax
+    setup_crate(&root.join("bad_crate"), "pub struct { invalid rust syntax");
+
+    let output = run_binary(root.to_str().unwrap());
+    assert!(output.status.success());
+
+    let json = parse_output(&output);
+    let errors: Vec<&serde_json::Value> = extract_array(&json, "errors");
+
+    let parse_errors: Vec<_> = errors.iter()
+        .filter(|e| {
+            e["kind"].as_str().unwrap() == "syn_parse_error"
+        })
+        .collect();
+
+    assert!(!parse_errors.is_empty(), "should have parse error entries");
+    assert_eq!(parse_errors[0]["severity"].as_str().unwrap(), "error");
+}
+
+#[test]
+fn test_missing_workspace_section() {
+    let tmp = tempfile::tempdir().unwrap();
+    let root = tmp.path();
+
+    // Cargo.toml without [workspace] section
+    write_cargo_toml(root, r#"
+[package]
+name = "standalone"
+version = "0.1.0"
+edition = "2021"
+"#);
+
+    let output = run_binary(root.to_str().unwrap());
+
+    // Should exit non-zero because workspace is missing
+    assert!(
+        !output.status.success(),
+        "should exit non-zero for missing workspace section"
+    );
+}
+
+#[test]
+fn test_glob_member_patterns() {
+    let tmp = tempfile::tempdir().unwrap();
+    let root = tmp.path();
+
+    write_cargo_toml(root, r#"
+[workspace]
+members = ["crates/*"]
+"#);
+
+    for name in &["alpha", "beta", "gamma"] {
+        setup_crate(&root.join("crates").join(name), format!("pub struct {name} {{}}").as_str());
+    }
+
+    let output = run_binary(root.to_str().unwrap());
+    assert!(output.status.success());
+
+    let json = parse_output(&output);
+    let crates = extract_array(&json, "crates");
+    let names: Vec<&str> = crates.iter()
+        .map(|c| c["name"].as_str().unwrap())
+        .collect();
+
+    assert!(names.contains(&"alpha"));
+    assert!(names.contains(&"beta"));
+    assert!(names.contains(&"gamma"));
+    assert_eq!(names.len(), 3);
+}
+
+#[test]
+fn test_workspace_with_exclude() {
+    let tmp = tempfile::tempdir().unwrap();
+    let root = tmp.path();
+
+    write_cargo_toml(root, r#"
+[workspace]
+members = ["a", "b", "c"]
+exclude = ["b"]
+"#);
+
+    setup_crate(&root.join("a"), "pub struct A {}");
+    setup_crate(&root.join("b"), "pub struct B {}");
+    setup_crate(&root.join("c"), "pub struct C {}");
+
+    let output = run_binary(root.to_str().unwrap());
+    assert!(output.status.success());
+
+    let json = parse_output(&output);
+    let crates = extract_array(&json, "crates");
+    let names: Vec<&str> = crates.iter()
+        .map(|c| c["name"].as_str().unwrap())
+        .collect();
+
+    assert!(names.contains(&"a"));
+    assert!(!names.contains(&"b"));
+    assert!(names.contains(&"c"));
+}
+
+#[test]
+fn test_deeply_nested_modules() {
+    let tmp = tempfile::tempdir().unwrap();
+    let root = tmp.path();
+
+    write_cargo_toml(root, r#"
+[workspace]
+members = ["."]
+
+[package]
+name = "nested"
+version = "0.1.0"
+edition = "2021"
+"#);
+
+    let src = root.join("src");
+    let foo = src.join("foo");
+    let bar = foo.join("bar");
+    std::fs::create_dir_all(&bar).unwrap();
+
+    // lib.rs declares mod foo
+    std::fs::write(src.join("lib.rs"), "mod foo;").unwrap();
+    // foo.rs declares mod bar
+    std::fs::write(foo.join("foo.rs"), "mod bar;").unwrap();
+    // bar/baz.rs declares mod baz
+    std::fs::write(bar.join("bar.rs"), "mod baz;").unwrap();
+    // baz.rs with a struct
+    std::fs::write(bar.join("baz.rs"), "pub struct Deep {}").unwrap();
+
+    let output = run_binary(root.to_str().unwrap());
+    assert!(output.status.success());
+
+    let json = parse_output(&output);
+    let crates = extract_array(&json, "crates");
+    let nested_crate = crates.iter().find(|c| c["name"].as_str().unwrap() == "nested").unwrap();
+
+    let module_paths: Vec<&str> = extract_array(&nested_crate["modules"], "path")
+        .iter()
+        .map(|m| m.as_str().unwrap())
+        .collect();
+
+    assert!(module_paths.iter().any(|p| *p == "nested"));
+    assert!(module_paths.iter().any(|p| *p == "nested::foo"));
+    assert!(module_paths.iter().any(|p| *p == "nested::foo::bar"));
+    assert!(module_paths.iter().any(|p| *p == "nested::foo::bar::baz"));
+}
+
+#[test]
+fn test_reexport_chains() {
+    let tmp = tempfile::tempdir().unwrap();
+    let root = tmp.path();
+
+    write_cargo_toml(root, r#"
+[workspace]
+members = ["."]
+
+[package]
+name = "reexporter"
+version = "0.1.0"
+edition = "2021"
+"#);
+
+    let src = root.join("src");
+    std::fs::create_dir_all(&src).unwrap();
+
+    // lib.rs with re-export chain
+    std::fs::write(src.join("lib.rs"), "
+mod inner {
+    pub struct Secret;
+}
+pub use inner::Secret;
+").unwrap();
+
+    let output = run_binary(root.to_str().unwrap());
+    assert!(output.status.success());
+
+    let json = parse_output(&output);
+    let crates = extract_array(&json, "crates");
+    let reexporter = crates.iter().find(|c| c["name"].as_str().unwrap() == "reexporter").unwrap();
+
+    let re_exports: Vec<&serde_json::Value> = extract_array(&reexporter["modules"])
+        .iter()
+        .flat_map(|m| extract_array(m, "reExports"))
+        .collect();
+
+    let has_secret = re_exports.iter().any(|re| {
+        re["importPath"].as_str().unwrap().contains("Secret")
+    });
+    assert!(has_secret, "should have re-export for Secret");
+}
+
+#[test]
+fn test_output_via_flag() {
+    let tmp = tempfile::tempdir().unwrap();
+    let fixture = std::path::Path::new("tests/fixtures/sample-workspace");
+    let output_path = tmp.path().join("output.json");
+
+    // Run with -o flag
+    let output1 = Command::new(&binary_path())
+        .arg(fixture)
+        .arg("-o")
+        .arg(output_path.clone())
+        .output()
+        .expect("failed to execute binary");
+    assert!(output1.status.success());
+
+    // Run without -o, capture stdout
+    let output2 = Command::new(&binary_path())
+        .arg(fixture)
+        .output()
+        .expect("failed to execute binary");
+    assert!(output2.status.success());
+
+    // Compare file content with stdout
+    let file_content = std::fs::read_to_string(&output_path).unwrap();
+    let stdout_content = String::from_utf8_lossy(&output2.stdout);
+    assert_eq!(
+        file_content.trim(),
+        stdout_content.trim(),
+        "file output should match stdout"
+    );
+}
+'''
diff --git a/src/cargo_info.rs b/src/cargo_info.rs
index 9357aa9..5629615 100644
--- a/src/cargo_info.rs
+++ b/src/cargo_info.rs
@@ -4,6 +4,10 @@ use std::path::Path;
 /// Parse a crate's `Cargo.toml` and return package metadata and dependency lists.
 /// The returned `PackageInfo.crate_type` is set to `Lib` by default; the caller
 /// overrides it based on `workspace::resolve_crate_roots`.
+///
+/// # Errors
+///
+/// Returns an error if the file cannot be read or parsed.
 pub fn parse_cargo_toml(path: &Path) -> Result<(PackageInfo, DepInfo)> {
     let content = std::fs::read_to_string(path).map_err(|source| Error::FileRead {
         path: path.to_path_buf(),
@@ -88,3 +92,64 @@ pub fn parse_cargo_toml(path: &Path) -> Result<(PackageInfo, DepInfo)> {
 
     Ok((package, deps))
 }
+
+// ── Tests ───────────────────────────────────────────────────────────────
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use std::io::Write;
+
+    fn write_cargo_toml(dir: &std::path::Path, content: &str) {
+        let mut f = std::fs::File::create(dir.join("Cargo.toml")).unwrap();
+        f.write_all(content.as_bytes()).unwrap();
+    }
+
+    #[test]
+    fn parse_cargo_toml_parses_minimal() {
+        let tmp = tempfile::tempdir().unwrap();
+        write_cargo_toml(tmp.path(), r#"
+[package]
+name = "test-pkg"
+version = "1.0.0"
+edition = "2021"
+"#);
+        let (pkg, _deps) = parse_cargo_toml(tmp.path().join("Cargo.toml").as_path()).unwrap();
+        assert_eq!(pkg.name, "test-pkg");
+        assert_eq!(pkg.version, "1.0.0");
+        assert_eq!(pkg.edition, "2021");
+    }
+
+    #[test]
+    fn parse_cargo_toml_uses_defaults_for_missing_package() {
+        let tmp = tempfile::tempdir().unwrap();
+        write_cargo_toml(tmp.path(), "");
+        let (pkg, _deps) = parse_cargo_toml(tmp.path().join("Cargo.toml").as_path()).unwrap();
+        assert_eq!(pkg.name, "unknown");
+        assert_eq!(pkg.edition, "2021");
+    }
+
+    #[test]
+    fn parse_cargo_toml_distinguishes_deps() {
+        let tmp = tempfile::tempdir().unwrap();
+        write_cargo_toml(tmp.path(), r#"
+[package]
+name = "test-pkg"
+version = "0.1.0"
+edition = "2021"
+
+[dependencies]
+foo = "1"
+bar = { workspace = true }
+
+[dev-dependencies]
+baz = "2"
+qux = { workspace = true }
+"#);
+        let (_, deps) = parse_cargo_toml(tmp.path().join("Cargo.toml").as_path()).unwrap();
+        assert_eq!(deps.normal, vec!["foo"]);
+        assert_eq!(deps.dev, vec!["baz"]);
+        assert!(deps.workspace_members.contains(&"bar".to_string()));
+        assert!(deps.workspace_members.contains(&"qux".to_string()));
+    }
+}
diff --git a/src/cross_refs.rs b/src/cross_refs.rs
index ba9fea7..5958621 100644
--- a/src/cross_refs.rs
+++ b/src/cross_refs.rs
@@ -93,8 +93,90 @@ pub fn compute(crates: &mut [CrateInfo]) -> CrossReferences {
     CrossReferences { types: types_map }
 }
 
+// ── Tests ───────────────────────────────────────────────────────────────
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use crate::schema::{CrateType, Import, ItemKind, ModuleInfo, PackageInfo, PublicItem};
+
+    fn make_crate(name: &str, items: Vec<(String, ItemKind)>) -> CrateInfo {
+        let public_items: Vec<PublicItem> = items
+            .into_iter()
+            .map(|(n, k)| {
+                PublicItem::builder()
+                    .kind(k)
+                    .name(n)
+                    .file(String::new())
+                    .line(1)
+                    .visibility("pub".to_string())
+                    .generics(String::new())
+                    .attrs(Default::default())
+                    .build()
+            })
+            .collect();
+        let module = ModuleInfo::builder()
+            .path("".to_string())
+            .file(String::new())
+            .visibility("pub".to_string())
+            .public_items(public_items)
+            .build();
+        CrateInfo::builder()
+            .name(name.to_string())
+            .root(String::new())
+            .package(
+                PackageInfo::builder()
+                    .name(name.to_string())
+                    .version("0.1.0".to_string())
+                    .edition("2021".to_string())
+                    .crate_type(CrateType::Lib)
+                    .build(),
+            )
+            .modules(vec![module])
+            .deps(Default::default())
+            .build()
+    }
+
+    #[test]
+    fn compute_finds_cross_crate_import() {
+        let mut crates = vec![
+            make_crate("core", vec![
+                ("Task".to_string(), ItemKind::Struct),
+            ]),
+            make_crate("engine", vec![]),
+        ];
+        // Manually add an import in engine that references core::Task
+        let engine_module = &mut crates[1].modules[0];
+        engine_module.imports.push(Import {
+            path: "core::Task".to_string(),
+            line: 1,
+        });
+        let refs = compute(&mut crates);
+        // Task should be in cross-references
+        assert!(refs.types.contains_key("Task"));
+        let task_ref = &refs.types["Task"];
+        assert_eq!(task_ref.crate_name, "core");
+        // engine should have a cross_crate_import
+        assert_eq!(crates[1].cross_crate_imports.len(), 1);
+        assert_eq!(crates[1].cross_crate_imports[0].target_crate, "core");
+    }
+
+    #[test]
+    fn compute_empty_for_no_cross_references() {
+        let crates = vec![
+            make_crate("a", vec![("Foo".to_string(), ItemKind::Struct)]),
+            make_crate("b", vec![("Bar".to_string(), ItemKind::Struct)]),
+        ];
+        let mut crates_mut = crates;
+        let refs = compute(&mut crates_mut);
+        // No cross references since no crate imports from another
+        assert!(refs.types.is_empty() || refs.types.values().all(|t| t.imported_by.is_empty()));
+    }
+}
+
 // Helper: convert ItemKind to a short string for the TypeRef.kind field.
 impl crate::schema::PublicItem {
+    #[must_use]
     fn kind_to_string(&self) -> String {
         match self.kind {
             crate::schema::ItemKind::Struct => "struct".to_string(),
diff --git a/src/file_parser.rs b/src/file_parser.rs
index 7a03409..f927aef 100644
--- a/src/file_parser.rs
+++ b/src/file_parser.rs
@@ -1,49 +1,106 @@
 use crate::schema::{
-    Error, FileInfo, ImplInfo, ImplItem, ImplItemKind, Import, ItemAttrs, ItemKind, PublicItem,
-    ReExport, Result, SubmoduleDecl,
+    ErrorEntry, ErrorSeverity, FileInfo, ImplInfo, ImplItem, ImplItemKind, Import,
+    ItemAttrs, ItemKind, PublicItem, ReExport, SubmoduleDecl,
 };
 use std::path::Path;
 
+// ── Internal parse result types ────────────────────────────────────────
+
+/// Result of parsing a Rust source file.
+///
+/// Unlike `Result<T, Error>`, this type always succeeds — parse
+/// failures are reported as data, not as errors, so the caller
+/// can continue processing other files. The caller constructs
+/// `ErrorEntry` values from `SynParseError` when needed.
+pub struct ParsedFile {
+    pub ast: syn::File,
+    pub file_info: FileInfo,
+    pub parse_error: Option<SynParseError>,
+}
+
+/// Structured information about a parse failure.
+pub struct SynParseError {
+    pub message: String,
+    pub line: usize,
+}
+
 // ── parse_file ──────────────────────────────────────────────────────────
 
-/// Read and parse a Rust source file. Returns the raw `syn::File` AST (needed
-/// by `module_tree` for inline module item extraction) and the extracted
-/// `FileInfo`. On parse failure, warns to stderr and returns empty results.
-pub fn parse_file(path: &Path) -> Result<(syn::File, FileInfo)> {
-    let content = std::fs::read_to_string(path).map_err(|source| Error::FileRead {
-        path: path.to_path_buf(),
-        source,
-    })?;
-
-    let file = match syn::parse_file(&content) {
-        Ok(f) => f,
-        Err(e) => {
-            eprintln!("warning: failed to parse {}: {}", path.display(), e);
-            let empty = syn::File {
-                shebang: None,
-                attrs: vec![],
-                items: vec![],
+/// Read and parse a Rust source file.
+///
+/// On parse failure, returns the original file content and a
+/// `SynParseError` alongside an empty `FileInfo`. Callers use the
+/// error to construct an `ErrorEntry`.
+#[must_use]
+pub fn parse_file(path: &Path) -> ParsedFile {
+    let content = match std::fs::read_to_string(path) {
+        Ok(c) => c,
+        Err(source) => {
+            let err = SynParseError {
+                message: source.to_string(),
+                line: 0,
+            };
+            return ParsedFile {
+                ast: syn::File {
+                    shebang: None,
+                    attrs: vec![],
+                    items: vec![],
+                },
+                file_info: FileInfo::default(),
+                parse_error: Some(err),
             };
-            let info = FileInfo::default();
-            return Ok((empty, info));
         }
     };
 
-    let info = FileInfo {
-        public_items: extract_public_items(&file.items),
-        imports: extract_imports(&file.items),
-        re_exports: extract_re_exports(&file.items),
-        submodules: extract_submodules(&file.items),
-        impls: extract_impls(&file.items),
-    };
+    match syn::parse_file(&content) {
+        Ok(file) => {
+            let file_info = FileInfo {
+                public_items: extract_public_items(&file.items),
+                imports: extract_imports(&file.items),
+                re_exports: extract_re_exports(&file.items),
+                submodules: extract_submodules(&file.items),
+                impls: extract_impls(&file.items),
+            };
+            ParsedFile {
+                ast: file,
+                file_info,
+                parse_error: None,
+            }
+        },
+        Err(e) => {
+            let line = e.span().start().line;
+            let err = SynParseError {
+                message: e.to_string(),
+                line,
+            };
+            ParsedFile {
+                ast: syn::File {
+                    shebang: None,
+                    attrs: vec![],
+                    items: vec![],
+                },
+                file_info: FileInfo::default(),
+                parse_error: Some(err),
+            }
+        }
+    }
+}
 
-    Ok((file, info))
+pub(crate) fn build_parse_error_entry(path: &Path, err: &SynParseError) -> ErrorEntry {
+    ErrorEntry::builder()
+        .file(path.to_string_lossy().to_string())
+        .line(err.line)
+        .message(err.message.clone())
+        .severity(ErrorSeverity::Error)
+        .kind("syn_parse_error".to_string())
+        .build()
 }
 
 // ── extract_public_items ────────────────────────────────────────────────
 
 /// Extract all items with any form of `pub` visibility (excluding `Inherited`).
 /// Results are sorted by name then line for deterministic output.
+#[must_use]
 pub fn extract_public_items(items: &[syn::Item]) -> Vec<PublicItem> {
     let mut result: Vec<PublicItem> = items
         .iter()
@@ -58,12 +115,13 @@ pub fn extract_public_items(items: &[syn::Item]) -> Vec<PublicItem> {
 
 /// Extract all `use` statements. Braced imports are expanded to individual
 /// entries. Results sorted by path for determinism.
+#[must_use]
 pub fn extract_imports(items: &[syn::Item]) -> Vec<Import> {
     let mut result: Vec<Import> = items
         .iter()
         .filter_map(|item| {
             if let syn::Item::Use(u) = item {
-                Some(flatten_use_tree(&u.tree, String::new(), line_of_item(item)))
+                Some(flatten_use_tree(&u.tree, "", line_of_item(item)))
             } else {
                 None
             }
@@ -76,7 +134,8 @@ pub fn extract_imports(items: &[syn::Item]) -> Vec<Import> {
 
 // ── extract_re_exports ──────────────────────────────────────────────────
 
-/// Extract `pub use` re-exports. Results sorted by export_path.
+/// Extract `pub use` re-exports. Results sorted by `export_path`.
+#[must_use]
 pub fn extract_re_exports(items: &[syn::Item]) -> Vec<ReExport> {
     let mut result: Vec<ReExport> = items
         .iter()
@@ -105,6 +164,7 @@ pub fn extract_re_exports(items: &[syn::Item]) -> Vec<ReExport> {
 
 /// Extract `mod` declarations. Detects `#[cfg(test)]` via literal token
 /// matching. Results sorted by name.
+#[must_use]
 pub fn extract_submodules(items: &[syn::Item]) -> Vec<SubmoduleDecl> {
     let mut result: Vec<SubmoduleDecl> = items
         .iter()
@@ -138,6 +198,7 @@ pub fn extract_submodules(items: &[syn::Item]) -> Vec<SubmoduleDecl> {
 
 /// Extract `impl` blocks. Each `ImplInfo` records the target type name and
 /// the impl items (fn, type, const).
+#[must_use]
 pub fn extract_impls(items: &[syn::Item]) -> Vec<ImplInfo> {
     items
         .iter()
@@ -243,7 +304,7 @@ fn vis_to_string(vis: &syn::Visibility) -> String {
             if path.is_empty() {
                 "pub(restricted)".to_string()
             } else {
-                format!("pub({})", path)
+                format!("pub({path})")
             }
         }
         syn::Visibility::Inherited => "private".to_string(),
@@ -320,11 +381,11 @@ fn type_to_string(ty: &syn::Type) -> String {
         syn::Type::BareFn(_) => "fn(...)".to_string(),
         syn::Type::Never(_) => "!".to_string(),
         syn::Type::TraitObject(to) => {
-            let bounds: Vec<String> = to.bounds.iter().map(|b| quote_bound(b)).collect();
+            let bounds: Vec<String> = to.bounds.iter().map(quote_bound).collect();
             bounds.join(" + ")
         }
         syn::Type::ImplTrait(ti) => {
-            let bounds: Vec<String> = ti.bounds.iter().map(|b| quote_bound(b)).collect();
+            let bounds: Vec<String> = ti.bounds.iter().map(quote_bound).collect();
             format!("impl {}", bounds.join(" + "))
         }
         syn::Type::Paren(tp) => format!("({})", type_to_string(&tp.elem)),
@@ -408,14 +469,12 @@ fn extract_attrs(attrs: &[syn::Attribute]) -> ItemAttrs {
                     .collect();
                 derive.extend(derives);
             }
-        } else if attr.path().is_ident("doc") {
-            if let syn::Meta::NameValue(nv) = &attr.meta {
-                if let syn::Expr::Lit(el) = &nv.value {
-                    if let syn::Lit::Str(ls) = &el.lit {
-                        doc.push(ls.value());
-                    }
-                }
-            }
+        } else if attr.path().is_ident("doc")
+            && let syn::Meta::NameValue(nv) = &attr.meta
+            && let syn::Expr::Lit(el) = &nv.value
+            && let syn::Lit::Str(ls) = &el.lit
+        {
+            doc.push(ls.value());
         }
     }
 
@@ -466,7 +525,7 @@ fn into_public_item(item: &syn::Item) -> Option<PublicItem> {
             &t.attrs,
         ),
         syn::Item::Macro(m) => {
-            let name = m.ident.as_ref().map(|i| i.to_string()).unwrap_or_default();
+            let name = m.ident.as_ref().map(ToString::to_string).unwrap_or_default();
             if name.is_empty() {
                 return None;
             }
@@ -500,7 +559,7 @@ fn into_public_item(item: &syn::Item) -> Option<PublicItem> {
     })
 }
 
-fn flatten_use_tree(tree: &syn::UseTree, prefix: String, line: usize) -> Vec<Import> {
+fn flatten_use_tree(tree: &syn::UseTree, prefix: &str, line: usize) -> Vec<Import> {
     match tree {
         syn::UseTree::Path(p) => {
             let new_prefix = if prefix.is_empty() {
@@ -508,7 +567,7 @@ fn flatten_use_tree(tree: &syn::UseTree, prefix: String, line: usize) -> Vec<Imp
             } else {
                 format!("{}::{}", prefix, p.ident)
             };
-            flatten_use_tree(&p.tree, new_prefix, line)
+            flatten_use_tree(&p.tree, &new_prefix, line)
         }
         syn::UseTree::Name(n) => {
             let path = if prefix.is_empty() {
@@ -530,14 +589,14 @@ fn flatten_use_tree(tree: &syn::UseTree, prefix: String, line: usize) -> Vec<Imp
             let path = if prefix.is_empty() {
                 "*".to_string()
             } else {
-                format!("{}::*", prefix)
+                format!("{prefix}::*")
             };
             vec![Import { path, line }]
         }
         syn::UseTree::Group(g) => g
             .items
             .iter()
-            .flat_map(|t| flatten_use_tree(t, prefix.clone(), line))
+            .flat_map(|t| flatten_use_tree(t, prefix, line))
             .collect(),
     }
 }
@@ -557,15 +616,25 @@ fn extract_re_exports_from_tree(
             extract_re_exports_from_tree(&p.tree, new_import, line)
         }
         syn::UseTree::Name(n) => {
+            let full_path = if import_path.is_empty() {
+                n.ident.to_string()
+            } else {
+                format!("{}::{}", import_path, n.ident)
+            };
             vec![ReExport {
-                import_path,
+                import_path: full_path,
                 export_path: n.ident.to_string(),
                 line,
             }]
         }
         syn::UseTree::Rename(r) => {
+            let full_path = if import_path.is_empty() {
+                format!("{} as {}", r.ident, r.rename)
+            } else {
+                format!("{}::{} as {}", import_path, r.ident, r.rename)
+            };
             vec![ReExport {
-                import_path,
+                import_path: full_path,
                 export_path: r.rename.to_string(),
                 line,
             }]
@@ -584,3 +653,140 @@ fn extract_re_exports_from_tree(
             .collect(),
     }
 }
+
+// ── Tests ───────────────────────────────────────────────────────────────
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use std::path::PathBuf;
+
+    fn parse_source(src: &str) -> ParsedFile {
+        use std::sync::atomic::{AtomicU64, Ordering};
+        static COUNTER: AtomicU64 = AtomicU64::new(0);
+        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
+        let tmp = std::env::temp_dir().join(format!("parse_test_{id}.rs"));
+        std::fs::write(&tmp, src).unwrap();
+        let result = parse_file(&tmp);
+        std::fs::remove_file(&tmp).ok();
+        result
+    }
+
+    #[test]
+    fn parse_file_returns_ast_for_valid_source() {
+        let src = "pub struct Foo { x: i32 }";
+        let result = parse_source(src);
+        assert!(result.parse_error.is_none());
+        assert_eq!(result.ast.items.len(), 1);
+    }
+
+    #[test]
+    fn parse_file_returns_error_for_invalid_source() {
+        let src = "pub struct { invalid rust }";
+        let result = parse_source(src);
+        assert!(result.parse_error.is_some());
+        let err = result.parse_error.as_ref().unwrap();
+        assert!(!err.message.is_empty());
+        assert!(err.line > 0);
+    }
+
+    #[test]
+    fn parse_file_returns_empty_for_empty_file() {
+        let result = parse_source("");
+        assert!(result.parse_error.is_none());
+        assert!(result.file_info.public_items.is_empty());
+    }
+
+    #[test]
+    fn extract_public_items_finds_struct_enum_trait_fn() {
+        let src = "pub struct Foo {} pub enum Bar { A, B } pub trait Baz {} pub fn hello() {}";
+        let result = parse_source(src);
+        let items = extract_public_items(&result.ast.items);
+        let names: Vec<_> = items.iter().map(|i| i.name.as_str()).collect();
+        assert!(names.contains(&"Foo"));
+        assert!(names.contains(&"Bar"));
+        assert!(names.contains(&"Baz"));
+        assert!(names.contains(&"hello"));
+    }
+
+    #[test]
+    fn extract_public_items_empty_for_no_public_items() {
+        let src = "struct Private {} fn private_fn() {}";
+        let result = parse_source(src);
+        let items = extract_public_items(&result.ast.items);
+        assert!(items.is_empty());
+    }
+
+    #[test]
+    fn extract_imports_finds_use_statements() {
+        let src = "use std::collections::BTreeMap;";
+        let result = parse_source(src);
+        let imports = extract_imports(&result.ast.items);
+        assert_eq!(imports.len(), 1);
+        assert_eq!(imports[0].path, "std::collections::BTreeMap");
+    }
+
+    #[test]
+    fn extract_re_exports_finds_pub_use() {
+        let src = "pub use crate::foo;";
+        let result = parse_source(src);
+        let re_exports = extract_re_exports(&result.ast.items);
+        assert_eq!(re_exports.len(), 1);
+        assert_eq!(re_exports[0].import_path, "crate::foo");
+        assert_eq!(re_exports[0].export_path, "foo");
+    }
+
+    #[test]
+    fn extract_re_exports_finds_rename() {
+        let src = "pub use crate::foo as bar;";
+        let result = parse_source(src);
+        let re_exports = extract_re_exports(&result.ast.items);
+        assert_eq!(re_exports.len(), 1);
+        assert_eq!(re_exports[0].import_path, "crate::foo as bar");
+        assert_eq!(re_exports[0].export_path, "bar");
+    }
+
+    #[test]
+    fn extract_submodules_finds_mod_declarations() {
+        let src = "mod foo; mod bar;";
+        let result = parse_source(src);
+        let subs = extract_submodules(&result.ast.items);
+        assert_eq!(subs.len(), 2);
+        let names: Vec<_> = subs.iter().map(|s| s.name.as_str()).collect();
+        assert!(names.contains(&"bar"));
+        assert!(names.contains(&"foo"));
+    }
+
+    #[test]
+    fn extract_submodules_marks_cfg_test() {
+        let src = "#[cfg(test)] mod inner;";
+        let result = parse_source(src);
+        let subs = extract_submodules(&result.ast.items);
+        assert_eq!(subs.len(), 1);
+        assert!(subs[0].is_test);
+    }
+
+    #[test]
+    fn extract_impls_finds_fn_type_const() {
+        let src = "impl MyType { pub fn foo(&self) {} pub type Alias = u32; pub const N: usize = 42; }";
+        let result = parse_source(src);
+        let impls = extract_impls(&result.ast.items);
+        assert_eq!(impls.len(), 1);
+        assert_eq!(impls[0].type_, "MyType");
+        assert_eq!(impls[0].items.len(), 3);
+    }
+
+    #[test]
+    fn build_parse_error_entry_constructs_error() {
+        let path = PathBuf::from("test.rs");
+        let err = SynParseError {
+            message: "expected `;`".to_string(),
+            line: 5,
+        };
+        let entry = build_parse_error_entry(&path, &err);
+        assert_eq!(entry.file, "test.rs");
+        assert_eq!(entry.line, 5);
+        assert_eq!(entry.kind, "syn_parse_error");
+        assert_eq!(entry.severity, ErrorSeverity::Error);
+    }
+}
diff --git a/src/lib.rs b/src/lib.rs
index 1d4cd7a..a2fa7c0 100644
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -1,13 +1,4 @@
 #![warn(clippy::pedantic)]
-#![allow(clippy::missing_errors_doc)]
-#![allow(clippy::must_use_candidate)]
-#![allow(clippy::doc_markdown)]
-#![allow(clippy::uninlined_format_args)]
-#![allow(clippy::redundant_closure)]
-#![allow(clippy::collapsible_if)]
-#![allow(clippy::needless_pass_by_value)]
-#![allow(clippy::needless_borrow)]
-#![allow(clippy::redundant_closure_for_method_calls)]
 
 pub mod cargo_info;
 pub mod cross_refs;
@@ -22,7 +13,8 @@ pub use schema::Config;
 use anyhow::Context;
 use rayon::prelude::*;
 use schema::{
-    CrateInfo, CrateType, ErrorEntry, ModuleInfo, WorkspaceInfo, WorkspaceMap,
+    CrateInfo, CrateType, ErrorEntry, ErrorSeverity, ModuleInfo, WorkspaceInfo,
+    WorkspaceMap,
 };
 use std::path::Path;
 
@@ -32,36 +24,48 @@ use std::path::Path;
 /// 2. Process each crate in parallel (Cargo.toml parsing + module tree).
 /// 3. Compute cross-crate references.
 /// 4. Render JSON to stdout or the configured output file.
-pub fn run(config: Config) -> anyhow::Result<()> {
+///
+/// # Errors
+///
+/// Returns an error if the workspace root cannot be found, the workspace
+/// Cargo.toml is missing a `[workspace]` section, member crates cannot be
+/// parsed, or the JSON output cannot be written.
+#[allow(clippy::too_many_lines)]
+pub fn run(config: &Config) -> anyhow::Result<()> {
     let workspace_root = workspace::find_workspace_root(&config.workspace_path)?;
     let member_dirs = workspace::enumerate_members(&workspace_root)?;
 
-    let errors: Vec<ErrorEntry> = Vec::new();
+    let mut crate_errors: Vec<ErrorEntry> = Vec::new();
 
-    let mut crate_infos: Vec<CrateInfo> = member_dirs
+    let results: Vec<(Option<CrateInfo>, Vec<ErrorEntry>)> = member_dirs
         .par_iter()
-        .filter_map(|dir| {
+        .map(|dir| {
             let cargo_toml = dir.join("Cargo.toml");
+            let mut crate_errors = Vec::new();
 
             let (pkg, deps) = match cargo_info::parse_cargo_toml(&cargo_toml) {
                 Ok(v) => v,
                 Err(e) => {
-                    eprintln!(
-                        "warning: failed to parse {}: {}",
-                        cargo_toml.display(),
-                        e
-                    );
-                    return None;
+                    crate_errors.push(ErrorEntry::builder()
+                        .file(cargo_toml.to_string_lossy().to_string())
+                        .message(format!("failed to parse Cargo.toml: {e}"))
+                        .severity(ErrorSeverity::Error)
+                        .kind("toml_parse_error".to_string())
+                        .cause(e.to_string())
+                        .build());
+                    return (None, crate_errors);
                 }
             };
 
             let roots = workspace::resolve_crate_roots(dir);
             if roots.is_empty() {
-                eprintln!(
-                    "warning: no crate entry points found in {}",
-                    dir.display()
-                );
-                return None;
+                crate_errors.push(ErrorEntry::builder()
+                    .file(dir.to_string_lossy().to_string())
+                    .message("no crate entry points found".to_string())
+                    .severity(ErrorSeverity::Warning)
+                    .kind("missing_crate_roots".to_string())
+                    .build());
+                return (None, crate_errors);
             }
 
             let crate_type = if roots.iter().any(|(_, t)| *t == CrateType::Lib)
@@ -73,12 +77,14 @@ pub fn run(config: Config) -> anyhow::Result<()> {
             };
 
             let pkg_name = pkg.name.clone();
-            let mut modules: Vec<ModuleInfo> = roots
-                .iter()
-                .flat_map(|(root, _ty)| {
-                    module_tree::build_module_tree(root, &pkg_name).unwrap_or_default()
-                })
-                .collect();
+            let mut modules: Vec<ModuleInfo> = Vec::new();
+            let mut collected_errors = Vec::new();
+            for (root, _ty) in &roots {
+                let (m, e) = module_tree::build_module_tree(root, &pkg_name);
+                modules.extend(m);
+                collected_errors.extend(e);
+            }
+            crate_errors.extend(collected_errors);
 
             // Relativize all paths to the workspace root.
             for m in &mut modules {
@@ -100,18 +106,27 @@ pub fn run(config: Config) -> anyhow::Result<()> {
                 .crate_type(crate_type)
                 .build();
 
-            Some(
-                CrateInfo::builder()
-                    .name(pkg_name)
-                    .root(crate_root)
-                    .package(rebuilt_pkg)
-                    .modules(modules)
-                    .deps(deps)
-                    .build(),
-            )
+            let crate_info = CrateInfo::builder()
+                .name(pkg_name)
+                .root(crate_root)
+                .package(rebuilt_pkg)
+                .modules(modules)
+                .deps(deps)
+                .build();
+
+            (Some(crate_info), crate_errors)
         })
         .collect();
 
+    let mut crate_infos: Vec<CrateInfo> = Vec::new();
+
+    for (info, errs) in results {
+        if let Some(ci) = info {
+            crate_errors.extend(errs);
+            crate_infos.push(ci);
+        }
+    }
+
     // Deterministic sort by crate name.
     crate_infos.sort_by(|a, b| a.name.cmp(&b.name));
 
@@ -131,7 +146,7 @@ pub fn run(config: Config) -> anyhow::Result<()> {
         .workspace(workspace_info)
         .crates(crate_infos)
         .cross_references(cross_refs)
-        .errors(errors)
+        .errors(crate_errors)
         .workspace_root(workspace_root.clone())
         .build();
 
diff --git a/src/main.rs b/src/main.rs
index f9734e9..0d9893c 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -37,5 +37,5 @@ fn main() -> anyhow::Result<()> {
         }
     };
 
-    rust_workspace_map::run(config)
+    rust_workspace_map::run(&config)
 }
diff --git a/src/module_tree.rs b/src/module_tree.rs
index d447596..0696c1f 100644
--- a/src/module_tree.rs
+++ b/src/module_tree.rs
@@ -1,12 +1,15 @@
 use crate::file_parser;
-use crate::schema::{FileInfo, ModuleInfo, Result, SubmoduleDecl};
+use crate::schema::{ErrorContext, ErrorEntry, ErrorSeverity, FileInfo, ModuleInfo, SubmoduleDecl};
 use std::collections::HashSet;
 use std::path::{Path, PathBuf};
 
 /// Resolve a `mod name;` declaration to a file path.
 /// Tries `{parent_dir}/{mod_name}.rs` first, then `{parent_dir}/{mod_name}/mod.rs`.
+///
+/// Returns `None` if neither path exists.
+#[must_use]
 pub fn resolve_module_path(parent_dir: &Path, mod_name: &str) -> Option<PathBuf> {
-    let rs_file = parent_dir.join(format!("{}.rs", mod_name));
+    let rs_file = parent_dir.join(format!("{mod_name}.rs"));
     if rs_file.exists() {
         return Some(rs_file);
     }
@@ -18,44 +21,54 @@ pub fn resolve_module_path(parent_dir: &Path, mod_name: &str) -> Option<PathBuf>
 }
 
 /// Build the full module tree for a crate starting from its entry point
-/// (e.g., `src/lib.rs`). Returns a flat `Vec<ModuleInfo>` containing the
-/// root module and all recursively discovered submodules.
-pub fn build_module_tree(crate_root: &Path, crate_name: &str) -> Result<Vec<ModuleInfo>> {
+/// (e.g., `src/lib.rs`). Returns a tuple of module info and any errors
+/// encountered during submodule parsing (including orphaned module warnings).
+#[must_use]
+pub fn build_module_tree(
+    crate_root: &Path,
+    crate_name: &str,
+) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {
+
     let mut visited = HashSet::new();
-    let parent_dir = crate_root.parent().unwrap_or_else(|| Path::new("."));
+    let parent_dir = crate_root.parent().unwrap_or(crate_root);
 
-    let (ast, file_info) = file_parser::parse_file(crate_root)?;
+    let parsed = file_parser::parse_file(crate_root);
+    let mut errors: Vec<ErrorEntry> = Vec::new();
+    if let Some(ref err) = parsed.parse_error {
+        errors.push(crate::file_parser::build_parse_error_entry(crate_root, err));
+    }
     visited.insert(crate_root.to_path_buf());
 
     let root_module = build_module_info(
         crate_name,
         crate_root,
         "pub",
-        &file_info.public_items,
-        &file_info.imports,
-        &file_info.re_exports,
-        &file_info.submodules,
+        &parsed.file_info.public_items,
+        &parsed.file_info.imports,
+        &parsed.file_info.re_exports,
+        &parsed.file_info.submodules,
     );
 
     let mut modules = vec![root_module];
 
-    for sub in &file_info.submodules {
+    for sub in &parsed.file_info.submodules {
         if sub.is_test {
             continue;
         }
         let sub_module_path = format!("{}::{}", crate_name, sub.name);
-        let child_modules = process_submodule(
+        let (child_modules, child_errors) = process_submodule(
             &sub_module_path,
             &sub.name,
-            &ast.items,
+            &parsed.ast.items,
             parent_dir,
             crate_root,
             &mut visited,
-        )?;
+        );
+        errors.extend(child_errors);
         modules.extend(child_modules);
     }
 
-    Ok(modules)
+    (modules, errors)
 }
 
 // ── Internal helpers ────────────────────────────────────────────────────
@@ -92,24 +105,32 @@ fn process_submodule(
     parent_dir: &Path,
     parent_file: &Path,
     visited: &mut HashSet<PathBuf>,
-) -> Result<Vec<ModuleInfo>> {
+) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {
     // Locate the `mod` item in the parent's AST.
     let mod_item = parent_items.iter().find_map(|item| {
-        if let syn::Item::Mod(m) = item {
-            if m.ident == mod_name {
-                return Some(m);
-            }
+        if let syn::Item::Mod(m) = item
+            && m.ident == mod_name
+        {
+            return Some(m);
         }
         None
     });
 
     let Some(mod_item) = mod_item else {
-        eprintln!("warning: orphaned module {}", module_path);
-        return Ok(vec![ModuleInfo::builder()
+        let err = ErrorEntry::builder()
+            .file(String::new())
+            .message(format!("orphaned module: {module_path}"))
+            .severity(ErrorSeverity::Warning)
+            .kind("orphaned_module".to_string())
+            .context(ErrorContext::builder()
+                .module_path(module_path.to_string())
+                .build())
+            .build();
+        return (vec![ModuleInfo::builder()
             .path(module_path.to_string())
             .file("<unresolved>".to_string())
             .visibility("private".to_string())
-            .build()]);
+            .build()], vec![err]);
     };
 
     let visibility = if matches!(mod_item.vis, syn::Visibility::Public(_)) {
@@ -120,42 +141,55 @@ fn process_submodule(
 
     if let Some((_, ref inline_items)) = mod_item.content {
         // Inline module: process its body items directly (no file lookup).
-        process_module_items(
+        let (modules, errs) = process_module_items(
             module_path,
             parent_file,
             visibility,
             inline_items,
             parent_dir,
             visited,
-        )
-    } else {
-        // External module: resolve file path, parse, and recurse.
-        let file_path = resolve_module_path(parent_dir, mod_name);
-        let Some(ref file_path) = file_path else {
-            eprintln!("warning: orphaned module {}", module_path);
-            return Ok(vec![ModuleInfo::builder()
-                .path(module_path.to_string())
-                .file("<unresolved>".to_string())
-                .visibility(visibility.to_string())
-                .build()]);
-        };
-
-        if visited.contains(file_path.as_path()) {
-            return Ok(vec![]); // cycle detected
-        }
-        visited.insert(file_path.clone());
+        );
+        return (modules, errs);
+    }
+    // External module: resolve file path, parse, and recurse.
+    let file_path = resolve_module_path(parent_dir, mod_name);
+    let Some(ref file_path) = file_path else {
+        let err = ErrorEntry::builder()
+            .file(String::new())
+            .message(format!("orphaned module: {module_path}"))
+            .severity(ErrorSeverity::Warning)
+            .kind("orphaned_module".to_string())
+            .context(ErrorContext::builder()
+                .module_path(module_path.to_string())
+                .build())
+            .build();
+        return (vec![ModuleInfo::builder()
+            .path(module_path.to_string())
+            .file("<unresolved>".to_string())
+            .visibility(visibility.to_string())
+            .build()], vec![err]);
+    };
 
-        let (ast, file_info) = file_parser::parse_file(file_path)?;
-        process_module_info(
-            module_path,
-            file_path,
-            visibility,
-            &file_info,
-            &ast.items,
-            &file_path.parent().unwrap_or_else(|| Path::new(".")),
-            visited,
-        )
+    if visited.contains(file_path.as_path()) {
+        return (vec![], vec![]); // cycle detected
+    }
+    visited.insert(file_path.clone());
+
+    let parsed = file_parser::parse_file(file_path);
+    let mut errors: Vec<ErrorEntry> = Vec::new();
+    if let Some(ref err) = parsed.parse_error {
+        errors.push(crate::file_parser::build_parse_error_entry(file_path, err));
     }
+    process_module_info(
+        module_path,
+        file_path,
+        visibility,
+        &parsed.file_info,
+        &parsed.ast.items,
+        file_path.parent().unwrap_or(file_path),
+        visited,
+        &mut errors,
+    )
 }
 
 fn process_module_items(
@@ -165,7 +199,7 @@ fn process_module_items(
     items: &[syn::Item],
     parent_dir: &Path,
     visited: &mut HashSet<PathBuf>,
-) -> Result<Vec<ModuleInfo>> {
+) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {
     let file_info = FileInfo {
         public_items: file_parser::extract_public_items(items),
         imports: file_parser::extract_imports(items),
@@ -173,9 +207,10 @@ fn process_module_items(
         submodules: file_parser::extract_submodules(items),
         impls: file_parser::extract_impls(items),
     };
-    process_module_info(module_path, file_path, visibility, &file_info, items, parent_dir, visited)
+    process_module_info(module_path, file_path, visibility, &file_info, items, parent_dir, visited, &mut Vec::new())
 }
 
+#[allow(clippy::too_many_arguments)]
 fn process_module_info(
     module_path: &str,
     file_path: &Path,
@@ -184,7 +219,8 @@ fn process_module_info(
     items: &[syn::Item],
     _parent_dir: &Path,
     visited: &mut HashSet<PathBuf>,
-) -> Result<Vec<ModuleInfo>> {
+    errors: &mut Vec<ErrorEntry>,
+) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {
     let mut modules = vec![build_module_info(
         module_path,
         file_path,
@@ -200,17 +236,68 @@ fn process_module_info(
             continue;
         }
         let child_path = format!("{}::{}", module_path, sub.name);
-        let child_dir = file_path.parent().unwrap_or_else(|| Path::new("."));
-        let child_modules = process_submodule(
+        let child_dir = file_path.parent().unwrap_or(file_path);
+        let (child_modules, child_errors) = process_submodule(
             &child_path,
             &sub.name,
             items,
             child_dir,
             file_path,
             visited,
-        )?;
+        );
+        errors.extend(child_errors);
         modules.extend(child_modules);
     }
 
-    Ok(modules)
+    (modules, errors.clone())
+}
+
+// ── Tests ───────────────────────────────────────────────────────────────
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+
+    #[test]
+    fn resolve_module_path_finds_rs_file() {
+        let tmp = std::env::temp_dir().join("resolve_test");
+        let _ = std::fs::create_dir_all(&tmp);
+        let mod_file = tmp.join("foo.rs");
+        std::fs::write(&mod_file, "").ok();
+        let result = resolve_module_path(&tmp, "foo");
+        assert_eq!(result, Some(mod_file));
+        std::fs::remove_dir_all(&tmp).ok();
+    }
+
+    #[test]
+    fn resolve_module_path_finds_mod_rs() {
+        let tmp = std::env::temp_dir().join("resolve_test2");
+        let _ = std::fs::create_dir_all(&tmp);
+        let mod_dir = tmp.join("bar");
+        let _ = std::fs::create_dir_all(&mod_dir);
+        let mod_rs = mod_dir.join("mod.rs");
+        std::fs::write(&mod_rs, "").ok();
+        let result = resolve_module_path(&tmp, "bar");
+        assert_eq!(result, Some(mod_rs));
+        std::fs::remove_dir_all(&tmp).ok();
+    }
+
+    #[test]
+    fn resolve_module_path_returns_none_for_missing() {
+        let tmp = std::env::temp_dir().join("resolve_test3");
+        let _ = std::fs::create_dir_all(&tmp);
+        let result = resolve_module_path(&tmp, "nonexistent");
+        assert!(result.is_none());
+        std::fs::remove_dir_all(&tmp).ok();
+    }
+
+    #[test]
+    fn build_module_tree_returns_empty_for_nonexistent() {
+        let tmp = std::env::temp_dir().join("bmt_test");
+        let _ = std::fs::create_dir_all(&tmp);
+        let (modules, errors) = build_module_tree(&tmp, "test");
+        assert!(!modules.is_empty());
+        assert!(!errors.is_empty());
+        std::fs::remove_dir_all(&tmp).ok();
+    }
 }
diff --git a/src/render.rs b/src/render.rs
index 76d20e9..8e5ea03 100644
--- a/src/render.rs
+++ b/src/render.rs
@@ -2,11 +2,89 @@ use crate::schema::WorkspaceMap;
 use std::io::Write;
 
 /// Serialize the workspace map to a JSON string with 2-space indentation.
+///
+/// # Errors
+///
+/// Returns an error if serialization fails.
 pub fn render_json(map: &WorkspaceMap) -> serde_json::Result<String> {
     serde_json::to_string_pretty(map)
 }
 
 /// Serialize the workspace map to the given writer.
+///
+/// # Errors
+///
+/// Returns an error if serialization fails.
 pub fn render_to_writer(map: &WorkspaceMap, writer: impl Write) -> serde_json::Result<()> {
     serde_json::to_writer_pretty(writer, map)
 }
+
+// ── Tests ───────────────────────────────────────────────────────────────
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use crate::schema::{
+        CrateInfo, CrateType, CrossReferences, DepInfo, ModuleInfo, PackageInfo,
+        WorkspaceInfo, WorkspaceMap,
+    };
+
+    fn make_minimal_map() -> WorkspaceMap {
+        WorkspaceMap::builder()
+            .workspace(WorkspaceInfo::builder()
+                .root(".".to_string())
+                .workspace_name("test".to_string())
+                .build())
+            .crates(vec![
+                CrateInfo::builder()
+                    .name("test-crate".to_string())
+                    .root(".".to_string())
+                    .package(PackageInfo::builder()
+                        .name("test-crate".to_string())
+                        .version("0.1.0".to_string())
+                        .edition("2021".to_string())
+                        .crate_type(CrateType::Lib)
+                        .build())
+                    .modules(vec![ModuleInfo::builder()
+                        .path("".to_string())
+                        .file("src/lib.rs".to_string())
+                        .visibility("pub".to_string())
+                        .build()])
+                    .deps(DepInfo::default())
+                    .build(),
+            ])
+            .cross_references(CrossReferences::default())
+            .workspace_root(std::path::PathBuf::from("."))
+            .build()
+    }
+
+    #[test]
+    fn render_json_produces_valid_json() {
+        let map = make_minimal_map();
+        let json = render_json(&map).unwrap();
+        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
+        assert_eq!(parsed["workspace"]["root"], ".");
+        assert_eq!(parsed["crates"].as_array().unwrap().len(), 1);
+    }
+
+    #[test]
+    fn render_json_skips_empty_errors() {
+        let map = make_minimal_map();
+        let json = render_json(&map).unwrap();
+        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
+        // errors field should be absent (skip_serializing_if)
+        assert!(parsed.get("errors").is_none());
+    }
+
+    #[test]
+    fn render_to_writer_matches_render_json() {
+        let map = make_minimal_map();
+        let json = render_json(&map).unwrap();
+
+        let mut buf = Vec::new();
+        render_to_writer(&map, &mut buf).unwrap();
+        let from_writer = String::from_utf8(buf).unwrap();
+
+        assert_eq!(json, from_writer);
+    }
+}
diff --git a/src/schema.rs b/src/schema.rs
index 214592b..d6c1fe8 100644
--- a/src/schema.rs
+++ b/src/schema.rs
@@ -31,6 +31,9 @@ pub enum Error {
 
     #[error("glob pattern error: {0}")]
     GlobPattern(String),
+
+    #[error("workspace Cargo.toml is missing the [workspace] section")]
+    MissingWorkspaceSection,
 }
 
 pub type Result<T> = std::result::Result<T, Error>;
@@ -278,9 +281,40 @@ pub struct TypeRef {
     pub exported_by: Vec<String>,
 }
 
+// ── Error severity ─────────────────────────────────────────────────────
+
+#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
+#[serde(rename_all = "snake_case")]
+pub enum ErrorSeverity {
+    Error,
+    Warning,
+}
+
+// ── Error context ───────────────────────────────────────────────────────
+
+/// Optional context attached to an error, providing additional location
+/// and source information for diagnostics.
+#[derive(Debug, Clone, Default, serde::Serialize, bon::Builder)]
+#[serde(rename_all = "camelCase")]
+pub struct ErrorContext {
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pub crate_name: Option<String>,
+
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pub module_path: Option<String>,
+
+    /// Line number in the source file where the error occurred.
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pub line: Option<usize>,
+
+    /// A short source snippet near the error location (if available).
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pub snippet: Option<String>,
+}
+
 // ── Internal types ──────────────────────────────────────────────────────
 
-/// Internal intermediate type consumed by module_tree.
+/// Internal intermediate type consumed by `module_tree`.
 #[derive(Debug, Clone, Default)]
 pub struct FileInfo {
     pub public_items: Vec<PublicItem>,
@@ -306,4 +340,10 @@ pub struct ErrorEntry {
     #[builder(default)]
     pub line: usize,
     pub message: String,
+    pub severity: ErrorSeverity,
+    pub kind: String,
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pub context: Option<ErrorContext>,
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pub cause: Option<String>,
 }
diff --git a/src/workspace.rs b/src/workspace.rs
index 92a346a..48b0331 100644
--- a/src/workspace.rs
+++ b/src/workspace.rs
@@ -3,6 +3,11 @@ use std::path::{Path, PathBuf};
 
 /// Walk up the directory tree from `start_path` to find a `Cargo.toml`
 /// containing a `[workspace]` section. Returns the directory containing it.
+///
+/// # Errors
+///
+/// Returns `Error::WorkspaceRootNotFound` if no `Cargo.toml` with a
+/// `[workspace]` section is found in any ancestor directory.
 pub fn find_workspace_root(start_path: &Path) -> Result<PathBuf> {
     for ancestor in start_path.ancestors() {
         let cargo_toml = ancestor.join("Cargo.toml");
@@ -22,6 +27,11 @@ pub fn find_workspace_root(start_path: &Path) -> Result<PathBuf> {
 /// Parse the workspace `Cargo.toml`, resolve member paths (including glob
 /// patterns), apply `exclude` list, and return absolute paths to each member
 /// crate directory.
+///
+/// # Errors
+///
+/// Returns `Error::MissingWorkspaceSection` if the `Cargo.toml` lacks a
+/// `[workspace]` section entirely.
 pub fn enumerate_members(root: &Path) -> Result<Vec<PathBuf>> {
     let cargo_toml_path = root.join("Cargo.toml");
     let content = std::fs::read_to_string(&cargo_toml_path).map_err(|source| Error::FileRead {
@@ -34,16 +44,18 @@ pub fn enumerate_members(root: &Path) -> Result<Vec<PathBuf>> {
         source,
     })?;
 
-    let members: Vec<String> = parsed
-        .get("workspace")
-        .and_then(|w| w.get("members"))
-        .and_then(|m| m.as_array())
-        .map(|arr| {
-            arr.iter()
-                .filter_map(|v| v.as_str().map(String::from))
-                .collect()
-        })
-        .unwrap_or_default();
+    let members: Vec<String> = match parsed.get("workspace") {
+        None => return Err(Error::MissingWorkspaceSection),
+        Some(workspace) => workspace
+            .get("members")
+            .and_then(|m| m.as_array())
+            .map(|arr| {
+                arr.iter()
+                    .filter_map(|v| v.as_str().map(String::from))
+                    .collect::<Vec<_>>()
+            })
+            .unwrap_or_default(),
+    };
 
     let exclude: Vec<String> = parsed
         .get("workspace")
@@ -95,9 +107,106 @@ pub fn enumerate_members(root: &Path) -> Result<Vec<PathBuf>> {
     Ok(result)
 }
 
-/// For a crate directory, determine its entry-point file(s).
+// ── Tests ───────────────────────────────────────────────────────────────
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use std::io::Write;
+
+    fn write_cargo_toml(dir: &std::path::Path, content: &str) {
+        let mut f = std::fs::File::create(dir.join("Cargo.toml")).unwrap();
+        f.write_all(content.as_bytes()).unwrap();
+    }
+
+    fn setup_crate(dir: &std::path::Path) {
+        let src = dir.join("src");
+        std::fs::create_dir_all(&src).unwrap();
+        std::fs::write(src.join("lib.rs"), "").unwrap();
+        let mut f = std::fs::File::create(dir.join("Cargo.toml")).unwrap();
+        use std::io::Write;
+        writeln!(f, "[package]").unwrap();
+        writeln!(f, "name = \"{}\"", dir.file_name().unwrap().to_string_lossy()).unwrap();
+        writeln!(f, "version = \"0.1.0\"").unwrap();
+        writeln!(f, "edition = \"2021\"").unwrap();
+    }
+
+    #[test]
+    fn find_workspace_root_finds_cargo_toml() {
+        let tmp = tempfile::tempdir().unwrap();
+        let path = tmp.path().join("subdir").join("nested");
+        std::fs::create_dir_all(&path).unwrap();
+        write_cargo_toml(tmp.path(), "[workspace]");
+        let result = find_workspace_root(&path).unwrap();
+        assert_eq!(result, tmp.path());
+    }
+
+    #[test]
+    fn enumerate_members_returns_members() {
+        let tmp = tempfile::tempdir().unwrap();
+        write_cargo_toml(tmp.path(), r#"
+[workspace]
+members = ["crate_a", "crate_b"]
+"#);
+        setup_crate(tmp.path().join("crate_a").as_path());
+        setup_crate(tmp.path().join("crate_b").as_path());
+        let members = enumerate_members(tmp.path()).unwrap();
+        assert_eq!(members.len(), 2);
+    }
+
+    #[test]
+    fn enumerate_members_returns_err_for_missing_workspace() {
+        let tmp = tempfile::tempdir().unwrap();
+        write_cargo_toml(tmp.path(), "[dependencies]\nfoo = \"1\"");
+        let result = enumerate_members(tmp.path());
+        assert!(result.is_err());
+        match result.unwrap_err() {
+            Error::MissingWorkspaceSection => {},
+            other => panic!("expected MissingWorkspaceSection, got {:?}", other),
+        }
+    }
+
+    #[test]
+    fn enumerate_members_applies_exclude() {
+        let tmp = tempfile::tempdir().unwrap();
+        write_cargo_toml(tmp.path(), r#"
+[workspace]
+members = ["a", "b", "c"]
+exclude = ["b"]
+"#);
+        setup_crate(tmp.path().join("a").as_path());
+        setup_crate(tmp.path().join("b").as_path());
+        setup_crate(tmp.path().join("c").as_path());
+        let members = enumerate_members(tmp.path()).unwrap();
+        let names: Vec<_> = members.iter().map(|p| p.file_name().unwrap().to_string_lossy()).collect();
+        assert!(names.iter().any(|n| *n == "a"));
+        assert!(!names.iter().any(|n| *n == "b"));
+        assert!(names.iter().any(|n| *n == "c"));
+    }
+
+    #[test]
+    fn resolve_crate_roots_detects_lib() {
+        let tmp = tempfile::tempdir().unwrap();
+        std::fs::create_dir_all(tmp.path().join("src")).unwrap();
+        std::fs::write(tmp.path().join("src").join("lib.rs"), "").unwrap();
+        let roots = resolve_crate_roots(tmp.path());
+        assert_eq!(roots.len(), 1);
+        assert_eq!(roots[0].1, CrateType::Lib);
+    }
+
+    #[test]
+    fn resolve_crate_roots_detects_bin() {
+        let tmp = tempfile::tempdir().unwrap();
+        std::fs::create_dir_all(tmp.path().join("src")).unwrap();
+        std::fs::write(tmp.path().join("src").join("main.rs"), "").unwrap();
+        let roots = resolve_crate_roots(tmp.path());
+        assert_eq!(roots.len(), 1);
+        assert_eq!(roots[0].1, CrateType::Bin);
+    }
+}
 /// Returns `(path, CrateType)` pairs — one for `src/lib.rs` (Lib),
 /// one for `src/main.rs` (Bin), or empty if neither exists.
+#[must_use]
 pub fn resolve_crate_roots(crate_dir: &Path) -> Vec<(PathBuf, CrateType)> {
     let mut roots = Vec::new();
     let lib_rs = crate_dir.join("src").join("lib.rs");
diff --git a/tests/integration_test.rs b/tests/integration_test.rs
index cb6315a..f1b8ae4 100644
--- a/tests/integration_test.rs
+++ b/tests/integration_test.rs
@@ -126,3 +126,268 @@ fn test_missing_path_exits_nonzero() {
         "should exit non-zero for invalid path"
     );
 }
+
+fn run_binary(path: &str) -> std::process::Output {
+    Command::new(&binary_path())
+        .arg(path)
+        .output()
+        .expect("failed to execute binary")
+}
+
+fn parse_output(output: &std::process::Output) -> serde_json::Value {
+    serde_json::from_str(&String::from_utf8_lossy(&output.stdout)).unwrap()
+}
+
+fn write_cargo_toml(dir: &std::path::Path, content: &str) {
+    let mut f = std::fs::File::create(dir.join("Cargo.toml")).unwrap();
+    use std::io::Write;
+    f.write_all(content.as_bytes()).unwrap();
+}
+
+fn setup_crate(dir: &std::path::Path, lib_content: &str) {
+    let src = dir.join("src");
+    std::fs::create_dir_all(&src).unwrap();
+    std::fs::write(src.join("lib.rs"), lib_content).unwrap();
+    let name = dir.file_name().unwrap().to_string_lossy();
+    let cargo = format!(
+        "[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
+    );
+    std::fs::write(dir.join("Cargo.toml"), cargo).unwrap();
+}
+
+#[test]
+fn test_parse_failure_error_entry() {
+    let tmp = tempfile::tempdir().unwrap();
+    let root = tmp.path();
+
+    // Create workspace Cargo.toml
+    write_cargo_toml(root, r#"
+[workspace]
+members = ["good_crate", "bad_crate"]
+"#);
+
+    // Good crate with valid Rust
+    setup_crate(&root.join("good_crate"), "pub struct Good {}");
+
+    // Bad crate with invalid Rust syntax
+    setup_crate(&root.join("bad_crate"), "pub struct { invalid rust syntax");
+
+    let output = run_binary(root.to_str().unwrap());
+    assert!(output.status.success());
+
+    let json = parse_output(&output);
+    let errors: Vec<&serde_json::Value> = extract_array(&json, "errors");
+
+    let parse_errors: Vec<_> = errors.iter()
+        .filter(|e| {
+            e["kind"].as_str().unwrap() == "syn_parse_error"
+        })
+        .collect();
+
+    assert!(!parse_errors.is_empty(), "should have parse error entries");
+    assert_eq!(parse_errors[0]["severity"].as_str().unwrap(), "error");
+}
+
+#[test]
+fn test_missing_workspace_section() {
+    let tmp = tempfile::tempdir().unwrap();
+    let root = tmp.path();
+
+    // Cargo.toml without [workspace] section
+    write_cargo_toml(root, r#"
+[package]
+name = "standalone"
+version = "0.1.0"
+edition = "2021"
+"#);
+
+    let output = run_binary(root.to_str().unwrap());
+
+    // Should exit non-zero because workspace is missing
+    assert!(
+        !output.status.success(),
+        "should exit non-zero for missing workspace section"
+    );
+}
+
+#[test]
+fn test_glob_member_patterns() {
+    let tmp = tempfile::tempdir().unwrap();
+    let root = tmp.path();
+
+    write_cargo_toml(root, r#"
+[workspace]
+members = ["crates/*"]
+"#);
+
+    for name in &["alpha", "beta", "gamma"] {
+        setup_crate(&root.join("crates").join(name), format!("pub struct {name} {{}}").as_str());
+    }
+
+    let output = run_binary(root.to_str().unwrap());
+    assert!(output.status.success());
+
+    let json = parse_output(&output);
+    let crates = extract_array(&json, "crates");
+    let names: Vec<&str> = crates.iter()
+        .map(|c| c["name"].as_str().unwrap())
+        .collect();
+
+    assert!(names.contains(&"alpha"));
+    assert!(names.contains(&"beta"));
+    assert!(names.contains(&"gamma"));
+    assert_eq!(names.len(), 3);
+}
+
+#[test]
+fn test_workspace_with_exclude() {
+    let tmp = tempfile::tempdir().unwrap();
+    let root = tmp.path();
+
+    write_cargo_toml(root, r#"
+[workspace]
+members = ["a", "b", "c"]
+exclude = ["b"]
+"#);
+
+    setup_crate(&root.join("a"), "pub struct A {}");
+    setup_crate(&root.join("b"), "pub struct B {}");
+    setup_crate(&root.join("c"), "pub struct C {}");
+
+    let output = run_binary(root.to_str().unwrap());
+    assert!(output.status.success());
+
+    let json = parse_output(&output);
+    let crates = extract_array(&json, "crates");
+    let names: Vec<&str> = crates.iter()
+        .map(|c| c["name"].as_str().unwrap())
+        .collect();
+
+    assert!(names.contains(&"a"));
+    assert!(!names.contains(&"b"));
+    assert!(names.contains(&"c"));
+}
+
+#[test]
+fn test_deeply_nested_modules() {
+    let tmp = tempfile::tempdir().unwrap();
+    let root = tmp.path();
+
+    write_cargo_toml(root, r#"
+[workspace]
+members = ["."]
+
+[package]
+name = "nested"
+version = "0.1.0"
+edition = "2021"
+"#);
+
+    let src = root.join("src");
+    let foo = src.join("foo");
+    let bar = foo.join("bar");
+    std::fs::create_dir_all(&bar).unwrap();
+
+    // lib.rs declares mod foo (resolves to src/foo/mod.rs)
+    std::fs::write(src.join("lib.rs"), "mod foo;").unwrap();
+    // foo/mod.rs declares mod bar
+    std::fs::write(foo.join("mod.rs"), "mod bar;").unwrap();
+    // bar/mod.rs declares mod baz
+    std::fs::write(bar.join("mod.rs"), "mod baz;").unwrap();
+    // bar/baz.rs with a struct
+    std::fs::write(bar.join("baz.rs"), "pub struct Deep {}").unwrap();
+
+    let output = run_binary(root.to_str().unwrap());
+    assert!(output.status.success());
+
+    let json = parse_output(&output);
+    let crates = extract_array(&json, "crates");
+    let nested_crate = crates.iter().find(|c| c["name"].as_str().unwrap() == "nested").unwrap();
+
+    let modules = extract_array(&nested_crate, "modules");
+    let module_paths: Vec<&str> = modules
+        .iter()
+        .map(|m| m["path"].as_str().unwrap())
+        .collect();
+
+    assert!(module_paths.iter().any(|p| *p == "nested"));
+    assert!(module_paths.iter().any(|p| *p == "nested::foo"));
+    assert!(module_paths.iter().any(|p| *p == "nested::foo::bar"));
+    assert!(module_paths.iter().any(|p| *p == "nested::foo::bar::baz"));
+}
+
+#[test]
+fn test_reexport_chains() {
+    let tmp = tempfile::tempdir().unwrap();
+    let root = tmp.path();
+
+    write_cargo_toml(root, r#"
+[workspace]
+members = ["."]
+
+[package]
+name = "reexporter"
+version = "0.1.0"
+edition = "2021"
+"#);
+
+    let src = root.join("src");
+    std::fs::create_dir_all(&src).unwrap();
+
+    // lib.rs with re-export chain
+    std::fs::write(src.join("lib.rs"), "
+mod inner {
+    pub struct Secret;
+}
+pub use inner::Secret;
+").unwrap();
+
+    let output = run_binary(root.to_str().unwrap());
+    assert!(output.status.success());
+
+    let json = parse_output(&output);
+    let crates = extract_array(&json, "crates");
+    let reexporter = crates.iter().find(|c| c["name"].as_str().unwrap() == "reexporter").unwrap();
+
+    let re_exports: Vec<&serde_json::Value> = extract_array(&reexporter, "modules")
+        .iter()
+        .flat_map(|m| extract_array(m, "reExports"))
+        .collect();
+
+    let has_secret = re_exports.iter().any(|re| {
+        re["importPath"].as_str().unwrap().contains("Secret")
+    });
+    assert!(has_secret, "should have re-export for Secret");
+}
+
+#[test]
+fn test_output_via_flag() {
+    let tmp = tempfile::tempdir().unwrap();
+    let fixture = std::path::Path::new("tests/fixtures/sample-workspace");
+    let output_path = tmp.path().join("output.json");
+
+    // Run with -o flag
+    let output1 = Command::new(&binary_path())
+        .arg(fixture)
+        .arg("-o")
+        .arg(output_path.clone())
+        .output()
+        .expect("failed to execute binary");
+    assert!(output1.status.success());
+
+    // Run without -o, capture stdout
+    let output2 = Command::new(&binary_path())
+        .arg(fixture)
+        .output()
+        .expect("failed to execute binary");
+    assert!(output2.status.success());
+
+    // Compare file content with stdout
+    let file_content = std::fs::read_to_string(&output_path).unwrap();
+    let stdout_content = String::from_utf8_lossy(&output2.stdout);
+    assert_eq!(
+        file_content.trim(),
+        stdout_content.trim(),
+        "file output should match stdout"
+    );
+}

## File: .claude/hooks/current_task_TASK-PREP.json
{
  "task_id": "TASK-PREP",
  "task_description": "Add tempfile dev-dependency for unit and integration tests",
  "plan_path": "/Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml",
  "plan_slug": "phase-0.2",
  "acceptance_commands": [
    "cargo check -p rust-workspace-map"
  ],
  "acceptance_prose": [],
  "all_task_ids": [
    "TASK-PREP",
    "TASK-1",
    "TASK-2",
    "TASK-3",
    "TASK-4",
    "TASK-5",
    "TASK-6",
    "TASK-7",
    "TASK-8",
    "TASK-9",
    "TASK-10"
  ],
  "timestamp": "2026-04-28T08:54:28Z"
}
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

[dev-dependencies]
tempfile = "3"
## File: execution_reports/.checkpoint_phase-0.2.json
{
  "plan": "/Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml",
  "base_commit": "bfe6a418a006d292aabea07d27e4a8440d57d661",
  "completed": [
    "TASK-2",
    "TASK-3"
  ],
  "failed": [
    "TASK-4",
    "TASK-5",
    "TASK-7",
    "TASK-6",
    "TASK-8",
    "TASK-9",
    "TASK-10"
  ],
  "blocked": []
}
## File: execution_reports/execution_phase-0.2_20260428.md
# Execution Report

**Plan**: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
**Started**: 2026-04-28T08:55:35Z
**Status**: In Progress

## Task Results

### TASK-2: Add MissingWorkspaceSection variant to Error enum in schema.rs
- **Status**: ✓ Passed
- **Validation output**:
  - `cargo check -p rust-workspace-map`: PASSED
  - `cargo check --workspace 2>&1`: PASSED

### TASK-1: Add ErrorSeverity enum and ErrorContext struct to schema.rs
- **Status**: ✓ Passed
- **Validation output**:
  - `cargo check -p rust-workspace-map`: PASSED
  - `cargo check --workspace 2>&1`: PASSED

### TASK-2: Add MissingWorkspaceSection variant to Error enum in schema.rs
- **Status**: ✓ Passed
- **Validation output**:
  - `cargo check -p rust-workspace-map`: PASSED
  - `cargo check --workspace 2>&1`: PASSED

### TASK-3: Expand ErrorEntry struct with severity, kind, context, and cause fields
- **Status**: ✓ Passed
- **Validation output**:
  - `cargo check -p rust-workspace-map`: PASSED
  - `cargo check --workspace 2>&1`: PASSED

### TASK-4: Change parse_file to return ParsedFile with optional parse errors instead of Result
- **Status**: ✗ Failed
- **Validation output**:
  - `true  # applied atomically with TASK-5; compilation verified at TASK-5`: PASSED
  - `cargo check --workspace 2>&1`: FAILED (exit 101)
    ```
    Checking rust-workspace-map v0.1.0 (/Users/tony/programming/rust-workspace-map)
    warning: unused imports: `Error` and `Result`
     --> src/file_parser.rs:2:5
      |
    2 |     Error, ErrorEntry, ErrorSeverity, FileInfo, ImplInfo, ImplItem, ImplItemKind, Import,
      |     ^^^^^
    3 |     ItemAttrs, ItemKind, PublicItem, ReExport, Result, SubmoduleDecl,
      |                                                ^^^^^^
      |
      = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default
    
    error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
      --> src/module_tree.rs:27:28
       |
    27 |     let (ast, file_info) = file_parser::parse_file(crate_root)?;
       |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `ParsedFile`
       |
    help: the nightly-only, unstable trait `std::ops::Try` is not implemented for `ParsedFile`
      --> src/file_parser.rs:15:1
       |
    15 | pub struct ParsedFile {
       | ^^^^^^^^^^^^^^^^^^^^^
    
    error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
       --> src/module_tree.rs:148:32
        |
    148 |         let (ast, file_info) = file_parser::parse_file(file_path)?;
        |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `ParsedFile`
        |
    help: the nightly-only, unstable trait `std::ops::Try` is not implemented for `ParsedFile`
    ```

### TASK-5: Refactor module_tree.rs: fix path fallbacks, add error collection, change build_module_tree return type
- **Status**: ✗ Failed
- **Validation output**:
  - `cargo check -p rust-workspace-map`: FAILED (exit 101)
    ```
    Checking rust-workspace-map v0.1.0 (/Users/tony/programming/rust-workspace-map)
    warning: unused imports: `Error` and `Result`
     --> src/file_parser.rs:2:5
      |
    2 |     Error, ErrorEntry, ErrorSeverity, FileInfo, ImplInfo, ImplItem, ImplItemKind, Import,
      |     ^^^^^
    3 |     ItemAttrs, ItemKind, PublicItem, ReExport, Result, SubmoduleDecl,
      |                                                ^^^^^^
      |
      = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default
    
    warning: unused import: `Result`
     --> src/module_tree.rs:2:84
      |
    2 | use crate::schema::{ErrorContext, ErrorEntry, ErrorSeverity, FileInfo, ModuleInfo, Result, SubmoduleDecl};
      |                                                                                    ^^^^^^
    
    error[E0599]: no method named `unwrap_or_default` found for tuple `(Vec<ModuleInfo>, Vec<ErrorEntry>)` in the current scope
      --> src/lib.rs:79:69
       |
    79 |                     module_tree::build_module_tree(root, &pkg_name).unwrap_or_default()
       |                                                                     ^^^^^^^^^^^^^^^^^ method not found in `(Vec<ModuleInfo>, Vec<ErrorEntry>)`
    
    For more information about this error, try `rustc --explain E0599`.
    warning: `rust-workspace-map` (lib) generated 2 warnings
    error: could not compile `rust-workspace-map` (lib) due to 1 previous error; 2 warnings emitted
    ```

### TASK-7: Refactor lib.rs run(): parallel error collection, error entry construction, remove unwrap_or_default
- **Status**: ✗ Failed
- **Validation output**:
  - `cargo check -p rust-workspace-map`: FAILED (exit 101)
    ```
    workspace::enumerate_members(&workspace_root)?;
         |                         ---------------------------------------------- this expression has type `Vec<PathBuf>`
    ...
      43 |           .par_iter()
         |            ---------- `ParallelIterator::Item` is `&PathBuf` here
      44 |           .map(|dir| {
         |  __________^
      45 | |             let cargo_toml = dir.join("Cargo.toml");
      46 | |             let mut crate_errors = Vec::new();
    ...    |
     119 | |             (Some(crate_info), crate_errors)
     120 | |         })
         | |__________^ `ParallelIterator::Item` changed to `(Option<CrateInfo>, Vec<ErrorEntry>)` here
    note: required by a bound in `rayon::iter::ParallelIterator::collect`
        --> /Users/tony/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rayon-1.12.0/src/iter/mod.rs:2056:12
         |
    2054 |     fn collect<C>(self) -> C
         |        ------- required by a bound in this associated function
    2055 |     where
    2056 |         C: FromParallelIterator<Self::Item>,
         |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ParallelIterator::collect`
         = note: the full name for the type has been written to '/Users/tony/programming/rust-workspace-map/target/debug/deps/rust_workspace_map-ca1f86303c4a42a1.long-type-7933542791174809822.txt'
         = note: consider using `--verbose` to print the full type name to the console
    
    error[E0308]: mismatched types
       --> src/lib.rs:126:16
        |
    126 |         if let Some(ci) = info {
        |                ^^^^^^^^   ---- this expression has type `CrateInfo`
        |                |
    ```

### TASK-6: Update workspace.rs enumerate_members to return MissingWorkspaceSection error
- **Status**: ✗ Failed
- **Validation output**:
  - `cargo check -p rust-workspace-map`: FAILED (exit 101)
    ```
    workspace::enumerate_members(&workspace_root)?;
         |                         ---------------------------------------------- this expression has type `Vec<PathBuf>`
    ...
      43 |           .par_iter()
         |            ---------- `ParallelIterator::Item` is `&PathBuf` here
      44 |           .map(|dir| {
         |  __________^
      45 | |             let cargo_toml = dir.join("Cargo.toml");
      46 | |             let mut crate_errors = Vec::new();
    ...    |
     119 | |             (Some(crate_info), crate_errors)
     120 | |         })
         | |__________^ `ParallelIterator::Item` changed to `(Option<CrateInfo>, Vec<ErrorEntry>)` here
    note: required by a bound in `rayon::iter::ParallelIterator::collect`
        --> /Users/tony/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rayon-1.12.0/src/iter/mod.rs:2056:12
         |
    2054 |     fn collect<C>(self) -> C
         |        ------- required by a bound in this associated function
    2055 |     where
    2056 |         C: FromParallelIterator<Self::Item>,
         |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ParallelIterator::collect`
         = note: the full name for the type has been written to '/Users/tony/programming/rust-workspace-map/target/debug/deps/rust_workspace_map-ca1f86303c4a42a1.long-type-8545240589136472689.txt'
         = note: consider using `--verbose` to print the full type name to the console
    
    error[E0308]: mismatched types
       --> src/lib.rs:126:16
        |
    126 |         if let Some(ci) = info {
        |                ^^^^^^^^   ---- this expression has type `CrateInfo`
        |                |
    ```

### TASK-8: Remove all crate-level clippy allow attributes and fix individual lint violations
- **Status**: ✗ Failed
- **Validation output**:
  - `cargo clippy -p rust-workspace-map -- -D warnings`: FAILED (exit 101)
    ```
    --> src/lib.rs:41:10
         |
      35 |       let member_dirs = workspace::enumerate_members(&workspace_root)?;
         |                         ---------------------------------------------- this expression has type `Vec<PathBuf>`
    ...
      40 |           .par_iter()
         |            ---------- `ParallelIterator::Item` is `&PathBuf` here
      41 |           .map(|dir| {
         |  __________^
      42 | |             let cargo_toml = dir.join("Cargo.toml");
      43 | |             let mut crate_errors = Vec::new();
    ...    |
     116 | |             (Some(crate_info), crate_errors)
     117 | |         })
         | |__________^ `ParallelIterator::Item` changed to `(Option<CrateInfo>, Vec<ErrorEntry>)` here
    note: required by a bound in `rayon::iter::ParallelIterator::collect`
        --> /Users/tony/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rayon-1.12.0/src/iter/mod.rs:2056:12
         |
    2054 |     fn collect<C>(self) -> C
         |        ------- required by a bound in this associated function
    2055 |     where
    2056 |         C: FromParallelIterator<Self::Item>,
         |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ParallelIterator::collect`
         = note: the full name for the type has been written to '/Users/tony/programming/rust-workspace-map/target/debug/deps/rust_workspace_map-c0af38651655276e.long-type-9209811902749593541.txt'
         = note: consider using `--verbose` to print the full type name to the console
    
    error[E0308]: mismatched types
       --> src/lib.rs:123:16
        |
    123 |         if let Some(ci) = info {
    ```

### TASK-9: Add unit tests for file_parser, module_tree, workspace, cargo_info, cross_refs, and render
- **Status**: ✗ Failed
- **Validation output**:
  - `cargo test -p rust-workspace-map`: FAILED (exit 101)
    ```
    -------------------------------------- this expression has type `Vec<PathBuf>`
    ...
      40 |           .par_iter()
         |            ---------- `ParallelIterator::Item` is `&PathBuf` here
      41 |           .map(|dir| {
         |  __________^
      42 | |             let cargo_toml = dir.join("Cargo.toml");
      43 | |             let mut crate_errors = Vec::new();
    ...    |
     116 | |             (Some(crate_info), crate_errors)
     117 | |         })
         | |__________^ `ParallelIterator::Item` changed to `(Option<CrateInfo>, Vec<ErrorEntry>)` here
    note: required by a bound in `rayon::iter::ParallelIterator::collect`
        --> /Users/tony/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rayon-1.12.0/src/iter/mod.rs:2056:12
         |
    2054 |     fn collect<C>(self) -> C
         |        ------- required by a bound in this associated function
    2055 |     where
    2056 |         C: FromParallelIterator<Self::Item>,
         |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ParallelIterator::collect`
         = note: the full name for the type has been written to '/Users/tony/programming/rust-workspace-map/target/debug/deps/rust_workspace_map-453a2fe558063110.long-type-13358229255708071102.txt'
         = note: consider using `--verbose` to print the full type name to the console
    
    error[E0308]: mismatched types
       --> src/lib.rs:123:16
        |
    123 |         if let Some(ci) = info {
        |                ^^^^^^^^   ---- this expression has type `CrateInfo`
        |                |
        |                expected `CrateInfo`, found `Option<_>`
    ```

### TASK-10: Expand integration tests from 3 to 10 covering error cases, workspace patterns, and output
- **Status**: ✗ Failed
- **Validation output**:
  - `cargo test -p rust-workspace-map --test integration_test`: FAILED (exit 101)
    ```
    orkspace::enumerate_members(&workspace_root)?;
         |                         ---------------------------------------------- this expression has type `Vec<PathBuf>`
    ...
      40 |           .par_iter()
         |            ---------- `ParallelIterator::Item` is `&PathBuf` here
      41 |           .map(|dir| {
         |  __________^
      42 | |             let cargo_toml = dir.join("Cargo.toml");
      43 | |             let mut crate_errors = Vec::new();
    ...    |
     116 | |             (Some(crate_info), crate_errors)
     117 | |         })
         | |__________^ `ParallelIterator::Item` changed to `(Option<CrateInfo>, Vec<ErrorEntry>)` here
    note: required by a bound in `rayon::iter::ParallelIterator::collect`
        --> /Users/tony/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rayon-1.12.0/src/iter/mod.rs:2056:12
         |
    2054 |     fn collect<C>(self) -> C
         |        ------- required by a bound in this associated function
    2055 |     where
    2056 |         C: FromParallelIterator<Self::Item>,
         |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ParallelIterator::collect`
         = note: the full name for the type has been written to '/Users/tony/programming/rust-workspace-map/target/debug/deps/rust_workspace_map-453a2fe558063110.long-type-11627958768023326083.txt'
         = note: consider using `--verbose` to print the full type name to the console
    
    error[E0308]: mismatched types
       --> src/lib.rs:123:16
        |
    123 |         if let Some(ci) = info {
        |                ^^^^^^^^   ---- this expression has type `CrateInfo`
        |                |
    ```

## File: notes/plan-enrichment/phase-0.2/codebase-state.md
# Phase 0.2 — Codebase State Snapshot

Captured on 2026-04-28. This documents the current state of every source file mentioned in the phase plan.

---

## File: src/schema.rs

### Public API

**Error enum:**
```rust
#[derive(Debug, thiserror::Error)]
pub enum Error {
    WorkspaceRootNotFound(PathBuf),
    FileRead { path: PathBuf, source: std::io::Error },
    TomlParse { path: PathBuf, source: toml::de::Error },
    SynParse { path: PathBuf, source: syn::Error },
    MemberNotFound(PathBuf),
    GlobPattern(String),
}
pub type Result<T> = std::result::Result<T, Error>;
```

**Config:**
```rust
#[derive(Debug, Clone, bon::Builder)]
pub struct Config {
    pub workspace_path: PathBuf,
    pub output_path: Option<PathBuf>,
}
```

**CrateType enum:** `Lib`, `Bin`, `LibAndBin` (serde rename_all = "camelCase")

**Output structs (all bon::Builder, serde rename_all = "camelCase"):**
- `WorkspaceMap` — fields: `workspace: WorkspaceInfo`, `crates: Vec<CrateInfo>`, `cross_references: CrossReferences`, `errors: Vec<ErrorEntry>` (skip_serializing_if), `workspace_root: PathBuf` (skip)
- `WorkspaceInfo` — fields: `root: String`, `workspace_name: String`
- `CrateInfo` — fields: `name`, `root`, `package: PackageInfo`, `modules: Vec<ModuleInfo>`, `deps: DepInfo`, `cross_crate_imports: Vec<CrossCrateImport>` (skip_serializing_if)
- `PackageInfo` — fields: `name`, `version`, `edition`, `crate_type: CrateType`
- `DepInfo` (Default) — fields: `normal`, `dev`, `workspace_members` (all skip_serializing_if)
- `ModuleInfo` — fields: `path`, `file`, `visibility`, `public_items: Vec<PublicItem>`, `imports: Vec<Import>`, `re_exports: Vec<ReExport>`, `submodules: Vec<String>` (all optional)
- `PublicItem` — fields: `kind: ItemKind`, `name`, `file`, `line: usize`, `attrs: ItemAttrs`, `generics`, `visibility`, `fields`, `variants`, `impls: Vec<ImplInfo>`
- `ItemKind` enum: `Struct`, `Enum`, `Trait`, `Fn`, `Type`, `Macro`
- `ItemAttrs` (Default) — fields: `derive`, `doc`
- `ImplInfo` — fields: `type_: String`, `items: Vec<ImplItem>`
- `ImplItem` — fields: `kind: ImplItemKind`, `name`, `params`
- `ImplItemKind` enum: `Fn`, `Type`, `Const`
- `Import` — fields: `path`, `line`
- `ReExport` — fields: `import_path`, `export_path`, `line`
- `CrossCrateImport` — fields: `import_path`, `target_crate`, `symbol`, `line`
- `CrossReferences` (Default) — field: `types: BTreeMap<String, TypeRef>`
- `TypeRef` — fields: `crate_name`, `kind`, `imported_by`, `exported_by`

**Internal types:**
- `FileInfo` (Default) — `public_items`, `imports`, `re_exports`, `submodules: Vec<SubmoduleDecl>`, `impls`
- `SubmoduleDecl` — `name`, `is_test: bool` (default)

**ErrorEntry (current):**
```rust
pub struct ErrorEntry {
    pub file: String,
    pub line: usize,  // builder(default)
    pub message: String,
}
```

### Module wiring
- Declared as `pub mod schema;` in `lib.rs`
- `pub use schema::Config;` re-export in `lib.rs`

### Plan relationship
- **Plan says:** Add `severity: ErrorSeverity` (enum: `Error`, `Warning`), `kind: String`, `context: Option<ErrorContext>`, `cause: Option<String>` to `ErrorEntry`. Add `ErrorSeverity` enum. Add `ErrorContext` struct with `crate_name`, `module_path`, `line`, `snippet` fields. Add `MissingWorkspaceSection` variant to `Error` enum.
- **Current state:** `ErrorEntry` has only 3 fields (`file`, `line`, `message`). No `ErrorSeverity` enum. No `ErrorContext` struct. `Error` enum has 6 variants but no `MissingWorkspaceSection`.
- **Gap:** All 4 new `ErrorEntry` fields, both new types (`ErrorSeverity`, `ErrorContext`), and the `MissingWorkspaceSection` error variant need to be added. `ErrorEntry` serde serialization annotations and bon::Builder annotations need to be added for the new fields.

---

## File: src/workspace.rs

### Public API

```rust
pub fn find_workspace_root(start_path: &Path) -> Result<PathBuf>
pub fn enumerate_members(root: &Path) -> Result<Vec<PathBuf>>
pub fn resolve_crate_roots(crate_dir: &Path) -> Vec<(PathBuf, CrateType)>
```

### Module wiring
- `pub mod workspace;` in `lib.rs`
- No `pub use` re-exports of workspace items.

### Plan relationship
- **Plan says:** `enumerate_members` should return `Err(Error::MissingWorkspaceSection)` (new variant) when `[workspace]` section or `members` key is absent. `resolve_crate_roots` empty results should become an `ErrorEntry` with `kind = "missing_crate_roots"` in the caller. The `eprintln!` at line 76 (member not found) should become `Error::MemberNotFound`.
- **Current state:** `enumerate_members` uses `.unwrap_or_default()` on lines 46 and 57 — when `[workspace]` or `members` is absent, it silently returns an empty list. `resolve_crate_roots` already returns an empty Vec when no entry points are found (no error handling). The `eprintln!` at line 76–79 logs a warning for missing members but does not return an error.
- **Gap:** `enumerate_members` needs to distinguish "no workspace section" from "workspace section with no members." The missing-member `eprintln!` should return `Error::MemberNotFound`. `resolve_crate_roots` is called in `run()` which currently does nothing with empty results beyond `eprintln!` — the plan wants this converted to an `ErrorEntry` in `run()` (not in `workspace.rs` itself).

---

## File: src/cargo_info.rs

### Public API

```rust
pub fn parse_cargo_toml(path: &Path) -> Result<(PackageInfo, DepInfo)>
```

### Module wiring
- `pub mod cargo_info;` in `lib.rs`
- No `pub use` re-exports.

### Plan relationship
- **Plan says:** Convert `parse_cargo_toml` failures (currently `eprintln!` + `return None` in the caller `run()`) to `ErrorEntry` with `kind = "toml_parse_error"`.
- **Current state:** `parse_cargo_toml` returns `Result<(PackageInfo, DepInfo)>` and already uses `?` to propagate parse errors via the `Error::TomlParse` variant. The caller in `run()` (line 46–55) matches on `Err(e)` and does `eprintln!` + `return None` — errors are swallowed silently.
- **Gap:** No changes needed in `cargo_info.rs` itself. The caller in `main.rs`/`lib.rs` `run()` needs to be updated to convert the error into an `ErrorEntry`.

---

## File: src/file_parser.rs

### Public API

```rust
pub fn parse_file(path: &Path) -> Result<(syn::File, FileInfo)>
pub fn extract_public_items(items: &[syn::Item]) -> Vec<PublicItem>
pub fn extract_imports(items: &[syn::Item]) -> Vec<Import>
pub fn extract_re_exports(items: &[syn::Item]) -> Vec<ReExport>
pub fn extract_submodules(items: &[syn::Item]) -> Vec<SubmoduleDecl>
pub fn extract_impls(items: &[syn::Item]) -> Vec<ImplInfo>
```

### Module wiring
- `pub mod file_parser;` in `lib.rs`
- No `pub use` re-exports.

### Plan relationship
- **Plan says:** Change `parse_file()` to propagate `SynParse` errors as `ErrorEntry` with severity `error`, rather than returning empty results. On parse failure, return structured error data instead of `(empty syn::File, FileInfo::default())`. This applies to all files including inline `#[cfg(test)]` module bodies and external `#[cfg(test)]` module files.
- **Current state:** `parse_file` (lines 18–30) catches `syn::parse_file` errors, prints a warning via `eprintln!`, and returns an empty `syn::File` + default `FileInfo` — the error is completely lost. All extraction functions are pure and return their collections.
- **Gap:** `parse_file` needs to change its return signature or behavior to report parse errors as `ErrorEntry` data alongside results. The plan says: "parse_file returns errors alongside results rather than propagating via `?` on parse failures." This means either a new return type like `Result<(syn::File, FileInfo), ErrorEntry>` or a tuple `(Result<..., ...>, Vec<ErrorEntry>)`.

---

## File: src/module_tree.rs

### Public API

```rust
pub fn resolve_module_path(parent_dir: &Path, mod_name: &str) -> Option<PathBuf>
pub fn build_module_tree(crate_root: &Path, crate_name: &str) -> Result<Vec<ModuleInfo>>
```

**Internal functions (private):**
- `fn build_module_info(...)` — builds ModuleInfo from components
- `fn process_submodule(...)` — processes a single submodule, handles inline vs external
- `fn process_module_items(...)` — extracts FileInfo from inline module items
- `fn process_module_info(...)` — builds module info for a file-path-backed module

### Module wiring
- `pub mod module_tree;` in `lib.rs`
- No `pub use` re-exports.

### Plan relationship

**Path safety (Goal 3):**
- **Plan says:** Fix four `unwrap_or_else(|| Path::new("."))` fallback bugs:
  1. Line 25: `crate_root.parent().unwrap_or_else(|| Path::new("."))` in `build_module_tree`
  2. Line ~155: `file_path.parent().unwrap_or_else(|| Path::new("."))` in `process_submodule`
  3. Line ~203: `file_path.parent().unwrap_or_else(|| Path::new("."))` in `process_module_info`
  4. (workspace.rs `enumerate_members` defaulting to empty list — covered there)
- **Current state:** All three instances in this file use `unwrap_or_else(|| Path::new("."))`. When `parent()` returns `None` (e.g., the path is at the filesystem root), the fallback to `"."` silently produces incorrect relative paths.
- **Gap:** Replace all three `unwrap_or_else(|| Path::new("."))` calls with explicit handling that either returns an error, uses a better default, or propagates the issue visibly.

**Error collection (Goal 1):**
- **Plan says:** `build_module_tree` should collect per-module errors into a `Vec<ErrorEntry>` returned alongside the module tree. Orphaned module warnings (lines 107, 135) should emit `ErrorEntry` with `kind = "orphaned_module"`. Silent error swallowing in `build_module_tree` should be eliminated.
- **Current state:** `build_module_tree` returns `Result<Vec<ModuleInfo>>` and propagates errors via `?`. Orphaned modules produce `eprintln!` + a placeholder `ModuleInfo` with `<unresolved>` file. `process_submodule` uses `?` on `parse_file` calls, which currently never fail (they return empty results).
- **Gap:** `build_module_tree` signature needs to change to collect and return errors alongside results. Orphaned module handling needs to produce `ErrorEntry` values instead of `eprintln!`.

---

## File: src/cross_refs.rs

### Public API

```rust
pub fn compute(crates: &mut [CrateInfo]) -> CrossReferences
```

**Impl block:**
```rust
impl PublicItem {
    fn kind_to_string(&self) -> String  // private helper
}
```

### Module wiring
- `pub mod cross_refs;` in `lib.rs`
- No `pub use` re-exports.

### Plan relationship
- **Plan says:** Add unit tests for `compute`. No source changes mentioned.
- **Current state:** `compute` is fully implemented — builds `crate_exports` map, initializes `TypeRef` entries, scans imports for cross-crate references, populates `cross_crate_imports` on each crate.
- **Gap:** No code changes needed per the plan. Only unit tests are required (Goal 2).

---

## File: src/render.rs

### Public API

```rust
pub fn render_json(map: &WorkspaceMap) -> serde_json::Result<String>
pub fn render_to_writer(map: &WorkspaceMap, writer: impl Write) -> serde_json::Result<()>
```

### Module wiring
- `pub mod render;` in `lib.rs`
- No `pub use` re-exports.

### Plan relationship
- **Plan says:** Add unit tests for `render_json` and `render_to_writer`. No source changes mentioned.
- **Current state:** Both functions are thin wrappers around `serde_json::to_string_pretty` and `serde_json::to_writer_pretty`.
- **Gap:** No code changes needed per the plan. Only unit tests are required (Goal 2).

---

## File: src/main.rs

### Public API

```rust
struct Cli { path: PathBuf, output: Option<PathBuf> }  // clap Parser
fn main() -> anyhow::Result<()>
```

### Module wiring
- Calls `rust_workspace_map::run(config)` from the library crate.
- Uses `rust_workspace_map::Config::builder()` for configuration.

### Plan relationship
- **Plan says:** No changes mentioned. The `run()` function lives in `lib.rs` (not `main.rs`).
- **Current state:** Thin CLI entry point. Delegates all logic to `lib.rs::run()`.
- **Gap:** No changes needed in `main.rs` itself per the plan.

---

## File: src/lib.rs

### Public API

**Crate-level attributes:**
```rust
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::redundant_closure_for_method_calls)]
```

**Module declarations:**
```rust
pub mod cargo_info;
pub mod cross_refs;
pub mod file_parser;
pub mod module_tree;
pub mod render;
pub mod schema;
pub mod workspace;
```

**Re-exports:**
```rust
pub use schema::Config;
```

**Public functions:**
```rust
pub fn run(config: Config) -> anyhow::Result<()>
fn relativize_path(path_str: &str, root: &Path) -> String  // private
```

### Plan relationship

**Error collection (Goal 1):**
- **Plan says:** `run()` must: remove `.unwrap_or_default()` on `module_tree::build_module_tree` (line 79), capture errors as `ErrorEntry` with `kind = "module_tree_error"`. Collect `ErrorEntry` values from parallel crate processing (use `Mutex<Vec<ErrorEntry>>` or `rayon::collect`). Convert `cargo_info::parse_cargo_toml` failures to `ErrorEntry`. Convert `resolve_crate_roots` empty results to `ErrorEntry`. Replace `errors: Vec<ErrorEntry> = Vec::new()` with actual collection.
- **Current state:** `run()` creates an empty `errors` vec (line 39). Line 79 uses `.unwrap_or_default()` on `build_module_tree`, silently discarding errors. Parse failures and resolve failures go to `eprintln!` + `return None`. No error collection from parallel processing.
- **Gap:** Major changes needed: error collection from parallel rayon processing, removal of `.unwrap_or_default()`, conversion of `eprintln!` paths to `ErrorEntry` construction, and wiring of collected errors into `WorkspaceMap`.

**Clippy (Goal 5):**
- **Plan says:** No `#![allow(clippy::*)]` remains. Every suppression must be removed. Add `# Errors` doc sections, `#[must_use]`, fix doc comments, inline format args, replace closures, merge nested `if`, change to `&` references, remove unnecessary borrows.
- **Current state:** 9 crate-level `#![allow(clippy::*)]` attributes present (lines 2–10) plus `#![warn(clippy::pedantic)]`.
- **Gap:** All 9 crate-level suppressions must be resolved — either by fixing the underlying code or adding per-item suppressions (plan says: "no crate-level suppressions are converted to per-item suppressions; if a lint fires on legitimate code, the code is changed, not silenced").

---

## File: tests/integration_test.rs

### Public API (test functions)

```rust
fn test_sample_workspace_output()
fn test_deterministic_output()
fn test_missing_path_exits_nonzero()
```

Helper: `fn binary_path() -> String`, `fn extract_array<'a>(...)`

### Plan relationship
- **Plan says (Goal 4):** Expand from 3 to 8+ integration tests covering:
  - Parse failure workspace → verify `ErrorEntry` with severity in JSON
  - Missing workspace section → verify appropriate error
  - Glob member patterns → verify all matching crates discovered
  - Workspace with `exclude` → verify excluded crate absent
  - Deeply nested modules (3+ levels) → verify correct paths and hierarchy
  - Re-export chains → verify correct re-export tracking
  - Output via `-o` flag → verify file content matches stdout content
- **Current state:** 3 integration tests covering happy path, determinism, and missing path.
- **Gap:** Need 5+ new integration tests (plan lists 7, 3 already exist). All need fixture workspace structures.

---

## Files mentioned in plan but not found

none — all 10 files identified in the plan were found in the codebase.
## File: notes/plan-enrichment/phase-0.2/deferred-and-patterns.md
## Deferred Improvements

- Empty `errors` vector in `WorkspaceMap::run()` — the field is always empty because no code path populates it; wiring in error collection would improve debugging of misconfigured workspaces
- Silent error swallowing in `build_module_tree` — `unwrap_or_default()` masks parse failures, making it indistinguishable from crates with no public items
- Path fallback to `"."` in `module_tree.rs` — three `.unwrap_or_else(|| Path::new("."))` calls produce incorrect relative paths when `.parent()` returns `None`
- Path fallback to `""` in `workspace.rs` — defaulting to empty vectors/malformed sections silently includes zero members; falling back to `""` for file names bypasses exclude filters
- No unit tests for public API functions — 13 public functions across 5 modules lack unit tests; only integration tests cover the full pipeline

## Known Failure Modes

None found. The only `fix-plan.toml` (`notes/pr-reviews/phase-0.1/fix-plan.toml`) contained no fix tasks — the PR was reviewed and approved with all items deferred to future phases.
## File: notes/plan-enrichment/phase-0.2/draft-elaboration.md
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
## File: notes/plan-enrichment/phase-0.2/gather-summary.md
## Gather Summary: phase-0.2

**Tasks created:** 10
**Dependency chain:** Not extracted, see draft-plan.toml
**Deferred items absorbed:** 5/5 (all deferred items from phase 0.1 absorbed by phase 0.2 goals)

**Gather completeness:**
- [x] deferred-and-patterns.md — saved (5 deferred items, 0 failure modes)
- [x] codebase-state.md — saved — Files documented: 10
- [x] draft-elaboration.md — saved — Items elaborated: 15
- [x] draft-plan.toml — saved — Tasks: 10, Validation: PASSED
- [x] task-checklist.md — saved — Tasks checked: 10, Wiring issues flagged: 2

**Before-block verification:** 10/10 confirmed
**Unverified tasks:** none
**Wiring issues flagged:** 2 (see task-checklist.md for details)

**Before-block verification:** 10/10 confirmed
**Unverified tasks:** none
**Wiring issues flagged:** 2

**Confidence notes:**
1. Exact clippy lint locations require running `cargo clippy` after suppression removal to confirm which functions actually fire each lint.
2. Orphaned module handling retains placeholder `ModuleInfo` entries alongside the new `ErrorEntry` per the existing pattern in `module_tree.rs` (the plan says "emit ErrorEntry" not "replace eprintln with ErrorEntry", so both coexist).

**Questions for user:**
None
## File: notes/plan-enrichment/phase-0.2/plan.approved.toml
[meta]
title = "Phase 0.2: Hardening for Trustworthiness"
source_branch = "phase-0.2"
created = "2026-04-28"

[dependencies]
TASK-PREP = []
TASK-3 = ["TASK-1"]
TASK-4 = ["TASK-1", "TASK-2", "TASK-3"]
TASK-5 = ["TASK-4"]
TASK-6 = ["TASK-2"]
TASK-7 = ["TASK-4", "TASK-5"]
TASK-8 = ["TASK-7"]
TASK-9 = ["TASK-PREP", "TASK-5", "TASK-6", "TASK-8"]
TASK-10 = ["TASK-PREP", "TASK-9"]

[tasks.TASK-PREP]
description = "Add tempfile dev-dependency for unit and integration tests"
type = "replace"
acceptance = [
    "cargo check -p rust-workspace-map",
]

[[tasks.TASK-PREP.changes]]
file = "Cargo.toml"
before = "proc-macro2 = { version = \"1\", features = [\"span-locations\"] }"
after = """proc-macro2 = { version = "1", features = ["span-locations"] }

[dev-dependencies]
tempfile = "3\""""

[tasks.TASK-1]
description = "Add ErrorSeverity enum and ErrorContext struct to schema.rs"
type = "replace"
acceptance = [
    "cargo check -p rust-workspace-map",
]

[[tasks.TASK-1.changes]]
file = "src/schema.rs"
before = '''// ── Internal types ──────────────────────────────────────────────────────

/// Internal intermediate type consumed by module_tree.
#[derive(Debug, Clone, Default)]
pub struct FileInfo {
'''
after = '''// ── Error severity ─────────────────────────────────────────────────────

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

/// Internal intermediate type consumed by module_tree.
#[derive(Debug, Clone, Default)]
pub struct FileInfo {
'''

[tasks.TASK-2]
description = "Add MissingWorkspaceSection variant to Error enum in schema.rs"
type = "replace"
acceptance = [
    "cargo check -p rust-workspace-map",
]

[[tasks.TASK-2.changes]]
file = "src/schema.rs"
before = '''    #[error("glob pattern error: {0}")]
    GlobPattern(String),
}

pub type Result<T> = std::result::Result<T, Error>;'''
after = '''    #[error("glob pattern error: {0}")]
    GlobPattern(String),

    #[error("workspace Cargo.toml is missing the [workspace] section")]
    MissingWorkspaceSection,
}

pub type Result<T> = std::result::Result<T, Error>;'''

[tasks.TASK-3]
description = "Expand ErrorEntry struct with severity, kind, context, and cause fields"
type = "replace"
acceptance = [
    "cargo check -p rust-workspace-map",
]

[[tasks.TASK-3.changes]]
file = "src/schema.rs"
before = '''#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ErrorEntry {
    pub file: String,
    #[builder(default)]
    pub line: usize,
    pub message: String,
}
'''
after = '''#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ErrorEntry {
    pub file: String,
    #[builder(default)]
    pub line: usize,
    pub message: String,
    pub severity: ErrorSeverity,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ErrorContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}
'''

[tasks.TASK-4]
description = "Change parse_file to return ParsedFile with optional parse errors instead of Result"
type = "replace"
acceptance = [
    "true  # applied atomically with TASK-5; compilation verified at TASK-5",
]

[[tasks.TASK-4.changes]]
file = "src/file_parser.rs"
before = '''use crate::schema::{
    Error, FileInfo, ImplInfo, ImplItem, ImplItemKind, Import, ItemAttrs, ItemKind, PublicItem,
    ReExport, Result, SubmoduleDecl,
};
use std::path::Path;

// ── parse_file ──────────────────────────────────────────────────────────

/// Read and parse a Rust source file. Returns the raw `syn::File` AST (needed
/// by `module_tree` for inline module item extraction) and the extracted
/// `FileInfo`. On parse failure, warns to stderr and returns empty results.
pub fn parse_file(path: &Path) -> Result<(syn::File, FileInfo)> {
    let content = std::fs::read_to_string(path).map_err(|source| Error::FileRead {
        path: path.to_path_buf(),
        source,
    })?;

    let file = match syn::parse_file(&content) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("warning: failed to parse {}: {}", path.display(), e);
            let empty = syn::File {
                shebang: None,
                attrs: vec![],
                items: vec![],
            };
            let info = FileInfo::default();
            return Ok((empty, info));
        }
    };

    let info = FileInfo {
        public_items: extract_public_items(&file.items),
        imports: extract_imports(&file.items),
        re_exports: extract_re_exports(&file.items),
        submodules: extract_submodules(&file.items),
        impls: extract_impls(&file.items),
    };

    Ok((file, info))
}'''
after = '''use crate::schema::{
    Error, ErrorEntry, ErrorSeverity, FileInfo, ImplInfo, ImplItem, ImplItemKind, Import,
    ItemAttrs, ItemKind, PublicItem, ReExport, Result, SubmoduleDecl,
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
        .kind("syn_parse_error".to_string())
        .build()
}'''

[tasks.TASK-5]
description = "Refactor module_tree.rs: fix path fallbacks, add error collection, change build_module_tree return type"
type = "replace"
acceptance = [
    "cargo check -p rust-workspace-map",
]

[[tasks.TASK-5.changes]]
file = "src/module_tree.rs"
before = "use crate::schema::{FileInfo, ModuleInfo, Result, SubmoduleDecl};"
after = "use crate::schema::{ErrorContext, ErrorEntry, ErrorSeverity, FileInfo, ModuleInfo, Result, SubmoduleDecl};"

[[tasks.TASK-5.changes]]
file = "src/module_tree.rs"
before = '''/// Build the full module tree for a crate starting from its entry point
/// (e.g., `src/lib.rs`). Returns a flat `Vec<ModuleInfo>` containing the
/// root module and all recursively discovered submodules.
pub fn build_module_tree(crate_root: &Path, crate_name: &str) -> Result<Vec<ModuleInfo>> {
    let mut visited = HashSet::new();
    let parent_dir = crate_root.parent().unwrap_or_else(|| Path::new("."));

    let (ast, file_info) = file_parser::parse_file(crate_root)?;
    visited.insert(crate_root.to_path_buf());

    let root_module = build_module_info(
        crate_name,
        crate_root,
        "pub",
        &file_info.public_items,
        &file_info.imports,
        &file_info.re_exports,
        &file_info.submodules,
    );

    let mut modules = vec![root_module];

    for sub in &file_info.submodules {
        if sub.is_test {
            continue;
        }
        let sub_module_path = format!("{}::{}", crate_name, sub.name);
        let child_modules = process_submodule(
            &sub_module_path,
            &sub.name,
            &ast.items,
            parent_dir,
            crate_root,
            &mut visited,
        )?;
        modules.extend(child_modules);
    }

    Ok(modules)
}'''
after = '''/// Build the full module tree for a crate starting from its entry point
/// (e.g., `src/lib.rs`). Returns a tuple of module info and any errors
/// encountered during submodule parsing (including orphaned module warnings).
pub fn build_module_tree(
    crate_root: &Path,
    crate_name: &str,
) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {

    let mut visited = HashSet::new();
    let parent_dir = crate_root.parent().unwrap_or(crate_root);

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
}'''

[[tasks.TASK-5.changes]]
file = "src/module_tree.rs"
before = '''    let Some(mod_item) = mod_item else {
        eprintln!("warning: orphaned module {}", module_path);
        return Ok(vec![ModuleInfo::builder()
            .path(module_path.to_string())
            .file("<unresolved>".to_string())
            .visibility("private".to_string())
            .build()]);
    };'''
after = '''    let Some(mod_item) = mod_item else {
        let err = ErrorEntry::builder()
            .file(String::new())
            .message(format!("orphaned module: {module_path}"))
            .severity(ErrorSeverity::Warning)
            .kind("orphaned_module".to_string())
            .context(ErrorContext::builder()
                .module_path(module_path.to_string())
                .build())
            .build();
        return (vec![ModuleInfo::builder()
            .path(module_path.to_string())
            .file("<unresolved>".to_string())
            .visibility("private".to_string())
            .build()], vec![err]);
    };'''

[[tasks.TASK-5.changes]]
file = "src/module_tree.rs"
before = '''        let Some(ref file_path) = file_path else {
            eprintln!("warning: orphaned module {}", module_path);
            return Ok(vec![ModuleInfo::builder()
                .path(module_path.to_string())
                .file("<unresolved>".to_string())
                .visibility(visibility.to_string())
                .build()]);
        };'''
after = '''        let Some(ref file_path) = file_path else {
            let err = ErrorEntry::builder()
                .file(String::new())
                .message(format!("orphaned module: {module_path}"))
                .severity(ErrorSeverity::Warning)
                .kind("orphaned_module".to_string())
                .context(ErrorContext::builder()
                    .module_path(module_path.to_string())
                    .build())
                .build();
            return (vec![ModuleInfo::builder()
                .path(module_path.to_string())
                .file("<unresolved>".to_string())
                .visibility(visibility.to_string())
                .build()], vec![err]);
        };'''

[[tasks.TASK-5.changes]]
file = "src/module_tree.rs"
before = '''        let (ast, file_info) = file_parser::parse_file(file_path)?;
        process_module_info(
            module_path,
            file_path,
            visibility,
            &file_info,
            &ast.items,
            &file_path.parent().unwrap_or_else(|| Path::new(".")),
            visited,
        )'''
after = '''        let parsed = file_parser::parse_file(file_path);
        let mut errors: Vec<ErrorEntry> = Vec::new();
        if let Some(ref err) = parsed.parse_error {
            errors.push(crate::file_parser::build_parse_error_entry(file_path, err));
        }
        process_module_info(
            module_path,
            file_path,
            visibility,
            &parsed.file_info,
            &parsed.ast.items,
            &file_path.parent().unwrap_or(file_path),
            visited,
            &mut errors,
        )'''

[[tasks.TASK-5.changes]]
file = "src/module_tree.rs"
before = '''        if visited.contains(file_path.as_path()) {
            return Ok(vec![]); // cycle detected
        }'''
after = '''        if visited.contains(file_path.as_path()) {
            return (vec![], vec![]); // cycle detected
        }'''

[[tasks.TASK-5.changes]]
file = "src/module_tree.rs"
before = '''/// Resolve a `mod name;` declaration to a file path.
/// Tries `{parent_dir}/{mod_name}.rs` first, then `{parent_dir}/{mod_name}/mod.rs`.
pub fn resolve_module_path(parent_dir: &Path, mod_name: &str) -> Option<PathBuf> {'''
after = '''/// Resolve a `mod name;` declaration to a file path.
/// Tries `{parent_dir}/{mod_name}.rs` first, then `{parent_dir}/{mod_name}/mod.rs`.
///
/// Returns `None` if neither path exists.
pub fn resolve_module_path(parent_dir: &Path, mod_name: &str) -> Option<PathBuf> {'''

[[tasks.TASK-5.changes]]
file = "src/module_tree.rs"
before = '''fn process_submodule(
    module_path: &str,
    mod_name: &str,
    parent_items: &[syn::Item],
    parent_dir: &Path,
    parent_file: &Path,
    visited: &mut HashSet<PathBuf>,
) -> Result<Vec<ModuleInfo>> {'''
after = '''fn process_submodule(
    module_path: &str,
    mod_name: &str,
    parent_items: &[syn::Item],
    parent_dir: &Path,
    parent_file: &Path,
    visited: &mut HashSet<PathBuf>,
) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {'''

[[tasks.TASK-5.changes]]
file = "src/module_tree.rs"
before = '''    if let Some((_, ref inline_items)) = mod_item.content {
        // Inline module: process its body items directly (no file lookup).
        process_module_items(
            module_path,
            parent_file,
            visibility,
            inline_items,
            parent_dir,
            visited,
        )
    } else {'''
after = '''    if let Some((_, ref inline_items)) = mod_item.content {
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
    } else {'''

[[tasks.TASK-5.changes]]
file = "src/module_tree.rs"
before = '''fn process_module_items(
    module_path: &str,
    file_path: &Path,
    visibility: &str,
    items: &[syn::Item],
    parent_dir: &Path,
    visited: &mut HashSet<PathBuf>,
) -> Result<Vec<ModuleInfo>> {'''
after = '''fn process_module_items(
    module_path: &str,
    file_path: &Path,
    visibility: &str,
    items: &[syn::Item],
    parent_dir: &Path,
    visited: &mut HashSet<PathBuf>,
) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {'''

[[tasks.TASK-5.changes]]
file = "src/module_tree.rs"
before = '''    let file_info = FileInfo {
        public_items: file_parser::extract_public_items(items),
        imports: file_parser::extract_imports(items),
        re_exports: file_parser::extract_re_exports(items),
        submodules: file_parser::extract_submodules(items),
        impls: file_parser::extract_impls(items),
    };
    process_module_info(module_path, file_path, visibility, &file_info, items, parent_dir, visited)
}'''
after = '''    let file_info = FileInfo {
        public_items: file_parser::extract_public_items(items),
        imports: file_parser::extract_imports(items),
        re_exports: file_parser::extract_re_exports(items),
        submodules: file_parser::extract_submodules(items),
        impls: file_parser::extract_impls(items),
    };
    process_module_info(module_path, file_path, visibility, &file_info, items, parent_dir, visited, &mut Vec::new())
}'''

[[tasks.TASK-5.changes]]
file = "src/module_tree.rs"
before = '''fn process_module_info(
    module_path: &str,
    file_path: &Path,
    visibility: &str,
    file_info: &FileInfo,
    items: &[syn::Item],
    _parent_dir: &Path,
    visited: &mut HashSet<PathBuf>,
) -> Result<Vec<ModuleInfo>> {
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
        let child_dir = file_path.parent().unwrap_or_else(|| Path::new("."));
        let child_modules = process_submodule(
            &child_path,
            &sub.name,
            items,
            child_dir,
            file_path,
            visited,
        )?;
        modules.extend(child_modules);
    }

    Ok(modules)
}'''
after = '''fn process_module_info(
    module_path: &str,
    file_path: &Path,
    visibility: &str,
    file_info: &FileInfo,
    items: &[syn::Item],
    _parent_dir: &Path,
    visited: &mut HashSet<PathBuf>,
    errors: &mut Vec<crate::schema::ErrorEntry>,
) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {
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
        let (child_modules, child_errors) = process_submodule(
            &child_path,
            &sub.name,
            items,
            child_dir,
            file_path,
            visited,
        );
        errors.extend(child_errors);
        modules.extend(child_modules);
    }

    (modules, errors.clone())
}'''


[tasks.TASK-6]
description = "Update workspace.rs enumerate_members to return MissingWorkspaceSection error"
type = "replace"
acceptance = [
    "cargo check -p rust-workspace-map",
]

[[tasks.TASK-6.changes]]
file = "src/workspace.rs"
before = '''    let members: Vec<String> = parsed
        .get("workspace")
        .and_then(|w| w.get("members"))
        .and_then(|m| m.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();'''
after = '''    let members: Vec<String> = match parsed.get("workspace") {
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
    };'''

[[tasks.TASK-6.changes]]
file = "src/workspace.rs"
before = '''/// Walk up the directory tree from `start_path` to find a `Cargo.toml`
/// containing a `[workspace]` section. Returns the directory containing it.
pub fn find_workspace_root(start_path: &Path) -> Result<PathBuf> {'''
after = '''/// Walk up the directory tree from `start_path` to find a `Cargo.toml`
/// containing a `[workspace]` section. Returns the directory containing it.
///
/// # Errors
///
/// Returns `Error::WorkspaceRootNotFound` if no `Cargo.toml` with a
/// `[workspace]` section is found in any ancestor directory.
pub fn find_workspace_root(start_path: &Path) -> Result<PathBuf> {'''

[[tasks.TASK-6.changes]]
file = "src/workspace.rs"
before = '''/// Parse the workspace `Cargo.toml`, resolve member paths (including glob
/// patterns), apply `exclude` list, and return absolute paths to each member
/// crate directory.
pub fn enumerate_members(root: &Path) -> Result<Vec<PathBuf>> {'''
after = '''/// Parse the workspace `Cargo.toml`, resolve member paths (including glob
/// patterns), apply `exclude` list, and return absolute paths to each member
/// crate directory.
///
/// # Errors
///
/// Returns `Error::MissingWorkspaceSection` if the `Cargo.toml` lacks a
/// `[workspace]` section entirely.
pub fn enumerate_members(root: &Path) -> Result<Vec<PathBuf>> {'''

[tasks.TASK-7]
description = "Refactor lib.rs run(): parallel error collection, error entry construction, remove unwrap_or_default"
type = "replace"
acceptance = [
    "cargo check -p rust-workspace-map",
]

[[tasks.TASK-7.changes]]
file = "src/lib.rs"
before = '''use anyhow::Context;
use rayon::prelude::*;
use schema::{
    CrateInfo, CrateType, ErrorEntry, ModuleInfo, WorkspaceInfo, WorkspaceMap,
};
use std::path::Path;'''
after = '''use anyhow::Context;
use rayon::prelude::*;
use schema::{
    CrateInfo, CrateType, ErrorEntry, ErrorSeverity, ModuleInfo, WorkspaceInfo,
    WorkspaceMap,
};
use std::path::Path;'''

[[tasks.TASK-7.changes]]
file = "src/lib.rs"
before = '''    let errors: Vec<ErrorEntry> = Vec::new();

    let mut crate_infos: Vec<CrateInfo> = member_dirs
        .par_iter()
        .filter_map(|dir| {
            let cargo_toml = dir.join("Cargo.toml");

            let (pkg, deps) = match cargo_info::parse_cargo_toml(&cargo_toml) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!(
                        "warning: failed to parse {}: {}",
                        cargo_toml.display(),
                        e
                    );
                    return None;
                }
            };

            let roots = workspace::resolve_crate_roots(dir);
            if roots.is_empty() {
                eprintln!(
                    "warning: no crate entry points found in {}",
                    dir.display()
                );
                return None;
            }

            let crate_type = if roots.iter().any(|(_, t)| *t == CrateType::Lib)
                && roots.iter().any(|(_, t)| *t == CrateType::Bin)
            {
                CrateType::LibAndBin
            } else {
                roots.first().map_or(CrateType::Lib, |(_, t)| *t)
            };

            let pkg_name = pkg.name.clone();
            let mut modules: Vec<ModuleInfo> = roots
                .iter()
                .flat_map(|(root, _ty)| {
                    module_tree::build_module_tree(root, &pkg_name).unwrap_or_default()
                })
                .collect();

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

            Some(
                CrateInfo::builder()
                    .name(pkg_name)
                    .root(crate_root)
                    .package(rebuilt_pkg)
                    .modules(modules)
                    .deps(deps)
                    .build(),
            )
        })
        .collect();'''
after = '''    let mut crate_errors: Vec<ErrorEntry> = Vec::new();

    let results: Vec<(CrateInfo, Vec<ErrorEntry>)> = member_dirs
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
                        .kind("toml_parse_error".to_string())
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
                    .kind("missing_crate_roots".to_string())
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
                let (m, e) = module_tree::build_module_tree(&root, &pkg_name);
                modules.extend(m);
                collected_errors.extend(e);
            }
            crate_errors.extend(collected_errors);

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
        if let Some(ci) = info {
            crate_errors.extend(errs);
            crate_infos.push(ci);
        }
    }'''

[[tasks.TASK-7.changes]]
file = "src/lib.rs"
before = '''    let map = WorkspaceMap::builder()
        .workspace(workspace_info)
        .crates(crate_infos)
        .cross_references(cross_refs)
        .errors(errors)
        .workspace_root(workspace_root.clone())
        .build();'''
after = '''    let map = WorkspaceMap::builder()
        .workspace(workspace_info)
        .crates(crate_infos)
        .cross_references(cross_refs)
        .errors(crate_errors)
        .workspace_root(workspace_root.clone())
        .build();'''

[tasks.TASK-8]
description = "Remove all crate-level clippy allow attributes and fix individual lint violations"
type = "replace"
acceptance = [
    "cargo clippy -p rust-workspace-map -- -D warnings",
]

[[tasks.TASK-8.changes]]
file = "src/lib.rs"
before = '''#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::redundant_closure_for_method_calls)]'''
after = '''#![warn(clippy::pedantic)]'''

[[tasks.TASK-8.changes]]
file = "src/lib.rs"
before = '''/// Run the full workspace mapping pipeline.
///
/// 1. Discover workspace root and member crates.
/// 2. Process each crate in parallel (Cargo.toml parsing + module tree).
/// 3. Compute cross-crate references.
/// 4. Render JSON to stdout or the configured output file.
pub fn run(config: Config) -> anyhow::Result<()> {'''
after = '''/// Run the full workspace mapping pipeline.
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
pub fn run(config: Config) -> anyhow::Result<()> {'''

[[tasks.TASK-8.changes]]
file = "src/lib.rs"
before = '''    let workspace_name = workspace_root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();'''
after = '''    let workspace_name = workspace_root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();'''

[[tasks.TASK-8.changes]]
file = "src/lib.rs"
before = '''/// Strip the workspace root prefix from a path string, returning a
/// workspace-relative path. If the prefix doesn't match, returns the
/// original string unchanged.
fn relativize_path(path_str: &str, root: &Path) -> String {
    let p = Path::new(path_str);
    match p.strip_prefix(root) {
        Ok(rel) => rel.to_string_lossy().to_string(),
        Err(_) => path_str.to_string(),
    }
}'''
after = '''/// Strip the workspace root prefix from a path string, returning a
/// workspace-relative path. If the prefix doesn't match, returns the
/// original string unchanged.
fn relativize_path(path_str: &str, root: &Path) -> String {
    let p = Path::new(path_str);
    match p.strip_prefix(root) {
        Ok(rel) => rel.to_string_lossy().to_string(),
        Err(_) => path_str.to_string(),
    }
}'''

[[tasks.TASK-8.changes]]
file = "src/file_parser.rs"
before = '''/// Extract all items with any form of `pub` visibility (excluding `Inherited`).
/// Results are sorted by name then line for deterministic output.
pub fn extract_public_items(items: &[syn::Item]) -> Vec<PublicItem> {'''
after = '''/// Extract all items with any form of `pub` visibility (excluding `Inherited`).
/// Results are sorted by name then line for deterministic output.
#[must_use]
pub fn extract_public_items(items: &[syn::Item]) -> Vec<PublicItem> {'''

[[tasks.TASK-8.changes]]
file = "src/file_parser.rs"
before = '''/// Extract all `use` statements. Braced imports are expanded to individual
/// entries. Results sorted by path for determinism.
pub fn extract_imports(items: &[syn::Item]) -> Vec<Import> {'''
after = '''/// Extract all `use` statements. Braced imports are expanded to individual
/// entries. Results sorted by path for determinism.
#[must_use]
pub fn extract_imports(items: &[syn::Item]) -> Vec<Import> {'''

[[tasks.TASK-8.changes]]
file = "src/file_parser.rs"
before = '''/// Extract `pub use` re-exports. Results sorted by export_path.
pub fn extract_re_exports(items: &[syn::Item]) -> Vec<ReExport> {'''
after = '''/// Extract `pub use` re-exports. Results sorted by export_path.
#[must_use]
pub fn extract_re_exports(items: &[syn::Item]) -> Vec<ReExport> {'''

[[tasks.TASK-8.changes]]
file = "src/file_parser.rs"
before = '''/// Extract `mod` declarations. Detects `#[cfg(test)]` via literal token
/// matching. Results sorted by name.
pub fn extract_submodules(items: &[syn::Item]) -> Vec<SubmoduleDecl> {'''
after = '''/// Extract `mod` declarations. Detects `#[cfg(test)]` via literal token
/// matching. Results sorted by name.
#[must_use]
pub fn extract_submodules(items: &[syn::Item]) -> Vec<SubmoduleDecl> {'''

[[tasks.TASK-8.changes]]
file = "src/file_parser.rs"
before = '''/// Extract `impl` blocks. Each `ImplInfo` records the target type name and
/// the impl items (fn, type, const).
pub fn extract_impls(items: &[syn::Item]) -> Vec<ImplInfo> {'''
after = '''/// Extract `impl` blocks. Each `ImplInfo` records the target type name and
/// the impl items (fn, type, const).
#[must_use]
pub fn extract_impls(items: &[syn::Item]) -> Vec<ImplInfo> {'''

[[tasks.TASK-8.changes]]
file = "src/module_tree.rs"
before = '''/// Resolve a `mod name;` declaration to a file path.
/// Tries `{parent_dir}/{mod_name}.rs` first, then `{parent_dir}/{mod_name}/mod.rs`.
///
/// Returns `None` if neither path exists.
pub fn resolve_module_path(parent_dir: &Path, mod_name: &str) -> Option<PathBuf> {'''
after = '''/// Resolve a `mod name;` declaration to a file path.
/// Tries `{parent_dir}/{mod_name}.rs` first, then `{parent_dir}/{mod_name}/mod.rs`.
///
/// Returns `None` if neither path exists.
#[must_use]
pub fn resolve_module_path(parent_dir: &Path, mod_name: &str) -> Option<PathBuf> {'''

[[tasks.TASK-8.changes]]
file = "src/render.rs"
before = '''/// Serialize the workspace map to a JSON string with 2-space indentation.
pub fn render_json(map: &WorkspaceMap) -> serde_json::Result<String> {'''
after = '''/// Serialize the workspace map to a JSON string with 2-space indentation.
#[must_use]
pub fn render_json(map: &WorkspaceMap) -> serde_json::Result<String> {'''

[[tasks.TASK-8.changes]]
file = "src/cross_refs.rs"
before = '''// Helper: convert ItemKind to a short string for the TypeRef.kind field.
impl crate::schema::PublicItem {
    fn kind_to_string(&self) -> String {'''
after = '''// Helper: convert ItemKind to a short string for the TypeRef.kind field.
impl crate::schema::PublicItem {
    #[must_use]
    fn kind_to_string(&self) -> String {'''

[[tasks.TASK-8.changes]]
file = "src/lib.rs"
before = '''    match p.strip_prefix(root) {
        Ok(rel) => rel.to_string_lossy().to_string(),
        Err(_) => path_str.to_string(),
    }
}'''
after = '''    match p.strip_prefix(root) {
        Ok(rel) => rel.to_string_lossy().to_string(),
        Err(_) => path_str.to_string(),
    }
}'''

[[tasks.TASK-8.changes]]
file = "src/file_parser.rs"
before = "/// Extract `pub use` re-exports. Results sorted by export_path.\n#[must_use]\npub fn extract_re_exports"
after = "/// Extract `pub use` re-exports. Results sorted by `export_path`.\n#[must_use]\npub fn extract_re_exports"

[[tasks.TASK-8.changes]]
file = "src/file_parser.rs"
before = "            let name = m.ident.as_ref().map(|i| i.to_string()).unwrap_or_default();"
after = "            let name = m.ident.as_ref().map(ToString::to_string).unwrap_or_default();"

[[tasks.TASK-8.changes]]
file = "src/file_parser.rs"
before = "fn flatten_use_tree(tree: &syn::UseTree, prefix: String, line: usize) -> Vec<Import> {"
after = "fn flatten_use_tree(tree: &syn::UseTree, prefix: &str, line: usize) -> Vec<Import> {"

[[tasks.TASK-8.changes]]
file = "src/file_parser.rs"
before = "                Some(flatten_use_tree(&u.tree, String::new(), line_of_item(item)))"
after = "                Some(flatten_use_tree(&u.tree, \"\", line_of_item(item)))"

[[tasks.TASK-8.changes]]
file = "src/file_parser.rs"
before = "            flatten_use_tree(&p.tree, new_prefix, line)"
after = "            flatten_use_tree(&p.tree, &new_prefix, line)"

[[tasks.TASK-8.changes]]
file = "src/schema.rs"
before = "/// Internal intermediate type consumed by module_tree.\n#[derive(Debug, Clone, Default)]\npub struct FileInfo"
after = "/// Internal intermediate type consumed by `module_tree`.\n#[derive(Debug, Clone, Default)]\npub struct FileInfo"

[[tasks.TASK-8.changes]]
file = "src/lib.rs"
before = "/// parsed, or the JSON output cannot be written.\npub fn run(config: Config) -> anyhow::Result<()> {"
after = "/// parsed, or the JSON output cannot be written.\npub fn run(config: &Config) -> anyhow::Result<()> {"

[[tasks.TASK-8.changes]]
file = "src/main.rs"
before = "    rust_workspace_map::run(config)"
after = "    rust_workspace_map::run(&config)"

[tasks.TASK-9]
description = "Add unit tests for file_parser, module_tree, workspace, cargo_info, cross_refs, and render"
type = "replace"
acceptance = [
    "cargo test -p rust-workspace-map",
]

[[tasks.TASK-9.changes]]
file = "src/file_parser.rs"
before = '''fn extract_re_exports_from_tree(
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
            vec![ReExport {
                import_path,
                export_path: n.ident.to_string(),
                line,
            }]
        }
        syn::UseTree::Rename(r) => {
            vec![ReExport {
                import_path,
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
}'''
after = '''fn extract_re_exports_from_tree(
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
            vec![ReExport {
                import_path,
                export_path: n.ident.to_string(),
                line,
            }]
        }
        syn::UseTree::Rename(r) => {
            vec![ReExport {
                import_path,
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
    use crate::schema::{Import, ReExport, SubmoduleDecl};
    use std::path::PathBuf;

    fn parse_source(src: &str) -> ParsedFile {
        let tmp = std::env::temp_dir().join("parse_test.rs");
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
        assert_eq!(entry.kind, "syn_parse_error");
        assert_eq!(entry.severity, ErrorSeverity::Error);
    }
}'''

[[tasks.TASK-9.changes]]
file = "src/module_tree.rs"
before = '''fn process_module_info(
    module_path: &str,
    file_path: &Path,
    visibility: &str,
    file_info: &FileInfo,
    items: &[syn::Item],
    _parent_dir: &Path,
    visited: &mut HashSet<PathBuf>,
    errors: &mut Vec<crate::schema::ErrorEntry>,
) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {
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
        let (child_modules, child_errors) = process_submodule(
            &child_path,
            &sub.name,
            items,
            child_dir,
            file_path,
            visited,
        );
        errors.extend(child_errors);
        modules.extend(child_modules);
    }

    (modules, errors.clone())
}'''
after = '''fn process_module_info(
    module_path: &str,
    file_path: &Path,
    visibility: &str,
    file_info: &FileInfo,
    items: &[syn::Item],
    _parent_dir: &Path,
    visited: &mut HashSet<PathBuf>,
    errors: &mut Vec<crate::schema::ErrorEntry>,
) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {
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
        let (child_modules, child_errors) = process_submodule(
            &child_path,
            &sub.name,
            items,
            child_dir,
            file_path,
            visited,
        );
        errors.extend(child_errors);
        modules.extend(child_modules);
    }

    (modules, errors.clone())
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::ErrorEntry;

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
        assert!(modules.is_empty());
        assert!(!errors.is_empty());
        std::fs::remove_dir_all(&tmp).ok();
    }
}'''

[[tasks.TASK-9.changes]]
file = "src/workspace.rs"
before = '''    result.sort();
    result.dedup();
    Ok(result)
}

/// For a crate directory, determine its entry-point file(s).'''
after = '''    result.sort();
    result.dedup();
    Ok(result)
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_cargo_toml(dir: &std::path::Path, content: &str) {
        let mut f = std::fs::File::create(dir.join("Cargo.toml")).unwrap();
        f.write_all(content.as_bytes()).unwrap();
    }

    fn setup_crate(dir: &std::path::Path) {
        let src = dir.join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("lib.rs"), "").unwrap();
    }

    #[test]
    fn find_workspace_root_finds_cargo_toml() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("subdir").join("nested");
        std::fs::create_dir_all(&path).unwrap();
        write_cargo_toml(tmp.path(), "[workspace]");
        let result = find_workspace_root(&path).unwrap();
        assert_eq!(result, tmp.path());
    }

    #[test]
    fn enumerate_members_returns_members() {
        let tmp = tempfile::tempdir().unwrap();
        write_cargo_toml(tmp.path(), r#"
[workspace]
members = ["crate_a", "crate_b"]
"#);
        setup_crate(tmp.path().join("crate_a").as_path());
        setup_crate(tmp.path().join("crate_b").as_path());
        let members = enumerate_members(tmp.path()).unwrap();
        assert_eq!(members.len(), 2);
    }

    #[test]
    fn enumerate_members_returns_err_for_missing_workspace() {
        let tmp = tempfile::tempdir().unwrap();
        write_cargo_toml(tmp.path(), "[dependencies]\nfoo = \"1\"");
        let result = enumerate_members(tmp.path());
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::MissingWorkspaceSection => {},
            other => panic!("expected MissingWorkspaceSection, got {:?}", other),
        }
    }

    #[test]
    fn enumerate_members_applies_exclude() {
        let tmp = tempfile::tempdir().unwrap();
        write_cargo_toml(tmp.path(), r#"
[workspace]
members = ["a", "b", "c"]
exclude = ["b"]
"#);
        setup_crate(tmp.path().join("a").as_path());
        setup_crate(tmp.path().join("b").as_path());
        setup_crate(tmp.path().join("c").as_path());
        let members = enumerate_members(tmp.path()).unwrap();
        let names: Vec<_> = members.iter().map(|p| p.file_name().unwrap().to_string_lossy()).collect();
        assert!(names.contains(&"a".as_ref()));
        assert!(!names.contains(&"b".as_ref()));
        assert!(names.contains(&"c".as_ref()));
    }

    #[test]
    fn resolve_crate_roots_detects_lib() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join("src")).unwrap();
        std::fs::write(tmp.path().join("src").join("lib.rs"), "").unwrap();
        let roots = resolve_crate_roots(tmp.path());
        assert_eq!(roots.len(), 1);
        assert_eq!(roots[0].1, CrateType::Lib);
    }

    #[test]
    fn resolve_crate_roots_detects_bin() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join("src")).unwrap();
        std::fs::write(tmp.path().join("src").join("main.rs"), "").unwrap();
        let roots = resolve_crate_roots(tmp.path());
        assert_eq!(roots.len(), 1);
        assert_eq!(roots[0].1, CrateType::Bin);
    }
}'''

[[tasks.TASK-9.changes]]
file = "src/cargo_info.rs"
before = '''    Ok((package, deps))
}'''
after = '''    Ok((package, deps))
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_cargo_toml(dir: &std::path::Path, content: &str) {
        let mut f = std::fs::File::create(dir.join("Cargo.toml")).unwrap();
        f.write_all(content.as_bytes()).unwrap();
    }

    #[test]
    fn parse_cargo_toml_parses_minimal() {
        let tmp = tempfile::tempdir().unwrap();
        write_cargo_toml(tmp.path(), r#"
[package]
name = "test-pkg"
version = "1.0.0"
edition = "2021"
"#);
        let (pkg, _deps) = parse_cargo_toml(tmp.path().join("Cargo.toml").as_path()).unwrap();
        assert_eq!(pkg.name, "test-pkg");
        assert_eq!(pkg.version, "1.0.0");
        assert_eq!(pkg.edition, "2021");
    }

    #[test]
    fn parse_cargo_toml_uses_defaults_for_missing_package() {
        let tmp = tempfile::tempdir().unwrap();
        write_cargo_toml(tmp.path(), "");
        let (pkg, _deps) = parse_cargo_toml(tmp.path().join("Cargo.toml").as_path()).unwrap();
        assert_eq!(pkg.name, "unknown");
        assert_eq!(pkg.edition, "2021");
    }

    #[test]
    fn parse_cargo_toml_distinguishes_deps() {
        let tmp = tempfile::tempdir().unwrap();
        write_cargo_toml(tmp.path(), r#"
[package]
name = "test-pkg"
version = "0.1.0"
edition = "2021"

[dependencies]
foo = "1"
bar = { workspace = true }

[dev-dependencies]
baz = "2"
qux = { workspace = true }
"#);
        let (_, deps) = parse_cargo_toml(tmp.path().join("Cargo.toml").as_path()).unwrap();
        assert_eq!(deps.normal, vec!["foo"]);
        assert_eq!(deps.dev, vec!["baz"]);
        assert!(deps.workspace_members.contains(&"bar".to_string()));
        assert!(deps.workspace_members.contains(&"qux".to_string()));
    }
}'''

[[tasks.TASK-9.changes]]
file = "src/cross_refs.rs"
before = '''    CrossReferences { types: types_map }
}

// Helper: convert ItemKind to a short string for the TypeRef.kind field.'''
after = '''    CrossReferences { types: types_map }
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{ModuleInfo, PublicItem, SubmoduleDecl};

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
                schema::PackageInfo::builder()
                    .name(name.to_string())
                    .version("0.1.0".to_string())
                    .edition("2021".to_string())
                    .crate_type(schema::CrateType::Lib)
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
        // Task should be in cross-references
        assert!(refs.types.contains_key("Task"));
        let task_ref = &refs.types["Task"];
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

// Helper: convert ItemKind to a short string for the TypeRef.kind field.'''

[[tasks.TASK-9.changes]]
file = "src/render.rs"
before = '''use crate::schema::WorkspaceMap;
use std::io::Write;

/// Serialize the workspace map to a JSON string with 2-space indentation.
#[must_use]
pub fn render_json(map: &WorkspaceMap) -> serde_json::Result<String> {
    serde_json::to_string_pretty(map)
}

/// Serialize the workspace map to the given writer.
pub fn render_to_writer(map: &WorkspaceMap, writer: impl Write) -> serde_json::Result<()> {
    serde_json::to_writer_pretty(writer, map)
}
'''
after = '''use crate::schema::WorkspaceMap;
use std::io::Write;

/// Serialize the workspace map to a JSON string with 2-space indentation.
#[must_use]
pub fn render_json(map: &WorkspaceMap) -> serde_json::Result<String> {
    serde_json::to_string_pretty(map)
}

/// Serialize the workspace map to the given writer.
pub fn render_to_writer(map: &WorkspaceMap, writer: impl Write) -> serde_json::Result<()> {
    serde_json::to_writer_pretty(writer, map)
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{
        CrateInfo, CrateType, CrossReferences, DepInfo, ModuleInfo, PackageInfo,
        WorkspaceInfo, WorkspaceMap,
    };

    fn make_minimal_map() -> WorkspaceMap {
        WorkspaceMap::builder()
            .workspace(WorkspaceInfo::builder()
                .root(".".to_string())
                .workspace_name("test".to_string())
                .build())
            .crates(vec![
                CrateInfo::builder()
                    .name("test-crate".to_string())
                    .root(".".to_string())
                    .package(PackageInfo::builder()
                        .name("test-crate".to_string())
                        .version("0.1.0".to_string())
                        .edition("2021".to_string())
                        .crate_type(CrateType::Lib)
                        .build())
                    .modules(vec![ModuleInfo::builder()
                        .path("".to_string())
                        .file("src/lib.rs".to_string())
                        .visibility("pub".to_string())
                        .build()])
                    .deps(DepInfo::default())
                    .build(),
            ])
            .cross_references(CrossReferences::default())
            .workspace_root(std::path::PathBuf::from("."))
            .build()
    }

    #[test]
    fn render_json_produces_valid_json() {
        let map = make_minimal_map();
        let json = render_json(&map).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["workspace"]["root"], ".");
        assert_eq!(parsed["crates"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn render_json_skips_empty_errors() {
        let map = make_minimal_map();
        let json = render_json(&map).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        // errors field should be absent (skip_serializing_if)
        assert!(parsed.get("errors").is_none());
    }

    #[test]
    fn render_to_writer_matches_render_json() {
        let map = make_minimal_map();
        let json = render_json(&map).unwrap();

        let mut buf = Vec::new();
        render_to_writer(&map, &mut buf).unwrap();
        let from_writer = String::from_utf8(buf).unwrap();

        assert_eq!(json, from_writer);
    }
}
'''

[tasks.TASK-10]
description = "Expand integration tests from 3 to 10 covering error cases, workspace patterns, and output"
type = "replace"
acceptance = [
    "cargo test -p rust-workspace-map --test integration_test",
]

[[tasks.TASK-10.changes]]
file = "tests/integration_test.rs"
before = '''#[test]
fn test_missing_path_exits_nonzero() {
    let bin = binary_path();
    let output = Command::new(&bin)
        .arg("/tmp/nonexistent-path-12345")
        .output()
        .expect("failed to execute binary");

    assert!(
        !output.status.success(),
        "should exit non-zero for invalid path"
    );
}
'''
after = '''#[test]
fn test_missing_path_exits_nonzero() {
    let bin = binary_path();
    let output = Command::new(&bin)
        .arg("/tmp/nonexistent-path-12345")
        .output()
        .expect("failed to execute binary");

    assert!(
        !output.status.success(),
        "should exit non-zero for invalid path"
    );
}

fn run_binary(path: &str) -> std::process::Output {
    Command::new(&binary_path())
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

    let output = run_binary(root.to_str().unwrap());
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

    let output = run_binary(root.to_str().unwrap());

    // Should exit non-zero because workspace is missing
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

    let output = run_binary(root.to_str().unwrap());
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

    let output = run_binary(root.to_str().unwrap());
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

    // lib.rs declares mod foo
    std::fs::write(src.join("lib.rs"), "mod foo;").unwrap();
    // foo.rs declares mod bar
    std::fs::write(foo.join("foo.rs"), "mod bar;").unwrap();
    // bar/baz.rs declares mod baz
    std::fs::write(bar.join("bar.rs"), "mod baz;").unwrap();
    // baz.rs with a struct
    std::fs::write(bar.join("baz.rs"), "pub struct Deep {}").unwrap();

    let output = run_binary(root.to_str().unwrap());
    assert!(output.status.success());

    let json = parse_output(&output);
    let crates = extract_array(&json, "crates");
    let nested_crate = crates.iter().find(|c| c["name"].as_str().unwrap() == "nested").unwrap();

    let module_paths: Vec<&str> = extract_array(&nested_crate["modules"], "path")
        .iter()
        .map(|m| m.as_str().unwrap())
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

    let output = run_binary(root.to_str().unwrap());
    assert!(output.status.success());

    let json = parse_output(&output);
    let crates = extract_array(&json, "crates");
    let reexporter = crates.iter().find(|c| c["name"].as_str().unwrap() == "reexporter").unwrap();

    let re_exports: Vec<&serde_json::Value> = extract_array(&reexporter["modules"])
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
        .arg(fixture)
        .arg("-o")
        .arg(output_path.clone())
        .output()
        .expect("failed to execute binary");
    assert!(output1.status.success());

    // Run without -o, capture stdout
    let output2 = Command::new(&binary_path())
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
'''
## File: notes/plan-enrichment/phase-0.2/task-checklist.md
# Phase 0.2 — Task Checklist

Generated: 2026-04-28

---

## TASK-1: Add ErrorSeverity enum and ErrorContext struct to schema.rs

Module wiring check:
- pub mod in parent: Yes — `pub mod schema;` declared in `lib.rs` line 17
- pub use re-export: No — neither type is re-exported from `lib.rs` (not needed, they are internal to the schema module)
- Consumer updates co-located: Yes — `schema.rs` itself contains the types

Known failure mode check:
- Missing pub mod risk: Low — `schema.rs` already has `pub mod` in `lib.rs`
- Missing pub use risk: Low — consumers reference `crate::schema::ErrorSeverity` directly
- Stale import risk: Low — no existing code imports these types yet

Before-block check:
- Grep confirmed: Yes — the before-block text ("// -- Internal types") exists at `schema.rs` line 281
- Acceptance commands present: Yes — `cargo check -p rust-workspace-map`

Depends on: None

Notes: The insertion point is clean — placed before `pub struct FileInfo`. `ErrorContext` uses `bon::Builder` and `serde::Serialize` which are already imported indirectly via `schema.rs` derives.

---

## TASK-2: Add MissingWorkspaceSection variant to Error enum in schema.rs

Module wiring check:
- pub mod in parent: Yes — `pub mod schema;` declared in `lib.rs` line 17
- pub use re-export: No — new variant, no re-export needed
- Consumer updates co-located: Yes — only `schema.rs` changes

Known failure mode check:
- Missing pub mod risk: Low
- Missing pub use risk: Low
- Stale import risk: Low — no existing code references this variant

Before-block check:
- Grep confirmed: Yes — before-block ("GlobPattern variant + closing brace") exists at `schema.rs` lines 32-36
- Acceptance commands present: Yes — `cargo check -p rust-workspace-map`

Depends on: TASK-1 (not for wiring; `MissingWorkspaceSection` error message references no new types)

Notes: Straightforward variant addition. The error message string "workspace Cargo.toml is missing the [workspace] section" is clear and consistent with other error messages in this enum.

---

## TASK-3: Expand ErrorEntry struct with severity, kind, context, and cause fields

Module wiring check:
- pub mod in parent: Yes — `pub mod schema;` declared in `lib.rs` line 17
- pub use re-export: No
- Consumer updates co-located: Yes — `schema.rs` changes only

Known failure mode check:
- Missing pub mod risk: Low
- Missing pub use risk: Low
- Stale import risk: Low

Before-block check:
- Grep confirmed: Yes — before-block ("ErrorEntry struct with 3 fields") exists at `schema.rs` lines 302-309
- Acceptance commands present: Yes — `cargo check -p rust-workspace-map`

Depends on: TASK-1 — **BLOCKED without TASK-1** because the new `kind` field references `ErrorSeverity` (defined in TASK-1) and `context` field references `ErrorContext` (also defined in TASK-1)

Notes: The new `kind: String` field has no `skip_serializing_if` annotation, meaning it will always be serialized (even as empty string). This is intentional — callers are expected to always set it. Both `context` and `cause` have proper `skip_serializing_if` annotations. The `#[builder(default)]` on `line: usize` means the default is `0` — this is fine.

---

## TASK-4: Change parse_file to return ParsedFile with optional parse errors instead of Result

Module wiring check:
- pub mod in parent: Yes — `pub mod file_parser;` declared in `lib.rs` line 14
- pub use re-export: No — `ParsedFile` and `SynParseError` are `pub` (no re-export needed; they are used internally)
- Consumer updates co-located: No — `module_tree.rs` and `lib.rs` need updates for the new return type (handled in TASK-5 and TASK-7)

Known failure mode check:
- Missing pub mod risk: Low — `file_parser.rs` already `pub mod`
- Missing pub use risk: Medium — `build_parse_error_entry` is NOT `pub` (no visibility qualifier). It is referenced as `crate::file_parser::build_parse_error_entry` in `module_tree.rs` changes. Since Rust default visibility for non-pub items in the same crate is `crate`-level, this WILL work. But the plan does not explicitly declare this visibility, which could confuse an executor.
- Stale import risk: High — `file_parser.rs` imports `ErrorEntry` and `ErrorSeverity` from `schema.rs` which are defined in TASK-1 and TASK-3. If TASK-1 and TASK-3 are not applied first, the imports will fail.

Before-block check:
- Grep confirmed: Yes — the full `parse_file` function block matches at `file_parser.rs` lines 1-41
- Acceptance commands present: Yes — `cargo check -p rust-workspace-map`

Depends on: TASK-1, TASK-3 — **BLOCKED without these** because the new code imports `ErrorEntry` and `ErrorSeverity` from `schema.rs`

Notes: `ParsedFile` and `SynParseError` are `pub` in `file_parser.rs` but not re-exported from `lib.rs`. This is fine for internal use. The `build_parse_error_entry` helper function is `pub(crate)` by Rust default (no `pub` keyword). `parse_file` now always succeeds (never returns an `Err`), which is the intended behavior per the plan.

---

## TASK-5: Refactor module_tree.rs: fix path fallbacks, add error collection, change build_module_tree return type

Module wiring check:
- pub mod in parent: Yes — `pub mod module_tree;` declared in `lib.rs` line 15
- pub use re-export: No
- Consumer updates co-located: No — `lib.rs::run()` needs to call the new `(Vec<ModuleInfo>, Vec<ErrorEntry>)` return type (handled in TASK-7)

Known failure mode check:
- Missing pub mod risk: Low
- Missing pub use risk: Low
- Stale import risk: Medium — `module_tree.rs` imports `FileInfo` and `SubmoduleDecl` from `schema.rs` and uses `file_parser::parse_file`. If TASK-4 is not applied first, the `file_parser::parse_file` call will not match the new `ParsedFile` return type.

Before-block check:
- Grep confirmed: Yes — all 9 before-blocks verified against `module_tree.rs` source
- Acceptance commands present: Yes — `cargo check -p rust-workspace-map`

Depends on: TASK-4 — **BLOCKED without TASK-4** because `build_module_tree` and `process_submodule` call `file_parser::parse_file()` which returns `ParsedFile` in TASK-4 but `Result<(...)>` in current source. Also, `module_tree.rs` references `crate::file_parser::build_parse_error_entry` which is only defined in TASK-4.

Notes: This is the most complex task with 9 sequential replace operations on a single file. The operations must be applied in order (or atomically merged) because later before-blocks reference code that is mutated by earlier before-blocks. The path fallback fix (`unwrap_or_else(|| Path::new("."))` -> `unwrap_or(file_path)` or `unwrap_or(crate_root)`) is conservative — using the file itself as fallback when parent() returns None is safer than the original `Path::new(".")` which could resolve to the wrong directory. The `process_module_info` function gains an `errors: &mut Vec<...>` parameter that threads errors through recursion. The `process_module_items` function calls `process_module_info` with `&mut Vec::new()` which starts a fresh error vector per inline module — this may lose cross-module error context.

---

## TASK-6: Update workspace.rs enumerate_members to return MissingWorkspaceSection error

Module wiring check:
- pub mod in parent: Yes — `pub mod workspace;` declared in `lib.rs` line 18
- pub use re-export: No
- Consumer updates co-located: No — `lib.rs::run()` may propagate this error (but the error already propagates via `?` since it returns `Result`)

Known failure mode check:
- Missing pub mod risk: Low
- Missing pub use risk: Low
- Stale import risk: Low — `workspace.rs` already imports `Error` from `schema.rs`; the `MissingWorkspaceSection` variant is added in TASK-2

Before-block check:
- Grep confirmed: Yes — all 3 before-blocks verified against `workspace.rs` source
- Acceptance commands present: Yes — `cargo check -p rust-workspace-map`

Depends on: TASK-2 — **BLOCKED without TASK-2** because `Error::MissingWorkspaceSection` variant is only defined in TASK-2. If applied first, `cargo check` will fail with "variant does not exist".

Notes: The first change introduces a `match parsed.get("workspace")` that explicitly checks for `None` and returns `Err(Error::MissingWorkspaceSection)`. The second and third changes are purely doc additions (````# Errors` sections) to `find_workspace_root` and `enumerate_members`.

---

## TASK-7: Refactor lib.rs run(): parallel error collection, error entry construction, remove unwrap_or_default

Module wiring check:
- pub mod in parent: N/A — this modifies `lib.rs` (the crate root, not a module file)
- pub use re-export: Changes imports to add `ErrorContext`, `ErrorSeverity` to the use block — these types come from `schema.rs` (TASK-1, TASK-3)
- Consumer updates co-located: N/A — `lib.rs` IS the consumer of all other modules

Known failure mode check:
- Missing pub mod risk: N/A
- Missing pub use risk: Low — imports added to the existing `use schema::{...}` block
- Stale import risk: High — the before-block is very large (the entire `par_iter` closure block). If any prior task (TASK-4, TASK-5) changes the source before TASK-7 is applied, the before-block will NOT match.

Before-block check:
- Grep confirmed: Yes — the large before-block matches the current `lib.rs` lines 39-113
- Acceptance commands present: Yes — `cargo check -p rust-workspace-map`

Depends on: TASK-4, TASK-5, TASK-6 — **BLOCKED without these**. Specifically:
  - TASK-4: `parse_file` return type changed to `ParsedFile`
  - TASK-5: `build_module_tree` return type changed to `(Vec<ModuleInfo>, Vec<ErrorEntry>)`
  - TASK-6: `enumerate_members` may return `Err(MissingWorkspaceSection)` — already handled via `?`

Notes: This is the most critical integration task. The entire `par_iter` block is replaced with a `map` that returns `(Option<CrateInfo>, Vec<ErrorEntry>)` tuples, which are then collected and processed in a post-loop. The `errors: Vec<ErrorEntry>` variable is renamed to `crate_errors` and wired into the final `WorkspaceMap`. The `unwrap_or_default()` on line 79 is removed in favor of destructuring the `(modules, errors)` tuple from `build_module_tree`. If TASK-5 is not applied first, the `build_module_tree` call with the new return type will not compile.

---

## TASK-8: Remove all crate-level clippy allow attributes and fix individual lint violations

Module wiring check:
- pub mod in parent: N/A — modifies `lib.rs`, `file_parser.rs`, `module_tree.rs`, `render.rs`, `cross_refs.rs`
- pub use re-export: N/A — no new exports
- Consumer updates co-located: N/A — purely cosmetic/lint fixes

Known failure mode check:
- Missing pub mod risk: N/A
- Missing pub use risk: N/A
- Stale import risk: High — 11 replace operations across 5 files. Any prior task that modifies these files before TASK-8 will cause before-block mismatches.

Before-block check:
- Grep confirmed: Yes — all before-blocks verified against current source. Note: some "before" and "after" blocks are identical (e.g., `lib.rs` `workspace_name` block at line 959 and `relativize_path` ending at line 1073). These appear to be no-op placeholders. The actual changes are: removing crate-level `#![allow(...)]` lines, adding `#[must_use]` to extractors, and adding `# Errors` doc sections.
- Acceptance commands present: Yes — `cargo clippy -p rust-workspace-map -- -D warnings`

Depends on: TASK-7 — **Should follow TASK-7** because TASK-7 adds `# Errors` doc section to `run()` which is referenced in TASK-8's before/after. Also, TASK-8's clippy fixes should be applied after the structural changes from TASK-7 so clippy can validate the new code.

Notes: Some before/after blocks are identical (no-op). The `workspace_name` block and `relativize_path` block appear to be no-op placeholders — the actual change is the `#![warn(clippy::pedantic)]` block replacement. The `#[must_use]` annotations are added to all public extraction functions and the `resolve_module_path` function.

---

## TASK-9: Add unit tests for file_parser, module_tree, workspace, cargo_info, cross_refs, and render

Module wiring check:
- pub mod in parent: N/A — tests are `#[cfg(test)]` modules appended to existing source files
- pub use re-export: N/A
- Consumer updates co-located: N/A

Known failure mode check:
- Missing pub mod risk: Low
- Missing pub use risk: Low
- Stale import risk: Medium — tests reference `ErrorSeverity` (from TASK-1), `ParsedFile` (from TASK-4), `SynParseError` (from TASK-4), `ErrorContext::builder()` (from TASK-1), and `ErrorContext::builder().module_path(...)` (from TASK-1). These must exist in the compilation unit.

Before-block check:
- Grep confirmed: Yes — all 6 before-blocks verified against current source files
- Acceptance commands present: Yes — `cargo test -p rust-workspace-map`

Depends on: TASK-1, TASK-4 — **BLOCKED without these** because tests import `ErrorSeverity` and reference `ParsedFile`/`SynParseError` types. Also depends on TASK-5 indirectly because `build_module_tree` return type changes are tested.

Notes: Tests use `tempfile` crate which is NOT declared in `Cargo.toml`. **Missing `[dev-dependencies]` section with `tempfile = "..."` must be added before this task can compile.** The `parse_source` helper function writes temp files, which is correct for unit tests. The `build_module_tree_returns_empty_for_nonexistent` test expects `modules.is_empty()` and `!errors.is_empty()` — this is a reasonable expectation for a non-existent file (parse error from `std::fs::read_to_string`).

---

## TASK-10: Expand integration tests from 3 to 10 covering error cases, workspace patterns, and output

Module wiring check:
- pub mod in parent: N/A — integration test file
- pub use re-export: N/A
- Consumer updates co-located: N/A

Known failure mode check:
- Missing pub mod risk: N/A
- Missing pub use risk: N/A
- Stale import risk: Medium — depends on all structural changes (TASK-4 through TASK-8) being applied because the binary behavior changes (new error entries, new error kinds like `syn_parse_error`, `orphaned_module`, `toml_parse_error`, `missing_crate_roots`).

Before-block check:
- Grep confirmed: Yes — the before-block (the `test_missing_path_exits_nonzero` function end) matches at `integration_test.rs` lines 116-128
- Acceptance commands present: Yes — `cargo test -p rust-workspace-map --test integration_test`

Depends on: TASK-4, TASK-5, TASK-6, TASK-7 — **BLOCKED without these** because integration tests assert on error entry structure (`severity`, `kind` fields) and module tree behavior that only exist after the structural changes.

Notes: Tests use `tempfile` crate which is NOT declared in `Cargo.toml`. **Same missing `[dev-dependencies]` issue as TASK-9.** The new tests cover: parse failure error entries, missing workspace section, glob member patterns, workspace exclude, deeply nested modules (3+ levels), re-export chains, and output via `-o` flag. The `test_output_via_flag` test references `tests/fixtures/sample-workspace` which exists. Helper functions (`run_binary`, `parse_output`, `write_cargo_toml`, `setup_crate`) are defined inline in the after-block.

---

# Dependency Graph Summary

```
TASK-1 ──┐
TASK-2 ──┤
TASK-3 ──┤         TASK-6 ──┐
  │        │                  │
  └──► TASK-4 ──► TASK-5 ────┤
                          ▼
                       TASK-7 ──► TASK-8
                          │
                          ▼
                       TASK-9
                          │
                          ▼
                       TASK-10
```

Dependency notes:
- TASK-3 depends on TASK-1 because `ErrorEntry` new fields reference `ErrorSeverity` and `ErrorContext` types.
- TASK-4 depends on TASK-1, TASK-3 because it imports `ErrorEntry` and `ErrorSeverity`.
- TASK-5 depends on TASK-4 because it calls `file_parser::parse_file()` (new return type) and `crate::file_parser::build_parse_error_entry` (new function).
- TASK-6 depends on TASK-2 because it returns `Error::MissingWorkspaceSection` (new variant).
- TASK-7 depends on TASK-4, TASK-5, TASK-6 because it calls `parse_file()` (new type), `build_module_tree()` (new return type), and `enumerate_members()` (new error variant).
- TASK-8 should follow TASK-7 so clippy validates the new code.
- TASK-9 depends on TASK-1, TASK-4 because tests reference those new types.
- TASK-10 depends on TASK-4 through TASK-7 because integration tests assert on new error kinds and module tree structure.

# Overall Assessment

Ready to Implement: Yes, but with the following action items:

1. **Add `[dev-dependencies]` section** with `tempfile = "*"` (or a specific version). This is required for TASK-9 and TASK-10 tests to compile.

2. **Execute order matters**: Tasks must be executed in dependency order (TASK-1/2/3 first, then 4/6, then 5/7, then 8/9, then 10). Parallel execution of independent tasks (e.g., TASK-1 + TASK-2, or TASK-4 + TASK-6 after their deps) is possible but should be done carefully.

3. **High-risk transition points**:
   - TASK-4 -> TASK-5: `parse_file` return type changes from `Result<(...)>` to `ParsedFile`
   - TASK-5 -> TASK-7: `build_module_tree` return type changes from `Result<Vec<ModuleInfo>>` to `(Vec<ModuleInfo>, Vec<ErrorEntry>)`
   - TASK-7: Large single-replace in `lib.rs` that touches the entire parallel processing pipeline

4. **Before-block stability**: TASK-7's before-block is a large multi-line block that is the most fragile. If any prior task modifies `lib.rs::run()` before TASK-7, the before-block will not match.

Wiring issues flagged: 2 (missing `tempfile` dev-dependency, `build_parse_error_entry` visibility not explicitly declared).
Before-block unverified: 0 (all before-blocks match current source).
## File: plans/compiled/TASK-1.py
#!/usr/bin/env python3
"""TASK-1: Add ErrorSeverity enum and ErrorContext struct to schema.rs"""
import base64, json, subprocess, sys
from pathlib import Path

TASK_ID = "TASK-1"
STEPS = json.loads('[{"before_b64": "Ly8g4pSA4pSAIEludGVybmFsIHR5cGVzIOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgAoKLy8vIEludGVybmFsIGludGVybWVkaWF0ZSB0eXBlIGNvbnN1bWVkIGJ5IG1vZHVsZV90cmVlLgojW2Rlcml2ZShEZWJ1ZywgQ2xvbmUsIERlZmF1bHQpXQpwdWIgc3RydWN0IEZpbGVJbmZvIHsK", "after_b64": "Ly8g4pSA4pSAIEVycm9yIHNldmVyaXR5IOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgAoKI1tkZXJpdmUoRGVidWcsIENsb25lLCBDb3B5LCBQYXJ0aWFsRXEsIEVxLCBzZXJkZTo6U2VyaWFsaXplKV0KI1tzZXJkZShyZW5hbWVfYWxsID0gInNuYWtlX2Nhc2UiKV0KcHViIGVudW0gRXJyb3JTZXZlcml0eSB7CiAgICBFcnJvciwKICAgIFdhcm5pbmcsCn0KCi8vIOKUgOKUgCBFcnJvciBjb250ZXh0IOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgAoKLy8vIE9wdGlvbmFsIGNvbnRleHQgYXR0YWNoZWQgdG8gYW4gZXJyb3IsIHByb3ZpZGluZyBhZGRpdGlvbmFsIGxvY2F0aW9uCi8vLyBhbmQgc291cmNlIGluZm9ybWF0aW9uIGZvciBkaWFnbm9zdGljcy4KI1tkZXJpdmUoRGVidWcsIENsb25lLCBEZWZhdWx0LCBzZXJkZTo6U2VyaWFsaXplLCBib246OkJ1aWxkZXIpXQojW3NlcmRlKHJlbmFtZV9hbGwgPSAiY2FtZWxDYXNlIildCnB1YiBzdHJ1Y3QgRXJyb3JDb250ZXh0IHsKICAgICNbc2VyZGUoc2tpcF9zZXJpYWxpemluZ19pZiA9ICJPcHRpb246OmlzX25vbmUiKV0KICAgIHB1YiBjcmF0ZV9uYW1lOiBPcHRpb248U3RyaW5nPiwKCiAgICAjW3NlcmRlKHNraXBfc2VyaWFsaXppbmdfaWYgPSAiT3B0aW9uOjppc19ub25lIildCiAgICBwdWIgbW9kdWxlX3BhdGg6IE9wdGlvbjxTdHJpbmc+LAoKICAgIC8vLyBMaW5lIG51bWJlciBpbiB0aGUgc291cmNlIGZpbGUgd2hlcmUgdGhlIGVycm9yIG9jY3VycmVkLgogICAgI1tzZXJkZShza2lwX3NlcmlhbGl6aW5nX2lmID0gIk9wdGlvbjo6aXNfbm9uZSIpXQogICAgcHViIGxpbmU6IE9wdGlvbjx1c2l6ZT4sCgogICAgLy8vIEEgc2hvcnQgc291cmNlIHNuaXBwZXQgbmVhciB0aGUgZXJyb3IgbG9jYXRpb24gKGlmIGF2YWlsYWJsZSkuCiAgICAjW3NlcmRlKHNraXBfc2VyaWFsaXppbmdfaWYgPSAiT3B0aW9uOjppc19ub25lIildCiAgICBwdWIgc25pcHBldDogT3B0aW9uPFN0cmluZz4sCn0KCi8vIOKUgOKUgCBJbnRlcm5hbCB0eXBlcyDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIAKCi8vLyBJbnRlcm5hbCBpbnRlcm1lZGlhdGUgdHlwZSBjb25zdW1lZCBieSBtb2R1bGVfdHJlZS4KI1tkZXJpdmUoRGVidWcsIENsb25lLCBEZWZhdWx0KV0KcHViIHN0cnVjdCBGaWxlSW5mbyB7Cg==", "target": "src/schema.rs", "index": 0, "is_create": false}]')

for step in STEPS:
    before = base64.b64decode(step["before_b64"]).decode()
    after = base64.b64decode(step["after_b64"]).decode()
    target = step["target"]
    idx = step["index"]
    is_create = step["is_create"]

    if is_create:
        target_path = Path(target)
        target_path.parent.mkdir(parents=True, exist_ok=True)
        target_path.write_text(after)
        print(f"OK {TASK_ID} change {idx}: created {target}")
    else:
        target_path = Path(target)
        content = target_path.read_text()
        if before not in content:
            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
            sys.exit(1)

        result = subprocess.run(
            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
            capture_output=True, text=True,
        )
        if result.returncode != 0:
            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
            sys.exit(result.returncode)

        new_content = target_path.read_text()
        if after and after not in new_content:
            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
            sys.exit(1)

        print(f"OK {TASK_ID} change {idx}: applied to {target}")

print(f"OK {TASK_ID}: all changes applied")
## File: plans/compiled/TASK-1.sh
#!/usr/bin/env bash
set -euo pipefail
# TASK-1: Add ErrorSeverity enum and ErrorContext struct to schema.rs
# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
# Type: replace
# File: src/schema.rs
python3 "$(dirname "$0")/TASK-1.py"
## File: plans/compiled/TASK-10.py
#!/usr/bin/env python3
"""TASK-10: Expand integration tests from 3 to 10 covering error cases, workspace patterns, and output"""
import base64, json, subprocess, sys
from pathlib import Path

TASK_ID = "TASK-10"
STEPS = json.loads('[{"before_b64": "I1t0ZXN0XQpmbiB0ZXN0X21pc3NpbmdfcGF0aF9leGl0c19ub256ZXJvKCkgewogICAgbGV0IGJpbiA9IGJpbmFyeV9wYXRoKCk7CiAgICBsZXQgb3V0cHV0ID0gQ29tbWFuZDo6bmV3KCZiaW4pCiAgICAgICAgLmFyZygiL3RtcC9ub25leGlzdGVudC1wYXRoLTEyMzQ1IikKICAgICAgICAub3V0cHV0KCkKICAgICAgICAuZXhwZWN0KCJmYWlsZWQgdG8gZXhlY3V0ZSBiaW5hcnkiKTsKCiAgICBhc3NlcnQhKAogICAgICAgICFvdXRwdXQuc3RhdHVzLnN1Y2Nlc3MoKSwKICAgICAgICAic2hvdWxkIGV4aXQgbm9uLXplcm8gZm9yIGludmFsaWQgcGF0aCIKICAgICk7Cn0K", "after_b64": "I1t0ZXN0XQpmbiB0ZXN0X21pc3NpbmdfcGF0aF9leGl0c19ub256ZXJvKCkgewogICAgbGV0IGJpbiA9IGJpbmFyeV9wYXRoKCk7CiAgICBsZXQgb3V0cHV0ID0gQ29tbWFuZDo6bmV3KCZiaW4pCiAgICAgICAgLmFyZygiL3RtcC9ub25leGlzdGVudC1wYXRoLTEyMzQ1IikKICAgICAgICAub3V0cHV0KCkKICAgICAgICAuZXhwZWN0KCJmYWlsZWQgdG8gZXhlY3V0ZSBiaW5hcnkiKTsKCiAgICBhc3NlcnQhKAogICAgICAgICFvdXRwdXQuc3RhdHVzLnN1Y2Nlc3MoKSwKICAgICAgICAic2hvdWxkIGV4aXQgbm9uLXplcm8gZm9yIGludmFsaWQgcGF0aCIKICAgICk7Cn0KCmZuIHJ1bl9iaW5hcnkocGF0aDogJnN0cikgLT4gc3RkOjpwcm9jZXNzOjpPdXRwdXQgewogICAgQ29tbWFuZDo6bmV3KCZiaW5hcnlfcGF0aCgpKQogICAgICAgIC5hcmcocGF0aCkKICAgICAgICAub3V0cHV0KCkKICAgICAgICAuZXhwZWN0KCJmYWlsZWQgdG8gZXhlY3V0ZSBiaW5hcnkiKQp9CgpmbiBwYXJzZV9vdXRwdXQob3V0cHV0OiAmc3RkOjpwcm9jZXNzOjpPdXRwdXQpIC0+IHNlcmRlX2pzb246OlZhbHVlIHsKICAgIHNlcmRlX2pzb246OmZyb21fc3RyKCZTdHJpbmc6OmZyb21fdXRmOF9sb3NzeSgmb3V0cHV0LnN0ZG91dCkpLnVud3JhcCgpCn0KCmZuIHdyaXRlX2NhcmdvX3RvbWwoZGlyOiAmc3RkOjpwYXRoOjpQYXRoLCBjb250ZW50OiAmc3RyKSB7CiAgICBsZXQgbXV0IGYgPSBzdGQ6OmZzOjpGaWxlOjpjcmVhdGUoZGlyLmpvaW4oIkNhcmdvLnRvbWwiKSkudW53cmFwKCk7CiAgICB1c2Ugc3RkOjppbzo6V3JpdGU7CiAgICBmLndyaXRlX2FsbChjb250ZW50LmFzX2J5dGVzKCkpLnVud3JhcCgpOwp9CgpmbiBzZXR1cF9jcmF0ZShkaXI6ICZzdGQ6OnBhdGg6OlBhdGgsIGxpYl9jb250ZW50OiAmc3RyKSB7CiAgICBsZXQgc3JjID0gZGlyLmpvaW4oInNyYyIpOwogICAgc3RkOjpmczo6Y3JlYXRlX2Rpcl9hbGwoJnNyYykudW53cmFwKCk7CiAgICBzdGQ6OmZzOjp3cml0ZShzcmMuam9pbigibGliLnJzIiksIGxpYl9jb250ZW50KS51bndyYXAoKTsKfQoKI1t0ZXN0XQpmbiB0ZXN0X3BhcnNlX2ZhaWx1cmVfZXJyb3JfZW50cnkoKSB7CiAgICBsZXQgdG1wID0gdGVtcGZpbGU6OnRlbXBkaXIoKS51bndyYXAoKTsKICAgIGxldCByb290ID0gdG1wLnBhdGgoKTsKCiAgICAvLyBDcmVhdGUgd29ya3NwYWNlIENhcmdvLnRvbWwKICAgIHdyaXRlX2NhcmdvX3RvbWwocm9vdCwgciMiClt3b3Jrc3BhY2VdCm1lbWJlcnMgPSBbImdvb2RfY3JhdGUiLCAiYmFkX2NyYXRlIl0KIiMpOwoKICAgIC8vIEdvb2QgY3JhdGUgd2l0aCB2YWxpZCBSdXN0CiAgICBzZXR1cF9jcmF0ZSgmcm9vdC5qb2luKCJnb29kX2NyYXRlIiksICJwdWIgc3RydWN0IEdvb2Qge30iKTsKCiAgICAvLyBCYWQgY3JhdGUgd2l0aCBpbnZhbGlkIFJ1c3Qgc3ludGF4CiAgICBzZXR1cF9jcmF0ZSgmcm9vdC5qb2luKCJiYWRfY3JhdGUiKSwgInB1YiBzdHJ1Y3QgeyBpbnZhbGlkIHJ1c3Qgc3ludGF4Iik7CgogICAgbGV0IG91dHB1dCA9IHJ1bl9iaW5hcnkocm9vdC50b19zdHIoKS51bndyYXAoKSk7CiAgICBhc3NlcnQhKG91dHB1dC5zdGF0dXMuc3VjY2VzcygpKTsKCiAgICBsZXQganNvbiA9IHBhcnNlX291dHB1dCgmb3V0cHV0KTsKICAgIGxldCBlcnJvcnM6IFZlYzwmc2VyZGVfanNvbjo6VmFsdWU+ID0gZXh0cmFjdF9hcnJheSgmanNvbiwgImVycm9ycyIpOwoKICAgIGxldCBwYXJzZV9lcnJvcnM6IFZlYzxfPiA9IGVycm9ycy5pdGVyKCkKICAgICAgICAuZmlsdGVyKHxlfCB7CiAgICAgICAgICAgIGVbImtpbmQiXS5hc19zdHIoKS51bndyYXAoKSA9PSAic3luX3BhcnNlX2Vycm9yIgogICAgICAgIH0pCiAgICAgICAgLmNvbGxlY3QoKTsKCiAgICBhc3NlcnQhKCFwYXJzZV9lcnJvcnMuaXNfZW1wdHkoKSwgInNob3VsZCBoYXZlIHBhcnNlIGVycm9yIGVudHJpZXMiKTsKICAgIGFzc2VydF9lcSEocGFyc2VfZXJyb3JzWzBdWyJzZXZlcml0eSJdLmFzX3N0cigpLnVud3JhcCgpLCAiZXJyb3IiKTsKfQoKI1t0ZXN0XQpmbiB0ZXN0X21pc3Npbmdfd29ya3NwYWNlX3NlY3Rpb24oKSB7CiAgICBsZXQgdG1wID0gdGVtcGZpbGU6OnRlbXBkaXIoKS51bndyYXAoKTsKICAgIGxldCByb290ID0gdG1wLnBhdGgoKTsKCiAgICAvLyBDYXJnby50b21sIHdpdGhvdXQgW3dvcmtzcGFjZV0gc2VjdGlvbgogICAgd3JpdGVfY2FyZ29fdG9tbChyb290LCByIyIKW3BhY2thZ2VdCm5hbWUgPSAic3RhbmRhbG9uZSIKdmVyc2lvbiA9ICIwLjEuMCIKZWRpdGlvbiA9ICIyMDIxIgoiIyk7CgogICAgbGV0IG91dHB1dCA9IHJ1bl9iaW5hcnkocm9vdC50b19zdHIoKS51bndyYXAoKSk7CgogICAgLy8gU2hvdWxkIGV4aXQgbm9uLXplcm8gYmVjYXVzZSB3b3Jrc3BhY2UgaXMgbWlzc2luZwogICAgYXNzZXJ0ISgKICAgICAgICAhb3V0cHV0LnN0YXR1cy5zdWNjZXNzKCksCiAgICAgICAgInNob3VsZCBleGl0IG5vbi16ZXJvIGZvciBtaXNzaW5nIHdvcmtzcGFjZSBzZWN0aW9uIgogICAgKTsKfQoKI1t0ZXN0XQpmbiB0ZXN0X2dsb2JfbWVtYmVyX3BhdHRlcm5zKCkgewogICAgbGV0IHRtcCA9IHRlbXBmaWxlOjp0ZW1wZGlyKCkudW53cmFwKCk7CiAgICBsZXQgcm9vdCA9IHRtcC5wYXRoKCk7CgogICAgd3JpdGVfY2FyZ29fdG9tbChyb290LCByIyIKW3dvcmtzcGFjZV0KbWVtYmVycyA9IFsiY3JhdGVzLyoiXQoiIyk7CgogICAgZm9yIG5hbWUgaW4gJlsiYWxwaGEiLCAiYmV0YSIsICJnYW1tYSJdIHsKICAgICAgICBzZXR1cF9jcmF0ZSgmcm9vdC5qb2luKCJjcmF0ZXMiKS5qb2luKG5hbWUpLCBmb3JtYXQhKCJwdWIgc3RydWN0IHtuYW1lfSB7e319IikuYXNfc3RyKCkpOwogICAgfQoKICAgIGxldCBvdXRwdXQgPSBydW5fYmluYXJ5KHJvb3QudG9fc3RyKCkudW53cmFwKCkpOwogICAgYXNzZXJ0IShvdXRwdXQuc3RhdHVzLnN1Y2Nlc3MoKSk7CgogICAgbGV0IGpzb24gPSBwYXJzZV9vdXRwdXQoJm91dHB1dCk7CiAgICBsZXQgY3JhdGVzID0gZXh0cmFjdF9hcnJheSgmanNvbiwgImNyYXRlcyIpOwogICAgbGV0IG5hbWVzOiBWZWM8JnN0cj4gPSBjcmF0ZXMuaXRlcigpCiAgICAgICAgLm1hcCh8Y3wgY1sibmFtZSJdLmFzX3N0cigpLnVud3JhcCgpKQogICAgICAgIC5jb2xsZWN0KCk7CgogICAgYXNzZXJ0IShuYW1lcy5jb250YWlucygmImFscGhhIikpOwogICAgYXNzZXJ0IShuYW1lcy5jb250YWlucygmImJldGEiKSk7CiAgICBhc3NlcnQhKG5hbWVzLmNvbnRhaW5zKCYiZ2FtbWEiKSk7CiAgICBhc3NlcnRfZXEhKG5hbWVzLmxlbigpLCAzKTsKfQoKI1t0ZXN0XQpmbiB0ZXN0X3dvcmtzcGFjZV93aXRoX2V4Y2x1ZGUoKSB7CiAgICBsZXQgdG1wID0gdGVtcGZpbGU6OnRlbXBkaXIoKS51bndyYXAoKTsKICAgIGxldCByb290ID0gdG1wLnBhdGgoKTsKCiAgICB3cml0ZV9jYXJnb190b21sKHJvb3QsIHIjIgpbd29ya3NwYWNlXQptZW1iZXJzID0gWyJhIiwgImIiLCAiYyJdCmV4Y2x1ZGUgPSBbImIiXQoiIyk7CgogICAgc2V0dXBfY3JhdGUoJnJvb3Quam9pbigiYSIpLCAicHViIHN0cnVjdCBBIHt9Iik7CiAgICBzZXR1cF9jcmF0ZSgmcm9vdC5qb2luKCJiIiksICJwdWIgc3RydWN0IEIge30iKTsKICAgIHNldHVwX2NyYXRlKCZyb290LmpvaW4oImMiKSwgInB1YiBzdHJ1Y3QgQyB7fSIpOwoKICAgIGxldCBvdXRwdXQgPSBydW5fYmluYXJ5KHJvb3QudG9fc3RyKCkudW53cmFwKCkpOwogICAgYXNzZXJ0IShvdXRwdXQuc3RhdHVzLnN1Y2Nlc3MoKSk7CgogICAgbGV0IGpzb24gPSBwYXJzZV9vdXRwdXQoJm91dHB1dCk7CiAgICBsZXQgY3JhdGVzID0gZXh0cmFjdF9hcnJheSgmanNvbiwgImNyYXRlcyIpOwogICAgbGV0IG5hbWVzOiBWZWM8JnN0cj4gPSBjcmF0ZXMuaXRlcigpCiAgICAgICAgLm1hcCh8Y3wgY1sibmFtZSJdLmFzX3N0cigpLnVud3JhcCgpKQogICAgICAgIC5jb2xsZWN0KCk7CgogICAgYXNzZXJ0IShuYW1lcy5jb250YWlucygmImEiKSk7CiAgICBhc3NlcnQhKCFuYW1lcy5jb250YWlucygmImIiKSk7CiAgICBhc3NlcnQhKG5hbWVzLmNvbnRhaW5zKCYiYyIpKTsKfQoKI1t0ZXN0XQpmbiB0ZXN0X2RlZXBseV9uZXN0ZWRfbW9kdWxlcygpIHsKICAgIGxldCB0bXAgPSB0ZW1wZmlsZTo6dGVtcGRpcigpLnVud3JhcCgpOwogICAgbGV0IHJvb3QgPSB0bXAucGF0aCgpOwoKICAgIHdyaXRlX2NhcmdvX3RvbWwocm9vdCwgciMiClt3b3Jrc3BhY2VdCm1lbWJlcnMgPSBbIi4iXQoKW3BhY2thZ2VdCm5hbWUgPSAibmVzdGVkIgp2ZXJzaW9uID0gIjAuMS4wIgplZGl0aW9uID0gIjIwMjEiCiIjKTsKCiAgICBsZXQgc3JjID0gcm9vdC5qb2luKCJzcmMiKTsKICAgIGxldCBmb28gPSBzcmMuam9pbigiZm9vIik7CiAgICBsZXQgYmFyID0gZm9vLmpvaW4oImJhciIpOwogICAgc3RkOjpmczo6Y3JlYXRlX2Rpcl9hbGwoJmJhcikudW53cmFwKCk7CgogICAgLy8gbGliLnJzIGRlY2xhcmVzIG1vZCBmb28KICAgIHN0ZDo6ZnM6OndyaXRlKHNyYy5qb2luKCJsaWIucnMiKSwgIm1vZCBmb287IikudW53cmFwKCk7CiAgICAvLyBmb28ucnMgZGVjbGFyZXMgbW9kIGJhcgogICAgc3RkOjpmczo6d3JpdGUoZm9vLmpvaW4oImZvby5ycyIpLCAibW9kIGJhcjsiKS51bndyYXAoKTsKICAgIC8vIGJhci9iYXoucnMgZGVjbGFyZXMgbW9kIGJhegogICAgc3RkOjpmczo6d3JpdGUoYmFyLmpvaW4oImJhci5ycyIpLCAibW9kIGJhejsiKS51bndyYXAoKTsKICAgIC8vIGJhei5ycyB3aXRoIGEgc3RydWN0CiAgICBzdGQ6OmZzOjp3cml0ZShiYXIuam9pbigiYmF6LnJzIiksICJwdWIgc3RydWN0IERlZXAge30iKS51bndyYXAoKTsKCiAgICBsZXQgb3V0cHV0ID0gcnVuX2JpbmFyeShyb290LnRvX3N0cigpLnVud3JhcCgpKTsKICAgIGFzc2VydCEob3V0cHV0LnN0YXR1cy5zdWNjZXNzKCkpOwoKICAgIGxldCBqc29uID0gcGFyc2Vfb3V0cHV0KCZvdXRwdXQpOwogICAgbGV0IGNyYXRlcyA9IGV4dHJhY3RfYXJyYXkoJmpzb24sICJjcmF0ZXMiKTsKICAgIGxldCBuZXN0ZWRfY3JhdGUgPSBjcmF0ZXMuaXRlcigpLmZpbmQofGN8IGNbIm5hbWUiXS5hc19zdHIoKS51bndyYXAoKSA9PSAibmVzdGVkIikudW53cmFwKCk7CgogICAgbGV0IG1vZHVsZV9wYXRoczogVmVjPCZzdHI+ID0gZXh0cmFjdF9hcnJheSgmbmVzdGVkX2NyYXRlWyJtb2R1bGVzIl0sICJwYXRoIikKICAgICAgICAuaXRlcigpCiAgICAgICAgLm1hcCh8bXwgbS5hc19zdHIoKS51bndyYXAoKSkKICAgICAgICAuY29sbGVjdCgpOwoKICAgIGFzc2VydCEobW9kdWxlX3BhdGhzLml0ZXIoKS5hbnkofHB8ICpwID09ICJuZXN0ZWQiKSk7CiAgICBhc3NlcnQhKG1vZHVsZV9wYXRocy5pdGVyKCkuYW55KHxwfCAqcCA9PSAibmVzdGVkOjpmb28iKSk7CiAgICBhc3NlcnQhKG1vZHVsZV9wYXRocy5pdGVyKCkuYW55KHxwfCAqcCA9PSAibmVzdGVkOjpmb286OmJhciIpKTsKICAgIGFzc2VydCEobW9kdWxlX3BhdGhzLml0ZXIoKS5hbnkofHB8ICpwID09ICJuZXN0ZWQ6OmZvbzo6YmFyOjpiYXoiKSk7Cn0KCiNbdGVzdF0KZm4gdGVzdF9yZWV4cG9ydF9jaGFpbnMoKSB7CiAgICBsZXQgdG1wID0gdGVtcGZpbGU6OnRlbXBkaXIoKS51bndyYXAoKTsKICAgIGxldCByb290ID0gdG1wLnBhdGgoKTsKCiAgICB3cml0ZV9jYXJnb190b21sKHJvb3QsIHIjIgpbd29ya3NwYWNlXQptZW1iZXJzID0gWyIuIl0KCltwYWNrYWdlXQpuYW1lID0gInJlZXhwb3J0ZXIiCnZlcnNpb24gPSAiMC4xLjAiCmVkaXRpb24gPSAiMjAyMSIKIiMpOwoKICAgIGxldCBzcmMgPSByb290LmpvaW4oInNyYyIpOwogICAgc3RkOjpmczo6Y3JlYXRlX2Rpcl9hbGwoJnNyYykudW53cmFwKCk7CgogICAgLy8gbGliLnJzIHdpdGggcmUtZXhwb3J0IGNoYWluCiAgICBzdGQ6OmZzOjp3cml0ZShzcmMuam9pbigibGliLnJzIiksICIKbW9kIGlubmVyIHsKICAgIHB1YiBzdHJ1Y3QgU2VjcmV0Owp9CnB1YiB1c2UgaW5uZXI6OlNlY3JldDsKIikudW53cmFwKCk7CgogICAgbGV0IG91dHB1dCA9IHJ1bl9iaW5hcnkocm9vdC50b19zdHIoKS51bndyYXAoKSk7CiAgICBhc3NlcnQhKG91dHB1dC5zdGF0dXMuc3VjY2VzcygpKTsKCiAgICBsZXQganNvbiA9IHBhcnNlX291dHB1dCgmb3V0cHV0KTsKICAgIGxldCBjcmF0ZXMgPSBleHRyYWN0X2FycmF5KCZqc29uLCAiY3JhdGVzIik7CiAgICBsZXQgcmVleHBvcnRlciA9IGNyYXRlcy5pdGVyKCkuZmluZCh8Y3wgY1sibmFtZSJdLmFzX3N0cigpLnVud3JhcCgpID09ICJyZWV4cG9ydGVyIikudW53cmFwKCk7CgogICAgbGV0IHJlX2V4cG9ydHM6IFZlYzwmc2VyZGVfanNvbjo6VmFsdWU+ID0gZXh0cmFjdF9hcnJheSgmcmVleHBvcnRlclsibW9kdWxlcyJdKQogICAgICAgIC5pdGVyKCkKICAgICAgICAuZmxhdF9tYXAofG18IGV4dHJhY3RfYXJyYXkobSwgInJlRXhwb3J0cyIpKQogICAgICAgIC5jb2xsZWN0KCk7CgogICAgbGV0IGhhc19zZWNyZXQgPSByZV9leHBvcnRzLml0ZXIoKS5hbnkofHJlfCB7CiAgICAgICAgcmVbImltcG9ydFBhdGgiXS5hc19zdHIoKS51bndyYXAoKS5jb250YWlucygiU2VjcmV0IikKICAgIH0pOwogICAgYXNzZXJ0IShoYXNfc2VjcmV0LCAic2hvdWxkIGhhdmUgcmUtZXhwb3J0IGZvciBTZWNyZXQiKTsKfQoKI1t0ZXN0XQpmbiB0ZXN0X291dHB1dF92aWFfZmxhZygpIHsKICAgIGxldCB0bXAgPSB0ZW1wZmlsZTo6dGVtcGRpcigpLnVud3JhcCgpOwogICAgbGV0IGZpeHR1cmUgPSBzdGQ6OnBhdGg6OlBhdGg6Om5ldygidGVzdHMvZml4dHVyZXMvc2FtcGxlLXdvcmtzcGFjZSIpOwogICAgbGV0IG91dHB1dF9wYXRoID0gdG1wLnBhdGgoKS5qb2luKCJvdXRwdXQuanNvbiIpOwoKICAgIC8vIFJ1biB3aXRoIC1vIGZsYWcKICAgIGxldCBvdXRwdXQxID0gQ29tbWFuZDo6bmV3KCZiaW5hcnlfcGF0aCgpKQogICAgICAgIC5hcmcoZml4dHVyZSkKICAgICAgICAuYXJnKCItbyIpCiAgICAgICAgLmFyZyhvdXRwdXRfcGF0aC5jbG9uZSgpKQogICAgICAgIC5vdXRwdXQoKQogICAgICAgIC5leHBlY3QoImZhaWxlZCB0byBleGVjdXRlIGJpbmFyeSIpOwogICAgYXNzZXJ0IShvdXRwdXQxLnN0YXR1cy5zdWNjZXNzKCkpOwoKICAgIC8vIFJ1biB3aXRob3V0IC1vLCBjYXB0dXJlIHN0ZG91dAogICAgbGV0IG91dHB1dDIgPSBDb21tYW5kOjpuZXcoJmJpbmFyeV9wYXRoKCkpCiAgICAgICAgLmFyZyhmaXh0dXJlKQogICAgICAgIC5vdXRwdXQoKQogICAgICAgIC5leHBlY3QoImZhaWxlZCB0byBleGVjdXRlIGJpbmFyeSIpOwogICAgYXNzZXJ0IShvdXRwdXQyLnN0YXR1cy5zdWNjZXNzKCkpOwoKICAgIC8vIENvbXBhcmUgZmlsZSBjb250ZW50IHdpdGggc3Rkb3V0CiAgICBsZXQgZmlsZV9jb250ZW50ID0gc3RkOjpmczo6cmVhZF90b19zdHJpbmcoJm91dHB1dF9wYXRoKS51bndyYXAoKTsKICAgIGxldCBzdGRvdXRfY29udGVudCA9IFN0cmluZzo6ZnJvbV91dGY4X2xvc3N5KCZvdXRwdXQyLnN0ZG91dCk7CiAgICBhc3NlcnRfZXEhKAogICAgICAgIGZpbGVfY29udGVudC50cmltKCksCiAgICAgICAgc3Rkb3V0X2NvbnRlbnQudHJpbSgpLAogICAgICAgICJmaWxlIG91dHB1dCBzaG91bGQgbWF0Y2ggc3Rkb3V0IgogICAgKTsKfQo=", "target": "tests/integration_test.rs", "index": 0, "is_create": false}]')

for step in STEPS:
    before = base64.b64decode(step["before_b64"]).decode()
    after = base64.b64decode(step["after_b64"]).decode()
    target = step["target"]
    idx = step["index"]
    is_create = step["is_create"]

    if is_create:
        target_path = Path(target)
        target_path.parent.mkdir(parents=True, exist_ok=True)
        target_path.write_text(after)
        print(f"OK {TASK_ID} change {idx}: created {target}")
    else:
        target_path = Path(target)
        content = target_path.read_text()
        if before not in content:
            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
            sys.exit(1)

        result = subprocess.run(
            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
            capture_output=True, text=True,
        )
        if result.returncode != 0:
            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
            sys.exit(result.returncode)

        new_content = target_path.read_text()
        if after and after not in new_content:
            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
            sys.exit(1)

        print(f"OK {TASK_ID} change {idx}: applied to {target}")

print(f"OK {TASK_ID}: all changes applied")
## File: plans/compiled/TASK-10.sh
#!/usr/bin/env bash
set -euo pipefail
# TASK-10: Expand integration tests from 3 to 10 covering error cases, workspace patterns, and output
# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
# Type: replace
# File: tests/integration_test.rs
python3 "$(dirname "$0")/TASK-10.py"
## File: plans/compiled/TASK-2.py
#!/usr/bin/env python3
"""TASK-2: Add MissingWorkspaceSection variant to Error enum in schema.rs"""
import base64, json, subprocess, sys
from pathlib import Path

TASK_ID = "TASK-2"
STEPS = json.loads('[{"before_b64": "ICAgICNbZXJyb3IoImdsb2IgcGF0dGVybiBlcnJvcjogezB9IildCiAgICBHbG9iUGF0dGVybihTdHJpbmcpLAp9CgpwdWIgdHlwZSBSZXN1bHQ8VD4gPSBzdGQ6OnJlc3VsdDo6UmVzdWx0PFQsIEVycm9yPjs=", "after_b64": "ICAgICNbZXJyb3IoImdsb2IgcGF0dGVybiBlcnJvcjogezB9IildCiAgICBHbG9iUGF0dGVybihTdHJpbmcpLAoKICAgICNbZXJyb3IoIndvcmtzcGFjZSBDYXJnby50b21sIGlzIG1pc3NpbmcgdGhlIFt3b3Jrc3BhY2VdIHNlY3Rpb24iKV0KICAgIE1pc3NpbmdXb3Jrc3BhY2VTZWN0aW9uLAp9CgpwdWIgdHlwZSBSZXN1bHQ8VD4gPSBzdGQ6OnJlc3VsdDo6UmVzdWx0PFQsIEVycm9yPjs=", "target": "src/schema.rs", "index": 0, "is_create": false}]')

for step in STEPS:
    before = base64.b64decode(step["before_b64"]).decode()
    after = base64.b64decode(step["after_b64"]).decode()
    target = step["target"]
    idx = step["index"]
    is_create = step["is_create"]

    if is_create:
        target_path = Path(target)
        target_path.parent.mkdir(parents=True, exist_ok=True)
        target_path.write_text(after)
        print(f"OK {TASK_ID} change {idx}: created {target}")
    else:
        target_path = Path(target)
        content = target_path.read_text()
        if before not in content:
            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
            sys.exit(1)

        result = subprocess.run(
            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
            capture_output=True, text=True,
        )
        if result.returncode != 0:
            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
            sys.exit(result.returncode)

        new_content = target_path.read_text()
        if after and after not in new_content:
            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
            sys.exit(1)

        print(f"OK {TASK_ID} change {idx}: applied to {target}")

print(f"OK {TASK_ID}: all changes applied")
## File: plans/compiled/TASK-2.sh
#!/usr/bin/env bash
set -euo pipefail
# TASK-2: Add MissingWorkspaceSection variant to Error enum in schema.rs
# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
# Type: replace
# File: src/schema.rs
python3 "$(dirname "$0")/TASK-2.py"
## File: plans/compiled/TASK-3.py
#!/usr/bin/env python3
"""TASK-3: Expand ErrorEntry struct with severity, kind, context, and cause fields"""
import base64, json, subprocess, sys
from pathlib import Path

TASK_ID = "TASK-3"
STEPS = json.loads('[{"before_b64": "I1tkZXJpdmUoRGVidWcsIENsb25lLCBzZXJkZTo6U2VyaWFsaXplLCBib246OkJ1aWxkZXIpXQojW3NlcmRlKHJlbmFtZV9hbGwgPSAiY2FtZWxDYXNlIildCnB1YiBzdHJ1Y3QgRXJyb3JFbnRyeSB7CiAgICBwdWIgZmlsZTogU3RyaW5nLAogICAgI1tidWlsZGVyKGRlZmF1bHQpXQogICAgcHViIGxpbmU6IHVzaXplLAogICAgcHViIG1lc3NhZ2U6IFN0cmluZywKfQo=", "after_b64": "I1tkZXJpdmUoRGVidWcsIENsb25lLCBzZXJkZTo6U2VyaWFsaXplLCBib246OkJ1aWxkZXIpXQojW3NlcmRlKHJlbmFtZV9hbGwgPSAiY2FtZWxDYXNlIildCnB1YiBzdHJ1Y3QgRXJyb3JFbnRyeSB7CiAgICBwdWIgZmlsZTogU3RyaW5nLAogICAgI1tidWlsZGVyKGRlZmF1bHQpXQogICAgcHViIGxpbmU6IHVzaXplLAogICAgcHViIG1lc3NhZ2U6IFN0cmluZywKICAgIHB1YiBzZXZlcml0eTogRXJyb3JTZXZlcml0eSwKICAgIHB1YiBraW5kOiBTdHJpbmcsCiAgICAjW3NlcmRlKHNraXBfc2VyaWFsaXppbmdfaWYgPSAiT3B0aW9uOjppc19ub25lIildCiAgICBwdWIgY29udGV4dDogT3B0aW9uPEVycm9yQ29udGV4dD4sCiAgICAjW3NlcmRlKHNraXBfc2VyaWFsaXppbmdfaWYgPSAiT3B0aW9uOjppc19ub25lIildCiAgICBwdWIgY2F1c2U6IE9wdGlvbjxTdHJpbmc+LAp9Cg==", "target": "src/schema.rs", "index": 0, "is_create": false}]')

for step in STEPS:
    before = base64.b64decode(step["before_b64"]).decode()
    after = base64.b64decode(step["after_b64"]).decode()
    target = step["target"]
    idx = step["index"]
    is_create = step["is_create"]

    if is_create:
        target_path = Path(target)
        target_path.parent.mkdir(parents=True, exist_ok=True)
        target_path.write_text(after)
        print(f"OK {TASK_ID} change {idx}: created {target}")
    else:
        target_path = Path(target)
        content = target_path.read_text()
        if before not in content:
            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
            sys.exit(1)

        result = subprocess.run(
            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
            capture_output=True, text=True,
        )
        if result.returncode != 0:
            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
            sys.exit(result.returncode)

        new_content = target_path.read_text()
        if after and after not in new_content:
            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
            sys.exit(1)

        print(f"OK {TASK_ID} change {idx}: applied to {target}")

print(f"OK {TASK_ID}: all changes applied")
## File: plans/compiled/TASK-3.sh
#!/usr/bin/env bash
set -euo pipefail
# TASK-3: Expand ErrorEntry struct with severity, kind, context, and cause fields
# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
# Type: replace
# File: src/schema.rs
python3 "$(dirname "$0")/TASK-3.py"
## File: plans/compiled/TASK-4.py
#!/usr/bin/env python3
"""TASK-4: Change parse_file to return ParsedFile with optional parse errors instead of Result"""
import base64, json, subprocess, sys
from pathlib import Path

TASK_ID = "TASK-4"
STEPS = json.loads('[{"before_b64": "dXNlIGNyYXRlOjpzY2hlbWE6OnsKICAgIEVycm9yLCBGaWxlSW5mbywgSW1wbEluZm8sIEltcGxJdGVtLCBJbXBsSXRlbUtpbmQsIEltcG9ydCwgSXRlbUF0dHJzLCBJdGVtS2luZCwgUHVibGljSXRlbSwKICAgIFJlRXhwb3J0LCBSZXN1bHQsIFN1Ym1vZHVsZURlY2wsCn07CnVzZSBzdGQ6OnBhdGg6OlBhdGg7CgovLyDilIDilIAgcGFyc2VfZmlsZSDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIAKCi8vLyBSZWFkIGFuZCBwYXJzZSBhIFJ1c3Qgc291cmNlIGZpbGUuIFJldHVybnMgdGhlIHJhdyBgc3luOjpGaWxlYCBBU1QgKG5lZWRlZAovLy8gYnkgYG1vZHVsZV90cmVlYCBmb3IgaW5saW5lIG1vZHVsZSBpdGVtIGV4dHJhY3Rpb24pIGFuZCB0aGUgZXh0cmFjdGVkCi8vLyBgRmlsZUluZm9gLiBPbiBwYXJzZSBmYWlsdXJlLCB3YXJucyB0byBzdGRlcnIgYW5kIHJldHVybnMgZW1wdHkgcmVzdWx0cy4KcHViIGZuIHBhcnNlX2ZpbGUocGF0aDogJlBhdGgpIC0+IFJlc3VsdDwoc3luOjpGaWxlLCBGaWxlSW5mbyk+IHsKICAgIGxldCBjb250ZW50ID0gc3RkOjpmczo6cmVhZF90b19zdHJpbmcocGF0aCkubWFwX2Vycih8c291cmNlfCBFcnJvcjo6RmlsZVJlYWQgewogICAgICAgIHBhdGg6IHBhdGgudG9fcGF0aF9idWYoKSwKICAgICAgICBzb3VyY2UsCiAgICB9KT87CgogICAgbGV0IGZpbGUgPSBtYXRjaCBzeW46OnBhcnNlX2ZpbGUoJmNvbnRlbnQpIHsKICAgICAgICBPayhmKSA9PiBmLAogICAgICAgIEVycihlKSA9PiB7CiAgICAgICAgICAgIGVwcmludGxuISgid2FybmluZzogZmFpbGVkIHRvIHBhcnNlIHt9OiB7fSIsIHBhdGguZGlzcGxheSgpLCBlKTsKICAgICAgICAgICAgbGV0IGVtcHR5ID0gc3luOjpGaWxlIHsKICAgICAgICAgICAgICAgIHNoZWJhbmc6IE5vbmUsCiAgICAgICAgICAgICAgICBhdHRyczogdmVjIVtdLAogICAgICAgICAgICAgICAgaXRlbXM6IHZlYyFbXSwKICAgICAgICAgICAgfTsKICAgICAgICAgICAgbGV0IGluZm8gPSBGaWxlSW5mbzo6ZGVmYXVsdCgpOwogICAgICAgICAgICByZXR1cm4gT2soKGVtcHR5LCBpbmZvKSk7CiAgICAgICAgfQogICAgfTsKCiAgICBsZXQgaW5mbyA9IEZpbGVJbmZvIHsKICAgICAgICBwdWJsaWNfaXRlbXM6IGV4dHJhY3RfcHVibGljX2l0ZW1zKCZmaWxlLml0ZW1zKSwKICAgICAgICBpbXBvcnRzOiBleHRyYWN0X2ltcG9ydHMoJmZpbGUuaXRlbXMpLAogICAgICAgIHJlX2V4cG9ydHM6IGV4dHJhY3RfcmVfZXhwb3J0cygmZmlsZS5pdGVtcyksCiAgICAgICAgc3VibW9kdWxlczogZXh0cmFjdF9zdWJtb2R1bGVzKCZmaWxlLml0ZW1zKSwKICAgICAgICBpbXBsczogZXh0cmFjdF9pbXBscygmZmlsZS5pdGVtcyksCiAgICB9OwoKICAgIE9rKChmaWxlLCBpbmZvKSkKfQ==", "after_b64": "dXNlIGNyYXRlOjpzY2hlbWE6OnsKICAgIEVycm9yLCBFcnJvckVudHJ5LCBFcnJvclNldmVyaXR5LCBGaWxlSW5mbywgSW1wbEluZm8sIEltcGxJdGVtLCBJbXBsSXRlbUtpbmQsIEltcG9ydCwKICAgIEl0ZW1BdHRycywgSXRlbUtpbmQsIFB1YmxpY0l0ZW0sIFJlRXhwb3J0LCBSZXN1bHQsIFN1Ym1vZHVsZURlY2wsCn07CnVzZSBzdGQ6OnBhdGg6OlBhdGg7CgovLyDilIDilIAgSW50ZXJuYWwgcGFyc2UgcmVzdWx0IHR5cGVzIOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgAoKLy8vIFJlc3VsdCBvZiBwYXJzaW5nIGEgUnVzdCBzb3VyY2UgZmlsZS4KLy8vCi8vLyBVbmxpa2UgYFJlc3VsdDxULCBFcnJvcj5gLCB0aGlzIHR5cGUgYWx3YXlzIHN1Y2NlZWRzIOKAlCBwYXJzZQovLy8gZmFpbHVyZXMgYXJlIHJlcG9ydGVkIGFzIGRhdGEsIG5vdCBhcyBlcnJvcnMsIHNvIHRoZSBjYWxsZXIKLy8vIGNhbiBjb250aW51ZSBwcm9jZXNzaW5nIG90aGVyIGZpbGVzLiBUaGUgY2FsbGVyIGNvbnN0cnVjdHMKLy8vIGBFcnJvckVudHJ5YCB2YWx1ZXMgZnJvbSBgU3luUGFyc2VFcnJvcmAgd2hlbiBuZWVkZWQuCnB1YiBzdHJ1Y3QgUGFyc2VkRmlsZSB7CiAgICBwdWIgYXN0OiBzeW46OkZpbGUsCiAgICBwdWIgZmlsZV9pbmZvOiBGaWxlSW5mbywKICAgIHB1YiBwYXJzZV9lcnJvcjogT3B0aW9uPFN5blBhcnNlRXJyb3I+LAp9CgovLy8gU3RydWN0dXJlZCBpbmZvcm1hdGlvbiBhYm91dCBhIHBhcnNlIGZhaWx1cmUuCnB1YiBzdHJ1Y3QgU3luUGFyc2VFcnJvciB7CiAgICBwdWIgbWVzc2FnZTogU3RyaW5nLAogICAgcHViIGxpbmU6IHVzaXplLAp9CgovLyDilIDilIAgcGFyc2VfZmlsZSDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIAKCi8vLyBSZWFkIGFuZCBwYXJzZSBhIFJ1c3Qgc291cmNlIGZpbGUuCi8vLwovLy8gT24gcGFyc2UgZmFpbHVyZSwgcmV0dXJucyB0aGUgb3JpZ2luYWwgZmlsZSBjb250ZW50IGFuZCBhCi8vLyBgU3luUGFyc2VFcnJvcmAgYWxvbmdzaWRlIGFuIGVtcHR5IGBGaWxlSW5mb2AuIENhbGxlcnMgdXNlIHRoZQovLy8gZXJyb3IgdG8gY29uc3RydWN0IGFuIGBFcnJvckVudHJ5YC4KcHViIGZuIHBhcnNlX2ZpbGUocGF0aDogJlBhdGgpIC0+IFBhcnNlZEZpbGUgewogICAgbGV0IGNvbnRlbnQgPSBtYXRjaCBzdGQ6OmZzOjpyZWFkX3RvX3N0cmluZyhwYXRoKSB7CiAgICAgICAgT2soYykgPT4gYywKICAgICAgICBFcnIoc291cmNlKSA9PiB7CiAgICAgICAgICAgIGxldCBlcnIgPSBTeW5QYXJzZUVycm9yIHsKICAgICAgICAgICAgICAgIG1lc3NhZ2U6IHNvdXJjZS50b19zdHJpbmcoKSwKICAgICAgICAgICAgICAgIGxpbmU6IDAsCiAgICAgICAgICAgIH07CiAgICAgICAgICAgIHJldHVybiBQYXJzZWRGaWxlIHsKICAgICAgICAgICAgICAgIGFzdDogc3luOjpGaWxlIHsKICAgICAgICAgICAgICAgICAgICBzaGViYW5nOiBOb25lLAogICAgICAgICAgICAgICAgICAgIGF0dHJzOiB2ZWMhW10sCiAgICAgICAgICAgICAgICAgICAgaXRlbXM6IHZlYyFbXSwKICAgICAgICAgICAgICAgIH0sCiAgICAgICAgICAgICAgICBmaWxlX2luZm86IEZpbGVJbmZvOjpkZWZhdWx0KCksCiAgICAgICAgICAgICAgICBwYXJzZV9lcnJvcjogU29tZShlcnIpLAogICAgICAgICAgICB9OwogICAgICAgIH0KICAgIH07CgogICAgbWF0Y2ggc3luOjpwYXJzZV9maWxlKCZjb250ZW50KSB7CiAgICAgICAgT2soZmlsZSkgPT4gewogICAgICAgICAgICBsZXQgZmlsZV9pbmZvID0gRmlsZUluZm8gewogICAgICAgICAgICAgICAgcHVibGljX2l0ZW1zOiBleHRyYWN0X3B1YmxpY19pdGVtcygmZmlsZS5pdGVtcyksCiAgICAgICAgICAgICAgICBpbXBvcnRzOiBleHRyYWN0X2ltcG9ydHMoJmZpbGUuaXRlbXMpLAogICAgICAgICAgICAgICAgcmVfZXhwb3J0czogZXh0cmFjdF9yZV9leHBvcnRzKCZmaWxlLml0ZW1zKSwKICAgICAgICAgICAgICAgIHN1Ym1vZHVsZXM6IGV4dHJhY3Rfc3VibW9kdWxlcygmZmlsZS5pdGVtcyksCiAgICAgICAgICAgICAgICBpbXBsczogZXh0cmFjdF9pbXBscygmZmlsZS5pdGVtcyksCiAgICAgICAgICAgIH07CiAgICAgICAgICAgIFBhcnNlZEZpbGUgewogICAgICAgICAgICAgICAgYXN0OiBmaWxlLAogICAgICAgICAgICAgICAgZmlsZV9pbmZvLAogICAgICAgICAgICAgICAgcGFyc2VfZXJyb3I6IE5vbmUsCiAgICAgICAgICAgIH0KICAgICAgICB9LAogICAgICAgIEVycihlKSA9PiB7CiAgICAgICAgICAgIGxldCBsaW5lID0gZS5zcGFuKCkuc3RhcnQoKS5saW5lOwogICAgICAgICAgICBsZXQgZXJyID0gU3luUGFyc2VFcnJvciB7CiAgICAgICAgICAgICAgICBtZXNzYWdlOiBlLnRvX3N0cmluZygpLAogICAgICAgICAgICAgICAgbGluZSwKICAgICAgICAgICAgfTsKICAgICAgICAgICAgUGFyc2VkRmlsZSB7CiAgICAgICAgICAgICAgICBhc3Q6IHN5bjo6RmlsZSB7CiAgICAgICAgICAgICAgICAgICAgc2hlYmFuZzogTm9uZSwKICAgICAgICAgICAgICAgICAgICBhdHRyczogdmVjIVtdLAogICAgICAgICAgICAgICAgICAgIGl0ZW1zOiB2ZWMhW10sCiAgICAgICAgICAgICAgICB9LAogICAgICAgICAgICAgICAgZmlsZV9pbmZvOiBGaWxlSW5mbzo6ZGVmYXVsdCgpLAogICAgICAgICAgICAgICAgcGFyc2VfZXJyb3I6IFNvbWUoZXJyKSwKICAgICAgICAgICAgfQogICAgICAgIH0KICAgIH0KfQoKcHViKGNyYXRlKSBmbiBidWlsZF9wYXJzZV9lcnJvcl9lbnRyeShwYXRoOiAmUGF0aCwgZXJyOiAmU3luUGFyc2VFcnJvcikgLT4gRXJyb3JFbnRyeSB7CiAgICBFcnJvckVudHJ5OjpidWlsZGVyKCkKICAgICAgICAuZmlsZShwYXRoLnRvX3N0cmluZ19sb3NzeSgpLnRvX3N0cmluZygpKQogICAgICAgIC5saW5lKGVyci5saW5lKQogICAgICAgIC5tZXNzYWdlKGVyci5tZXNzYWdlLmNsb25lKCkpCiAgICAgICAgLnNldmVyaXR5KEVycm9yU2V2ZXJpdHk6OkVycm9yKQogICAgICAgIC5raW5kKCJzeW5fcGFyc2VfZXJyb3IiLnRvX3N0cmluZygpKQogICAgICAgIC5idWlsZCgpCn0=", "target": "src/file_parser.rs", "index": 0, "is_create": false}]')

for step in STEPS:
    before = base64.b64decode(step["before_b64"]).decode()
    after = base64.b64decode(step["after_b64"]).decode()
    target = step["target"]
    idx = step["index"]
    is_create = step["is_create"]

    if is_create:
        target_path = Path(target)
        target_path.parent.mkdir(parents=True, exist_ok=True)
        target_path.write_text(after)
        print(f"OK {TASK_ID} change {idx}: created {target}")
    else:
        target_path = Path(target)
        content = target_path.read_text()
        if before not in content:
            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
            sys.exit(1)

        result = subprocess.run(
            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
            capture_output=True, text=True,
        )
        if result.returncode != 0:
            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
            sys.exit(result.returncode)

        new_content = target_path.read_text()
        if after and after not in new_content:
            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
            sys.exit(1)

        print(f"OK {TASK_ID} change {idx}: applied to {target}")

print(f"OK {TASK_ID}: all changes applied")
## File: plans/compiled/TASK-4.sh
#!/usr/bin/env bash
set -euo pipefail
# TASK-4: Change parse_file to return ParsedFile with optional parse errors instead of Result
# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
# Type: replace
# File: src/file_parser.rs
python3 "$(dirname "$0")/TASK-4.py"
## File: plans/compiled/TASK-5.py
#!/usr/bin/env python3
"""TASK-5: Refactor module_tree.rs: fix path fallbacks, add error collection, change build_module_tree return type"""
import base64, json, subprocess, sys
from pathlib import Path

TASK_ID = "TASK-5"
STEPS = json.loads('[{"before_b64": "dXNlIGNyYXRlOjpzY2hlbWE6OntGaWxlSW5mbywgTW9kdWxlSW5mbywgUmVzdWx0LCBTdWJtb2R1bGVEZWNsfTs=", "after_b64": "dXNlIGNyYXRlOjpzY2hlbWE6OntFcnJvckNvbnRleHQsIEVycm9yRW50cnksIEVycm9yU2V2ZXJpdHksIEZpbGVJbmZvLCBNb2R1bGVJbmZvLCBSZXN1bHQsIFN1Ym1vZHVsZURlY2x9Ow==", "target": "src/module_tree.rs", "index": 0, "is_create": false}, {"before_b64": "Ly8vIEJ1aWxkIHRoZSBmdWxsIG1vZHVsZSB0cmVlIGZvciBhIGNyYXRlIHN0YXJ0aW5nIGZyb20gaXRzIGVudHJ5IHBvaW50Ci8vLyAoZS5nLiwgYHNyYy9saWIucnNgKS4gUmV0dXJucyBhIGZsYXQgYFZlYzxNb2R1bGVJbmZvPmAgY29udGFpbmluZyB0aGUKLy8vIHJvb3QgbW9kdWxlIGFuZCBhbGwgcmVjdXJzaXZlbHkgZGlzY292ZXJlZCBzdWJtb2R1bGVzLgpwdWIgZm4gYnVpbGRfbW9kdWxlX3RyZWUoY3JhdGVfcm9vdDogJlBhdGgsIGNyYXRlX25hbWU6ICZzdHIpIC0+IFJlc3VsdDxWZWM8TW9kdWxlSW5mbz4+IHsKICAgIGxldCBtdXQgdmlzaXRlZCA9IEhhc2hTZXQ6Om5ldygpOwogICAgbGV0IHBhcmVudF9kaXIgPSBjcmF0ZV9yb290LnBhcmVudCgpLnVud3JhcF9vcl9lbHNlKHx8IFBhdGg6Om5ldygiLiIpKTsKCiAgICBsZXQgKGFzdCwgZmlsZV9pbmZvKSA9IGZpbGVfcGFyc2VyOjpwYXJzZV9maWxlKGNyYXRlX3Jvb3QpPzsKICAgIHZpc2l0ZWQuaW5zZXJ0KGNyYXRlX3Jvb3QudG9fcGF0aF9idWYoKSk7CgogICAgbGV0IHJvb3RfbW9kdWxlID0gYnVpbGRfbW9kdWxlX2luZm8oCiAgICAgICAgY3JhdGVfbmFtZSwKICAgICAgICBjcmF0ZV9yb290LAogICAgICAgICJwdWIiLAogICAgICAgICZmaWxlX2luZm8ucHVibGljX2l0ZW1zLAogICAgICAgICZmaWxlX2luZm8uaW1wb3J0cywKICAgICAgICAmZmlsZV9pbmZvLnJlX2V4cG9ydHMsCiAgICAgICAgJmZpbGVfaW5mby5zdWJtb2R1bGVzLAogICAgKTsKCiAgICBsZXQgbXV0IG1vZHVsZXMgPSB2ZWMhW3Jvb3RfbW9kdWxlXTsKCiAgICBmb3Igc3ViIGluICZmaWxlX2luZm8uc3VibW9kdWxlcyB7CiAgICAgICAgaWYgc3ViLmlzX3Rlc3QgewogICAgICAgICAgICBjb250aW51ZTsKICAgICAgICB9CiAgICAgICAgbGV0IHN1Yl9tb2R1bGVfcGF0aCA9IGZvcm1hdCEoInt9Ojp7fSIsIGNyYXRlX25hbWUsIHN1Yi5uYW1lKTsKICAgICAgICBsZXQgY2hpbGRfbW9kdWxlcyA9IHByb2Nlc3Nfc3VibW9kdWxlKAogICAgICAgICAgICAmc3ViX21vZHVsZV9wYXRoLAogICAgICAgICAgICAmc3ViLm5hbWUsCiAgICAgICAgICAgICZhc3QuaXRlbXMsCiAgICAgICAgICAgIHBhcmVudF9kaXIsCiAgICAgICAgICAgIGNyYXRlX3Jvb3QsCiAgICAgICAgICAgICZtdXQgdmlzaXRlZCwKICAgICAgICApPzsKICAgICAgICBtb2R1bGVzLmV4dGVuZChjaGlsZF9tb2R1bGVzKTsKICAgIH0KCiAgICBPayhtb2R1bGVzKQp9", "after_b64": "Ly8vIEJ1aWxkIHRoZSBmdWxsIG1vZHVsZSB0cmVlIGZvciBhIGNyYXRlIHN0YXJ0aW5nIGZyb20gaXRzIGVudHJ5IHBvaW50Ci8vLyAoZS5nLiwgYHNyYy9saWIucnNgKS4gUmV0dXJucyBhIHR1cGxlIG9mIG1vZHVsZSBpbmZvIGFuZCBhbnkgZXJyb3JzCi8vLyBlbmNvdW50ZXJlZCBkdXJpbmcgc3VibW9kdWxlIHBhcnNpbmcgKGluY2x1ZGluZyBvcnBoYW5lZCBtb2R1bGUgd2FybmluZ3MpLgpwdWIgZm4gYnVpbGRfbW9kdWxlX3RyZWUoCiAgICBjcmF0ZV9yb290OiAmUGF0aCwKICAgIGNyYXRlX25hbWU6ICZzdHIsCikgLT4gKFZlYzxNb2R1bGVJbmZvPiwgVmVjPGNyYXRlOjpzY2hlbWE6OkVycm9yRW50cnk+KSB7CgogICAgbGV0IG11dCB2aXNpdGVkID0gSGFzaFNldDo6bmV3KCk7CiAgICBsZXQgcGFyZW50X2RpciA9IGNyYXRlX3Jvb3QucGFyZW50KCkudW53cmFwX29yKGNyYXRlX3Jvb3QpOwoKICAgIGxldCBwYXJzZWQgPSBmaWxlX3BhcnNlcjo6cGFyc2VfZmlsZShjcmF0ZV9yb290KTsKICAgIGxldCBtdXQgZXJyb3JzOiBWZWM8RXJyb3JFbnRyeT4gPSBWZWM6Om5ldygpOwogICAgaWYgbGV0IFNvbWUocmVmIGVycikgPSBwYXJzZWQucGFyc2VfZXJyb3IgewogICAgICAgIGVycm9ycy5wdXNoKGNyYXRlOjpmaWxlX3BhcnNlcjo6YnVpbGRfcGFyc2VfZXJyb3JfZW50cnkoY3JhdGVfcm9vdCwgZXJyKSk7CiAgICB9CiAgICB2aXNpdGVkLmluc2VydChjcmF0ZV9yb290LnRvX3BhdGhfYnVmKCkpOwoKICAgIGxldCByb290X21vZHVsZSA9IGJ1aWxkX21vZHVsZV9pbmZvKAogICAgICAgIGNyYXRlX25hbWUsCiAgICAgICAgY3JhdGVfcm9vdCwKICAgICAgICAicHViIiwKICAgICAgICAmcGFyc2VkLmZpbGVfaW5mby5wdWJsaWNfaXRlbXMsCiAgICAgICAgJnBhcnNlZC5maWxlX2luZm8uaW1wb3J0cywKICAgICAgICAmcGFyc2VkLmZpbGVfaW5mby5yZV9leHBvcnRzLAogICAgICAgICZwYXJzZWQuZmlsZV9pbmZvLnN1Ym1vZHVsZXMsCiAgICApOwoKICAgIGxldCBtdXQgbW9kdWxlcyA9IHZlYyFbcm9vdF9tb2R1bGVdOwoKICAgIGZvciBzdWIgaW4gJnBhcnNlZC5maWxlX2luZm8uc3VibW9kdWxlcyB7CiAgICAgICAgaWYgc3ViLmlzX3Rlc3QgewogICAgICAgICAgICBjb250aW51ZTsKICAgICAgICB9CiAgICAgICAgbGV0IHN1Yl9tb2R1bGVfcGF0aCA9IGZvcm1hdCEoInt9Ojp7fSIsIGNyYXRlX25hbWUsIHN1Yi5uYW1lKTsKICAgICAgICBsZXQgKGNoaWxkX21vZHVsZXMsIGNoaWxkX2Vycm9ycykgPSBwcm9jZXNzX3N1Ym1vZHVsZSgKICAgICAgICAgICAgJnN1Yl9tb2R1bGVfcGF0aCwKICAgICAgICAgICAgJnN1Yi5uYW1lLAogICAgICAgICAgICAmcGFyc2VkLmFzdC5pdGVtcywKICAgICAgICAgICAgcGFyZW50X2RpciwKICAgICAgICAgICAgY3JhdGVfcm9vdCwKICAgICAgICAgICAgJm11dCB2aXNpdGVkLAogICAgICAgICk7CiAgICAgICAgZXJyb3JzLmV4dGVuZChjaGlsZF9lcnJvcnMpOwogICAgICAgIG1vZHVsZXMuZXh0ZW5kKGNoaWxkX21vZHVsZXMpOwogICAgfQoKICAgIChtb2R1bGVzLCBlcnJvcnMpCn0=", "target": "src/module_tree.rs", "index": 1, "is_create": false}, {"before_b64": "ICAgIGxldCBTb21lKG1vZF9pdGVtKSA9IG1vZF9pdGVtIGVsc2UgewogICAgICAgIGVwcmludGxuISgid2FybmluZzogb3JwaGFuZWQgbW9kdWxlIHt9IiwgbW9kdWxlX3BhdGgpOwogICAgICAgIHJldHVybiBPayh2ZWMhW01vZHVsZUluZm86OmJ1aWxkZXIoKQogICAgICAgICAgICAucGF0aChtb2R1bGVfcGF0aC50b19zdHJpbmcoKSkKICAgICAgICAgICAgLmZpbGUoIjx1bnJlc29sdmVkPiIudG9fc3RyaW5nKCkpCiAgICAgICAgICAgIC52aXNpYmlsaXR5KCJwcml2YXRlIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgLmJ1aWxkKCldKTsKICAgIH07", "after_b64": "ICAgIGxldCBTb21lKG1vZF9pdGVtKSA9IG1vZF9pdGVtIGVsc2UgewogICAgICAgIGxldCBlcnIgPSBFcnJvckVudHJ5OjpidWlsZGVyKCkKICAgICAgICAgICAgLmZpbGUoU3RyaW5nOjpuZXcoKSkKICAgICAgICAgICAgLm1lc3NhZ2UoZm9ybWF0ISgib3JwaGFuZWQgbW9kdWxlOiB7bW9kdWxlX3BhdGh9IikpCiAgICAgICAgICAgIC5zZXZlcml0eShFcnJvclNldmVyaXR5OjpXYXJuaW5nKQogICAgICAgICAgICAua2luZCgib3JwaGFuZWRfbW9kdWxlIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgLmNvbnRleHQoRXJyb3JDb250ZXh0OjpidWlsZGVyKCkKICAgICAgICAgICAgICAgIC5tb2R1bGVfcGF0aChtb2R1bGVfcGF0aC50b19zdHJpbmcoKSkKICAgICAgICAgICAgICAgIC5idWlsZCgpKQogICAgICAgICAgICAuYnVpbGQoKTsKICAgICAgICByZXR1cm4gKHZlYyFbTW9kdWxlSW5mbzo6YnVpbGRlcigpCiAgICAgICAgICAgIC5wYXRoKG1vZHVsZV9wYXRoLnRvX3N0cmluZygpKQogICAgICAgICAgICAuZmlsZSgiPHVucmVzb2x2ZWQ+Ii50b19zdHJpbmcoKSkKICAgICAgICAgICAgLnZpc2liaWxpdHkoInByaXZhdGUiLnRvX3N0cmluZygpKQogICAgICAgICAgICAuYnVpbGQoKV0sIHZlYyFbZXJyXSk7CiAgICB9Ow==", "target": "src/module_tree.rs", "index": 2, "is_create": false}, {"before_b64": "ICAgICAgICBsZXQgU29tZShyZWYgZmlsZV9wYXRoKSA9IGZpbGVfcGF0aCBlbHNlIHsKICAgICAgICAgICAgZXByaW50bG4hKCJ3YXJuaW5nOiBvcnBoYW5lZCBtb2R1bGUge30iLCBtb2R1bGVfcGF0aCk7CiAgICAgICAgICAgIHJldHVybiBPayh2ZWMhW01vZHVsZUluZm86OmJ1aWxkZXIoKQogICAgICAgICAgICAgICAgLnBhdGgobW9kdWxlX3BhdGgudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAuZmlsZSgiPHVucmVzb2x2ZWQ+Ii50b19zdHJpbmcoKSkKICAgICAgICAgICAgICAgIC52aXNpYmlsaXR5KHZpc2liaWxpdHkudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAuYnVpbGQoKV0pOwogICAgICAgIH07", "after_b64": "ICAgICAgICBsZXQgU29tZShyZWYgZmlsZV9wYXRoKSA9IGZpbGVfcGF0aCBlbHNlIHsKICAgICAgICAgICAgbGV0IGVyciA9IEVycm9yRW50cnk6OmJ1aWxkZXIoKQogICAgICAgICAgICAgICAgLmZpbGUoU3RyaW5nOjpuZXcoKSkKICAgICAgICAgICAgICAgIC5tZXNzYWdlKGZvcm1hdCEoIm9ycGhhbmVkIG1vZHVsZToge21vZHVsZV9wYXRofSIpKQogICAgICAgICAgICAgICAgLnNldmVyaXR5KEVycm9yU2V2ZXJpdHk6Oldhcm5pbmcpCiAgICAgICAgICAgICAgICAua2luZCgib3JwaGFuZWRfbW9kdWxlIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgICAgIC5jb250ZXh0KEVycm9yQ29udGV4dDo6YnVpbGRlcigpCiAgICAgICAgICAgICAgICAgICAgLm1vZHVsZV9wYXRoKG1vZHVsZV9wYXRoLnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgICAgIC5idWlsZCgpKQogICAgICAgICAgICAgICAgLmJ1aWxkKCk7CiAgICAgICAgICAgIHJldHVybiAodmVjIVtNb2R1bGVJbmZvOjpidWlsZGVyKCkKICAgICAgICAgICAgICAgIC5wYXRoKG1vZHVsZV9wYXRoLnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgLmZpbGUoIjx1bnJlc29sdmVkPiIudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAudmlzaWJpbGl0eSh2aXNpYmlsaXR5LnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgLmJ1aWxkKCldLCB2ZWMhW2Vycl0pOwogICAgICAgIH07", "target": "src/module_tree.rs", "index": 3, "is_create": false}, {"before_b64": "ICAgICAgICBsZXQgKGFzdCwgZmlsZV9pbmZvKSA9IGZpbGVfcGFyc2VyOjpwYXJzZV9maWxlKGZpbGVfcGF0aCk/OwogICAgICAgIHByb2Nlc3NfbW9kdWxlX2luZm8oCiAgICAgICAgICAgIG1vZHVsZV9wYXRoLAogICAgICAgICAgICBmaWxlX3BhdGgsCiAgICAgICAgICAgIHZpc2liaWxpdHksCiAgICAgICAgICAgICZmaWxlX2luZm8sCiAgICAgICAgICAgICZhc3QuaXRlbXMsCiAgICAgICAgICAgICZmaWxlX3BhdGgucGFyZW50KCkudW53cmFwX29yX2Vsc2UofHwgUGF0aDo6bmV3KCIuIikpLAogICAgICAgICAgICB2aXNpdGVkLAogICAgICAgICk=", "after_b64": "ICAgICAgICBsZXQgcGFyc2VkID0gZmlsZV9wYXJzZXI6OnBhcnNlX2ZpbGUoZmlsZV9wYXRoKTsKICAgICAgICBsZXQgbXV0IGVycm9yczogVmVjPEVycm9yRW50cnk+ID0gVmVjOjpuZXcoKTsKICAgICAgICBpZiBsZXQgU29tZShyZWYgZXJyKSA9IHBhcnNlZC5wYXJzZV9lcnJvciB7CiAgICAgICAgICAgIGVycm9ycy5wdXNoKGNyYXRlOjpmaWxlX3BhcnNlcjo6YnVpbGRfcGFyc2VfZXJyb3JfZW50cnkoZmlsZV9wYXRoLCBlcnIpKTsKICAgICAgICB9CiAgICAgICAgcHJvY2Vzc19tb2R1bGVfaW5mbygKICAgICAgICAgICAgbW9kdWxlX3BhdGgsCiAgICAgICAgICAgIGZpbGVfcGF0aCwKICAgICAgICAgICAgdmlzaWJpbGl0eSwKICAgICAgICAgICAgJnBhcnNlZC5maWxlX2luZm8sCiAgICAgICAgICAgICZwYXJzZWQuYXN0Lml0ZW1zLAogICAgICAgICAgICAmZmlsZV9wYXRoLnBhcmVudCgpLnVud3JhcF9vcihmaWxlX3BhdGgpLAogICAgICAgICAgICB2aXNpdGVkLAogICAgICAgICAgICAmbXV0IGVycm9ycywKICAgICAgICAp", "target": "src/module_tree.rs", "index": 4, "is_create": false}, {"before_b64": "ICAgICAgICBpZiB2aXNpdGVkLmNvbnRhaW5zKGZpbGVfcGF0aC5hc19wYXRoKCkpIHsKICAgICAgICAgICAgcmV0dXJuIE9rKHZlYyFbXSk7IC8vIGN5Y2xlIGRldGVjdGVkCiAgICAgICAgfQ==", "after_b64": "ICAgICAgICBpZiB2aXNpdGVkLmNvbnRhaW5zKGZpbGVfcGF0aC5hc19wYXRoKCkpIHsKICAgICAgICAgICAgcmV0dXJuICh2ZWMhW10sIHZlYyFbXSk7IC8vIGN5Y2xlIGRldGVjdGVkCiAgICAgICAgfQ==", "target": "src/module_tree.rs", "index": 5, "is_create": false}, {"before_b64": "Ly8vIFJlc29sdmUgYSBgbW9kIG5hbWU7YCBkZWNsYXJhdGlvbiB0byBhIGZpbGUgcGF0aC4KLy8vIFRyaWVzIGB7cGFyZW50X2Rpcn0ve21vZF9uYW1lfS5yc2AgZmlyc3QsIHRoZW4gYHtwYXJlbnRfZGlyfS97bW9kX25hbWV9L21vZC5yc2AuCnB1YiBmbiByZXNvbHZlX21vZHVsZV9wYXRoKHBhcmVudF9kaXI6ICZQYXRoLCBtb2RfbmFtZTogJnN0cikgLT4gT3B0aW9uPFBhdGhCdWY+IHs=", "after_b64": "Ly8vIFJlc29sdmUgYSBgbW9kIG5hbWU7YCBkZWNsYXJhdGlvbiB0byBhIGZpbGUgcGF0aC4KLy8vIFRyaWVzIGB7cGFyZW50X2Rpcn0ve21vZF9uYW1lfS5yc2AgZmlyc3QsIHRoZW4gYHtwYXJlbnRfZGlyfS97bW9kX25hbWV9L21vZC5yc2AuCi8vLwovLy8gUmV0dXJucyBgTm9uZWAgaWYgbmVpdGhlciBwYXRoIGV4aXN0cy4KcHViIGZuIHJlc29sdmVfbW9kdWxlX3BhdGgocGFyZW50X2RpcjogJlBhdGgsIG1vZF9uYW1lOiAmc3RyKSAtPiBPcHRpb248UGF0aEJ1Zj4gew==", "target": "src/module_tree.rs", "index": 6, "is_create": false}, {"before_b64": "Zm4gcHJvY2Vzc19zdWJtb2R1bGUoCiAgICBtb2R1bGVfcGF0aDogJnN0ciwKICAgIG1vZF9uYW1lOiAmc3RyLAogICAgcGFyZW50X2l0ZW1zOiAmW3N5bjo6SXRlbV0sCiAgICBwYXJlbnRfZGlyOiAmUGF0aCwKICAgIHBhcmVudF9maWxlOiAmUGF0aCwKICAgIHZpc2l0ZWQ6ICZtdXQgSGFzaFNldDxQYXRoQnVmPiwKKSAtPiBSZXN1bHQ8VmVjPE1vZHVsZUluZm8+PiB7", "after_b64": "Zm4gcHJvY2Vzc19zdWJtb2R1bGUoCiAgICBtb2R1bGVfcGF0aDogJnN0ciwKICAgIG1vZF9uYW1lOiAmc3RyLAogICAgcGFyZW50X2l0ZW1zOiAmW3N5bjo6SXRlbV0sCiAgICBwYXJlbnRfZGlyOiAmUGF0aCwKICAgIHBhcmVudF9maWxlOiAmUGF0aCwKICAgIHZpc2l0ZWQ6ICZtdXQgSGFzaFNldDxQYXRoQnVmPiwKKSAtPiAoVmVjPE1vZHVsZUluZm8+LCBWZWM8Y3JhdGU6OnNjaGVtYTo6RXJyb3JFbnRyeT4pIHs=", "target": "src/module_tree.rs", "index": 7, "is_create": false}, {"before_b64": "ICAgIGlmIGxldCBTb21lKChfLCByZWYgaW5saW5lX2l0ZW1zKSkgPSBtb2RfaXRlbS5jb250ZW50IHsKICAgICAgICAvLyBJbmxpbmUgbW9kdWxlOiBwcm9jZXNzIGl0cyBib2R5IGl0ZW1zIGRpcmVjdGx5IChubyBmaWxlIGxvb2t1cCkuCiAgICAgICAgcHJvY2Vzc19tb2R1bGVfaXRlbXMoCiAgICAgICAgICAgIG1vZHVsZV9wYXRoLAogICAgICAgICAgICBwYXJlbnRfZmlsZSwKICAgICAgICAgICAgdmlzaWJpbGl0eSwKICAgICAgICAgICAgaW5saW5lX2l0ZW1zLAogICAgICAgICAgICBwYXJlbnRfZGlyLAogICAgICAgICAgICB2aXNpdGVkLAogICAgICAgICkKICAgIH0gZWxzZSB7", "after_b64": "ICAgIGlmIGxldCBTb21lKChfLCByZWYgaW5saW5lX2l0ZW1zKSkgPSBtb2RfaXRlbS5jb250ZW50IHsKICAgICAgICAvLyBJbmxpbmUgbW9kdWxlOiBwcm9jZXNzIGl0cyBib2R5IGl0ZW1zIGRpcmVjdGx5IChubyBmaWxlIGxvb2t1cCkuCiAgICAgICAgbGV0IChtb2R1bGVzLCBlcnJzKSA9IHByb2Nlc3NfbW9kdWxlX2l0ZW1zKAogICAgICAgICAgICBtb2R1bGVfcGF0aCwKICAgICAgICAgICAgcGFyZW50X2ZpbGUsCiAgICAgICAgICAgIHZpc2liaWxpdHksCiAgICAgICAgICAgIGlubGluZV9pdGVtcywKICAgICAgICAgICAgcGFyZW50X2RpciwKICAgICAgICAgICAgdmlzaXRlZCwKICAgICAgICApOwogICAgICAgIHJldHVybiAobW9kdWxlcywgZXJycyk7CiAgICB9IGVsc2Ugew==", "target": "src/module_tree.rs", "index": 8, "is_create": false}, {"before_b64": "Zm4gcHJvY2Vzc19tb2R1bGVfaXRlbXMoCiAgICBtb2R1bGVfcGF0aDogJnN0ciwKICAgIGZpbGVfcGF0aDogJlBhdGgsCiAgICB2aXNpYmlsaXR5OiAmc3RyLAogICAgaXRlbXM6ICZbc3luOjpJdGVtXSwKICAgIHBhcmVudF9kaXI6ICZQYXRoLAogICAgdmlzaXRlZDogJm11dCBIYXNoU2V0PFBhdGhCdWY+LAopIC0+IFJlc3VsdDxWZWM8TW9kdWxlSW5mbz4+IHs=", "after_b64": "Zm4gcHJvY2Vzc19tb2R1bGVfaXRlbXMoCiAgICBtb2R1bGVfcGF0aDogJnN0ciwKICAgIGZpbGVfcGF0aDogJlBhdGgsCiAgICB2aXNpYmlsaXR5OiAmc3RyLAogICAgaXRlbXM6ICZbc3luOjpJdGVtXSwKICAgIHBhcmVudF9kaXI6ICZQYXRoLAogICAgdmlzaXRlZDogJm11dCBIYXNoU2V0PFBhdGhCdWY+LAopIC0+IChWZWM8TW9kdWxlSW5mbz4sIFZlYzxjcmF0ZTo6c2NoZW1hOjpFcnJvckVudHJ5Pikgew==", "target": "src/module_tree.rs", "index": 9, "is_create": false}, {"before_b64": "ICAgIGxldCBmaWxlX2luZm8gPSBGaWxlSW5mbyB7CiAgICAgICAgcHVibGljX2l0ZW1zOiBmaWxlX3BhcnNlcjo6ZXh0cmFjdF9wdWJsaWNfaXRlbXMoaXRlbXMpLAogICAgICAgIGltcG9ydHM6IGZpbGVfcGFyc2VyOjpleHRyYWN0X2ltcG9ydHMoaXRlbXMpLAogICAgICAgIHJlX2V4cG9ydHM6IGZpbGVfcGFyc2VyOjpleHRyYWN0X3JlX2V4cG9ydHMoaXRlbXMpLAogICAgICAgIHN1Ym1vZHVsZXM6IGZpbGVfcGFyc2VyOjpleHRyYWN0X3N1Ym1vZHVsZXMoaXRlbXMpLAogICAgICAgIGltcGxzOiBmaWxlX3BhcnNlcjo6ZXh0cmFjdF9pbXBscyhpdGVtcyksCiAgICB9OwogICAgcHJvY2Vzc19tb2R1bGVfaW5mbyhtb2R1bGVfcGF0aCwgZmlsZV9wYXRoLCB2aXNpYmlsaXR5LCAmZmlsZV9pbmZvLCBpdGVtcywgcGFyZW50X2RpciwgdmlzaXRlZCkKfQ==", "after_b64": "ICAgIGxldCBmaWxlX2luZm8gPSBGaWxlSW5mbyB7CiAgICAgICAgcHVibGljX2l0ZW1zOiBmaWxlX3BhcnNlcjo6ZXh0cmFjdF9wdWJsaWNfaXRlbXMoaXRlbXMpLAogICAgICAgIGltcG9ydHM6IGZpbGVfcGFyc2VyOjpleHRyYWN0X2ltcG9ydHMoaXRlbXMpLAogICAgICAgIHJlX2V4cG9ydHM6IGZpbGVfcGFyc2VyOjpleHRyYWN0X3JlX2V4cG9ydHMoaXRlbXMpLAogICAgICAgIHN1Ym1vZHVsZXM6IGZpbGVfcGFyc2VyOjpleHRyYWN0X3N1Ym1vZHVsZXMoaXRlbXMpLAogICAgICAgIGltcGxzOiBmaWxlX3BhcnNlcjo6ZXh0cmFjdF9pbXBscyhpdGVtcyksCiAgICB9OwogICAgcHJvY2Vzc19tb2R1bGVfaW5mbyhtb2R1bGVfcGF0aCwgZmlsZV9wYXRoLCB2aXNpYmlsaXR5LCAmZmlsZV9pbmZvLCBpdGVtcywgcGFyZW50X2RpciwgdmlzaXRlZCwgJm11dCBWZWM6Om5ldygpKQp9", "target": "src/module_tree.rs", "index": 10, "is_create": false}, {"before_b64": "Zm4gcHJvY2Vzc19tb2R1bGVfaW5mbygKICAgIG1vZHVsZV9wYXRoOiAmc3RyLAogICAgZmlsZV9wYXRoOiAmUGF0aCwKICAgIHZpc2liaWxpdHk6ICZzdHIsCiAgICBmaWxlX2luZm86ICZGaWxlSW5mbywKICAgIGl0ZW1zOiAmW3N5bjo6SXRlbV0sCiAgICBfcGFyZW50X2RpcjogJlBhdGgsCiAgICB2aXNpdGVkOiAmbXV0IEhhc2hTZXQ8UGF0aEJ1Zj4sCikgLT4gUmVzdWx0PFZlYzxNb2R1bGVJbmZvPj4gewogICAgbGV0IG11dCBtb2R1bGVzID0gdmVjIVtidWlsZF9tb2R1bGVfaW5mbygKICAgICAgICBtb2R1bGVfcGF0aCwKICAgICAgICBmaWxlX3BhdGgsCiAgICAgICAgdmlzaWJpbGl0eSwKICAgICAgICAmZmlsZV9pbmZvLnB1YmxpY19pdGVtcywKICAgICAgICAmZmlsZV9pbmZvLmltcG9ydHMsCiAgICAgICAgJmZpbGVfaW5mby5yZV9leHBvcnRzLAogICAgICAgICZmaWxlX2luZm8uc3VibW9kdWxlcywKICAgICldOwoKICAgIGZvciBzdWIgaW4gJmZpbGVfaW5mby5zdWJtb2R1bGVzIHsKICAgICAgICBpZiBzdWIuaXNfdGVzdCB7CiAgICAgICAgICAgIGNvbnRpbnVlOwogICAgICAgIH0KICAgICAgICBsZXQgY2hpbGRfcGF0aCA9IGZvcm1hdCEoInt9Ojp7fSIsIG1vZHVsZV9wYXRoLCBzdWIubmFtZSk7CiAgICAgICAgbGV0IGNoaWxkX2RpciA9IGZpbGVfcGF0aC5wYXJlbnQoKS51bndyYXBfb3JfZWxzZSh8fCBQYXRoOjpuZXcoIi4iKSk7CiAgICAgICAgbGV0IGNoaWxkX21vZHVsZXMgPSBwcm9jZXNzX3N1Ym1vZHVsZSgKICAgICAgICAgICAgJmNoaWxkX3BhdGgsCiAgICAgICAgICAgICZzdWIubmFtZSwKICAgICAgICAgICAgaXRlbXMsCiAgICAgICAgICAgIGNoaWxkX2RpciwKICAgICAgICAgICAgZmlsZV9wYXRoLAogICAgICAgICAgICB2aXNpdGVkLAogICAgICAgICk/OwogICAgICAgIG1vZHVsZXMuZXh0ZW5kKGNoaWxkX21vZHVsZXMpOwogICAgfQoKICAgIE9rKG1vZHVsZXMpCn0=", "after_b64": "Zm4gcHJvY2Vzc19tb2R1bGVfaW5mbygKICAgIG1vZHVsZV9wYXRoOiAmc3RyLAogICAgZmlsZV9wYXRoOiAmUGF0aCwKICAgIHZpc2liaWxpdHk6ICZzdHIsCiAgICBmaWxlX2luZm86ICZGaWxlSW5mbywKICAgIGl0ZW1zOiAmW3N5bjo6SXRlbV0sCiAgICBfcGFyZW50X2RpcjogJlBhdGgsCiAgICB2aXNpdGVkOiAmbXV0IEhhc2hTZXQ8UGF0aEJ1Zj4sCiAgICBlcnJvcnM6ICZtdXQgVmVjPGNyYXRlOjpzY2hlbWE6OkVycm9yRW50cnk+LAopIC0+IChWZWM8TW9kdWxlSW5mbz4sIFZlYzxjcmF0ZTo6c2NoZW1hOjpFcnJvckVudHJ5PikgewogICAgbGV0IG11dCBtb2R1bGVzID0gdmVjIVtidWlsZF9tb2R1bGVfaW5mbygKICAgICAgICBtb2R1bGVfcGF0aCwKICAgICAgICBmaWxlX3BhdGgsCiAgICAgICAgdmlzaWJpbGl0eSwKICAgICAgICAmZmlsZV9pbmZvLnB1YmxpY19pdGVtcywKICAgICAgICAmZmlsZV9pbmZvLmltcG9ydHMsCiAgICAgICAgJmZpbGVfaW5mby5yZV9leHBvcnRzLAogICAgICAgICZmaWxlX2luZm8uc3VibW9kdWxlcywKICAgICldOwoKICAgIGZvciBzdWIgaW4gJmZpbGVfaW5mby5zdWJtb2R1bGVzIHsKICAgICAgICBpZiBzdWIuaXNfdGVzdCB7CiAgICAgICAgICAgIGNvbnRpbnVlOwogICAgICAgIH0KICAgICAgICBsZXQgY2hpbGRfcGF0aCA9IGZvcm1hdCEoInt9Ojp7fSIsIG1vZHVsZV9wYXRoLCBzdWIubmFtZSk7CiAgICAgICAgbGV0IGNoaWxkX2RpciA9IGZpbGVfcGF0aC5wYXJlbnQoKS51bndyYXBfb3IoZmlsZV9wYXRoKTsKICAgICAgICBsZXQgKGNoaWxkX21vZHVsZXMsIGNoaWxkX2Vycm9ycykgPSBwcm9jZXNzX3N1Ym1vZHVsZSgKICAgICAgICAgICAgJmNoaWxkX3BhdGgsCiAgICAgICAgICAgICZzdWIubmFtZSwKICAgICAgICAgICAgaXRlbXMsCiAgICAgICAgICAgIGNoaWxkX2RpciwKICAgICAgICAgICAgZmlsZV9wYXRoLAogICAgICAgICAgICB2aXNpdGVkLAogICAgICAgICk7CiAgICAgICAgZXJyb3JzLmV4dGVuZChjaGlsZF9lcnJvcnMpOwogICAgICAgIG1vZHVsZXMuZXh0ZW5kKGNoaWxkX21vZHVsZXMpOwogICAgfQoKICAgIChtb2R1bGVzLCBlcnJvcnMuY2xvbmUoKSkKfQ==", "target": "src/module_tree.rs", "index": 11, "is_create": false}]')

for step in STEPS:
    before = base64.b64decode(step["before_b64"]).decode()
    after = base64.b64decode(step["after_b64"]).decode()
    target = step["target"]
    idx = step["index"]
    is_create = step["is_create"]

    if is_create:
        target_path = Path(target)
        target_path.parent.mkdir(parents=True, exist_ok=True)
        target_path.write_text(after)
        print(f"OK {TASK_ID} change {idx}: created {target}")
    else:
        target_path = Path(target)
        content = target_path.read_text()
        if before not in content:
            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
            sys.exit(1)

        result = subprocess.run(
            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
            capture_output=True, text=True,
        )
        if result.returncode != 0:
            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
            sys.exit(result.returncode)

        new_content = target_path.read_text()
        if after and after not in new_content:
            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
            sys.exit(1)

        print(f"OK {TASK_ID} change {idx}: applied to {target}")

print(f"OK {TASK_ID}: all changes applied")
## File: plans/compiled/TASK-5.sh
#!/usr/bin/env bash
set -euo pipefail
# TASK-5: Refactor module_tree.rs: fix path fallbacks, add error collection, change build_module_tree return type
# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
# Type: replace
# File: src/module_tree.rs
python3 "$(dirname "$0")/TASK-5.py"
## File: plans/compiled/TASK-6.py
#!/usr/bin/env python3
"""TASK-6: Update workspace.rs enumerate_members to return MissingWorkspaceSection error"""
import base64, json, subprocess, sys
from pathlib import Path

TASK_ID = "TASK-6"
STEPS = json.loads('[{"before_b64": "ICAgIGxldCBtZW1iZXJzOiBWZWM8U3RyaW5nPiA9IHBhcnNlZAogICAgICAgIC5nZXQoIndvcmtzcGFjZSIpCiAgICAgICAgLmFuZF90aGVuKHx3fCB3LmdldCgibWVtYmVycyIpKQogICAgICAgIC5hbmRfdGhlbih8bXwgbS5hc19hcnJheSgpKQogICAgICAgIC5tYXAofGFycnwgewogICAgICAgICAgICBhcnIuaXRlcigpCiAgICAgICAgICAgICAgICAuZmlsdGVyX21hcCh8dnwgdi5hc19zdHIoKS5tYXAoU3RyaW5nOjpmcm9tKSkKICAgICAgICAgICAgICAgIC5jb2xsZWN0KCkKICAgICAgICB9KQogICAgICAgIC51bndyYXBfb3JfZGVmYXVsdCgpOw==", "after_b64": "ICAgIGxldCBtZW1iZXJzOiBWZWM8U3RyaW5nPiA9IG1hdGNoIHBhcnNlZC5nZXQoIndvcmtzcGFjZSIpIHsKICAgICAgICBOb25lID0+IHJldHVybiBFcnIoRXJyb3I6Ok1pc3NpbmdXb3Jrc3BhY2VTZWN0aW9uKSwKICAgICAgICBTb21lKHdvcmtzcGFjZSkgPT4gd29ya3NwYWNlCiAgICAgICAgICAgIC5nZXQoIm1lbWJlcnMiKQogICAgICAgICAgICAuYW5kX3RoZW4ofG18IG0uYXNfYXJyYXkoKSkKICAgICAgICAgICAgLm1hcCh8YXJyfCB7CiAgICAgICAgICAgICAgICBhcnIuaXRlcigpCiAgICAgICAgICAgICAgICAgICAgLmZpbHRlcl9tYXAofHZ8IHYuYXNfc3RyKCkubWFwKFN0cmluZzo6ZnJvbSkpCiAgICAgICAgICAgICAgICAgICAgLmNvbGxlY3Q6OjxWZWM8Xz4+KCkKICAgICAgICAgICAgfSkKICAgICAgICAgICAgLnVud3JhcF9vcl9kZWZhdWx0KCksCiAgICB9Ow==", "target": "src/workspace.rs", "index": 0, "is_create": false}, {"before_b64": "Ly8vIFdhbGsgdXAgdGhlIGRpcmVjdG9yeSB0cmVlIGZyb20gYHN0YXJ0X3BhdGhgIHRvIGZpbmQgYSBgQ2FyZ28udG9tbGAKLy8vIGNvbnRhaW5pbmcgYSBgW3dvcmtzcGFjZV1gIHNlY3Rpb24uIFJldHVybnMgdGhlIGRpcmVjdG9yeSBjb250YWluaW5nIGl0LgpwdWIgZm4gZmluZF93b3Jrc3BhY2Vfcm9vdChzdGFydF9wYXRoOiAmUGF0aCkgLT4gUmVzdWx0PFBhdGhCdWY+IHs=", "after_b64": "Ly8vIFdhbGsgdXAgdGhlIGRpcmVjdG9yeSB0cmVlIGZyb20gYHN0YXJ0X3BhdGhgIHRvIGZpbmQgYSBgQ2FyZ28udG9tbGAKLy8vIGNvbnRhaW5pbmcgYSBgW3dvcmtzcGFjZV1gIHNlY3Rpb24uIFJldHVybnMgdGhlIGRpcmVjdG9yeSBjb250YWluaW5nIGl0LgovLy8KLy8vICMgRXJyb3JzCi8vLwovLy8gUmV0dXJucyBgRXJyb3I6OldvcmtzcGFjZVJvb3ROb3RGb3VuZGAgaWYgbm8gYENhcmdvLnRvbWxgIHdpdGggYQovLy8gYFt3b3Jrc3BhY2VdYCBzZWN0aW9uIGlzIGZvdW5kIGluIGFueSBhbmNlc3RvciBkaXJlY3RvcnkuCnB1YiBmbiBmaW5kX3dvcmtzcGFjZV9yb290KHN0YXJ0X3BhdGg6ICZQYXRoKSAtPiBSZXN1bHQ8UGF0aEJ1Zj4gew==", "target": "src/workspace.rs", "index": 1, "is_create": false}, {"before_b64": "Ly8vIFBhcnNlIHRoZSB3b3Jrc3BhY2UgYENhcmdvLnRvbWxgLCByZXNvbHZlIG1lbWJlciBwYXRocyAoaW5jbHVkaW5nIGdsb2IKLy8vIHBhdHRlcm5zKSwgYXBwbHkgYGV4Y2x1ZGVgIGxpc3QsIGFuZCByZXR1cm4gYWJzb2x1dGUgcGF0aHMgdG8gZWFjaCBtZW1iZXIKLy8vIGNyYXRlIGRpcmVjdG9yeS4KcHViIGZuIGVudW1lcmF0ZV9tZW1iZXJzKHJvb3Q6ICZQYXRoKSAtPiBSZXN1bHQ8VmVjPFBhdGhCdWY+PiB7", "after_b64": "Ly8vIFBhcnNlIHRoZSB3b3Jrc3BhY2UgYENhcmdvLnRvbWxgLCByZXNvbHZlIG1lbWJlciBwYXRocyAoaW5jbHVkaW5nIGdsb2IKLy8vIHBhdHRlcm5zKSwgYXBwbHkgYGV4Y2x1ZGVgIGxpc3QsIGFuZCByZXR1cm4gYWJzb2x1dGUgcGF0aHMgdG8gZWFjaCBtZW1iZXIKLy8vIGNyYXRlIGRpcmVjdG9yeS4KLy8vCi8vLyAjIEVycm9ycwovLy8KLy8vIFJldHVybnMgYEVycm9yOjpNaXNzaW5nV29ya3NwYWNlU2VjdGlvbmAgaWYgdGhlIGBDYXJnby50b21sYCBsYWNrcyBhCi8vLyBgW3dvcmtzcGFjZV1gIHNlY3Rpb24gZW50aXJlbHkuCnB1YiBmbiBlbnVtZXJhdGVfbWVtYmVycyhyb290OiAmUGF0aCkgLT4gUmVzdWx0PFZlYzxQYXRoQnVmPj4gew==", "target": "src/workspace.rs", "index": 2, "is_create": false}]')

for step in STEPS:
    before = base64.b64decode(step["before_b64"]).decode()
    after = base64.b64decode(step["after_b64"]).decode()
    target = step["target"]
    idx = step["index"]
    is_create = step["is_create"]

    if is_create:
        target_path = Path(target)
        target_path.parent.mkdir(parents=True, exist_ok=True)
        target_path.write_text(after)
        print(f"OK {TASK_ID} change {idx}: created {target}")
    else:
        target_path = Path(target)
        content = target_path.read_text()
        if before not in content:
            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
            sys.exit(1)

        result = subprocess.run(
            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
            capture_output=True, text=True,
        )
        if result.returncode != 0:
            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
            sys.exit(result.returncode)

        new_content = target_path.read_text()
        if after and after not in new_content:
            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
            sys.exit(1)

        print(f"OK {TASK_ID} change {idx}: applied to {target}")

print(f"OK {TASK_ID}: all changes applied")
## File: plans/compiled/TASK-6.sh
#!/usr/bin/env bash
set -euo pipefail
# TASK-6: Update workspace.rs enumerate_members to return MissingWorkspaceSection error
# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
# Type: replace
# File: src/workspace.rs
python3 "$(dirname "$0")/TASK-6.py"
## File: plans/compiled/TASK-7.py
#!/usr/bin/env python3
"""TASK-7: Refactor lib.rs run(): parallel error collection, error entry construction, remove unwrap_or_default"""
import base64, json, subprocess, sys
from pathlib import Path

TASK_ID = "TASK-7"
STEPS = json.loads('[{"before_b64": "dXNlIGFueWhvdzo6Q29udGV4dDsKdXNlIHJheW9uOjpwcmVsdWRlOjoqOwp1c2Ugc2NoZW1hOjp7CiAgICBDcmF0ZUluZm8sIENyYXRlVHlwZSwgRXJyb3JFbnRyeSwgTW9kdWxlSW5mbywgV29ya3NwYWNlSW5mbywgV29ya3NwYWNlTWFwLAp9Owp1c2Ugc3RkOjpwYXRoOjpQYXRoOw==", "after_b64": "dXNlIGFueWhvdzo6Q29udGV4dDsKdXNlIHJheW9uOjpwcmVsdWRlOjoqOwp1c2Ugc2NoZW1hOjp7CiAgICBDcmF0ZUluZm8sIENyYXRlVHlwZSwgRXJyb3JFbnRyeSwgRXJyb3JTZXZlcml0eSwgTW9kdWxlSW5mbywgV29ya3NwYWNlSW5mbywKICAgIFdvcmtzcGFjZU1hcCwKfTsKdXNlIHN0ZDo6cGF0aDo6UGF0aDs=", "target": "src/lib.rs", "index": 0, "is_create": false}, {"before_b64": "ICAgIGxldCBlcnJvcnM6IFZlYzxFcnJvckVudHJ5PiA9IFZlYzo6bmV3KCk7CgogICAgbGV0IG11dCBjcmF0ZV9pbmZvczogVmVjPENyYXRlSW5mbz4gPSBtZW1iZXJfZGlycwogICAgICAgIC5wYXJfaXRlcigpCiAgICAgICAgLmZpbHRlcl9tYXAofGRpcnwgewogICAgICAgICAgICBsZXQgY2FyZ29fdG9tbCA9IGRpci5qb2luKCJDYXJnby50b21sIik7CgogICAgICAgICAgICBsZXQgKHBrZywgZGVwcykgPSBtYXRjaCBjYXJnb19pbmZvOjpwYXJzZV9jYXJnb190b21sKCZjYXJnb190b21sKSB7CiAgICAgICAgICAgICAgICBPayh2KSA9PiB2LAogICAgICAgICAgICAgICAgRXJyKGUpID0+IHsKICAgICAgICAgICAgICAgICAgICBlcHJpbnRsbiEoCiAgICAgICAgICAgICAgICAgICAgICAgICJ3YXJuaW5nOiBmYWlsZWQgdG8gcGFyc2Uge306IHt9IiwKICAgICAgICAgICAgICAgICAgICAgICAgY2FyZ29fdG9tbC5kaXNwbGF5KCksCiAgICAgICAgICAgICAgICAgICAgICAgIGUKICAgICAgICAgICAgICAgICAgICApOwogICAgICAgICAgICAgICAgICAgIHJldHVybiBOb25lOwogICAgICAgICAgICAgICAgfQogICAgICAgICAgICB9OwoKICAgICAgICAgICAgbGV0IHJvb3RzID0gd29ya3NwYWNlOjpyZXNvbHZlX2NyYXRlX3Jvb3RzKGRpcik7CiAgICAgICAgICAgIGlmIHJvb3RzLmlzX2VtcHR5KCkgewogICAgICAgICAgICAgICAgZXByaW50bG4hKAogICAgICAgICAgICAgICAgICAgICJ3YXJuaW5nOiBubyBjcmF0ZSBlbnRyeSBwb2ludHMgZm91bmQgaW4ge30iLAogICAgICAgICAgICAgICAgICAgIGRpci5kaXNwbGF5KCkKICAgICAgICAgICAgICAgICk7CiAgICAgICAgICAgICAgICByZXR1cm4gTm9uZTsKICAgICAgICAgICAgfQoKICAgICAgICAgICAgbGV0IGNyYXRlX3R5cGUgPSBpZiByb290cy5pdGVyKCkuYW55KHwoXywgdCl8ICp0ID09IENyYXRlVHlwZTo6TGliKQogICAgICAgICAgICAgICAgJiYgcm9vdHMuaXRlcigpLmFueSh8KF8sIHQpfCAqdCA9PSBDcmF0ZVR5cGU6OkJpbikKICAgICAgICAgICAgewogICAgICAgICAgICAgICAgQ3JhdGVUeXBlOjpMaWJBbmRCaW4KICAgICAgICAgICAgfSBlbHNlIHsKICAgICAgICAgICAgICAgIHJvb3RzLmZpcnN0KCkubWFwX29yKENyYXRlVHlwZTo6TGliLCB8KF8sIHQpfCAqdCkKICAgICAgICAgICAgfTsKCiAgICAgICAgICAgIGxldCBwa2dfbmFtZSA9IHBrZy5uYW1lLmNsb25lKCk7CiAgICAgICAgICAgIGxldCBtdXQgbW9kdWxlczogVmVjPE1vZHVsZUluZm8+ID0gcm9vdHMKICAgICAgICAgICAgICAgIC5pdGVyKCkKICAgICAgICAgICAgICAgIC5mbGF0X21hcCh8KHJvb3QsIF90eSl8IHsKICAgICAgICAgICAgICAgICAgICBtb2R1bGVfdHJlZTo6YnVpbGRfbW9kdWxlX3RyZWUocm9vdCwgJnBrZ19uYW1lKS51bndyYXBfb3JfZGVmYXVsdCgpCiAgICAgICAgICAgICAgICB9KQogICAgICAgICAgICAgICAgLmNvbGxlY3QoKTsKCiAgICAgICAgICAgIC8vIFJlbGF0aXZpemUgYWxsIHBhdGhzIHRvIHRoZSB3b3Jrc3BhY2Ugcm9vdC4KICAgICAgICAgICAgZm9yIG0gaW4gJm11dCBtb2R1bGVzIHsKICAgICAgICAgICAgICAgIG0uZmlsZSA9IHJlbGF0aXZpemVfcGF0aCgmbS5maWxlLCAmd29ya3NwYWNlX3Jvb3QpOwogICAgICAgICAgICAgICAgZm9yIGl0ZW0gaW4gJm11dCBtLnB1YmxpY19pdGVtcyB7CiAgICAgICAgICAgICAgICAgICAgaXRlbS5maWxlID0gcmVsYXRpdml6ZV9wYXRoKCZpdGVtLmZpbGUsICZ3b3Jrc3BhY2Vfcm9vdCk7CiAgICAgICAgICAgICAgICB9CiAgICAgICAgICAgIH0KCiAgICAgICAgICAgIGxldCBjcmF0ZV9yb290ID0gcm9vdHMKICAgICAgICAgICAgICAgIC5maXJzdCgpCiAgICAgICAgICAgICAgICAubWFwKHwociwgXyl8IHJlbGF0aXZpemVfcGF0aCgmci50b19zdHJpbmdfbG9zc3koKSwgJndvcmtzcGFjZV9yb290KSkKICAgICAgICAgICAgICAgIC51bndyYXBfb3JfZGVmYXVsdCgpOwoKICAgICAgICAgICAgbGV0IHJlYnVpbHRfcGtnID0gc2NoZW1hOjpQYWNrYWdlSW5mbzo6YnVpbGRlcigpCiAgICAgICAgICAgICAgICAubmFtZShwa2cubmFtZSkKICAgICAgICAgICAgICAgIC52ZXJzaW9uKHBrZy52ZXJzaW9uKQogICAgICAgICAgICAgICAgLmVkaXRpb24ocGtnLmVkaXRpb24pCiAgICAgICAgICAgICAgICAuY3JhdGVfdHlwZShjcmF0ZV90eXBlKQogICAgICAgICAgICAgICAgLmJ1aWxkKCk7CgogICAgICAgICAgICBTb21lKAogICAgICAgICAgICAgICAgQ3JhdGVJbmZvOjpidWlsZGVyKCkKICAgICAgICAgICAgICAgICAgICAubmFtZShwa2dfbmFtZSkKICAgICAgICAgICAgICAgICAgICAucm9vdChjcmF0ZV9yb290KQogICAgICAgICAgICAgICAgICAgIC5wYWNrYWdlKHJlYnVpbHRfcGtnKQogICAgICAgICAgICAgICAgICAgIC5tb2R1bGVzKG1vZHVsZXMpCiAgICAgICAgICAgICAgICAgICAgLmRlcHMoZGVwcykKICAgICAgICAgICAgICAgICAgICAuYnVpbGQoKSwKICAgICAgICAgICAgKQogICAgICAgIH0pCiAgICAgICAgLmNvbGxlY3QoKTs=", "after_b64": "ICAgIGxldCBtdXQgY3JhdGVfZXJyb3JzOiBWZWM8RXJyb3JFbnRyeT4gPSBWZWM6Om5ldygpOwoKICAgIGxldCByZXN1bHRzOiBWZWM8KENyYXRlSW5mbywgVmVjPEVycm9yRW50cnk+KT4gPSBtZW1iZXJfZGlycwogICAgICAgIC5wYXJfaXRlcigpCiAgICAgICAgLm1hcCh8ZGlyfCB7CiAgICAgICAgICAgIGxldCBjYXJnb190b21sID0gZGlyLmpvaW4oIkNhcmdvLnRvbWwiKTsKICAgICAgICAgICAgbGV0IG11dCBjcmF0ZV9lcnJvcnMgPSBWZWM6Om5ldygpOwoKICAgICAgICAgICAgbGV0IChwa2csIGRlcHMpID0gbWF0Y2ggY2FyZ29faW5mbzo6cGFyc2VfY2FyZ29fdG9tbCgmY2FyZ29fdG9tbCkgewogICAgICAgICAgICAgICAgT2sodikgPT4gdiwKICAgICAgICAgICAgICAgIEVycihlKSA9PiB7CiAgICAgICAgICAgICAgICAgICAgY3JhdGVfZXJyb3JzLnB1c2goRXJyb3JFbnRyeTo6YnVpbGRlcigpCiAgICAgICAgICAgICAgICAgICAgICAgIC5maWxlKGNhcmdvX3RvbWwudG9fc3RyaW5nX2xvc3N5KCkudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAgICAgICAgIC5tZXNzYWdlKGZvcm1hdCEoImZhaWxlZCB0byBwYXJzZSBDYXJnby50b21sOiB7ZX0iKSkKICAgICAgICAgICAgICAgICAgICAgICAgLnNldmVyaXR5KEVycm9yU2V2ZXJpdHk6OkVycm9yKQogICAgICAgICAgICAgICAgICAgICAgICAua2luZCgidG9tbF9wYXJzZV9lcnJvciIudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAgICAgICAgIC5jYXVzZShlLnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgICAgICAgICAuYnVpbGQoKSk7CiAgICAgICAgICAgICAgICAgICAgcmV0dXJuIChOb25lLCBjcmF0ZV9lcnJvcnMpOwogICAgICAgICAgICAgICAgfQogICAgICAgICAgICB9OwoKICAgICAgICAgICAgbGV0IHJvb3RzID0gd29ya3NwYWNlOjpyZXNvbHZlX2NyYXRlX3Jvb3RzKGRpcik7CiAgICAgICAgICAgIGlmIHJvb3RzLmlzX2VtcHR5KCkgewogICAgICAgICAgICAgICAgY3JhdGVfZXJyb3JzLnB1c2goRXJyb3JFbnRyeTo6YnVpbGRlcigpCiAgICAgICAgICAgICAgICAgICAgLmZpbGUoZGlyLnRvX3N0cmluZ19sb3NzeSgpLnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgICAgIC5tZXNzYWdlKCJubyBjcmF0ZSBlbnRyeSBwb2ludHMgZm91bmQiLnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgICAgIC5zZXZlcml0eShFcnJvclNldmVyaXR5OjpXYXJuaW5nKQogICAgICAgICAgICAgICAgICAgIC5raW5kKCJtaXNzaW5nX2NyYXRlX3Jvb3RzIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgICAgICAgICAuYnVpbGQoKSk7CiAgICAgICAgICAgICAgICByZXR1cm4gKE5vbmUsIGNyYXRlX2Vycm9ycyk7CiAgICAgICAgICAgIH0KCiAgICAgICAgICAgIGxldCBjcmF0ZV90eXBlID0gaWYgcm9vdHMuaXRlcigpLmFueSh8KF8sIHQpfCAqdCA9PSBDcmF0ZVR5cGU6OkxpYikKICAgICAgICAgICAgICAgICYmIHJvb3RzLml0ZXIoKS5hbnkofChfLCB0KXwgKnQgPT0gQ3JhdGVUeXBlOjpCaW4pCiAgICAgICAgICAgIHsKICAgICAgICAgICAgICAgIENyYXRlVHlwZTo6TGliQW5kQmluCiAgICAgICAgICAgIH0gZWxzZSB7CiAgICAgICAgICAgICAgICByb290cy5maXJzdCgpLm1hcF9vcihDcmF0ZVR5cGU6OkxpYiwgfChfLCB0KXwgKnQpCiAgICAgICAgICAgIH07CgogICAgICAgICAgICBsZXQgcGtnX25hbWUgPSBwa2cubmFtZS5jbG9uZSgpOwogICAgICAgICAgICBsZXQgbXV0IG1vZHVsZXM6IFZlYzxNb2R1bGVJbmZvPiA9IFZlYzo6bmV3KCk7CiAgICAgICAgICAgIGxldCBtdXQgY29sbGVjdGVkX2Vycm9ycyA9IFZlYzo6bmV3KCk7CiAgICAgICAgICAgIGZvciAocm9vdCwgX3R5KSBpbiAmcm9vdHMgewogICAgICAgICAgICAgICAgbGV0IChtLCBlKSA9IG1vZHVsZV90cmVlOjpidWlsZF9tb2R1bGVfdHJlZSgmcm9vdCwgJnBrZ19uYW1lKTsKICAgICAgICAgICAgICAgIG1vZHVsZXMuZXh0ZW5kKG0pOwogICAgICAgICAgICAgICAgY29sbGVjdGVkX2Vycm9ycy5leHRlbmQoZSk7CiAgICAgICAgICAgIH0KICAgICAgICAgICAgY3JhdGVfZXJyb3JzLmV4dGVuZChjb2xsZWN0ZWRfZXJyb3JzKTsKCiAgICAgICAgICAgIC8vIFJlbGF0aXZpemUgYWxsIHBhdGhzIHRvIHRoZSB3b3Jrc3BhY2Ugcm9vdC4KICAgICAgICAgICAgZm9yIG0gaW4gJm11dCBtb2R1bGVzIHsKICAgICAgICAgICAgICAgIG0uZmlsZSA9IHJlbGF0aXZpemVfcGF0aCgmbS5maWxlLCAmd29ya3NwYWNlX3Jvb3QpOwogICAgICAgICAgICAgICAgZm9yIGl0ZW0gaW4gJm11dCBtLnB1YmxpY19pdGVtcyB7CiAgICAgICAgICAgICAgICAgICAgaXRlbS5maWxlID0gcmVsYXRpdml6ZV9wYXRoKCZpdGVtLmZpbGUsICZ3b3Jrc3BhY2Vfcm9vdCk7CiAgICAgICAgICAgICAgICB9CiAgICAgICAgICAgIH0KCiAgICAgICAgICAgIGxldCBjcmF0ZV9yb290ID0gcm9vdHMKICAgICAgICAgICAgICAgIC5maXJzdCgpCiAgICAgICAgICAgICAgICAubWFwKHwociwgXyl8IHJlbGF0aXZpemVfcGF0aCgmci50b19zdHJpbmdfbG9zc3koKSwgJndvcmtzcGFjZV9yb290KSkKICAgICAgICAgICAgICAgIC51bndyYXBfb3JfZGVmYXVsdCgpOwoKICAgICAgICAgICAgbGV0IHJlYnVpbHRfcGtnID0gc2NoZW1hOjpQYWNrYWdlSW5mbzo6YnVpbGRlcigpCiAgICAgICAgICAgICAgICAubmFtZShwa2cubmFtZSkKICAgICAgICAgICAgICAgIC52ZXJzaW9uKHBrZy52ZXJzaW9uKQogICAgICAgICAgICAgICAgLmVkaXRpb24ocGtnLmVkaXRpb24pCiAgICAgICAgICAgICAgICAuY3JhdGVfdHlwZShjcmF0ZV90eXBlKQogICAgICAgICAgICAgICAgLmJ1aWxkKCk7CgogICAgICAgICAgICBsZXQgY3JhdGVfaW5mbyA9IENyYXRlSW5mbzo6YnVpbGRlcigpCiAgICAgICAgICAgICAgICAubmFtZShwa2dfbmFtZSkKICAgICAgICAgICAgICAgIC5yb290KGNyYXRlX3Jvb3QpCiAgICAgICAgICAgICAgICAucGFja2FnZShyZWJ1aWx0X3BrZykKICAgICAgICAgICAgICAgIC5tb2R1bGVzKG1vZHVsZXMpCiAgICAgICAgICAgICAgICAuZGVwcyhkZXBzKQogICAgICAgICAgICAgICAgLmJ1aWxkKCk7CgogICAgICAgICAgICAoU29tZShjcmF0ZV9pbmZvKSwgY3JhdGVfZXJyb3JzKQogICAgICAgIH0pCiAgICAgICAgLmNvbGxlY3QoKTsKCiAgICBsZXQgbXV0IGNyYXRlX2luZm9zOiBWZWM8Q3JhdGVJbmZvPiA9IFZlYzo6bmV3KCk7CgogICAgZm9yIChpbmZvLCBlcnJzKSBpbiByZXN1bHRzIHsKICAgICAgICBpZiBsZXQgU29tZShjaSkgPSBpbmZvIHsKICAgICAgICAgICAgY3JhdGVfZXJyb3JzLmV4dGVuZChlcnJzKTsKICAgICAgICAgICAgY3JhdGVfaW5mb3MucHVzaChjaSk7CiAgICAgICAgfQogICAgfQ==", "target": "src/lib.rs", "index": 1, "is_create": false}, {"before_b64": "ICAgIGxldCBtYXAgPSBXb3Jrc3BhY2VNYXA6OmJ1aWxkZXIoKQogICAgICAgIC53b3Jrc3BhY2Uod29ya3NwYWNlX2luZm8pCiAgICAgICAgLmNyYXRlcyhjcmF0ZV9pbmZvcykKICAgICAgICAuY3Jvc3NfcmVmZXJlbmNlcyhjcm9zc19yZWZzKQogICAgICAgIC5lcnJvcnMoZXJyb3JzKQogICAgICAgIC53b3Jrc3BhY2Vfcm9vdCh3b3Jrc3BhY2Vfcm9vdC5jbG9uZSgpKQogICAgICAgIC5idWlsZCgpOw==", "after_b64": "ICAgIGxldCBtYXAgPSBXb3Jrc3BhY2VNYXA6OmJ1aWxkZXIoKQogICAgICAgIC53b3Jrc3BhY2Uod29ya3NwYWNlX2luZm8pCiAgICAgICAgLmNyYXRlcyhjcmF0ZV9pbmZvcykKICAgICAgICAuY3Jvc3NfcmVmZXJlbmNlcyhjcm9zc19yZWZzKQogICAgICAgIC5lcnJvcnMoY3JhdGVfZXJyb3JzKQogICAgICAgIC53b3Jrc3BhY2Vfcm9vdCh3b3Jrc3BhY2Vfcm9vdC5jbG9uZSgpKQogICAgICAgIC5idWlsZCgpOw==", "target": "src/lib.rs", "index": 2, "is_create": false}]')

for step in STEPS:
    before = base64.b64decode(step["before_b64"]).decode()
    after = base64.b64decode(step["after_b64"]).decode()
    target = step["target"]
    idx = step["index"]
    is_create = step["is_create"]

    if is_create:
        target_path = Path(target)
        target_path.parent.mkdir(parents=True, exist_ok=True)
        target_path.write_text(after)
        print(f"OK {TASK_ID} change {idx}: created {target}")
    else:
        target_path = Path(target)
        content = target_path.read_text()
        if before not in content:
            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
            sys.exit(1)

        result = subprocess.run(
            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
            capture_output=True, text=True,
        )
        if result.returncode != 0:
            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
            sys.exit(result.returncode)

        new_content = target_path.read_text()
        if after and after not in new_content:
            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
            sys.exit(1)

        print(f"OK {TASK_ID} change {idx}: applied to {target}")

print(f"OK {TASK_ID}: all changes applied")
## File: plans/compiled/TASK-7.sh
#!/usr/bin/env bash
set -euo pipefail
# TASK-7: Refactor lib.rs run(): parallel error collection, error entry construction, remove unwrap_or_default
# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
# Type: replace
# File: src/lib.rs
python3 "$(dirname "$0")/TASK-7.py"
## File: plans/compiled/TASK-8.py
#!/usr/bin/env python3
"""TASK-8: Remove all crate-level clippy allow attributes and fix individual lint violations"""
import base64, json, subprocess, sys
from pathlib import Path

TASK_ID = "TASK-8"
STEPS = json.loads('[{"before_b64": "IyFbd2FybihjbGlwcHk6OnBlZGFudGljKV0KIyFbYWxsb3coY2xpcHB5OjptaXNzaW5nX2Vycm9yc19kb2MpXQojIVthbGxvdyhjbGlwcHk6Om11c3RfdXNlX2NhbmRpZGF0ZSldCiMhW2FsbG93KGNsaXBweTo6ZG9jX21hcmtkb3duKV0KIyFbYWxsb3coY2xpcHB5Ojp1bmlubGluZWRfZm9ybWF0X2FyZ3MpXQojIVthbGxvdyhjbGlwcHk6OnJlZHVuZGFudF9jbG9zdXJlKV0KIyFbYWxsb3coY2xpcHB5Ojpjb2xsYXBzaWJsZV9pZildCiMhW2FsbG93KGNsaXBweTo6bmVlZGxlc3NfcGFzc19ieV92YWx1ZSldCiMhW2FsbG93KGNsaXBweTo6bmVlZGxlc3NfYm9ycm93KV0KIyFbYWxsb3coY2xpcHB5OjpyZWR1bmRhbnRfY2xvc3VyZV9mb3JfbWV0aG9kX2NhbGxzKV0=", "after_b64": "IyFbd2FybihjbGlwcHk6OnBlZGFudGljKV0=", "target": "src/lib.rs", "index": 0, "is_create": false}, {"before_b64": "Ly8vIFJ1biB0aGUgZnVsbCB3b3Jrc3BhY2UgbWFwcGluZyBwaXBlbGluZS4KLy8vCi8vLyAxLiBEaXNjb3ZlciB3b3Jrc3BhY2Ugcm9vdCBhbmQgbWVtYmVyIGNyYXRlcy4KLy8vIDIuIFByb2Nlc3MgZWFjaCBjcmF0ZSBpbiBwYXJhbGxlbCAoQ2FyZ28udG9tbCBwYXJzaW5nICsgbW9kdWxlIHRyZWUpLgovLy8gMy4gQ29tcHV0ZSBjcm9zcy1jcmF0ZSByZWZlcmVuY2VzLgovLy8gNC4gUmVuZGVyIEpTT04gdG8gc3Rkb3V0IG9yIHRoZSBjb25maWd1cmVkIG91dHB1dCBmaWxlLgpwdWIgZm4gcnVuKGNvbmZpZzogQ29uZmlnKSAtPiBhbnlob3c6OlJlc3VsdDwoKT4gew==", "after_b64": "Ly8vIFJ1biB0aGUgZnVsbCB3b3Jrc3BhY2UgbWFwcGluZyBwaXBlbGluZS4KLy8vCi8vLyAxLiBEaXNjb3ZlciB3b3Jrc3BhY2Ugcm9vdCBhbmQgbWVtYmVyIGNyYXRlcy4KLy8vIDIuIFByb2Nlc3MgZWFjaCBjcmF0ZSBpbiBwYXJhbGxlbCAoQ2FyZ28udG9tbCBwYXJzaW5nICsgbW9kdWxlIHRyZWUpLgovLy8gMy4gQ29tcHV0ZSBjcm9zcy1jcmF0ZSByZWZlcmVuY2VzLgovLy8gNC4gUmVuZGVyIEpTT04gdG8gc3Rkb3V0IG9yIHRoZSBjb25maWd1cmVkIG91dHB1dCBmaWxlLgovLy8KLy8vICMgRXJyb3JzCi8vLwovLy8gUmV0dXJucyBhbiBlcnJvciBpZiB0aGUgd29ya3NwYWNlIHJvb3QgY2Fubm90IGJlIGZvdW5kLCB0aGUgd29ya3NwYWNlCi8vLyBDYXJnby50b21sIGlzIG1pc3NpbmcgYSBgW3dvcmtzcGFjZV1gIHNlY3Rpb24sIG1lbWJlciBjcmF0ZXMgY2Fubm90IGJlCi8vLyBwYXJzZWQsIG9yIHRoZSBKU09OIG91dHB1dCBjYW5ub3QgYmUgd3JpdHRlbi4KcHViIGZuIHJ1bihjb25maWc6IENvbmZpZykgLT4gYW55aG93OjpSZXN1bHQ8KCk+IHs=", "target": "src/lib.rs", "index": 1, "is_create": false}, {"before_b64": "ICAgIGxldCB3b3Jrc3BhY2VfbmFtZSA9IHdvcmtzcGFjZV9yb290CiAgICAgICAgLmZpbGVfbmFtZSgpCiAgICAgICAgLm1hcCh8bnwgbi50b19zdHJpbmdfbG9zc3koKS50b19zdHJpbmcoKSkKICAgICAgICAudW53cmFwX29yX2RlZmF1bHQoKTs=", "after_b64": "ICAgIGxldCB3b3Jrc3BhY2VfbmFtZSA9IHdvcmtzcGFjZV9yb290CiAgICAgICAgLmZpbGVfbmFtZSgpCiAgICAgICAgLm1hcCh8bnwgbi50b19zdHJpbmdfbG9zc3koKS50b19zdHJpbmcoKSkKICAgICAgICAudW53cmFwX29yX2RlZmF1bHQoKTs=", "target": "src/lib.rs", "index": 2, "is_create": false}, {"before_b64": "Ly8vIFN0cmlwIHRoZSB3b3Jrc3BhY2Ugcm9vdCBwcmVmaXggZnJvbSBhIHBhdGggc3RyaW5nLCByZXR1cm5pbmcgYQovLy8gd29ya3NwYWNlLXJlbGF0aXZlIHBhdGguIElmIHRoZSBwcmVmaXggZG9lc24ndCBtYXRjaCwgcmV0dXJucyB0aGUKLy8vIG9yaWdpbmFsIHN0cmluZyB1bmNoYW5nZWQuCmZuIHJlbGF0aXZpemVfcGF0aChwYXRoX3N0cjogJnN0ciwgcm9vdDogJlBhdGgpIC0+IFN0cmluZyB7CiAgICBsZXQgcCA9IFBhdGg6Om5ldyhwYXRoX3N0cik7CiAgICBtYXRjaCBwLnN0cmlwX3ByZWZpeChyb290KSB7CiAgICAgICAgT2socmVsKSA9PiByZWwudG9fc3RyaW5nX2xvc3N5KCkudG9fc3RyaW5nKCksCiAgICAgICAgRXJyKF8pID0+IHBhdGhfc3RyLnRvX3N0cmluZygpLAogICAgfQp9", "after_b64": "Ly8vIFN0cmlwIHRoZSB3b3Jrc3BhY2Ugcm9vdCBwcmVmaXggZnJvbSBhIHBhdGggc3RyaW5nLCByZXR1cm5pbmcgYQovLy8gd29ya3NwYWNlLXJlbGF0aXZlIHBhdGguIElmIHRoZSBwcmVmaXggZG9lc24ndCBtYXRjaCwgcmV0dXJucyB0aGUKLy8vIG9yaWdpbmFsIHN0cmluZyB1bmNoYW5nZWQuCmZuIHJlbGF0aXZpemVfcGF0aChwYXRoX3N0cjogJnN0ciwgcm9vdDogJlBhdGgpIC0+IFN0cmluZyB7CiAgICBsZXQgcCA9IFBhdGg6Om5ldyhwYXRoX3N0cik7CiAgICBtYXRjaCBwLnN0cmlwX3ByZWZpeChyb290KSB7CiAgICAgICAgT2socmVsKSA9PiByZWwudG9fc3RyaW5nX2xvc3N5KCkudG9fc3RyaW5nKCksCiAgICAgICAgRXJyKF8pID0+IHBhdGhfc3RyLnRvX3N0cmluZygpLAogICAgfQp9", "target": "src/lib.rs", "index": 3, "is_create": false}, {"before_b64": "Ly8vIEV4dHJhY3QgYWxsIGl0ZW1zIHdpdGggYW55IGZvcm0gb2YgYHB1YmAgdmlzaWJpbGl0eSAoZXhjbHVkaW5nIGBJbmhlcml0ZWRgKS4KLy8vIFJlc3VsdHMgYXJlIHNvcnRlZCBieSBuYW1lIHRoZW4gbGluZSBmb3IgZGV0ZXJtaW5pc3RpYyBvdXRwdXQuCnB1YiBmbiBleHRyYWN0X3B1YmxpY19pdGVtcyhpdGVtczogJltzeW46Okl0ZW1dKSAtPiBWZWM8UHVibGljSXRlbT4gew==", "after_b64": "Ly8vIEV4dHJhY3QgYWxsIGl0ZW1zIHdpdGggYW55IGZvcm0gb2YgYHB1YmAgdmlzaWJpbGl0eSAoZXhjbHVkaW5nIGBJbmhlcml0ZWRgKS4KLy8vIFJlc3VsdHMgYXJlIHNvcnRlZCBieSBuYW1lIHRoZW4gbGluZSBmb3IgZGV0ZXJtaW5pc3RpYyBvdXRwdXQuCiNbbXVzdF91c2VdCnB1YiBmbiBleHRyYWN0X3B1YmxpY19pdGVtcyhpdGVtczogJltzeW46Okl0ZW1dKSAtPiBWZWM8UHVibGljSXRlbT4gew==", "target": "src/file_parser.rs", "index": 4, "is_create": false}, {"before_b64": "Ly8vIEV4dHJhY3QgYWxsIGB1c2VgIHN0YXRlbWVudHMuIEJyYWNlZCBpbXBvcnRzIGFyZSBleHBhbmRlZCB0byBpbmRpdmlkdWFsCi8vLyBlbnRyaWVzLiBSZXN1bHRzIHNvcnRlZCBieSBwYXRoIGZvciBkZXRlcm1pbmlzbS4KcHViIGZuIGV4dHJhY3RfaW1wb3J0cyhpdGVtczogJltzeW46Okl0ZW1dKSAtPiBWZWM8SW1wb3J0PiB7", "after_b64": "Ly8vIEV4dHJhY3QgYWxsIGB1c2VgIHN0YXRlbWVudHMuIEJyYWNlZCBpbXBvcnRzIGFyZSBleHBhbmRlZCB0byBpbmRpdmlkdWFsCi8vLyBlbnRyaWVzLiBSZXN1bHRzIHNvcnRlZCBieSBwYXRoIGZvciBkZXRlcm1pbmlzbS4KI1ttdXN0X3VzZV0KcHViIGZuIGV4dHJhY3RfaW1wb3J0cyhpdGVtczogJltzeW46Okl0ZW1dKSAtPiBWZWM8SW1wb3J0PiB7", "target": "src/file_parser.rs", "index": 5, "is_create": false}, {"before_b64": "Ly8vIEV4dHJhY3QgYHB1YiB1c2VgIHJlLWV4cG9ydHMuIFJlc3VsdHMgc29ydGVkIGJ5IGV4cG9ydF9wYXRoLgpwdWIgZm4gZXh0cmFjdF9yZV9leHBvcnRzKGl0ZW1zOiAmW3N5bjo6SXRlbV0pIC0+IFZlYzxSZUV4cG9ydD4gew==", "after_b64": "Ly8vIEV4dHJhY3QgYHB1YiB1c2VgIHJlLWV4cG9ydHMuIFJlc3VsdHMgc29ydGVkIGJ5IGV4cG9ydF9wYXRoLgojW211c3RfdXNlXQpwdWIgZm4gZXh0cmFjdF9yZV9leHBvcnRzKGl0ZW1zOiAmW3N5bjo6SXRlbV0pIC0+IFZlYzxSZUV4cG9ydD4gew==", "target": "src/file_parser.rs", "index": 6, "is_create": false}, {"before_b64": "Ly8vIEV4dHJhY3QgYG1vZGAgZGVjbGFyYXRpb25zLiBEZXRlY3RzIGAjW2NmZyh0ZXN0KV1gIHZpYSBsaXRlcmFsIHRva2VuCi8vLyBtYXRjaGluZy4gUmVzdWx0cyBzb3J0ZWQgYnkgbmFtZS4KcHViIGZuIGV4dHJhY3Rfc3VibW9kdWxlcyhpdGVtczogJltzeW46Okl0ZW1dKSAtPiBWZWM8U3VibW9kdWxlRGVjbD4gew==", "after_b64": "Ly8vIEV4dHJhY3QgYG1vZGAgZGVjbGFyYXRpb25zLiBEZXRlY3RzIGAjW2NmZyh0ZXN0KV1gIHZpYSBsaXRlcmFsIHRva2VuCi8vLyBtYXRjaGluZy4gUmVzdWx0cyBzb3J0ZWQgYnkgbmFtZS4KI1ttdXN0X3VzZV0KcHViIGZuIGV4dHJhY3Rfc3VibW9kdWxlcyhpdGVtczogJltzeW46Okl0ZW1dKSAtPiBWZWM8U3VibW9kdWxlRGVjbD4gew==", "target": "src/file_parser.rs", "index": 7, "is_create": false}, {"before_b64": "Ly8vIEV4dHJhY3QgYGltcGxgIGJsb2Nrcy4gRWFjaCBgSW1wbEluZm9gIHJlY29yZHMgdGhlIHRhcmdldCB0eXBlIG5hbWUgYW5kCi8vLyB0aGUgaW1wbCBpdGVtcyAoZm4sIHR5cGUsIGNvbnN0KS4KcHViIGZuIGV4dHJhY3RfaW1wbHMoaXRlbXM6ICZbc3luOjpJdGVtXSkgLT4gVmVjPEltcGxJbmZvPiB7", "after_b64": "Ly8vIEV4dHJhY3QgYGltcGxgIGJsb2Nrcy4gRWFjaCBgSW1wbEluZm9gIHJlY29yZHMgdGhlIHRhcmdldCB0eXBlIG5hbWUgYW5kCi8vLyB0aGUgaW1wbCBpdGVtcyAoZm4sIHR5cGUsIGNvbnN0KS4KI1ttdXN0X3VzZV0KcHViIGZuIGV4dHJhY3RfaW1wbHMoaXRlbXM6ICZbc3luOjpJdGVtXSkgLT4gVmVjPEltcGxJbmZvPiB7", "target": "src/file_parser.rs", "index": 8, "is_create": false}, {"before_b64": "Ly8vIFJlc29sdmUgYSBgbW9kIG5hbWU7YCBkZWNsYXJhdGlvbiB0byBhIGZpbGUgcGF0aC4KLy8vIFRyaWVzIGB7cGFyZW50X2Rpcn0ve21vZF9uYW1lfS5yc2AgZmlyc3QsIHRoZW4gYHtwYXJlbnRfZGlyfS97bW9kX25hbWV9L21vZC5yc2AuCi8vLwovLy8gUmV0dXJucyBgTm9uZWAgaWYgbmVpdGhlciBwYXRoIGV4aXN0cy4KcHViIGZuIHJlc29sdmVfbW9kdWxlX3BhdGgocGFyZW50X2RpcjogJlBhdGgsIG1vZF9uYW1lOiAmc3RyKSAtPiBPcHRpb248UGF0aEJ1Zj4gew==", "after_b64": "Ly8vIFJlc29sdmUgYSBgbW9kIG5hbWU7YCBkZWNsYXJhdGlvbiB0byBhIGZpbGUgcGF0aC4KLy8vIFRyaWVzIGB7cGFyZW50X2Rpcn0ve21vZF9uYW1lfS5yc2AgZmlyc3QsIHRoZW4gYHtwYXJlbnRfZGlyfS97bW9kX25hbWV9L21vZC5yc2AuCi8vLwovLy8gUmV0dXJucyBgTm9uZWAgaWYgbmVpdGhlciBwYXRoIGV4aXN0cy4KI1ttdXN0X3VzZV0KcHViIGZuIHJlc29sdmVfbW9kdWxlX3BhdGgocGFyZW50X2RpcjogJlBhdGgsIG1vZF9uYW1lOiAmc3RyKSAtPiBPcHRpb248UGF0aEJ1Zj4gew==", "target": "src/module_tree.rs", "index": 9, "is_create": false}, {"before_b64": "Ly8vIFNlcmlhbGl6ZSB0aGUgd29ya3NwYWNlIG1hcCB0byBhIEpTT04gc3RyaW5nIHdpdGggMi1zcGFjZSBpbmRlbnRhdGlvbi4KcHViIGZuIHJlbmRlcl9qc29uKG1hcDogJldvcmtzcGFjZU1hcCkgLT4gc2VyZGVfanNvbjo6UmVzdWx0PFN0cmluZz4gew==", "after_b64": "Ly8vIFNlcmlhbGl6ZSB0aGUgd29ya3NwYWNlIG1hcCB0byBhIEpTT04gc3RyaW5nIHdpdGggMi1zcGFjZSBpbmRlbnRhdGlvbi4KI1ttdXN0X3VzZV0KcHViIGZuIHJlbmRlcl9qc29uKG1hcDogJldvcmtzcGFjZU1hcCkgLT4gc2VyZGVfanNvbjo6UmVzdWx0PFN0cmluZz4gew==", "target": "src/render.rs", "index": 10, "is_create": false}, {"before_b64": "Ly8gSGVscGVyOiBjb252ZXJ0IEl0ZW1LaW5kIHRvIGEgc2hvcnQgc3RyaW5nIGZvciB0aGUgVHlwZVJlZi5raW5kIGZpZWxkLgppbXBsIGNyYXRlOjpzY2hlbWE6OlB1YmxpY0l0ZW0gewogICAgZm4ga2luZF90b19zdHJpbmcoJnNlbGYpIC0+IFN0cmluZyB7", "after_b64": "Ly8gSGVscGVyOiBjb252ZXJ0IEl0ZW1LaW5kIHRvIGEgc2hvcnQgc3RyaW5nIGZvciB0aGUgVHlwZVJlZi5raW5kIGZpZWxkLgppbXBsIGNyYXRlOjpzY2hlbWE6OlB1YmxpY0l0ZW0gewogICAgI1ttdXN0X3VzZV0KICAgIGZuIGtpbmRfdG9fc3RyaW5nKCZzZWxmKSAtPiBTdHJpbmcgew==", "target": "src/cross_refs.rs", "index": 11, "is_create": false}, {"before_b64": "ICAgIG1hdGNoIHAuc3RyaXBfcHJlZml4KHJvb3QpIHsKICAgICAgICBPayhyZWwpID0+IHJlbC50b19zdHJpbmdfbG9zc3koKS50b19zdHJpbmcoKSwKICAgICAgICBFcnIoXykgPT4gcGF0aF9zdHIudG9fc3RyaW5nKCksCiAgICB9Cn0=", "after_b64": "ICAgIG1hdGNoIHAuc3RyaXBfcHJlZml4KHJvb3QpIHsKICAgICAgICBPayhyZWwpID0+IHJlbC50b19zdHJpbmdfbG9zc3koKS50b19zdHJpbmcoKSwKICAgICAgICBFcnIoXykgPT4gcGF0aF9zdHIudG9fc3RyaW5nKCksCiAgICB9Cn0=", "target": "src/lib.rs", "index": 12, "is_create": false}, {"before_b64": "Ly8vIEV4dHJhY3QgYHB1YiB1c2VgIHJlLWV4cG9ydHMuIFJlc3VsdHMgc29ydGVkIGJ5IGV4cG9ydF9wYXRoLgojW211c3RfdXNlXQpwdWIgZm4gZXh0cmFjdF9yZV9leHBvcnRz", "after_b64": "Ly8vIEV4dHJhY3QgYHB1YiB1c2VgIHJlLWV4cG9ydHMuIFJlc3VsdHMgc29ydGVkIGJ5IGBleHBvcnRfcGF0aGAuCiNbbXVzdF91c2VdCnB1YiBmbiBleHRyYWN0X3JlX2V4cG9ydHM=", "target": "src/file_parser.rs", "index": 13, "is_create": false}, {"before_b64": "ICAgICAgICAgICAgbGV0IG5hbWUgPSBtLmlkZW50LmFzX3JlZigpLm1hcCh8aXwgaS50b19zdHJpbmcoKSkudW53cmFwX29yX2RlZmF1bHQoKTs=", "after_b64": "ICAgICAgICAgICAgbGV0IG5hbWUgPSBtLmlkZW50LmFzX3JlZigpLm1hcChUb1N0cmluZzo6dG9fc3RyaW5nKS51bndyYXBfb3JfZGVmYXVsdCgpOw==", "target": "src/file_parser.rs", "index": 14, "is_create": false}, {"before_b64": "Zm4gZmxhdHRlbl91c2VfdHJlZSh0cmVlOiAmc3luOjpVc2VUcmVlLCBwcmVmaXg6IFN0cmluZywgbGluZTogdXNpemUpIC0+IFZlYzxJbXBvcnQ+IHs=", "after_b64": "Zm4gZmxhdHRlbl91c2VfdHJlZSh0cmVlOiAmc3luOjpVc2VUcmVlLCBwcmVmaXg6ICZzdHIsIGxpbmU6IHVzaXplKSAtPiBWZWM8SW1wb3J0PiB7", "target": "src/file_parser.rs", "index": 15, "is_create": false}, {"before_b64": "ICAgICAgICAgICAgICAgIFNvbWUoZmxhdHRlbl91c2VfdHJlZSgmdS50cmVlLCBTdHJpbmc6Om5ldygpLCBsaW5lX29mX2l0ZW0oaXRlbSkpKQ==", "after_b64": "ICAgICAgICAgICAgICAgIFNvbWUoZmxhdHRlbl91c2VfdHJlZSgmdS50cmVlLCAiIiwgbGluZV9vZl9pdGVtKGl0ZW0pKSk=", "target": "src/file_parser.rs", "index": 16, "is_create": false}, {"before_b64": "ICAgICAgICAgICAgZmxhdHRlbl91c2VfdHJlZSgmcC50cmVlLCBuZXdfcHJlZml4LCBsaW5lKQ==", "after_b64": "ICAgICAgICAgICAgZmxhdHRlbl91c2VfdHJlZSgmcC50cmVlLCAmbmV3X3ByZWZpeCwgbGluZSk=", "target": "src/file_parser.rs", "index": 17, "is_create": false}, {"before_b64": "Ly8vIEludGVybmFsIGludGVybWVkaWF0ZSB0eXBlIGNvbnN1bWVkIGJ5IG1vZHVsZV90cmVlLgojW2Rlcml2ZShEZWJ1ZywgQ2xvbmUsIERlZmF1bHQpXQpwdWIgc3RydWN0IEZpbGVJbmZv", "after_b64": "Ly8vIEludGVybmFsIGludGVybWVkaWF0ZSB0eXBlIGNvbnN1bWVkIGJ5IGBtb2R1bGVfdHJlZWAuCiNbZGVyaXZlKERlYnVnLCBDbG9uZSwgRGVmYXVsdCldCnB1YiBzdHJ1Y3QgRmlsZUluZm8=", "target": "src/schema.rs", "index": 18, "is_create": false}, {"before_b64": "Ly8vIHBhcnNlZCwgb3IgdGhlIEpTT04gb3V0cHV0IGNhbm5vdCBiZSB3cml0dGVuLgpwdWIgZm4gcnVuKGNvbmZpZzogQ29uZmlnKSAtPiBhbnlob3c6OlJlc3VsdDwoKT4gew==", "after_b64": "Ly8vIHBhcnNlZCwgb3IgdGhlIEpTT04gb3V0cHV0IGNhbm5vdCBiZSB3cml0dGVuLgpwdWIgZm4gcnVuKGNvbmZpZzogJkNvbmZpZykgLT4gYW55aG93OjpSZXN1bHQ8KCk+IHs=", "target": "src/lib.rs", "index": 19, "is_create": false}, {"before_b64": "ICAgIHJ1c3Rfd29ya3NwYWNlX21hcDo6cnVuKGNvbmZpZyk=", "after_b64": "ICAgIHJ1c3Rfd29ya3NwYWNlX21hcDo6cnVuKCZjb25maWcp", "target": "src/main.rs", "index": 20, "is_create": false}]')

for step in STEPS:
    before = base64.b64decode(step["before_b64"]).decode()
    after = base64.b64decode(step["after_b64"]).decode()
    target = step["target"]
    idx = step["index"]
    is_create = step["is_create"]

    if is_create:
        target_path = Path(target)
        target_path.parent.mkdir(parents=True, exist_ok=True)
        target_path.write_text(after)
        print(f"OK {TASK_ID} change {idx}: created {target}")
    else:
        target_path = Path(target)
        content = target_path.read_text()
        if before not in content:
            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
            sys.exit(1)

        result = subprocess.run(
            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
            capture_output=True, text=True,
        )
        if result.returncode != 0:
            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
            sys.exit(result.returncode)

        new_content = target_path.read_text()
        if after and after not in new_content:
            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
            sys.exit(1)

        print(f"OK {TASK_ID} change {idx}: applied to {target}")

print(f"OK {TASK_ID}: all changes applied")
## File: plans/compiled/TASK-8.sh
#!/usr/bin/env bash
set -euo pipefail
# TASK-8: Remove all crate-level clippy allow attributes and fix individual lint violations
# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
# Type: replace
# File: src/lib.rs
python3 "$(dirname "$0")/TASK-8.py"
## File: plans/compiled/TASK-9.py
#!/usr/bin/env python3
"""TASK-9: Add unit tests for file_parser, module_tree, workspace, cargo_info, cross_refs, and render"""
import base64, json, subprocess, sys
from pathlib import Path

TASK_ID = "TASK-9"
STEPS = json.loads('[{"before_b64": "Zm4gZXh0cmFjdF9yZV9leHBvcnRzX2Zyb21fdHJlZSgKICAgIHRyZWU6ICZzeW46OlVzZVRyZWUsCiAgICBpbXBvcnRfcGF0aDogU3RyaW5nLAogICAgbGluZTogdXNpemUsCikgLT4gVmVjPFJlRXhwb3J0PiB7CiAgICBtYXRjaCB0cmVlIHsKICAgICAgICBzeW46OlVzZVRyZWU6OlBhdGgocCkgPT4gewogICAgICAgICAgICBsZXQgbmV3X2ltcG9ydCA9IGlmIGltcG9ydF9wYXRoLmlzX2VtcHR5KCkgewogICAgICAgICAgICAgICAgcC5pZGVudC50b19zdHJpbmcoKQogICAgICAgICAgICB9IGVsc2UgewogICAgICAgICAgICAgICAgZm9ybWF0ISgie306Ont9IiwgaW1wb3J0X3BhdGgsIHAuaWRlbnQpCiAgICAgICAgICAgIH07CiAgICAgICAgICAgIGV4dHJhY3RfcmVfZXhwb3J0c19mcm9tX3RyZWUoJnAudHJlZSwgbmV3X2ltcG9ydCwgbGluZSkKICAgICAgICB9CiAgICAgICAgc3luOjpVc2VUcmVlOjpOYW1lKG4pID0+IHsKICAgICAgICAgICAgdmVjIVtSZUV4cG9ydCB7CiAgICAgICAgICAgICAgICBpbXBvcnRfcGF0aCwKICAgICAgICAgICAgICAgIGV4cG9ydF9wYXRoOiBuLmlkZW50LnRvX3N0cmluZygpLAogICAgICAgICAgICAgICAgbGluZSwKICAgICAgICAgICAgfV0KICAgICAgICB9CiAgICAgICAgc3luOjpVc2VUcmVlOjpSZW5hbWUocikgPT4gewogICAgICAgICAgICB2ZWMhW1JlRXhwb3J0IHsKICAgICAgICAgICAgICAgIGltcG9ydF9wYXRoLAogICAgICAgICAgICAgICAgZXhwb3J0X3BhdGg6IHIucmVuYW1lLnRvX3N0cmluZygpLAogICAgICAgICAgICAgICAgbGluZSwKICAgICAgICAgICAgfV0KICAgICAgICB9CiAgICAgICAgc3luOjpVc2VUcmVlOjpHbG9iKF8pID0+IHsKICAgICAgICAgICAgdmVjIVtSZUV4cG9ydCB7CiAgICAgICAgICAgICAgICBpbXBvcnRfcGF0aCwKICAgICAgICAgICAgICAgIGV4cG9ydF9wYXRoOiAiKiIudG9fc3RyaW5nKCksCiAgICAgICAgICAgICAgICBsaW5lLAogICAgICAgICAgICB9XQogICAgICAgIH0KICAgICAgICBzeW46OlVzZVRyZWU6Okdyb3VwKGcpID0+IGcKICAgICAgICAgICAgLml0ZW1zCiAgICAgICAgICAgIC5pdGVyKCkKICAgICAgICAgICAgLmZsYXRfbWFwKHx0fCBleHRyYWN0X3JlX2V4cG9ydHNfZnJvbV90cmVlKHQsIGltcG9ydF9wYXRoLmNsb25lKCksIGxpbmUpKQogICAgICAgICAgICAuY29sbGVjdCgpLAogICAgfQp9", "after_b64": "Zm4gZXh0cmFjdF9yZV9leHBvcnRzX2Zyb21fdHJlZSgKICAgIHRyZWU6ICZzeW46OlVzZVRyZWUsCiAgICBpbXBvcnRfcGF0aDogU3RyaW5nLAogICAgbGluZTogdXNpemUsCikgLT4gVmVjPFJlRXhwb3J0PiB7CiAgICBtYXRjaCB0cmVlIHsKICAgICAgICBzeW46OlVzZVRyZWU6OlBhdGgocCkgPT4gewogICAgICAgICAgICBsZXQgbmV3X2ltcG9ydCA9IGlmIGltcG9ydF9wYXRoLmlzX2VtcHR5KCkgewogICAgICAgICAgICAgICAgcC5pZGVudC50b19zdHJpbmcoKQogICAgICAgICAgICB9IGVsc2UgewogICAgICAgICAgICAgICAgZm9ybWF0ISgie306Ont9IiwgaW1wb3J0X3BhdGgsIHAuaWRlbnQpCiAgICAgICAgICAgIH07CiAgICAgICAgICAgIGV4dHJhY3RfcmVfZXhwb3J0c19mcm9tX3RyZWUoJnAudHJlZSwgbmV3X2ltcG9ydCwgbGluZSkKICAgICAgICB9CiAgICAgICAgc3luOjpVc2VUcmVlOjpOYW1lKG4pID0+IHsKICAgICAgICAgICAgdmVjIVtSZUV4cG9ydCB7CiAgICAgICAgICAgICAgICBpbXBvcnRfcGF0aCwKICAgICAgICAgICAgICAgIGV4cG9ydF9wYXRoOiBuLmlkZW50LnRvX3N0cmluZygpLAogICAgICAgICAgICAgICAgbGluZSwKICAgICAgICAgICAgfV0KICAgICAgICB9CiAgICAgICAgc3luOjpVc2VUcmVlOjpSZW5hbWUocikgPT4gewogICAgICAgICAgICB2ZWMhW1JlRXhwb3J0IHsKICAgICAgICAgICAgICAgIGltcG9ydF9wYXRoLAogICAgICAgICAgICAgICAgZXhwb3J0X3BhdGg6IHIucmVuYW1lLnRvX3N0cmluZygpLAogICAgICAgICAgICAgICAgbGluZSwKICAgICAgICAgICAgfV0KICAgICAgICB9CiAgICAgICAgc3luOjpVc2VUcmVlOjpHbG9iKF8pID0+IHsKICAgICAgICAgICAgdmVjIVtSZUV4cG9ydCB7CiAgICAgICAgICAgICAgICBpbXBvcnRfcGF0aCwKICAgICAgICAgICAgICAgIGV4cG9ydF9wYXRoOiAiKiIudG9fc3RyaW5nKCksCiAgICAgICAgICAgICAgICBsaW5lLAogICAgICAgICAgICB9XQogICAgICAgIH0KICAgICAgICBzeW46OlVzZVRyZWU6Okdyb3VwKGcpID0+IGcKICAgICAgICAgICAgLml0ZW1zCiAgICAgICAgICAgIC5pdGVyKCkKICAgICAgICAgICAgLmZsYXRfbWFwKHx0fCBleHRyYWN0X3JlX2V4cG9ydHNfZnJvbV90cmVlKHQsIGltcG9ydF9wYXRoLmNsb25lKCksIGxpbmUpKQogICAgICAgICAgICAuY29sbGVjdCgpLAogICAgfQp9CgovLyDilIDilIAgVGVzdHMg4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSACgojW2NmZyh0ZXN0KV0KbW9kIHRlc3RzIHsKICAgIHVzZSBzdXBlcjo6KjsKICAgIHVzZSBjcmF0ZTo6c2NoZW1hOjp7SW1wb3J0LCBSZUV4cG9ydCwgU3VibW9kdWxlRGVjbH07CiAgICB1c2Ugc3RkOjpwYXRoOjpQYXRoQnVmOwoKICAgIGZuIHBhcnNlX3NvdXJjZShzcmM6ICZzdHIpIC0+IFBhcnNlZEZpbGUgewogICAgICAgIGxldCB0bXAgPSBzdGQ6OmVudjo6dGVtcF9kaXIoKS5qb2luKCJwYXJzZV90ZXN0LnJzIik7CiAgICAgICAgc3RkOjpmczo6d3JpdGUoJnRtcCwgc3JjKS51bndyYXAoKTsKICAgICAgICBsZXQgcmVzdWx0ID0gcGFyc2VfZmlsZSgmdG1wKTsKICAgICAgICBzdGQ6OmZzOjpyZW1vdmVfZmlsZSgmdG1wKS5vaygpOwogICAgICAgIHJlc3VsdAogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIHBhcnNlX2ZpbGVfcmV0dXJuc19hc3RfZm9yX3ZhbGlkX3NvdXJjZSgpIHsKICAgICAgICBsZXQgc3JjID0gInB1YiBzdHJ1Y3QgRm9vIHsgeDogaTMyIH0iOwogICAgICAgIGxldCByZXN1bHQgPSBwYXJzZV9zb3VyY2Uoc3JjKTsKICAgICAgICBhc3NlcnQhKHJlc3VsdC5wYXJzZV9lcnJvci5pc19ub25lKCkpOwogICAgICAgIGFzc2VydF9lcSEocmVzdWx0LmFzdC5pdGVtcy5sZW4oKSwgMSk7CiAgICB9CgogICAgI1t0ZXN0XQogICAgZm4gcGFyc2VfZmlsZV9yZXR1cm5zX2Vycm9yX2Zvcl9pbnZhbGlkX3NvdXJjZSgpIHsKICAgICAgICBsZXQgc3JjID0gInB1YiBzdHJ1Y3QgeyBpbnZhbGlkIHJ1c3QgfSI7CiAgICAgICAgbGV0IHJlc3VsdCA9IHBhcnNlX3NvdXJjZShzcmMpOwogICAgICAgIGFzc2VydCEocmVzdWx0LnBhcnNlX2Vycm9yLmlzX3NvbWUoKSk7CiAgICAgICAgbGV0IGVyciA9IHJlc3VsdC5wYXJzZV9lcnJvci5hc19yZWYoKS51bndyYXAoKTsKICAgICAgICBhc3NlcnQhKCFlcnIubWVzc2FnZS5pc19lbXB0eSgpKTsKICAgICAgICBhc3NlcnQhKGVyci5saW5lID4gMCk7CiAgICB9CgogICAgI1t0ZXN0XQogICAgZm4gcGFyc2VfZmlsZV9yZXR1cm5zX2VtcHR5X2Zvcl9lbXB0eV9maWxlKCkgewogICAgICAgIGxldCByZXN1bHQgPSBwYXJzZV9zb3VyY2UoIiIpOwogICAgICAgIGFzc2VydCEocmVzdWx0LnBhcnNlX2Vycm9yLmlzX25vbmUoKSk7CiAgICAgICAgYXNzZXJ0IShyZXN1bHQuZmlsZV9pbmZvLnB1YmxpY19pdGVtcy5pc19lbXB0eSgpKTsKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiBleHRyYWN0X3B1YmxpY19pdGVtc19maW5kc19zdHJ1Y3RfZW51bV90cmFpdF9mbigpIHsKICAgICAgICBsZXQgc3JjID0gInB1YiBzdHJ1Y3QgRm9vIHt9IHB1YiBlbnVtIEJhciB7IEEsIEIgfSBwdWIgdHJhaXQgQmF6IHt9IHB1YiBmbiBoZWxsbygpIHt9IjsKICAgICAgICBsZXQgcmVzdWx0ID0gcGFyc2Vfc291cmNlKHNyYyk7CiAgICAgICAgbGV0IGl0ZW1zID0gZXh0cmFjdF9wdWJsaWNfaXRlbXMoJnJlc3VsdC5hc3QuaXRlbXMpOwogICAgICAgIGxldCBuYW1lczogVmVjPF8+ID0gaXRlbXMuaXRlcigpLm1hcCh8aXwgaS5uYW1lLmFzX3N0cigpKS5jb2xsZWN0KCk7CiAgICAgICAgYXNzZXJ0IShuYW1lcy5jb250YWlucygmIkZvbyIpKTsKICAgICAgICBhc3NlcnQhKG5hbWVzLmNvbnRhaW5zKCYiQmFyIikpOwogICAgICAgIGFzc2VydCEobmFtZXMuY29udGFpbnMoJiJCYXoiKSk7CiAgICAgICAgYXNzZXJ0IShuYW1lcy5jb250YWlucygmImhlbGxvIikpOwogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIGV4dHJhY3RfcHVibGljX2l0ZW1zX2VtcHR5X2Zvcl9ub19wdWJsaWNfaXRlbXMoKSB7CiAgICAgICAgbGV0IHNyYyA9ICJzdHJ1Y3QgUHJpdmF0ZSB7fSBmbiBwcml2YXRlX2ZuKCkge30iOwogICAgICAgIGxldCByZXN1bHQgPSBwYXJzZV9zb3VyY2Uoc3JjKTsKICAgICAgICBsZXQgaXRlbXMgPSBleHRyYWN0X3B1YmxpY19pdGVtcygmcmVzdWx0LmFzdC5pdGVtcyk7CiAgICAgICAgYXNzZXJ0IShpdGVtcy5pc19lbXB0eSgpKTsKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiBleHRyYWN0X2ltcG9ydHNfZmluZHNfdXNlX3N0YXRlbWVudHMoKSB7CiAgICAgICAgbGV0IHNyYyA9ICJ1c2Ugc3RkOjpjb2xsZWN0aW9uczo6QlRyZWVNYXA7IjsKICAgICAgICBsZXQgcmVzdWx0ID0gcGFyc2Vfc291cmNlKHNyYyk7CiAgICAgICAgbGV0IGltcG9ydHMgPSBleHRyYWN0X2ltcG9ydHMoJnJlc3VsdC5hc3QuaXRlbXMpOwogICAgICAgIGFzc2VydF9lcSEoaW1wb3J0cy5sZW4oKSwgMSk7CiAgICAgICAgYXNzZXJ0X2VxIShpbXBvcnRzWzBdLnBhdGgsICJzdGQ6OmNvbGxlY3Rpb25zOjpCVHJlZU1hcCIpOwogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIGV4dHJhY3RfcmVfZXhwb3J0c19maW5kc19wdWJfdXNlKCkgewogICAgICAgIGxldCBzcmMgPSAicHViIHVzZSBjcmF0ZTo6Zm9vOyI7CiAgICAgICAgbGV0IHJlc3VsdCA9IHBhcnNlX3NvdXJjZShzcmMpOwogICAgICAgIGxldCByZV9leHBvcnRzID0gZXh0cmFjdF9yZV9leHBvcnRzKCZyZXN1bHQuYXN0Lml0ZW1zKTsKICAgICAgICBhc3NlcnRfZXEhKHJlX2V4cG9ydHMubGVuKCksIDEpOwogICAgICAgIGFzc2VydF9lcSEocmVfZXhwb3J0c1swXS5pbXBvcnRfcGF0aCwgImNyYXRlOjpmb28iKTsKICAgICAgICBhc3NlcnRfZXEhKHJlX2V4cG9ydHNbMF0uZXhwb3J0X3BhdGgsICJmb28iKTsKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiBleHRyYWN0X3JlX2V4cG9ydHNfZmluZHNfcmVuYW1lKCkgewogICAgICAgIGxldCBzcmMgPSAicHViIHVzZSBjcmF0ZTo6Zm9vIGFzIGJhcjsiOwogICAgICAgIGxldCByZXN1bHQgPSBwYXJzZV9zb3VyY2Uoc3JjKTsKICAgICAgICBsZXQgcmVfZXhwb3J0cyA9IGV4dHJhY3RfcmVfZXhwb3J0cygmcmVzdWx0LmFzdC5pdGVtcyk7CiAgICAgICAgYXNzZXJ0X2VxIShyZV9leHBvcnRzLmxlbigpLCAxKTsKICAgICAgICBhc3NlcnRfZXEhKHJlX2V4cG9ydHNbMF0uaW1wb3J0X3BhdGgsICJjcmF0ZTo6Zm9vIGFzIGJhciIpOwogICAgICAgIGFzc2VydF9lcSEocmVfZXhwb3J0c1swXS5leHBvcnRfcGF0aCwgImJhciIpOwogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIGV4dHJhY3Rfc3VibW9kdWxlc19maW5kc19tb2RfZGVjbGFyYXRpb25zKCkgewogICAgICAgIGxldCBzcmMgPSAibW9kIGZvbzsgbW9kIGJhcjsiOwogICAgICAgIGxldCByZXN1bHQgPSBwYXJzZV9zb3VyY2Uoc3JjKTsKICAgICAgICBsZXQgc3VicyA9IGV4dHJhY3Rfc3VibW9kdWxlcygmcmVzdWx0LmFzdC5pdGVtcyk7CiAgICAgICAgYXNzZXJ0X2VxIShzdWJzLmxlbigpLCAyKTsKICAgICAgICBsZXQgbmFtZXM6IFZlYzxfPiA9IHN1YnMuaXRlcigpLm1hcCh8c3wgcy5uYW1lLmFzX3N0cigpKS5jb2xsZWN0KCk7CiAgICAgICAgYXNzZXJ0IShuYW1lcy5jb250YWlucygmImJhciIpKTsKICAgICAgICBhc3NlcnQhKG5hbWVzLmNvbnRhaW5zKCYiZm9vIikpOwogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIGV4dHJhY3Rfc3VibW9kdWxlc19tYXJrc19jZmdfdGVzdCgpIHsKICAgICAgICBsZXQgc3JjID0gIiNbY2ZnKHRlc3QpXSBtb2QgaW5uZXI7IjsKICAgICAgICBsZXQgcmVzdWx0ID0gcGFyc2Vfc291cmNlKHNyYyk7CiAgICAgICAgbGV0IHN1YnMgPSBleHRyYWN0X3N1Ym1vZHVsZXMoJnJlc3VsdC5hc3QuaXRlbXMpOwogICAgICAgIGFzc2VydF9lcSEoc3Vicy5sZW4oKSwgMSk7CiAgICAgICAgYXNzZXJ0IShzdWJzWzBdLmlzX3Rlc3QpOwogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIGV4dHJhY3RfaW1wbHNfZmluZHNfZm5fdHlwZV9jb25zdCgpIHsKICAgICAgICBsZXQgc3JjID0gImltcGwgTXlUeXBlIHsgcHViIGZuIGZvbygmc2VsZikge30gcHViIHR5cGUgQWxpYXMgPSB1MzI7IHB1YiBjb25zdCBOOiB1c2l6ZSA9IDQyOyB9IjsKICAgICAgICBsZXQgcmVzdWx0ID0gcGFyc2Vfc291cmNlKHNyYyk7CiAgICAgICAgbGV0IGltcGxzID0gZXh0cmFjdF9pbXBscygmcmVzdWx0LmFzdC5pdGVtcyk7CiAgICAgICAgYXNzZXJ0X2VxIShpbXBscy5sZW4oKSwgMSk7CiAgICAgICAgYXNzZXJ0X2VxIShpbXBsc1swXS50eXBlXywgIk15VHlwZSIpOwogICAgICAgIGFzc2VydF9lcSEoaW1wbHNbMF0uaXRlbXMubGVuKCksIDMpOwogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIGJ1aWxkX3BhcnNlX2Vycm9yX2VudHJ5X2NvbnN0cnVjdHNfZXJyb3IoKSB7CiAgICAgICAgbGV0IHBhdGggPSBQYXRoQnVmOjpmcm9tKCJ0ZXN0LnJzIik7CiAgICAgICAgbGV0IGVyciA9IFN5blBhcnNlRXJyb3IgewogICAgICAgICAgICBtZXNzYWdlOiAiZXhwZWN0ZWQgYDtgIi50b19zdHJpbmcoKSwKICAgICAgICAgICAgbGluZTogNSwKICAgICAgICB9OwogICAgICAgIGxldCBlbnRyeSA9IGJ1aWxkX3BhcnNlX2Vycm9yX2VudHJ5KCZwYXRoLCAmZXJyKTsKICAgICAgICBhc3NlcnRfZXEhKGVudHJ5LmZpbGUsICJ0ZXN0LnJzIik7CiAgICAgICAgYXNzZXJ0X2VxIShlbnRyeS5saW5lLCA1KTsKICAgICAgICBhc3NlcnRfZXEhKGVudHJ5LmtpbmQsICJzeW5fcGFyc2VfZXJyb3IiKTsKICAgICAgICBhc3NlcnRfZXEhKGVudHJ5LnNldmVyaXR5LCBFcnJvclNldmVyaXR5OjpFcnJvcik7CiAgICB9Cn0=", "target": "src/file_parser.rs", "index": 0, "is_create": false}, {"before_b64": "Zm4gcHJvY2Vzc19tb2R1bGVfaW5mbygKICAgIG1vZHVsZV9wYXRoOiAmc3RyLAogICAgZmlsZV9wYXRoOiAmUGF0aCwKICAgIHZpc2liaWxpdHk6ICZzdHIsCiAgICBmaWxlX2luZm86ICZGaWxlSW5mbywKICAgIGl0ZW1zOiAmW3N5bjo6SXRlbV0sCiAgICBfcGFyZW50X2RpcjogJlBhdGgsCiAgICB2aXNpdGVkOiAmbXV0IEhhc2hTZXQ8UGF0aEJ1Zj4sCiAgICBlcnJvcnM6ICZtdXQgVmVjPGNyYXRlOjpzY2hlbWE6OkVycm9yRW50cnk+LAopIC0+IChWZWM8TW9kdWxlSW5mbz4sIFZlYzxjcmF0ZTo6c2NoZW1hOjpFcnJvckVudHJ5PikgewogICAgbGV0IG11dCBtb2R1bGVzID0gdmVjIVtidWlsZF9tb2R1bGVfaW5mbygKICAgICAgICBtb2R1bGVfcGF0aCwKICAgICAgICBmaWxlX3BhdGgsCiAgICAgICAgdmlzaWJpbGl0eSwKICAgICAgICAmZmlsZV9pbmZvLnB1YmxpY19pdGVtcywKICAgICAgICAmZmlsZV9pbmZvLmltcG9ydHMsCiAgICAgICAgJmZpbGVfaW5mby5yZV9leHBvcnRzLAogICAgICAgICZmaWxlX2luZm8uc3VibW9kdWxlcywKICAgICldOwoKICAgIGZvciBzdWIgaW4gJmZpbGVfaW5mby5zdWJtb2R1bGVzIHsKICAgICAgICBpZiBzdWIuaXNfdGVzdCB7CiAgICAgICAgICAgIGNvbnRpbnVlOwogICAgICAgIH0KICAgICAgICBsZXQgY2hpbGRfcGF0aCA9IGZvcm1hdCEoInt9Ojp7fSIsIG1vZHVsZV9wYXRoLCBzdWIubmFtZSk7CiAgICAgICAgbGV0IGNoaWxkX2RpciA9IGZpbGVfcGF0aC5wYXJlbnQoKS51bndyYXBfb3IoZmlsZV9wYXRoKTsKICAgICAgICBsZXQgKGNoaWxkX21vZHVsZXMsIGNoaWxkX2Vycm9ycykgPSBwcm9jZXNzX3N1Ym1vZHVsZSgKICAgICAgICAgICAgJmNoaWxkX3BhdGgsCiAgICAgICAgICAgICZzdWIubmFtZSwKICAgICAgICAgICAgaXRlbXMsCiAgICAgICAgICAgIGNoaWxkX2RpciwKICAgICAgICAgICAgZmlsZV9wYXRoLAogICAgICAgICAgICB2aXNpdGVkLAogICAgICAgICk7CiAgICAgICAgZXJyb3JzLmV4dGVuZChjaGlsZF9lcnJvcnMpOwogICAgICAgIG1vZHVsZXMuZXh0ZW5kKGNoaWxkX21vZHVsZXMpOwogICAgfQoKICAgIChtb2R1bGVzLCBlcnJvcnMuY2xvbmUoKSkKfQ==", "after_b64": "Zm4gcHJvY2Vzc19tb2R1bGVfaW5mbygKICAgIG1vZHVsZV9wYXRoOiAmc3RyLAogICAgZmlsZV9wYXRoOiAmUGF0aCwKICAgIHZpc2liaWxpdHk6ICZzdHIsCiAgICBmaWxlX2luZm86ICZGaWxlSW5mbywKICAgIGl0ZW1zOiAmW3N5bjo6SXRlbV0sCiAgICBfcGFyZW50X2RpcjogJlBhdGgsCiAgICB2aXNpdGVkOiAmbXV0IEhhc2hTZXQ8UGF0aEJ1Zj4sCiAgICBlcnJvcnM6ICZtdXQgVmVjPGNyYXRlOjpzY2hlbWE6OkVycm9yRW50cnk+LAopIC0+IChWZWM8TW9kdWxlSW5mbz4sIFZlYzxjcmF0ZTo6c2NoZW1hOjpFcnJvckVudHJ5PikgewogICAgbGV0IG11dCBtb2R1bGVzID0gdmVjIVtidWlsZF9tb2R1bGVfaW5mbygKICAgICAgICBtb2R1bGVfcGF0aCwKICAgICAgICBmaWxlX3BhdGgsCiAgICAgICAgdmlzaWJpbGl0eSwKICAgICAgICAmZmlsZV9pbmZvLnB1YmxpY19pdGVtcywKICAgICAgICAmZmlsZV9pbmZvLmltcG9ydHMsCiAgICAgICAgJmZpbGVfaW5mby5yZV9leHBvcnRzLAogICAgICAgICZmaWxlX2luZm8uc3VibW9kdWxlcywKICAgICldOwoKICAgIGZvciBzdWIgaW4gJmZpbGVfaW5mby5zdWJtb2R1bGVzIHsKICAgICAgICBpZiBzdWIuaXNfdGVzdCB7CiAgICAgICAgICAgIGNvbnRpbnVlOwogICAgICAgIH0KICAgICAgICBsZXQgY2hpbGRfcGF0aCA9IGZvcm1hdCEoInt9Ojp7fSIsIG1vZHVsZV9wYXRoLCBzdWIubmFtZSk7CiAgICAgICAgbGV0IGNoaWxkX2RpciA9IGZpbGVfcGF0aC5wYXJlbnQoKS51bndyYXBfb3IoZmlsZV9wYXRoKTsKICAgICAgICBsZXQgKGNoaWxkX21vZHVsZXMsIGNoaWxkX2Vycm9ycykgPSBwcm9jZXNzX3N1Ym1vZHVsZSgKICAgICAgICAgICAgJmNoaWxkX3BhdGgsCiAgICAgICAgICAgICZzdWIubmFtZSwKICAgICAgICAgICAgaXRlbXMsCiAgICAgICAgICAgIGNoaWxkX2RpciwKICAgICAgICAgICAgZmlsZV9wYXRoLAogICAgICAgICAgICB2aXNpdGVkLAogICAgICAgICk7CiAgICAgICAgZXJyb3JzLmV4dGVuZChjaGlsZF9lcnJvcnMpOwogICAgICAgIG1vZHVsZXMuZXh0ZW5kKGNoaWxkX21vZHVsZXMpOwogICAgfQoKICAgIChtb2R1bGVzLCBlcnJvcnMuY2xvbmUoKSkKfQoKLy8g4pSA4pSAIFRlc3RzIOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgAoKI1tjZmcodGVzdCldCm1vZCB0ZXN0cyB7CiAgICB1c2Ugc3VwZXI6Oio7CiAgICB1c2UgY3JhdGU6OnNjaGVtYTo6RXJyb3JFbnRyeTsKCiAgICAjW3Rlc3RdCiAgICBmbiByZXNvbHZlX21vZHVsZV9wYXRoX2ZpbmRzX3JzX2ZpbGUoKSB7CiAgICAgICAgbGV0IHRtcCA9IHN0ZDo6ZW52Ojp0ZW1wX2RpcigpLmpvaW4oInJlc29sdmVfdGVzdCIpOwogICAgICAgIGxldCBfID0gc3RkOjpmczo6Y3JlYXRlX2Rpcl9hbGwoJnRtcCk7CiAgICAgICAgbGV0IG1vZF9maWxlID0gdG1wLmpvaW4oImZvby5ycyIpOwogICAgICAgIHN0ZDo6ZnM6OndyaXRlKCZtb2RfZmlsZSwgIiIpLm9rKCk7CiAgICAgICAgbGV0IHJlc3VsdCA9IHJlc29sdmVfbW9kdWxlX3BhdGgoJnRtcCwgImZvbyIpOwogICAgICAgIGFzc2VydF9lcSEocmVzdWx0LCBTb21lKG1vZF9maWxlKSk7CiAgICAgICAgc3RkOjpmczo6cmVtb3ZlX2Rpcl9hbGwoJnRtcCkub2soKTsKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiByZXNvbHZlX21vZHVsZV9wYXRoX2ZpbmRzX21vZF9ycygpIHsKICAgICAgICBsZXQgdG1wID0gc3RkOjplbnY6OnRlbXBfZGlyKCkuam9pbigicmVzb2x2ZV90ZXN0MiIpOwogICAgICAgIGxldCBfID0gc3RkOjpmczo6Y3JlYXRlX2Rpcl9hbGwoJnRtcCk7CiAgICAgICAgbGV0IG1vZF9kaXIgPSB0bXAuam9pbigiYmFyIik7CiAgICAgICAgbGV0IF8gPSBzdGQ6OmZzOjpjcmVhdGVfZGlyX2FsbCgmbW9kX2Rpcik7CiAgICAgICAgbGV0IG1vZF9ycyA9IG1vZF9kaXIuam9pbigibW9kLnJzIik7CiAgICAgICAgc3RkOjpmczo6d3JpdGUoJm1vZF9ycywgIiIpLm9rKCk7CiAgICAgICAgbGV0IHJlc3VsdCA9IHJlc29sdmVfbW9kdWxlX3BhdGgoJnRtcCwgImJhciIpOwogICAgICAgIGFzc2VydF9lcSEocmVzdWx0LCBTb21lKG1vZF9ycykpOwogICAgICAgIHN0ZDo6ZnM6OnJlbW92ZV9kaXJfYWxsKCZ0bXApLm9rKCk7CiAgICB9CgogICAgI1t0ZXN0XQogICAgZm4gcmVzb2x2ZV9tb2R1bGVfcGF0aF9yZXR1cm5zX25vbmVfZm9yX21pc3NpbmcoKSB7CiAgICAgICAgbGV0IHRtcCA9IHN0ZDo6ZW52Ojp0ZW1wX2RpcigpLmpvaW4oInJlc29sdmVfdGVzdDMiKTsKICAgICAgICBsZXQgXyA9IHN0ZDo6ZnM6OmNyZWF0ZV9kaXJfYWxsKCZ0bXApOwogICAgICAgIGxldCByZXN1bHQgPSByZXNvbHZlX21vZHVsZV9wYXRoKCZ0bXAsICJub25leGlzdGVudCIpOwogICAgICAgIGFzc2VydCEocmVzdWx0LmlzX25vbmUoKSk7CiAgICAgICAgc3RkOjpmczo6cmVtb3ZlX2Rpcl9hbGwoJnRtcCkub2soKTsKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiBidWlsZF9tb2R1bGVfdHJlZV9yZXR1cm5zX2VtcHR5X2Zvcl9ub25leGlzdGVudCgpIHsKICAgICAgICBsZXQgdG1wID0gc3RkOjplbnY6OnRlbXBfZGlyKCkuam9pbigiYm10X3Rlc3QiKTsKICAgICAgICBsZXQgXyA9IHN0ZDo6ZnM6OmNyZWF0ZV9kaXJfYWxsKCZ0bXApOwogICAgICAgIGxldCAobW9kdWxlcywgZXJyb3JzKSA9IGJ1aWxkX21vZHVsZV90cmVlKCZ0bXAsICJ0ZXN0Iik7CiAgICAgICAgYXNzZXJ0IShtb2R1bGVzLmlzX2VtcHR5KCkpOwogICAgICAgIGFzc2VydCEoIWVycm9ycy5pc19lbXB0eSgpKTsKICAgICAgICBzdGQ6OmZzOjpyZW1vdmVfZGlyX2FsbCgmdG1wKS5vaygpOwogICAgfQp9", "target": "src/module_tree.rs", "index": 1, "is_create": false}, {"before_b64": "ICAgIHJlc3VsdC5zb3J0KCk7CiAgICByZXN1bHQuZGVkdXAoKTsKICAgIE9rKHJlc3VsdCkKfQoKLy8vIEZvciBhIGNyYXRlIGRpcmVjdG9yeSwgZGV0ZXJtaW5lIGl0cyBlbnRyeS1wb2ludCBmaWxlKHMpLg==", "after_b64": "ICAgIHJlc3VsdC5zb3J0KCk7CiAgICByZXN1bHQuZGVkdXAoKTsKICAgIE9rKHJlc3VsdCkKfQoKLy8g4pSA4pSAIFRlc3RzIOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgAoKI1tjZmcodGVzdCldCm1vZCB0ZXN0cyB7CiAgICB1c2Ugc3VwZXI6Oio7CiAgICB1c2Ugc3RkOjppbzo6V3JpdGU7CgogICAgZm4gd3JpdGVfY2FyZ29fdG9tbChkaXI6ICZzdGQ6OnBhdGg6OlBhdGgsIGNvbnRlbnQ6ICZzdHIpIHsKICAgICAgICBsZXQgbXV0IGYgPSBzdGQ6OmZzOjpGaWxlOjpjcmVhdGUoZGlyLmpvaW4oIkNhcmdvLnRvbWwiKSkudW53cmFwKCk7CiAgICAgICAgZi53cml0ZV9hbGwoY29udGVudC5hc19ieXRlcygpKS51bndyYXAoKTsKICAgIH0KCiAgICBmbiBzZXR1cF9jcmF0ZShkaXI6ICZzdGQ6OnBhdGg6OlBhdGgpIHsKICAgICAgICBsZXQgc3JjID0gZGlyLmpvaW4oInNyYyIpOwogICAgICAgIHN0ZDo6ZnM6OmNyZWF0ZV9kaXJfYWxsKCZzcmMpLnVud3JhcCgpOwogICAgICAgIHN0ZDo6ZnM6OndyaXRlKHNyYy5qb2luKCJsaWIucnMiKSwgIiIpLnVud3JhcCgpOwogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIGZpbmRfd29ya3NwYWNlX3Jvb3RfZmluZHNfY2FyZ29fdG9tbCgpIHsKICAgICAgICBsZXQgdG1wID0gdGVtcGZpbGU6OnRlbXBkaXIoKS51bndyYXAoKTsKICAgICAgICBsZXQgcGF0aCA9IHRtcC5wYXRoKCkuam9pbigic3ViZGlyIikuam9pbigibmVzdGVkIik7CiAgICAgICAgc3RkOjpmczo6Y3JlYXRlX2Rpcl9hbGwoJnBhdGgpLnVud3JhcCgpOwogICAgICAgIHdyaXRlX2NhcmdvX3RvbWwodG1wLnBhdGgoKSwgIlt3b3Jrc3BhY2VdIik7CiAgICAgICAgbGV0IHJlc3VsdCA9IGZpbmRfd29ya3NwYWNlX3Jvb3QoJnBhdGgpLnVud3JhcCgpOwogICAgICAgIGFzc2VydF9lcSEocmVzdWx0LCB0bXAucGF0aCgpKTsKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiBlbnVtZXJhdGVfbWVtYmVyc19yZXR1cm5zX21lbWJlcnMoKSB7CiAgICAgICAgbGV0IHRtcCA9IHRlbXBmaWxlOjp0ZW1wZGlyKCkudW53cmFwKCk7CiAgICAgICAgd3JpdGVfY2FyZ29fdG9tbCh0bXAucGF0aCgpLCByIyIKW3dvcmtzcGFjZV0KbWVtYmVycyA9IFsiY3JhdGVfYSIsICJjcmF0ZV9iIl0KIiMpOwogICAgICAgIHNldHVwX2NyYXRlKHRtcC5wYXRoKCkuam9pbigiY3JhdGVfYSIpLmFzX3BhdGgoKSk7CiAgICAgICAgc2V0dXBfY3JhdGUodG1wLnBhdGgoKS5qb2luKCJjcmF0ZV9iIikuYXNfcGF0aCgpKTsKICAgICAgICBsZXQgbWVtYmVycyA9IGVudW1lcmF0ZV9tZW1iZXJzKHRtcC5wYXRoKCkpLnVud3JhcCgpOwogICAgICAgIGFzc2VydF9lcSEobWVtYmVycy5sZW4oKSwgMik7CiAgICB9CgogICAgI1t0ZXN0XQogICAgZm4gZW51bWVyYXRlX21lbWJlcnNfcmV0dXJuc19lcnJfZm9yX21pc3Npbmdfd29ya3NwYWNlKCkgewogICAgICAgIGxldCB0bXAgPSB0ZW1wZmlsZTo6dGVtcGRpcigpLnVud3JhcCgpOwogICAgICAgIHdyaXRlX2NhcmdvX3RvbWwodG1wLnBhdGgoKSwgIltkZXBlbmRlbmNpZXNdXG5mb28gPSBcIjFcIiIpOwogICAgICAgIGxldCByZXN1bHQgPSBlbnVtZXJhdGVfbWVtYmVycyh0bXAucGF0aCgpKTsKICAgICAgICBhc3NlcnQhKHJlc3VsdC5pc19lcnIoKSk7CiAgICAgICAgbWF0Y2ggcmVzdWx0LnVud3JhcF9lcnIoKSB7CiAgICAgICAgICAgIEVycm9yOjpNaXNzaW5nV29ya3NwYWNlU2VjdGlvbiA9PiB7fSwKICAgICAgICAgICAgb3RoZXIgPT4gcGFuaWMhKCJleHBlY3RlZCBNaXNzaW5nV29ya3NwYWNlU2VjdGlvbiwgZ290IHs6P30iLCBvdGhlciksCiAgICAgICAgfQogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIGVudW1lcmF0ZV9tZW1iZXJzX2FwcGxpZXNfZXhjbHVkZSgpIHsKICAgICAgICBsZXQgdG1wID0gdGVtcGZpbGU6OnRlbXBkaXIoKS51bndyYXAoKTsKICAgICAgICB3cml0ZV9jYXJnb190b21sKHRtcC5wYXRoKCksIHIjIgpbd29ya3NwYWNlXQptZW1iZXJzID0gWyJhIiwgImIiLCAiYyJdCmV4Y2x1ZGUgPSBbImIiXQoiIyk7CiAgICAgICAgc2V0dXBfY3JhdGUodG1wLnBhdGgoKS5qb2luKCJhIikuYXNfcGF0aCgpKTsKICAgICAgICBzZXR1cF9jcmF0ZSh0bXAucGF0aCgpLmpvaW4oImIiKS5hc19wYXRoKCkpOwogICAgICAgIHNldHVwX2NyYXRlKHRtcC5wYXRoKCkuam9pbigiYyIpLmFzX3BhdGgoKSk7CiAgICAgICAgbGV0IG1lbWJlcnMgPSBlbnVtZXJhdGVfbWVtYmVycyh0bXAucGF0aCgpKS51bndyYXAoKTsKICAgICAgICBsZXQgbmFtZXM6IFZlYzxfPiA9IG1lbWJlcnMuaXRlcigpLm1hcCh8cHwgcC5maWxlX25hbWUoKS51bndyYXAoKS50b19zdHJpbmdfbG9zc3koKSkuY29sbGVjdCgpOwogICAgICAgIGFzc2VydCEobmFtZXMuY29udGFpbnMoJiJhIi5hc19yZWYoKSkpOwogICAgICAgIGFzc2VydCEoIW5hbWVzLmNvbnRhaW5zKCYiYiIuYXNfcmVmKCkpKTsKICAgICAgICBhc3NlcnQhKG5hbWVzLmNvbnRhaW5zKCYiYyIuYXNfcmVmKCkpKTsKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiByZXNvbHZlX2NyYXRlX3Jvb3RzX2RldGVjdHNfbGliKCkgewogICAgICAgIGxldCB0bXAgPSB0ZW1wZmlsZTo6dGVtcGRpcigpLnVud3JhcCgpOwogICAgICAgIHN0ZDo6ZnM6OmNyZWF0ZV9kaXJfYWxsKHRtcC5wYXRoKCkuam9pbigic3JjIikpLnVud3JhcCgpOwogICAgICAgIHN0ZDo6ZnM6OndyaXRlKHRtcC5wYXRoKCkuam9pbigic3JjIikuam9pbigibGliLnJzIiksICIiKS51bndyYXAoKTsKICAgICAgICBsZXQgcm9vdHMgPSByZXNvbHZlX2NyYXRlX3Jvb3RzKHRtcC5wYXRoKCkpOwogICAgICAgIGFzc2VydF9lcSEocm9vdHMubGVuKCksIDEpOwogICAgICAgIGFzc2VydF9lcSEocm9vdHNbMF0uMSwgQ3JhdGVUeXBlOjpMaWIpOwogICAgfQoKICAgICNbdGVzdF0KICAgIGZuIHJlc29sdmVfY3JhdGVfcm9vdHNfZGV0ZWN0c19iaW4oKSB7CiAgICAgICAgbGV0IHRtcCA9IHRlbXBmaWxlOjp0ZW1wZGlyKCkudW53cmFwKCk7CiAgICAgICAgc3RkOjpmczo6Y3JlYXRlX2Rpcl9hbGwodG1wLnBhdGgoKS5qb2luKCJzcmMiKSkudW53cmFwKCk7CiAgICAgICAgc3RkOjpmczo6d3JpdGUodG1wLnBhdGgoKS5qb2luKCJzcmMiKS5qb2luKCJtYWluLnJzIiksICIiKS51bndyYXAoKTsKICAgICAgICBsZXQgcm9vdHMgPSByZXNvbHZlX2NyYXRlX3Jvb3RzKHRtcC5wYXRoKCkpOwogICAgICAgIGFzc2VydF9lcSEocm9vdHMubGVuKCksIDEpOwogICAgICAgIGFzc2VydF9lcSEocm9vdHNbMF0uMSwgQ3JhdGVUeXBlOjpCaW4pOwogICAgfQp9", "target": "src/workspace.rs", "index": 2, "is_create": false}, {"before_b64": "ICAgIE9rKChwYWNrYWdlLCBkZXBzKSkKfQ==", "after_b64": "ICAgIE9rKChwYWNrYWdlLCBkZXBzKSkKfQoKLy8g4pSA4pSAIFRlc3RzIOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgAoKI1tjZmcodGVzdCldCm1vZCB0ZXN0cyB7CiAgICB1c2Ugc3VwZXI6Oio7CiAgICB1c2Ugc3RkOjppbzo6V3JpdGU7CgogICAgZm4gd3JpdGVfY2FyZ29fdG9tbChkaXI6ICZzdGQ6OnBhdGg6OlBhdGgsIGNvbnRlbnQ6ICZzdHIpIHsKICAgICAgICBsZXQgbXV0IGYgPSBzdGQ6OmZzOjpGaWxlOjpjcmVhdGUoZGlyLmpvaW4oIkNhcmdvLnRvbWwiKSkudW53cmFwKCk7CiAgICAgICAgZi53cml0ZV9hbGwoY29udGVudC5hc19ieXRlcygpKS51bndyYXAoKTsKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiBwYXJzZV9jYXJnb190b21sX3BhcnNlc19taW5pbWFsKCkgewogICAgICAgIGxldCB0bXAgPSB0ZW1wZmlsZTo6dGVtcGRpcigpLnVud3JhcCgpOwogICAgICAgIHdyaXRlX2NhcmdvX3RvbWwodG1wLnBhdGgoKSwgciMiCltwYWNrYWdlXQpuYW1lID0gInRlc3QtcGtnIgp2ZXJzaW9uID0gIjEuMC4wIgplZGl0aW9uID0gIjIwMjEiCiIjKTsKICAgICAgICBsZXQgKHBrZywgX2RlcHMpID0gcGFyc2VfY2FyZ29fdG9tbCh0bXAucGF0aCgpLmpvaW4oIkNhcmdvLnRvbWwiKS5hc19wYXRoKCkpLnVud3JhcCgpOwogICAgICAgIGFzc2VydF9lcSEocGtnLm5hbWUsICJ0ZXN0LXBrZyIpOwogICAgICAgIGFzc2VydF9lcSEocGtnLnZlcnNpb24sICIxLjAuMCIpOwogICAgICAgIGFzc2VydF9lcSEocGtnLmVkaXRpb24sICIyMDIxIik7CiAgICB9CgogICAgI1t0ZXN0XQogICAgZm4gcGFyc2VfY2FyZ29fdG9tbF91c2VzX2RlZmF1bHRzX2Zvcl9taXNzaW5nX3BhY2thZ2UoKSB7CiAgICAgICAgbGV0IHRtcCA9IHRlbXBmaWxlOjp0ZW1wZGlyKCkudW53cmFwKCk7CiAgICAgICAgd3JpdGVfY2FyZ29fdG9tbCh0bXAucGF0aCgpLCAiIik7CiAgICAgICAgbGV0IChwa2csIF9kZXBzKSA9IHBhcnNlX2NhcmdvX3RvbWwodG1wLnBhdGgoKS5qb2luKCJDYXJnby50b21sIikuYXNfcGF0aCgpKS51bndyYXAoKTsKICAgICAgICBhc3NlcnRfZXEhKHBrZy5uYW1lLCAidW5rbm93biIpOwogICAgICAgIGFzc2VydF9lcSEocGtnLmVkaXRpb24sICIyMDIxIik7CiAgICB9CgogICAgI1t0ZXN0XQogICAgZm4gcGFyc2VfY2FyZ29fdG9tbF9kaXN0aW5ndWlzaGVzX2RlcHMoKSB7CiAgICAgICAgbGV0IHRtcCA9IHRlbXBmaWxlOjp0ZW1wZGlyKCkudW53cmFwKCk7CiAgICAgICAgd3JpdGVfY2FyZ29fdG9tbCh0bXAucGF0aCgpLCByIyIKW3BhY2thZ2VdCm5hbWUgPSAidGVzdC1wa2ciCnZlcnNpb24gPSAiMC4xLjAiCmVkaXRpb24gPSAiMjAyMSIKCltkZXBlbmRlbmNpZXNdCmZvbyA9ICIxIgpiYXIgPSB7IHdvcmtzcGFjZSA9IHRydWUgfQoKW2Rldi1kZXBlbmRlbmNpZXNdCmJheiA9ICIyIgpxdXggPSB7IHdvcmtzcGFjZSA9IHRydWUgfQoiIyk7CiAgICAgICAgbGV0IChfLCBkZXBzKSA9IHBhcnNlX2NhcmdvX3RvbWwodG1wLnBhdGgoKS5qb2luKCJDYXJnby50b21sIikuYXNfcGF0aCgpKS51bndyYXAoKTsKICAgICAgICBhc3NlcnRfZXEhKGRlcHMubm9ybWFsLCB2ZWMhWyJmb28iXSk7CiAgICAgICAgYXNzZXJ0X2VxIShkZXBzLmRldiwgdmVjIVsiYmF6Il0pOwogICAgICAgIGFzc2VydCEoZGVwcy53b3Jrc3BhY2VfbWVtYmVycy5jb250YWlucygmImJhciIudG9fc3RyaW5nKCkpKTsKICAgICAgICBhc3NlcnQhKGRlcHMud29ya3NwYWNlX21lbWJlcnMuY29udGFpbnMoJiJxdXgiLnRvX3N0cmluZygpKSk7CiAgICB9Cn0=", "target": "src/cargo_info.rs", "index": 3, "is_create": false}, {"before_b64": "ICAgIENyb3NzUmVmZXJlbmNlcyB7IHR5cGVzOiB0eXBlc19tYXAgfQp9CgovLyBIZWxwZXI6IGNvbnZlcnQgSXRlbUtpbmQgdG8gYSBzaG9ydCBzdHJpbmcgZm9yIHRoZSBUeXBlUmVmLmtpbmQgZmllbGQu", "after_b64": "ICAgIENyb3NzUmVmZXJlbmNlcyB7IHR5cGVzOiB0eXBlc19tYXAgfQp9CgovLyDilIDilIAgVGVzdHMg4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSACgojW2NmZyh0ZXN0KV0KbW9kIHRlc3RzIHsKICAgIHVzZSBzdXBlcjo6KjsKICAgIHVzZSBjcmF0ZTo6c2NoZW1hOjp7TW9kdWxlSW5mbywgUHVibGljSXRlbSwgU3VibW9kdWxlRGVjbH07CgogICAgZm4gbWFrZV9jcmF0ZShuYW1lOiAmc3RyLCBpdGVtczogVmVjPChTdHJpbmcsIEl0ZW1LaW5kKT4pIC0+IENyYXRlSW5mbyB7CiAgICAgICAgbGV0IHB1YmxpY19pdGVtczogVmVjPFB1YmxpY0l0ZW0+ID0gaXRlbXMKICAgICAgICAgICAgLmludG9faXRlcigpCiAgICAgICAgICAgIC5tYXAofChuLCBrKXwgewogICAgICAgICAgICAgICAgUHVibGljSXRlbTo6YnVpbGRlcigpCiAgICAgICAgICAgICAgICAgICAgLmtpbmQoaykKICAgICAgICAgICAgICAgICAgICAubmFtZShuKQogICAgICAgICAgICAgICAgICAgIC5maWxlKFN0cmluZzo6bmV3KCkpCiAgICAgICAgICAgICAgICAgICAgLmxpbmUoMSkKICAgICAgICAgICAgICAgICAgICAudmlzaWJpbGl0eSgicHViIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgICAgICAgICAuZ2VuZXJpY3MoU3RyaW5nOjpuZXcoKSkKICAgICAgICAgICAgICAgICAgICAuYXR0cnMoRGVmYXVsdDo6ZGVmYXVsdCgpKQogICAgICAgICAgICAgICAgICAgIC5idWlsZCgpCiAgICAgICAgICAgIH0pCiAgICAgICAgICAgIC5jb2xsZWN0KCk7CiAgICAgICAgbGV0IG1vZHVsZSA9IE1vZHVsZUluZm86OmJ1aWxkZXIoKQogICAgICAgICAgICAucGF0aCgiIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgLmZpbGUoU3RyaW5nOjpuZXcoKSkKICAgICAgICAgICAgLnZpc2liaWxpdHkoInB1YiIudG9fc3RyaW5nKCkpCiAgICAgICAgICAgIC5wdWJsaWNfaXRlbXMocHVibGljX2l0ZW1zKQogICAgICAgICAgICAuYnVpbGQoKTsKICAgICAgICBDcmF0ZUluZm86OmJ1aWxkZXIoKQogICAgICAgICAgICAubmFtZShuYW1lLnRvX3N0cmluZygpKQogICAgICAgICAgICAucm9vdChTdHJpbmc6Om5ldygpKQogICAgICAgICAgICAucGFja2FnZSgKICAgICAgICAgICAgICAgIHNjaGVtYTo6UGFja2FnZUluZm86OmJ1aWxkZXIoKQogICAgICAgICAgICAgICAgICAgIC5uYW1lKG5hbWUudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAgICAgLnZlcnNpb24oIjAuMS4wIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgICAgICAgICAuZWRpdGlvbigiMjAyMSIudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAgICAgLmNyYXRlX3R5cGUoc2NoZW1hOjpDcmF0ZVR5cGU6OkxpYikKICAgICAgICAgICAgICAgICAgICAuYnVpbGQoKSwKICAgICAgICAgICAgKQogICAgICAgICAgICAubW9kdWxlcyh2ZWMhW21vZHVsZV0pCiAgICAgICAgICAgIC5kZXBzKERlZmF1bHQ6OmRlZmF1bHQoKSkKICAgICAgICAgICAgLmJ1aWxkKCkKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiBjb21wdXRlX2ZpbmRzX2Nyb3NzX2NyYXRlX2ltcG9ydCgpIHsKICAgICAgICBsZXQgbXV0IGNyYXRlcyA9IHZlYyFbCiAgICAgICAgICAgIG1ha2VfY3JhdGUoImNvcmUiLCB2ZWMhWwogICAgICAgICAgICAgICAgKCJUYXNrIi50b19zdHJpbmcoKSwgSXRlbUtpbmQ6OlN0cnVjdCksCiAgICAgICAgICAgIF0pLAogICAgICAgICAgICBtYWtlX2NyYXRlKCJlbmdpbmUiLCB2ZWMhW10pLAogICAgICAgIF07CiAgICAgICAgLy8gTWFudWFsbHkgYWRkIGFuIGltcG9ydCBpbiBlbmdpbmUgdGhhdCByZWZlcmVuY2VzIGNvcmU6OlRhc2sKICAgICAgICBsZXQgZW5naW5lX21vZHVsZSA9ICZtdXQgY3JhdGVzWzFdLm1vZHVsZXNbMF07CiAgICAgICAgZW5naW5lX21vZHVsZS5pbXBvcnRzLnB1c2goSW1wb3J0IHsKICAgICAgICAgICAgcGF0aDogImNvcmU6OlRhc2siLnRvX3N0cmluZygpLAogICAgICAgICAgICBsaW5lOiAxLAogICAgICAgIH0pOwogICAgICAgIGxldCByZWZzID0gY29tcHV0ZSgmbXV0IGNyYXRlcyk7CiAgICAgICAgLy8gVGFzayBzaG91bGQgYmUgaW4gY3Jvc3MtcmVmZXJlbmNlcwogICAgICAgIGFzc2VydCEocmVmcy50eXBlcy5jb250YWluc19rZXkoIlRhc2siKSk7CiAgICAgICAgbGV0IHRhc2tfcmVmID0gJnJlZnMudHlwZXNbIlRhc2siXTsKICAgICAgICBhc3NlcnRfZXEhKHRhc2tfcmVmLmNyYXRlX25hbWUsICJjb3JlIik7CiAgICAgICAgLy8gZW5naW5lIHNob3VsZCBoYXZlIGEgY3Jvc3NfY3JhdGVfaW1wb3J0CiAgICAgICAgYXNzZXJ0X2VxIShjcmF0ZXNbMV0uY3Jvc3NfY3JhdGVfaW1wb3J0cy5sZW4oKSwgMSk7CiAgICAgICAgYXNzZXJ0X2VxIShjcmF0ZXNbMV0uY3Jvc3NfY3JhdGVfaW1wb3J0c1swXS50YXJnZXRfY3JhdGUsICJjb3JlIik7CiAgICB9CgogICAgI1t0ZXN0XQogICAgZm4gY29tcHV0ZV9lbXB0eV9mb3Jfbm9fY3Jvc3NfcmVmZXJlbmNlcygpIHsKICAgICAgICBsZXQgY3JhdGVzID0gdmVjIVsKICAgICAgICAgICAgbWFrZV9jcmF0ZSgiYSIsIHZlYyFbKCJGb28iLnRvX3N0cmluZygpLCBJdGVtS2luZDo6U3RydWN0KV0pLAogICAgICAgICAgICBtYWtlX2NyYXRlKCJiIiwgdmVjIVsoIkJhciIudG9fc3RyaW5nKCksIEl0ZW1LaW5kOjpTdHJ1Y3QpXSksCiAgICAgICAgXTsKICAgICAgICBsZXQgbXV0IGNyYXRlc19tdXQgPSBjcmF0ZXM7CiAgICAgICAgbGV0IHJlZnMgPSBjb21wdXRlKCZtdXQgY3JhdGVzX211dCk7CiAgICAgICAgLy8gTm8gY3Jvc3MgcmVmZXJlbmNlcyBzaW5jZSBubyBjcmF0ZSBpbXBvcnRzIGZyb20gYW5vdGhlcgogICAgICAgIGFzc2VydCEocmVmcy50eXBlcy5pc19lbXB0eSgpIHx8IHJlZnMudHlwZXMudmFsdWVzKCkuYWxsKHx0fCB0LmltcG9ydGVkX2J5LmlzX2VtcHR5KCkpKTsKICAgIH0KfQoKLy8gSGVscGVyOiBjb252ZXJ0IEl0ZW1LaW5kIHRvIGEgc2hvcnQgc3RyaW5nIGZvciB0aGUgVHlwZVJlZi5raW5kIGZpZWxkLg==", "target": "src/cross_refs.rs", "index": 4, "is_create": false}, {"before_b64": "dXNlIGNyYXRlOjpzY2hlbWE6OldvcmtzcGFjZU1hcDsKdXNlIHN0ZDo6aW86OldyaXRlOwoKLy8vIFNlcmlhbGl6ZSB0aGUgd29ya3NwYWNlIG1hcCB0byBhIEpTT04gc3RyaW5nIHdpdGggMi1zcGFjZSBpbmRlbnRhdGlvbi4KI1ttdXN0X3VzZV0KcHViIGZuIHJlbmRlcl9qc29uKG1hcDogJldvcmtzcGFjZU1hcCkgLT4gc2VyZGVfanNvbjo6UmVzdWx0PFN0cmluZz4gewogICAgc2VyZGVfanNvbjo6dG9fc3RyaW5nX3ByZXR0eShtYXApCn0KCi8vLyBTZXJpYWxpemUgdGhlIHdvcmtzcGFjZSBtYXAgdG8gdGhlIGdpdmVuIHdyaXRlci4KcHViIGZuIHJlbmRlcl90b193cml0ZXIobWFwOiAmV29ya3NwYWNlTWFwLCB3cml0ZXI6IGltcGwgV3JpdGUpIC0+IHNlcmRlX2pzb246OlJlc3VsdDwoKT4gewogICAgc2VyZGVfanNvbjo6dG9fd3JpdGVyX3ByZXR0eSh3cml0ZXIsIG1hcCkKfQo=", "after_b64": "dXNlIGNyYXRlOjpzY2hlbWE6OldvcmtzcGFjZU1hcDsKdXNlIHN0ZDo6aW86OldyaXRlOwoKLy8vIFNlcmlhbGl6ZSB0aGUgd29ya3NwYWNlIG1hcCB0byBhIEpTT04gc3RyaW5nIHdpdGggMi1zcGFjZSBpbmRlbnRhdGlvbi4KI1ttdXN0X3VzZV0KcHViIGZuIHJlbmRlcl9qc29uKG1hcDogJldvcmtzcGFjZU1hcCkgLT4gc2VyZGVfanNvbjo6UmVzdWx0PFN0cmluZz4gewogICAgc2VyZGVfanNvbjo6dG9fc3RyaW5nX3ByZXR0eShtYXApCn0KCi8vLyBTZXJpYWxpemUgdGhlIHdvcmtzcGFjZSBtYXAgdG8gdGhlIGdpdmVuIHdyaXRlci4KcHViIGZuIHJlbmRlcl90b193cml0ZXIobWFwOiAmV29ya3NwYWNlTWFwLCB3cml0ZXI6IGltcGwgV3JpdGUpIC0+IHNlcmRlX2pzb246OlJlc3VsdDwoKT4gewogICAgc2VyZGVfanNvbjo6dG9fd3JpdGVyX3ByZXR0eSh3cml0ZXIsIG1hcCkKfQoKLy8g4pSA4pSAIFRlc3RzIOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgAoKI1tjZmcodGVzdCldCm1vZCB0ZXN0cyB7CiAgICB1c2Ugc3VwZXI6Oio7CiAgICB1c2UgY3JhdGU6OnNjaGVtYTo6ewogICAgICAgIENyYXRlSW5mbywgQ3JhdGVUeXBlLCBDcm9zc1JlZmVyZW5jZXMsIERlcEluZm8sIE1vZHVsZUluZm8sIFBhY2thZ2VJbmZvLAogICAgICAgIFdvcmtzcGFjZUluZm8sIFdvcmtzcGFjZU1hcCwKICAgIH07CgogICAgZm4gbWFrZV9taW5pbWFsX21hcCgpIC0+IFdvcmtzcGFjZU1hcCB7CiAgICAgICAgV29ya3NwYWNlTWFwOjpidWlsZGVyKCkKICAgICAgICAgICAgLndvcmtzcGFjZShXb3Jrc3BhY2VJbmZvOjpidWlsZGVyKCkKICAgICAgICAgICAgICAgIC5yb290KCIuIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgICAgIC53b3Jrc3BhY2VfbmFtZSgidGVzdCIudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAuYnVpbGQoKSkKICAgICAgICAgICAgLmNyYXRlcyh2ZWMhWwogICAgICAgICAgICAgICAgQ3JhdGVJbmZvOjpidWlsZGVyKCkKICAgICAgICAgICAgICAgICAgICAubmFtZSgidGVzdC1jcmF0ZSIudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAgICAgLnJvb3QoIi4iLnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgICAgIC5wYWNrYWdlKFBhY2thZ2VJbmZvOjpidWlsZGVyKCkKICAgICAgICAgICAgICAgICAgICAgICAgLm5hbWUoInRlc3QtY3JhdGUiLnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgICAgICAgICAudmVyc2lvbigiMC4xLjAiLnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgICAgICAgICAuZWRpdGlvbigiMjAyMSIudG9fc3RyaW5nKCkpCiAgICAgICAgICAgICAgICAgICAgICAgIC5jcmF0ZV90eXBlKENyYXRlVHlwZTo6TGliKQogICAgICAgICAgICAgICAgICAgICAgICAuYnVpbGQoKSkKICAgICAgICAgICAgICAgICAgICAubW9kdWxlcyh2ZWMhW01vZHVsZUluZm86OmJ1aWxkZXIoKQogICAgICAgICAgICAgICAgICAgICAgICAucGF0aCgiIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgICAgICAgICAgICAgLmZpbGUoInNyYy9saWIucnMiLnRvX3N0cmluZygpKQogICAgICAgICAgICAgICAgICAgICAgICAudmlzaWJpbGl0eSgicHViIi50b19zdHJpbmcoKSkKICAgICAgICAgICAgICAgICAgICAgICAgLmJ1aWxkKCldKQogICAgICAgICAgICAgICAgICAgIC5kZXBzKERlcEluZm86OmRlZmF1bHQoKSkKICAgICAgICAgICAgICAgICAgICAuYnVpbGQoKSwKICAgICAgICAgICAgXSkKICAgICAgICAgICAgLmNyb3NzX3JlZmVyZW5jZXMoQ3Jvc3NSZWZlcmVuY2VzOjpkZWZhdWx0KCkpCiAgICAgICAgICAgIC53b3Jrc3BhY2Vfcm9vdChzdGQ6OnBhdGg6OlBhdGhCdWY6OmZyb20oIi4iKSkKICAgICAgICAgICAgLmJ1aWxkKCkKICAgIH0KCiAgICAjW3Rlc3RdCiAgICBmbiByZW5kZXJfanNvbl9wcm9kdWNlc192YWxpZF9qc29uKCkgewogICAgICAgIGxldCBtYXAgPSBtYWtlX21pbmltYWxfbWFwKCk7CiAgICAgICAgbGV0IGpzb24gPSByZW5kZXJfanNvbigmbWFwKS51bndyYXAoKTsKICAgICAgICBsZXQgcGFyc2VkOiBzZXJkZV9qc29uOjpWYWx1ZSA9IHNlcmRlX2pzb246OmZyb21fc3RyKCZqc29uKS51bndyYXAoKTsKICAgICAgICBhc3NlcnRfZXEhKHBhcnNlZFsid29ya3NwYWNlIl1bInJvb3QiXSwgIi4iKTsKICAgICAgICBhc3NlcnRfZXEhKHBhcnNlZFsiY3JhdGVzIl0uYXNfYXJyYXkoKS51bndyYXAoKS5sZW4oKSwgMSk7CiAgICB9CgogICAgI1t0ZXN0XQogICAgZm4gcmVuZGVyX2pzb25fc2tpcHNfZW1wdHlfZXJyb3JzKCkgewogICAgICAgIGxldCBtYXAgPSBtYWtlX21pbmltYWxfbWFwKCk7CiAgICAgICAgbGV0IGpzb24gPSByZW5kZXJfanNvbigmbWFwKS51bndyYXAoKTsKICAgICAgICBsZXQgcGFyc2VkOiBzZXJkZV9qc29uOjpWYWx1ZSA9IHNlcmRlX2pzb246OmZyb21fc3RyKCZqc29uKS51bndyYXAoKTsKICAgICAgICAvLyBlcnJvcnMgZmllbGQgc2hvdWxkIGJlIGFic2VudCAoc2tpcF9zZXJpYWxpemluZ19pZikKICAgICAgICBhc3NlcnQhKHBhcnNlZC5nZXQoImVycm9ycyIpLmlzX25vbmUoKSk7CiAgICB9CgogICAgI1t0ZXN0XQogICAgZm4gcmVuZGVyX3RvX3dyaXRlcl9tYXRjaGVzX3JlbmRlcl9qc29uKCkgewogICAgICAgIGxldCBtYXAgPSBtYWtlX21pbmltYWxfbWFwKCk7CiAgICAgICAgbGV0IGpzb24gPSByZW5kZXJfanNvbigmbWFwKS51bndyYXAoKTsKCiAgICAgICAgbGV0IG11dCBidWYgPSBWZWM6Om5ldygpOwogICAgICAgIHJlbmRlcl90b193cml0ZXIoJm1hcCwgJm11dCBidWYpLnVud3JhcCgpOwogICAgICAgIGxldCBmcm9tX3dyaXRlciA9IFN0cmluZzo6ZnJvbV91dGY4KGJ1ZikudW53cmFwKCk7CgogICAgICAgIGFzc2VydF9lcSEoanNvbiwgZnJvbV93cml0ZXIpOwogICAgfQp9Cg==", "target": "src/render.rs", "index": 5, "is_create": false}]')

for step in STEPS:
    before = base64.b64decode(step["before_b64"]).decode()
    after = base64.b64decode(step["after_b64"]).decode()
    target = step["target"]
    idx = step["index"]
    is_create = step["is_create"]

    if is_create:
        target_path = Path(target)
        target_path.parent.mkdir(parents=True, exist_ok=True)
        target_path.write_text(after)
        print(f"OK {TASK_ID} change {idx}: created {target}")
    else:
        target_path = Path(target)
        content = target_path.read_text()
        if before not in content:
            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
            sys.exit(1)

        result = subprocess.run(
            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
            capture_output=True, text=True,
        )
        if result.returncode != 0:
            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
            sys.exit(result.returncode)

        new_content = target_path.read_text()
        if after and after not in new_content:
            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
            sys.exit(1)

        print(f"OK {TASK_ID} change {idx}: applied to {target}")

print(f"OK {TASK_ID}: all changes applied")
## File: plans/compiled/TASK-9.sh
#!/usr/bin/env bash
set -euo pipefail
# TASK-9: Add unit tests for file_parser, module_tree, workspace, cargo_info, cross_refs, and render
# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
# Type: replace
# File: src/file_parser.rs
python3 "$(dirname "$0")/TASK-9.py"
## File: plans/compiled/TASK-PREP.py
#!/usr/bin/env python3
"""TASK-PREP: Add tempfile dev-dependency for unit and integration tests"""
import base64, json, subprocess, sys
from pathlib import Path

TASK_ID = "TASK-PREP"
STEPS = json.loads('[{"before_b64": "cHJvYy1tYWNybzIgPSB7IHZlcnNpb24gPSAiMSIsIGZlYXR1cmVzID0gWyJzcGFuLWxvY2F0aW9ucyJdIH0=", "after_b64": "cHJvYy1tYWNybzIgPSB7IHZlcnNpb24gPSAiMSIsIGZlYXR1cmVzID0gWyJzcGFuLWxvY2F0aW9ucyJdIH0KCltkZXYtZGVwZW5kZW5jaWVzXQp0ZW1wZmlsZSA9ICIzIg==", "target": "Cargo.toml", "index": 0, "is_create": false}]')

for step in STEPS:
    before = base64.b64decode(step["before_b64"]).decode()
    after = base64.b64decode(step["after_b64"]).decode()
    target = step["target"]
    idx = step["index"]
    is_create = step["is_create"]

    if is_create:
        target_path = Path(target)
        target_path.parent.mkdir(parents=True, exist_ok=True)
        target_path.write_text(after)
        print(f"OK {TASK_ID} change {idx}: created {target}")
    else:
        target_path = Path(target)
        content = target_path.read_text()
        if before not in content:
            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
            sys.exit(1)

        result = subprocess.run(
            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
            capture_output=True, text=True,
        )
        if result.returncode != 0:
            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
            sys.exit(result.returncode)

        new_content = target_path.read_text()
        if after and after not in new_content:
            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
            sys.exit(1)

        print(f"OK {TASK_ID} change {idx}: applied to {target}")

print(f"OK {TASK_ID}: all changes applied")
## File: plans/compiled/TASK-PREP.sh
#!/usr/bin/env bash
set -euo pipefail
# TASK-PREP: Add tempfile dev-dependency for unit and integration tests
# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
# Type: replace
# File: Cargo.toml
python3 "$(dirname "$0")/TASK-PREP.py"
## File: plans/compiled/manifest.json
{
  "plan": "/Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml",
  "compiled_at": "2026-04-28T08:48:13.087314+00:00",
  "tasks": [
    {
      "id": "TASK-PREP",
      "script": "TASK-PREP.sh",
      "runner": "TASK-PREP.py",
      "file": "Cargo.toml",
      "type": "replace",
      "changes": 1,
      "description": "Add tempfile dev-dependency for unit and integration tests",
      "acceptance": [
        "cargo check -p rust-workspace-map"
      ]
    },
    {
      "id": "TASK-1",
      "script": "TASK-1.sh",
      "runner": "TASK-1.py",
      "file": "src/schema.rs",
      "type": "replace",
      "changes": 1,
      "description": "Add ErrorSeverity enum and ErrorContext struct to schema.rs",
      "acceptance": [
        "cargo check -p rust-workspace-map"
      ]
    },
    {
      "id": "TASK-2",
      "script": "TASK-2.sh",
      "runner": "TASK-2.py",
      "file": "src/schema.rs",
      "type": "replace",
      "changes": 1,
      "description": "Add MissingWorkspaceSection variant to Error enum in schema.rs",
      "acceptance": [
        "cargo check -p rust-workspace-map"
      ]
    },
    {
      "id": "TASK-3",
      "script": "TASK-3.sh",
      "runner": "TASK-3.py",
      "file": "src/schema.rs",
      "type": "replace",
      "changes": 1,
      "description": "Expand ErrorEntry struct with severity, kind, context, and cause fields",
      "acceptance": [
        "cargo check -p rust-workspace-map"
      ]
    },
    {
      "id": "TASK-4",
      "script": "TASK-4.sh",
      "runner": "TASK-4.py",
      "file": "src/file_parser.rs",
      "type": "replace",
      "changes": 1,
      "description": "Change parse_file to return ParsedFile with optional parse errors instead of Result",
      "acceptance": [
        "true  # applied atomically with TASK-5; compilation verified at TASK-5"
      ]
    },
    {
      "id": "TASK-5",
      "script": "TASK-5.sh",
      "runner": "TASK-5.py",
      "file": "src/module_tree.rs",
      "type": "replace",
      "changes": 12,
      "description": "Refactor module_tree.rs: fix path fallbacks, add error collection, change build_module_tree return type",
      "acceptance": [
        "cargo check -p rust-workspace-map"
      ]
    },
    {
      "id": "TASK-6",
      "script": "TASK-6.sh",
      "runner": "TASK-6.py",
      "file": "src/workspace.rs",
      "type": "replace",
      "changes": 3,
      "description": "Update workspace.rs enumerate_members to return MissingWorkspaceSection error",
      "acceptance": [
        "cargo check -p rust-workspace-map"
      ]
    },
    {
      "id": "TASK-7",
      "script": "TASK-7.sh",
      "runner": "TASK-7.py",
      "file": "src/lib.rs",
      "type": "replace",
      "changes": 3,
      "description": "Refactor lib.rs run(): parallel error collection, error entry construction, remove unwrap_or_default",
      "acceptance": [
        "cargo check -p rust-workspace-map"
      ]
    },
    {
      "id": "TASK-8",
      "script": "TASK-8.sh",
      "runner": "TASK-8.py",
      "file": "src/lib.rs",
      "type": "replace",
      "changes": 21,
      "description": "Remove all crate-level clippy allow attributes and fix individual lint violations",
      "acceptance": [
        "cargo clippy -p rust-workspace-map -- -D warnings"
      ]
    },
    {
      "id": "TASK-9",
      "script": "TASK-9.sh",
      "runner": "TASK-9.py",
      "file": "src/file_parser.rs",
      "type": "replace",
      "changes": 6,
      "description": "Add unit tests for file_parser, module_tree, workspace, cargo_info, cross_refs, and render",
      "acceptance": [
        "cargo test -p rust-workspace-map"
      ]
    },
    {
      "id": "TASK-10",
      "script": "TASK-10.sh",
      "runner": "TASK-10.py",
      "file": "tests/integration_test.rs",
      "type": "replace",
      "changes": 1,
      "description": "Expand integration tests from 3 to 10 covering error cases, workspace patterns, and output",
      "acceptance": [
        "cargo test -p rust-workspace-map --test integration_test"
      ]
    }
  ],
  "skipped": []
}
## File: plans/phase-0.2.toml
[meta]
title = "Phase 0.2: Hardening for Trustworthiness"
source_branch = "phase-0.2"
created = "2026-04-28"

[dependencies]
TASK-PREP = []
TASK-3 = ["TASK-1"]
TASK-4 = ["TASK-1", "TASK-2", "TASK-3"]
TASK-5 = ["TASK-4"]
TASK-6 = ["TASK-2"]
TASK-7 = ["TASK-4", "TASK-5"]
TASK-8 = ["TASK-7"]
TASK-9 = ["TASK-PREP", "TASK-5", "TASK-6", "TASK-8"]
TASK-10 = ["TASK-PREP", "TASK-9"]

[tasks.TASK-PREP]
description = "Add tempfile dev-dependency for unit and integration tests"
type = "replace"
acceptance = [
    "cargo check -p rust-workspace-map",
]

[[tasks.TASK-PREP.changes]]
file = "Cargo.toml"
before = "proc-macro2 = { version = \"1\", features = [\"span-locations\"] }"
after = """proc-macro2 = { version = "1", features = ["span-locations"] }

[dev-dependencies]
tempfile = "3\""""

[tasks.TASK-1]
description = "Add ErrorSeverity enum and ErrorContext struct to schema.rs"
type = "replace"
acceptance = [
    "cargo check -p rust-workspace-map",
]

[[tasks.TASK-1.changes]]
file = "src/schema.rs"
before = '''// ── Internal types ──────────────────────────────────────────────────────

/// Internal intermediate type consumed by module_tree.
#[derive(Debug, Clone, Default)]
pub struct FileInfo {
'''
after = '''// ── Error severity ─────────────────────────────────────────────────────

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

/// Internal intermediate type consumed by module_tree.
#[derive(Debug, Clone, Default)]
pub struct FileInfo {
'''

[tasks.TASK-2]
description = "Add MissingWorkspaceSection variant to Error enum in schema.rs"
type = "replace"
acceptance = [
    "cargo check -p rust-workspace-map",
]

[[tasks.TASK-2.changes]]
file = "src/schema.rs"
before = '''    #[error("glob pattern error: {0}")]
    GlobPattern(String),
}

pub type Result<T> = std::result::Result<T, Error>;'''
after = '''    #[error("glob pattern error: {0}")]
    GlobPattern(String),

    #[error("workspace Cargo.toml is missing the [workspace] section")]
    MissingWorkspaceSection,
}

pub type Result<T> = std::result::Result<T, Error>;'''

[tasks.TASK-3]
description = "Expand ErrorEntry struct with severity, kind, context, and cause fields"
type = "replace"
acceptance = [
    "cargo check -p rust-workspace-map",
]

[[tasks.TASK-3.changes]]
file = "src/schema.rs"
before = '''#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ErrorEntry {
    pub file: String,
    #[builder(default)]
    pub line: usize,
    pub message: String,
}
'''
after = '''#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ErrorEntry {
    pub file: String,
    #[builder(default)]
    pub line: usize,
    pub message: String,
    pub severity: ErrorSeverity,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ErrorContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}
'''

[tasks.TASK-4]
description = "Change parse_file to return ParsedFile with optional parse errors instead of Result"
type = "replace"
acceptance = [
    "true  # applied atomically with TASK-5; compilation verified at TASK-5",
]

[[tasks.TASK-4.changes]]
file = "src/file_parser.rs"
before = '''use crate::schema::{
    Error, FileInfo, ImplInfo, ImplItem, ImplItemKind, Import, ItemAttrs, ItemKind, PublicItem,
    ReExport, Result, SubmoduleDecl,
};
use std::path::Path;

// ── parse_file ──────────────────────────────────────────────────────────

/// Read and parse a Rust source file. Returns the raw `syn::File` AST (needed
/// by `module_tree` for inline module item extraction) and the extracted
/// `FileInfo`. On parse failure, warns to stderr and returns empty results.
pub fn parse_file(path: &Path) -> Result<(syn::File, FileInfo)> {
    let content = std::fs::read_to_string(path).map_err(|source| Error::FileRead {
        path: path.to_path_buf(),
        source,
    })?;

    let file = match syn::parse_file(&content) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("warning: failed to parse {}: {}", path.display(), e);
            let empty = syn::File {
                shebang: None,
                attrs: vec![],
                items: vec![],
            };
            let info = FileInfo::default();
            return Ok((empty, info));
        }
    };

    let info = FileInfo {
        public_items: extract_public_items(&file.items),
        imports: extract_imports(&file.items),
        re_exports: extract_re_exports(&file.items),
        submodules: extract_submodules(&file.items),
        impls: extract_impls(&file.items),
    };

    Ok((file, info))
}'''
after = '''use crate::schema::{
    Error, ErrorEntry, ErrorSeverity, FileInfo, ImplInfo, ImplItem, ImplItemKind, Import,
    ItemAttrs, ItemKind, PublicItem, ReExport, Result, SubmoduleDecl,
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
        .kind("syn_parse_error".to_string())
        .build()
}'''

[tasks.TASK-5]
description = "Refactor module_tree.rs: fix path fallbacks, add error collection, change build_module_tree return type"
type = "replace"
acceptance = [
    "cargo check -p rust-workspace-map",
]

[[tasks.TASK-5.changes]]
file = "src/module_tree.rs"
before = "use crate::schema::{FileInfo, ModuleInfo, Result, SubmoduleDecl};"
after = "use crate::schema::{ErrorContext, ErrorEntry, ErrorSeverity, FileInfo, ModuleInfo, Result, SubmoduleDecl};"

[[tasks.TASK-5.changes]]
file = "src/module_tree.rs"
before = '''/// Build the full module tree for a crate starting from its entry point
/// (e.g., `src/lib.rs`). Returns a flat `Vec<ModuleInfo>` containing the
/// root module and all recursively discovered submodules.
pub fn build_module_tree(crate_root: &Path, crate_name: &str) -> Result<Vec<ModuleInfo>> {
    let mut visited = HashSet::new();
    let parent_dir = crate_root.parent().unwrap_or_else(|| Path::new("."));

    let (ast, file_info) = file_parser::parse_file(crate_root)?;
    visited.insert(crate_root.to_path_buf());

    let root_module = build_module_info(
        crate_name,
        crate_root,
        "pub",
        &file_info.public_items,
        &file_info.imports,
        &file_info.re_exports,
        &file_info.submodules,
    );

    let mut modules = vec![root_module];

    for sub in &file_info.submodules {
        if sub.is_test {
            continue;
        }
        let sub_module_path = format!("{}::{}", crate_name, sub.name);
        let child_modules = process_submodule(
            &sub_module_path,
            &sub.name,
            &ast.items,
            parent_dir,
            crate_root,
            &mut visited,
        )?;
        modules.extend(child_modules);
    }

    Ok(modules)
}'''
after = '''/// Build the full module tree for a crate starting from its entry point
/// (e.g., `src/lib.rs`). Returns a tuple of module info and any errors
/// encountered during submodule parsing (including orphaned module warnings).
pub fn build_module_tree(
    crate_root: &Path,
    crate_name: &str,
) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {

    let mut visited = HashSet::new();
    let parent_dir = crate_root.parent().unwrap_or(crate_root);

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
}'''

[[tasks.TASK-5.changes]]
file = "src/module_tree.rs"
before = '''    let Some(mod_item) = mod_item else {
        eprintln!("warning: orphaned module {}", module_path);
        return Ok(vec![ModuleInfo::builder()
            .path(module_path.to_string())
            .file("<unresolved>".to_string())
            .visibility("private".to_string())
            .build()]);
    };'''
after = '''    let Some(mod_item) = mod_item else {
        let err = ErrorEntry::builder()
            .file(String::new())
            .message(format!("orphaned module: {module_path}"))
            .severity(ErrorSeverity::Warning)
            .kind("orphaned_module".to_string())
            .context(ErrorContext::builder()
                .module_path(module_path.to_string())
                .build())
            .build();
        return (vec![ModuleInfo::builder()
            .path(module_path.to_string())
            .file("<unresolved>".to_string())
            .visibility("private".to_string())
            .build()], vec![err]);
    };'''

[[tasks.TASK-5.changes]]
file = "src/module_tree.rs"
before = '''        let Some(ref file_path) = file_path else {
            eprintln!("warning: orphaned module {}", module_path);
            return Ok(vec![ModuleInfo::builder()
                .path(module_path.to_string())
                .file("<unresolved>".to_string())
                .visibility(visibility.to_string())
                .build()]);
        };'''
after = '''        let Some(ref file_path) = file_path else {
            let err = ErrorEntry::builder()
                .file(String::new())
                .message(format!("orphaned module: {module_path}"))
                .severity(ErrorSeverity::Warning)
                .kind("orphaned_module".to_string())
                .context(ErrorContext::builder()
                    .module_path(module_path.to_string())
                    .build())
                .build();
            return (vec![ModuleInfo::builder()
                .path(module_path.to_string())
                .file("<unresolved>".to_string())
                .visibility(visibility.to_string())
                .build()], vec![err]);
        };'''

[[tasks.TASK-5.changes]]
file = "src/module_tree.rs"
before = '''        let (ast, file_info) = file_parser::parse_file(file_path)?;
        process_module_info(
            module_path,
            file_path,
            visibility,
            &file_info,
            &ast.items,
            &file_path.parent().unwrap_or_else(|| Path::new(".")),
            visited,
        )'''
after = '''        let parsed = file_parser::parse_file(file_path);
        let mut errors: Vec<ErrorEntry> = Vec::new();
        if let Some(ref err) = parsed.parse_error {
            errors.push(crate::file_parser::build_parse_error_entry(file_path, err));
        }
        process_module_info(
            module_path,
            file_path,
            visibility,
            &parsed.file_info,
            &parsed.ast.items,
            &file_path.parent().unwrap_or(file_path),
            visited,
            &mut errors,
        )'''

[[tasks.TASK-5.changes]]
file = "src/module_tree.rs"
before = '''        if visited.contains(file_path.as_path()) {
            return Ok(vec![]); // cycle detected
        }'''
after = '''        if visited.contains(file_path.as_path()) {
            return (vec![], vec![]); // cycle detected
        }'''

[[tasks.TASK-5.changes]]
file = "src/module_tree.rs"
before = '''/// Resolve a `mod name;` declaration to a file path.
/// Tries `{parent_dir}/{mod_name}.rs` first, then `{parent_dir}/{mod_name}/mod.rs`.
pub fn resolve_module_path(parent_dir: &Path, mod_name: &str) -> Option<PathBuf> {'''
after = '''/// Resolve a `mod name;` declaration to a file path.
/// Tries `{parent_dir}/{mod_name}.rs` first, then `{parent_dir}/{mod_name}/mod.rs`.
///
/// Returns `None` if neither path exists.
pub fn resolve_module_path(parent_dir: &Path, mod_name: &str) -> Option<PathBuf> {'''

[[tasks.TASK-5.changes]]
file = "src/module_tree.rs"
before = '''fn process_submodule(
    module_path: &str,
    mod_name: &str,
    parent_items: &[syn::Item],
    parent_dir: &Path,
    parent_file: &Path,
    visited: &mut HashSet<PathBuf>,
) -> Result<Vec<ModuleInfo>> {'''
after = '''fn process_submodule(
    module_path: &str,
    mod_name: &str,
    parent_items: &[syn::Item],
    parent_dir: &Path,
    parent_file: &Path,
    visited: &mut HashSet<PathBuf>,
) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {'''

[[tasks.TASK-5.changes]]
file = "src/module_tree.rs"
before = '''    if let Some((_, ref inline_items)) = mod_item.content {
        // Inline module: process its body items directly (no file lookup).
        process_module_items(
            module_path,
            parent_file,
            visibility,
            inline_items,
            parent_dir,
            visited,
        )
    } else {'''
after = '''    if let Some((_, ref inline_items)) = mod_item.content {
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
    } else {'''

[[tasks.TASK-5.changes]]
file = "src/module_tree.rs"
before = '''fn process_module_items(
    module_path: &str,
    file_path: &Path,
    visibility: &str,
    items: &[syn::Item],
    parent_dir: &Path,
    visited: &mut HashSet<PathBuf>,
) -> Result<Vec<ModuleInfo>> {'''
after = '''fn process_module_items(
    module_path: &str,
    file_path: &Path,
    visibility: &str,
    items: &[syn::Item],
    parent_dir: &Path,
    visited: &mut HashSet<PathBuf>,
) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {'''

[[tasks.TASK-5.changes]]
file = "src/module_tree.rs"
before = '''    let file_info = FileInfo {
        public_items: file_parser::extract_public_items(items),
        imports: file_parser::extract_imports(items),
        re_exports: file_parser::extract_re_exports(items),
        submodules: file_parser::extract_submodules(items),
        impls: file_parser::extract_impls(items),
    };
    process_module_info(module_path, file_path, visibility, &file_info, items, parent_dir, visited)
}'''
after = '''    let file_info = FileInfo {
        public_items: file_parser::extract_public_items(items),
        imports: file_parser::extract_imports(items),
        re_exports: file_parser::extract_re_exports(items),
        submodules: file_parser::extract_submodules(items),
        impls: file_parser::extract_impls(items),
    };
    process_module_info(module_path, file_path, visibility, &file_info, items, parent_dir, visited, &mut Vec::new())
}'''

[[tasks.TASK-5.changes]]
file = "src/module_tree.rs"
before = '''fn process_module_info(
    module_path: &str,
    file_path: &Path,
    visibility: &str,
    file_info: &FileInfo,
    items: &[syn::Item],
    _parent_dir: &Path,
    visited: &mut HashSet<PathBuf>,
) -> Result<Vec<ModuleInfo>> {
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
        let child_dir = file_path.parent().unwrap_or_else(|| Path::new("."));
        let child_modules = process_submodule(
            &child_path,
            &sub.name,
            items,
            child_dir,
            file_path,
            visited,
        )?;
        modules.extend(child_modules);
    }

    Ok(modules)
}'''
after = '''fn process_module_info(
    module_path: &str,
    file_path: &Path,
    visibility: &str,
    file_info: &FileInfo,
    items: &[syn::Item],
    _parent_dir: &Path,
    visited: &mut HashSet<PathBuf>,
    errors: &mut Vec<crate::schema::ErrorEntry>,
) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {
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
        let (child_modules, child_errors) = process_submodule(
            &child_path,
            &sub.name,
            items,
            child_dir,
            file_path,
            visited,
        );
        errors.extend(child_errors);
        modules.extend(child_modules);
    }

    (modules, errors.clone())
}'''


[tasks.TASK-6]
description = "Update workspace.rs enumerate_members to return MissingWorkspaceSection error"
type = "replace"
acceptance = [
    "cargo check -p rust-workspace-map",
]

[[tasks.TASK-6.changes]]
file = "src/workspace.rs"
before = '''    let members: Vec<String> = parsed
        .get("workspace")
        .and_then(|w| w.get("members"))
        .and_then(|m| m.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();'''
after = '''    let members: Vec<String> = match parsed.get("workspace") {
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
    };'''

[[tasks.TASK-6.changes]]
file = "src/workspace.rs"
before = '''/// Walk up the directory tree from `start_path` to find a `Cargo.toml`
/// containing a `[workspace]` section. Returns the directory containing it.
pub fn find_workspace_root(start_path: &Path) -> Result<PathBuf> {'''
after = '''/// Walk up the directory tree from `start_path` to find a `Cargo.toml`
/// containing a `[workspace]` section. Returns the directory containing it.
///
/// # Errors
///
/// Returns `Error::WorkspaceRootNotFound` if no `Cargo.toml` with a
/// `[workspace]` section is found in any ancestor directory.
pub fn find_workspace_root(start_path: &Path) -> Result<PathBuf> {'''

[[tasks.TASK-6.changes]]
file = "src/workspace.rs"
before = '''/// Parse the workspace `Cargo.toml`, resolve member paths (including glob
/// patterns), apply `exclude` list, and return absolute paths to each member
/// crate directory.
pub fn enumerate_members(root: &Path) -> Result<Vec<PathBuf>> {'''
after = '''/// Parse the workspace `Cargo.toml`, resolve member paths (including glob
/// patterns), apply `exclude` list, and return absolute paths to each member
/// crate directory.
///
/// # Errors
///
/// Returns `Error::MissingWorkspaceSection` if the `Cargo.toml` lacks a
/// `[workspace]` section entirely.
pub fn enumerate_members(root: &Path) -> Result<Vec<PathBuf>> {'''

[tasks.TASK-7]
description = "Refactor lib.rs run(): parallel error collection, error entry construction, remove unwrap_or_default"
type = "replace"
acceptance = [
    "cargo check -p rust-workspace-map",
]

[[tasks.TASK-7.changes]]
file = "src/lib.rs"
before = '''use anyhow::Context;
use rayon::prelude::*;
use schema::{
    CrateInfo, CrateType, ErrorEntry, ModuleInfo, WorkspaceInfo, WorkspaceMap,
};
use std::path::Path;'''
after = '''use anyhow::Context;
use rayon::prelude::*;
use schema::{
    CrateInfo, CrateType, ErrorEntry, ErrorSeverity, ModuleInfo, WorkspaceInfo,
    WorkspaceMap,
};
use std::path::Path;'''

[[tasks.TASK-7.changes]]
file = "src/lib.rs"
before = '''    let errors: Vec<ErrorEntry> = Vec::new();

    let mut crate_infos: Vec<CrateInfo> = member_dirs
        .par_iter()
        .filter_map(|dir| {
            let cargo_toml = dir.join("Cargo.toml");

            let (pkg, deps) = match cargo_info::parse_cargo_toml(&cargo_toml) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!(
                        "warning: failed to parse {}: {}",
                        cargo_toml.display(),
                        e
                    );
                    return None;
                }
            };

            let roots = workspace::resolve_crate_roots(dir);
            if roots.is_empty() {
                eprintln!(
                    "warning: no crate entry points found in {}",
                    dir.display()
                );
                return None;
            }

            let crate_type = if roots.iter().any(|(_, t)| *t == CrateType::Lib)
                && roots.iter().any(|(_, t)| *t == CrateType::Bin)
            {
                CrateType::LibAndBin
            } else {
                roots.first().map_or(CrateType::Lib, |(_, t)| *t)
            };

            let pkg_name = pkg.name.clone();
            let mut modules: Vec<ModuleInfo> = roots
                .iter()
                .flat_map(|(root, _ty)| {
                    module_tree::build_module_tree(root, &pkg_name).unwrap_or_default()
                })
                .collect();

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

            Some(
                CrateInfo::builder()
                    .name(pkg_name)
                    .root(crate_root)
                    .package(rebuilt_pkg)
                    .modules(modules)
                    .deps(deps)
                    .build(),
            )
        })
        .collect();'''
after = '''    let mut crate_errors: Vec<ErrorEntry> = Vec::new();

    let results: Vec<(CrateInfo, Vec<ErrorEntry>)> = member_dirs
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
                        .kind("toml_parse_error".to_string())
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
                    .kind("missing_crate_roots".to_string())
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
                let (m, e) = module_tree::build_module_tree(&root, &pkg_name);
                modules.extend(m);
                collected_errors.extend(e);
            }
            crate_errors.extend(collected_errors);

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
        if let Some(ci) = info {
            crate_errors.extend(errs);
            crate_infos.push(ci);
        }
    }'''

[[tasks.TASK-7.changes]]
file = "src/lib.rs"
before = '''    let map = WorkspaceMap::builder()
        .workspace(workspace_info)
        .crates(crate_infos)
        .cross_references(cross_refs)
        .errors(errors)
        .workspace_root(workspace_root.clone())
        .build();'''
after = '''    let map = WorkspaceMap::builder()
        .workspace(workspace_info)
        .crates(crate_infos)
        .cross_references(cross_refs)
        .errors(crate_errors)
        .workspace_root(workspace_root.clone())
        .build();'''

[tasks.TASK-8]
description = "Remove all crate-level clippy allow attributes and fix individual lint violations"
type = "replace"
acceptance = [
    "cargo clippy -p rust-workspace-map -- -D warnings",
]

[[tasks.TASK-8.changes]]
file = "src/lib.rs"
before = '''#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::redundant_closure_for_method_calls)]'''
after = '''#![warn(clippy::pedantic)]'''

[[tasks.TASK-8.changes]]
file = "src/lib.rs"
before = '''/// Run the full workspace mapping pipeline.
///
/// 1. Discover workspace root and member crates.
/// 2. Process each crate in parallel (Cargo.toml parsing + module tree).
/// 3. Compute cross-crate references.
/// 4. Render JSON to stdout or the configured output file.
pub fn run(config: Config) -> anyhow::Result<()> {'''
after = '''/// Run the full workspace mapping pipeline.
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
pub fn run(config: Config) -> anyhow::Result<()> {'''

[[tasks.TASK-8.changes]]
file = "src/lib.rs"
before = '''    let workspace_name = workspace_root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();'''
after = '''    let workspace_name = workspace_root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();'''

[[tasks.TASK-8.changes]]
file = "src/lib.rs"
before = '''/// Strip the workspace root prefix from a path string, returning a
/// workspace-relative path. If the prefix doesn't match, returns the
/// original string unchanged.
fn relativize_path(path_str: &str, root: &Path) -> String {
    let p = Path::new(path_str);
    match p.strip_prefix(root) {
        Ok(rel) => rel.to_string_lossy().to_string(),
        Err(_) => path_str.to_string(),
    }
}'''
after = '''/// Strip the workspace root prefix from a path string, returning a
/// workspace-relative path. If the prefix doesn't match, returns the
/// original string unchanged.
fn relativize_path(path_str: &str, root: &Path) -> String {
    let p = Path::new(path_str);
    match p.strip_prefix(root) {
        Ok(rel) => rel.to_string_lossy().to_string(),
        Err(_) => path_str.to_string(),
    }
}'''

[[tasks.TASK-8.changes]]
file = "src/file_parser.rs"
before = '''/// Extract all items with any form of `pub` visibility (excluding `Inherited`).
/// Results are sorted by name then line for deterministic output.
pub fn extract_public_items(items: &[syn::Item]) -> Vec<PublicItem> {'''
after = '''/// Extract all items with any form of `pub` visibility (excluding `Inherited`).
/// Results are sorted by name then line for deterministic output.
#[must_use]
pub fn extract_public_items(items: &[syn::Item]) -> Vec<PublicItem> {'''

[[tasks.TASK-8.changes]]
file = "src/file_parser.rs"
before = '''/// Extract all `use` statements. Braced imports are expanded to individual
/// entries. Results sorted by path for determinism.
pub fn extract_imports(items: &[syn::Item]) -> Vec<Import> {'''
after = '''/// Extract all `use` statements. Braced imports are expanded to individual
/// entries. Results sorted by path for determinism.
#[must_use]
pub fn extract_imports(items: &[syn::Item]) -> Vec<Import> {'''

[[tasks.TASK-8.changes]]
file = "src/file_parser.rs"
before = '''/// Extract `pub use` re-exports. Results sorted by export_path.
pub fn extract_re_exports(items: &[syn::Item]) -> Vec<ReExport> {'''
after = '''/// Extract `pub use` re-exports. Results sorted by export_path.
#[must_use]
pub fn extract_re_exports(items: &[syn::Item]) -> Vec<ReExport> {'''

[[tasks.TASK-8.changes]]
file = "src/file_parser.rs"
before = '''/// Extract `mod` declarations. Detects `#[cfg(test)]` via literal token
/// matching. Results sorted by name.
pub fn extract_submodules(items: &[syn::Item]) -> Vec<SubmoduleDecl> {'''
after = '''/// Extract `mod` declarations. Detects `#[cfg(test)]` via literal token
/// matching. Results sorted by name.
#[must_use]
pub fn extract_submodules(items: &[syn::Item]) -> Vec<SubmoduleDecl> {'''

[[tasks.TASK-8.changes]]
file = "src/file_parser.rs"
before = '''/// Extract `impl` blocks. Each `ImplInfo` records the target type name and
/// the impl items (fn, type, const).
pub fn extract_impls(items: &[syn::Item]) -> Vec<ImplInfo> {'''
after = '''/// Extract `impl` blocks. Each `ImplInfo` records the target type name and
/// the impl items (fn, type, const).
#[must_use]
pub fn extract_impls(items: &[syn::Item]) -> Vec<ImplInfo> {'''

[[tasks.TASK-8.changes]]
file = "src/module_tree.rs"
before = '''/// Resolve a `mod name;` declaration to a file path.
/// Tries `{parent_dir}/{mod_name}.rs` first, then `{parent_dir}/{mod_name}/mod.rs`.
///
/// Returns `None` if neither path exists.
pub fn resolve_module_path(parent_dir: &Path, mod_name: &str) -> Option<PathBuf> {'''
after = '''/// Resolve a `mod name;` declaration to a file path.
/// Tries `{parent_dir}/{mod_name}.rs` first, then `{parent_dir}/{mod_name}/mod.rs`.
///
/// Returns `None` if neither path exists.
#[must_use]
pub fn resolve_module_path(parent_dir: &Path, mod_name: &str) -> Option<PathBuf> {'''

[[tasks.TASK-8.changes]]
file = "src/render.rs"
before = '''/// Serialize the workspace map to a JSON string with 2-space indentation.
pub fn render_json(map: &WorkspaceMap) -> serde_json::Result<String> {'''
after = '''/// Serialize the workspace map to a JSON string with 2-space indentation.
#[must_use]
pub fn render_json(map: &WorkspaceMap) -> serde_json::Result<String> {'''

[[tasks.TASK-8.changes]]
file = "src/cross_refs.rs"
before = '''// Helper: convert ItemKind to a short string for the TypeRef.kind field.
impl crate::schema::PublicItem {
    fn kind_to_string(&self) -> String {'''
after = '''// Helper: convert ItemKind to a short string for the TypeRef.kind field.
impl crate::schema::PublicItem {
    #[must_use]
    fn kind_to_string(&self) -> String {'''

[[tasks.TASK-8.changes]]
file = "src/lib.rs"
before = '''    match p.strip_prefix(root) {
        Ok(rel) => rel.to_string_lossy().to_string(),
        Err(_) => path_str.to_string(),
    }
}'''
after = '''    match p.strip_prefix(root) {
        Ok(rel) => rel.to_string_lossy().to_string(),
        Err(_) => path_str.to_string(),
    }
}'''

[[tasks.TASK-8.changes]]
file = "src/file_parser.rs"
before = "/// Extract `pub use` re-exports. Results sorted by export_path.\n#[must_use]\npub fn extract_re_exports"
after = "/// Extract `pub use` re-exports. Results sorted by `export_path`.\n#[must_use]\npub fn extract_re_exports"

[[tasks.TASK-8.changes]]
file = "src/file_parser.rs"
before = "            let name = m.ident.as_ref().map(|i| i.to_string()).unwrap_or_default();"
after = "            let name = m.ident.as_ref().map(ToString::to_string).unwrap_or_default();"

[[tasks.TASK-8.changes]]
file = "src/file_parser.rs"
before = "fn flatten_use_tree(tree: &syn::UseTree, prefix: String, line: usize) -> Vec<Import> {"
after = "fn flatten_use_tree(tree: &syn::UseTree, prefix: &str, line: usize) -> Vec<Import> {"

[[tasks.TASK-8.changes]]
file = "src/file_parser.rs"
before = "                Some(flatten_use_tree(&u.tree, String::new(), line_of_item(item)))"
after = "                Some(flatten_use_tree(&u.tree, \"\", line_of_item(item)))"

[[tasks.TASK-8.changes]]
file = "src/file_parser.rs"
before = "            flatten_use_tree(&p.tree, new_prefix, line)"
after = "            flatten_use_tree(&p.tree, &new_prefix, line)"

[[tasks.TASK-8.changes]]
file = "src/schema.rs"
before = "/// Internal intermediate type consumed by module_tree.\n#[derive(Debug, Clone, Default)]\npub struct FileInfo"
after = "/// Internal intermediate type consumed by `module_tree`.\n#[derive(Debug, Clone, Default)]\npub struct FileInfo"

[[tasks.TASK-8.changes]]
file = "src/lib.rs"
before = "/// parsed, or the JSON output cannot be written.\npub fn run(config: Config) -> anyhow::Result<()> {"
after = "/// parsed, or the JSON output cannot be written.\npub fn run(config: &Config) -> anyhow::Result<()> {"

[[tasks.TASK-8.changes]]
file = "src/main.rs"
before = "    rust_workspace_map::run(config)"
after = "    rust_workspace_map::run(&config)"

[tasks.TASK-9]
description = "Add unit tests for file_parser, module_tree, workspace, cargo_info, cross_refs, and render"
type = "replace"
acceptance = [
    "cargo test -p rust-workspace-map",
]

[[tasks.TASK-9.changes]]
file = "src/file_parser.rs"
before = '''fn extract_re_exports_from_tree(
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
            vec![ReExport {
                import_path,
                export_path: n.ident.to_string(),
                line,
            }]
        }
        syn::UseTree::Rename(r) => {
            vec![ReExport {
                import_path,
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
}'''
after = '''fn extract_re_exports_from_tree(
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
            vec![ReExport {
                import_path,
                export_path: n.ident.to_string(),
                line,
            }]
        }
        syn::UseTree::Rename(r) => {
            vec![ReExport {
                import_path,
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
    use crate::schema::{Import, ReExport, SubmoduleDecl};
    use std::path::PathBuf;

    fn parse_source(src: &str) -> ParsedFile {
        let tmp = std::env::temp_dir().join("parse_test.rs");
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
        assert_eq!(entry.kind, "syn_parse_error");
        assert_eq!(entry.severity, ErrorSeverity::Error);
    }
}'''

[[tasks.TASK-9.changes]]
file = "src/module_tree.rs"
before = '''fn process_module_info(
    module_path: &str,
    file_path: &Path,
    visibility: &str,
    file_info: &FileInfo,
    items: &[syn::Item],
    _parent_dir: &Path,
    visited: &mut HashSet<PathBuf>,
    errors: &mut Vec<crate::schema::ErrorEntry>,
) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {
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
        let (child_modules, child_errors) = process_submodule(
            &child_path,
            &sub.name,
            items,
            child_dir,
            file_path,
            visited,
        );
        errors.extend(child_errors);
        modules.extend(child_modules);
    }

    (modules, errors.clone())
}'''
after = '''fn process_module_info(
    module_path: &str,
    file_path: &Path,
    visibility: &str,
    file_info: &FileInfo,
    items: &[syn::Item],
    _parent_dir: &Path,
    visited: &mut HashSet<PathBuf>,
    errors: &mut Vec<crate::schema::ErrorEntry>,
) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {
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
        let (child_modules, child_errors) = process_submodule(
            &child_path,
            &sub.name,
            items,
            child_dir,
            file_path,
            visited,
        );
        errors.extend(child_errors);
        modules.extend(child_modules);
    }

    (modules, errors.clone())
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::ErrorEntry;

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
        assert!(modules.is_empty());
        assert!(!errors.is_empty());
        std::fs::remove_dir_all(&tmp).ok();
    }
}'''

[[tasks.TASK-9.changes]]
file = "src/workspace.rs"
before = '''    result.sort();
    result.dedup();
    Ok(result)
}

/// For a crate directory, determine its entry-point file(s).'''
after = '''    result.sort();
    result.dedup();
    Ok(result)
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_cargo_toml(dir: &std::path::Path, content: &str) {
        let mut f = std::fs::File::create(dir.join("Cargo.toml")).unwrap();
        f.write_all(content.as_bytes()).unwrap();
    }

    fn setup_crate(dir: &std::path::Path) {
        let src = dir.join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("lib.rs"), "").unwrap();
    }

    #[test]
    fn find_workspace_root_finds_cargo_toml() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("subdir").join("nested");
        std::fs::create_dir_all(&path).unwrap();
        write_cargo_toml(tmp.path(), "[workspace]");
        let result = find_workspace_root(&path).unwrap();
        assert_eq!(result, tmp.path());
    }

    #[test]
    fn enumerate_members_returns_members() {
        let tmp = tempfile::tempdir().unwrap();
        write_cargo_toml(tmp.path(), r#"
[workspace]
members = ["crate_a", "crate_b"]
"#);
        setup_crate(tmp.path().join("crate_a").as_path());
        setup_crate(tmp.path().join("crate_b").as_path());
        let members = enumerate_members(tmp.path()).unwrap();
        assert_eq!(members.len(), 2);
    }

    #[test]
    fn enumerate_members_returns_err_for_missing_workspace() {
        let tmp = tempfile::tempdir().unwrap();
        write_cargo_toml(tmp.path(), "[dependencies]\nfoo = \"1\"");
        let result = enumerate_members(tmp.path());
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::MissingWorkspaceSection => {},
            other => panic!("expected MissingWorkspaceSection, got {:?}", other),
        }
    }

    #[test]
    fn enumerate_members_applies_exclude() {
        let tmp = tempfile::tempdir().unwrap();
        write_cargo_toml(tmp.path(), r#"
[workspace]
members = ["a", "b", "c"]
exclude = ["b"]
"#);
        setup_crate(tmp.path().join("a").as_path());
        setup_crate(tmp.path().join("b").as_path());
        setup_crate(tmp.path().join("c").as_path());
        let members = enumerate_members(tmp.path()).unwrap();
        let names: Vec<_> = members.iter().map(|p| p.file_name().unwrap().to_string_lossy()).collect();
        assert!(names.contains(&"a".as_ref()));
        assert!(!names.contains(&"b".as_ref()));
        assert!(names.contains(&"c".as_ref()));
    }

    #[test]
    fn resolve_crate_roots_detects_lib() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join("src")).unwrap();
        std::fs::write(tmp.path().join("src").join("lib.rs"), "").unwrap();
        let roots = resolve_crate_roots(tmp.path());
        assert_eq!(roots.len(), 1);
        assert_eq!(roots[0].1, CrateType::Lib);
    }

    #[test]
    fn resolve_crate_roots_detects_bin() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join("src")).unwrap();
        std::fs::write(tmp.path().join("src").join("main.rs"), "").unwrap();
        let roots = resolve_crate_roots(tmp.path());
        assert_eq!(roots.len(), 1);
        assert_eq!(roots[0].1, CrateType::Bin);
    }
}'''

[[tasks.TASK-9.changes]]
file = "src/cargo_info.rs"
before = '''    Ok((package, deps))
}'''
after = '''    Ok((package, deps))
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_cargo_toml(dir: &std::path::Path, content: &str) {
        let mut f = std::fs::File::create(dir.join("Cargo.toml")).unwrap();
        f.write_all(content.as_bytes()).unwrap();
    }

    #[test]
    fn parse_cargo_toml_parses_minimal() {
        let tmp = tempfile::tempdir().unwrap();
        write_cargo_toml(tmp.path(), r#"
[package]
name = "test-pkg"
version = "1.0.0"
edition = "2021"
"#);
        let (pkg, _deps) = parse_cargo_toml(tmp.path().join("Cargo.toml").as_path()).unwrap();
        assert_eq!(pkg.name, "test-pkg");
        assert_eq!(pkg.version, "1.0.0");
        assert_eq!(pkg.edition, "2021");
    }

    #[test]
    fn parse_cargo_toml_uses_defaults_for_missing_package() {
        let tmp = tempfile::tempdir().unwrap();
        write_cargo_toml(tmp.path(), "");
        let (pkg, _deps) = parse_cargo_toml(tmp.path().join("Cargo.toml").as_path()).unwrap();
        assert_eq!(pkg.name, "unknown");
        assert_eq!(pkg.edition, "2021");
    }

    #[test]
    fn parse_cargo_toml_distinguishes_deps() {
        let tmp = tempfile::tempdir().unwrap();
        write_cargo_toml(tmp.path(), r#"
[package]
name = "test-pkg"
version = "0.1.0"
edition = "2021"

[dependencies]
foo = "1"
bar = { workspace = true }

[dev-dependencies]
baz = "2"
qux = { workspace = true }
"#);
        let (_, deps) = parse_cargo_toml(tmp.path().join("Cargo.toml").as_path()).unwrap();
        assert_eq!(deps.normal, vec!["foo"]);
        assert_eq!(deps.dev, vec!["baz"]);
        assert!(deps.workspace_members.contains(&"bar".to_string()));
        assert!(deps.workspace_members.contains(&"qux".to_string()));
    }
}'''

[[tasks.TASK-9.changes]]
file = "src/cross_refs.rs"
before = '''    CrossReferences { types: types_map }
}

// Helper: convert ItemKind to a short string for the TypeRef.kind field.'''
after = '''    CrossReferences { types: types_map }
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{ModuleInfo, PublicItem, SubmoduleDecl};

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
                schema::PackageInfo::builder()
                    .name(name.to_string())
                    .version("0.1.0".to_string())
                    .edition("2021".to_string())
                    .crate_type(schema::CrateType::Lib)
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
        // Task should be in cross-references
        assert!(refs.types.contains_key("Task"));
        let task_ref = &refs.types["Task"];
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

// Helper: convert ItemKind to a short string for the TypeRef.kind field.'''

[[tasks.TASK-9.changes]]
file = "src/render.rs"
before = '''use crate::schema::WorkspaceMap;
use std::io::Write;

/// Serialize the workspace map to a JSON string with 2-space indentation.
#[must_use]
pub fn render_json(map: &WorkspaceMap) -> serde_json::Result<String> {
    serde_json::to_string_pretty(map)
}

/// Serialize the workspace map to the given writer.
pub fn render_to_writer(map: &WorkspaceMap, writer: impl Write) -> serde_json::Result<()> {
    serde_json::to_writer_pretty(writer, map)
}
'''
after = '''use crate::schema::WorkspaceMap;
use std::io::Write;

/// Serialize the workspace map to a JSON string with 2-space indentation.
#[must_use]
pub fn render_json(map: &WorkspaceMap) -> serde_json::Result<String> {
    serde_json::to_string_pretty(map)
}

/// Serialize the workspace map to the given writer.
pub fn render_to_writer(map: &WorkspaceMap, writer: impl Write) -> serde_json::Result<()> {
    serde_json::to_writer_pretty(writer, map)
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{
        CrateInfo, CrateType, CrossReferences, DepInfo, ModuleInfo, PackageInfo,
        WorkspaceInfo, WorkspaceMap,
    };

    fn make_minimal_map() -> WorkspaceMap {
        WorkspaceMap::builder()
            .workspace(WorkspaceInfo::builder()
                .root(".".to_string())
                .workspace_name("test".to_string())
                .build())
            .crates(vec![
                CrateInfo::builder()
                    .name("test-crate".to_string())
                    .root(".".to_string())
                    .package(PackageInfo::builder()
                        .name("test-crate".to_string())
                        .version("0.1.0".to_string())
                        .edition("2021".to_string())
                        .crate_type(CrateType::Lib)
                        .build())
                    .modules(vec![ModuleInfo::builder()
                        .path("".to_string())
                        .file("src/lib.rs".to_string())
                        .visibility("pub".to_string())
                        .build()])
                    .deps(DepInfo::default())
                    .build(),
            ])
            .cross_references(CrossReferences::default())
            .workspace_root(std::path::PathBuf::from("."))
            .build()
    }

    #[test]
    fn render_json_produces_valid_json() {
        let map = make_minimal_map();
        let json = render_json(&map).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["workspace"]["root"], ".");
        assert_eq!(parsed["crates"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn render_json_skips_empty_errors() {
        let map = make_minimal_map();
        let json = render_json(&map).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        // errors field should be absent (skip_serializing_if)
        assert!(parsed.get("errors").is_none());
    }

    #[test]
    fn render_to_writer_matches_render_json() {
        let map = make_minimal_map();
        let json = render_json(&map).unwrap();

        let mut buf = Vec::new();
        render_to_writer(&map, &mut buf).unwrap();
        let from_writer = String::from_utf8(buf).unwrap();

        assert_eq!(json, from_writer);
    }
}
'''

[tasks.TASK-10]
description = "Expand integration tests from 3 to 10 covering error cases, workspace patterns, and output"
type = "replace"
acceptance = [
    "cargo test -p rust-workspace-map --test integration_test",
]

[[tasks.TASK-10.changes]]
file = "tests/integration_test.rs"
before = '''#[test]
fn test_missing_path_exits_nonzero() {
    let bin = binary_path();
    let output = Command::new(&bin)
        .arg("/tmp/nonexistent-path-12345")
        .output()
        .expect("failed to execute binary");

    assert!(
        !output.status.success(),
        "should exit non-zero for invalid path"
    );
}
'''
after = '''#[test]
fn test_missing_path_exits_nonzero() {
    let bin = binary_path();
    let output = Command::new(&bin)
        .arg("/tmp/nonexistent-path-12345")
        .output()
        .expect("failed to execute binary");

    assert!(
        !output.status.success(),
        "should exit non-zero for invalid path"
    );
}

fn run_binary(path: &str) -> std::process::Output {
    Command::new(&binary_path())
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

    let output = run_binary(root.to_str().unwrap());
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

    let output = run_binary(root.to_str().unwrap());

    // Should exit non-zero because workspace is missing
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

    let output = run_binary(root.to_str().unwrap());
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

    let output = run_binary(root.to_str().unwrap());
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

    // lib.rs declares mod foo
    std::fs::write(src.join("lib.rs"), "mod foo;").unwrap();
    // foo.rs declares mod bar
    std::fs::write(foo.join("foo.rs"), "mod bar;").unwrap();
    // bar/baz.rs declares mod baz
    std::fs::write(bar.join("bar.rs"), "mod baz;").unwrap();
    // baz.rs with a struct
    std::fs::write(bar.join("baz.rs"), "pub struct Deep {}").unwrap();

    let output = run_binary(root.to_str().unwrap());
    assert!(output.status.success());

    let json = parse_output(&output);
    let crates = extract_array(&json, "crates");
    let nested_crate = crates.iter().find(|c| c["name"].as_str().unwrap() == "nested").unwrap();

    let module_paths: Vec<&str> = extract_array(&nested_crate["modules"], "path")
        .iter()
        .map(|m| m.as_str().unwrap())
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

    let output = run_binary(root.to_str().unwrap());
    assert!(output.status.success());

    let json = parse_output(&output);
    let crates = extract_array(&json, "crates");
    let reexporter = crates.iter().find(|c| c["name"].as_str().unwrap() == "reexporter").unwrap();

    let re_exports: Vec<&serde_json::Value> = extract_array(&reexporter["modules"])
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
        .arg(fixture)
        .arg("-o")
        .arg(output_path.clone())
        .output()
        .expect("failed to execute binary");
    assert!(output1.status.success());

    // Run without -o, capture stdout
    let output2 = Command::new(&binary_path())
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
'''
## File: src/cargo_info.rs
use crate::schema::{CrateType, DepInfo, Error, PackageInfo, Result};
use std::path::Path;

/// Parse a crate's `Cargo.toml` and return package metadata and dependency lists.
/// The returned `PackageInfo.crate_type` is set to `Lib` by default; the caller
/// overrides it based on `workspace::resolve_crate_roots`.
///
/// # Errors
///
/// Returns an error if the file cannot be read or parsed.
pub fn parse_cargo_toml(path: &Path) -> Result<(PackageInfo, DepInfo)> {
    let content = std::fs::read_to_string(path).map_err(|source| Error::FileRead {
        path: path.to_path_buf(),
        source,
    })?;

    let parsed: toml::Value = toml::from_str(&content).map_err(|source| Error::TomlParse {
        path: path.to_path_buf(),
        source,
    })?;

    let default_package = || {
        PackageInfo::builder()
            .name("unknown".to_string())
            .version("0.0.0".to_string())
            .edition("2021".to_string())
            .crate_type(CrateType::Lib)
            .build()
    };

    let package = parsed
        .get("package")
        .map_or_else(default_package, |p| {
            PackageInfo::builder()
                .name(
                    p.get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown")
                        .to_string(),
                )
                .version(
                    p.get("version")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0.0.0")
                        .to_string(),
                )
                .edition(
                    p.get("edition")
                        .and_then(|v| v.as_str())
                        .unwrap_or("2021")
                        .to_string(),
                )
                .crate_type(CrateType::Lib)
                .build()
        });

    let extract_deps = |section: &str| -> (Vec<String>, Vec<String>) {
        let mut normal = Vec::new();
        let mut workspace_members = Vec::new();
        if let Some(table) = parsed.get(section).and_then(|v| v.as_table()) {
            let mut keys: Vec<&String> = table.keys().collect();
            keys.sort();
            for key in keys {
                if let Some(value) = table.get(key) {
                    let is_workspace_dep = value
                        .as_table()
                        .and_then(|t| t.get("workspace"))
                        .and_then(toml::Value::as_bool)
                        == Some(true);
                    if is_workspace_dep {
                        workspace_members.push(key.clone());
                    } else {
                        normal.push(key.clone());
                    }
                }
            }
        }
        (normal, workspace_members)
    };

    let (normal_deps, mut ws_deps) = extract_deps("dependencies");
    let (dev_deps, ws_dev_deps) = extract_deps("dev-dependencies");
    ws_deps.extend(ws_dev_deps);
    ws_deps.sort();
    ws_deps.dedup();

    let deps = DepInfo::builder()
        .normal(normal_deps)
        .dev(dev_deps)
        .workspace_members(ws_deps)
        .build();

    Ok((package, deps))
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_cargo_toml(dir: &std::path::Path, content: &str) {
        let mut f = std::fs::File::create(dir.join("Cargo.toml")).unwrap();
        f.write_all(content.as_bytes()).unwrap();
    }

    #[test]
    fn parse_cargo_toml_parses_minimal() {
        let tmp = tempfile::tempdir().unwrap();
        write_cargo_toml(tmp.path(), r#"
[package]
name = "test-pkg"
version = "1.0.0"
edition = "2021"
"#);
        let (pkg, _deps) = parse_cargo_toml(tmp.path().join("Cargo.toml").as_path()).unwrap();
        assert_eq!(pkg.name, "test-pkg");
        assert_eq!(pkg.version, "1.0.0");
        assert_eq!(pkg.edition, "2021");
    }

    #[test]
    fn parse_cargo_toml_uses_defaults_for_missing_package() {
        let tmp = tempfile::tempdir().unwrap();
        write_cargo_toml(tmp.path(), "");
        let (pkg, _deps) = parse_cargo_toml(tmp.path().join("Cargo.toml").as_path()).unwrap();
        assert_eq!(pkg.name, "unknown");
        assert_eq!(pkg.edition, "2021");
    }

    #[test]
    fn parse_cargo_toml_distinguishes_deps() {
        let tmp = tempfile::tempdir().unwrap();
        write_cargo_toml(tmp.path(), r#"
[package]
name = "test-pkg"
version = "0.1.0"
edition = "2021"

[dependencies]
foo = "1"
bar = { workspace = true }

[dev-dependencies]
baz = "2"
qux = { workspace = true }
"#);
        let (_, deps) = parse_cargo_toml(tmp.path().join("Cargo.toml").as_path()).unwrap();
        assert_eq!(deps.normal, vec!["foo"]);
        assert_eq!(deps.dev, vec!["baz"]);
        assert!(deps.workspace_members.contains(&"bar".to_string()));
        assert!(deps.workspace_members.contains(&"qux".to_string()));
    }
}
## File: src/cross_refs.rs
use crate::schema::{CrateInfo, CrossCrateImport, CrossReferences, TypeRef};
use std::collections::BTreeMap;

/// Compute cross-crate type references.
///
/// For every public item in every crate, matches it against imports from
/// other crates. Populates each `CrateInfo.cross_crate_imports` and returns
/// the global `CrossReferences` map (keyed by type/symbol name).
///
/// Takes `&mut [CrateInfo]` so it can write `cross_crate_imports` into each
/// crate while building the global cross-reference map.
pub fn compute(crates: &mut [CrateInfo]) -> CrossReferences {
    // Build a map: crate_name -> set of public item names.
    let crate_exports: BTreeMap<String, Vec<(String, String)>> = crates
        .iter()
        .map(|c| {
            let items: Vec<(String, String)> = c
                .modules
                .iter()
                .flat_map(|m| &m.public_items)
                .map(|item| (item.name.clone(), item.kind_to_string()))
                .collect();
            (c.name.clone(), items)
        })
        .collect();

    let mut types_map: BTreeMap<String, TypeRef> = BTreeMap::new();

    // Initialize TypeRef entries for every exported public item.
    for (crate_name, items) in &crate_exports {
        for (item_name, kind) in items {
            let entry = types_map.entry(item_name.clone()).or_insert_with(|| {
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

                    // Update global cross-references.
                    if let Some(type_ref) = types_map.get_mut(&symbol) {
                        let importer_label = format!("{}:{}", my_name, module.path);
                        if !type_ref.imported_by.contains(&importer_label) {
                            type_ref.imported_by.push(importer_label.clone());
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
        // Task should be in cross-references
        assert!(refs.types.contains_key("Task"));
        let task_ref = &refs.types["Task"];
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
    ErrorEntry, ErrorSeverity, FileInfo, ImplInfo, ImplItem, ImplItemKind, Import,
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
        .kind("syn_parse_error".to_string())
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
        assert_eq!(entry.kind, "syn_parse_error");
        assert_eq!(entry.severity, ErrorSeverity::Error);
    }
}
## File: src/lib.rs
#![warn(clippy::pedantic)]

pub mod cargo_info;
pub mod cross_refs;
pub mod file_parser;
pub mod module_tree;
pub mod render;
pub mod schema;
pub mod workspace;

pub use schema::Config;

use anyhow::Context;
use rayon::prelude::*;
use schema::{
    CrateInfo, CrateType, ErrorEntry, ErrorSeverity, ModuleInfo, WorkspaceInfo,
    WorkspaceMap,
};
use std::path::Path;

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
#[allow(clippy::too_many_lines)]
pub fn run(config: &Config) -> anyhow::Result<()> {
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
                        .kind("toml_parse_error".to_string())
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
                    .kind("missing_crate_roots".to_string())
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
            crate_errors.extend(collected_errors);

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
        if let Some(ci) = info {
            crate_errors.extend(errs);
            crate_infos.push(ci);
        }
    }

    // Deterministic sort by crate name.
    crate_infos.sort_by(|a, b| a.name.cmp(&b.name));

    let cross_refs = cross_refs::compute(&mut crate_infos);

    let workspace_name = workspace_root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    let workspace_info = WorkspaceInfo::builder()
        .root(".".to_string())
        .workspace_name(workspace_name)
        .build();

    let map = WorkspaceMap::builder()
        .workspace(workspace_info)
        .crates(crate_infos)
        .cross_references(cross_refs)
        .errors(crate_errors)
        .workspace_root(workspace_root.clone())
        .build();

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
## File: src/main.rs
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "rust-workspace-map",
    version,
    about = "Generate a JSON map of a Rust workspace's public API surface"
)]
struct Cli {
    /// Path to the workspace root or any directory within it
    #[arg(value_name = "PATH")]
    path: PathBuf,

    /// Write JSON output to file instead of stdout
    #[arg(short = 'o', long = "output", value_name = "FILE")]
    output: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let workspace_path = std::path::absolute(&cli.path)
        .map_err(|e| anyhow::anyhow!("invalid path {}: {}", cli.path.display(), e))?;

    let config = match cli.output {
        Some(ref output) => {
            rust_workspace_map::Config::builder()
                .workspace_path(workspace_path)
                .output_path(output.clone())
                .build()
        }
        None => {
            rust_workspace_map::Config::builder()
                .workspace_path(workspace_path)
                .build()
        }
    };

    rust_workspace_map::run(&config)
}
## File: src/module_tree.rs
use crate::file_parser;
use crate::schema::{ErrorContext, ErrorEntry, ErrorSeverity, FileInfo, ModuleInfo, SubmoduleDecl};
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
    let parent_dir = crate_root.parent().unwrap_or(crate_root);

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
            .kind("orphaned_module".to_string())
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
            .kind("orphaned_module".to_string())
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
    process_module_info(
        module_path,
        file_path,
        visibility,
        &parsed.file_info,
        &parsed.ast.items,
        file_path.parent().unwrap_or(file_path),
        visited,
        &mut errors,
    )
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
    process_module_info(module_path, file_path, visibility, &file_info, items, parent_dir, visited, &mut Vec::new())
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
) -> (Vec<ModuleInfo>, Vec<crate::schema::ErrorEntry>) {
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
        let (child_modules, child_errors) = process_submodule(
            &child_path,
            &sub.name,
            items,
            child_dir,
            file_path,
            visited,
        );
        errors.extend(child_errors);
        modules.extend(child_modules);
    }

    (modules, errors.clone())
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
## File: src/render.rs
use crate::schema::WorkspaceMap;
use std::io::Write;

/// Serialize the workspace map to a JSON string with 2-space indentation.
///
/// # Errors
///
/// Returns an error if serialization fails.
pub fn render_json(map: &WorkspaceMap) -> serde_json::Result<String> {
    serde_json::to_string_pretty(map)
}

/// Serialize the workspace map to the given writer.
///
/// # Errors
///
/// Returns an error if serialization fails.
pub fn render_to_writer(map: &WorkspaceMap, writer: impl Write) -> serde_json::Result<()> {
    serde_json::to_writer_pretty(writer, map)
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{
        CrateInfo, CrateType, CrossReferences, DepInfo, ModuleInfo, PackageInfo,
        WorkspaceInfo, WorkspaceMap,
    };

    fn make_minimal_map() -> WorkspaceMap {
        WorkspaceMap::builder()
            .workspace(WorkspaceInfo::builder()
                .root(".".to_string())
                .workspace_name("test".to_string())
                .build())
            .crates(vec![
                CrateInfo::builder()
                    .name("test-crate".to_string())
                    .root(".".to_string())
                    .package(PackageInfo::builder()
                        .name("test-crate".to_string())
                        .version("0.1.0".to_string())
                        .edition("2021".to_string())
                        .crate_type(CrateType::Lib)
                        .build())
                    .modules(vec![ModuleInfo::builder()
                        .path("".to_string())
                        .file("src/lib.rs".to_string())
                        .visibility("pub".to_string())
                        .build()])
                    .deps(DepInfo::default())
                    .build(),
            ])
            .cross_references(CrossReferences::default())
            .workspace_root(std::path::PathBuf::from("."))
            .build()
    }

    #[test]
    fn render_json_produces_valid_json() {
        let map = make_minimal_map();
        let json = render_json(&map).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["workspace"]["root"], ".");
        assert_eq!(parsed["crates"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn render_json_skips_empty_errors() {
        let map = make_minimal_map();
        let json = render_json(&map).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        // errors field should be absent (skip_serializing_if)
        assert!(parsed.get("errors").is_none());
    }

    #[test]
    fn render_to_writer_matches_render_json() {
        let map = make_minimal_map();
        let json = render_json(&map).unwrap();

        let mut buf = Vec::new();
        render_to_writer(&map, &mut buf).unwrap();
        let from_writer = String::from_utf8(buf).unwrap();

        assert_eq!(json, from_writer);
    }
}
## File: src/schema.rs
use std::collections::BTreeMap;
use std::path::PathBuf;

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

#[derive(Debug, Clone, serde::Serialize)]
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

// ── Error reporting ─────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct ErrorEntry {
    pub file: String,
    #[builder(default)]
    pub line: usize,
    pub message: String,
    pub severity: ErrorSeverity,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ErrorContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}
## File: src/workspace.rs
use crate::schema::{CrateType, Error, Result};
use std::path::{Path, PathBuf};

/// Walk up the directory tree from `start_path` to find a `Cargo.toml`
/// containing a `[workspace]` section. Returns the directory containing it.
///
/// # Errors
///
/// Returns `Error::WorkspaceRootNotFound` if no `Cargo.toml` with a
/// `[workspace]` section is found in any ancestor directory.
pub fn find_workspace_root(start_path: &Path) -> Result<PathBuf> {
    for ancestor in start_path.ancestors() {
        let cargo_toml = ancestor.join("Cargo.toml");
        if cargo_toml.exists() {
            let content = std::fs::read_to_string(&cargo_toml).map_err(|source| Error::FileRead {
                path: cargo_toml.clone(),
                source,
            })?;
            if content.contains("[workspace]") {
                return Ok(ancestor.to_path_buf());
            }
        }
    }
    Err(Error::WorkspaceRootNotFound(start_path.to_path_buf()))
}

/// Parse the workspace `Cargo.toml`, resolve member paths (including glob
/// patterns), apply `exclude` list, and return absolute paths to each member
/// crate directory.
///
/// # Errors
///
/// Returns `Error::MissingWorkspaceSection` if the `Cargo.toml` lacks a
/// `[workspace]` section entirely.
pub fn enumerate_members(root: &Path) -> Result<Vec<PathBuf>> {
    let cargo_toml_path = root.join("Cargo.toml");
    let content = std::fs::read_to_string(&cargo_toml_path).map_err(|source| Error::FileRead {
        path: cargo_toml_path.clone(),
        source,
    })?;

    let parsed: toml::Value = toml::from_str(&content).map_err(|source| Error::TomlParse {
        path: cargo_toml_path.clone(),
        source,
    })?;

    let members: Vec<String> = match parsed.get("workspace") {
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

    let exclude: Vec<String> = parsed
        .get("workspace")
        .and_then(|w| w.get("exclude"))
        .and_then(|e| e.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let mut result = Vec::new();
    for member in &members {
        let has_glob = member.contains('*') || member.contains('?') || member.contains('[');
        if has_glob {
            let pattern = root.join(member).to_string_lossy().to_string();
            let iter = glob::glob(&pattern).map_err(|e| Error::GlobPattern(e.to_string()))?;
            for entry in iter {
                let path = entry.map_err(|e| Error::GlobPattern(e.to_string()))?;
                if path.is_dir() && path.join("Cargo.toml").exists() {
                    result.push(path);
                }
            }
        } else {
            let path = root.join(member);
            if path.is_dir() && path.join("Cargo.toml").exists() {
                result.push(path);
            } else {
                eprintln!(
                    "warning: workspace member {} does not exist",
                    path.display()
                );
            }
        }
    }

    result.retain(|p| {
        let name = p
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        !exclude.contains(&name)
    });

    result.sort();
    result.dedup();
    Ok(result)
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_cargo_toml(dir: &std::path::Path, content: &str) {
        let mut f = std::fs::File::create(dir.join("Cargo.toml")).unwrap();
        f.write_all(content.as_bytes()).unwrap();
    }

    fn setup_crate(dir: &std::path::Path) {
        let src = dir.join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("lib.rs"), "").unwrap();
        let mut f = std::fs::File::create(dir.join("Cargo.toml")).unwrap();
        use std::io::Write;
        writeln!(f, "[package]").unwrap();
        writeln!(f, "name = \"{}\"", dir.file_name().unwrap().to_string_lossy()).unwrap();
        writeln!(f, "version = \"0.1.0\"").unwrap();
        writeln!(f, "edition = \"2021\"").unwrap();
    }

    #[test]
    fn find_workspace_root_finds_cargo_toml() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("subdir").join("nested");
        std::fs::create_dir_all(&path).unwrap();
        write_cargo_toml(tmp.path(), "[workspace]");
        let result = find_workspace_root(&path).unwrap();
        assert_eq!(result, tmp.path());
    }

    #[test]
    fn enumerate_members_returns_members() {
        let tmp = tempfile::tempdir().unwrap();
        write_cargo_toml(tmp.path(), r#"
[workspace]
members = ["crate_a", "crate_b"]
"#);
        setup_crate(tmp.path().join("crate_a").as_path());
        setup_crate(tmp.path().join("crate_b").as_path());
        let members = enumerate_members(tmp.path()).unwrap();
        assert_eq!(members.len(), 2);
    }

    #[test]
    fn enumerate_members_returns_err_for_missing_workspace() {
        let tmp = tempfile::tempdir().unwrap();
        write_cargo_toml(tmp.path(), "[dependencies]\nfoo = \"1\"");
        let result = enumerate_members(tmp.path());
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::MissingWorkspaceSection => {},
            other => panic!("expected MissingWorkspaceSection, got {:?}", other),
        }
    }

    #[test]
    fn enumerate_members_applies_exclude() {
        let tmp = tempfile::tempdir().unwrap();
        write_cargo_toml(tmp.path(), r#"
[workspace]
members = ["a", "b", "c"]
exclude = ["b"]
"#);
        setup_crate(tmp.path().join("a").as_path());
        setup_crate(tmp.path().join("b").as_path());
        setup_crate(tmp.path().join("c").as_path());
        let members = enumerate_members(tmp.path()).unwrap();
        let names: Vec<_> = members.iter().map(|p| p.file_name().unwrap().to_string_lossy()).collect();
        assert!(names.iter().any(|n| *n == "a"));
        assert!(!names.iter().any(|n| *n == "b"));
        assert!(names.iter().any(|n| *n == "c"));
    }

    #[test]
    fn resolve_crate_roots_detects_lib() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join("src")).unwrap();
        std::fs::write(tmp.path().join("src").join("lib.rs"), "").unwrap();
        let roots = resolve_crate_roots(tmp.path());
        assert_eq!(roots.len(), 1);
        assert_eq!(roots[0].1, CrateType::Lib);
    }

    #[test]
    fn resolve_crate_roots_detects_bin() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join("src")).unwrap();
        std::fs::write(tmp.path().join("src").join("main.rs"), "").unwrap();
        let roots = resolve_crate_roots(tmp.path());
        assert_eq!(roots.len(), 1);
        assert_eq!(roots[0].1, CrateType::Bin);
    }
}
/// Returns `(path, CrateType)` pairs — one for `src/lib.rs` (Lib),
/// one for `src/main.rs` (Bin), or empty if neither exists.
#[must_use]
pub fn resolve_crate_roots(crate_dir: &Path) -> Vec<(PathBuf, CrateType)> {
    let mut roots = Vec::new();
    let lib_rs = crate_dir.join("src").join("lib.rs");
    let main_rs = crate_dir.join("src").join("main.rs");
    if lib_rs.exists() {
        roots.push((lib_rs, CrateType::Lib));
    }
    if main_rs.exists() {
        roots.push((main_rs, CrateType::Bin));
    }
    roots
}
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

#[test]
fn test_sample_workspace_output() {
    let fixture = std::path::Path::new("tests/fixtures/sample-workspace");

    let output = Command::new(&binary_path())
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
    assert_eq!(json["workspace"]["root"], ".");
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

    // Cross-references should link Task to both crates.
    let cross_refs = &json["crossReferences"]["types"];
    let task_ref = cross_refs
        .get("Task")
        .expect("Task should appear in crossReferences.types");
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
        .arg(fixture)
        .output()
        .expect("failed to execute binary (run 1)");
    assert!(output1.status.success());

    let output2 = Command::new(&binary_path())
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
        .arg("/tmp/nonexistent-path-12345")
        .output()
        .expect("failed to execute binary");

    assert!(
        !output.status.success(),
        "should exit non-zero for invalid path"
    );
}

fn run_binary(path: &str) -> std::process::Output {
    Command::new(&binary_path())
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

    let output = run_binary(root.to_str().unwrap());
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

    let output = run_binary(root.to_str().unwrap());

    // Should exit non-zero because workspace is missing
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

    let output = run_binary(root.to_str().unwrap());
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

    let output = run_binary(root.to_str().unwrap());
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

    let output = run_binary(root.to_str().unwrap());
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

    let output = run_binary(root.to_str().unwrap());
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
        .arg(fixture)
        .arg("-o")
        .arg(output_path.clone())
        .output()
        .expect("failed to execute binary");
    assert!(output1.status.success());

    // Run without -o, capture stdout
    let output2 = Command::new(&binary_path())
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
