use std::collections::BTreeMap;
use crate::ability::ability::Ability;
use crate::ability::ability_id::AbilityId;
use crate::ability::ability_kind::AbilityKind;
use crate::grid::grid_map::GridMap;
use crate::grid::position::Position;
use crate::rng::seeded_rng::SeededRng;
use crate::turn::action_entry::ActionEntry;
use crate::unit::unit::Unit;
use crate::unit::unit_id::UnitId;

pub struct TacticsAi;

impl TacticsAi {
    /// Deterministic AI decision for a unit.
    /// Strategy:
    /// 1. If an enemy is in ability range, use best offensive ability
    /// 2. If an ally needs healing and we have heal, heal lowest HP ally
    /// 3. Otherwise, move toward nearest enemy, then try to attack
    /// 4. If nothing to do, wait
    pub fn decide(
        unit_id: UnitId,
        units: &[Unit],
        abilities: &BTreeMap<AbilityId, Ability>,
        grid: &GridMap,
        rng: &mut SeededRng,
    ) -> Vec<ActionEntry> {
        let mut actions = Vec::new();
        let unit = match units.iter().find(|u| u.id == unit_id && u.alive) {
            Some(u) => u.clone(),
            None => return actions,
        };

        let unit_abilities: Vec<&Ability> = unit
            .abilities
            .iter()
            .filter_map(|aid| abilities.get(aid))
            .collect();

        let enemies: Vec<&Unit> = units
            .iter()
            .filter(|u| u.alive && u.team != unit.team)
            .collect();

        // Find best attack: sort abilities by power desc, then ability id asc
        let mut attack_abilities: Vec<&Ability> = unit_abilities
            .iter()
            .filter(|a| matches!(a.kind, AbilityKind::MeleeAttack | AbilityKind::RangedAttack))
            .copied()
            .collect();
        attack_abilities.sort_by(|a, b| {
            b.power.cmp(&a.power).then_with(|| a.id.cmp(&b.id))
        });

        // Find reachable targets
        let mut reachable_targets: Vec<(&Unit, &Ability)> = Vec::new();
        for ability in &attack_abilities {
            for enemy in &enemies {
                let dist = unit.position.manhattan_distance(&enemy.position);
                if dist <= ability.range {
                    reachable_targets.push((enemy, ability));
                }
            }
        }

        // Sort by: lowest enemy HP, then lowest enemy id
        reachable_targets.sort_by(|a, b| {
            a.0.hp.cmp(&b.0.hp).then_with(|| a.0.id.cmp(&b.0.id))
        });

        if let Some((target, ability)) = reachable_targets.first() {
            let damage = ability.power + unit.attack;
            actions.push(ActionEntry::UseAbility {
                unit_id,
                ability_id: ability.id,
                target_id: target.id,
                damage,
            });
            return actions;
        }

        // Try healing if ally needs it
        let heal_abilities: Vec<&Ability> = unit_abilities
            .iter()
            .filter(|a| matches!(a.kind, AbilityKind::Heal))
            .copied()
            .collect();

        if !heal_abilities.is_empty() {
            let mut wounded_allies: Vec<&Unit> = units
                .iter()
                .filter(|u| u.alive && u.team == unit.team && u.hp < u.max_hp)
                .collect();
            wounded_allies.sort_by(|a, b| {
                a.hp.cmp(&b.hp).then_with(|| a.id.cmp(&b.id))
            });

            for ally in &wounded_allies {
                for ability in &heal_abilities {
                    let dist = unit.position.manhattan_distance(&ally.position);
                    if dist <= ability.range {
                        actions.push(ActionEntry::UseAbility {
                            unit_id,
                            ability_id: ability.id,
                            target_id: ally.id,
                            damage: ability.power,
                        });
                        return actions;
                    }
                }
            }
        }

        // Move toward nearest enemy
        if !enemies.is_empty() {
            let mut sorted_enemies: Vec<&Unit> = enemies.clone();
            sorted_enemies.sort_by(|a, b| {
                let da = unit.position.manhattan_distance(&a.position);
                let db = unit.position.manhattan_distance(&b.position);
                da.cmp(&db).then_with(|| a.id.cmp(&b.id))
            });

            let target_enemy = sorted_enemies[0];
            let new_pos = Self::step_toward(
                &unit.position,
                &target_enemy.position,
                unit.move_range,
                grid,
                units,
                unit_id,
            );

            if new_pos != unit.position {
                actions.push(ActionEntry::Move {
                    unit_id,
                    from: unit.position,
                    to: new_pos,
                });

                // After moving, check if we can now attack
                for ability in &attack_abilities {
                    let dist = new_pos.manhattan_distance(&target_enemy.position);
                    if dist <= ability.range {
                        let damage = ability.power + unit.attack;
                        actions.push(ActionEntry::UseAbility {
                            unit_id,
                            ability_id: ability.id,
                            target_id: target_enemy.id,
                            damage,
                        });
                        return actions;
                    }
                }
            }
        }

        if actions.is_empty() {
            let _ = rng.draw_u64(&format!("wait_{}", unit_id.0));
            actions.push(ActionEntry::Wait { unit_id });
        }

        actions
    }

    /// Move step-by-step toward a target, one axis at a time, deterministic.
    fn step_toward(
        from: &Position,
        to: &Position,
        move_range: u32,
        grid: &GridMap,
        units: &[Unit],
        self_id: UnitId,
    ) -> Position {
        let mut pos = *from;
        for _ in 0..move_range {
            let dx = (to.x - pos.x).signum();
            let dy = (to.y - pos.y).signum();

            if dx == 0 && dy == 0 {
                break;
            }

            // Try X axis first (deterministic choice), then Y axis
            let candidates = if dx.abs() >= dy.abs() {
                vec![
                    Position::new(pos.x + dx, pos.y),
                    Position::new(pos.x, pos.y + dy),
                ]
            } else {
                vec![
                    Position::new(pos.x, pos.y + dy),
                    Position::new(pos.x + dx, pos.y),
                ]
            };

            let mut moved = false;
            for candidate in candidates {
                if grid.is_walkable(&candidate)
                    && !units.iter().any(|u| u.alive && u.id != self_id && u.position == candidate)
                {
                    pos = candidate;
                    moved = true;
                    break;
                }
            }

            if !moved {
                break;
            }
        }
        pos
    }
}
