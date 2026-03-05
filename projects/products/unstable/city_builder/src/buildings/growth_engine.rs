use super::building::Building;
use super::building_id::BuildingId;
use crate::config::sim_config::SimConfig;
use crate::map::tile_id::TileId;
use crate::snapshot::state_snapshot::StateSnapshot;
use crate::time::tick::Tick;
use crate::zoning::zone_kind::ZoneKind;
use crate::zoning::zone_rules::ZoneRules;

#[derive(Debug, Clone)]
pub struct GrowthEngine {
    next_id: u64,
}

impl GrowthEngine {
    pub fn new() -> Self {
        Self { next_id: 1 }
    }

    pub fn tick(&mut self, state: &mut StateSnapshot, tick: Tick, config: &SimConfig) {
        let mut to_build: Vec<TileId> = Vec::new();

        let mut candidates: Vec<TileId> = state.grid.tiles.keys().copied().collect();
        candidates.sort();

        for tile_id in candidates {
            if state.buildings.contains_key(&tile_id) {
                continue;
            }
            let zone = state
                .grid
                .tiles
                .get(&tile_id)
                .map(|t| t.zone)
                .unwrap_or(ZoneKind::None);
            if zone == ZoneKind::None {
                continue;
            }
            let threshold = ZoneRules::growth_threshold(zone, tick.0);
            let neighbors = state.grid.neighbors(&tile_id);
            let has_adjacent_road = neighbors.iter().any(|n| state.road_graph.has_road(n))
                || state.road_graph.has_road(&tile_id);

            if !has_adjacent_road {
                continue;
            }

            let mut seed = config.seed
                ^ (tile_id.x as u64 * 31
                    + tile_id.y as u64 * 17
                    + tick.0 * 7
                    + config.grid_width as u64
                    + config.grid_height as u64);
            let r = SimConfig::next_rand(&mut seed);
            if r.is_multiple_of(threshold) {
                to_build.push(tile_id);
            }
        }

        for tile_id in to_build {
            let zone = state
                .grid
                .tiles
                .get(&tile_id)
                .map(|t| t.zone)
                .unwrap_or(ZoneKind::None);
            let id = BuildingId(self.next_id);
            self.next_id += 1;
            let building = Building::new(id, tile_id, zone);
            state.buildings.insert(tile_id, building);
        }
    }
}

impl Default for GrowthEngine {
    fn default() -> Self {
        Self::new()
    }
}
