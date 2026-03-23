use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetQualityReport {
    pub total_entries: usize,
    pub scored_entries: usize,
    pub average_score: Option<f64>,
    pub low_score_entries: usize,
    pub corrected_entries: usize,
    pub correction_ratio: f64,
    pub success_ratio: f64,
}
