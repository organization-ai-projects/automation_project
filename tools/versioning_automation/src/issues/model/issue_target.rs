//! tools/versioning_automation/src/issues/model/issue_target.rs
#[derive(Debug, Clone)]
pub(crate) struct IssueTarget {
    pub(crate) issue: String,
    pub(crate) repo: Option<String>,
}
