#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrUpsertCommentOptions {
    pub(crate) pr_number: String,
    pub(crate) repo: Option<String>,
    pub(crate) marker: String,
    pub(crate) body: String,
}
