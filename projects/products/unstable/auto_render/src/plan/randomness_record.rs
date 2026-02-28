use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomnessRecord {
    pub seed: u64,
    pub transcript_ref: Option<String>,
}
