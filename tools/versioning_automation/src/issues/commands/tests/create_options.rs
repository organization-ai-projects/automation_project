//! tools/versioning_automation/src/issues/commands/tests/create_options.rs
use crate::issues::commands::create_options::CreateOptions;

#[test]
fn test_run_create_success() {
    let options = CreateOptions {
        title: "Test Issue".to_string(),
        context: "Test Context".to_string(),
        problem: "Test Problem".to_string(),
        acceptances: vec!["Acceptance 1".to_string()],
        parent: "none".to_string(),
        labels: vec!["bug".to_string()],
        assignees: vec!["user1".to_string()],
        related_issues: vec!["#123".to_string()],
        related_prs: vec!["#456".to_string()],
        repo: Some("test_repo".to_string()),
        dry_run: true,
    };
    let result = options.run_create();
    assert_eq!(result, 0);
}

#[test]
fn test_run_create_missing_repo() {
    let options = CreateOptions {
        title: "Test Issue".to_string(),
        context: "Test Context".to_string(),
        problem: "Test Problem".to_string(),
        acceptances: vec!["Acceptance 1".to_string()],
        parent: "none".to_string(),
        labels: vec!["bug".to_string()],
        assignees: vec!["user1".to_string()],
        related_issues: vec!["#123".to_string()],
        related_prs: vec!["#456".to_string()],
        repo: None,
        dry_run: true,
    };
    let result = options.run_create();
    assert_eq!(result, 0);
}
