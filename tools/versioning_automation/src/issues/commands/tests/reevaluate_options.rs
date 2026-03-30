//! tools/versioning_automation/src/issues/commands/tests/reevaluate_options.rs
use crate::issues::commands::reevaluate_options::ReevaluateOptions;

#[test]
fn test_run_reevaluate_with_repo() {
    let options = ReevaluateOptions {
        issue: "123".to_string(),
        repo: Some("test_repo".to_string()),
    };
    let result = options.run_reevaluate();
    assert_eq!(result, 0);
}
