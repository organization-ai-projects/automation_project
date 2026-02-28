use serde::{Deserialize, Serialize};
use super::{EntityId, LightKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightDescriptor {
    pub entity_id: EntityId,
    pub kind: LightKind,
    pub color: [f32; 3],
    pub intensity: f32,
}
