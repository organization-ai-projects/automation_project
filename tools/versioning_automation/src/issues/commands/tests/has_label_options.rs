//! tools/versioning_automation/src/issues/commands/tests/has_label_options.rs
use crate::issues::commands::has_label_options::HasLabelOptions;

#[test]
fn test_run_has_label_success() {
    let options = HasLabelOptions {
        issue: "123".to_string(),
        label: "bug".to_string(),
        repo: Some("test_repo".to_string()),
    };
    let result = options.run_has_label();
    assert_eq!(result, 0);
}

#[test]
fn test_run_has_label_missing_repo() {
    let options = HasLabelOptions {
        issue: "123".to_string(),
        label: "bug".to_string(),
        repo: None,
    };
    let result = options.run_has_label();
    assert_eq!(result, 0);
}
