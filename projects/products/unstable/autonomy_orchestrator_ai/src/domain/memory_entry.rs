use crate::domain::DecisionReliabilityUpdate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub run_id: String,
    pub run_index: u64,
    pub failure_signature: Option<String>,
    pub terminal_state_code: String,
    pub blocked_reason_codes: Vec<String>,
    pub reliability_updates: Vec<DecisionReliabilityUpdate>,
    pub recorded_at_unix_secs: u64,
}
