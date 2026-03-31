use crate::events::ArenaEvent;
use crate::events::EventEntry;
use crate::time::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct EventLog {
    pub(crate) entries: Vec<EventEntry>,
}

impl EventLog {
    pub(crate) fn record(&mut self, tick: Tick, event: ArenaEvent) {
        self.entries.push(EventEntry { tick, event });
    }
}
