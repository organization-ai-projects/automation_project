use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExpertStatus {
    Active,
    Inactive,
    Deprecated,
    Error,
}
