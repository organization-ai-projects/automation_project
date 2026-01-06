// projects/libraries/protocol/src/event_type.rs
use serde::{Deserialize, Serialize};

/// Types of events in the protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    /// System started or initialized
    Started,
    /// System stopped or shut down
    Stopped,
    /// Data was created
    Created,
    /// Data was updated
    Updated,
    /// Data was deleted
    Deleted,
    /// An error occurred
    Error,
    /// A warning was issued
    Warning,
    /// Informational event
    Info,
    /// Task completed successfully
    Completed,
    /// Task failed
    Failed,
    /// Progress update
    Progress,
    /// State changed
    StateChanged,
    /// Custom event type
    Custom,
}

impl EventType {
    /// Returns a string representation of the event type
    pub fn as_str(&self) -> &'static str {
        match self {
            EventType::Started => "started",
            EventType::Stopped => "stopped",
            EventType::Created => "created",
            EventType::Updated => "updated",
            EventType::Deleted => "deleted",
            EventType::Error => "error",
            EventType::Warning => "warning",
            EventType::Info => "info",
            EventType::Completed => "completed",
            EventType::Failed => "failed",
            EventType::Progress => "progress",
            EventType::StateChanged => "state_changed",
            EventType::Custom => "custom",
        }
    }
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
