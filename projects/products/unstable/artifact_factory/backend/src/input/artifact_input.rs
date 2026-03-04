use crate::input::artifact_kind::ArtifactKind;
use serde::{Deserialize, Serialize};

/// Represents a single input artifact (report, replay, manifest, schema, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactInput {
    pub path: String,
    pub content: String,
    pub kind: ArtifactKind,
}
