use crate::validation_error::ValidationError;
use common::custom_uuid::Id128;
use protocol_macros::EnumMethods;
use serde::{Deserialize, Serialize};

/// Variants of protocol events
#[derive(Debug, Clone, Serialize, Deserialize, EnumMethods)]
pub enum EventVariant {
    /// Represents an acknowledgment event with an ID
    Acknowledged { id: Id128 },

    /// Represents a creation event with associated data
    Created { id: Id128, data: String },

    /// Represents an update event with old and new data
    Updated {
        id: Id128,
        old_data: String,
        new_data: String,
    },

    /// Represents a deletion event with an ID
    Deleted { id: Id128 },

    /// Represents an error event with an ID and message
    Error { id: Id128, message: String },

    /// Default variant for uninitialized or unknown events
    Default,
}

impl EventVariant {
    /// Validates the specific variant of the event
    pub fn validate(&self) -> Result<(), ValidationError> {
        match self {
            EventVariant::Acknowledged { id } => {
                if id.to_hex().trim().is_empty() {
                    return Err(ValidationError::InvalidVariant(
                        "Acknowledged ID is empty".into(),
                    ));
                }
            }
            EventVariant::Created { id, data } => {
                if id.to_hex().trim().is_empty() || data.trim().is_empty() {
                    return Err(ValidationError::InvalidVariant(
                        "Created variant has empty fields".into(),
                    ));
                }
            }
            EventVariant::Updated {
                id,
                old_data,
                new_data,
            } => {
                if id.to_hex().trim().is_empty()
                    || old_data.trim().is_empty()
                    || new_data.trim().is_empty()
                {
                    return Err(ValidationError::InvalidVariant(
                        "Updated variant has empty fields".into(),
                    ));
                }
            }
            EventVariant::Deleted { id } => {
                if id.to_hex().trim().is_empty() {
                    return Err(ValidationError::InvalidVariant(
                        "Deleted ID is empty".into(),
                    ));
                }
            }
            EventVariant::Error { id, message } => {
                if id.to_hex().trim().is_empty() || message.trim().is_empty() {
                    return Err(ValidationError::InvalidVariant(
                        "Error variant has empty fields".into(),
                    ));
                }
            }
            EventVariant::Default => {}
        }
        Ok(())
    }
}
