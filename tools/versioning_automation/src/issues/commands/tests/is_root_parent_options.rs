//! tools/versioning_automation/src/issues/commands/tests/is_root_parent_options.rs
use crate::issues::commands::is_root_parent_options::IsRootParentOptions;

#[test]
fn test_run_is_root_parent_success() {
    let options = IsRootParentOptions {
        issue: "123".to_string(),
        repo: Some("test_repo".to_string()),
    };
    let result = options.run_is_root_parent();
    assert_eq!(result, 0);
}
