//! tools/versioning_automation/src/pr/commands/tests/pr_non_closing_refs_options.rs
use crate::pr::commands::PrNonClosingRefsOptions;

#[test]
fn test_run_non_closing_refs_with_part_of() {
    let options = PrNonClosingRefsOptions {
        text: "Part of|123\nPart of|456".to_string(),
    };
    let result = options.run_non_closing_refs();
    assert_eq!(result, 0);
}

#[test]
fn test_run_non_closing_refs_without_part_of() {
    let options = PrNonClosingRefsOptions {
        text: "Related to|123\nRelated to|456".to_string(),
    };
    let result = options.run_non_closing_refs();
    assert_eq!(result, 0);
}
