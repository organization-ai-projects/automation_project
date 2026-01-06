// projects/libraries/protocol/src/command.rs
use serde::{Deserialize, Serialize};
use crate::metadata::Metadata;
use crate::error::ValidationError;
use crate::command_type::CommandType;

/// Maximum allowed length for command names (in characters)
pub const MAX_COMMAND_NAME_LENGTH: usize = 256;
/// Maximum allowed size for command payloads (in bytes)
pub const MAX_COMMAND_PAYLOAD_SIZE: usize = 10 * 1024 * 1024; // 10 MB

/// Represents a command in the protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    /// The name/identifier of the command
    pub name: String,
    /// The type of command
    pub command_type: CommandType,
    /// The payload data for the command
    pub payload: String,
    /// Associated metadata
    pub metadata: Metadata,
}

impl Command {
    /// Creates a new command with the current timestamp
    pub fn new(name: String, command_type: CommandType, payload: String) -> Self {
        Self {
            name,
            command_type,
            payload,
            metadata: Metadata::now(),
        }
    }

    /// Creates a new command with custom metadata
    pub fn with_metadata(name: String, command_type: CommandType, payload: String, metadata: Metadata) -> Self {
        Self {
            name,
            command_type,
            payload,
            metadata,
        }
    }

    /// Validates the command structure and content
    ///
    /// Returns `Ok(())` if valid, or a descriptive error otherwise
    pub fn validate(&self) -> Result<(), ValidationError> {
        // Check name is not empty
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName);
        }

        // Check name length
        if self.name.len() > MAX_COMMAND_NAME_LENGTH {
            return Err(ValidationError::NameTooLong {
                length: self.name.len(),
                max: MAX_COMMAND_NAME_LENGTH,
            });
        }

        // Check name format (alphanumeric, underscore, hyphen, dot)
        if !self.name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.') {
            return Err(ValidationError::InvalidNameFormat(self.name.clone()));
        }

        // Check payload is not empty
        if self.payload.trim().is_empty() {
            return Err(ValidationError::EmptyPayload);
        }

        // Check payload size
        if self.payload.len() > MAX_COMMAND_PAYLOAD_SIZE {
            return Err(ValidationError::PayloadTooLarge {
                size: self.payload.len(),
                max: MAX_COMMAND_PAYLOAD_SIZE,
            });
        }

        // Validate metadata
        self.metadata.validate()?;

        Ok(())
    }
}
