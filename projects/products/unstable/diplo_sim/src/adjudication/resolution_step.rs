use serde::{Deserialize, Serialize};
use crate::map::territory_id::TerritoryId;
use crate::model::unit_id::UnitId;
use crate::time::turn::Turn;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResolutionOutcome {
    Stayed,
    Moved,
    Bounced,
    Supported,
}

/// Records the outcome of resolving a single unit's order in a turn.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolutionStep {
    pub turn: Turn,
    pub unit_id: UnitId,
    pub from: TerritoryId,
    pub to: TerritoryId,
    pub strength: u32,
    pub outcome: ResolutionOutcome,
}
