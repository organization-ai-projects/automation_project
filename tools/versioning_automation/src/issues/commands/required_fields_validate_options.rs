//! tools/versioning_automation/src/issues/commands/required_fields_validate_options.rs
use crate::issues::{Validation, commands::RequiredFieldsValidationMode};

#[derive(Debug, Clone)]
pub(crate) struct RequiredFieldsValidateOptions {
    pub(crate) title: String,
    pub(crate) body: String,
    pub(crate) labels_raw: String,
    pub(crate) mode: RequiredFieldsValidationMode,
}

impl RequiredFieldsValidateOptions {
    pub(crate) fn run_required_fields_validate(self) -> i32 {
        let result = match self.mode {
            RequiredFieldsValidationMode::Title => {
                Validation::validate_title(&self.title, &self.labels_raw)
            }
            RequiredFieldsValidationMode::Body => {
                Validation::validate_body(&self.body, &self.labels_raw)
            }
            RequiredFieldsValidationMode::Content => {
                Validation::validate_content(&self.title, &self.body, &self.labels_raw)
            }
        };

        match result {
            Ok(entries) => {
                for entry in entries {
                    println!("{}", entry.as_pipe_line());
                }
                0
            }
            Err(message) => {
                eprintln!("{message}");
                1
            }
        }
    }
}
