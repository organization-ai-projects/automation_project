// projects/libraries/protocol/src/lib.rs
//! Protocol library for command and event-based communication
//!
//! This library provides the core data structures and validation logic for a
//! protocol-based communication system. It supports typed commands and events
//! with metadata, validation, and error handling.
//!
//! # Examples
//!
//! ```no_run
//! use protocol::{Command, CommandType, Event, EventType};
//!
//! // Create a new command
//! let cmd = Command::new(
//!     "execute_task".to_string(),
//!     CommandType::Execute,
//!     r#"{"task": "example"}"#.to_string()
//! );
//!
//! // Validate the command
//! if let Err(e) = cmd.validate() {
//!     eprintln!("Invalid command: {}", e);
//! }
//!
//! // Create an event
//! let event = Event::new(
//!     "task_completed".to_string(),
//!     EventType::Completed,
//!     r#"{"result": "success"}"#.to_string()
//! );
//! ```

/// Protocol version following semantic versioning
pub const PROTOCOL_VERSION: &str = "1.0.0";

/// Major version number
pub const PROTOCOL_VERSION_MAJOR: u32 = 1;
/// Minor version number
pub const PROTOCOL_VERSION_MINOR: u32 = 0;
/// Patch version number
pub const PROTOCOL_VERSION_PATCH: u32 = 0;

pub mod command;
pub mod command_type;
pub mod event;
pub mod event_type;
pub mod metadata;
pub mod error;

// Re-export main types for convenience
pub use command::Command;
pub use command_type::CommandType;
pub use event::Event;
pub use event_type::EventType;
pub use metadata::Metadata;
pub use error::ValidationError;

/// Initializes the protocol library
///
/// Currently just prints a message, but can be extended for setup tasks
pub fn init() {
    println!("Initializing protocol library v{}...", PROTOCOL_VERSION);
}

/// Returns the protocol version string
pub fn version() -> &'static str {
    PROTOCOL_VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(version(), "1.0.0");
    }

    #[test]
    fn test_command_creation() {
        let cmd = Command::new(
            "test".to_string(),
            CommandType::Execute,
            "payload".to_string()
        );
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn test_event_creation() {
        let event = Event::new(
            "test_event".to_string(),
            EventType::Info,
            "data".to_string()
        );
        assert!(event.validate().is_ok());
    }

    #[test]
    fn test_validation_empty_name() {
        let cmd = Command::new(
            "".to_string(),
            CommandType::Execute,
            "payload".to_string()
        );
        assert!(cmd.validate().is_err());
    }

    #[test]
    fn test_validation_empty_payload() {
        let cmd = Command::new(
            "test".to_string(),
            CommandType::Execute,
            "".to_string()
        );
        assert!(cmd.validate().is_err());
    }

    #[test]
    fn test_validation_invalid_name_format() {
        let cmd = Command::new(
            "test command!@#".to_string(),
            CommandType::Execute,
            "payload".to_string()
        );
        assert!(cmd.validate().is_err());
    }
}
