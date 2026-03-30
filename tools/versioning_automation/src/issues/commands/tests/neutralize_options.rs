//! tools/versioning_automation/src/issues/commands/tests/neutralize_options.rs
use crate::issues::commands::neutralize_options::NeutralizeOptions;

#[test]
fn test_run_neutralize_missing_repo() {
    let options = NeutralizeOptions {
        pr: "123".to_string(),
        repo: None,
    };
    let result = options.run_neutralize();
    assert_ne!(result, 0);
}
