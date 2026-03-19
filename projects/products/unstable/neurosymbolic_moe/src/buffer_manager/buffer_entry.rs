use serde::{Deserialize, Serialize};

use crate::moe_core::TaskId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BufferType {
    Working,
    Session,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferEntry {
    pub key: String,
    pub value: String,
    pub created_at: u64,
    pub task_id: Option<TaskId>,
    pub session_id: Option<String>,
}

impl BufferEntry {
    pub fn new(key: impl Into<String>, value: impl Into<String>, created_at: u64) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            created_at,
            task_id: None,
            session_id: None,
        }
    }

    pub fn with_task_id(mut self, task_id: TaskId) -> Self {
        self.task_id = Some(task_id);
        self
    }

    pub fn with_session_id(mut self, session_id: impl Into<String>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }
}
