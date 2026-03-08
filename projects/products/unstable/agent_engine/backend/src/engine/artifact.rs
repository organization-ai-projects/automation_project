//! projects/products/unstable/agent_engine/backend/src/engine/artifact.rs
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct Artifact {
    pub name: String,
    pub kind: String,
    pub content: String,
}
