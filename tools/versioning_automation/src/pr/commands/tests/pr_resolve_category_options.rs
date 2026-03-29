//! tools/versioning_automation/src/pr/commands/tests/pr_resolve_category_options.rs
use crate::pr::commands::PrResolveCategoryOptions;

#[test]
fn test_run_resolve_category_valid() {
    let options = PrResolveCategoryOptions {
        label_category: "bug".to_string(),
        title_category: "enhancement".to_string(),
        default_category: "Unknown".to_string(),
    };
    let result = options.run_resolve_category();
    assert_eq!(result, 0);
}

#[test]
fn test_run_resolve_category_empty_labels() {
    let options = PrResolveCategoryOptions {
        label_category: "".to_string(),
        title_category: "feature".to_string(),
        default_category: "Unknown".to_string(),
    };
    let result = options.run_resolve_category();
    assert_eq!(result, 0);
}
