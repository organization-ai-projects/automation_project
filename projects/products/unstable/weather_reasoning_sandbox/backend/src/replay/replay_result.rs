use serde::{Deserialize, Serialize};

use crate::domain::checksum_value::ChecksumValue;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReplayResult {
    pub original_report_checksum: ChecksumValue,
    pub replay_report_checksum: ChecksumValue,
    pub original_snapshot_checksum: ChecksumValue,
    pub replay_snapshot_checksum: ChecksumValue,
    pub is_equivalent: bool,
}
