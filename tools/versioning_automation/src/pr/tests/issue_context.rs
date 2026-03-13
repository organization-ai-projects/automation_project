use crate::pr::commands::pr_issue_context_options::PrIssueContextOptions;
use crate::pr::issue_context::run_issue_context;

#[test]
fn issue_context_command_returns_success_with_unresolvable_issue() {
    let opts = PrIssueContextOptions {
        issue_number: "999999".to_string(),
        repo: Some("owner/repo".to_string()),
    };
    let code = run_issue_context(opts);
    assert_eq!(code, 0);
}
