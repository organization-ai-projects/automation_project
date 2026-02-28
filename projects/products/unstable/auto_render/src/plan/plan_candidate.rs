use serde::{Deserialize, Serialize};
use super::{Plan, RandomnessRecord};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanCandidate {
    pub plan: Plan,
    pub score: f64,
    pub constraints_satisfied: Vec<String>,
    pub constraints_violated: Vec<String>,
    pub explanation_trace: String,
    pub randomness_record: RandomnessRecord,
}
