use protocol::ProtocolId;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskId(pub ProtocolId);

impl TaskId {
    pub fn new() -> Self {
        Self(ProtocolId::default())
    }

    pub fn from_protocol_id(id: ProtocolId) -> Self {
        Self(id)
    }
}

impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
