use serde::{Deserialize, Serialize};
use super::territory_id::TerritoryId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Territory {
    pub id: TerritoryId,
    pub name: String,
}
