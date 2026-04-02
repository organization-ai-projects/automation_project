use serde::{Deserialize, Serialize};

use crate::domain::prediction_confidence::PredictionConfidence;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorrectedPrediction {
    pub forecast_label: String,
    pub confidence: PredictionConfidence,
    pub corrections_applied: Vec<String>,
    pub explanation: String,
}

impl CorrectedPrediction {
    pub fn canonical_string(&self) -> String {
        let corrections = self.corrections_applied.join(";");
        format!(
            "label={},confidence=[{}],corrections=[{}],explanation={}",
            self.forecast_label,
            self.confidence.canonical_string(),
            corrections,
            self.explanation,
        )
    }
}
