//! tools/versioning_automation/src/issues/commands/tests/label_exists_options.rs
use crate::issues::commands::label_exists_options::LabelExistsOptions;

#[test]
fn test_run_label_exists_true() {
    let options = LabelExistsOptions {
        repo: "test_repo".to_string(),
        label: "bug".to_string(),
    };
    let result = options.run_label_exists();
    assert_eq!(result, 0);
}

#[test]
fn test_run_label_exists_false() {
    let options = LabelExistsOptions {
        repo: "test_repo".to_string(),
        label: "nonexistent".to_string(),
    };
    let result = options.run_label_exists();
    assert_eq!(result, 0);
}
