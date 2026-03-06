// projects/products/unstable/digital_pet/backend/src/events/sim_event.rs
use crate::events::sim_event_kind::SimEventKind;
use crate::time::tick::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimEvent {
    pub tick: Tick,
    pub kind: SimEventKind,
}

impl SimEvent {
    pub fn evolved(tick: Tick, from: String, to: String) -> Self {
        Self {
            tick,
            kind: SimEventKind::Evolved { from, to },
        }
    }
}
