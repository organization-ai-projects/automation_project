//! tools/versioning_automation/src/issues/commands/list_by_label_options.rs
#[derive(Debug, Clone)]
pub(crate) struct ListByLabelOptions {
    pub(crate) label: String,
    pub(crate) repo: Option<String>,
}
