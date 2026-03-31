use serde::{Deserialize, Serialize};

use crate::domain::dataset_identifier::DatasetIdentifier;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunMetadata {
    pub seed: u64,
    pub tick_count: u64,
    pub dataset: DatasetIdentifier,
    pub dataset_checksum: String,
}
