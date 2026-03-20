//! tools/versioning_automation/src/issues/commands/validate_footer_options.rs
#[derive(Debug, Clone)]
pub(crate) struct ValidateFooterOptions {
    pub(crate) file: String,
    pub(crate) repo: Option<String>,
}
