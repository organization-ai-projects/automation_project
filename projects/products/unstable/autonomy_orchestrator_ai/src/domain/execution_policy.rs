use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionPolicy {
    pub execution_max_iterations: u32,
    pub reviewer_remediation_max_cycles: u32,
}
