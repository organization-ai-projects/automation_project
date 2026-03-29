//! tools/versioning_automation/src/issues/tests/dispatch.rs
use crate::issues::dispatch::run;

#[test]
fn test_run_help() {
    let args = vec!["help".to_string()];
    let result = run(&args);
    assert_eq!(result, 0);
}

#[test]
fn test_run_invalid_command() {
    let args = vec!["invalid".to_string()];
    let result = run(&args);
    assert_eq!(result, 2);
}
