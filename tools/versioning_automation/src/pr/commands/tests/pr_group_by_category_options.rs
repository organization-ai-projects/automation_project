//! tools/versioning_automation/src/pr/commands/tests/pr_group_by_category_options.rs
use crate::pr::commands::pr_group_by_category_options::PrGroupByCategoryOptions;

#[test]
fn test_run_group_by_category_resolved() {
    let options = PrGroupByCategoryOptions {
        text: "123|Bug Fixes|fix|#123".to_string(),
        mode: "resolved".to_string(),
    };
    let result = options.run_group_by_category();
    assert_eq!(result, 0);
}

#[test]
fn test_run_group_by_category_invalid_mode() {
    let options = PrGroupByCategoryOptions {
        text: "123|Bug Fixes|fix|#123".to_string(),
        mode: "invalid".to_string(),
    };
    let result = options.run_group_by_category();
    assert_eq!(result, 2);
}
