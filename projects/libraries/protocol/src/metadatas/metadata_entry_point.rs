// projects/libraries/protocol/src/metadatas/metadata_entry_point.rs
use crate::protocol_id::ProtocolId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetadataEntrypoint {
    pub id: ProtocolId,
    pub title: String,
    pub role: String,
}
