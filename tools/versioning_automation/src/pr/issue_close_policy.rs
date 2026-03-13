use crate::pr::commands::pr_issue_close_policy_options::PrIssueClosePolicyOptions;

pub(crate) fn run_issue_close_policy(opts: PrIssueClosePolicyOptions) -> i32 {
    let outcome = decide_close_policy(opts);
    println!("POLICY|{}|{}", outcome.kind, outcome.reason);
    0
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IssueClosePolicy {
    pub(crate) kind: String,
    pub(crate) reason: String,
}

pub(crate) fn decide_close_policy(opts: PrIssueClosePolicyOptions) -> IssueClosePolicy {
    if opts.action != "Closes" {
        return IssueClosePolicy {
            kind: "continue".to_string(),
            reason: String::new(),
        };
    }

    if opts.is_pr_ref {
        return IssueClosePolicy {
            kind: "skip_pr_ref".to_string(),
            reason: String::new(),
        };
    }

    if !opts.non_compliance_reason.trim().is_empty() {
        return IssueClosePolicy {
            kind: "skip_non_compliance".to_string(),
            reason: opts.non_compliance_reason,
        };
    }

    IssueClosePolicy {
        kind: "continue".to_string(),
        reason: String::new(),
    }
}
