use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExpertStatus {
    Active,
    Inactive,
    Deprecated,
    Error,
}
