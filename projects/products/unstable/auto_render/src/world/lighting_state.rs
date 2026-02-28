use super::LightDescriptor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightingState {
    pub ambient_color: [f32; 3],
    pub lights: Vec<LightDescriptor>,
}

impl Default for LightingState {
    fn default() -> Self {
        Self {
            ambient_color: [0.1, 0.1, 0.1],
            lights: vec![],
        }
    }
}
