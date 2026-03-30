//! tools/versioning_automation/src/issues/commands/tests/auto_link_options.rs
use crate::issues::commands::auto_link_options::{AutoLinkError, AutoLinkOptions};

#[test]
fn test_run_auto_link_with_valid_issue_and_repo() {
    let options = AutoLinkOptions {
        issue: "123".to_string(),
        repo: Some("owner/repo".to_string()),
    };
    let result = options.run_auto_link();
    assert!(
        matches!(
            result,
            Err(AutoLinkError::UnableToReadIssue(issue)) if issue == "123"
        ),
        "Expected AutoLinkError::UnableToReadIssue for empty issue state"
    );
}

#[test]
fn test_run_auto_link_with_missing_repo() {
    let options = AutoLinkOptions {
        issue: "123".to_string(),
        repo: None,
    };
    let result = options.run_auto_link();
    match result {
        Err(AutoLinkError::RepoNotSpecified { issue, message }) => {
            println!(
                "Captured RepoNotSpecified error: issue = {}, message = {}",
                issue, message
            );
            assert_eq!(issue, "123", "Expected issue to be '123'");
            assert!(
                !message.is_empty(),
                "Expected a non-empty error message for missing repo"
            );
        }
        _ => panic!("Expected AutoLinkError::RepoNotSpecified for missing repo"),
    }
}

#[test]
fn test_run_auto_link_with_empty_issue() {
    let options = AutoLinkOptions {
        issue: "".to_string(),
        repo: Some("owner/repo".to_string()),
    };
    let result = options.run_auto_link();
    assert!(
        matches!(result, Err(AutoLinkError::MissingIssueField)),
        "Expected AutoLinkError::MissingIssueField for empty issue"
    );
}
