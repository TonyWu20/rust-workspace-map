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

// ── Existing tests (rewritten for index subcommand) ─────────────────────

#[test]
fn test_sample_workspace_output() {
    let fixture = std::path::Path::new("tests/fixtures/sample-workspace");

    let output = Command::new(&binary_path())
        .arg("index")
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
    assert!(!json["workspace"]["root"].as_str().unwrap().is_empty());
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

    // Cross-references: keys are now canonical paths (module.path::item.name).
    let cross_refs = &json["crossReferences"]["types"];
    // Task is exported from the "core" module, so key is "core::Task".
    let task_ref = cross_refs
        .get("core::Task")
        .expect("core::Task should appear in crossReferences.types (canonical path)");
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
        .arg("index")
        .arg(fixture)
        .output()
        .expect("failed to execute binary (run 1)");
    assert!(output1.status.success());

    let output2 = Command::new(&binary_path())
        .arg("index")
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
        .arg("index")
        .arg("/tmp/nonexistent-path-12345")
        .output()
        .expect("failed to execute binary");

    assert!(
        !output.status.success(),
        "should exit non-zero for invalid path"
    );
}

fn run_index(path: &str) -> std::process::Output {
    Command::new(&binary_path())
        .arg("index")
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

    let output = run_index(root.to_str().unwrap());
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

    let output = Command::new(&binary_path())
        .arg("index")
        .arg(root.to_str().unwrap())
        .output();

    // Should exit non-zero because workspace is missing
    let output = output.expect("failed to execute binary");
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

    let output = run_index(root.to_str().unwrap());
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

    let output = run_index(root.to_str().unwrap());
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

    let output = run_index(root.to_str().unwrap());
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

    let output = run_index(root.to_str().unwrap());
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
        .arg("index")
        .arg(fixture)
        .arg("-o")
        .arg(output_path.clone())
        .output()
        .expect("failed to execute binary");
    assert!(output1.status.success());

    // Run without -o, capture stdout
    let output2 = Command::new(&binary_path())
        .arg("index")
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

// ── New validation tests ────────────────────────────────────────────────

#[test]
fn test_validate_orphan_file_exits_2() {
    let fixture = std::path::Path::new("tests/fixtures/bad-orphan");

    let output = Command::new(&binary_path())
        .arg("index")
        .arg("--validate")
        .arg(fixture)
        .output()
        .expect("failed to execute binary");

    assert_eq!(
        output.status.code().unwrap(),
        2,
        "validation should exit 2 for orphan files"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("output is not valid JSON");

    let errors = extract_array(&json, "errors");
    assert!(!errors.is_empty(), "should have validation errors");

    let orphan_error = errors.iter().find(|e| e["kind"].as_str().unwrap() == "orphan_file")
        .expect("should have orphan_file error");

    assert_eq!(orphan_error["severity"].as_str().unwrap(), "warning");
    let message = orphan_error["message"].as_str().unwrap();
    assert!(message.contains("forgotten"), "message should mention the orphan file");
    assert!(message.contains("pub mod"), "message should suggest fix hint");
}

#[test]
fn test_validate_dead_reexport_exits_2() {
    let fixture = std::path::Path::new("tests/fixtures/bad-dead-reexport");

    let output = Command::new(&binary_path())
        .arg("index")
        .arg("--validate")
        .arg(fixture)
        .output()
        .expect("failed to execute binary");

    assert_eq!(
        output.status.code().unwrap(),
        2,
        "validation should exit 2 for dead re-exports"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("output is not valid JSON");

    let errors = extract_array(&json, "errors");
    let dead_reexport = errors.iter().find(|e| e["kind"].as_str().unwrap() == "dead_re_export")
        .expect("should have dead_re_export error");

    assert_eq!(dead_reexport["severity"].as_str().unwrap(), "warning");
}

#[test]
fn test_recursive_orphan_detection() {
    let fixture = std::path::Path::new("tests/fixtures/bad-orphan");

    let output = Command::new(&binary_path())
        .arg("index")
        .arg("--validate")
        .arg(fixture)
        .output()
        .expect("failed to execute binary");

    assert_eq!(
        output.status.code().unwrap(),
        2,
        "validation should exit 2 for orphan files"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("output is not valid JSON");

    let errors = extract_array(&json, "errors");

    let orphan_errors: Vec<_> = errors
        .iter()
        .filter(|e| e["kind"].as_str().unwrap() == "orphan_file")
        .collect();

    assert!(
        !orphan_errors.is_empty(),
        "should have orphan_file errors"
    );

    // The depth-2 orphan at sub/deep/orphan.rs must be detected
    let has_deep_orphan = orphan_errors.iter().any(|e| {
        e["file"].as_str().unwrap().contains("deep/orphan.rs")
    });
    assert!(
        has_deep_orphan,
        "should detect the deeply nested orphan at sub/deep/orphan.rs"
    );

    // The shallow orphan forgotten.rs must still be detected
    let has_forgotten = orphan_errors.iter().any(|e| {
        e["file"].as_str().unwrap().contains("forgotten.rs")
    });
    assert!(
        has_forgotten,
        "should still detect the shallow orphan forgotten.rs"
    );
}

#[test]
fn test_index_no_validate_exits_0() {
    let fixture = std::path::Path::new("tests/fixtures/bad-orphan");

    let output = Command::new(&binary_path())
        .arg("index")
        .arg(fixture)
        .output()
        .expect("failed to execute binary");

    assert_eq!(
        output.status.code().unwrap(),
        0,
        "without --validate, should exit 0"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("output is not valid JSON");

    let errors = extract_array(&json, "errors");
    let has_orphan = errors.iter().any(|e| {
        e["kind"].as_str().unwrap() == "orphan_file"
    });
    assert!(!has_orphan, "without --validate, should not have OrphanFile/DeadReExport errors");
}

// ── New lookup tests ────────────────────────────────────────────────────

#[test]
fn test_lookup_symbol_found() {
    let fixture = std::path::Path::new("tests/fixtures/sample-workspace");

    let output = Command::new(&binary_path())
        .arg("lookup")
        .arg(fixture)
        .arg("--symbol")
        .arg("Task")
        .output()
        .expect("failed to execute binary");

    assert!(output.status.success(), "should find Task symbol");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("output is not valid JSON");

    assert_eq!(json["status"].as_str().unwrap(), "found");
    assert!(json["crateName"].as_str().unwrap().is_empty() == false);
    assert_eq!(json["kind"].as_str().unwrap(), "struct");
}

#[test]
fn test_lookup_symbol_not_found() {
    let fixture = std::path::Path::new("tests/fixtures/sample-workspace");

    let output = Command::new(&binary_path())
        .arg("lookup")
        .arg(fixture)
        .arg("--symbol")
        .arg("DoesNotExist")
        .output()
        .expect("failed to execute binary");

    assert_eq!(
        output.status.code().unwrap(),
        1,
        "should exit 1 for not found symbol"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("output is not valid JSON");

    assert_eq!(json["status"].as_str().unwrap(), "not_found");
}

#[test]
fn test_lookup_file() {
    let fixture = std::path::Path::new("tests/fixtures/sample-workspace");

    let output = Command::new(&binary_path())
        .arg("lookup")
        .arg(fixture)
        .arg("--file")
        .arg("core/src/lib.rs")
        .output()
        .expect("failed to execute binary");

    assert!(output.status.success(), "should find core/src/lib.rs");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("output is not valid JSON");

    assert!(json.get("fileEntry").is_some(), "should have fileEntry");
    assert!(json.get("primaryModule").is_some(), "should have primaryModule");
}

// ── Serde regression test ───────────────────────────────────────────────

#[test]
fn test_orphaned_module_serde_regression() {
    // Verify DiagnosticKind::OrphanedModule serializes as "orphaned_module".
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    write_cargo_toml(root, r#"
[workspace]
members = ["."]

[package]
name = "orphan-mod-test"
version = "0.1.0"
edition = "2021"
"#);

    let src = root.join("src");
    std::fs::create_dir_all(&src).unwrap();

    // lib.rs with an undeclared module
    std::fs::write(src.join("lib.rs"), "mod nonexistent;").unwrap();

    let output = run_index(root.to_str().unwrap());
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("output is not valid JSON");

    let errors = extract_array(&json, "errors");
    let orphaned = errors.iter().find(|e| e["kind"].as_str().unwrap() == "orphaned_module")
        .expect("should have orphaned_module error (snake_case string)");

    assert_eq!(orphaned["severity"].as_str().unwrap(), "warning");
}
