use crate::{snapshot::state_snapshot::StateSnapshot, traffic::route::Route};

#[derive(Debug, Clone)]
pub struct TrafficEngine;

impl TrafficEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn tick(&mut self, state: &mut StateSnapshot) {
        let mut vehicle_ids: Vec<u64> = state.vehicles.keys().copied().collect();
        vehicle_ids.sort();

        for vid in vehicle_ids {
            if let Some(v) = state.vehicles.get(&vid) {
                let v = v.clone();
                let reachable = state.road_graph.reachable_from(&v.origin);
                let path = state.road_graph.bfs_path(v.origin, v.destination);
                state.routes.insert(
                    vid,
                    Route {
                        vehicle_id: vid,
                        path: if path.is_empty() && reachable.contains(&v.destination) {
                            vec![v.origin, v.destination]
                        } else {
                            path
                        },
                    },
                );
            }
        }
    }
}

impl Default for TrafficEngine {
    fn default() -> Self {
        Self::new()
    }
}
