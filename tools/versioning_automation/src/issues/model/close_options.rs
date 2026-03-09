//! tools/versioning_automation/src/issues/model/close_options.rs
#[derive(Debug, Clone)]
pub(crate) struct CloseOptions {
    pub(crate) issue: String,
    pub(crate) repo: Option<String>,
    pub(crate) reason: String,
}
