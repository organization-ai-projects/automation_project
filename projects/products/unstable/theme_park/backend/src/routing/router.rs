#![allow(dead_code)]
use crate::map::node_id::NodeId;
use crate::map::path_graph::PathGraph;
use crate::routing::route::Route;
use std::collections::BTreeMap;

/// Deterministic shortest-path router using Dijkstra.
/// Tie-break: smallest NodeId wins at equal cost.
pub struct Router;

impl Router {
    /// Find the shortest path from `start` to `goal` in `graph`.
    /// Returns `None` if unreachable.
    pub fn find_path(graph: &PathGraph, start: NodeId, goal: NodeId) -> Option<Route> {
        if start == goal {
            return Some(Route::new(vec![start]));
        }
        if !graph.contains(start) || !graph.contains(goal) {
            return None;
        }

        // (cost, node, parent_node)
        // Priority queue implemented as sorted BTreeMap<(cost, NodeId), NodeId>
        // for deterministic tie-breaking on NodeId.
        let mut dist: BTreeMap<NodeId, u32> = BTreeMap::new();
        let mut prev: BTreeMap<NodeId, NodeId> = BTreeMap::new();
        let mut open: std::collections::BTreeSet<(u32, NodeId)> =
            std::collections::BTreeSet::new();

        dist.insert(start, 0);
        open.insert((0, start));

        while let Some((cost, node)) = open.pop_first() {
            if node == goal {
                return Some(Self::reconstruct(start, goal, &prev));
            }
            if dist.get(&node).copied().unwrap_or(u32::MAX) < cost {
                continue;
            }
            for &(neighbour, edge_cost) in graph.neighbours(node) {
                let new_cost = cost.saturating_add(edge_cost);
                let old_cost = dist.get(&neighbour).copied().unwrap_or(u32::MAX);
                if new_cost < old_cost {
                    dist.insert(neighbour, new_cost);
                    prev.insert(neighbour, node);
                    open.insert((new_cost, neighbour));
                }
            }
        }
        None
    }

    fn reconstruct(
        start: NodeId,
        goal: NodeId,
        prev: &BTreeMap<NodeId, NodeId>,
    ) -> Route {
        let mut path = Vec::new();
        let mut cur = goal;
        loop {
            path.push(cur);
            if cur == start {
                break;
            }
            match prev.get(&cur) {
                Some(&p) => cur = p,
                None => break,
            }
        }
        path.reverse();
        Route::new(path)
    }
}
