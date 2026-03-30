//! tools/versioning_automation/src/pr/commands/pr_issue_context_options.rs
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrIssueRefKindOptions {
    pub(crate) issue_number: String,
    pub(crate) repo: Option<String>,
}
