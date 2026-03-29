//! tools/versioning_automation/src/issues/commands/tests/parent_guard_options.rs
use crate::issues::commands::parent_guard_options::ParentGuardOptions;

#[test]
fn test_run_parent_guard_with_issue() {
    let options = ParentGuardOptions {
        issue: Some("123".to_string()),
        child: None,
        strict_guard: true,
    };
    let result = options.run_parent_guard();
    assert_eq!(result, 0);
}

#[test]
fn test_run_parent_guard_with_child() {
    let options = ParentGuardOptions {
        issue: None,
        child: Some("456".to_string()),
        strict_guard: false,
    };
    let result = options.run_parent_guard();
    assert_eq!(result, 0);
}
