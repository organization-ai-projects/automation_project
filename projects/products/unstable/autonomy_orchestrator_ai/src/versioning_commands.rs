// projects/products/unstable/autonomy_orchestrator_ai/src/versioning_commands.rs
use serde::Deserialize;

use crate::validation_invocation_artifact::ValidationInvocationArtifact;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum VersioningCommands {
    Legacy(Vec<String>),
    Current(Vec<ValidationInvocationArtifact>),
}
