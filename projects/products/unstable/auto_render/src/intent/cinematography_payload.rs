use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CinematographyPayload {
    pub subject_description: String,
    pub shot_type: String,
    pub lighting_style: String,
    pub background: String,
    pub fov_deg: Option<f64>,
    pub camera_distance: Option<f64>,
}
