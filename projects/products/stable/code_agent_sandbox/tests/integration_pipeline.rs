// projects/products/stable/code_agent_sandbox/tests/integration_pipeline.rs
use common_json::{pjson, to_string};
use std::{fs, process::Command};
use tempfile::tempdir;

mod helpers;
use helpers::*;

/// Helper to set up temp directories for a test.
fn setup_temp_dirs() -> (tempfile::TempDir, String, String) {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let repo_root = temp_dir.path().join("repo_root");
    let runs_root = temp_dir.path().join("runs_root");
    fs::create_dir_all(&repo_root).expect("Failed to create repo_root");
    fs::create_dir_all(&runs_root).expect("Failed to create runs_root");

    let repo_root_s = repo_root.to_string_lossy().to_string();
    let runs_root_s = runs_root.to_string_lossy().to_string();

    (temp_dir, repo_root_s, runs_root_s)
}

#[test]
fn test_allowed_write_request() {
    let (_temp_dir, repo_root_s, runs_root_s) = setup_temp_dirs();

    // Request to write a file in an allowed path
    let input = to_string(&pjson!({
        "runId": "test_run_123",
        "workspaceMode": "assist",
        "actions": [
            { "kind": "writeFile", "path": "src/agent_tmp.txt", "contents": "data", "createDirs": true }
        ]
    }))
    .expect("Failed to serialize allowed request to JSON");

    let output = run_sandbox_with_stdin(&repo_root_s, &runs_root_s, &input);
    assert_output_success(&output, &input);

    let json = parse_stdout_json(&output, &input);
    assert_eq!(get_json_string(&json, "runId"), Some("test_run_123"));

    let results = get_json_array(&json, "results");
    assert!(results.is_some(), "results must be an array: {:?}", json);
}

#[test]
fn test_forbidden_write_request() {
    let (_temp_dir, repo_root_s, runs_root_s) = setup_temp_dirs();

    // Request to write to .git/** which is forbidden
    let input = to_string(&pjson!({
        "runId": "test_run_forbidden",
        "workspaceMode": "assist",
        "actions": [
            { "kind": "writeFile", "path": ".git/config", "contents": "nope", "createDirs": false }
        ]
    }))
    .expect("Failed to serialize forbidden request to JSON");

    let output = run_sandbox_with_stdin(&repo_root_s, &runs_root_s, &input);
    let json = parse_stdout_json(&output, &input);

    let results = get_json_array(&json, "results");
    assert!(results.is_some(), "results must be an array: {:?}", json);

    assert!(
        has_failed_result(&json),
        "expected a failed action result for forbidden path (.git/**). got: {:?}",
        json
    );
}

#[test]
fn test_invalid_cli_args() {
    // Run with 0 args (invalid)
    let output = Command::new(env!("CARGO_BIN_EXE_code_agent_sandbox"))
        .output()
        .expect("Failed to run binary with no args");

    assert!(
        !output.status.success(),
        "expected non-zero exit for invalid args"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Usage") || stderr.contains("invalid arguments"),
        "unexpected stderr for invalid args:\n{}",
        stderr
    );
}
