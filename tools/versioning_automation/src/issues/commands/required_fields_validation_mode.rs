//! tools/versioning_automation/src/issues/commands/required_fields_validate_options.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RequiredFieldsValidationMode {
    Title,
    Body,
    Content,
}
