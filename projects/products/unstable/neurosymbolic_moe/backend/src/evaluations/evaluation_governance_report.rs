use crate::moe_core::ExpertId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationGovernanceReport {
    pub min_expert_success_rate: f64,
    pub min_routing_accuracy: f64,
    pub underperforming_experts: Vec<ExpertId>,
    pub routing_accuracy_below_threshold: bool,
    pub ready_for_promotion: bool,
}
