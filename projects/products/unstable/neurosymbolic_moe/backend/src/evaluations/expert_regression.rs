use crate::moe_core::ExpertId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpertRegression {
    pub expert_id: ExpertId,
    pub previous_success_rate: f64,
    pub current_success_rate: f64,
    pub delta: f64,
}
