use super::{EntityId, Transform};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldEntity {
    pub id: EntityId,
    pub transform: Transform,
    pub name: String,
}
