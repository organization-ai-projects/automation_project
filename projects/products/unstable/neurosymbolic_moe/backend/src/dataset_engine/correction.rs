use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Correction {
    pub entry_id: String,
    pub corrected_output: String,
    pub reason: String,
    pub corrected_at: u64,
}
