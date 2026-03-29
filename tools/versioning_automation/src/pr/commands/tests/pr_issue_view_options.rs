//! tools/versioning_automation/src/pr/commands/tests/pr_issue_view_options.rs
use crate::pr::commands::PrIssueViewOptions;

#[test]
fn test_run_issue_view_valid() {
    let options = PrIssueViewOptions {
        issue_number: "456".to_string(),
        repo: Some("test_repo".to_string()),
    };
    let result = options.run_issue_view();
    assert_eq!(result, 0);
}

#[test]
fn test_run_issue_view_invalid_repo() {
    let options = PrIssueViewOptions {
        issue_number: "456".to_string(),
        repo: Some("invalid_repo".to_string()),
    };
    let result = options.run_issue_view();
    assert_eq!(result, 0);
}
