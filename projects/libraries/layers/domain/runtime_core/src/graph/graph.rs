use crate::diagnostics::error::RuntimeError;
use crate::graph::edge::Edge;
use crate::graph::node::Node;
use crate::id::runtime_id::RuntimeId;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Self {
        Self { nodes, edges }
    }

    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }

    /// Returns nodes in a deterministic topological order.
    /// Nodes with equal in-degree are ordered by ascending RuntimeId value.
    /// Returns `RuntimeError::CyclicGraph` if a cycle is detected.
    pub fn topological_order(&self) -> Result<Vec<RuntimeId>, RuntimeError> {
        let mut in_degree: HashMap<RuntimeId, usize> = HashMap::new();
        let mut adjacency: HashMap<RuntimeId, Vec<RuntimeId>> = HashMap::new();

        for node in &self.nodes {
            in_degree.entry(node.id).or_insert(0);
            adjacency.entry(node.id).or_default();
        }

        for edge in &self.edges {
            *in_degree.entry(edge.to).or_insert(0) += 1;
            adjacency.entry(edge.from).or_default().push(edge.to);
        }

        // Kahn's algorithm with deterministic tie-breaking via sorted queue
        let mut queue: VecDeque<RuntimeId> = {
            let mut zero: Vec<RuntimeId> = in_degree
                .iter()
                .filter(|&(_, &deg)| deg == 0)
                .map(|(&id, _)| id)
                .collect();
            zero.sort();
            zero.into()
        };

        let mut order = Vec::with_capacity(self.nodes.len());

        while let Some(current) = queue.pop_front() {
            order.push(current);

            if let Some(neighbors) = adjacency.get(&current) {
                let mut next_ready: Vec<RuntimeId> = Vec::new();
                for &neighbor in neighbors {
                    let deg = in_degree.get_mut(&neighbor).expect("node in adjacency");
                    *deg -= 1;
                    if *deg == 0 {
                        next_ready.push(neighbor);
                    }
                }
                next_ready.sort();
                for id in next_ready {
                    queue.push_back(id);
                }
            }
        }

        if order.len() != self.nodes.len() {
            return Err(RuntimeError::CyclicGraph);
        }

        Ok(order)
    }

    /// Returns `true` if the graph contains a cycle.
    pub fn has_cycle(&self) -> bool {
        self.topological_order().is_err()
    }

    pub fn node_ids(&self) -> HashSet<RuntimeId> {
        self.nodes.iter().map(|n| n.id).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(v: u64) -> RuntimeId {
        RuntimeId::new(v)
    }

    fn node(v: u64) -> Node {
        Node::new(id(v), format!("n{v}"))
    }

    #[test]
    fn empty_graph_has_no_cycle() {
        let g = Graph::new(vec![], vec![]);
        assert!(!g.has_cycle());
    }

    #[test]
    fn linear_chain_topological_order() {
        let g = Graph::new(
            vec![node(1), node(2), node(3)],
            vec![Edge::new(id(1), id(2)), Edge::new(id(2), id(3))],
        );
        let order = g.topological_order().unwrap();
        assert_eq!(order, vec![id(1), id(2), id(3)]);
    }

    #[test]
    fn cycle_detection() {
        let g = Graph::new(
            vec![node(1), node(2), node(3)],
            vec![
                Edge::new(id(1), id(2)),
                Edge::new(id(2), id(3)),
                Edge::new(id(3), id(1)),
            ],
        );
        assert!(g.has_cycle());
        assert!(matches!(
            g.topological_order(),
            Err(RuntimeError::CyclicGraph)
        ));
    }

    #[test]
    fn deterministic_order_with_parallel_nodes() {
        let g = Graph::new(vec![node(3), node(1), node(2)], vec![]);
        let order = g.topological_order().unwrap();
        assert_eq!(order, vec![id(1), id(2), id(3)]);
    }
}
