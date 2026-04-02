#[derive(Debug)]
pub(crate) struct CreateBranchOptions {
    pub(crate) branch_name: Option<String>,
    pub(crate) remote: String,
    pub(crate) base_branch: String,
}
