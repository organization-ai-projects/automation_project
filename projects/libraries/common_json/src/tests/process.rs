// projects/libraries/common_json/src/tests/process.rs
use crate::process::parse_json_stdout;
use std::process::Command;

#[cfg(test)]
#[test]
fn test_parse_json_stdout() {
    // Use an actual command to obtain a valid ExitStatus
    let output = Command::new("echo")
        .arg("{\"key\":\"value\"}")
        .output()
        .expect("Failed to execute echo command");
    
    let parsed = parse_json_stdout(&output, "");
    match parsed {
        Ok(json) => assert!(json.is_object()),
        Err(err) => panic!("Error parsing JSON from stdout: {:?}", err),
    }
}
