use serde::{Deserialize, Serialize};

use crate::value_types::PassRate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionOutcomeStats {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub unknown: usize,
    pub pass_rate: PassRate,
}
