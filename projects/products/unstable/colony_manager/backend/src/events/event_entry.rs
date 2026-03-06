use crate::events::colony_event::ColonyEvent;
use crate::time::tick::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEntry {
    pub tick: Tick,
    pub event: ColonyEvent,
    pub draw_index: usize,
}
