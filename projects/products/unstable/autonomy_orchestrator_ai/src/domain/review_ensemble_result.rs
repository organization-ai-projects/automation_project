use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewEnsembleResult {
    pub passed: bool,
    pub confidence: u8,
    pub reason_codes: Vec<String>,
}
