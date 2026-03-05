use crate::{buildings, zoning};

#[derive(Debug, Clone, serde::Serialize)]
pub struct SnapshotBuildingDto {
    pub x: u32,
    pub y: u32,
    pub kind: buildings::building_kind::BuildingKind,
    pub zone: zoning::zone_kind::ZoneKind,
    pub population: u64,
    pub happiness: i32,
}
