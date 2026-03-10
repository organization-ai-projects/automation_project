use crate::issues;

#[test]
fn execute_create_dry_run_still_works_after_refactor() {
    let args = vec![
        "create".to_string(),
        "--title".to_string(),
        "feat(example): dry run".to_string(),
        "--context".to_string(),
        "ctx".to_string(),
        "--problem".to_string(),
        "problem".to_string(),
        "--acceptance".to_string(),
        "criterion".to_string(),
        "--dry-run".to_string(),
    ];
    let code = issues::run(&args);
    assert_eq!(code, 0);
}

#[test]
fn execute_create_dry_run_accepts_related_refs_and_assignee() {
    let args = vec![
        "create".to_string(),
        "--title".to_string(),
        "feat(example): dry run refs".to_string(),
        "--context".to_string(),
        "ctx".to_string(),
        "--problem".to_string(),
        "problem".to_string(),
        "--acceptance".to_string(),
        "criterion".to_string(),
        "--assignee".to_string(),
        "octocat".to_string(),
        "--related-issue".to_string(),
        "#12".to_string(),
        "--related-pr".to_string(),
        "#34".to_string(),
        "--dry-run".to_string(),
    ];
    let code = issues::run(&args);
    assert_eq!(code, 0);
}

#[test]
fn execute_non_compliance_reason_runs() {
    let args = vec![
        "non-compliance-reason".to_string(),
        "--title".to_string(),
        "feat(scope): summary".to_string(),
        "--body".to_string(),
        "## Context\n\nx\n\n## Problem\n\ny\n\n## Acceptance Criteria\n\nDone when :\n\n- [ ] z\n\n## Hierarchy\n\nParent: none".to_string(),
    ];
    let code = issues::run(&args);
    assert_eq!(code, 0);
}
