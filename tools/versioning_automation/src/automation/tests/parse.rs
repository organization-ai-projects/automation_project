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
