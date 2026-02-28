use super::faction_id::FactionId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Faction {
    pub id: FactionId,
    pub name: String,
}
