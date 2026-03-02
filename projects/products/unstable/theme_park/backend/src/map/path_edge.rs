#![allow(dead_code)]
use crate::map::node_id::NodeId;
use serde::{Deserialize, Serialize};

/// Undirected weighted edge between two nodes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathEdge {
    pub from: NodeId,
    pub to: NodeId,
    pub cost: u32,
}

impl PathEdge {
    pub fn new(from: NodeId, to: NodeId, cost: u32) -> Self {
        Self { from, to, cost }
    }
}
