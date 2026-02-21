//projects/products/unstable/autonomous_dev_ai/src/lifecycle/retry_strategy.rs
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct RetryStrategy {
    max_attempts: usize,
    initial_delay: Duration,
    max_delay: Duration,
    multiplier: f64,
}

impl RetryStrategy {
    pub fn new(max_attempts: usize) -> Self {
        Self {
            max_attempts,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            multiplier: 2.0,
        }
    }

    pub fn with_delays(mut self, initial: Duration, max: Duration) -> Self {
        self.initial_delay = initial;
        self.max_delay = max;
        self
    }

    pub fn delay_for_attempt(&self, attempt: usize) -> Option<Duration> {
        if attempt >= self.max_attempts {
            return None;
        }

        let delay_ms = self.initial_delay.as_millis() as f64 * self.multiplier.powi(attempt as i32);
        let delay = Duration::from_millis(delay_ms as u64);
        Some(delay.min(self.max_delay))
    }

    pub fn max_attempts(&self) -> usize {
        self.max_attempts
    }
}

impl Default for RetryStrategy {
    fn default() -> Self {
        Self::new(3)
    }
}
