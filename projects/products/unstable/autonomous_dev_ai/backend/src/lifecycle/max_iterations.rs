// projects/products/unstable/autonomous_dev_ai/src/lifecycle/max_iterations.rs
use std::num::NonZeroUsize;

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
