//! tools/versioning_automation/src/issues/commands/tests/reopen_on_dev_options.rs
use crate::issues::commands::reopen_on_dev_options::ReopenOnDevOptions;

#[test]
fn test_run_reopen_on_dev_with_repo() {
    let options = ReopenOnDevOptions {
        pr: "456".to_string(),
        label: "bug".to_string(),
        repo: Some("test_repo".to_string()),
    };
    let result = options.run_reopen_on_dev();
    assert_eq!(result, 0);
}

#[test]
fn test_run_reopen_on_dev_without_repo() {
    let options = ReopenOnDevOptions {
        pr: "456".to_string(),
        label: "bug".to_string(),
        repo: None,
    };
    let result = options.run_reopen_on_dev();
    assert_eq!(result, 0);
}
