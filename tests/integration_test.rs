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
