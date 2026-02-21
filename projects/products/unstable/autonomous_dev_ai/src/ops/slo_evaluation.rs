// projects/products/unstable/autonomous_dev_ai/src/ops/slo_evaluation.rs
use serde::{Deserialize, Serialize};

/// Result of evaluating an SLO against observed data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SloEvaluation {
    pub slo_name: String,
    pub observed_ratio: f64,
    pub target: f64,
    pub met: bool,
}
