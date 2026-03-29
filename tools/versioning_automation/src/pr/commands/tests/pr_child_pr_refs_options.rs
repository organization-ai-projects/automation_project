//! tools/versioning_automation/src/pr/commands/tests/pr_child_pr_refs_options.rs
use crate::pr::commands::pr_child_pr_refs_options::PrChildPrRefsOptions;

#[test]
fn test_run_child_pr_refs_valid() {
    let options = PrChildPrRefsOptions {
        pr_number: "123".to_string(),
        repo: Some("test_repo".to_string()),
    };
    let result = options.run_child_pr_refs();
    assert_eq!(result, 0);
}

#[test]
fn test_run_child_pr_refs_invalid_repo() {
    let options = PrChildPrRefsOptions {
        pr_number: "123".to_string(),
        repo: Some("invalid_repo".to_string()),
    };
    let result = options.run_child_pr_refs();
    assert_eq!(result, 0);
}
