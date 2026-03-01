// projects/products/unstable/hospital_tycoon/backend/src/time/tick_clock.rs
use crate::time::tick::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickClock {
    pub seed: u64,
    pub max_ticks: u64,
    current: Tick,
}

impl TickClock {
    pub fn new(seed: u64, max_ticks: u64) -> Self {
        Self {
            seed,
            max_ticks,
            current: Tick::zero(),
        }
    }
    pub fn tick(&mut self) {
        if !self.is_done() {
            self.current = self.current.next();
        }
    }
    pub fn current_tick(&self) -> Tick {
        self.current
    }
    pub fn is_done(&self) -> bool {
        self.current.value() >= self.max_ticks
    }
}
