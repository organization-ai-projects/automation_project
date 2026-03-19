//! tools/versioning_automation/src/issues/commands/neutralize_options.rs

#[derive(Debug, Clone)]
pub(crate) struct NeutralizeOptions {
    pub(crate) pr: String,
    pub(crate) repo: Option<String>,
}
