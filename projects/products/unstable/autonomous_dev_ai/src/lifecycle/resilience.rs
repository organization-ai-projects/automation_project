//! Resilience patterns: circuit breaker and retry strategy.

use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

#[derive(Debug)]
pub struct CircuitBreaker {
    state: CircuitState,
    failure_count: usize,
    half_open_success_count: usize,
    failure_threshold: usize,
    success_threshold: usize,
    timeout: Duration,
    last_failure_time: Option<Instant>,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: usize, success_threshold: usize, timeout: Duration) -> Self {
        Self {
            state: CircuitState::Closed,
            failure_count: 0,
            half_open_success_count: 0,
            failure_threshold,
            success_threshold,
            timeout,
            last_failure_time: None,
        }
    }

    pub fn should_allow_request(&mut self) -> bool {
        match self.state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                if let Some(last_failure) = self.last_failure_time {
                    if last_failure.elapsed() > self.timeout {
                        tracing::info!("Circuit breaker transitioning to HalfOpen");
                        self.state = CircuitState::HalfOpen;
                        self.half_open_success_count = 0;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            CircuitState::HalfOpen => true,
        }
    }

    pub fn record_success(&mut self) {
        match self.state {
            CircuitState::Closed => {
                self.failure_count = 0;
            }
            CircuitState::HalfOpen => {
                self.half_open_success_count = self.half_open_success_count.saturating_add(1);
                if self.half_open_success_count >= self.success_threshold {
                    tracing::info!("Circuit breaker transitioning to Closed");
                    self.state = CircuitState::Closed;
                    self.failure_count = 0;
                    self.half_open_success_count = 0;
                }
            }
            CircuitState::Open => {}
        }
    }

    pub fn record_failure(&mut self) {
        self.failure_count = self.failure_count.saturating_add(1);
        self.last_failure_time = Some(Instant::now());

        match self.state {
            CircuitState::Closed => {
                if self.failure_count >= self.failure_threshold {
                    tracing::warn!("Circuit breaker transitioning to Open");
                    self.state = CircuitState::Open;
                }
            }
            CircuitState::HalfOpen => {
                tracing::warn!("Circuit breaker transitioning back to Open");
                self.state = CircuitState::Open;
                self.half_open_success_count = 0;
            }
            CircuitState::Open => {}
        }
    }

    pub fn state(&self) -> CircuitState {
        self.state
    }
}

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
