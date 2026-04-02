use crate::diagnostics::RogueliteArenaError;
use crate::io::JsonCodec;
use crate::scenarios::Scenario;
use crate::scenarios::WaveTemplate;
use std::path::Path;

pub(crate) struct ScenarioLoader;

impl ScenarioLoader {
    pub(crate) fn load(path: &Path) -> Result<Scenario, RogueliteArenaError> {
        let scenario: Scenario = JsonCodec::load(path)?;
        if scenario.waves.is_empty() {
            return Err(RogueliteArenaError::InvalidScenario(
                "scenario must declare at least one wave".to_string(),
            ));
        }
        if scenario.player_hp == 0 {
            return Err(RogueliteArenaError::InvalidScenario(
                "player HP must be > 0".to_string(),
            ));
        }
        Ok(scenario)
    }

    pub(crate) fn default_scenario(name: &str) -> Scenario {
        match name {
            "arena_hard" => Scenario {
                name: "arena_hard".to_string(),
                player_hp: 100,
                player_attack: 12,
                player_defense: 5,
                waves: vec![
                    WaveTemplate {
                        enemy_count: 3,
                        enemy_hp: 30,
                        enemy_attack: 8,
                        enemy_defense: 3,
                    },
                    WaveTemplate {
                        enemy_count: 2,
                        enemy_hp: 50,
                        enemy_attack: 15,
                        enemy_defense: 6,
                    },
                    WaveTemplate {
                        enemy_count: 1,
                        enemy_hp: 100,
                        enemy_attack: 20,
                        enemy_defense: 10,
                    },
                ],
            },
            _ => Scenario {
                name: name.to_string(),
                player_hp: 100,
                player_attack: 10,
                player_defense: 3,
                waves: vec![
                    WaveTemplate {
                        enemy_count: 2,
                        enemy_hp: 20,
                        enemy_attack: 5,
                        enemy_defense: 2,
                    },
                    WaveTemplate {
                        enemy_count: 3,
                        enemy_hp: 25,
                        enemy_attack: 7,
                        enemy_defense: 3,
                    },
                ],
            },
        }
    }
}
