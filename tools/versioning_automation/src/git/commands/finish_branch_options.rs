//! tools/versioning_automation/src/git/commands/finish_branch_options.rs
#[derive(Debug)]
pub(crate) struct FinishBranchOptions {
    pub(crate) branch_name: Option<String>,
    pub(crate) remote: String,
    pub(crate) base_branch: String,
}
