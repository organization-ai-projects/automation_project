// projects/libraries/common_json/src/tests/process.rs
use crate::process::parse_json_stdout;
use std::process::Command;

#[cfg(test)]
#[test]
fn test_parse_json_stdout() {
    // Use an actual command to obtain a valid ExitStatus, then override stdout/stderr
    // Using cargo --version is more portable than echo (which behaves differently on Windows)
    let mut temp_output = Command::new("cargo")
        .arg("--version")
        .output()
        .expect("Failed to execute cargo command");
    temp_output.stdout = b"{\"key\":\"value\"}".to_vec();
    temp_output.stderr = Vec::new();
    let output = temp_output;

    let parsed = parse_json_stdout(&output, "");
    match parsed {
        Ok(json) => assert!(json.is_object()),
        Err(err) => panic!("Error parsing JSON from stdout: {:?}", err),
    }
}
