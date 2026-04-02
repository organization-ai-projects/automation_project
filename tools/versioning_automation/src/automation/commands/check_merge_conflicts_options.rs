#[derive(Debug, Clone)]
pub(crate) struct CheckMergeConflictsOptions {
    pub(crate) remote: String,
    pub(crate) base_branch: String,
}
