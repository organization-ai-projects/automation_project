use super::ability_id::AbilityId;
use super::ability_kind::AbilityKind;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Ability {
    pub id: AbilityId,
    pub name: String,
    pub kind: AbilityKind,
    pub range: u32,
    pub power: i32,
}
