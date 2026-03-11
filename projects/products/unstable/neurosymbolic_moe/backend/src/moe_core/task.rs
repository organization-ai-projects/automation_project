use std::collections::HashMap;

use super::task_id::TaskId;
use super::task_priority::TaskPriority;
use super::task_type::TaskType;
use serde::{Deserialize, Serialize};

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
