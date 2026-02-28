#![allow(dead_code)]
use crate::events::event_id::EventId;
use crate::time::tick::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelEvent {
    pub id: EventId,
    pub tick: Tick,
    pub kind: String,
    pub payload: serde_json::Value,
}

impl KernelEvent {
    pub fn new(
        id: EventId,
        tick: Tick,
        kind: impl Into<String>,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            id,
            tick,
            kind: kind.into(),
            payload,
        }
    }
}
