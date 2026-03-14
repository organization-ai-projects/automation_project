//! tools/versioning_automation/src/git/tests/parse.rs
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

#[test]
fn parse_branch_creation_check_maps_passthrough_command() {
    let args = vec![
        "branch-creation-check".to_string(),
        "checkout".to_string(),
        "-b".to_string(),
        "feat/test".to_string(),
    ];
    let action = super::super::parse::parse(&args).expect("parse branch-creation-check");
    match action {
        GitAction::BranchCreationCheck(opts) => {
            assert_eq!(opts.command.as_deref(), Some("checkout"));
            assert_eq!(opts.args, vec!["-b".to_string(), "feat/test".to_string()]);
        }
        _ => panic!("expected BranchCreationCheck action"),
    }
}
