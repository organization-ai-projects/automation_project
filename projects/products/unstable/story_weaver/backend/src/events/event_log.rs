use serde::{Deserialize, Serialize};

use crate::events::StoryEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventLog {
    events: Vec<StoryEvent>,
}

impl EventLog {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn push(&mut self, event: StoryEvent) {
        self.events.push(event);
    }

    pub fn events(&self) -> &[StoryEvent] {
        &self.events
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}
