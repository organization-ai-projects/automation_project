// projects/libraries/protocol/src/event.rs
use serde::{Deserialize, Serialize};
use crate::metadata::Metadata;
use crate::error::ValidationError;
use crate::event_type::EventType;

/// Maximum allowed length for event names (in characters)
pub const MAX_EVENT_NAME_LENGTH: usize = 256;
/// Maximum allowed size for event data (in bytes)
pub const MAX_EVENT_DATA_SIZE: usize = 10 * 1024 * 1024; // 10 MB

/// Represents an event in the protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// The name/identifier of the event
    pub name: String,
    /// The type of event
    pub event_type: EventType,
    /// The event data payload
    pub data: String,
    /// Associated metadata
    pub metadata: Metadata,
}

impl Event {
    /// Creates a new event with the current timestamp
    pub fn new(name: String, event_type: EventType, data: String) -> Self {
        Self {
            name,
            event_type,
            data,
            metadata: Metadata::now(),
        }
    }

    /// Creates a new event with custom metadata
    pub fn with_metadata(name: String, event_type: EventType, data: String, metadata: Metadata) -> Self {
        Self {
            name,
            event_type,
            data,
            metadata,
        }
    }

    /// Validates the event structure and content
    ///
    /// Returns `Ok(())` if valid, or a descriptive error otherwise
    pub fn validate(&self) -> Result<(), ValidationError> {
        // Check name is not empty
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName);
        }

        // Check name length
        if self.name.len() > MAX_EVENT_NAME_LENGTH {
            return Err(ValidationError::NameTooLong {
                length: self.name.len(),
                max: MAX_EVENT_NAME_LENGTH,
            });
        }

        // Check name format (alphanumeric, underscore, hyphen, dot)
        if !self.name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.') {
            return Err(ValidationError::InvalidNameFormat(self.name.clone()));
        }

        // Check data is not empty
        if self.data.trim().is_empty() {
            return Err(ValidationError::EmptyPayload);
        }

        // Check data size
        if self.data.len() > MAX_EVENT_DATA_SIZE {
            return Err(ValidationError::PayloadTooLarge {
                size: self.data.len(),
                max: MAX_EVENT_DATA_SIZE,
            });
        }

        // Validate metadata
        self.metadata.validate()?;

        Ok(())
    }
}
