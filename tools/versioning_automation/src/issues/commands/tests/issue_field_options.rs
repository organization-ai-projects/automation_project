//! tools/versioning_automation/src/issues/commands/tests/issue_field_options.rs
use crate::issues::commands::IssueFieldName;
use crate::issues::commands::issue_field_options::IssueFieldOptions;

#[test]
fn test_run_field_title() {
    let options = IssueFieldOptions {
        issue: "123".to_string(),
        repo: Some("test_repo".to_string()),
        name: IssueFieldName::Title,
    };
    let result = options.run_field();
    assert_eq!(result, 0);
}

#[test]
fn test_run_field_labels_raw() {
    let options = IssueFieldOptions {
        issue: "123".to_string(),
        repo: Some("test_repo".to_string()),
        name: IssueFieldName::LabelsRaw,
    };
    let result = options.run_field();
    assert_eq!(result, 0);
}
