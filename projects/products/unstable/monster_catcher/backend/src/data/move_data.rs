use crate::data::move_id::MoveId;
use crate::data::type_id::TypeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoveData {
    pub id: MoveId,
    pub name: String,
    pub move_type: TypeId,
    pub power: u32,
    pub accuracy: u32,
    pub pp: u32,
    pub effect: MoveEffect,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum MoveEffect {
    Damage,
    StatusInflict { status: String },
    Heal { amount: u32 },
}
