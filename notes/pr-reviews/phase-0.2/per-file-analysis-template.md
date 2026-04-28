# Per-File Analysis Template

## Instructions

Read `raw-diff.md` for diff context, then fill in the judgment fields below.

**Do NOT modify the `### Facts` table.** These values come from file-manifest.json and are authoritative — they cannot be changed.

---

## File: Cargo.lock

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +286 |
| Lines removed | -1 |
| Trailing newline | YES |
| Added functions | — |
| Modified functions | — |
| Removed functions | — |
| Added imports | — |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: Cargo.toml

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +3 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | — |
| Modified functions | — |
| Removed functions | — |
| Added imports | — |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: execution_reports/execution_fix-plan_20260428.md

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +54 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | — |
| Modified functions | — |
| Removed functions | — |
| Added imports | — |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: execution_reports/execution_phase-0.2_20260428.md

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +288 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | — |
| Modified functions | — |
| Removed functions | — |
| Added imports | — |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: notes/plan-enrichment/phase-0.2/codebase-state.md

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +328 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | Error, Result, Config, ErrorEntry, find_workspace_root, enumerate_members, resolve_crate_roots, parse_cargo_toml, parse_file, extract_public_items, extract_imports, extract_re_exports, extract_submodules, extract_impls, resolve_module_path, build_module_tree, compute, PublicItem, kind_to_string, render_json, render_to_writer, Cli, main, run, relativize_path, test_sample_workspace_output, test_deterministic_output, test_missing_path_exits_nonzero |
| Modified functions | — |
| Removed functions | — |
| Added imports | — |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: notes/plan-enrichment/phase-0.2/deferred-and-patterns.md

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +28 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | — |
| Modified functions | — |
| Removed functions | — |
| Added imports | — |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: notes/plan-enrichment/phase-0.2/draft-elaboration.md

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +563 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | ErrorSeverity, ErrorContext, ErrorEntry, parse_file, ParsedFile, SynParseError, build_parse_error_entry, build_module_tree |
| Modified functions | — |
| Removed functions | — |
| Added imports | — |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: notes/plan-enrichment/phase-0.2/gather-summary.md

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +27 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | — |
| Modified functions | — |
| Removed functions | — |
| Added imports | — |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: notes/plan-enrichment/phase-0.2/plan.approved.toml

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +2168 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | FileInfo, ErrorSeverity, ErrorContext, Result, ErrorEntry, parse_file, ParsedFile, SynParseError, build_module_tree, resolve_module_path, find_workspace_root, enumerate_members, run, relativize_path, extract_public_items, extract_imports, extract_re_exports, extract_submodules, extract_impls, render_json, crate, kind_to_string, parse_source, parse_file_returns_ast_for_valid_source, parse_file_returns_error_for_invalid_source, parse_file_returns_empty_for_empty_file, extract_public_items_finds_struct_enum_trait_fn, extract_public_items_empty_for_no_public_items, extract_imports_finds_use_statements, extract_re_exports_finds_pub_use, extract_re_exports_finds_rename, extract_submodules_finds_mod_declarations, extract_submodules_marks_cfg_test, extract_impls_finds_fn_type_const, build_parse_error_entry_constructs_error, resolve_module_path_finds_rs_file, resolve_module_path_finds_mod_rs, resolve_module_path_returns_none_for_missing, build_module_tree_returns_empty_for_nonexistent, write_cargo_toml, setup_crate, find_workspace_root_finds_cargo_toml, enumerate_members_returns_members, enumerate_members_returns_err_for_missing_workspace, enumerate_members_applies_exclude, resolve_crate_roots_detects_lib, resolve_crate_roots_detects_bin, parse_cargo_toml_parses_minimal, parse_cargo_toml_uses_defaults_for_missing_package, parse_cargo_toml_distinguishes_deps, make_crate, compute_finds_cross_crate_import, compute_empty_for_no_cross_references, render_to_writer, make_minimal_map, render_json_produces_valid_json, render_json_skips_empty_errors, render_to_writer_matches_render_json, test_missing_path_exits_nonzero, run_binary, parse_output, test_parse_failure_error_entry, test_missing_workspace_section, test_glob_member_patterns, test_workspace_with_exclude, test_deeply_nested_modules, test_reexport_chains, Secret, test_output_via_flag |
| Modified functions | — |
| Removed functions | — |
| Added imports | std::path::Path, rayon::prelude::*, super::*, crate::schema::{Import, ReExport, SubmoduleDecl}, std::path::PathBuf, crate::schema::ErrorEntry, std::io::Write, crate::schema::{ModuleInfo, PublicItem, SubmoduleDecl} |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: notes/plan-enrichment/phase-0.2/task-checklist.md

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +274 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | — |
| Modified functions | — |
| Removed functions | — |
| Added imports | — |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: notes/pr-reviews/phase-0.2/context.md

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +128 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | — |
| Modified functions | — |
| Removed functions | — |
| Added imports | — |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: notes/pr-reviews/phase-0.2/cross-reference-validation.md

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +57 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | — |
| Modified functions | — |
| Removed functions | — |
| Added imports | — |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: notes/pr-reviews/phase-0.2/cross-reference.md

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +59 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | — |
| Modified functions | — |
| Removed functions | — |
| Added imports | — |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: notes/pr-reviews/phase-0.2/deferred.md

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +8 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | — |
| Modified functions | — |
| Removed functions | — |
| Added imports | — |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: notes/pr-reviews/phase-0.2/draft-fix-document.md

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +9 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | — |
| Modified functions | — |
| Removed functions | — |
| Added imports | — |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: notes/pr-reviews/phase-0.2/draft-fix-plan.toml

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +27 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | — |
| Modified functions | — |
| Removed functions | — |
| Added imports | — |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: notes/pr-reviews/phase-0.2/draft-review.md

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +15 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | — |
| Modified functions | — |
| Removed functions | — |
| Added imports | — |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: notes/pr-reviews/phase-0.2/file-manifest.json

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +791 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | — |
| Modified functions | — |
| Removed functions | — |
| Added imports | — |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: notes/pr-reviews/phase-0.2/fix-plan.toml

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +27 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | — |
| Modified functions | — |
| Removed functions | — |
| Added imports | — |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: notes/pr-reviews/phase-0.2/gather-summary.md

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +34 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | — |
| Modified functions | — |
| Removed functions | — |
| Added imports | — |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: notes/pr-reviews/phase-0.2/per-file-analysis-template.md

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +1450 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | — |
| Modified functions | — |
| Removed functions | — |
| Added imports | — |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: notes/pr-reviews/phase-0.2/per-file-analysis.md

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +1448 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | — |
| Modified functions | — |
| Removed functions | — |
| Added imports | — |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: notes/pr-reviews/phase-0.2/raw-diff.md

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +19027 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | parse_cargo_toml, crate, kind_to_string, extract_public_items, extract_imports, extract_re_exports, extract_submodules, extract_impls, resolve_module_path, process_module_items, process_module_info, render_json, render_to_writer, Result, FileInfo, find_workspace_root, enumerate_members, resolve_crate_roots, Error, Config, ErrorEntry, parse_file, build_module_tree, compute, PublicItem, Cli, main, run, relativize_path, test_sample_workspace_output, test_deterministic_output, test_missing_path_exits_nonzero, ErrorSeverity, ErrorContext, ParsedFile, SynParseError, build_parse_error_entry, parse_source, parse_file_returns_ast_for_valid_source, parse_file_returns_error_for_invalid_source, parse_file_returns_empty_for_empty_file, extract_public_items_finds_struct_enum_trait_fn, extract_public_items_empty_for_no_public_items, extract_imports_finds_use_statements, extract_re_exports_finds_pub_use, extract_re_exports_finds_rename, extract_submodules_finds_mod_declarations, extract_submodules_marks_cfg_test, extract_impls_finds_fn_type_const, build_parse_error_entry_constructs_error, resolve_module_path_finds_rs_file, resolve_module_path_finds_mod_rs, resolve_module_path_returns_none_for_missing, build_module_tree_returns_empty_for_nonexistent, write_cargo_toml, setup_crate, find_workspace_root_finds_cargo_toml, enumerate_members_returns_members, enumerate_members_returns_err_for_missing_workspace, enumerate_members_applies_exclude, resolve_crate_roots_detects_lib, resolve_crate_roots_detects_bin, parse_cargo_toml_parses_minimal, parse_cargo_toml_uses_defaults_for_missing_package, parse_cargo_toml_distinguishes_deps, make_crate, compute_finds_cross_crate_import, compute_empty_for_no_cross_references, make_minimal_map, render_json_produces_valid_json, render_json_skips_empty_errors, render_to_writer_matches_render_json, run_binary, parse_output, test_parse_failure_error_entry, test_missing_workspace_section, test_glob_member_patterns, test_workspace_with_exclude, test_deeply_nested_modules, test_reexport_chains, Secret, test_output_via_flag, is_visibility_inherited, item_vis, line_of_item, item_ident_span, vis_to_string, generics_to_string, type_to_string, quote_bound, fields_to_strings, variants_to_strings, extract_attrs, into_public_item, flatten_use_tree, extract_re_exports_from_tree, COUNTER, build_module_info, process_submodule, CrateType, WorkspaceMap, WorkspaceInfo, CrateInfo, PackageInfo, DepInfo, ModuleInfo, ItemKind, ItemAttrs, ImplInfo, ImplItem, ImplItemKind, Import, ReExport, CrossCrateImport, CrossReferences, TypeRef, SubmoduleDecl, binary_path, extract_array |
| Modified functions | — |
| Removed functions | — |
| Added imports | std::path::Path, anyhow::Context, rayon::prelude::*, crate::file_parser, std::collections::HashSet, std::path::{Path, PathBuf}, std::io::Write, super::*, crate::schema::{Import, ReExport, SubmoduleDecl}, std::path::PathBuf, crate::schema::ErrorEntry, crate::schema::{ModuleInfo, PublicItem, SubmoduleDecl}, crate::schema::{CrateType, DepInfo, Error, PackageInfo, Result}, crate::schema::{CrateInfo, CrossCrateImport, CrossReferences, TypeRef}, std::collections::BTreeMap, crate::schema::{CrateType, Import, ItemKind, ModuleInfo, PackageInfo, PublicItem}, std::sync::atomic::{AtomicU64, Ordering}, clap::Parser, crate::schema::{ErrorContext, ErrorEntry, ErrorSeverity, FileInfo, ModuleInfo, SubmoduleDecl}, crate::schema::WorkspaceMap, crate::schema::{CrateType, Error, Result}, std::process::Command |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: notes/pr-reviews/phase-0.2/review.md

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +35 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | — |
| Modified functions | — |
| Removed functions | — |
| Added imports | — |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: notes/pr-reviews/phase-0.2/status.md

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +53 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | — |
| Modified functions | — |
| Removed functions | — |
| Added imports | — |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: plans/phase-0.2.toml

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +2168 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | FileInfo, ErrorSeverity, ErrorContext, Result, ErrorEntry, parse_file, ParsedFile, SynParseError, build_module_tree, resolve_module_path, find_workspace_root, enumerate_members, run, relativize_path, extract_public_items, extract_imports, extract_re_exports, extract_submodules, extract_impls, render_json, crate, kind_to_string, parse_source, parse_file_returns_ast_for_valid_source, parse_file_returns_error_for_invalid_source, parse_file_returns_empty_for_empty_file, extract_public_items_finds_struct_enum_trait_fn, extract_public_items_empty_for_no_public_items, extract_imports_finds_use_statements, extract_re_exports_finds_pub_use, extract_re_exports_finds_rename, extract_submodules_finds_mod_declarations, extract_submodules_marks_cfg_test, extract_impls_finds_fn_type_const, build_parse_error_entry_constructs_error, resolve_module_path_finds_rs_file, resolve_module_path_finds_mod_rs, resolve_module_path_returns_none_for_missing, build_module_tree_returns_empty_for_nonexistent, write_cargo_toml, setup_crate, find_workspace_root_finds_cargo_toml, enumerate_members_returns_members, enumerate_members_returns_err_for_missing_workspace, enumerate_members_applies_exclude, resolve_crate_roots_detects_lib, resolve_crate_roots_detects_bin, parse_cargo_toml_parses_minimal, parse_cargo_toml_uses_defaults_for_missing_package, parse_cargo_toml_distinguishes_deps, make_crate, compute_finds_cross_crate_import, compute_empty_for_no_cross_references, render_to_writer, make_minimal_map, render_json_produces_valid_json, render_json_skips_empty_errors, render_to_writer_matches_render_json, test_missing_path_exits_nonzero, run_binary, parse_output, test_parse_failure_error_entry, test_missing_workspace_section, test_glob_member_patterns, test_workspace_with_exclude, test_deeply_nested_modules, test_reexport_chains, Secret, test_output_via_flag |
| Modified functions | — |
| Removed functions | — |
| Added imports | std::path::Path, rayon::prelude::*, super::*, crate::schema::{Import, ReExport, SubmoduleDecl}, std::path::PathBuf, crate::schema::ErrorEntry, std::io::Write, crate::schema::{ModuleInfo, PublicItem, SubmoduleDecl} |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: src/cargo_info.rs

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +65 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | write_cargo_toml, parse_cargo_toml_parses_minimal, parse_cargo_toml_uses_defaults_for_missing_package, parse_cargo_toml_distinguishes_deps |
| Modified functions | — |
| Removed functions | — |
| Added imports | super::*, std::io::Write |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: src/cross_refs.rs

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +82 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | make_crate, compute_finds_cross_crate_import, compute_empty_for_no_cross_references |
| Modified functions | — |
| Removed functions | — |
| Added imports | super::*, crate::schema::{CrateType, Import, ItemKind, ModuleInfo, PackageInfo, PublicItem} |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: src/file_parser.rs

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +255 |
| Lines removed | -49 |
| Trailing newline | YES |
| Added functions | ParsedFile, SynParseError, parse_file, flatten_use_tree, parse_source, COUNTER, parse_file_returns_ast_for_valid_source, parse_file_returns_error_for_invalid_source, parse_file_returns_empty_for_empty_file, extract_public_items_finds_struct_enum_trait_fn, extract_public_items_empty_for_no_public_items, extract_imports_finds_use_statements, extract_re_exports_finds_pub_use, extract_re_exports_finds_rename, extract_submodules_finds_mod_declarations, extract_submodules_marks_cfg_test, extract_impls_finds_fn_type_const, build_parse_error_entry_constructs_error |
| Modified functions | — |
| Removed functions | parse_file, flatten_use_tree |
| Added imports | super::*, std::path::PathBuf, std::sync::atomic::{AtomicU64, Ordering} |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: src/lib.rs

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +65 |
| Lines removed | -41 |
| Trailing newline | YES |
| Added functions | run |
| Modified functions | — |
| Removed functions | run |
| Added imports | — |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: src/main.rs

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +1 |
| Lines removed | -1 |
| Trailing newline | YES |
| Added functions | — |
| Modified functions | — |
| Removed functions | — |
| Added imports | — |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: src/module_tree.rs

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +146 |
| Lines removed | -59 |
| Trailing newline | YES |
| Added functions | build_module_tree, resolve_module_path_finds_rs_file, resolve_module_path_finds_mod_rs, resolve_module_path_returns_none_for_missing, build_module_tree_returns_empty_for_nonexistent |
| Modified functions | — |
| Removed functions | build_module_tree |
| Added imports | crate::schema::{ErrorContext, ErrorEntry, ErrorSeverity, FileInfo, ModuleInfo, SubmoduleDecl}, super::* |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: src/render.rs

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +78 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | make_minimal_map, render_json_produces_valid_json, render_json_skips_empty_errors, render_to_writer_matches_render_json |
| Modified functions | — |
| Removed functions | — |
| Added imports | super::* |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: src/schema.rs

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +41 |
| Lines removed | -1 |
| Trailing newline | YES |
| Added functions | ErrorSeverity, ErrorContext |
| Modified functions | — |
| Removed functions | — |
| Added imports | — |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: src/workspace.rs

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +120 |
| Lines removed | -11 |
| Trailing newline | YES |
| Added functions | write_cargo_toml, setup_crate, find_workspace_root_finds_cargo_toml, enumerate_members_returns_members, enumerate_members_returns_err_for_missing_workspace, enumerate_members_applies_exclude, resolve_crate_roots_detects_lib, resolve_crate_roots_detects_bin |
| Modified functions | — |
| Removed functions | — |
| Added imports | super::*, std::io::Write |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: tests/fixtures/sample-workspace/Cargo.lock

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +83 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | — |
| Modified functions | — |
| Removed functions | — |
| Added imports | — |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

## File: tests/integration_test.rs

### Facts (from file-manifest.json — authoritative, do not modify)

| Property | Value |
|----------|-------|
| Lines added | +265 |
| Lines removed | -0 |
| Trailing newline | YES |
| Added functions | run_binary, parse_output, write_cargo_toml, setup_crate, test_parse_failure_error_entry, test_missing_workspace_section, test_glob_member_patterns, test_workspace_with_exclude, test_deeply_nested_modules, test_reexport_chains, Secret, test_output_via_flag |
| Modified functions | — |
| Removed functions | — |
| Added imports | std::io::Write |

### Intent

[Fill in: one sentence on what changed, based on the diff]

### Checklist

- Unnecessary clone/unwrap/expect? [Fill in: Yes (cite location) / No]
- Error handling: [Fill in: observation]
- Dead code or unused imports? [Fill in: Yes / No]
- New public API: tests present? [Fill in: Yes / No / Not applicable]
- Change appears within plan scope? [Fill in: Yes / No / Unclear — no plan available yet]

### Notes

[Fill in: other observations — no classifications, just facts]

---

