use crate::issues::commands::{IssueFieldName, IssueFieldOptions};

#[test]
fn issue_field_options_store_requested_field() {
    let options = IssueFieldOptions {
        issue: "42".to_string(),
        repo: Some("owner/repo".to_string()),
        name: IssueFieldName::LabelsRaw,
    };

    assert_eq!(options.issue, "42");
    assert_eq!(options.repo.as_deref(), Some("owner/repo"));
    assert_eq!(options.name, IssueFieldName::LabelsRaw);
}
