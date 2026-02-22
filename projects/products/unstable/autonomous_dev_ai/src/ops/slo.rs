// projects/products/unstable/autonomous_dev_ai/src/ops/slo.rs
use serde::{Deserialize, Serialize};

use crate::ops::Sli;

/// A Service-Level Objective: an SLI with a target threshold.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slo {
    pub sli: Sli,
    /// Target threshold value.
    pub target: f64,
    /// Measurement window in seconds.
    pub window_secs: u64,
    /// When true the observed value must be >= target (e.g., success rates).
    /// When false the observed value must be <= target (e.g., latency budgets).
    pub higher_is_better: bool,
}

impl Slo {
    pub fn new(name: &str, description: &str, target: f64, window_secs: u64) -> Self {
        Self {
            sli: Sli {
                name: name.to_string(),
                description: description.to_string(),
            },
            target,
            window_secs,
            higher_is_better: true,
        }
    }

    pub fn lower_is_better(mut self) -> Self {
        self.higher_is_better = false;
        self
    }
}
