use serde::{Deserialize, Serialize};
use super::{EntityId, Transform};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldEntity {
    pub id: EntityId,
    pub transform: Transform,
    pub name: String,
}
