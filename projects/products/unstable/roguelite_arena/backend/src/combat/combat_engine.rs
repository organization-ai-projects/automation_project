use crate::combat::Ability;
use crate::combat::AbilityId;
use crate::combat::HitResult;
use crate::diagnostics::RogueliteArenaError;
use crate::events::ArenaEvent;
use crate::events::EventLog;
use crate::loot::Item;
use crate::loot::ItemId;
use crate::loot::LootTable;
use crate::model::ArenaState;
use crate::model::Enemy;
use crate::model::EnemyId;
use crate::model::Player;
use crate::model::Wave;
use crate::report::RunHash;
use crate::report::RunReport;
use crate::rng::RngDraw;
use crate::rng::SeededRng;
use crate::scenarios::Scenario;
use crate::scenarios::WaveTemplate;
use crate::snapshot::SnapshotHash;
use crate::snapshot::StateSnapshot;
use crate::time::TickClock;
use std::collections::BTreeMap;

pub(crate) struct CombatEngine;

impl CombatEngine {
    pub(crate) fn run(
        scenario: &Scenario,
        ticks: u64,
        seed: u64,
    ) -> Result<(RunReport, Vec<RngDraw>), RogueliteArenaError> {
        let mut rng = SeededRng::from_seed(seed);
        let mut rng_draws: Vec<RngDraw> = Vec::new();
        let report = Self::run_inner(scenario, ticks, seed, &mut rng, &mut rng_draws)?;
        Ok((report, rng_draws))
    }

