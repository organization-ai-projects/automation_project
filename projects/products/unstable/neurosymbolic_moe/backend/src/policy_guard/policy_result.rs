use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyResult {
    pub policy_id: String,
    pub passed: bool,
    pub reason: Option<String>,
}
