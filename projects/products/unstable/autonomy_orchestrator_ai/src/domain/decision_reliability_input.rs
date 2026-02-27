use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionReliabilityInput {
    pub contributor_id: String,
    pub capability: String,
    pub score: u8,
}
