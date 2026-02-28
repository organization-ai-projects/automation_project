use super::territory_id::TerritoryId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Territory {
    pub id: TerritoryId,
    pub name: String,
}
