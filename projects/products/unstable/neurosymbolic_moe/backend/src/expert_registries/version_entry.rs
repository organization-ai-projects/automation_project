use serde::{Deserialize, Serialize};

use crate::moe_core::{ExpertId, ExpertStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionEntry {
    pub expert_id: ExpertId,
    pub version: String,
    pub registered_at: u64,
    pub status: ExpertStatus,
}
