//! projects/products/unstable/autonomous_dev_ai/src/lifecycle/step_index.rs
// Step index in a plan (0-indexed).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct StepIndex(usize);

impl StepIndex {
    pub(crate) fn zero() -> Self {
        Self(0)
    }

    pub(crate) fn increment(self) -> Self {
        Self(self.0.saturating_add(1))
    }

    pub(crate) fn get(self) -> usize {
        self.0
    }
}
