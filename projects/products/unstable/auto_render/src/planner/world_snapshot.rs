use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSnapshot {
    pub entities_count: usize,
    pub camera_fov: f64,
    pub has_lighting: bool,
}
