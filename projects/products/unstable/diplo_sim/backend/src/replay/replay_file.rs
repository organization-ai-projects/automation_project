use super::event_log::EventLog;
use serde::{Deserialize, Serialize};

/// Complete replay file: everything needed to reproduce a match.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayFile {
    /// SHA-256 hex of the map JSON.
    pub map_hash: String,
    /// Name of the map used.
    pub map_name: String,
    /// Embedded map JSON for self-contained replay.
    pub map_json: String,
    /// Base seed for AI order generation.
    pub seed: u64,
    /// Number of factions.
    pub num_factions: u32,
    /// All turn events.
    pub event_log: EventLog,
}
