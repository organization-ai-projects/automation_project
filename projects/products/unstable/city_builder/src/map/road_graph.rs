use super::{Road, TileId};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RoadGraph {
    pub adjacency: BTreeMap<TileId, BTreeSet<TileId>>,
}

impl RoadGraph {
    pub fn new() -> Self {
        Self {
            adjacency: BTreeMap::new(),
        }
    }

    pub fn add_road(&mut self, road: &Road) {
        self.adjacency.entry(road.from).or_default().insert(road.to);
        self.adjacency.entry(road.to).or_default().insert(road.from);
    }

    pub fn has_road(&self, id: &TileId) -> bool {
        self.adjacency.contains_key(id)
    }

    pub fn reachable_from(&self, start: &TileId) -> BTreeSet<TileId> {
        let mut visited = BTreeSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(*start);
        visited.insert(*start);
        while let Some(cur) = queue.pop_front() {
            if let Some(neighbors) = self.adjacency.get(&cur) {
                for n in neighbors {
                    if visited.insert(*n) {
                        queue.push_back(*n);
                    }
                }
            }
        }
        visited
    }

    pub fn bfs_path(&self, from: TileId, to: TileId) -> Vec<TileId> {
        if from == to {
            return vec![from];
        }
        let mut parent: BTreeMap<TileId, TileId> = BTreeMap::new();
        let mut queue = VecDeque::new();
        queue.push_back(from);
        parent.insert(from, from);
        while let Some(cur) = queue.pop_front() {
            if cur == to {
                let mut path = vec![cur];
                let mut c = cur;
                while c != from {
                    c = parent[&c];
                    path.push(c);
                }
                path.reverse();
                return path;
            }
            if let Some(neighbors) = self.adjacency.get(&cur) {
                for &n in neighbors {
                    if !parent.contains_key(&n) {
                        parent.insert(n, cur);
                        queue.push_back(n);
                    }
                }
            }
        }
        vec![]
    }
}

impl Default for RoadGraph {
    fn default() -> Self {
        Self::new()
    }
}
