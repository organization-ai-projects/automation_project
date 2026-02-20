// projects/libraries/protocol/src/metadatas/metadata_ai_hints.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetadataAIHints {
    pub primary_language: String,
    pub important_paths: Vec<String>,
    pub config_files: Vec<String>,
}
