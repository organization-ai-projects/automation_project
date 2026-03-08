//! projects/products/unstable/agent_engine/backend/src/engine/execution_context.rs
use crate::engine::task::Task;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionContext {
    pub task: Task,
    pub output: BTreeMap<String, String>,
    pub logs: Vec<String>,
}

impl ExecutionContext {
    pub fn new(task: Task) -> Self {
        Self {
            task,
            output: BTreeMap::new(),
            logs: Vec::new(),
        }
    }

    pub fn append_log(&mut self, message: impl Into<String>) {
        self.logs.push(message.into());
    }

    pub fn set_output(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.output.insert(key.into(), value.into());
    }
}
