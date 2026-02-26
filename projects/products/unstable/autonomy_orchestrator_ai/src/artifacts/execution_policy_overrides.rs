use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExecutionPolicyOverrides {
    #[serde(default)]
    pub execution_max_iterations: Option<u32>,
    #[serde(default)]
    pub reviewer_remediation_max_cycles: Option<u32>,
}
