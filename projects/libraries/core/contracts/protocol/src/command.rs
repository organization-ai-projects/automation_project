// projects/libraries/protocol/src/command.rs
use crate::payload::Payload;
use crate::{CommandType, metadata::Metadata};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub metadata: Metadata,
    pub command_type: CommandType,

    // For StartJob and generic payload transport
    pub action: Option<String>,   // ex: "git_autopilot.preview"
    pub payload: Option<Payload>, // Replaced payload_type and payload with Payload struct
}

impl Command {
    pub fn validate(&self) -> bool {
        let action_ok = self.action.as_ref().is_some_and(|s| !s.trim().is_empty());
        let payload_ok = self.payload.as_ref().is_some_and(|p| {
            p.payload_type
                .as_ref()
                .is_some_and(|s| !s.trim().is_empty())
                && p.payload.is_some()
        });
        action_ok && payload_ok
    }
}
