//! tools/versioning_automation/src/pr/commands/tests/pr_effective_category_options.rs
use crate::pr::commands::PrEffectiveCategoryOptions;

#[test]
fn test_run_effective_category_valid() {
    let options = PrEffectiveCategoryOptions {
        labels_raw: "bug||enhancement".to_string(),
        title: Some("Fix critical bug".to_string()),
        title_category: None,
        default_category: "Unknown".to_string(),
    };
    let result = options.run_effective_category();
    assert_eq!(result, 0);
}

#[test]
fn test_run_effective_category_no_labels() {
    let options = PrEffectiveCategoryOptions {
        labels_raw: "".to_string(),
        title: Some("Add new feature".to_string()),
        title_category: None,
        default_category: "Unknown".to_string(),
    };
    let result = options.run_effective_category();
    assert_eq!(result, 0);
}
