// projects/products/unstable/autonomy_orchestrator_ai/src/domain/binary_invocation_spec.rs

use crate::domain::Stage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BinaryInvocationSpec {
    pub stage: Stage,
    pub command: String,
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
    pub timeout_ms: u64,
    pub expected_artifacts: Vec<String>,
}
