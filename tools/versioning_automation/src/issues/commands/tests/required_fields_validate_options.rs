//! tools/versioning_automation/src/issues/commands/tests/required_fields_validate_options.rs
use crate::issues::commands::RequiredFieldsValidationMode;
use crate::issues::commands::required_fields_validate_options::RequiredFieldsValidateOptions;

#[test]
fn test_run_required_fields_validate_title() {
    let options = RequiredFieldsValidateOptions {
        title: "Test Title".to_string(),
        body: "Test Body".to_string(),
        labels_raw: "bug".to_string(),
        mode: RequiredFieldsValidationMode::Title,
    };
    let result = options.run_required_fields_validate();
    assert_eq!(result, 0);
}

#[test]
fn test_run_required_fields_validate_body() {
    let options = RequiredFieldsValidateOptions {
        title: "Test Title".to_string(),
        body: "Test Body".to_string(),
        labels_raw: "bug".to_string(),
        mode: RequiredFieldsValidationMode::Body,
    };
    let result = options.run_required_fields_validate();
    assert_eq!(result, 0);
}
