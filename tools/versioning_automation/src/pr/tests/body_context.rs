#[test]
fn run_body_context_returns_success_when_pr_fetch_fails() {
    let code = crate::pr::body_context::run_body_context(
        crate::pr::commands::pr_body_context_options::PrBodyContextOptions {
            pr_number: "999999".to_string(),
            repo: Some("owner/repo".to_string()),
        },
    );
    assert_eq!(code, 0);
}
