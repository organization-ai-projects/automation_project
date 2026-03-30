//! tools/versioning_automation/src/pr/commands/pr_issue_decision_options.rs
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrIssueDecisionOptions {
    pub(crate) action: String,
    pub(crate) issue: String,
    pub(crate) default_category: String,
    pub(crate) seen_reopen: bool,
    pub(crate) reopen_category: String,
    pub(crate) inferred_decision: String,
    pub(crate) explicit_decision: String,
    pub(crate) allow_inferred: bool,
}
