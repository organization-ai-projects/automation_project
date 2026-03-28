//! tools/versioning_automation/src/git/commands/add_commit_push_options.rs
#[derive(Debug)]
pub(crate) struct AddCommitPushOptions {
    pub(crate) message: String,
    pub(crate) no_verify: bool,
    pub(crate) remote: String,
}
