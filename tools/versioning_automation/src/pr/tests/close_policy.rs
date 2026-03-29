//! tools/versioning_automation/src/pr/tests/close_policy.rs
use crate::pr::{commands::PrIssueClosePolicyOptions, issue_close_policy::decide_close_policy};

#[test]
fn close_policy_skips_pr_ref() {
    let out = decide_close_policy(PrIssueClosePolicyOptions {
        action: "Closes".to_string(),
        is_pr_ref: true,
        non_compliance_reason: String::new(),
    });

    assert_eq!(out.kind, "skip_pr_ref");
}

#[test]
fn close_policy_skips_non_compliance() {
    let out = decide_close_policy(PrIssueClosePolicyOptions {
        action: "Closes".to_string(),
        is_pr_ref: false,
        non_compliance_reason: "missing parent".to_string(),
    });

    assert_eq!(out.kind, "skip_non_compliance");
    assert_eq!(out.reason, "missing parent");
}

#[test]
fn close_policy_continues_for_non_closes() {
    let out = decide_close_policy(PrIssueClosePolicyOptions {
        action: "Reopen".to_string(),
        is_pr_ref: true,
        non_compliance_reason: "x".to_string(),
    });

    assert_eq!(out.kind, "continue");
}
