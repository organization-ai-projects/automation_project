// ai/src/task.rs
use crate::strategy::SolverStrategy;  // ← Correction ici
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

#[derive(Debug, Clone)]
pub struct Task {
    task_type: TaskType,
    input: String,
    context: Option<String>,
}

impl Task {
    pub fn new_code_generation(input: String) -> Self {
        Self {
            task_type: TaskType::SimpleGeneration,
            input,
            context: None,
        }
    }

    pub fn new_code_analysis(input: String) -> Self {
        Self {
            task_type: TaskType::CodeAnalysis,
            input,
            context: None,
        }
    }

    pub fn new_refactoring(input: String, instruction: String) -> Self {
        Self {
            task_type: TaskType::Refactoring,
            input,
            context: Some(instruction),
        }
    }

    pub fn task_type(&self) -> &TaskType {
        &self.task_type
    }

    // Méthode helper pour lib.rs
    pub fn task_type_str(&self) -> &str {
        match self.task_type {
            TaskType::CodeAnalysis => "analysis",
            TaskType::Linting => "linting",
            TaskType::Documentation => "documentation",
            TaskType::SimpleGeneration => "generation",
            TaskType::ComplexGeneration => "generation",
            TaskType::Refactoring => "refactoring",
            TaskType::IntentParsing => "intent",
        }
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn context(&self) -> Option<&str> {
        self.context.as_deref()
    }
}

#[derive(Debug, Clone)]
pub struct TaskResult {
    pub output: String,
    pub confidence: f64,
    pub strategy_used: SolverStrategy,  // ← Plus de crate::solver
    pub metadata: Option<String>,
}