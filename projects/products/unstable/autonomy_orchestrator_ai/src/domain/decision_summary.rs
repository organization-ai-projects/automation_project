use serde::{Deserialize, Serialize};

use crate::domain::{
    DecisionContribution, DecisionReliabilityFactor, DecisionReliabilityUpdate, FinalDecision,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionSummary {
    pub final_decision: FinalDecision,
    pub decision_confidence: u8,
    pub decision_rationale_codes: Vec<String>,
    pub contributions: Vec<DecisionContribution>,
    pub reliability_factors: Vec<DecisionReliabilityFactor>,
    pub reliability_updates: Vec<DecisionReliabilityUpdate>,
    pub threshold: u8,
}
