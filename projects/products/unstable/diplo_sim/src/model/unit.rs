use serde::{Deserialize, Serialize};
use super::faction_id::FactionId;
use super::unit_id::UnitId;
use crate::map::territory_id::TerritoryId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Unit {
    pub id: UnitId,
    pub faction_id: FactionId,
    pub territory_id: TerritoryId,
}
