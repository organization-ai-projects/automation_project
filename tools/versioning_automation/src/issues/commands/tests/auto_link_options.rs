//! tools/versioning_automation/src/issues/commands/tests/auto_link_options.rs
use crate::issues::commands::auto_link_options::AutoLinkOptions;

#[test]
fn test_run_auto_link_success() {
    let options = AutoLinkOptions {
        issue: "123".to_string(),
        repo: Some("test_repo".to_string()),
    };
    let result = options.run_auto_link();
    assert_eq!(result, 0);
}

#[test]
fn test_run_auto_link_missing_repo() {
    let options = AutoLinkOptions {
        issue: "123".to_string(),
        repo: None,
    };
    let result = options.run_auto_link();
    assert_ne!(result, 0);
}
