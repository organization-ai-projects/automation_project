// projects/products/unstable/autonomous_dev_ai/src/lifecycle/step_index.rs
// Step index in a plan (0-indexed).
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
