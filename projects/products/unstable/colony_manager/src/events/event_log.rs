use crate::events::colony_event::ColonyEvent;
use crate::time::tick::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventLog {
    pub entries: Vec<EventEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEntry {
    pub tick: Tick,
    pub event: ColonyEvent,
    pub draw_index: usize,
}

impl EventLog {
    pub fn record(&mut self, tick: Tick, event: ColonyEvent, draw_index: usize) {
        self.entries.push(EventEntry { tick, event, draw_index });
    }
}
