//! tools/versioning_automation/src/pr/commands/tests/pr_closure_marker_options.rs
use crate::pr::commands::pr_closure_marker_options::PrClosureMarkerOptions;

#[test]
fn test_run_closure_marker_apply() {
    let options = PrClosureMarkerOptions {
        text: "Some text".to_string(),
        keyword_pattern: "fixes".to_string(),
        issue: "#123".to_string(),
        mode: "apply".to_string(),
    };
    let result = options.run_closure_marker();
    assert_eq!(result, 0);
}

#[test]
fn test_run_closure_marker_remove() {
    let options = PrClosureMarkerOptions {
        text: "Some text".to_string(),
        keyword_pattern: "fixes".to_string(),
        issue: "#123".to_string(),
        mode: "remove".to_string(),
    };
    let result = options.run_closure_marker();
    assert_eq!(result, 0);
}

#[test]
fn test_run_closure_marker_invalid_mode() {
    let options = PrClosureMarkerOptions {
        text: "Some text".to_string(),
        keyword_pattern: "fixes".to_string(),
        issue: "#123".to_string(),
        mode: "invalid".to_string(),
    };
    let result = options.run_closure_marker();
    assert_eq!(result, 2);
}
