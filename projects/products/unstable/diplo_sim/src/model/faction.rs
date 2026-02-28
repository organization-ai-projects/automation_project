use serde::{Deserialize, Serialize};
use super::faction_id::FactionId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Faction {
    pub id: FactionId,
    pub name: String,
}
