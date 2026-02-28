use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogicalClock {
    counter: u64,
}

impl LogicalClock {
    pub fn new() -> Self {
        Self { counter: 0 }
    }

    /// Advances the clock and returns the new tick value.
    pub fn tick(&mut self) -> u64 {
        self.counter += 1;
        self.counter
    }

    pub fn current(&self) -> u64 {
        self.counter
    }
}

impl Default for LogicalClock {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_at_zero() {
        let clk = LogicalClock::new();
        assert_eq!(clk.current(), 0);
    }

    #[test]
    fn tick_increments_and_returns_new_value() {
        let mut clk = LogicalClock::new();
        assert_eq!(clk.tick(), 1);
        assert_eq!(clk.tick(), 2);
        assert_eq!(clk.current(), 2);
    }
}
