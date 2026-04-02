use serde::{Deserialize, Serialize};

use crate::events::sim_event::SimEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventLog {
    events: Vec<SimEvent>,
}

impl EventLog {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn push(&mut self, event: SimEvent) {
        self.events.push(event);
    }

    pub fn events(&self) -> &[SimEvent] {
        &self.events
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}
