use serde::{Deserialize, Serialize};

use crate::decision::{CandidateAction, DecisionConfidence};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionEntry {
    pub timestamp: String,
    pub ticker: String,
    pub action: CandidateAction,
    pub confidence: DecisionConfidence,
    pub rationale: String,
    pub invalidation_conditions: Vec<String>,
}

impl DecisionEntry {
    pub fn new(
        timestamp: impl Into<String>,
        ticker: impl Into<String>,
        action: CandidateAction,
        confidence: DecisionConfidence,
        rationale: impl Into<String>,
        invalidation_conditions: Vec<String>,
    ) -> Self {
        Self {
            timestamp: timestamp.into(),
            ticker: ticker.into(),
            action,
            confidence,
            rationale: rationale.into(),
            invalidation_conditions,
        }
    }
}
