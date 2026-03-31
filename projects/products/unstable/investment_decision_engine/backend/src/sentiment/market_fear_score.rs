use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketFearScore {
    pub score: f64,
    pub label: FearLevel,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FearLevel {
    ExtremeFear,
    Fear,
    Neutral,
    Greed,
    ExtremeGreed,
}

impl MarketFearScore {
    pub fn from_score(score: f64, source: impl Into<String>) -> Self {
        let label = match score {
            s if s <= 20.0 => FearLevel::ExtremeFear,
            s if s <= 40.0 => FearLevel::Fear,
            s if s <= 60.0 => FearLevel::Neutral,
            s if s <= 80.0 => FearLevel::Greed,
            _ => FearLevel::ExtremeGreed,
        };
        Self {
            score,
            label,
            source: source.into(),
        }
    }

    pub fn is_fearful(&self) -> bool {
        matches!(self.label, FearLevel::ExtremeFear | FearLevel::Fear)
    }
}
