use crate::issues::contracts::cli::ReadOptions;

#[test]
fn read_options_can_be_built() {
    let value = ReadOptions {
        issue: Some("42".to_string()),
        repo: Some("org/repo".to_string()),
        json: Some("title".to_string()),
        jq: None,
        template: None,
    };
    assert_eq!(value.issue.as_deref(), Some("42"));
}
