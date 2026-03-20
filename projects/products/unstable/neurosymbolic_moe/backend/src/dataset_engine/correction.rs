use common_time::Timestamp;
use protocol::ProtocolId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Correction {
    pub entry_id: ProtocolId,
    pub corrected_output: String,
    pub reason: String,
    pub corrected_at: Timestamp,
}
