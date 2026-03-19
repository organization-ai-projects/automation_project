use crate::pr::commands::pr_normalize_issue_key_options::PrNormalizeIssueKeyOptions;
use crate::pr::normalize_issue_key::run_normalize_issue_key;

#[test]
fn normalize_issue_key_command_runs_with_valid_ref() {
    let opts = PrNormalizeIssueKeyOptions {
        raw: "prefix #42 suffix".to_string(),
    };
    let code = run_normalize_issue_key(opts);
    assert_eq!(code, 0);
}

#[test]
fn normalize_issue_key_command_fails_without_ref() {
    let opts = PrNormalizeIssueKeyOptions {
        raw: "no issue key".to_string(),
    };
    let code = run_normalize_issue_key(opts);
    assert_eq!(code, 1);
}
