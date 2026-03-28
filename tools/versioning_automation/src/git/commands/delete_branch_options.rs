//! tools/versioning_automation/src/git/commands/delete_branch_options.rs
#[derive(Debug)]
pub(crate) struct DeleteBranchOptions {
    pub(crate) branch_name: String,
    pub(crate) force: bool,
    pub(crate) remote: String,
    pub(crate) base_branch: String,
}
