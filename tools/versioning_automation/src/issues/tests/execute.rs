use crate::issues::commands::{CreateOptions, NonComplianceReasonOptions};
use crate::issues::execute::{run_create, run_non_compliance_reason};

#[test]
fn execute_create_dry_run_still_works_after_refactor() {
    let code = run_create(CreateOptions {
        title: "feat(example): dry run".to_string(),
        context: "ctx".to_string(),
        problem: "problem".to_string(),
        acceptances: vec!["criterion".to_string()],
        parent: "none".to_string(),
        labels: vec!["issue".to_string()],
        assignees: vec![],
        related_issues: vec![],
        related_prs: vec![],
        repo: None,
        dry_run: true,
    });
    assert_eq!(code, 0);
}

#[test]
fn execute_create_dry_run_accepts_related_refs_and_assignee() {
    let code = run_create(CreateOptions {
        title: "feat(example): dry run refs".to_string(),
        context: "ctx".to_string(),
        problem: "problem".to_string(),
        acceptances: vec!["criterion".to_string()],
        parent: "none".to_string(),
        labels: vec!["issue".to_string()],
        assignees: vec!["octocat".to_string()],
        related_issues: vec!["#12".to_string()],
        related_prs: vec!["#34".to_string()],
        repo: None,
        dry_run: true,
    });
    assert_eq!(code, 0);
}

#[test]
fn execute_non_compliance_reason_runs() {
    let code = run_non_compliance_reason(NonComplianceReasonOptions {
        title: "feat(scope): summary".to_string(),
        body: String::new(),
        labels_raw: "issue-required-missing".to_string(),
    });
    assert_eq!(code, 0);
}
