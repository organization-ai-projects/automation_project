use crate::time::tick::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickClock { current: Tick }

impl TickClock {
    pub fn new() -> Self { Self { current: Tick(0) } }
    pub fn tick(&mut self) { self.current.advance(); }
    pub fn current(&self) -> Tick { self.current }
}

impl Default for TickClock { fn default() -> Self { Self::new() } }
