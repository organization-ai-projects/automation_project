use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NeuralSummary {
    pub ticker: String,
    pub company_summary: Option<String>,
    pub earnings_evolution: Option<String>,
    pub narrative_classification: Option<String>,
    pub decline_classification: Option<DeclineClassification>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeclineClassification {
    FundamentallyJustified,
    PanicDriven,
    NarrativeOvershoot,
    Mixed,
    Unclear,
}

impl NeuralSummary {
    pub fn empty(ticker: impl Into<String>) -> Self {
        Self {
            ticker: ticker.into(),
            company_summary: None,
            earnings_evolution: None,
            narrative_classification: None,
            decline_classification: None,
        }
    }
}
