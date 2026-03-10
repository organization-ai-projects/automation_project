//! tools/versioning_automation/src/issues/commands/reevaluate_options.rs
#[derive(Debug, Clone)]
pub(crate) struct ReevaluateOptions {
    pub(crate) issue: String,
    pub(crate) repo: Option<String>,
}
