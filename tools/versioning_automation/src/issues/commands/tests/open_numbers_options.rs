//! tools/versioning_automation/src/issues/commands/tests/open_numbers_options.rs
use crate::issues::commands::open_numbers_options::OpenNumbersOptions;

#[test]
fn test_run_open_numbers_with_repo() {
    let options = OpenNumbersOptions {
        repo: Some("test_repo".to_string()),
    };
    let result = options.run_open_numbers();
    assert_eq!(result, 0);
}

#[test]
fn test_run_open_numbers_without_repo() {
    let options = OpenNumbersOptions { repo: None };
    let result = options.run_open_numbers();
    assert_eq!(result, 0);
}
