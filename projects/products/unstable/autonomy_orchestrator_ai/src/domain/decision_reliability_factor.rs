use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionReliabilityFactor {
    pub contributor_id: String,
    pub capability: String,
    pub reliability_score: u8,
    pub reliability_factor: u16,
    pub base_score: u64,
    pub adjusted_score: u64,
}
