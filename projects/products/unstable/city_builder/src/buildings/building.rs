use super::{BuildingId, BuildingKind};
use crate::map::TileId;
use crate::zoning::ZoneKind;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Building {
    pub id: BuildingId,
    pub tile: TileId,
    pub kind: BuildingKind,
    pub zone: ZoneKind,
    pub population: u64,
    pub happiness: i32,
}

impl Building {
    pub fn new(id: BuildingId, tile: TileId, zone: ZoneKind) -> Self {
        let kind = match zone {
            ZoneKind::Residential => BuildingKind::House,
            ZoneKind::Commercial => BuildingKind::Shop,
            ZoneKind::Industrial => BuildingKind::Factory,
            ZoneKind::None => BuildingKind::House,
        };
        let population = match zone {
            ZoneKind::Residential => 10,
            ZoneKind::Commercial => 0,
            ZoneKind::Industrial => 0,
            ZoneKind::None => 0,
        };
        Self {
            id,
            tile,
            kind,
            zone,
            population,
            happiness: 50,
        }
    }
}
