#![allow(dead_code)]
use crate::map::node_id::NodeId;
use crate::map::path_edge::PathEdge;
use crate::map::path_node::PathNode;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Undirected weighted graph of park positions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathGraph {
    pub nodes: BTreeMap<NodeId, PathNode>,
    /// adjacency list: node -> sorted vec of (neighbour, cost)
    adjacency: BTreeMap<NodeId, Vec<(NodeId, u32)>>,
}

impl PathGraph {
    pub fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
            adjacency: BTreeMap::new(),
        }
    }

    pub fn add_node(&mut self, node: PathNode) {
        self.nodes.insert(node.id, node);
    }

    pub fn add_edge(&mut self, edge: PathEdge) {
        self.adjacency
            .entry(edge.from)
            .or_default()
            .push((edge.to, edge.cost));
        self.adjacency
            .entry(edge.to)
            .or_default()
            .push((edge.from, edge.cost));
        // Keep neighbours sorted for determinism (stable tie-break by NodeId).
        if let Some(list) = self.adjacency.get_mut(&edge.from) {
            list.sort_by_key(|&(n, c)| (c, n));
        }
        if let Some(list) = self.adjacency.get_mut(&edge.to) {
            list.sort_by_key(|&(n, c)| (c, n));
        }
    }

    pub fn neighbours(&self, node: NodeId) -> &[(NodeId, u32)] {
        self.adjacency.get(&node).map(|v| v.as_slice()).unwrap_or(&[])
    }

    pub fn contains(&self, node: NodeId) -> bool {
        self.nodes.contains_key(&node)
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

impl Default for PathGraph {
    fn default() -> Self {
        Self::new()
    }
}
