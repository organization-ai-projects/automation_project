// projects/libraries/protocol/src/metadatas/metadata_entry_points.rs
use serde::{Deserialize, Serialize};

use crate::MetadataEntrypoint;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetadataEntrypoints {
    pub ui: Vec<MetadataEntrypoint>,
}
