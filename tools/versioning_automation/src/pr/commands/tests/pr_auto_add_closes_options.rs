//! tools/versioning_automation/src/pr/commands/tests/pr_auto_add_closes_options.rs
use crate::pr::commands::PrAutoAddClosesOptions;

#[test]
fn test_run_auto_add_closes_invalid_repo() {
    let options = PrAutoAddClosesOptions {
        pr_number: "123".to_string(),
        repo: Some("invalid_repo".to_string()),
    };
    let result = options.run_auto_add_closes();
    assert_eq!(result, 3);
}
