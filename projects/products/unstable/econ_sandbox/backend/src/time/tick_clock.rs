use crate::time::tick::Tick;

pub struct TickClock {
    current: u64,
    total: u64,
}

impl TickClock {
    pub fn new(total: u64) -> Self {
        Self { current: 0, total }
    }

    pub fn current(&self) -> Tick {
        Tick(self.current)
    }

    pub fn advance(&mut self) {
        self.current += 1;
    }

    pub fn is_done(&self) -> bool {
        self.current >= self.total
    }
}
