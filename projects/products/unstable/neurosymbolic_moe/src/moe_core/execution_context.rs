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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_empty_context() {
        let ctx = ExecutionContext::new(TaskId::new("t1"));
        assert_eq!(ctx.task_id.as_str(), "t1");
        assert!(ctx.retrieved_context.is_empty());
        assert!(ctx.memory_entries.is_empty());
        assert!(ctx.buffer_data.is_empty());
        assert!(ctx.parameters.is_empty());
    }

    #[test]
    fn with_retrieved_context_sets_context() {
        let ctx = ExecutionContext::new(TaskId::new("t1"))
            .with_retrieved_context(vec!["ctx1".into(), "ctx2".into()]);
        assert_eq!(ctx.retrieved_context.len(), 2);
        assert_eq!(ctx.retrieved_context[0], "ctx1");
    }

    #[test]
    fn with_memory_entries_sets_entries() {
        let ctx = ExecutionContext::new(TaskId::new("t1")).with_memory_entries(vec!["mem1".into()]);
        assert_eq!(ctx.memory_entries, vec!["mem1"]);
    }

    #[test]
    fn with_buffer_data_sets_data() {
        let ctx = ExecutionContext::new(TaskId::new("t1")).with_buffer_data(vec!["buf1".into()]);
        assert_eq!(ctx.buffer_data, vec!["buf1"]);
    }

    #[test]
    fn with_parameter_adds_parameter() {
        let ctx = ExecutionContext::new(TaskId::new("t1"))
            .with_parameter("k", "v")
            .with_parameter("k2", "v2");
        assert_eq!(ctx.parameters.get("k").unwrap(), "v");
        assert_eq!(ctx.parameters.get("k2").unwrap(), "v2");
    }
}
