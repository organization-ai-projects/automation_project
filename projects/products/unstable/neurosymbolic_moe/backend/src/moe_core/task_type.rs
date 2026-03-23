use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    CodeGeneration,
    CodeAnalysis,
    CodeTransformation,
    Refactoring,
    Documentation,
    Planning,
    Retrieval,
    Evaluation,
    Validation,
    Custom(String),
}
