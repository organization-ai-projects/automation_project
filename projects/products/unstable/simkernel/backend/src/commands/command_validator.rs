use crate::commands::command::Command;
use crate::diagnostics::backend_error::BackendError;

pub struct CommandValidator;

impl CommandValidator {
    pub fn validate(cmd: &Command) -> Result<(), BackendError> {
        if cmd.payload.is_null() {
            return Err(BackendError::InvalidCommand("null payload".to_string()));
        }
        Ok(())
    }
}
