use serde::{Deserialize, Serialize};

use crate::graph::edge::Edge;
use crate::graph::node::Node;

/// A complete directed graph composed of nodes and edges.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphDefinition {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl GraphDefinition {
    /// Returns a canonically ordered copy (nodes by id, edges by from then to).
    pub fn canonicalize(&self) -> Self {
        let mut nodes = self.nodes.clone();
        nodes.sort();
        let mut edges = self.edges.clone();
        edges.sort();
        Self { nodes, edges }
    }
}
