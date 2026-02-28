use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LightKind {
    Directional,
    Point,
    Spot,
}
