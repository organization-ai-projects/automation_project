// projects/products/unstable/autonomy_orchestrator_ai/src/artifacts/validation_invocation_artifact.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidationInvocationArtifact {
    pub command: String,
    pub args: Vec<String>,
}
