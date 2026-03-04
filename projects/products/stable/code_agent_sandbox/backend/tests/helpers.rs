// projects/products/stable/code_agent_sandbox/tests/helpers.rs
//
// Shared test helpers for integration tests.

use common_json::{Json, from_json_str};
use std::io::Write;
use std::process::{Command, Output, Stdio};

/// Runs the code_agent_sandbox binary with given repo/runs roots and stdin input.
pub fn run_sandbox_with_stdin(
    repo_root: &str,
    runs_root: &str,
    input: &str,
) -> anyhow::Result<Output> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_code_agent_sandbox_backend"))
        .arg(repo_root)
        .arg(runs_root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    {
        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| anyhow::anyhow!("Failed to open stdin"))?;
        stdin.write_all(input.as_bytes())?;
    }

    Ok(child.wait_with_output()?)
}

/// Parses the stdout of a process output as JSON with rich diagnostics on failure.
pub fn parse_stdout_json(output: &Output, input: &str) -> anyhow::Result<Json> {
    let stdout = String::from_utf8_lossy(&output.stdout);
    match from_json_str(&stdout) {
        Ok(json) => Ok(json),
        Err(e) => anyhow::bail!(
            "stdout is not valid JSON: {e}\nstatus: {:?}\nstdout:\n{}\nstderr:\n{}\ninput:\n{}",
            output.status.code(),
            stdout,
            String::from_utf8_lossy(&output.stderr),
            input
        ),
    }
}

/// Validates that an output indicates success with full diagnostic details on failure.
pub fn assert_output_success(output: &Output, input: &str) -> anyhow::Result<()> {
    if !output.status.success() {
        anyhow::bail!(
            "Command failed\nstatus: {:?}\nstdout:\n{}\nstderr:\n{}\ninput:\n{}",
            output.status.code(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
            input
        );
    }
    Ok(())
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
