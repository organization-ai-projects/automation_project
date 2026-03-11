use serde::{Deserialize, Serialize};

use super::policy_type::PolicyType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub id: String,
    pub name: String,
    pub description: String,
    pub policy_type: PolicyType,
    pub active: bool,
}
