//! tools/versioning_automation/src/issues/commands/tests/validate_footer_options.rs
use crate::issues::commands::validate_footer_options::ValidateFooterOptions;

#[test]
fn test_run_validate_footer_with_repo() {
    let options = ValidateFooterOptions {
        file: "test_commit_message.txt".to_string(),
        repo: Some("test_repo".to_string()),
    };
    let result = options.run_validate_footer();
    assert_eq!(result, 0);
}

#[test]
fn test_run_validate_footer_without_repo() {
    let options = ValidateFooterOptions {
        file: "test_commit_message.txt".to_string(),
        repo: None,
    };
    let result = options.run_validate_footer();
    assert_eq!(result, 0);
}
