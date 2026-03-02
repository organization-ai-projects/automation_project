use crate::snapshot::state_snapshot::StateSnapshot;

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
                let path = state.road_graph.bfs_path(v.origin, v.destination);
                state.routes.insert(
                    vid,
                    super::Route {
                        vehicle_id: vid,
                        path,
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
