//! projects/products/unstable/agent_engine/backend/src/engine/step_result.rs

use crate::engine::artifact;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct StepResult {
    pub step: String,
    pub success: bool,
    pub artifacts: Vec<artifact::Artifact>,
}
