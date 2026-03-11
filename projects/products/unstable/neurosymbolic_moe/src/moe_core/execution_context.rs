use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::task::TaskId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContext {
    pub task_id: TaskId,
    pub retrieved_context: Vec<String>,
    pub memory_entries: Vec<String>,
    pub buffer_data: Vec<String>,
    pub parameters: HashMap<String, String>,
}

impl ExecutionContext {
    pub fn new(task_id: TaskId) -> Self {
        Self {
            task_id,
            retrieved_context: Vec::new(),
            memory_entries: Vec::new(),
            buffer_data: Vec::new(),
            parameters: HashMap::new(),
        }
    }

    pub fn with_retrieved_context(mut self, context: Vec<String>) -> Self {
        self.retrieved_context = context;
        self
    }

    pub fn with_memory_entries(mut self, entries: Vec<String>) -> Self {
        self.memory_entries = entries;
        self
    }

    pub fn with_buffer_data(mut self, data: Vec<String>) -> Self {
        self.buffer_data = data;
        self
    }

    pub fn with_parameter(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.parameters.insert(key.into(), value.into());
        self
    }
}
