use super::zone_kind::ZoneKind;
use crate::map::tile_id::TileId;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Zone {
    pub tile: TileId,
    pub kind: ZoneKind,
}
