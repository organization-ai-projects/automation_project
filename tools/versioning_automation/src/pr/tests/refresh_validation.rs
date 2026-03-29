//! tools/versioning_automation/src/pr/tests/refresh_validation.rs
use crate::pr::{commands::PrRefreshValidationOptions, refresh_validation};

#[test]
fn refresh_validation_returns_non_zero_when_pr_snapshot_cannot_be_fetched() {
    let code = refresh_validation::run_refresh_validation(PrRefreshValidationOptions {
        pr_number: "999999".to_string(),
        repo: Some("owner/repo".to_string()),
    });
    assert_ne!(code, 0);
}
