//! tools/versioning_automation/src/issues/commands/tests/state_options.rs
use crate::issues::commands::state_options::StateOptions;

#[test]
fn test_run_state_with_repo() {
    let options = StateOptions {
        issue: "123".to_string(),
        repo: Some("test_repo".to_string()),
    };
    let result = options.run_state();
    assert_eq!(result, 0);
}

#[test]
fn test_run_state_without_repo() {
    let options = StateOptions {
        issue: "123".to_string(),
        repo: None,
    };
    let result = options.run_state();
    assert_eq!(result, 0);
}
