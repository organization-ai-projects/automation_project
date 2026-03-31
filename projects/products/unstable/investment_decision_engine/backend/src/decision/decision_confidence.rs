use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionConfidence {
    pub score: f64,
    pub label: ConfidenceLabel,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConfidenceLabel {
    VeryLow,
    Low,
    Moderate,
    High,
    VeryHigh,
}

impl DecisionConfidence {
    pub fn from_score(score: f64) -> Self {
        let label = match score {
            s if s >= 0.9 => ConfidenceLabel::VeryHigh,
            s if s >= 0.7 => ConfidenceLabel::High,
            s if s >= 0.5 => ConfidenceLabel::Moderate,
            s if s >= 0.3 => ConfidenceLabel::Low,
            _ => ConfidenceLabel::VeryLow,
        };
        Self { score, label }
    }
}
