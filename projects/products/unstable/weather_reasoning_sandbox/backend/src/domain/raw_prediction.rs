use serde::{Deserialize, Serialize};

use crate::domain::prediction_confidence::PredictionConfidence;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawPrediction {
    pub forecast_label: String,
    pub confidence: PredictionConfidence,
    pub rationale: String,
}

impl RawPrediction {
    pub fn canonical_string(&self) -> String {
        format!(
            "label={},confidence=[{}],rationale={}",
            self.forecast_label,
            self.confidence.canonical_string(),
            self.rationale,
        )
    }
}
