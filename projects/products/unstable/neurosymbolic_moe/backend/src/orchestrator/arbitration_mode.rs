//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/arbitration_mode.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArbitrationMode {
    Aggregation,
    RouterScoreWeighted,
}
