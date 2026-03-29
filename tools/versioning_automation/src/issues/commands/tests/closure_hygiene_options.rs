//! tools/versioning_automation/src/issues/commands/tests/closure_hygiene_options.rs
use crate::issues::commands::closure_hygiene_options::ClosureHygieneOptions;

#[test]
fn test_run_closure_hygiene_success() {
    let options = ClosureHygieneOptions {
        repo: Some("test_repo".to_string()),
    };
    let result = options.run_closure_hygiene();
    assert_eq!(result, 0);
}

#[test]
fn test_run_closure_hygiene_missing_repo() {
    let options = ClosureHygieneOptions { repo: None };
    let result = options.run_closure_hygiene();
    assert_ne!(result, 0);
}
