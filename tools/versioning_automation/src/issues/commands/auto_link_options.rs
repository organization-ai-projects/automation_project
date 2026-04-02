//! tools/versioning_automation/src/issues/commands/auto_link_options.rs

#[derive(Debug, Clone)]
pub(crate) struct AutoLinkOptions {
    pub(crate) issue: String,
    pub(crate) repo: Option<String>,
}
