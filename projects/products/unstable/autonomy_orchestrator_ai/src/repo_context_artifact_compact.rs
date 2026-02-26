// projects/products/unstable/autonomy_orchestrator_ai/src/repo_context_artifact_compact.rs
use serde::Deserialize;

use crate::versioning_commands::VersioningCommands;

#[derive(Debug, Deserialize)]
pub struct RepoContextArtifactCompat {
    pub detected_validation_commands: VersioningCommands,
}
