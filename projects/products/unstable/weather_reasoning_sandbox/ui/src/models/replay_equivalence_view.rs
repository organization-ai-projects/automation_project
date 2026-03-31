use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayEquivalenceView {
    pub original_report_checksum: String,
    pub replay_report_checksum: String,
    pub original_snapshot_checksum: String,
    pub replay_snapshot_checksum: String,
    pub is_equivalent: bool,
}
