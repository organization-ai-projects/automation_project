use serde::{Deserialize, Serialize};

use super::turn_report::TurnReport;

/// Canonical run report. JSON fields are serialized in struct definition order;
/// use JsonCodec (which sorts keys) for canonical bytes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunReport {
    pub game_id: String,
    pub seed: u64,
    pub turns_played: u64,
    pub turn_reports: Vec<TurnReport>,
    pub final_snapshot_hash: String,
}
