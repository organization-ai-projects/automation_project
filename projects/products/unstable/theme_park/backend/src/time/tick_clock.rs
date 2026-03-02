#![allow(dead_code)]
use crate::time::tick::Tick;
use serde::{Deserialize, Serialize};

/// Logical clock that only advances through explicit calls — no wall-clock.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickClock {
    pub tick: Tick,
}

impl TickClock {
    pub fn new() -> Self {
        Self { tick: Tick::zero() }
    }

    pub fn advance(&mut self) {
        self.tick = self.tick.next();
    }

    pub fn current(&self) -> Tick {
        self.tick
    }
}

impl Default for TickClock {
    fn default() -> Self {
        Self::new()
    }
}
