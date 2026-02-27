use serde::{Deserialize, Serialize};

use crate::domain::{DecisionContribution, FinalDecision};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionSummary {
    pub final_decision: FinalDecision,
    pub decision_confidence: u8,
    pub decision_rationale_codes: Vec<String>,
    pub contributions: Vec<DecisionContribution>,
    pub threshold: u8,
}
