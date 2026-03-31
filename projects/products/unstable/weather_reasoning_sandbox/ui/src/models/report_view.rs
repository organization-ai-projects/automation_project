use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportView {
    pub seed: u64,
    pub tick_count: u64,
    pub dataset_name: String,
    pub contradiction_count: usize,
    pub total_violations: usize,
    pub total_corrections: usize,
    pub report_checksum: String,
    pub snapshot_checksum: Option<String>,
    pub final_forecast_label: Option<String>,
}
