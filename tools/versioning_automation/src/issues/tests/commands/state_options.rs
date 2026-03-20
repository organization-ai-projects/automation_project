use crate::issues::commands::StateOptions;

#[test]
fn state_options_can_be_built() {
    let value = StateOptions {
        issue: "42".to_string(),
        repo: Some("owner/repo".to_string()),
    };
    assert_eq!(value.issue, "42");
    assert_eq!(value.repo.as_deref(), Some("owner/repo"));
}
