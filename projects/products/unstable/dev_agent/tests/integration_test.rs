// Integration tests for dev_agent.
//
// These tests exercise the agent pipeline end-to-end using the tiny fixture
// repo bundled at tests/fixtures/tiny_repo.

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("tiny_repo")
}

fn unique_temp_dir(suffix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let pid = std::process::id();
    let dir = std::env::temp_dir().join(format!("dev_agent_integration_{suffix}_{pid}_{nanos}"));
    fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn dev_agent_bin() -> PathBuf {
    // CARGO_BIN_EXE_dev_agent is set by Cargo when running integration tests.
    PathBuf::from(env!("CARGO_BIN_EXE_dev_agent"))
}

// ── scan ─────────────────────────────────────────────────────────────────────

#[test]
fn scan_produces_valid_json() {
    let fixture = fixture_path();
    let output = Command::new(dev_agent_bin())
        .args(["scan", fixture.to_str().unwrap()])
        .output()
        .expect("failed to run dev_agent scan");

    assert!(
        output.status.success(),
        "scan failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout).expect("scan output must be valid JSON");
    assert!(parsed.get("entries").is_some(), "must have 'entries' key");
}

#[test]
fn scan_is_deterministic() {
    let fixture = fixture_path();
    let run = |args: &[&str]| {
        Command::new(dev_agent_bin())
            .args(args)
            .output()
            .expect("failed to run dev_agent")
    };

    let o1 = run(&["scan", fixture.to_str().unwrap()]);
    let o2 = run(&["scan", fixture.to_str().unwrap()]);

    assert!(o1.status.success());
    assert!(o2.status.success());
    assert_eq!(
        o1.stdout, o2.stdout,
        "scan output must be identical across runs"
    );
}

// ── plan ─────────────────────────────────────────────────────────────────────

#[test]
fn plan_produces_valid_json() {
    let fixture = fixture_path();
    let output = Command::new(dev_agent_bin())
        .args(["plan", fixture.to_str().unwrap()])
        .output()
        .expect("failed to run dev_agent plan");

    assert!(
        output.status.success(),
        "plan failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout).expect("plan output must be valid JSON");
    assert!(parsed.get("tasks").is_some(), "must have 'tasks' key");
    assert!(parsed.get("edges").is_some(), "must have 'edges' key");
}

#[test]
fn plan_is_deterministic() {
    let fixture = fixture_path();
    let run = || {
        Command::new(dev_agent_bin())
            .args(["plan", fixture.to_str().unwrap()])
            .output()
            .expect("failed to run dev_agent plan")
    };

    let o1 = run();
    let o2 = run();
    assert!(o1.status.success());
    assert!(o2.status.success());
    assert_eq!(
        o1.stdout, o2.stdout,
        "plan output must be identical across runs"
    );
}

// ── apply ─────────────────────────────────────────────────────────────────────

#[test]
fn apply_writes_edits_and_produces_report() {
    let dir = unique_temp_dir("apply");

    // Pre-populate a file so the scanner has something to find
    fs::write(dir.join("hello.txt"), "original").unwrap();

    // Build the patch JSON
    let edits = serde_json::json!([
        {"path": "hello.txt", "new_content": "patched content"}
    ]);
    let patch_path = dir.join("patch.json");
    fs::write(&patch_path, edits.to_string()).unwrap();

    let output = Command::new(dev_agent_bin())
        .args(["apply", dir.to_str().unwrap(), patch_path.to_str().unwrap()])
        .output()
        .expect("failed to run dev_agent apply");

    assert!(
        output.status.success(),
        "apply failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // The target file must contain the patched content.
    let content = fs::read_to_string(dir.join("hello.txt")).unwrap();
    assert_eq!(content, "patched content");

    // The output must be a valid AgentReport JSON.
    let stdout = String::from_utf8_lossy(&output.stdout);
    let report: serde_json::Value =
        serde_json::from_str(&stdout).expect("apply output must be valid JSON");
    assert!(report.get("plan").is_some());
    assert!(report.get("applied_edits").is_some());
    assert!(report.get("verify_outcomes").is_some());
    assert!(report.get("event_log").is_some());

    fs::remove_dir_all(dir).unwrap();
}

// ── verify ────────────────────────────────────────────────────────────────────

#[test]
fn verify_produces_report_with_skipped_steps_by_default() {
    let fixture = fixture_path();
    let output = Command::new(dev_agent_bin())
        .args(["verify", fixture.to_str().unwrap(), "--fmt"])
        .output()
        .expect("failed to run dev_agent verify");

    assert!(
        output.status.success(),
        "verify failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let report: serde_json::Value =
        serde_json::from_str(&stdout).expect("verify output must be valid JSON");

    let outcomes = report["verify_outcomes"].as_array().expect("array");
    assert!(!outcomes.is_empty(), "must have at least one outcome");
    assert!(
        outcomes.iter().all(|o| o["skipped"] == true),
        "all steps must be skipped without --run"
    );
}

// ── plan → apply → verify flow ────────────────────────────────────────────────

#[test]
fn plan_apply_verify_full_flow() {
    let dir = unique_temp_dir("flow");
    fs::write(dir.join("lib.rs"), "pub fn hello() {}").unwrap();

    // 1. plan
    let plan_out = Command::new(dev_agent_bin())
        .args(["plan", dir.to_str().unwrap()])
        .output()
        .expect("plan");
    assert!(plan_out.status.success());

    // 2. apply (add a new file via patch)
    let edits = serde_json::json!([
        {"path": "lib.rs", "new_content": "pub fn hello() -> &'static str { \"hi\" }"}
    ]);
    let patch_path = dir.join("patch.json");
    fs::write(&patch_path, edits.to_string()).unwrap();

    let apply_out = Command::new(dev_agent_bin())
        .args(["apply", dir.to_str().unwrap(), patch_path.to_str().unwrap()])
        .output()
        .expect("apply");
    assert!(apply_out.status.success());

    // 3. verify (skipped, no --run)
    let verify_out = Command::new(dev_agent_bin())
        .args(["verify", dir.to_str().unwrap()])
        .output()
        .expect("verify");
    assert!(verify_out.status.success());

    // The report must contain expected keys.
    let report: serde_json::Value =
        serde_json::from_str(&String::from_utf8_lossy(&verify_out.stdout)).expect("verify JSON");
    assert!(report.get("plan").is_some());
    assert!(report.get("verify_outcomes").is_some());

    fs::remove_dir_all(dir).unwrap();
}
