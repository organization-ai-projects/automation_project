//! projects/products/unstable/autonomous_dev_ai/src/persistence/action_outcome_stats.rs
use serde::{Deserialize, Serialize};

use crate::value_types::{ActionName, PassRate};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionOutcomeStats {
    pub(crate) action: ActionName,
    pub(crate) total: u32,
    pub(crate) passed: u32,
    pub(crate) failed: u32,
    pub(crate) unknown: u32,
    pub(crate) pass_rate: PassRate,
}

impl ActionOutcomeStats {
    pub(crate) fn new(
        action: ActionName,
        passed: u32,
        failed: u32,
        unknown: u32,
        pass_rate: PassRate,
    ) -> Self {
        let total = passed + failed + unknown;
        Self {
            action,
            total,
            passed,
            failed,
            unknown,
            pass_rate,
        }
    }

    pub fn action(&self) -> &ActionName {
        &self.action
    }

    pub fn set_action(&mut self, action: ActionName) {
        self.action = action;
    }

    pub fn passed(&self) -> u32 {
        self.passed
    }

    pub fn set_passed(&mut self, value: u32) {
        let diff = value as i32 - self.passed as i32;
        self.passed = value;
        self.total = (self.total as i32 + diff) as u32;
    }

    pub fn failed(&self) -> u32 {
        self.failed
    }

    pub fn set_failed(&mut self, value: u32) {
        let diff = value as i32 - self.failed as i32;
        self.failed = value;
        self.total = (self.total as i32 + diff) as u32;
    }

    pub fn unknown(&self) -> u32 {
        self.unknown
    }

    pub fn set_unknown(&mut self, value: u32) {
        let diff = value as i32 - self.unknown as i32;
        self.unknown = value;
        self.total = (self.total as i32 + diff) as u32;
    }

    pub fn total(&self) -> u32 {
        self.total
    }
}
