#![allow(dead_code)]
use crate::time::tick::Tick;
use crate::time::turn::Turn;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicalClock {
    pub tick: Tick,
    pub turn: Turn,
    pub ticks_per_turn: u64,
}

impl LogicalClock {
    pub fn new() -> Self {
        Self {
            tick: Tick::zero(),
            turn: Turn::zero(),
            ticks_per_turn: 10,
        }
    }

    pub fn advance_tick(&mut self) {
        self.tick = self.tick.next();
        if self.tick.0.is_multiple_of(self.ticks_per_turn) {
            self.turn = self.turn.next();
        }
    }
}

impl Default for LogicalClock {
    fn default() -> Self {
        Self::new()
    }
}
