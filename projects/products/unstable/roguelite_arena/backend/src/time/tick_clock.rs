use crate::time::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TickClock {
    current: Tick,
}

impl TickClock {
    pub(crate) fn new() -> Self {
        Self { current: Tick(0) }
    }

    pub(crate) fn tick(&mut self) {
        self.current.advance();
    }

    pub(crate) fn current(&self) -> Tick {
        self.current
    }
}

impl Default for TickClock {
    fn default() -> Self {
        Self::new()
    }
}
