use serde::Deserialize;

use crate::artifacts::ValidationInvocationArtifact;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum VersioningCommands {
    Legacy(Vec<String>),
    Current(Vec<ValidationInvocationArtifact>),
}
