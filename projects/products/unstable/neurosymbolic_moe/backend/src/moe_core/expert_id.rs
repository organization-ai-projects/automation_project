use protocol::ProtocolId;
use common::Id128;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) struct ExpertId(pub(crate) ProtocolId);

impl ExpertId {
    pub fn new() -> Self {
        Self(ProtocolId::new(Id128::new(0, None, None)))
    }

    pub fn from_protocol_id(id: ProtocolId) -> Self {
        Self(id)
    }
}

impl fmt::Display for ExpertId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialOrd for ExpertId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ExpertId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .as_inner()
            .as_bytes()
            .cmp(other.0.as_inner().as_bytes())
    }
}
