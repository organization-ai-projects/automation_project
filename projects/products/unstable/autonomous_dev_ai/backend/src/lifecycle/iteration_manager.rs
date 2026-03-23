//! projects/products/unstable/autonomous_dev_ai/backend/src/lifecycle/iteration_manager.rs
use crate::lifecycle::{IterationNumber, StepIndex};

/// Manages iteration and step indices for the lifecycle.
pub struct IterationManager {
    current_iteration_number: IterationNumber,
    current_step_index: StepIndex,
}

impl IterationManager {
    /// Creates a new `IterationManager` with the given iteration number and step index.
    pub fn new(current_iteration_number: IterationNumber, current_step_index: StepIndex) -> Self {
        Self {
            current_iteration_number,
            current_step_index,
        }
    }

    /// Sets the iteration number.
    pub fn set_iteration_number(&mut self, value: IterationNumber) {
        self.current_iteration_number = value;
    }

    /// Resets the step index to its initial value.
    pub fn reset_step_index(&mut self) {
        self.current_step_index = StepIndex::zero();
    }

    /// Advances the step index to the next step.
    pub fn advance_step_index(&mut self) {
        self.current_step_index = self.current_step_index.increment();
    }

    /// Returns the current iteration number.
    pub fn current_iteration(&self) -> IterationNumber {
        self.current_iteration_number
    }

    /// Returns the current step index.
    pub fn current_step(&self) -> StepIndex {
        self.current_step_index
    }
}
