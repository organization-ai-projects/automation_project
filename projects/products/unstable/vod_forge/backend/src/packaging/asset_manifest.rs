use serde::{Deserialize, Serialize};
use crate::packaging::asset_id::AssetId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkEntry {
    pub asset_id: AssetId,
    pub offset: u64,
    pub length: u64,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetManifest {
    pub bundle_id: String,
    pub chunks: Vec<ChunkEntry>,
}

impl AssetManifest {
    /// Serialize to canonical JSON with deterministic field order.
    pub fn to_canonical_json(&self) -> String {
        let chunks: Vec<String> = self
            .chunks
            .iter()
            .map(|c| {
                format!(
                    r#"{{"asset_id":"{}","length":{},"offset":{},"sha256":"{}"}}"#,
                    c.asset_id.0, c.length, c.offset, c.sha256
                )
            })
            .collect();
        format!(
            r#"{{"bundle_id":"{}","chunks":[{}]}}"#,
            self.bundle_id,
            chunks.join(",")
        )
    }
}
