use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceStateDiff {
    pub source_version: u64,
    pub target_version: u64,
    pub version_delta: i64,
    pub schema_version_changed: bool,
    pub checksum_changed: bool,
    pub policy_changed: bool,
    pub baseline_changed: bool,
    pub report_changed: bool,
    pub has_drift: bool,
}
