// projects/products/unstable/autonomous_dev_ai/src/lifecycle/iteration_number.rs
use std::num::NonZeroUsize;

use crate::lifecycle::MaxIterations;

/// Iteration number (1-indexed, cannot be zero).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IterationNumber(NonZeroUsize);

impl IterationNumber {
    pub fn from_usize(value: usize) -> Option<Self> {
        NonZeroUsize::new(value).map(Self)
    }

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
        self.0.get() > limit.get()
    }
}

impl std::fmt::Display for IterationNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
