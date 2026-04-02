//! tools/versioning_automation/src/issues/commands/has_label_options.rs
#[derive(Debug, Clone)]
pub(crate) struct HasLabelOptions {
    pub(crate) issue: String,
    pub(crate) label: String,
    pub(crate) repo: Option<String>,
}
