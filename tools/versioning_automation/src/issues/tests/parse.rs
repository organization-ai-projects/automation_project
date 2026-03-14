//! tools/versioning_automation/src/issues/tests/parse.rs
use crate::issues::commands::{
    DoneStatusMode, IssueAction, IssueFieldName, RequiredFieldsValidationMode,
};
use crate::issues::parse::parse;

fn to_args(values: &[&str]) -> Vec<String> {
    values.iter().map(|v| (*v).to_string()).collect()
}

#[test]
fn parse_help_returns_help_action() {
    let action = parse(&to_args(&["help"])).expect("parse help");
    match action {
        IssueAction::Help => {}
        _ => panic!("expected Help action"),
    }
}

#[test]
fn parse_unknown_subcommand_returns_error() {
    let err = parse(&to_args(&["unknown"])).expect_err("expected parse error");
    assert!(err.contains("Unknown issue subcommand"));
}

#[test]
fn parse_create_maps_required_and_optional_flags() {
    let action = parse(&to_args(&[
        "create",
        "--title",
        "feat(scope): summary",
        "--context",
        "context",
        "--problem",
        "problem",
        "--acceptance",
        "criterion",
        "--assignee",
        "octocat",
        "--related-issue",
        "#12",
        "--related-pr",
        "#34",
        "--dry-run",
    ]))
    .expect("parse create");

    match action {
        IssueAction::Create(opts) => {
            assert_eq!(opts.title, "feat(scope): summary");
            assert_eq!(opts.context, "context");
            assert_eq!(opts.problem, "problem");
            assert_eq!(opts.acceptances, vec!["criterion".to_string()]);
            assert_eq!(opts.assignees, vec!["octocat".to_string()]);
            assert_eq!(opts.related_issues, vec!["#12".to_string()]);
            assert_eq!(opts.related_prs, vec!["#34".to_string()]);
            assert!(opts.dry_run);
            assert!(opts.labels.iter().any(|l| l == "issue"));
        }
        _ => panic!("expected Create action"),
    }
}

#[test]
fn parse_create_requires_acceptance() {
    let err = parse(&to_args(&[
        "create",
        "--title",
        "feat(scope): summary",
        "--context",
        "context",
        "--problem",
        "problem",
    ]))
    .expect_err("expected parse error");

    assert!(err.contains("create requires"));
}

#[test]
fn parse_done_status_on_dev_merge_maps_mode_and_pr() {
    let action = parse(&to_args(&[
        "done-status",
        "--on-dev-merge",
        "--pr",
        "12",
        "--label",
        "done-in-dev",
    ]))
    .expect("parse done-status");

    match action {
        IssueAction::DoneStatus(opts) => {
            match opts.mode {
                DoneStatusMode::OnDevMerge => {}
                _ => panic!("expected OnDevMerge mode"),
            }
            assert_eq!(opts.pr, Some("12".to_string()));
            assert_eq!(opts.issue, None);
            assert_eq!(opts.label, "done-in-dev");
        }
        _ => panic!("expected DoneStatus action"),
    }
}

#[test]
fn parse_done_status_requires_mode() {
    let err = parse(&to_args(&["done-status", "--pr", "12"])).expect_err("expected parse error");
    assert!(err.contains("requires one mode"));
}

#[test]
fn parse_done_status_rejects_non_numeric_pr() {
    let err = parse(&to_args(&["done-status", "--on-dev-merge", "--pr", "abc"]))
        .expect_err("expected parse error");

    assert!(err.contains("--pr requires a positive integer"));
}

#[test]
fn parse_parent_guard_issue_mode() {
    let action = parse(&to_args(&[
        "parent-guard",
        "--issue",
        "12",
        "--strict-guard",
        "false",
    ]))
    .expect("parse parent-guard");

    match action {
        IssueAction::ParentGuard(opts) => {
            assert_eq!(opts.issue, Some("12".to_string()));
            assert_eq!(opts.child, None);
            assert!(!opts.strict_guard);
        }
        _ => panic!("expected ParentGuard action"),
    }
}

