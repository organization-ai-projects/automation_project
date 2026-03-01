use super::CoverageMap;
use crate::map::TileId;
use crate::snapshot::state_snapshot::StateSnapshot;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct ServiceEngine;

impl ServiceEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn tick(&mut self, state: &mut StateSnapshot) {
        let mut coverage = CoverageMap::new();

        let mut service_buildings: Vec<_> = state.service_buildings.values().cloned().collect();
        service_buildings.sort_by_key(|s| s.tile);

        for sb in &service_buildings {
            let covered = Self::bfs_coverage(state, sb.tile, sb.coverage_radius);
            coverage.covered.entry(sb.kind).or_default().extend(covered);
        }

        state.coverage = coverage;
    }

    fn bfs_coverage(state: &StateSnapshot, start: TileId, radius: u32) -> BTreeSet<TileId> {
        let mut visited: BTreeMap<TileId, u32> = BTreeMap::new();
        let mut queue = VecDeque::new();
        queue.push_back((start, 0u32));
        visited.insert(start, 0);

        while let Some((cur, dist)) = queue.pop_front() {
            if dist >= radius {
                continue;
            }
            let neighbors = state.grid.neighbors(&cur);
            for n in neighbors {
                if !visited.contains_key(&n) {
                    visited.insert(n, dist + 1);
                    queue.push_back((n, dist + 1));
                }
            }
        }

        visited.into_keys().collect()
    }
}

impl Default for ServiceEngine {
    fn default() -> Self {
        Self::new()
    }
}
