use protocol::json::{Json, from_json_str, json};
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use tempfile::tempdir;

fn run_with_stdin(repo_root: &str, runs_root: &str, input: &str) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_code_agent_sandbox"))
        .arg(repo_root)
        .arg(runs_root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn code_agent_sandbox binary");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(input.as_bytes())
            .expect("Failed to write request JSON to stdin");
    }

    child.wait_with_output().expect("Failed to wait output")
}

fn parse_stdout_json(output: &std::process::Output, input: &str) -> Json {
    let stdout = String::from_utf8_lossy(&output.stdout);
    from_json_str(&stdout).unwrap_or_else(|e| {
        panic!(
            "stdout is not valid JSON: {e}\nstatus: {:?}\nstdout:\n{}\nstderr:\n{}\ninput:\n{}",
            output.status.code(),
            stdout,
            String::from_utf8_lossy(&output.stderr),
            input
        )
    })
}

#[test]
fn test_integration_pipeline() {
    // 1) Temp dirs
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let repo_root = temp_dir.path().join("repo_root");
    let runs_root = temp_dir.path().join("runs_root");
    fs::create_dir_all(&repo_root).expect("Failed to create repo_root");
    fs::create_dir_all(&runs_root).expect("Failed to create runs_root");

    let repo_root_s = repo_root.to_string_lossy().to_string();
    let runs_root_s = runs_root.to_string_lossy().to_string();

    // 2) Allowed request (matches your serde + PATH_RIGHTS)
    // IMPORTANT:
    // - Request fields are camelCase: runId, workspaceMode
    // - Action kind is camelCase: writeFile
    // - createDirs is camelCase
    // - Path is relative (policy normalizes / strips leading slashes)
    let input_allowed = json!({
        "runId": "test_run_123",
        "workspaceMode": "assist",
        "actions": [
            { "kind": "writeFile", "path": "src/agent_tmp.txt", "contents": "data", "createDirs": true }
        ]
    })
    .to_string();

    let out_allowed = run_with_stdin(&repo_root_s, &runs_root_s, &input_allowed);

    if !out_allowed.status.success() {
        panic!(
            "allowed request failed\nstatus: {:?}\nstdout:\n{}\nstderr:\n{}\ninput:\n{}",
            out_allowed.status.code(),
            String::from_utf8_lossy(&out_allowed.stdout),
            String::from_utf8_lossy(&out_allowed.stderr),
            input_allowed
        );
    }

    let v = parse_stdout_json(&out_allowed, &input_allowed);
    assert_eq!(v["runId"], "test_run_123");
    assert!(v["results"].is_array(), "results must be an array: {v}");

    // 3) Forbidden request (.git/** is FORBIDDEN)
    let input_forbidden = json!({
        "runId": "test_run_forbidden",
        "workspaceMode": "assist",
        "actions": [
            { "kind": "writeFile", "path": ".git/config", "contents": "nope", "createDirs": false }
        ]
    })
    .to_string();

    let out_forbidden = run_with_stdin(&repo_root_s, &runs_root_s, &input_forbidden);

    // Depending on your engine design, it may exit non-zero OR return ok:false results.
    // We'll parse JSON and assert at least one ok=false.
    let v2 = parse_stdout_json(&out_forbidden, &input_forbidden);

    assert!(v2["results"].is_array(), "results must be an array: {v2}");

    let any_failed = v2["results"]
        .as_array()
        .unwrap()
        .iter()
        .any(|r| r.get("ok").and_then(|x| x.as_bool()) == Some(false));

    assert!(
        any_failed,
        "expected a failed action result for forbidden path (.git/**). got: {v2}"
    );

    // 4) Invalid CLI args (0 args)
    let out_invalid = Command::new(env!("CARGO_BIN_EXE_code_agent_sandbox"))
        .output()
        .expect("Failed to run binary with no args");

    assert!(
        !out_invalid.status.success(),
        "expected non-zero exit for invalid args"
    );

    let stderr = String::from_utf8_lossy(&out_invalid.stderr);
    assert!(
        stderr.contains("Usage") || stderr.contains("invalid arguments"),
        "unexpected stderr for invalid args:\n{}",
        stderr
    );
}
