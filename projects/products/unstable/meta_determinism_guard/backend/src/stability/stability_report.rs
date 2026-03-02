use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StabilityReport {
    pub stable: bool,
    pub runs: u32,
    pub run_hashes: Vec<String>,
    pub diff: Option<String>,
}
