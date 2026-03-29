//! tools/versioning_automation/src/pr/commands/tests/pr_body_context_options.rs
use crate::pr::commands::PrBodyContextOptions;

#[test]
fn test_run_body_context_valid() {
    let options = PrBodyContextOptions {
        pr_number: "123".to_string(),
        repo: Some("test_repo".to_string()),
    };
    let result = options.run_body_context();
    assert_eq!(result, 0);
}

#[test]
fn test_run_body_context_invalid_repo() {
    let options = PrBodyContextOptions {
        pr_number: "123".to_string(),
        repo: Some("invalid_repo".to_string()),
    };
    let result = options.run_body_context();
    assert_eq!(result, 0);
}
