//! tools/versioning_automation/src/issues/tests/render.rs
use crate::issues::{commands::CreateOptions, print_usage, render_direct_issue_body};

#[test]
fn render_usage_prints_without_panic() {
    print_usage();
}

#[test]
fn render_direct_issue_body_includes_references_section_when_present() {
    let options = CreateOptions {
        title: "feat(scope): title".to_string(),
        context: "Context".to_string(),
        problem: "Problem".to_string(),
        acceptances: vec!["One criterion".to_string()],
        parent: "none".to_string(),
        labels: vec![],
        assignees: vec![],
        related_issues: vec!["#123".to_string()],
        related_prs: vec!["#456".to_string()],
        repo: None,
        dry_run: true,
    };

    let body = render_direct_issue_body(&options);
    assert!(body.contains("## References"));
    assert!(body.contains("Related issue(s): #123"));
    assert!(body.contains("Related PR(s): #456"));
}
