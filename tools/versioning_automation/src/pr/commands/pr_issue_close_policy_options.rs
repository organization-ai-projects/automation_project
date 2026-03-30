//! tools/versioning_automation/src/pr/commands/pr_issue_close_policy_options.rs
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrIssueClosePolicyOptions {
    pub(crate) action: String,
    pub(crate) is_pr_ref: bool,
    pub(crate) non_compliance_reason: String,
}
