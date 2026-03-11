use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskId(pub String);

impl TaskId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: TaskId,
    pub task_type: TaskType,
    pub input: String,
    pub context: Option<String>,
    pub priority: TaskPriority,
    pub metadata: HashMap<String, String>,
}

impl Task {
    pub fn new(id: impl Into<String>, task_type: TaskType, input: impl Into<String>) -> Self {
        Self {
            id: TaskId::new(id),
            task_type,
            input: input.into(),
            context: None,
            priority: TaskPriority::Normal,
            metadata: HashMap::new(),
        }
    }

    pub fn id(&self) -> &TaskId {
        &self.id
    }

    pub fn task_type(&self) -> &TaskType {
        &self.task_type
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn context(&self) -> Option<&str> {
        self.context.as_deref()
    }

    pub fn priority(&self) -> &TaskPriority {
        &self.priority
    }

    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    pub fn with_priority(mut self, priority: TaskPriority) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_task_with_correct_fields() {
        let task = Task::new("t1", TaskType::CodeGeneration, "write code");
        assert_eq!(task.id().as_str(), "t1");
        assert_eq!(task.input(), "write code");
        assert!(task.context().is_none());
        assert!(matches!(task.priority(), TaskPriority::Normal));
        assert!(task.metadata.is_empty());
    }

    #[test]
    fn with_context_sets_context() {
        let task = Task::new("t2", TaskType::CodeAnalysis, "analyze").with_context("extra ctx");
        assert_eq!(task.context(), Some("extra ctx"));
    }

    #[test]
    fn with_priority_sets_priority() {
        let task =
            Task::new("t3", TaskType::Retrieval, "retrieve").with_priority(TaskPriority::Critical);
        assert!(matches!(task.priority(), TaskPriority::Critical));
    }

    #[test]
    fn with_metadata_adds_entry() {
        let task = Task::new("t4", TaskType::Validation, "validate")
            .with_metadata("key", "value")
            .with_metadata("k2", "v2");
        assert_eq!(task.metadata.get("key").unwrap(), "value");
        assert_eq!(task.metadata.get("k2").unwrap(), "v2");
    }
}
