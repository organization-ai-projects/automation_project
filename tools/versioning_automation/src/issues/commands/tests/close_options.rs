//! tools/versioning_automation/src/issues/commands/tests/close_options.rs
use crate::issues::commands::close_options::CloseOptions;

#[test]
fn test_run_close_success() {
    let options = CloseOptions {
        issue: "123".to_string(),
        repo: Some("test_repo".to_string()),
        reason: "completed".to_string(),
        comment: Some("Closing issue.".to_string()),
    };
    let result = options.run_close();
    assert_eq!(result, 0);
}

#[test]
fn test_run_close_missing_repo() {
    let options = CloseOptions {
        issue: "123".to_string(),
        repo: None,
        reason: "completed".to_string(),
        comment: None,
    };
    let result = options.run_close();
    assert_eq!(result, 0);
}
