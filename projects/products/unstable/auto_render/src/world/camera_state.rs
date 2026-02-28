use super::Transform;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraState {
    pub transform: Transform,
    pub fov_deg: f64,
    pub tracking_target: Option<u64>,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            fov_deg: 60.0,
            tracking_target: None,
        }
    }
}
