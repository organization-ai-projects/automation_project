// projects/products/unstable/digital_pet/backend/src/events/event_log.rs
use crate::events::sim_event::SimEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventLog {
    pub events: Vec<SimEvent>,
}

impl EventLog {
    pub fn new() -> Self { Self::default() }
    pub fn push(&mut self, event: SimEvent) { self.events.push(event); }
    pub fn len(&self) -> usize { self.events.len() }
    pub fn is_empty(&self) -> bool { self.events.is_empty() }
}
