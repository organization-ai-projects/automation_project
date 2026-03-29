//! tools/versioning_automation/src/issues/commands/tests/assignee_logins_options.rs
use crate::issues::commands::assignee_logins_options::AssigneeLoginsOptions;

#[test]
fn test_run_assignee_logins_success() {
    let options = AssigneeLoginsOptions {
        issue: "123".to_string(),
        repo: Some("test_repo".to_string()),
    };
    let result = options.run_assignee_logins();
    assert_eq!(result, 0);
}

#[test]
fn test_run_assignee_logins_missing_repo() {
    let options = AssigneeLoginsOptions {
        issue: "123".to_string(),
        repo: None,
    };
    let result = options.run_assignee_logins();
    assert_eq!(result, 0);
}
