use super::ZoneKind;
use crate::map::TileId;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Zone {
    pub tile: TileId,
    pub kind: ZoneKind,
}
