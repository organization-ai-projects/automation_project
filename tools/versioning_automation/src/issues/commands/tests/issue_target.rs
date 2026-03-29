//! tools/versioning_automation/src/issues/commands/tests/issue_target.rs
use crate::issues::commands::issue_target::IssueTarget;

#[test]
fn test_run_reopen_success() {
    let options = IssueTarget {
        issue: "123".to_string(),
        repo: Some("test_repo".to_string()),
    };
    let result = options.run_reopen();
    assert_eq!(result, 0);
}

#[test]
fn test_run_delete_success() {
    let options = IssueTarget {
        issue: "123".to_string(),
        repo: Some("test_repo".to_string()),
    };
    let result = options.run_delete();
    assert_eq!(result, 0);
}
