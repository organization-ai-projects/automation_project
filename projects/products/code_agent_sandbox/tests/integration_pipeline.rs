use std::fs;
use std::io::Write;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_integration_pipeline() {
    // Step 1: Create temporary directories for the test
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let repo_root = temp_dir.path().join("repo_root");
    let runs_root = temp_dir.path().join("runs_root");
    fs::create_dir_all(&repo_root).expect("Failed to create repo_root");
    fs::create_dir_all(&runs_root).expect("Failed to create runs_root");

    // Step 2: Create a valid JSON input
    let json_input = r#"{
        "action": "allowed_action",
        "args": "valid_arg"
    }"#;

    // Step 3: Test an allowed action with JSON input
    let allowed_action = Command::new(env!("CARGO_BIN_EXE_code_agent_sandbox"))
        .arg(repo_root.to_str().unwrap())
        .arg(runs_root.to_str().unwrap())
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn binary for allowed action");

    {
        let mut stdin = allowed_action.stdin.as_ref().expect("Failed to open stdin");
        stdin
            .write_all(json_input.as_bytes())
            .expect("Failed to write JSON input");
    }

    let output = allowed_action
        .wait_with_output()
        .expect("Failed to read output");

    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success(), "Allowed action failed");

    // Step 4: Validate JSON output from stdout
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("runId"),
        "Missing runId in JSON output: {}",
        stdout
    );
    assert!(
        stdout.contains("workspaceMode"),
        "Missing workspaceMode in JSON output: {}",
        stdout
    );

    // Step 5: Test a disallowed action
    let disallowed_action = Command::new(env!("CARGO_BIN_EXE_code_agent_sandbox"))
        .arg(repo_root.to_str().unwrap())
        .arg(runs_root.to_str().unwrap())
        .arg("--action")
        .arg("disallowed_action")
        .arg("--args")
        .arg("valid_arg")
        .output()
        .expect("Failed to execute binary for disallowed action");

    assert!(
        !disallowed_action.status.success(),
        "Disallowed action should have failed"
    );
    let disallowed_output = String::from_utf8_lossy(&disallowed_action.stderr);
    assert!(
        disallowed_output.contains("PolicyViolation"),
        "Unexpected output for disallowed action: {}",
        disallowed_output
    );

    // Step 6: Test invalid arguments
    let invalid_args = Command::new(env!("CARGO_BIN_EXE_code_agent_sandbox"))
        .arg(repo_root.to_str().unwrap())
        .arg(runs_root.to_str().unwrap())
        .arg("--action")
        .arg("allowed_action")
        .arg("--args")
        .arg("invalid_arg")
        .output()
        .expect("Failed to execute binary for invalid arguments");

    assert!(
        !invalid_args.status.success(),
        "Invalid arguments should have failed"
    );
    let invalid_output = String::from_utf8_lossy(&invalid_args.stderr);
    assert!(
        invalid_output.contains("PolicyViolation"),
        "Unexpected output for invalid arguments: {}",
        invalid_output
    );
}
