use crate::data::data_store::DataStore;
use crate::data::move_id::MoveId;
use crate::diagnostics::error::BackendError;
use crate::model::monster::Monster;
use crate::progression::level::LevelTable;
use crate::progression::xp::XpGain;

pub struct ProgressionEngine;

impl ProgressionEngine {
    pub fn award_xp(
        monster: &mut Monster,
        base_yield: u32,
        enemy_level: u32,
        data: &DataStore,
    ) -> Result<(XpGain, Vec<MoveId>), BackendError> {
        let gain = XpGain::compute(base_yield, enemy_level);
        monster.xp += gain.xp_gained;

        let mut new_moves = Vec::new();
        while let Some(new_level) = LevelTable::check_level_up(monster.level, monster.xp) {
            monster.level = new_level;
            let species = data.get_species(&monster.species_id);
            if let Some(s) = species {
                for entry in &s.learnset {
                    if entry.level == new_level && !monster.moves.contains(&entry.move_id) {
                        if monster.moves.len() >= 4 {
                            monster.moves.remove(0);
                        }
                        monster.moves.push(entry.move_id.clone());
                        new_moves.push(entry.move_id.clone());
                    }
                }
                monster.max_hp = compute_stat(s.base_hp, new_level) + new_level + 10;
                monster.attack = compute_stat(s.base_attack, new_level);
                monster.defense = compute_stat(s.base_defense, new_level);
                monster.speed = compute_stat(s.base_speed, new_level);
                monster.current_hp = monster.max_hp;
            }
            if new_level >= 100 {
                break;
            }
        }

        Ok((gain, new_moves))
    }
}

fn compute_stat(base: u32, level: u32) -> u32 {
    (base * 2 * level) / 100 + 5
}
