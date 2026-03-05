use crate::report::sim_report::SimReport;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReplayCheckpoint {
    pub tick: u64,
    pub snapshot_hash: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReplayFile {
    pub scenario_path: String,
    pub scenario_name: String,
    pub seed: u64,
    pub total_ticks: u64,
    pub expected_report: SimReport,
    pub checkpoints: Vec<ReplayCheckpoint>,
}
