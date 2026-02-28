// projects/products/unstable/digital_pet/backend/src/events/sim_event.rs
use crate::time::tick::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimEvent {
    pub tick: Tick,
    pub kind: SimEventKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimEventKind {
    Evolved { from: String, to: String },
    CareAction { kind: String },
    CareMistake { reason: String },
    BattleStarted,
    BattleEnded { winner: String },
}

impl SimEvent {
    pub fn evolved(tick: Tick, from: String, to: String) -> Self {
        Self {
            tick,
            kind: SimEventKind::Evolved { from, to },
        }
    }
}
