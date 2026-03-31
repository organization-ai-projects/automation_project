use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContradictionView {
    pub tick: u64,
    pub raw_forecast_label: String,
    pub violation_count: usize,
    pub violation_ids: Vec<String>,
    pub correction_count: usize,
    pub corrected_forecast_label: String,
}
