use crate::map::TileId;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Route {
    pub vehicle_id: u64,
    pub path: Vec<TileId>,
}
