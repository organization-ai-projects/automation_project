// projects/libraries/protocol/src/command_type.rs
use serde::{Deserialize, Serialize};

/// Types of commands supported by the protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandType {
    /// Execute a task or operation
    Execute,
    /// Query for information
    Query,
    /// Update or modify existing data
    Update,
    /// Delete data or resources
    Delete,
    /// Create new resources
    Create,
    /// Subscribe to events or updates
    Subscribe,
    /// Unsubscribe from events or updates
    Unsubscribe,
    /// Configuration command
    Configure,
    /// Custom command type
    Custom,
}

impl CommandType {
    /// Returns a string representation of the command type
    pub fn as_str(&self) -> &'static str {
        match self {
            CommandType::Execute => "execute",
            CommandType::Query => "query",
            CommandType::Update => "update",
            CommandType::Delete => "delete",
            CommandType::Create => "create",
            CommandType::Subscribe => "subscribe",
            CommandType::Unsubscribe => "unsubscribe",
            CommandType::Configure => "configure",
            CommandType::Custom => "custom",
        }
    }
}

impl std::fmt::Display for CommandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
