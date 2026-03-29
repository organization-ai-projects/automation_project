//! tools/versioning_automation/src/pr/tests/execute.rs
use crate::pr::execute;

#[test]
fn run_returns_error_code_for_unknown_subcommand() {
    let code = execute::run(&["not-a-valid-command".to_string()]);
    assert_eq!(code, 2);
}

#[test]
fn run_help_returns_success() {
    let code = execute::run(&["help".to_string()]);
    assert_eq!(code, 0);
}
