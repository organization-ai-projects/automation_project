//! tools/versioning_automation/src/git/commands/branch_creation_check_options.rs
#[derive(Debug)]
pub(crate) struct BranchCreationCheckOptions {
    pub(crate) command: Option<String>,
    pub(crate) args: Vec<String>,
}
