use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecksumView {
    pub report_checksum: String,
    pub snapshot_checksum: Option<String>,
}
