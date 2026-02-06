//! projects/products/varina/backend/src/autopilot/autopilot_error.rs
use std::fmt;

use command_runner::CommandError;

/// Structure representing an autopilot-specific error.
/// Allows converting external errors (like `CommandError`) into internal errors.
#[derive(Debug)]
pub struct AutopilotError {
    pub message: String,
}

impl fmt::Display for AutopilotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(f)
    }
}

impl From<String> for AutopilotError {
    fn from(message: String) -> Self {
        Self { message }
    }
}

impl From<AutopilotError> for String {
    fn from(error: AutopilotError) -> Self {
        format!("AutopilotError: {}", error).to_string()
    }
}

impl From<CommandError> for AutopilotError {
    fn from(error: CommandError) -> Self {
        Self {
            message: error.to_string(),
        }
    }
}
