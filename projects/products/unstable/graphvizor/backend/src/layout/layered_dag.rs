use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::graph::GraphDefinition;
use crate::layout::layout_engine::LayoutEngine;
use crate::layout::node_position::NodePosition;

/// Layered DAG layout using topological sort with deterministic tie-breaking.
pub struct LayeredDag;

impl LayoutEngine for LayeredDag {
    fn compute(&self, graph: &GraphDefinition) -> Vec<NodePosition> {
        let canonical = graph.canonicalize();

        // Build adjacency (forward) and compute in-degrees
        let mut in_degree: BTreeMap<&str, usize> = BTreeMap::new();
        let mut adjacency: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();

        for node in &canonical.nodes {
            in_degree.entry(node.id.as_str()).or_insert(0);
            adjacency.entry(node.id.as_str()).or_default();
        }

        for edge in &canonical.edges {
            *in_degree.entry(edge.to.as_str()).or_insert(0) += 1;
            adjacency
                .entry(edge.from.as_str())
                .or_default()
                .insert(edge.to.as_str());
        }

        // Kahn's algorithm with BTreeSet for deterministic ordering
        let mut queue: VecDeque<&str> = VecDeque::new();
        {
            let mut ready: Vec<&str> = in_degree
                .iter()
                .filter(|&(_, deg)| *deg == 0)
                .map(|(&id, _)| id)
                .collect();
            ready.sort();
            for id in ready {
                queue.push_back(id);
            }
        }

        // Assign layers: BFS, each round is a layer
        let mut layer_map: BTreeMap<&str, usize> = BTreeMap::new();
        while let Some(node_id) = queue.pop_front() {
            let current_layer = layer_map.get(node_id).copied().unwrap_or(0);
            layer_map.entry(node_id).or_insert(current_layer);

            if let Some(neighbors) = adjacency.get(node_id) {
                let mut next_ready: Vec<&str> = Vec::new();
                for &neighbor in neighbors {
                    let deg = in_degree.get_mut(neighbor).unwrap();
                    *deg -= 1;
                    // Set layer to max of current assignment and parent+1
                    let new_layer = current_layer + 1;
                    let existing = layer_map.entry(neighbor).or_insert(0);
                    if new_layer > *existing {
                        *existing = new_layer;
                    }
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

        // Group nodes by layer
        let mut layers: BTreeMap<usize, Vec<&str>> = BTreeMap::new();
        for (&id, &layer) in &layer_map {
            layers.entry(layer).or_default().push(id);
        }
        // Sort within each layer for determinism
        for nodes in layers.values_mut() {
            nodes.sort();
        }

        // Assign positions
        let layer_height = 100i64;
        let node_spacing = 150i64;

        let mut positions = Vec::new();
        for (&layer_idx, nodes) in &layers {
            let total_width = (nodes.len() as i64 - 1) * node_spacing;
            let start_x = -total_width / 2;
            for (i, &node_id) in nodes.iter().enumerate() {
                positions.push(NodePosition {
                    id: node_id.to_string(),
                    x: start_x + i as i64 * node_spacing,
                    y: layer_idx as i64 * layer_height,
                });
            }
        }

        // Sort output by id for canonical ordering
        positions.sort_by(|a, b| a.id.cmp(&b.id));
        positions
    }
}
