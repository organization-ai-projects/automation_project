use crate::events::model_event::ModelEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventLog {
    pub events: Vec<ModelEvent>,
}

impl EventLog {
    pub fn push(&mut self, event: ModelEvent) {
        self.events.push(event);
    }
}
