//! tools/versioning_automation/src/issues/commands/tests/close_options.rs
use crate::issues::commands::close_options::CloseOptions;

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
