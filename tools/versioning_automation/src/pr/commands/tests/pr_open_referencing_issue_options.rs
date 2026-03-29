//! tools/versioning_automation/src/pr/commands/tests/pr_open_referencing_issue_options.rs
use crate::pr::commands::PrOpenReferencingIssueOptions;

#[test]
fn test_run_open_referencing_issue_valid() {
    let options = PrOpenReferencingIssueOptions {
        issue_number: "789".to_string(),
        repo: Some("test_repo".to_string()),
    };
    let result = options.run_open_referencing_issue();
    assert_eq!(result, 0);
}

#[test]
fn test_run_open_referencing_issue_invalid_repo() {
    let options = PrOpenReferencingIssueOptions {
        issue_number: "789".to_string(),
        repo: Some("invalid_repo".to_string()),
    };
    let result = options.run_open_referencing_issue();
    assert_eq!(result, 0);
}
