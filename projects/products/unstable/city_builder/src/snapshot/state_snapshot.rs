use crate::buildings::Building;
use crate::config::sim_config::SimConfig;
use crate::map::{GridMap, RoadGraph, TileId};
use crate::scenario::scenario::Scenario;
use crate::services::{CoverageMap, ServiceBuilding};
use crate::traffic::{Route, Vehicle};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct StateSnapshot {
    pub grid: GridMap,
    pub road_graph: RoadGraph,
    pub buildings: BTreeMap<TileId, Building>,
    pub service_buildings: BTreeMap<TileId, ServiceBuilding>,
    pub coverage: CoverageMap,
    pub vehicles: BTreeMap<u64, Vehicle>,
    pub routes: BTreeMap<u64, Route>,
    pub budget_balance: i64,
}

impl StateSnapshot {
    pub fn from_scenario(scenario: &Scenario, _config: &SimConfig) -> Self {
        let mut grid = GridMap::new(scenario.grid_width, scenario.grid_height);
        let mut road_graph = RoadGraph::new();

        for iz in &scenario.initial_zones {
            let tile_id = TileId { x: iz.x, y: iz.y };
            if let Some(tile) = grid.get_mut(&tile_id) {
                tile.zone = iz.kind;
            }
        }

        for ir in &scenario.initial_roads {
            let (mut x, mut y) = (ir.x1, ir.y1);
            loop {
                let tile_id = TileId { x, y };
                if let Some(tile) = grid.get_mut(&tile_id) {
                    tile.has_road = true;
                }
                if x < ir.x2 {
                    let next = TileId { x: x + 1, y };
                    road_graph.add_road(&crate::map::Road { from: tile_id, to: next });
                    x += 1;
                } else if x > ir.x2 {
                    let next = TileId { x: x - 1, y };
                    road_graph.add_road(&crate::map::Road { from: tile_id, to: next });
                    x -= 1;
                } else if y < ir.y2 {
                    let next = TileId { x, y: y + 1 };
                    road_graph.add_road(&crate::map::Road { from: tile_id, to: next });
                    y += 1;
                } else if y > ir.y2 {
                    let next = TileId { x, y: y - 1 };
                    road_graph.add_road(&crate::map::Road { from: tile_id, to: next });
                    y -= 1;
                } else {
                    break;
                }
            }
        }

        let mut service_buildings = BTreeMap::new();
        for is in &scenario.initial_services {
            let tile_id = TileId { x: is.x, y: is.y };
            service_buildings.insert(tile_id, ServiceBuilding {
                tile: tile_id,
                kind: is.kind,
                coverage_radius: is.coverage_radius,
            });
        }

        Self {
            grid,
            road_graph,
            buildings: BTreeMap::new(),
            service_buildings,
            coverage: CoverageMap::new(),
            vehicles: BTreeMap::new(),
            routes: BTreeMap::new(),
            budget_balance: 10000,
        }
    }

    pub fn total_population(&self) -> u64 {
        self.buildings.values().map(|b| b.population).sum()
    }
}
