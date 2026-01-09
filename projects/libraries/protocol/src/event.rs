// projects/libraries/protocol/src/event.rs
use crate::log_level::LogLevel;
use crate::metadata::Metadata;
use crate::validation_error::ValidationError;
use crate::{EventVariant, Payload, event_type::EventType};
use serde::{Deserialize, Serialize};

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

    /// Generic payload encapsulated in a `Payload` struct
    pub payload: Option<Payload>,

    // Common fields for log/progress (optional depending on event_type)
    pub level: Option<LogLevel>,
    pub message: Option<String>,
    pub pct: Option<u8>,

    /// The specific variant of the event
    pub variant: EventVariant,
}

type CommonFields = (
    Option<Payload>,
    Option<LogLevel>,
    Option<String>,
    Option<u8>,
    EventVariant,
);

impl Event {
    /// Common initialization for all constructors
    fn initialize_common_fields() -> CommonFields {
        (None, None, None, None, EventVariant::Default)
    }

    /// Creates a new event with the current timestamp
    pub fn new(name: String, event_type: EventType, data: String) -> Self {
        Self::create_event(
            name,
            event_type,
            data,
            Metadata::now(),
            EventVariant::Default,
        )
    }

    /// Helper method to create an event with customizable metadata and variant
    fn create_event(
        name: String,
        event_type: EventType,
        data: String,
        metadata: Metadata,
        variant: EventVariant,
    ) -> Self {
        let (payload, level, message, pct, _) = Self::initialize_common_fields();
        Self {
            name,
            event_type,
            data,
            metadata,
            payload,
            level,
            message,
            pct,
            variant,
        }
    }

    /// Creates a new event with custom metadata
    pub fn with_metadata(
        name: String,
        event_type: EventType,
        data: String,
        metadata: Metadata,
    ) -> Self {
        Self::create_event(name, event_type, data, metadata, EventVariant::Default)
    }

    /// Creates a new event with a specific variant
    pub fn with_variant(
        name: String,
        event_type: EventType,
        data: String,
        variant: EventVariant,
    ) -> Self {
        Self::create_event(name, event_type, data, Metadata::now(), variant)
    }

    /// Creates a new event with a `Payload`
    pub fn with_payload(
        name: String,
        event_type: EventType,
        metadata: Metadata,
        payload: Payload,
    ) -> Self {
        Self {
            name,
            event_type,
            data: payload
                .payload
                .clone()
                .map_or_else(|| "".to_string(), |v| v.to_string()),
            metadata,
            payload: Some(payload),
            level: None,
            message: None,
            pct: None,
            variant: EventVariant::Default,
        }
    }

    /// Updates the payload of an existing event
    pub fn update_payload(&mut self, payload: Payload) {
        self.payload = Some(payload);
    }

    /// Extracts the payload as a `Payload` struct
    pub fn extract_payload(&self) -> Option<Payload> {
        self.payload.clone()
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
        if !self
            .name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.')
        {
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

        // Validate variant-specific rules
        self.variant.validate()
    }
}
