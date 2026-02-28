use super::TileId;
use crate::zoning::zone_kind::ZoneKind;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tile {
    pub id: TileId,
    pub zone: ZoneKind,
    pub has_road: bool,
}

impl Tile {
    pub fn new(id: TileId) -> Self {
        Self { id, zone: ZoneKind::None, has_road: false }
    }
}
