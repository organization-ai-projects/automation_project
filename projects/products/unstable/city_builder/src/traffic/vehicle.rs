use crate::map::TileId;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct Vehicle {
    pub id: u64,
    pub origin: TileId,
    pub destination: TileId,
}
