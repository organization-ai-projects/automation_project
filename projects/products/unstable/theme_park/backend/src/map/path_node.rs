#![allow(dead_code)]
use crate::map::node_id::NodeId;
use serde::{Deserialize, Serialize};

/// A named position in the park map.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathNode {
    pub id: NodeId,
    pub name: String,
}

impl PathNode {
    pub fn new(id: NodeId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
        }
    }
}
