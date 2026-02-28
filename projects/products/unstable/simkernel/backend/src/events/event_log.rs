#![allow(dead_code)]
use crate::events::event_id::EventId;
use crate::events::kernel_event::KernelEvent;
use crate::time::tick::Tick;

#[derive(Debug, Clone, Default)]
pub struct EventLog {
    events: Vec<KernelEvent>,
    next_id: u64,
}

impl EventLog {
    pub fn new() -> Self { Self::default() }

    pub fn push(&mut self, event: KernelEvent) {
        self.events.push(event);
    }

    pub fn emit(&mut self, tick: Tick, kind: impl Into<String>, payload: serde_json::Value) -> EventId {
        let id = EventId(self.next_id);
        self.next_id += 1;
        self.events.push(KernelEvent::new(id, tick, kind, payload));
        id
    }

    pub fn iter(&self) -> impl Iterator<Item = &KernelEvent> {
        self.events.iter()
    }

    pub fn len(&self) -> usize { self.events.len() }

    pub fn is_empty(&self) -> bool { self.events.is_empty() }

    pub fn checksum(&self) -> u64 {
        self.events.iter().fold(0u64, |acc, e| {
            acc.wrapping_add(e.id.0).wrapping_mul(1_000_003)
        })
    }
}
