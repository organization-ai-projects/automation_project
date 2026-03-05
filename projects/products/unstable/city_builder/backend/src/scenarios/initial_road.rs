#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InitialRoad {
    pub x1: u32,
    pub y1: u32,
    pub x2: u32,
    pub y2: u32,
}
