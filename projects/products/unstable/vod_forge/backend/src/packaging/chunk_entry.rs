use crate::packaging::AssetId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkEntry {
    pub asset_id: AssetId,
    pub offset: u64,
    pub length: u64,
    pub sha256: String,
}
