use serde::{Deserialize, Serialize};

/// Represents a single input artifact (report, replay, manifest, schema, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactInput {
    pub path: String,
    pub content: String,
    pub kind: ArtifactKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactKind {
    Report,
    Replay,
    Manifest,
    ProtocolSchema,
    Unknown,
}
