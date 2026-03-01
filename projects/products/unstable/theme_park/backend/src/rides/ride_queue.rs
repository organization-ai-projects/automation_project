#![allow(dead_code)]
use crate::visitors::visitor_id::VisitorId;
use serde::{Deserialize, Serialize};

/// Queue for a single ride. Deterministic FIFO, insertion-ordered.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RideQueue {
    entries: Vec<VisitorId>,
}

impl RideQueue {
    pub fn new() -> Self {
        Self::default()
    }

    /// Enqueue a visitor (appended to back).
    pub fn enqueue(&mut self, id: VisitorId) {
        if !self.entries.contains(&id) {
            self.entries.push(id);
        }
    }

    /// Dequeue up to `n` visitors (from front). Deterministic.
    pub fn dequeue_batch(&mut self, n: usize) -> Vec<VisitorId> {
        let take = n.min(self.entries.len());
        self.entries.drain(..take).collect()
    }

    pub fn contains(&self, id: VisitorId) -> bool {
        self.entries.contains(&id)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn remove(&mut self, id: VisitorId) {
        self.entries.retain(|v| *v != id);
    }
}
