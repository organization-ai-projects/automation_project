use serde::{Deserialize, Serialize};

use crate::decision::{CandidateAction, DecisionConfidence, DecisionReason, WaitThesis};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionScore {
    pub action: CandidateAction,
    pub score: f64,
    pub reasons: Vec<DecisionReason>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionSummary {
    pub recommended_action: CandidateAction,
    pub confidence: DecisionConfidence,
    pub action_scores: Vec<ActionScore>,
    pub primary_reasons: Vec<DecisionReason>,
    pub principal_risks: Vec<String>,
    pub invalidation_conditions: Vec<String>,
    pub new_facts_that_would_change: Vec<String>,
    pub wait_thesis: Option<WaitThesis>,
    #[serde(skip_serializing_if = "is_false")]
    pub recommendation_gated: bool,
}

fn is_false(v: &bool) -> bool {
    !v
}

impl DecisionSummary {
    pub fn gated() -> Self {
        Self {
            recommended_action: CandidateAction::Hold,
            confidence: DecisionConfidence::from_score(0.0),
            action_scores: Vec::new(),
            primary_reasons: Vec::new(),
            principal_risks: Vec::new(),
            invalidation_conditions: Vec::new(),
            new_facts_that_would_change: Vec::new(),
            wait_thesis: None,
            recommendation_gated: true,
        }
    }
}