#[test]
fn parse_parent_guard_rejects_issue_and_child_together() {
    let err = parse(&to_args(&[
        "parent-guard",
        "--issue",
        "12",
        "--child",
        "34",
    ]))
    .expect_err("expected parse error");

    assert!(err.contains("not both"));
}

#[test]
fn parse_parent_guard_rejects_invalid_strict_guard_value() {
    let err = parse(&to_args(&[
        "parent-guard",
        "--issue",
        "12",
        "--strict-guard",
        "yes",
    ]))
    .expect_err("expected parse error");

    assert!(err.contains("expects true|false"));
}

#[test]
fn parse_required_fields_validate_maps_mode() {
    let action = parse(&to_args(&[
        "required-fields-validate",
        "--mode",
        "content",
        "--title",
        "feat(scope): ok",
    ]))
    .expect("parse required-fields-validate");

    match action {
        IssueAction::RequiredFieldsValidate(opts) => {
            assert_eq!(opts.mode, RequiredFieldsValidationMode::Content);
            assert_eq!(opts.title, "feat(scope): ok");
        }
        _ => panic!("expected RequiredFieldsValidate action"),
    }
}

#[test]
fn parse_required_fields_validate_rejects_invalid_mode() {
    let err = parse(&to_args(&["required-fields-validate", "--mode", "invalid"]))
        .expect_err("expected parse error");

    assert!(err.contains("--mode must be one of"));
}

#[test]
fn parse_read_rejects_non_numeric_issue() {
    let err = parse(&to_args(&["read", "--issue", "abc"])).expect_err("expected parse error");
    assert!(err.contains("--issue requires a positive integer"));
}

#[test]
fn parse_update_requires_edit_option() {
    let err = parse(&to_args(&["update", "--issue", "12"])).expect_err("expected parse error");
    assert!(err.contains("requires at least one edit option"));
}

#[test]
fn parse_close_rejects_invalid_reason() {
    let err = parse(&to_args(&["close", "--issue", "12", "--reason", "invalid"]))
        .expect_err("expected parse error");

    assert!(err.contains("--reason must be"));
}

#[test]
fn parse_field_maps_labels_raw_name() {
    let action = parse(&to_args(&[
        "field",
        "--issue",
        "12",
        "--name",
        "labels-raw",
    ]))
    .expect("parse field");

    match action {
        IssueAction::Field(opts) => {
            assert_eq!(opts.issue, "12");
            assert_eq!(opts.repo, None);
            assert_eq!(opts.name, IssueFieldName::LabelsRaw);
        }
        _ => panic!("expected Field action"),
    }
}

#[test]
fn parse_field_rejects_invalid_name() {
    let err = parse(&to_args(&["field", "--issue", "12", "--name", "invalid"]))
        .expect_err("expected parse error");

    assert!(err.contains("--name must be one of"));
}

#[test]
fn parse_repo_name_rejects_extra_options() {
    let err =
        parse(&to_args(&["repo-name", "--repo", "owner/repo"])).expect_err("expected parse error");
    assert!(err.contains("does not accept additional options"));
}

#[test]
fn parse_current_login_returns_action() {
    let action = parse(&to_args(&["current-login"])).expect("parse current-login");
    match action {
        IssueAction::CurrentLogin => {}
        _ => panic!("expected CurrentLogin action"),
    }
}

#[test]
fn parse_current_login_rejects_extra_options() {
    let err = parse(&to_args(&["current-login", "--repo", "owner/repo"]))
        .expect_err("expected parse error");
    assert!(err.contains("does not accept additional options"));
}

#[test]
fn parse_upsert_marker_comment_rejects_invalid_announce_bool() {
    let err = parse(&to_args(&[
        "upsert-marker-comment",
        "--repo",
        "owner/repo",
        "--issue",
        "12",
        "--marker",
        "marker",
        "--body",
        "body",
        "--announce",
        "yes",
    ]))
    .expect_err("expected parse error");

    assert!(err.contains("expects true|false"));
}

#[test]
fn parse_tasklist_refs_requires_body() {
    let err = parse(&to_args(&["tasklist-refs"])).expect_err("expected parse error");
    assert!(err.contains("tasklist-refs requires: --body"));
}
