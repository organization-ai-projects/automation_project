// projects/libraries/common_json/src/tests/process.rs
use crate::process::parse_json_stdout;

#[cfg(test)]
#[test]
fn test_parse_json_stdout() {
    let output = std::process::Output {
        status: std::process::ExitStatus::default(),
        stdout: b"{\"key\":\"value\"}".to_vec(),
        stderr: Vec::new(),
    };
    let parsed = parse_json_stdout(&output, "");
    match parsed {
        Ok(json) => assert!(json.is_object()),
        Err(err) => panic!("Error parsing JSON from stdout: {:?}", err),
    }
}
