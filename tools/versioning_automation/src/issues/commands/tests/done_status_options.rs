//! tools/versioning_automation/src/issues/commands/tests/done_status_options.rs
use crate::issues::commands::done_status_mode::DoneStatusMode;
use crate::issues::commands::done_status_options::DoneStatusOptions;

#[test]
fn test_run_done_status_on_dev_merge() {
    let options = DoneStatusOptions {
        mode: DoneStatusMode::OnDevMerge,
        pr: Some("123".to_string()),
        issue: None,
        label: "done".to_string(),
        repo: Some("test_repo".to_string()),
    };
    let result = options.run_done_status();
    assert_eq!(result, 0);
}

#[test]
fn test_run_done_status_on_issue_closed() {
    let options = DoneStatusOptions {
        mode: DoneStatusMode::OnIssueClosed,
        pr: None,
        issue: Some("123".to_string()),
        label: "done".to_string(),
        repo: Some("test_repo".to_string()),
    };
    let result = options.run_done_status();
    assert_eq!(result, 0);
}
