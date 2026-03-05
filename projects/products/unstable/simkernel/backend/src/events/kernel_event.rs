use crate::events::event_id::EventId;
use crate::time::tick::Tick;
use common_json::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelEvent {
    pub id: EventId,
    pub tick: Tick,
    pub kind: String,
    pub payload: Json,
}

impl KernelEvent {
    pub fn new(id: EventId, tick: Tick, kind: impl Into<String>, payload: Json) -> Self {
        Self {
            id,
            tick,
            kind: kind.into(),
            payload,
        }
    }
}
