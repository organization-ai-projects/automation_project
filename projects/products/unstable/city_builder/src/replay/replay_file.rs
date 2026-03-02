#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReplayFile {
    pub scenario_name: String,
    pub seed: u64,
    pub total_ticks: u64,
}
