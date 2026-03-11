use serde::{Deserialize, Serialize};

use super::session_buffer::SessionBuffer;
use super::working_buffer::WorkingBuffer;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferManager {
    working: WorkingBuffer,
    sessions: SessionBuffer,
}

impl BufferManager {
    pub fn new(working_capacity: usize) -> Self {
        Self {
            working: WorkingBuffer::new(working_capacity),
            sessions: SessionBuffer::new(),
        }
    }

    pub fn working(&self) -> &WorkingBuffer {
        &self.working
    }

    pub fn working_mut(&mut self) -> &mut WorkingBuffer {
        &mut self.working
    }

    pub fn sessions(&self) -> &SessionBuffer {
        &self.sessions
    }

    pub fn sessions_mut(&mut self) -> &mut SessionBuffer {
        &mut self.sessions
    }

    pub fn clear_all(&mut self) {
        self.working.clear();
        self.sessions = SessionBuffer::new();
    }
}
