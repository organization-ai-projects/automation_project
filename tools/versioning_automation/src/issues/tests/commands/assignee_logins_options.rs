use crate::issues::commands::AssigneeLoginsOptions;

#[test]
fn assignee_logins_options_can_be_built() {
    let value = AssigneeLoginsOptions {
        issue: "12".to_string(),
        repo: Some("owner/repo".to_string()),
    };
    assert_eq!(value.issue, "12");
    assert_eq!(value.repo.as_deref(), Some("owner/repo"));
}
