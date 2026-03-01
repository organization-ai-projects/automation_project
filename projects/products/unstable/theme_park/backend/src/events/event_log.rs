#![allow(dead_code)]
use crate::events::sim_event::SimEvent;
use serde::{Deserialize, Serialize};

/// Append-only log of all simulation events.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventLog {
    events: Vec<SimEvent>,
}

impl EventLog {
    pub fn new() -> Self {
        Self::default()
    }

    /// Append an event to the log.
    pub fn push(&mut self, event: SimEvent) {
        self.events.push(event);
    }

    pub fn iter(&self) -> impl Iterator<Item = &SimEvent> {
        self.events.iter()
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Deterministic checksum over all event kinds and tick values.
    pub fn checksum(&self) -> u64 {
        self.events.iter().enumerate().fold(0u64, |acc, (i, e)| {
            let tick_val = e.tick().value();
            let kind_hash = e.kind_name().bytes().fold(0u64, |a, b| {
                a.wrapping_mul(31).wrapping_add(b as u64)
            });
            acc.wrapping_add(tick_val.wrapping_mul(i as u64 + 1))
                .wrapping_add(kind_hash)
        })
    }

    pub fn events(&self) -> &[SimEvent] {
        &self.events
    }
}
