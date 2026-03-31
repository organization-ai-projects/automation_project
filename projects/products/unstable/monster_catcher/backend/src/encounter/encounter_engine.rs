use crate::data::data_store::DataStore;
use crate::diagnostics::error::BackendError;
use crate::encounter::encounter_table::EncounterTable;
use crate::model::monster::Monster;
use crate::model::monster_id::MonsterId;
use crate::rng::rng_draw::RngDraw;
use rand::Rng;
use rand::rngs::SmallRng;

pub struct EncounterEngine;

impl EncounterEngine {
    pub fn generate_encounter(
        rng: &mut SmallRng,
        table: &EncounterTable,
        data: &DataStore,
        step: u64,
        next_monster_index: &mut u64,
        draws: &mut Vec<RngDraw>,
    ) -> Result<Monster, BackendError> {
        let total = table.total_weight();
        if total == 0 {
            return Err(BackendError::Engine("encounter table is empty".to_string()));
        }
        let roll: u64 = rng.random_range(0..total as u64);
        draws.push(RngDraw::new(step, "encounter_species", roll, total as u64));

        let mut cumulative = 0u64;
        let mut chosen = &table.entries[0];
        for entry in &table.entries {
            cumulative += entry.weight as u64;
            if roll < cumulative {
                chosen = entry;
                break;
            }
        }

        let level_range = chosen.max_level.saturating_sub(chosen.min_level) + 1;
        let level_roll: u64 = rng.random_range(0..level_range as u64);
        draws.push(RngDraw::new(
            step,
            "encounter_level",
            level_roll,
            level_range as u64,
        ));
        let level = chosen.min_level + level_roll as u32;

        let species = data.get_species(&chosen.species_id).ok_or_else(|| {
            BackendError::Data(format!("species not found: {}", chosen.species_id.0))
        })?;

        let moves = species
            .learnset
            .iter()
            .filter(|e| e.level <= level)
            .map(|e| e.move_id.clone())
            .collect::<Vec<_>>();
        let active_moves: Vec<_> = if moves.len() > 4 {
            moves[moves.len() - 4..].to_vec()
        } else {
            moves
        };

        *next_monster_index += 1;
        let monster = Monster::new(
            MonsterId(format!("wild_{}", *next_monster_index)),
            chosen.species_id.clone(),
            level,
            species.base_hp,
            species.base_attack,
            species.base_defense,
            species.base_speed,
            active_moves,
        );

        Ok(monster)
    }
}
