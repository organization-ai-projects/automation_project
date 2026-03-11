use serde::{Deserialize, Serialize};

use super::error::ExpertError;
use super::execution_context::ExecutionContext;
use super::output::ExpertOutput;
use super::task::Task;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExpertId(pub String);

impl ExpertId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExpertCapability {
    CodeGeneration,
    CodeTransformation,
    StructureAnalysis,
    GitWorkflow,
    IssuePlanning,
    Routing,
    Retrieval,
    Summarization,
    Evaluation,
    Validation,
    MemoryManagement,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExpertStatus {
    Active,
    Inactive,
    Deprecated,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExpertType {
    Deterministic,
    Symbolic,
    Neural,
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpertMetadata {
    pub id: ExpertId,
    pub name: String,
    pub version: String,
    pub capabilities: Vec<ExpertCapability>,
    pub status: ExpertStatus,
    pub expert_type: ExpertType,
}

pub trait Expert {
    fn id(&self) -> &ExpertId;
    fn metadata(&self) -> &ExpertMetadata;
    fn can_handle(&self, task: &Task) -> bool;
    fn execute(&self, task: &Task, context: &ExecutionContext)
    -> Result<ExpertOutput, ExpertError>;
}
