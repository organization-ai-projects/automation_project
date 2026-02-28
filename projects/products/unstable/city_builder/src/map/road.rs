use super::TileId;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Road {
    pub from: TileId,
    pub to: TileId,
}
