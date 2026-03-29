//! tools/versioning_automation/src/issues/commands/tests/list_by_label_options.rs
use crate::issues::commands::list_by_label_options::ListByLabelOptions;

#[test]
fn test_run_list_by_label_with_repo() {
    let options = ListByLabelOptions {
        label: "bug".to_string(),
        repo: Some("test_repo".to_string()),
    };
    let result = options.run_list_by_label();
    assert_eq!(result, 0);
}

#[test]
fn test_run_list_by_label_without_repo() {
    let options = ListByLabelOptions {
        label: "bug".to_string(),
        repo: None,
    };
    let result = options.run_list_by_label();
    assert_eq!(result, 0);
}
