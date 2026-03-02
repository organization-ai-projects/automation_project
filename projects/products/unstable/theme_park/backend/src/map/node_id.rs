#![allow(dead_code)]
use serde::{Deserialize, Serialize};

/// Unique identifier for a path node in the park map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct NodeId(pub u32);

impl NodeId {
    pub fn new(v: u32) -> Self {
        Self(v)
    }

    pub fn value(self) -> u32 {
        self.0
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "node({})", self.0)
    }
}
