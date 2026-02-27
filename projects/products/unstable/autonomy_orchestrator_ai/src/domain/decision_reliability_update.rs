use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionReliabilityUpdate {
    pub contributor_id: String,
    pub capability: String,
    pub previous_score: u8,
    pub new_score: u8,
    pub reason_code: String,
}
