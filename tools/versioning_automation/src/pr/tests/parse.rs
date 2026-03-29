//! tools/versioning_automation/src/pr/tests/parse.rs
use crate::pr::{
    commands::{PrAction, PrFieldName},
    parse::parse,
};

fn to_args(values: &[&str]) -> Vec<String> {
    values.iter().map(|v| (*v).to_string()).collect()
}

#[test]
fn parse_help_returns_help_action() {
    let action = parse(&to_args(&["help"])).expect("parse help");
    match action {
        PrAction::Help => {}
        _ => panic!("expected Help action"),
    }
}

#[test]
fn parse_unknown_subcommand_returns_error() {
    let err = parse(&to_args(&["unknown"])).expect_err("expected parse error");
    assert!(err.contains("Unknown pr subcommand"));
}

#[test]
fn parse_field_maps_state_name() {
    let action = parse(&to_args(&["field", "--pr", "42", "--name", "state"])).expect("parse field");

    match action {
        PrAction::Field(opts) => {
            assert_eq!(opts.pr_number, "42");
            assert_eq!(opts.name, PrFieldName::State);
        }
        _ => panic!("expected Field action"),
    }
}

#[test]
fn parse_field_rejects_invalid_name() {
    let err = parse(&to_args(&["field", "--pr", "42", "--name", "invalid"]))
        .expect_err("expected parse error");

    assert!(err.contains("--name must be one of"));
}

#[test]
fn parse_field_rejects_invalid_pr_number() {
    let err = parse(&to_args(&["field", "--pr", "abc", "--name", "state"]))
        .expect_err("expected parse error");

    assert!(err.contains("--pr requires a positive numeric value"));
}

#[test]
fn parse_directives_requires_input_source() {
    let err = parse(&to_args(&["directives"])).expect_err("expected parse error");
    assert!(err.contains("directives requires --text <value> or --stdin"));
}

#[test]
fn parse_directives_apply_requires_input_source() {
    let err = parse(&to_args(&["directives-apply"])).expect_err("expected parse error");
    assert!(err.contains("directives-apply requires --text <value> or --stdin"));
}

#[test]
fn parse_directive_conflicts_with_text_maps_action() {
    let action = parse(&to_args(&[
        "directive-conflicts",
        "--text",
        "Closes #1\nReopen #1",
    ]))
    .expect("parse directive-conflicts");

    match action {
        PrAction::DirectiveConflicts(opts) => {
            assert!(opts.text.contains("Closes #1"));
        }
        _ => panic!("expected DirectiveConflicts action"),
    }
}

#[test]
fn parse_directive_conflict_guard_requires_pr() {
    let err = parse(&to_args(&["directive-conflict-guard"])).expect_err("expected parse error");
    assert!(err.contains("--pr"));
}

#[test]
fn parse_duplicate_actions_requires_mode_and_repo() {
    let err = parse(&to_args(&["duplicate-actions", "--text", "#2|#1"]))
        .expect_err("expected parse error");

    assert!(err.contains("--mode is required") || err.contains("--repo is required"));
}

#[test]
fn parse_group_by_category_requires_mode() {
    let err = parse(&to_args(&[
        "group-by-category",
        "--text",
        "1|Bug Fixes|Closes|#1",
    ]))
    .expect_err("expected parse error");

    assert!(err.contains("--mode is required"));
}

#[test]
fn parse_issue_context_requires_issue() {
    let err = parse(&to_args(&["issue-context"])).expect_err("expected parse error");
    assert!(err.contains("--issue requires a positive numeric value"));
}

#[test]
fn parse_pr_state_requires_pr() {
    let err = parse(&to_args(&["pr-state"])).expect_err("expected parse error");
    assert!(err.contains("--pr requires a positive numeric value"));
}

#[test]
fn parse_update_body_requires_body() {
    let err = parse(&to_args(&["update-body", "--pr", "42"])).expect_err("expected parse error");

    assert!(err.contains("--body is required"));
}

#[test]
fn parse_upsert_comment_requires_marker_and_body() {
    let err = parse(&to_args(&["upsert-comment", "--pr", "42"])).expect_err("expected parse error");

    assert!(err.contains("--marker is required") || err.contains("--body is required"));
}

#[test]
fn parse_open_referencing_issue_requires_issue() {
    let err = parse(&to_args(&["open-referencing-issue"])).expect_err("expected parse error");
    assert!(err.contains("--issue requires a positive numeric value"));
}

#[test]
fn parse_normalize_issue_key_requires_raw() {
    let err = parse(&to_args(&["normalize-issue-key"])).expect_err("expected parse error");
    assert!(err.contains("--raw is required"));
}

#[test]
fn parse_sort_bullets_requires_input_file() {
    let err = parse(&to_args(&["sort-bullets"])).expect_err("expected parse error");
    assert!(err.contains("--input-file is required"));
}

#[test]
fn parse_issue_close_policy_maps_required_flags() {
    let action = parse(&to_args(&[
        "issue-close-policy",
        "--action",
        "Closes",
        "--is-pr-ref",
        "false",
        "--non-compliance-reason",
        "reason",
    ]))
    .expect("parse issue-close-policy");

    match action {
        PrAction::IssueClosePolicy(opts) => {
            assert_eq!(opts.action, "Closes");
            assert!(!opts.is_pr_ref);
            assert_eq!(opts.non_compliance_reason, "reason");
        }
        _ => panic!("expected IssueClosePolicy action"),
    }
}

#[test]
fn parse_closure_marker_requires_flags() {
    let err = parse(&to_args(&["closure-marker"])).expect_err("expected parse error");
    assert!(err.contains("--text") || err.contains("--keyword-pattern") || err.contains("--issue"));
}
