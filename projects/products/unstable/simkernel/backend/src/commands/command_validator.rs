#![allow(dead_code)]
use crate::commands::command::Command;
use crate::diagnostics::error::SimError;

pub struct CommandValidator;

impl CommandValidator {
    pub fn validate(cmd: &Command) -> Result<(), SimError> {
        if cmd.payload.is_null() {
            return Err(SimError::InvalidCommand("null payload".to_string()));
        }
        Ok(())
    }
}
