use crate::map::territory_id::TerritoryId;
use crate::model::unit_id::UnitId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderKind {
    Hold,
    Move {
        target: TerritoryId,
    },
    Support {
        supported_unit_id: UnitId,
        target: TerritoryId,
    },
}
