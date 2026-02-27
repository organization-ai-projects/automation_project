use serde::{Deserialize, Serialize};

use crate::domain::FinalDecision;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionContribution {
    pub contributor_id: String,
    pub capability: String,
    pub vote: FinalDecision,
    pub confidence: u8,
    pub weight: u8,
    pub reason_codes: Vec<String>,
    pub artifact_refs: Vec<String>,
}
