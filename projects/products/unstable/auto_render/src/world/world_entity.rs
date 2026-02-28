use super::{EntityId, Transform};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldEntity {
    pub id: EntityId,
    pub transform: Transform,
    pub name: String,
    pub components: BTreeMap<String, String>,
}
