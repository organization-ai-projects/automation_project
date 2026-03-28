//! tools/versioning_automation/src/git/commands/create_after_delete_options.rs
#[derive(Debug)]
pub(crate) struct CreateAfterDeleteOptions {
    pub(crate) remote: String,
    pub(crate) base_branch: String,
}
