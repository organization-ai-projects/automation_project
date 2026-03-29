//! tools/versioning_automation/src/pr/commands/tests/pr_breaking_detect_options.rs
use crate::pr::commands::pr_breaking_detect_options::PrBreakingDetectOptions;

#[test]
fn test_run_breaking_detect_with_labels() {
    let options = PrBreakingDetectOptions {
        text: "".to_string(),
        labels_raw: Some("breaking-change".to_string()),
    };
    let result = options.run_breaking_detect();
    assert_eq!(result, 0);
}

#[test]
fn test_run_breaking_detect_with_text() {
    let options = PrBreakingDetectOptions {
        text: "This introduces a breaking change.".to_string(),
        labels_raw: None,
    };
    let result = options.run_breaking_detect();
    assert_eq!(result, 0);
}

#[test]
fn test_run_breaking_detect_no_breaking() {
    let options = PrBreakingDetectOptions {
        text: "No breaking changes here.".to_string(),
        labels_raw: None,
    };
    let result = options.run_breaking_detect();
    assert_eq!(result, 0);
}
