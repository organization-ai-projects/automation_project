#[test]
fn refresh_validation_returns_non_zero_when_pr_snapshot_cannot_be_fetched() {
    let code = crate::pr::refresh_validation::run_refresh_validation(
        crate::pr::commands::pr_refresh_validation_options::PrRefreshValidationOptions {
            pr_number: "999999".to_string(),
            repo: Some("owner/repo".to_string()),
        },
    );
    assert_ne!(code, 0);
}
