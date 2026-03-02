use serde::{Deserialize, Serialize};

/// A generic envelope for framing IPC messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<T> {
    pub id: u64,
    pub payload: T,
}
