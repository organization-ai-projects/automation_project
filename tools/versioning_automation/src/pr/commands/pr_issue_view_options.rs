#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrIssueViewOptions {
    pub(crate) issue_number: String,
    pub(crate) repo: Option<String>,
}
