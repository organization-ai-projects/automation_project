use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub schema_version: u32,
    pub record_count: usize,
    pub snapshot_hash: String,
    pub canonical_bytes_len: u64,
}
