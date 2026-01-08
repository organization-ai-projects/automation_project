// ai/src/task.rs
use crate::task_type::TaskType;

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
