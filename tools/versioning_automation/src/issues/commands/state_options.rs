//! tools/versioning_automation/src/issues/commands/state_options.rs
#[derive(Debug, Clone)]
pub(crate) struct StateOptions {
    pub(crate) issue: String,
    pub(crate) repo: Option<String>,
}
