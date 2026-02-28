use super::Tick;

#[derive(Debug, Clone)]
pub struct TickClock {
    pub current: Tick,
}

impl TickClock {
    pub fn new() -> Self {
        Self { current: Tick(0) }
    }

    pub fn advance(&mut self) {
        self.current = Tick(self.current.0 + 1);
    }
}

impl Default for TickClock {
    fn default() -> Self {
        Self::new()
    }
}
