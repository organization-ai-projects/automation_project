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
    /// Log line event
    LogLine,
    /// Job state changed event
    JobStateChanged,
    /// Job finished event
    JobFinished,
    /// Payload event
    Payload,
    /// Acknowledgment event
    Acknowledgment,
}

impl EventType {
    /// Static mapping of all EventType variants to their string representations
    const EVENT_TYPE_STRINGS: &'static [(&'static str, EventType)] = &[
        ("started", EventType::Started),
        ("stopped", EventType::Stopped),
        ("created", EventType::Created),
        ("updated", EventType::Updated),
        ("deleted", EventType::Deleted),
        ("error", EventType::Error),
        ("warning", EventType::Warning),
        ("info", EventType::Info),
        ("completed", EventType::Completed),
        ("failed", EventType::Failed),
        ("progress", EventType::Progress),
        ("state_changed", EventType::StateChanged),
        ("custom", EventType::Custom),
        ("log_line", EventType::LogLine),
        ("job_state_changed", EventType::JobStateChanged),
        ("job_finished", EventType::JobFinished),
        ("payload", EventType::Payload),
        ("acknowledgment", EventType::Acknowledgment),
    ];

    /// Returns a string representation of the event type
    pub fn as_str(&self) -> &'static str {
        Self::EVENT_TYPE_STRINGS
            .iter()
            .find(|(_, event_type)| event_type == self)
            .map(|(name, _)| *name)
            .unwrap_or("unknown")
    }

    /// Returns a mapping of all EventType variants to their string representations
    pub fn all_as_str() -> &'static [(&'static str, EventType)] {
        Self::EVENT_TYPE_STRINGS
    }
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
