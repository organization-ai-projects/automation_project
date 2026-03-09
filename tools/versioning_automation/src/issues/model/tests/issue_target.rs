use crate::issues::model::IssueTarget;

#[test]
fn issue_target_can_be_built() {
    let value = IssueTarget {
        issue: "42".to_string(),
        repo: Some("org/repo".to_string()),
    };
    assert_eq!(value.repo.as_deref(), Some("org/repo"));
}
