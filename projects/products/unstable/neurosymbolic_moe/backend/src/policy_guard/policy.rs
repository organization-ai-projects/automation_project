use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyType {
    ContentFilter,
    SafetyCheck,
    FormatValidation,
    LengthLimit(usize),
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub id: String,
    pub name: String,
    pub description: String,
    pub policy_type: PolicyType,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyResult {
    pub policy_id: String,
    pub passed: bool,
    pub reason: Option<String>,
}
