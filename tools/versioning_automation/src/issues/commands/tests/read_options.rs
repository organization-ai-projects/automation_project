//! tools/versioning_automation/src/issues/commands/tests/read_options.rs
use crate::issues::commands::read_options::ReadOptions;

#[test]
fn test_run_read_with_issue() {
    let options = ReadOptions {
        issue: Some("123".to_string()),
        repo: Some("test_repo".to_string()),
        json: None,
        jq: None,
        template: None,
    };
    let result = options.run_read();
    assert_eq!(result, 0);
}

#[test]
fn test_run_read_without_issue() {
    let options = ReadOptions {
        issue: None,
        repo: Some("test_repo".to_string()),
        json: Some("number".to_string()),
        jq: Some(".[]".to_string()),
        template: None,
    };
    let result = options.run_read();
    assert_eq!(result, 0);
}
