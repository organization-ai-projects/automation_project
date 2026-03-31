use crate::ability::ability::Ability;
use crate::config::battle_config::BattleConfig;
use crate::unit::unit::Unit;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Scenario {
    pub name: String,
    pub config: BattleConfig,
    pub units: Vec<Unit>,
    pub abilities: Vec<Ability>,
}
