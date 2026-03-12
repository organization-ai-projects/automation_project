use serde::{Deserialize, Serialize};

use super::DatasetTrainingSample;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetTrainingShard {
    pub schema_version: u32,
    pub generated_at: u64,
    pub split_seed: u64,
    pub validation_ratio: f64,
    pub shard_index: usize,
    pub total_shards: usize,
    pub train_samples: Vec<DatasetTrainingSample>,
    pub validation_samples: Vec<DatasetTrainingSample>,
}
