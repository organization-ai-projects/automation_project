use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionReason {
    pub category: ReasonCategory,
    pub description: String,
    pub weight: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReasonCategory {
    Fundamental,
    Technical,
    Sentiment,
    Risk,
    Valuation,
    CostBasis,
    ThesisIntegrity,
    Neural,
}

impl DecisionReason {
    pub fn new(
        category: ReasonCategory,
        description: impl Into<String>,
        weight: f64,
    ) -> Self {
        Self {
            category,
            description: description.into(),
            weight,
        }
    }
}
