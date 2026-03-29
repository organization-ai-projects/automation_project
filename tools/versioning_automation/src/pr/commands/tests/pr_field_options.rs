//! tools/versioning_automation/src/pr/commands/tests/pr_field_options.rs
use crate::pr::commands::{PrFieldName, PrFieldOptions};

#[test]
fn test_run_field_commit_messages() {
    let options = PrFieldOptions {
        pr_number: "123".to_string(),
        repo: Some("test_repo".to_string()),
        name: PrFieldName::CommitMessages,
    };
    let result = options.run_field();
    assert_eq!(result, 0);
}

#[test]
fn test_run_field_title() {
    let options = PrFieldOptions {
        pr_number: "123".to_string(),
        repo: Some("test_repo".to_string()),
        name: PrFieldName::Title,
    };
    let result = options.run_field();
    assert_eq!(result, 0);
}
