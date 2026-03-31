#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AbilityKind {
    MeleeAttack,
    RangedAttack,
    Heal,
}
