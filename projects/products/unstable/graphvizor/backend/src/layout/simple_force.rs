use std::collections::BTreeMap;

use crate::graph::GraphDefinition;
use crate::layout::layout_engine::LayoutEngine;
use crate::layout::node_position::NodePosition;

/// Simple force-directed layout with deterministic seed.
pub struct SimpleForce {
    pub seed: u64,
    pub iterations: usize,
}

impl Default for SimpleForce {
    fn default() -> Self {
        Self {
            seed: 42,
            iterations: 50,
        }
    }
}

impl SimpleForce {
    /// Deterministic pseudo-random number generator (splitmix64).
    fn next_random(state: &mut u64) -> f64 {
        *state = state.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = *state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z = z ^ (z >> 31);
        (z as f64) / (u64::MAX as f64)
    }
}

impl LayoutEngine for SimpleForce {
    fn compute(&self, graph: &GraphDefinition) -> Vec<NodePosition> {
        let canonical = graph.canonicalize();
        if canonical.nodes.is_empty() {
            return Vec::new();
        }

        let mut rng_state = self.seed;

        // Initialize positions deterministically based on seed
        let mut positions: BTreeMap<String, (f64, f64)> = BTreeMap::new();
        for node in &canonical.nodes {
            let x = (Self::next_random(&mut rng_state) - 0.5) * 400.0;
            let y = (Self::next_random(&mut rng_state) - 0.5) * 400.0;
            positions.insert(node.id.clone(), (x, y));
        }

        let repulsion = 5000.0f64;
        let attraction = 0.01f64;
        let damping = 0.9f64;

        // Velocity map
        let mut velocities: BTreeMap<String, (f64, f64)> = BTreeMap::new();
        for node in &canonical.nodes {
            velocities.insert(node.id.clone(), (0.0, 0.0));
        }

        let node_ids: Vec<String> = canonical.nodes.iter().map(|n| n.id.clone()).collect();

        for _ in 0..self.iterations {
            let mut forces: BTreeMap<String, (f64, f64)> = BTreeMap::new();
            for id in &node_ids {
                forces.insert(id.clone(), (0.0, 0.0));
            }

            // Repulsion between all pairs
            for i in 0..node_ids.len() {
                for j in (i + 1)..node_ids.len() {
                    let (x1, y1) = positions[&node_ids[i]];
                    let (x2, y2) = positions[&node_ids[j]];
                    let dx = x1 - x2;
                    let dy = y1 - y2;
                    let dist_sq = dx * dx + dy * dy;
                    let dist = dist_sq.sqrt().max(1.0);
                    let force = repulsion / dist_sq.max(1.0);
                    let fx = force * dx / dist;
                    let fy = force * dy / dist;
                    forces.get_mut(&node_ids[i]).unwrap().0 += fx;
                    forces.get_mut(&node_ids[i]).unwrap().1 += fy;
                    forces.get_mut(&node_ids[j]).unwrap().0 -= fx;
                    forces.get_mut(&node_ids[j]).unwrap().1 -= fy;
                }
            }

            // Attraction along edges
            for edge in &canonical.edges {
                if let (Some(&(x1, y1)), Some(&(x2, y2))) =
                    (positions.get(&edge.from), positions.get(&edge.to))
                {
                    let dx = x2 - x1;
                    let dy = y2 - y1;
                    let fx = attraction * dx;
                    let fy = attraction * dy;
                    forces.get_mut(&edge.from).unwrap().0 += fx;
                    forces.get_mut(&edge.from).unwrap().1 += fy;
                    forces.get_mut(&edge.to).unwrap().0 -= fx;
                    forces.get_mut(&edge.to).unwrap().1 -= fy;
                }
            }

            // Apply forces
            for id in &node_ids {
                let (fx, fy) = forces[id];
                let vel = velocities.get_mut(id).unwrap();
                vel.0 = (vel.0 + fx) * damping;
                vel.1 = (vel.1 + fy) * damping;
                let pos = positions.get_mut(id).unwrap();
                pos.0 += vel.0;
                pos.1 += vel.1;
            }
        }

        // Convert to integer positions and sort by id
        let mut result: Vec<NodePosition> = positions
            .into_iter()
            .map(|(id, (x, y))| NodePosition {
                id,
                x: x.round() as i64,
                y: y.round() as i64,
            })
            .collect();
        result.sort_by(|a, b| a.id.cmp(&b.id));
        result
    }
}
