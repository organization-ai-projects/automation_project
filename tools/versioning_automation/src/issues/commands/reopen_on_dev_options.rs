//! tools/versioning_automation/src/issues/commands/reopen_on_dev_options.rs
#[derive(Debug, Clone)]
pub(crate) struct ReopenOnDevOptions {
    pub(crate) pr: String,
    pub(crate) label: String,
    pub(crate) repo: Option<String>,
}
