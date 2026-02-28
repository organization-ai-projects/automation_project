use super::replay_event::ReplayEvent;
use serde::{Deserialize, Serialize};

/// An ordered log of all replay events across all turns.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventLog {
    pub events: Vec<ReplayEvent>,
}

impl EventLog {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn push(&mut self, event: ReplayEvent) {
        self.events.push(event);
    }
}

impl Default for EventLog {
    fn default() -> Self {
        Self::new()
    }
}
