use crate::automation::commands::AutomationAction;

#[test]
fn parse_help_returns_help_action() {
    let args = vec!["help".to_string()];
    let action = super::super::parse::parse(&args).expect("parse help");
    match action {
        AutomationAction::Help => {}
        _ => panic!("expected help action"),
    }
}

#[test]
fn parse_labels_sync_rejects_unknown_option() {
    let args = vec!["labels-sync".to_string(), "--unknown".to_string()];
    let err = super::super::parse::parse(&args).expect_err("expected parse error");
    assert!(err.contains("Unexpected argument"));
}

#[test]
fn parse_ci_watch_pr_rejects_invalid_poll_interval() {
    let args = vec![
        "ci-watch-pr".to_string(),
        "--poll-interval".to_string(),
        "x".to_string(),
    ];
    let err = super::super::parse::parse(&args).expect_err("expected parse error");
    assert!(err.contains("--poll-interval must be a positive integer"));
}

#[test]
fn parse_changed_crates_accepts_two_refs() {
    let args = vec![
        "changed-crates".to_string(),
        "origin/main".to_string(),
        "origin/dev".to_string(),
    ];
    let action = super::super::parse::parse(&args).expect("parse changed-crates");
    match action {
        AutomationAction::ChangedCrates(opts) => {
            assert_eq!(opts.ref1.as_deref(), Some("origin/main"));
            assert_eq!(opts.ref2.as_deref(), Some("origin/dev"));
        }
        _ => panic!("expected changed-crates action"),
    }
}

#[test]
fn parse_check_merge_conflicts_rejects_unknown_option() {
    let args = vec!["check-merge-conflicts".to_string(), "--unknown".to_string()];
    let err = super::super::parse::parse(&args).expect_err("expected parse error");
    assert!(err.contains("Unexpected argument"));
}

#[test]
fn parse_audit_security_maps_action() {
    let args = vec!["audit-security".to_string()];
    let action = super::super::parse::parse(&args).expect("parse audit-security");
    match action {
        AutomationAction::AuditSecurity(_) => {}
        _ => panic!("expected audit-security action"),
    }
}

#[test]
fn parse_build_ui_bundles_rejects_unexpected_argument() {
    let args = vec!["build-ui-bundles".to_string(), "--x".to_string()];
    let err = super::super::parse::parse(&args).expect_err("expected parse error");
    assert!(err.contains("Unexpected argument"));
}

#[test]
fn parse_pre_add_review_maps_action() {
    let args = vec!["pre-add-review".to_string()];
    let action = super::super::parse::parse(&args).expect("parse pre-add-review");
    match action {
        AutomationAction::PreAddReview(_) => {}
        _ => panic!("expected pre-add-review action"),
    }
}

#[test]
fn parse_test_coverage_maps_action() {
    let args = vec!["test-coverage".to_string()];
    let action = super::super::parse::parse(&args).expect("parse test-coverage");
    match action {
        AutomationAction::TestCoverage(_) => {}
        _ => panic!("expected test-coverage action"),
    }
}
