use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub turn: u64,
    pub expected_snapshot_hash: Option<String>,
}
