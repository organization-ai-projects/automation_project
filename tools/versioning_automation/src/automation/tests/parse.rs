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
fn parse_pre_commit_check_maps_action() {
    let args = vec!["pre-commit-check".to_string()];
    let action = super::super::parse::parse(&args).expect("parse pre-commit-check");
    match action {
        AutomationAction::PreCommitCheck(_) => {}
        _ => panic!("expected pre-commit-check action"),
    }
}

#[test]
fn parse_post_checkout_check_maps_action() {
    let args = vec!["post-checkout-check".to_string()];
    let action = super::super::parse::parse(&args).expect("parse post-checkout-check");
    match action {
        AutomationAction::PostCheckoutCheck(_) => {}
        _ => panic!("expected post-checkout-check action"),
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

#[test]
fn parse_audit_issue_status_maps_options() {
    let args = vec![
        "audit-issue-status".to_string(),
        "--base".to_string(),
        "origin/release".to_string(),
        "--head".to_string(),
        "origin/dev".to_string(),
        "--limit".to_string(),
        "10".to_string(),
    ];
    let action = super::super::parse::parse(&args).expect("parse audit-issue-status");
    match action {
        AutomationAction::AuditIssueStatus(opts) => {
            assert_eq!(opts.base_ref, "origin/release");
            assert_eq!(opts.head_ref, "origin/dev");
            assert_eq!(opts.limit, 10);
        }
        _ => panic!("expected audit-issue-status action"),
    }
}

#[test]
fn parse_release_prepare_requires_version() {
    let args = vec!["release-prepare".to_string()];
    let err = super::super::parse::parse(&args).expect_err("expected parse error");
    assert!(err.contains("release-prepare requires"));
}

#[test]
fn parse_release_prepare_maps_options() {
    let args = vec![
        "release-prepare".to_string(),
        "1.2.3".to_string(),
        "--auto-changelog".to_string(),
    ];
    let action = super::super::parse::parse(&args).expect("parse release-prepare");
    match action {
        AutomationAction::ReleasePrepare(opts) => {
            assert_eq!(opts.version, "1.2.3");
            assert!(opts.auto_changelog);
        }
        _ => panic!("expected release-prepare action"),
    }
}

#[test]
fn parse_pre_push_check_maps_action() {
    let args = vec!["pre-push-check".to_string()];
    let action = super::super::parse::parse(&args).expect("parse pre-push-check");
    match action {
        AutomationAction::PrePushCheck(_) => {}
        _ => panic!("expected pre-push-check action"),
    }
}

#[test]
fn parse_install_hooks_maps_action() {
    let args = vec!["install-hooks".to_string()];
    let action = super::super::parse::parse(&args).expect("parse install-hooks");
    match action {
        AutomationAction::InstallHooks(_) => {}
        _ => panic!("expected install-hooks action"),
    }
}

#[test]
fn parse_commit_msg_check_maps_action() {
    let args = vec![
        "commit-msg-check".to_string(),
        "--file".to_string(),
        ".git/COMMIT_EDITMSG".to_string(),
    ];
    let action = super::super::parse::parse(&args).expect("parse commit-msg-check");
    match action {
        AutomationAction::CommitMsgCheck(_) => {}
        _ => panic!("expected commit-msg-check action"),
    }
}

#[test]
fn parse_prepare_commit_msg_maps_action() {
    let args = vec![
        "prepare-commit-msg".to_string(),
        "--file".to_string(),
        ".git/COMMIT_EDITMSG".to_string(),
        "--source".to_string(),
        "template".to_string(),
    ];
    let action = super::super::parse::parse(&args).expect("parse prepare-commit-msg");
    match action {
        AutomationAction::PrepareCommitMsg(_) => {}
        _ => panic!("expected prepare-commit-msg action"),
    }
}

#[test]
fn parse_pre_branch_create_check_maps_action() {
    let args = vec![
        "pre-branch-create-check".to_string(),
        "--branch".to_string(),
        "feature/test".to_string(),
    ];
    let action = super::super::parse::parse(&args).expect("parse pre-branch-create-check");
    match action {
        AutomationAction::PreBranchCreateCheck(_) => {}
        _ => panic!("expected pre-branch-create-check action"),
    }
}
