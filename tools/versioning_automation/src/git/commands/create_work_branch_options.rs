#[derive(Debug)]
pub(crate) struct CreateWorkBranchOptions {
    pub(crate) branch_type: String,
    pub(crate) description: String,
    pub(crate) remote: String,
    pub(crate) base_branch: String,
}
