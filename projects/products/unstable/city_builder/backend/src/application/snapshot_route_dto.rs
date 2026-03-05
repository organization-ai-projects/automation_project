use crate::map;

#[derive(Debug, Clone, serde::Serialize)]
pub struct SnapshotRouteDto {
    pub vehicle_id: u64,
    pub path: Vec<map::tile_id::TileId>,
}
