#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReplayCheckpoint {
    pub tick: u64,
    pub snapshot_hash: String,
}
