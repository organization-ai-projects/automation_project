use crate::git::commands::GitAction;

#[test]
fn parse_help_returns_help_action() {
    let args = vec!["help".to_string()];
    let action = super::super::parse::parse(&args).expect("parse help");
    match action {
        GitAction::Help => {}
        _ => panic!("expected help action"),
    }
}

#[test]
fn parse_create_work_branch_requires_type_and_description() {
    let args = vec!["create-work-branch".to_string()];
    let err = super::super::parse::parse(&args).expect_err("expected parse error");
    assert!(err.contains("Usage: create-work-branch"));
}

#[test]
fn parse_push_branch_rejects_unknown_option() {
    let args = vec!["push-branch".to_string(), "--unknown".to_string()];
    let err = super::super::parse::parse(&args).expect_err("expected parse error");
    assert!(err.contains("Unexpected argument"));
}
