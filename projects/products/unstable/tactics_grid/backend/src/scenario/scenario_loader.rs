use crate::ability::ability::Ability;
use crate::ability::ability_id::AbilityId;
use crate::ability::ability_kind::AbilityKind;
use crate::config::battle_config::BattleConfig;
use crate::grid::position::Position;
use crate::unit::team::Team;
use crate::unit::unit::Unit;
use crate::unit::unit_id::UnitId;
use super::scenario::Scenario;
use crate::diagnostics::tactics_grid_error::TacticsGridError;
use std::path::Path;

pub struct ScenarioLoader;

impl ScenarioLoader {
    pub fn load_from_file(path: &Path) -> Result<Scenario, TacticsGridError> {
        let data = std::fs::read_to_string(path)?;
        let scenario: Scenario = common_json::from_str(&data)?;
        Ok(scenario)
    }

    pub fn default_scenario(name: &str) -> Result<Scenario, TacticsGridError> {
        match name {
            "skirmish" => Ok(Self::skirmish()),
            _ => Err(TacticsGridError::InvalidScenario(format!(
                "unknown scenario: {name}"
            ))),
        }
    }

    fn skirmish() -> Scenario {
        let abilities = vec![
            Ability {
                id: AbilityId(1),
                name: "Slash".to_string(),
                kind: AbilityKind::MeleeAttack,
                range: 1,
                power: 8,
            },
            Ability {
                id: AbilityId(2),
                name: "Arrow".to_string(),
                kind: AbilityKind::RangedAttack,
                range: 4,
                power: 6,
            },
            Ability {
                id: AbilityId(3),
                name: "Heal".to_string(),
                kind: AbilityKind::Heal,
                range: 3,
                power: 5,
            },
        ];

        let units = vec![
            Unit {
                id: UnitId(1),
                name: "Knight".to_string(),
                team: Team::Player,
                position: Position::new(1, 1),
                hp: 30,
                max_hp: 30,
                attack: 10,
                defense: 5,
                speed: 3,
                move_range: 3,
                abilities: vec![AbilityId(1)],
                alive: true,
            },
            Unit {
                id: UnitId(2),
                name: "Archer".to_string(),
                team: Team::Player,
                position: Position::new(1, 3),
                hp: 20,
                max_hp: 20,
                attack: 8,
                defense: 2,
                speed: 5,
                move_range: 4,
                abilities: vec![AbilityId(2)],
                alive: true,
            },
            Unit {
                id: UnitId(3),
                name: "Cleric".to_string(),
                team: Team::Player,
                position: Position::new(0, 2),
                hp: 18,
                max_hp: 18,
                attack: 4,
                defense: 3,
                speed: 4,
                move_range: 3,
                abilities: vec![AbilityId(1), AbilityId(3)],
                alive: true,
            },
            Unit {
                id: UnitId(4),
                name: "Goblin".to_string(),
                team: Team::Enemy,
                position: Position::new(6, 1),
                hp: 15,
                max_hp: 15,
                attack: 7,
                defense: 2,
                speed: 6,
                move_range: 4,
                abilities: vec![AbilityId(1)],
                alive: true,
            },
            Unit {
                id: UnitId(5),
                name: "Orc".to_string(),
                team: Team::Enemy,
                position: Position::new(6, 3),
                hp: 25,
                max_hp: 25,
                attack: 9,
                defense: 4,
                speed: 2,
                move_range: 2,
                abilities: vec![AbilityId(1)],
                alive: true,
            },
            Unit {
                id: UnitId(6),
                name: "Shaman".to_string(),
                team: Team::Enemy,
                position: Position::new(7, 2),
                hp: 16,
                max_hp: 16,
                attack: 5,
                defense: 2,
                speed: 4,
                move_range: 3,
                abilities: vec![AbilityId(2), AbilityId(3)],
                alive: true,
            },
        ];

        Scenario {
            name: "skirmish".to_string(),
            config: BattleConfig::default(),
            units,
            abilities,
        }
    }
}
