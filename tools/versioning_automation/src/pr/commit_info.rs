#[derive(Debug, Clone)]
pub(crate) struct CommitInfo {
    pub(crate) short_hash: String,
    pub(crate) subject: String,
    pub(crate) body: String,
}
