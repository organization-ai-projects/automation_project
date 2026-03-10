use crate::issues;

#[test]
fn parse_via_public_run_help_works() {
    let args = vec!["help".to_string()];
    let code = issues::run(&args);
    assert_eq!(code, 0);
}

#[test]
fn parse_reevaluate_requires_issue() {
    let args = vec!["reevaluate".to_string()];
    let code = issues::run(&args);
    assert_eq!(code, 2);
}

#[test]
fn parse_required_fields_validate_accepts_mode() {
    let args = vec![
        "required-fields-validate".to_string(),
        "--mode".to_string(),
        "content".to_string(),
        "--title".to_string(),
        "feat(scope): ok".to_string(),
        "--body".to_string(),
        "## Context".to_string(),
    ];
    let code = issues::run(&args);
    assert_eq!(code, 0);
}

#[test]
fn parse_fetch_non_compliance_requires_issue_number() {
    let args = vec!["fetch-non-compliance-reason".to_string()];
    let code = issues::run(&args);
    assert_eq!(code, 2);
}

#[test]
fn parse_label_exists_requires_repo_and_label() {
    let args = vec!["label-exists".to_string()];
    let code = issues::run(&args);
    assert_eq!(code, 2);
}

#[test]
fn parse_label_exists_accepts_required_fields() {
    let args = vec![
        "label-exists".to_string(),
        "--repo".to_string(),
        "owner/repo".to_string(),
        "--label".to_string(),
        "bug".to_string(),
    ];
    let code = issues::run(&args);
    assert_eq!(code, 0);
}

#[test]
fn parse_sync_project_status_requires_fields() {
    let args = vec!["sync-project-status".to_string()];
    let code = issues::run(&args);
    assert_eq!(code, 2);
}

#[test]
fn parse_sync_project_status_accepts_required_fields() {
    let args = vec![
        "sync-project-status".to_string(),
        "--repo".to_string(),
        "owner/repo".to_string(),
        "--issue".to_string(),
        "42".to_string(),
        "--status".to_string(),
        "Todo".to_string(),
    ];
    let code = issues::run(&args);
    assert_eq!(code, 0);
}

#[test]
fn parse_repo_name_rejects_unknown_options() {
    let args = vec!["repo-name".to_string(), "--repo".to_string()];
    let code = issues::run(&args);
    assert_eq!(code, 2);
}

#[test]
fn parse_create_accepts_extended_create_contract_flags() {
    let args = vec![
        "create".to_string(),
        "--title".to_string(),
        "feat(scope): summary".to_string(),
        "--context".to_string(),
        "context".to_string(),
        "--problem".to_string(),
        "problem".to_string(),
        "--acceptance".to_string(),
        "criterion".to_string(),
        "--template".to_string(),
        ".github/ISSUE_TEMPLATE/direct_issue.md".to_string(),
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
fn parse_tasklist_refs_requires_body() {
    let args = vec!["tasklist-refs".to_string()];
    let code = issues::run(&args);
    assert_eq!(code, 2);
}

#[test]
fn parse_tasklist_refs_accepts_body() {
    let args = vec![
        "tasklist-refs".to_string(),
        "--body".to_string(),
        "- [ ] #12".to_string(),
    ];
    let code = issues::run(&args);
    assert_eq!(code, 0);
}

#[test]
fn parse_subissue_refs_requires_all_fields() {
    let args = vec![
        "subissue-refs".to_string(),
        "--owner".to_string(),
        "o".to_string(),
    ];
    let code = issues::run(&args);
    assert_eq!(code, 2);
}

#[test]
fn parse_upsert_marker_comment_requires_required_fields() {
    let args = vec![
        "upsert-marker-comment".to_string(),
        "--repo".to_string(),
        "owner/repo".to_string(),
    ];
    let code = issues::run(&args);
    assert_eq!(code, 2);
}

#[test]
fn parse_open_numbers_accepts_optional_repo() {
    let args = vec![
        "open-numbers".to_string(),
        "--repo".to_string(),
        "owner/repo".to_string(),
    ];
    let code = issues::run(&args);
    assert_eq!(code, 0);
}

#[test]
fn parse_assignee_logins_requires_issue() {
    let args = vec!["assignee-logins".to_string()];
    let code = issues::run(&args);
    assert_eq!(code, 2);
}

#[test]
fn parse_state_requires_issue() {
    let args = vec!["state".to_string()];
    let code = issues::run(&args);
    assert_eq!(code, 2);
}

#[test]
fn parse_has_label_requires_issue_and_label() {
    let args = vec![
        "has-label".to_string(),
        "--issue".to_string(),
        "12".to_string(),
    ];
    let code = issues::run(&args);
    assert_eq!(code, 2);
}
