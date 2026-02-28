use super::TickReport;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SimReport {
    pub scenario_name: String,
    pub seed: u64,
    pub total_ticks: u64,
    pub tick_reports: Vec<TickReport>,
    pub run_hash: String,
}
