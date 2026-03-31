use crate::combat::action::BattleAction;
use crate::combat::battle_report::BattleReport;
use crate::combat::battle_state::BattleState;
use crate::combat::damage::DamageCalc;
use crate::combat::turn::Turn;
use crate::data::data_store::DataStore;
use crate::data::type_id::TypeId;
use crate::diagnostics::error::BackendError;
use crate::rng::rng_draw::RngDraw;
use rand::Rng;
use rand::rngs::SmallRng;

pub struct CombatEngine;

impl CombatEngine {
    pub fn execute_turn(
        rng: &mut SmallRng,
        state: &mut BattleState,
        player_action: BattleAction,
        data: &DataStore,
        step: u64,
        draws: &mut Vec<RngDraw>,
    ) -> Result<(), BackendError> {
        if state.finished {
            return Err(BackendError::Combat("battle already finished".to_string()));
        }

        let turn_number = state.turns.len() as u32 + 1;

        let enemy_action = Self::pick_enemy_action(rng, &state.enemy_monster, step, draws);

        let mut player_damage = 0u32;
        let mut enemy_damage = 0u32;
        let mut status_applied = Vec::new();

        let player_first = state.player_monster.speed >= state.enemy_monster.speed;

        if player_first {
            player_damage = Self::apply_action(
                rng,
                &player_action,
                &state.player_monster,
                &state.enemy_monster,
                data,
                step,
                "player_damage",
                draws,
                &mut status_applied,
            )?;
            state.enemy_monster.apply_damage(player_damage);

            if !state.enemy_monster.is_fainted() {
                enemy_damage = Self::apply_action(
                    rng,
                    &enemy_action,
                    &state.enemy_monster,
                    &state.player_monster,
                    data,
                    step,
                    "enemy_damage",
                    draws,
                    &mut status_applied,
                )?;
                state.player_monster.apply_damage(enemy_damage);
            }
        } else {
            enemy_damage = Self::apply_action(
                rng,
                &enemy_action,
                &state.enemy_monster,
                &state.player_monster,
                data,
                step,
                "enemy_damage",
                draws,
                &mut status_applied,
            )?;
            state.player_monster.apply_damage(enemy_damage);

            if !state.player_monster.is_fainted() {
                player_damage = Self::apply_action(
                    rng,
                    &player_action,
                    &state.player_monster,
                    &state.enemy_monster,
                    data,
                    step,
                    "player_damage",
                    draws,
                    &mut status_applied,
                )?;
                state.enemy_monster.apply_damage(player_damage);
            }
        }

        let turn = Turn {
            turn_number,
            player_action,
            enemy_action,
            player_damage_dealt: player_damage,
            enemy_damage_dealt: enemy_damage,
            player_hp_after: state.player_monster.current_hp,
            enemy_hp_after: state.enemy_monster.current_hp,
            status_applied,
        };
        state.turns.push(turn);

        if state.enemy_monster.is_fainted() {
            state.finished = true;
            state.player_won = Some(true);
        } else if state.player_monster.is_fainted() {
            state.finished = true;
            state.player_won = Some(false);
        }

        Ok(())
    }

    pub fn build_report(state: &BattleState, xp_gained: u64) -> BattleReport {
        BattleReport {
            battle_id: state.id.clone(),
            turn_count: state.turns.len() as u32,
            player_won: state.player_won.unwrap_or(false),
            turns: state.turns.clone(),
            xp_gained,
        }
    }

    fn pick_enemy_action(
        rng: &mut SmallRng,
        enemy: &crate::model::monster::Monster,
        step: u64,
        draws: &mut Vec<RngDraw>,
    ) -> BattleAction {
        if enemy.moves.is_empty() {
            return BattleAction::Flee;
        }
        let index: u64 = rng.random_range(0..enemy.moves.len() as u64);
        draws.push(RngDraw::new(
            step,
            "enemy_move_pick",
            index,
            enemy.moves.len() as u64,
        ));
        BattleAction::UseMove {
            move_id: enemy.moves[index as usize].clone(),
        }
    }

    fn apply_action(
        rng: &mut SmallRng,
        action: &BattleAction,
        attacker: &crate::model::monster::Monster,
        defender: &crate::model::monster::Monster,
        data: &DataStore,
        step: u64,
        label: &str,
        draws: &mut Vec<RngDraw>,
        _status_applied: &mut Vec<String>,
    ) -> Result<u32, BackendError> {
        match action {
            BattleAction::UseMove { move_id } => {
                let move_data = data.get_move(move_id).ok_or_else(|| {
                    BackendError::Combat(format!("move not found: {}", move_id.0))
                })?;
                let species = data.get_species(&defender.species_id);
                let defender_types: Vec<TypeId> = match species {
                    Some(s) => {
                        let mut types = vec![s.primary_type.clone()];
                        if let Some(ref sec) = s.secondary_type {
                            types.push(sec.clone());
                        }
                        types
                    }
                    None => vec![],
                };
                let damage = DamageCalc::calculate(
                    rng,
                    move_data.power,
                    attacker.attack,
                    defender.defense,
                    attacker.level,
                    &move_data.move_type,
                    &defender_types,
                    &data.type_chart,
                    step,
                    label,
                    draws,
                );
                Ok(damage)
            }
            BattleAction::Flee => Ok(0),
        }
    }
}
