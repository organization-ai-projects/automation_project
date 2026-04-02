use crate::data::move_id::MoveId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum BattleAction {
    UseMove { move_id: MoveId },
    Flee,
}
