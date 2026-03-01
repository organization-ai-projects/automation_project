use crate::time::Tick;

pub struct TickClock {
    current: Tick,
}

impl TickClock {
    pub fn new() -> Self {
        Self { current: Tick(0) }
    }

    pub fn advance(&mut self) {
        self.current = Tick(self.current.0 + 1);
    }

    pub fn current(&self) -> Tick {
        self.current
    }
}

impl Default for TickClock {
    fn default() -> Self {
        Self::new()
    }
}
