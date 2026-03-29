//! tools/versioning_automation/src/issues/commands/tests/update_options.rs
use crate::issues::commands::update_options::UpdateOptions;

#[test]
fn test_run_update_with_repo() {
    let options = UpdateOptions {
        issue: "123".to_string(),
        repo: Some("test_repo".to_string()),
        edit_args: vec![("title".to_string(), "Updated Title".to_string())],
    };
    let result = options.run_update();
    assert_eq!(result, 0);
}

#[test]
fn test_run_update_without_repo() {
    let options = UpdateOptions {
        issue: "123".to_string(),
        repo: None,
        edit_args: vec![("title".to_string(), "Updated Title".to_string())],
    };
    let result = options.run_update();
    assert_eq!(result, 0);
}
