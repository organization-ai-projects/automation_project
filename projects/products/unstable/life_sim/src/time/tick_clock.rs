use super::tick::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickClock {
    current: Tick,
}

impl TickClock {
    pub fn new(start: Tick) -> Self {
        Self { current: start }
    }

    pub fn advance(&mut self) {
        self.current = Tick(self.current.0 + 1);
    }

    pub fn current(&self) -> Tick {
        self.current
    }

    #[allow(dead_code)]
    pub fn elapsed_since(&self, start: Tick) -> u64 {
        self.current.0.saturating_sub(start.0)
    }
}
