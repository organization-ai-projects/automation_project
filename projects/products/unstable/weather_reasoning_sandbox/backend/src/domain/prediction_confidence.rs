use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictionConfidence {
    pub clear_sky: f64,
    pub precipitation: f64,
    pub storm: f64,
    pub calm: f64,
    pub windy: f64,
}

impl PredictionConfidence {
    pub fn canonical_string(&self) -> String {
        format!(
            "clear={:.4},precip={:.4},storm={:.4},calm={:.4},windy={:.4}",
            self.clear_sky, self.precipitation, self.storm, self.calm, self.windy,
        )
    }
}
