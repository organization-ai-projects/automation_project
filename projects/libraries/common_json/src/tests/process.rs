// projects/libraries/common_json/src/tests/process.rs
use crate::process::parse_json_stdout;
use super::test_helpers::{TestResult, assert_json_object};
use std::process::Command;

#[test]
fn test_parse_json_stdout() -> TestResult {
    // Use an actual command to obtain a valid ExitStatus, then override stdout/stderr
    // Using cargo --version is more portable than echo (which behaves differently on Windows)
    let mut temp_output = Command::new("cargo")
        .arg("--version")
        .output()
        .expect("Failed to execute cargo command");
    temp_output.stdout = b"{\"key\":\"value\"}".to_vec();
    temp_output.stderr = Vec::new();
    let output = temp_output;

    let parsed = parse_json_stdout(&output, "")?;
    assert_json_object(&parsed);
    Ok(())
}
