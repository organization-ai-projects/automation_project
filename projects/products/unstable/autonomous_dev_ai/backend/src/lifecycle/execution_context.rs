// projects/products/unstable/autonomous_dev_ai/src/lifecycle/execution_context.rs
use std::time::{Duration, Instant};

use crate::{lifecycle::IterationNumber, timeout::Timeout};

/// Execution context for each iteration.
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub iteration: IterationNumber,
    pub start_time: Instant,
    pub timeout: Timeout,
}

impl ExecutionContext {
    pub fn new(iteration: IterationNumber, timeout: Timeout) -> Self {
        Self {
            iteration,
            start_time: Instant::now(),
            timeout,
        }
    }

    pub fn is_timed_out(&self) -> bool {
        self.start_time.elapsed() > self.timeout.duration
    }

    pub fn remaining_time(&self) -> Option<Duration> {
        self.timeout.duration.checked_sub(self.start_time.elapsed())
    }
}
