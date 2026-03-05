use crate::services::service_kind::ServiceKind;
use crate::zoning::zone_kind::ZoneKind;

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
