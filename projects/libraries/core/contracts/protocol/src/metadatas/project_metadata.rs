// projects/libraries/protocol/src/metadatas/project_metadata.rs
use serde::{Deserialize, Serialize};

use crate::{MetadataAIHints, MetadataDomain, MetadataEntrypoints, protocol_id::ProtocolId};
use common_time::timestamp_utils::Timestamp;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectMetadata {
    pub schema_version: u32,
    pub generated_at: Timestamp,
    pub id: ProtocolId,
    pub name: String,
    pub kind: String, // "product" | "library"
    pub version: String,
    pub entrypoints: Option<MetadataEntrypoints>,
    pub capabilities: Vec<String>,
    pub domains: Vec<MetadataDomain>,
    pub ai_hints: Option<MetadataAIHints>,
}
