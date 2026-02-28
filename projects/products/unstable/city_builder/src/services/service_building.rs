use super::ServiceKind;
use crate::map::TileId;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceBuilding {
    pub tile: TileId,
    pub kind: ServiceKind,
    pub coverage_radius: u32,
}
