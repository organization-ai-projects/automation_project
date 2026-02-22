// projects/products/unstable/autonomy_orchestrator_ai/src/domain/binary_invocation_spec.rs

use crate::domain::Stage;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryInvocationSpec {
    pub stage: Stage,
    pub command: String,
    pub args: Vec<String>,
    pub timeout_ms: u64,
    pub expected_artifacts: Vec<String>,
}
