//! tools/versioning_automation/src/issues/commands/required_fields_validate_options.rs
use crate::issues::commands::RequiredFieldsValidationMode;

#[derive(Debug, Clone)]
pub(crate) struct RequiredFieldsValidateOptions {
    pub(crate) title: String,
    pub(crate) body: String,
    pub(crate) labels_raw: String,
    pub(crate) mode: RequiredFieldsValidationMode,
}
