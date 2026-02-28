use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlanSchemaVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}
