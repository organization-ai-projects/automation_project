use crate::sim::sim_event::SimEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventLog {
    pub events: Vec<SimEvent>,
}

impl EventLog {
    pub fn push(&mut self, event: SimEvent) {
        self.events.push(event);
    }

    #[allow(dead_code)]
    pub fn iter(&self) -> impl Iterator<Item = &SimEvent> {
        self.events.iter()
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.events.len()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}
