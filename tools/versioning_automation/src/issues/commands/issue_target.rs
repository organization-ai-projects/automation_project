//! tools/versioning_automation/src/issues/commands/issue_target.rs
#[derive(Debug, Clone)]
pub(crate) struct IssueTarget {
    pub(crate) issue: String,
    pub(crate) repo: Option<String>,
}
