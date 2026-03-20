//! tools/versioning_automation/src/automation/commands/prepare_commit_msg_options.rs
#[derive(Debug)]
pub(crate) struct PrepareCommitMsgOptions {
    pub(crate) file: String,
    pub(crate) source: Option<String>,
}
