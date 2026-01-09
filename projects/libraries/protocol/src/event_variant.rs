use crate::generate_enum_methods;
use crate::validation_error::ValidationError;
use serde::{Deserialize, Serialize};

/// Variants of protocol events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventVariant {
    /// Represents an acknowledgment event with an ID
    Acknowledged { id: String },

    /// Represents a creation event with associated data
    Created { id: String, data: String },

    /// Represents an update event with old and new data
    Updated {
        id: String,
        old_data: String,
        new_data: String,
    },

    /// Represents a deletion event with an ID
    Deleted { id: String },

    /// Represents an error event with an ID and message
    Error { id: String, message: String },

    /// Default variant for uninitialized or unknown events
    Default,
}

generate_enum_methods!(EventVariant,
    acknowledged => Acknowledged { id: String },
    created => Created { id: String, data: String },
    updated => Updated { id: String, old_data: String, new_data: String },
    deleted => Deleted { id: String },
    error => Error { id: String, message: String },
    default_variant => Default {},
);

impl EventVariant {
    /// Validates the specific variant of the event
    pub fn validate(&self) -> Result<(), ValidationError> {
        match self {
            EventVariant::Acknowledged { id } => {
                if id.trim().is_empty() {
                    return Err(ValidationError::InvalidVariant(
                        "Acknowledged ID is empty".into(),
                    ));
                }
            }
            EventVariant::Created { id, data } => {
                if id.trim().is_empty() || data.trim().is_empty() {
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
                if id.trim().is_empty() || old_data.trim().is_empty() || new_data.trim().is_empty()
                {
                    return Err(ValidationError::InvalidVariant(
                        "Updated variant has empty fields".into(),
                    ));
                }
            }
            EventVariant::Deleted { id } => {
                if id.trim().is_empty() {
                    return Err(ValidationError::InvalidVariant(
                        "Deleted ID is empty".into(),
                    ));
                }
            }
            EventVariant::Error { id, message } => {
                if id.trim().is_empty() || message.trim().is_empty() {
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
