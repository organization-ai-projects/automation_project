use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuousGovernancePolicy {
    pub min_expert_success_rate: f64,
    pub min_routing_accuracy: f64,
    pub low_score_threshold: f64,
    pub regression_drop_threshold: f64,
    pub block_on_human_review: bool,
    pub auto_promote_on_pass: bool,
}

impl ContinuousGovernancePolicy {
    pub fn new(
        min_expert_success_rate: f64,
        min_routing_accuracy: f64,
        low_score_threshold: f64,
        regression_drop_threshold: f64,
        block_on_human_review: bool,
    ) -> Self {
        Self {
            min_expert_success_rate,
            min_routing_accuracy,
            low_score_threshold,
            regression_drop_threshold,
            block_on_human_review,
            auto_promote_on_pass: false,
        }
    }

    pub fn with_auto_promote_on_pass(mut self, enabled: bool) -> Self {
        self.auto_promote_on_pass = enabled;
        self
    }
}
