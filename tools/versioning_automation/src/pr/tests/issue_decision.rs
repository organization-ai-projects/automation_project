//! tools/versioning_automation/src/pr/tests/issue_decision.rs
use crate::pr::{commands::PrIssueDecisionOptions, issue_decision::decide};

fn base_opts() -> PrIssueDecisionOptions {
    PrIssueDecisionOptions {
        action: "Closes".to_string(),
        issue: "#1".to_string(),
        default_category: "Mixed".to_string(),
        seen_reopen: false,
        reopen_category: String::new(),
        inferred_decision: String::new(),
        explicit_decision: String::new(),
        allow_inferred: true,
    }
}

#[test]
fn resolve_reopen_when_close_hits_seen_reopen() {
    let mut opts = base_opts();
    opts.seen_reopen = true;
    opts.reopen_category = "UI".to_string();
    let out = decide(opts);
    assert_eq!(out.kind, "resolve_reopen");
    assert_eq!(out.category, "UI");
    assert!(out.force_category);
}

#[test]
fn conflict_when_allow_inferred_and_no_inferred_decision() {
    let opts = base_opts();
    let out = decide(opts);
    assert_eq!(out.kind, "conflict");
}

#[test]
fn ignore_reopen_when_effective_close() {
    let mut opts = base_opts();
    opts.action = "Reopen".to_string();
    opts.explicit_decision = "close".to_string();
    opts.allow_inferred = false;
    let out = decide(opts);
    assert_eq!(out.kind, "ignore");
}

#[test]
fn resolve_reopen_when_effective_reopen() {
    let mut opts = base_opts();
    opts.allow_inferred = false;
    opts.inferred_decision = "reopen".to_string();
    let out = decide(opts);
    assert_eq!(out.kind, "resolve_reopen");
    assert_eq!(out.category, "Mixed");
}

#[test]
fn continue_when_no_directive_override() {
    let mut opts = base_opts();
    opts.allow_inferred = false;
    let out = decide(opts);
    assert_eq!(out.kind, "continue");
}

#[test]
fn cancel_closes_returns_cancel_close_decision() {
    let mut opts = base_opts();
    opts.action = "Cancel-Closes".to_string();
    let out = decide(opts);
    assert_eq!(out.kind, "cancel_close");
    assert_eq!(out.final_action, "cancel_close");
}
