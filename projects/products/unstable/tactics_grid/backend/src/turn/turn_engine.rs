use std::collections::BTreeMap;
use crate::ability::ability::Ability;
use crate::ability::ability_id::AbilityId;
use crate::ai::tactics_ai::TacticsAi;
use crate::diagnostics::tactics_grid_error::TacticsGridError;
use crate::grid::grid_map::GridMap;
use crate::report::battle_report::BattleReport;
use crate::report::run_hash::RunHash;
use crate::rng::rng_draw::RngDraw;
use crate::rng::seed::Seed;
use crate::rng::seeded_rng::SeededRng;
use crate::scenario::scenario::Scenario;
use crate::snapshot::snapshot_hash::SnapshotHash;
use crate::unit::team::Team;
use crate::unit::unit::Unit;
use crate::unit::unit_id::UnitId;
use super::action_entry::ActionEntry;
use super::initiative::Initiative;

pub struct TurnEngine;

impl TurnEngine {
    pub fn run_battle(
        scenario: &Scenario,
        seed: Seed,
    ) -> Result<(BattleReport, Vec<RngDraw>), TacticsGridError> {
        let config = &scenario.config;
        let grid = GridMap::new(config.grid_width, config.grid_height);
        let mut units: Vec<Unit> = scenario.units.clone();
        let abilities: BTreeMap<AbilityId, Ability> =
            scenario.abilities.iter().map(|a| (a.id, a.clone())).collect();
        let mut rng = SeededRng::new(seed);
        let mut all_actions: Vec<ActionEntry> = Vec::new();
        let mut snapshot_hashes: BTreeMap<String, String> = BTreeMap::new();

        let initial_hash = SnapshotHash::compute_units(&units);
        snapshot_hashes.insert("turn_0".to_string(), initial_hash);

        let mut current_turn: u32 = 0;

        while current_turn < config.max_turns {
            current_turn += 1;

            let order = Initiative::compute_order(&units);
            if order.is_empty() {
                break;
            }

            if Self::check_victory(&units).is_some() {
                break;
            }

            for unit_id in &order {
                let unit_alive = units.iter().any(|u| u.id == *unit_id && u.alive);
                if !unit_alive {
                    continue;
                }

                let actions = TacticsAi::decide(
                    *unit_id,
                    &units,
                    &abilities,
                    &grid,
                    &mut rng,
                );

                for action in actions {
                    Self::apply_action(&action, &mut units, &abilities);
                    all_actions.push(action);
                }

                let defeated: Vec<UnitId> = units
                    .iter()
                    .filter(|u| !u.alive && u.hp == 0)
                    .map(|u| u.id)
                    .collect();

                for did in &defeated {
                    let already = all_actions.iter().any(|a| matches!(a, ActionEntry::Defeated { unit_id } if unit_id == did));
                    if !already {
                        all_actions.push(ActionEntry::Defeated { unit_id: *did });
                    }
                }

                if Self::check_victory(&units).is_some() {
                    break;
                }
            }

            let turn_hash = SnapshotHash::compute_units(&units);
            snapshot_hashes.insert(format!("turn_{current_turn}"), turn_hash);
        }

        let winner = Self::check_victory(&units);
        let draws = rng.into_draws();

        let mut report = BattleReport {
            scenario_name: scenario.name.clone(),
            seed: seed.0,
            turns_played: current_turn,
            winner: winner.map(|t| format!("{t:?}")),
            actions: all_actions,
            snapshot_hashes,
            unit_summaries: units
                .iter()
                .map(|u| crate::report::battle_report::UnitSummary {
                    id: u.id,
                    name: u.name.clone(),
                    team: u.team,
                    alive: u.alive,
                    hp: u.hp,
                    max_hp: u.max_hp,
                })
                .collect(),
            run_hash: RunHash("".to_string()),
        };

        report.compute_hash();

        Ok((report, draws))
    }

    fn apply_action(
        action: &ActionEntry,
        units: &mut [Unit],
        abilities: &BTreeMap<AbilityId, Ability>,
    ) {
        match action {
            ActionEntry::Move { unit_id, to, .. } => {
                if let Some(u) = units.iter_mut().find(|u| u.id == *unit_id) {
                    u.position = *to;
                }
            }
            ActionEntry::UseAbility {
                ability_id,
                target_id,
                damage,
                ..
            } => {
                let ability = abilities.get(ability_id);
                if let Some(ability) = ability {
                    match ability.kind {
                        crate::ability::ability_kind::AbilityKind::Heal => {
                            if let Some(target) = units.iter_mut().find(|u| u.id == *target_id) {
                                target.heal(*damage);
                            }
                        }
                        _ => {
                            if let Some(target) = units.iter_mut().find(|u| u.id == *target_id) {
                                target.take_damage(*damage);
                            }
                        }
                    }
                }
            }
            ActionEntry::Wait { .. } | ActionEntry::Defeated { .. } => {}
        }
    }

    fn check_victory(units: &[Unit]) -> Option<Team> {
        let player_alive = units.iter().any(|u| u.team == Team::Player && u.alive);
        let enemy_alive = units.iter().any(|u| u.team == Team::Enemy && u.alive);

        if !enemy_alive && player_alive {
            Some(Team::Player)
        } else if !player_alive && enemy_alive {
            Some(Team::Enemy)
        } else {
            None
        }
    }
}
