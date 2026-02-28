use super::{Plan, RandomnessRecord};
use crate::planner::ExplanationTrace;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanCandidate {
    pub plan: Plan,
    pub score: f64,
    pub constraints_satisfied: Vec<String>,
    pub constraints_violated: Vec<String>,
    pub explanation_trace: ExplanationTrace,
    pub randomness_record: RandomnessRecord,
}
