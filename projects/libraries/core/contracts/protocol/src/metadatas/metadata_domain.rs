// projects/libraries/protocol/src/metadatas/metadata_domain.rs
use crate::protocol_id::ProtocolId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetadataDomain {
    pub id: ProtocolId,
    pub desc: String,
}
