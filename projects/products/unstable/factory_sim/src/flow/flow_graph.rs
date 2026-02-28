use crate::model::entity_id::EntityId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A node in the flow graph representing a single entity slot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowNode {
    pub id: EntityId,
}

/// A directed edge in the flow graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowEdge {
    pub from: EntityId,
    pub to: EntityId,
}

/// The flow graph describing how items can travel between entities.
/// Uses BTreeMap for deterministic ordering.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FlowGraph {
    nodes: BTreeMap<u64, FlowNode>,
    edges: Vec<FlowEdge>,
}

impl FlowGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, id: EntityId) {
        self.nodes.insert(id.value(), FlowNode { id });
    }

    pub fn add_edge(&mut self, from: EntityId, to: EntityId) {
        self.edges.push(FlowEdge { from, to });
    }

    pub fn nodes(&self) -> impl Iterator<Item = &FlowNode> {
        self.nodes.values()
    }

    pub fn edges(&self) -> &[FlowEdge] {
        &self.edges
    }

    /// Returns downstream neighbours in insertion order (deterministic per add_edge calls).
    pub fn neighbors(&self, id: EntityId) -> Vec<EntityId> {
        self.edges
            .iter()
            .filter(|e| e.from == id)
            .map(|e| e.to)
            .collect()
    }

    /// Validates that all edge endpoints exist as nodes.
    pub fn is_consistent(&self) -> bool {
        self.edges
            .iter()
            .all(|e| self.nodes.contains_key(&e.from.value()) && self.nodes.contains_key(&e.to.value()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consistent_graph() {
        let mut g = FlowGraph::new();
        g.add_node(EntityId::new(1));
        g.add_node(EntityId::new(2));
        g.add_edge(EntityId::new(1), EntityId::new(2));
        assert!(g.is_consistent());
    }

    #[test]
    fn inconsistent_graph_missing_node() {
        let mut g = FlowGraph::new();
        g.add_node(EntityId::new(1));
        g.add_edge(EntityId::new(1), EntityId::new(99));
        assert!(!g.is_consistent());
    }

    #[test]
    fn neighbors_correct() {
        let mut g = FlowGraph::new();
        g.add_node(EntityId::new(1));
        g.add_node(EntityId::new(2));
        g.add_node(EntityId::new(3));
        g.add_edge(EntityId::new(1), EntityId::new(2));
        g.add_edge(EntityId::new(1), EntityId::new(3));
        let n = g.neighbors(EntityId::new(1));
        assert_eq!(n.len(), 2);
    }
}
