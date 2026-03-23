//! projects/products/unstable/neurosymbolic_moe/backend/src/retrieval_engine/chunk.rs
use std::collections::HashMap;

use protocol::ProtocolId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub id: ProtocolId,
    pub content: String,
    pub source: String,
    pub start_offset: usize,
    pub end_offset: usize,
    pub metadata: HashMap<String, String>,
}

impl Chunk {
    pub fn new(
        id: impl Into<ProtocolId>,
        content: impl Into<String>,
        source: impl Into<String>,
        start_offset: usize,
        end_offset: usize,
    ) -> Self {
        Self {
            id: id.into(),
            content: content.into(),
            source: source.into(),
            start_offset,
            end_offset,
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}
