// projects/libraries/protocol/src/metadatas/project_metadata.rs
use crate::{MetadataAIHints, MetadataDomain, MetadataEntrypoints, protocol_id::ProtocolId};
use common_time::timestamp_utils::Timestamp;
use serde::{Deserialize, Serialize};

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
    /// Optional CLI binary name for tools that expose a short command (e.g. "va").
    /// When set, `[[bin]] name = cli_name` is the only allowed `[[bin]]` declaration.
    /// Must match exactly the `name` field in `[[bin]]` of `Cargo.toml`.
    #[serde(default)]
    pub cli_name: Option<String>,
}
