use crate::issues::commands::{CreateOptions, NonComplianceReasonOptions};
use crate::issues::execute::{
    IssueSyncIntent, extract_closing_issue_numbers, extract_reopen_issue_numbers, plan_issue_sync,
    plan_reopen_sync, pr_state_allows_reopen_sync, run_create, run_non_compliance_reason,
};

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

#[test]
fn extract_closing_issue_numbers_ignores_cancelled_closes() {
    let out = extract_closing_issue_numbers("Closes #12\nCancel-Closes #12\nCloses #13");
    assert_eq!(out, vec!["13".to_string()]);
}

#[test]
fn extract_reopen_issue_numbers_keeps_effective_reopen() {
    let out = extract_reopen_issue_numbers("Closes #12\nCancel-Closes #12\nReopen #12");
    assert_eq!(out, vec!["12".to_string()]);
}

#[test]
fn extract_reopen_issue_numbers_ignores_reopen_when_later_close_wins() {
    let out = extract_reopen_issue_numbers("Reopen #12\nCloses #12");
    assert!(out.is_empty());
}

#[test]
fn extract_closing_issue_numbers_keeps_later_close_after_reopen() {
    let out = extract_closing_issue_numbers("Reopen #12\nCloses #12");
    assert_eq!(out, vec!["12".to_string()]);
}

#[test]
fn reopen_sync_allows_open_and_merged_pr_states() {
    assert!(pr_state_allows_reopen_sync("OPEN"));
    assert!(pr_state_allows_reopen_sync("MERGED"));
    assert!(!pr_state_allows_reopen_sync("CLOSED"));
    assert!(!pr_state_allows_reopen_sync(""));
}

#[test]
fn plan_reopen_sync_reopens_closed_issue_and_removes_done_in_dev_when_present() {
    let plan = plan_reopen_sync("CLOSED", true);
    assert!(plan.reopen_issue);
    assert!(!plan.add_done_in_dev_label);
    assert!(plan.remove_done_in_dev_label);
}

#[test]
fn plan_reopen_sync_only_removes_done_in_dev_for_open_issue() {
    let plan = plan_reopen_sync("OPEN", true);
    assert!(!plan.reopen_issue);
    assert!(!plan.add_done_in_dev_label);
    assert!(plan.remove_done_in_dev_label);
}

#[test]
fn plan_reopen_sync_is_noop_for_open_issue_without_done_in_dev() {
    let plan = plan_reopen_sync("OPEN", false);
    assert!(!plan.reopen_issue);
    assert!(!plan.add_done_in_dev_label);
    assert!(!plan.remove_done_in_dev_label);
}

#[test]
fn plan_issue_sync_adds_done_in_dev_only_for_open_issue_without_label() {
    let plan = plan_issue_sync("OPEN", false, IssueSyncIntent::MarkDoneInDev);
    assert!(!plan.reopen_issue);
    assert!(plan.add_done_in_dev_label);
    assert!(!plan.remove_done_in_dev_label);
}

#[test]
fn plan_issue_sync_skips_done_in_dev_when_label_is_already_present() {
    let plan = plan_issue_sync("OPEN", true, IssueSyncIntent::MarkDoneInDev);
    assert!(!plan.reopen_issue);
    assert!(!plan.add_done_in_dev_label);
    assert!(!plan.remove_done_in_dev_label);
}

#[test]
fn plan_issue_sync_skips_done_in_dev_for_closed_issue() {
    let plan = plan_issue_sync("CLOSED", false, IssueSyncIntent::MarkDoneInDev);
    assert!(!plan.reopen_issue);
    assert!(!plan.add_done_in_dev_label);
    assert!(!plan.remove_done_in_dev_label);
}
