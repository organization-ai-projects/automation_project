use serde::{Deserialize, Serialize};

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
