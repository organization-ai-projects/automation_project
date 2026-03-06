use crate::application::snapshot_building_dto::SnapshotBuildingDto;
use crate::application::snapshot_route_dto::SnapshotRouteDto;
use crate::application::snapshot_service_dto::SnapshotServiceDto;
use crate::snapshot;

#[derive(Debug, Clone, serde::Serialize)]
pub struct SnapshotStateDto {
    pub budget_balance: i64,
    pub buildings: Vec<SnapshotBuildingDto>,
    pub services: Vec<SnapshotServiceDto>,
    pub routes: Vec<SnapshotRouteDto>,
}

impl SnapshotStateDto {
    pub fn from_state(state: &snapshot::state_snapshot::StateSnapshot) -> Self {
        let mut buildings = Vec::new();
        for (tile, building) in &state.buildings {
            buildings.push(SnapshotBuildingDto {
                x: tile.x,
                y: tile.y,
                kind: building.kind,
                zone: building.zone,
                population: building.population,
                happiness: building.happiness,
            });
        }

        let mut services = Vec::new();
        for (tile, service) in &state.service_buildings {
            services.push(SnapshotServiceDto {
                x: tile.x,
                y: tile.y,
                kind: service.kind,
                coverage_radius: service.coverage_radius,
            });
        }

        let mut routes = Vec::new();
        for route in state.routes.values() {
            routes.push(SnapshotRouteDto {
                vehicle_id: route.vehicle_id,
                path: route.path.clone(),
            });
        }

        Self {
            budget_balance: state.budget_balance,
            buildings,
            services,
            routes,
        }
    }
}
