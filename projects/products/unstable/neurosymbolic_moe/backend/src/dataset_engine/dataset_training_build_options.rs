use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetTrainingBuildOptions {
    pub generated_at: u64,
    pub validation_ratio: f64,
    pub min_score: Option<f64>,
    pub include_failure_entries: bool,
    pub include_partial_entries: bool,
    pub include_unknown_entries: bool,
    pub require_correction_for_failure: bool,
    pub split_seed: u64,
}

impl Default for DatasetTrainingBuildOptions {
    fn default() -> Self {
        Self {
            generated_at: 0,
            validation_ratio: 0.1,
            min_score: None,
            include_failure_entries: true,
            include_partial_entries: false,
            include_unknown_entries: false,
            require_correction_for_failure: true,
            split_seed: 0,
        }
    }
}
