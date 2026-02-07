// projects/products/stable/code_agent_sandbox/tests/helpers.rs
//
// Shared test helpers for integration tests.

use common_json::{from_json_str, Json};
use std::io::Write;
use std::process::{Command, Output, Stdio};

/// Runs the code_agent_sandbox binary with given repo/runs roots and stdin input.
pub fn run_sandbox_with_stdin(repo_root: &str, runs_root: &str, input: &str) -> Output {
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

/// Parses the stdout of a process output as JSON, or panics with diagnostic info.
pub fn parse_stdout_json(output: &Output, input: &str) -> Json {
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

/// Asserts that an output indicates success, or panics with full diagnostic details.
pub fn assert_output_success(output: &Output, input: &str) {
    if !output.status.success() {
        panic!(
            "Command failed\nstatus: {:?}\nstdout:\n{}\nstderr:\n{}\ninput:\n{}",
            output.status.code(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
            input
        );
    }
}

/// Extracts a string field from a JSON object, or None if missing/wrong type.
pub fn get_json_string<'a>(obj: &'a Json, key: &str) -> Option<&'a str> {
    obj.as_object()?.get(key)?.as_str()
}

/// Extracts an array field from a JSON object, or None if missing/wrong type.
pub fn get_json_array<'a>(obj: &'a Json, key: &str) -> Option<&'a Vec<Json>> {
    obj.as_object()?.get(key)?.as_array()
}

/// Checks if any result in a "results" array has ok=false.
pub fn has_failed_result(json: &Json) -> bool {
    let Some(results) = get_json_array(json, "results") else {
        return false;
    };
    results.iter().any(|r| {
        r.as_object()
            .and_then(|obj| obj.get("ok"))
            .and_then(|x| x.as_bool())
            == Some(false)
    })
}
