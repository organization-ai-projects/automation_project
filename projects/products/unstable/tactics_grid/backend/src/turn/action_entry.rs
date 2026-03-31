use crate::ability::ability_id::AbilityId;
use crate::grid::position::Position;
use crate::unit::unit_id::UnitId;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ActionEntry {
    Move {
        unit_id: UnitId,
        from: Position,
        to: Position,
    },
    UseAbility {
        unit_id: UnitId,
        ability_id: AbilityId,
        target_id: UnitId,
        damage: i32,
    },
    Wait {
        unit_id: UnitId,
    },
    Defeated {
        unit_id: UnitId,
    },
}
