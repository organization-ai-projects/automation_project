#[derive(Debug)]
pub(crate) struct CleanupAfterPrOptions {
    pub(crate) delete_only: bool,
    pub(crate) remote: String,
    pub(crate) base_branch: String,
}
