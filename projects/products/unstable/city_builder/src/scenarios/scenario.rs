use crate::services::service_kind::ServiceKind;
use crate::zoning::zone_kind::ZoneKind;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Scenario {
    pub name: String,
    pub grid_width: u32,
    pub grid_height: u32,
    pub initial_zones: Vec<InitialZone>,
    pub initial_roads: Vec<InitialRoad>,
    pub initial_services: Vec<InitialService>,
    pub scripted_actions: Vec<ScriptedAction>,
    pub checkpoints: Vec<Checkpoint>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ScriptedAction {
    PlaceZone {
        tick: u64,
        x: u32,
        y: u32,
        kind: ZoneKind,
    },
    PlaceRoad {
        tick: u64,
        x1: u32,
        y1: u32,
        x2: u32,
        y2: u32,
    },
    PlaceService {
        tick: u64,
        x: u32,
        y: u32,
        kind: ServiceKind,
        coverage_radius: u32,
    },
    SetTax {
        tick: u64,
        percent: i64,
    },
}

impl ScriptedAction {
    pub fn tick(&self) -> u64 {
        match self {
            Self::PlaceZone { tick, .. }
            | Self::PlaceRoad { tick, .. }
            | Self::PlaceService { tick, .. }
            | Self::SetTax { tick, .. } => *tick,
        }
    }
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
