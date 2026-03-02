use crate::services::ServiceKind;
use crate::zoning::ZoneKind;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Scenario {
    pub name: String,
    pub grid_width: u32,
    pub grid_height: u32,
    pub initial_zones: Vec<InitialZone>,
    pub initial_roads: Vec<InitialRoad>,
    pub initial_services: Vec<InitialService>,
    pub scripted_actions: Vec<serde_json::Value>,
    pub checkpoints: Vec<Checkpoint>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InitialZone {
    pub x: u32,
    pub y: u32,
    pub kind: ZoneKind,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InitialRoad {
    pub x1: u32,
    pub y1: u32,
    pub x2: u32,
    pub y2: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InitialService {
    pub x: u32,
    pub y: u32,
    pub kind: ServiceKind,
    pub coverage_radius: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Checkpoint {
    pub tick: u64,
    pub expected_hash: String,
}
