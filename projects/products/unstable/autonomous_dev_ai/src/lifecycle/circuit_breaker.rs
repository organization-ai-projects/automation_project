// projects/products/unstable/autonomous_dev_ai/src/lifecycle/circuit_breaker.rs
use std::time::Instant;

use crate::{lifecycle::CircuitState, timeout::Timeout};

#[derive(Debug)]
pub struct CircuitBreaker {
    state: CircuitState,
    failure_count: usize,
    half_open_success_count: usize,
    failure_threshold: usize,
    success_threshold: usize,
    timeout: Timeout,
    last_failure_time: Option<Instant>,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: usize, success_threshold: usize, timeout: Timeout) -> Self {
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
                    if last_failure.elapsed() > self.timeout.duration {
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
