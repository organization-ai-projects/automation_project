//! tools/versioning_automation/src/issues/commands/label_exists_options.rs

#[derive(Debug, Clone)]
pub(crate) struct LabelExistsOptions {
    pub(crate) repo: String,
    pub(crate) label: String,
}
