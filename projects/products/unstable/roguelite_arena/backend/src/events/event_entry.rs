use crate::events::ArenaEvent;
use crate::time::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EventEntry {
    pub(crate) tick: Tick,
    pub(crate) event: ArenaEvent,
}