    fn run_inner(
        scenario: &Scenario,
        ticks: u64,
        seed: u64,
        rng: &mut SeededRng,
        rng_draws: &mut Vec<RngDraw>,
    ) -> Result<RunReport, RogueliteArenaError> {
        let loot_table = LootTable::default_table();

        let player = Player {
            hp: scenario.player_hp,
            max_hp: scenario.player_hp,
            attack: scenario.player_attack,
            defense: scenario.player_defense,
            abilities: vec![AbilityId(0)],
            equipped_items: Vec::new(),
        };

        let mut state = ArenaState {
            player,
            current_wave: None,
            wave_index: 0,
            waves_cleared: 0,
            enemies_killed: 0,
            loot_collected: Vec::new(),
            clock: TickClock::new(),
        };

        let mut abilities = vec![Ability {
            id: AbilityId(0),
            name: "Power Strike".to_string(),
            damage_bonus: 5,
            cooldown: 3,
            current_cooldown: 0,
        }];

        let mut event_log = EventLog::default();
        let mut snapshot_hashes: BTreeMap<String, String> = BTreeMap::new();
        let mut enemy_id_counter: u32 = 0;
        let mut item_id_counter: u32 = 0;
        let mut total_damage_dealt: u64 = 0;
        let mut total_damage_taken: u64 = 0;

        for tick_idx in 0..ticks {
            state.clock.tick();
            let current_tick = state.clock.current();

            if !state.player.is_alive() {
                event_log.record(current_tick, ArenaEvent::PlayerDefeated);
                break;
            }

            // Spawn wave if needed
            if state.current_wave.as_ref().map_or(true, |w| w.is_cleared()) {
                if (state.wave_index as usize) < scenario.waves.len() {
                    let template = &scenario.waves[state.wave_index as usize];
                    let wave = Self::spawn_wave(state.wave_index, template, &mut enemy_id_counter);
                    event_log.record(
                        current_tick,
                        ArenaEvent::WaveStarted {
                            wave_index: state.wave_index,
                            enemy_count: wave.enemies.len() as u32,
                        },
                    );
                    state.current_wave = Some(wave);
                    state.wave_index += 1;
                } else if state.current_wave.as_ref().map_or(true, |w| w.is_cleared()) {
                    event_log.record(current_tick, ArenaEvent::AllWavesCleared);
                    break;
                }
            }

            // Cooldown abilities
            for ability in &mut abilities {
                ability.tick_cooldown();
            }

            // Player attacks
            if let Some(ref mut wave) = state.current_wave {
                let alive: Vec<usize> = wave
                    .enemies
                    .iter()
                    .enumerate()
                    .filter(|(_, e)| e.is_alive())
                    .map(|(i, _)| i)
                    .collect();

                if !alive.is_empty() {
                    // Select target deterministically
                    let target_roll = rng.next_u64();
                    let target_idx_resolved = (target_roll % alive.len() as u64) as usize;
                    rng_draws.push(RngDraw {
                        raw_value: target_roll,
                        resolved_index: target_idx_resolved,
                    });
                    let target_enemy_idx = alive[target_idx_resolved];

                    // Check for ability use
                    let mut bonus_damage = 0u32;
                    for ability in &mut abilities {
                        if ability.is_ready() {
                            bonus_damage = ability.damage_bonus;
                            ability.use_ability();
                            event_log.record(
                                current_tick,
                                ArenaEvent::AbilityUsed {
                                    ability_name: ability.name.clone(),
                                },
                            );
                            break;
                        }
                    }

                    // Hit resolution
                    let hit_roll = rng.next_u64();
                    let hit_chance_resolved = (hit_roll % 100) as u32;
                    rng_draws.push(RngDraw {
                        raw_value: hit_roll,
                        resolved_index: hit_chance_resolved as usize,
                    });

                    let enemy = &wave.enemies[target_enemy_idx];
                    let hit_result = Self::resolve_hit(
                        state.player.attack + bonus_damage,
                        enemy.defense,
                        hit_chance_resolved,
                    );

                    let dmg = hit_result.damage_dealt();
                    total_damage_dealt += dmg as u64;

                    if dmg > 0 {
                        let enemy_mut = &mut wave.enemies[target_enemy_idx];
                        enemy_mut.hp = enemy_mut.hp.saturating_sub(dmg);
                        event_log.record(
                            current_tick,
                            ArenaEvent::PlayerAttack {
                                target: enemy_mut.id,
                                result: hit_result,
                            },
                        );

                        if !enemy_mut.is_alive() {
                            state.enemies_killed += 1;
                            event_log.record(
                                current_tick,
                                ArenaEvent::EnemyDefeated {
                                    enemy_id: enemy_mut.id,
                                },
                            );

                            // Loot drop
                            let loot_roll = rng.next_u64();
                            let loot_idx = (loot_roll % loot_table.items.len() as u64) as usize;
                            rng_draws.push(RngDraw {
                                raw_value: loot_roll,
                                resolved_index: loot_idx,
                            });

                            let loot_template = &loot_table.items[loot_idx];
                            let item = Item {
                                id: ItemId(item_id_counter),
                                name: loot_template.name.clone(),
                                attack_bonus: loot_template.attack_bonus,
                                defense_bonus: loot_template.defense_bonus,
                                hp_bonus: loot_template.hp_bonus,
                            };
                            item_id_counter += 1;

                            // Apply item bonuses
                            state.player.attack += item.attack_bonus;
                            state.player.defense += item.defense_bonus;
                            state.player.max_hp += item.hp_bonus;
                            state.player.hp =
                                (state.player.hp + item.hp_bonus).min(state.player.max_hp);
                            state.player.equipped_items.push(item.id);

                            event_log.record(
                                current_tick,
                                ArenaEvent::LootDropped {
                                    item_name: item.name.clone(),
                                    item_id: item.id,
                                },
                            );
                            state.loot_collected.push(item);
                        }
                    } else {
                        event_log.record(
                            current_tick,
                            ArenaEvent::PlayerAttack {
                                target: wave.enemies[target_enemy_idx].id,
                                result: hit_result,
                            },
                        );
                    }

                    // Check wave cleared
                    if wave.is_cleared() {
                        state.waves_cleared += 1;
                        event_log.record(
                            current_tick,
                            ArenaEvent::WaveCleared {
                                wave_index: wave.index,
                            },
                        );
                    }
                }
            }

            // Enemies attack player
            if let Some(ref wave) = state.current_wave {
                let alive_enemies: Vec<(EnemyId, u32, u32)> = wave
                    .enemies
                    .iter()
                    .filter(|e| e.is_alive())
                    .map(|e| (e.id, e.attack, e.defense))
                    .collect();

                for (enemy_id, enemy_attack, _enemy_defense) in alive_enemies {
                    let enemy_hit_roll = rng.next_u64();
                    let enemy_hit_resolved = (enemy_hit_roll % 100) as u32;
                    rng_draws.push(RngDraw {
                        raw_value: enemy_hit_roll,
                        resolved_index: enemy_hit_resolved as usize,
                    });

                    let enemy_result =
                        Self::resolve_hit(enemy_attack, state.player.defense, enemy_hit_resolved);

                    let enemy_dmg = enemy_result.damage_dealt();
                    total_damage_taken += enemy_dmg as u64;
                    state.player.hp = state.player.hp.saturating_sub(enemy_dmg);

                    event_log.record(
                        current_tick,
                        ArenaEvent::EnemyAttack {
                            enemy_id,
                            result: enemy_result,
                        },
                    );
                }
            }

            // Snapshot every 10 ticks
            if (tick_idx + 1) % 10 == 0 {
                let snap = StateSnapshot::take(current_tick, &state);
                let hash = SnapshotHash::compute(&snap);
                snapshot_hashes.insert(format!("tick_{}", current_tick.value()), hash.0);
            }
        }

        let mut report = RunReport {
            ticks_run: ticks,
            scenario_name: scenario.name.clone(),
            seed,
            player_survived: state.player.is_alive(),
            player_final_hp: state.player.hp,
            enemies_killed: state.enemies_killed,
            waves_cleared: state.waves_cleared,
            total_damage_dealt,
            total_damage_taken,
            items_collected: state.loot_collected.len() as u32,
            event_count: event_log.entries.len(),
            snapshot_hashes,
            run_hash: RunHash(String::new()),
        };
        report.compute_hash();
        Ok(report)
    }

    fn spawn_wave(wave_index: u32, template: &WaveTemplate, enemy_id_counter: &mut u32) -> Wave {
        let mut enemies = Vec::new();
        for _ in 0..template.enemy_count {
            enemies.push(Enemy {
                id: EnemyId(*enemy_id_counter),
                hp: template.enemy_hp,
                max_hp: template.enemy_hp,
                attack: template.enemy_attack,
                defense: template.enemy_defense,
            });
            *enemy_id_counter += 1;
        }
        Wave {
            index: wave_index,
            enemies,
        }
    }

    fn resolve_hit(attacker_power: u32, defender_defense: u32, roll: u32) -> HitResult {
        if roll < 10 {
            HitResult::Miss
        } else if roll >= 90 {
            let base = attacker_power.saturating_sub(defender_defense / 2);
            let crit_damage = (base * 2).max(1);
            HitResult::Critical {
                amount: crit_damage,
            }
        } else {
            let damage = attacker_power.saturating_sub(defender_defense).max(1);
            HitResult::Damage { amount: damage }
        }
    }
}
