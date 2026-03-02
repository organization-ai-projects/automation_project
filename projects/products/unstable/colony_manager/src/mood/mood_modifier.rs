use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoodModifier {
    pub source: String,
    pub delta: f32,
}
