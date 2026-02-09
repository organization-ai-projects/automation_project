//! Strong types for lifecycle management.

use std::num::NonZeroUsize;
use std::time::{Duration, Instant};

/// Iteration number (1-indexed, cannot be zero).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IterationNumber(NonZeroUsize);

impl IterationNumber {
    pub fn first() -> Self {
        Self(NonZeroUsize::MIN)
    }

    pub fn try_next(self) -> Option<Self> {
        NonZeroUsize::new(self.0.get().checked_add(1)?).map(Self)
    }

    pub fn get(self) -> usize {
        self.0.get()
    }

    pub fn exceeds(self, limit: MaxIterations) -> bool {
        self.0.get() > limit.0.get()
    }
}

impl std::fmt::Display for IterationNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Maximum iterations (must be at least 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaxIterations(NonZeroUsize);

impl MaxIterations {
    pub fn new(value: usize) -> Option<Self> {
        NonZeroUsize::new(value).map(Self)
    }

    pub fn default_value() -> Self {
        Self(NonZeroUsize::new(10).unwrap_or(NonZeroUsize::MIN))
    }

    pub fn get(self) -> usize {
        self.0.get()
    }
}

impl Default for MaxIterations {
    fn default() -> Self {
        Self::default_value()
    }
}

/// Step index in a plan (0-indexed).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct StepIndex(usize);

impl StepIndex {
    pub fn zero() -> Self {
        Self(0)
    }

    pub fn increment(self) -> Self {
        Self(self.0.saturating_add(1))
    }

    pub fn get(self) -> usize {
        self.0
    }
}

/// Execution context for each iteration.
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub iteration: IterationNumber,
    pub start_time: Instant,
    pub timeout: Duration,
}

impl ExecutionContext {
    pub fn new(iteration: IterationNumber, timeout: Duration) -> Self {
        Self {
            iteration,
            start_time: Instant::now(),
            timeout,
        }
    }

    pub fn is_timed_out(&self) -> bool {
        self.start_time.elapsed() > self.timeout
    }

    pub fn remaining_time(&self) -> Option<Duration> {
        self.timeout.checked_sub(self.start_time.elapsed())
    }
}
