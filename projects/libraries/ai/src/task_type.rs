use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    CodeAnalysis,
    Linting,
    Documentation,
    SimpleGeneration,
    ComplexGeneration,
    Refactoring,
    IntentParsing,
}
