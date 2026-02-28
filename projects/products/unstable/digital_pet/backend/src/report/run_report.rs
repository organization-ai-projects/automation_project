// projects/products/unstable/digital_pet/backend/src/report/run_report.rs
use crate::care::care_engine::CareEngine;
use crate::events::event_log::EventLog;
use crate::model::pet::Pet;
use crate::needs::needs_state::NeedsState;
use crate::report::run_hash::RunHash;
use crate::time::tick_clock::TickClock;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunReport {
    pub seed: u64,
    pub final_species: String,
    pub evolution_stage: u32,
    pub total_ticks: u64,
    pub care_mistakes: usize,
    pub final_happiness: u32,
    pub final_discipline: u32,
    pub final_hp: u32,
    pub event_count: usize,
    pub run_hash: String,
}

impl RunReport {
    pub fn generate(pet: &Pet, needs: &NeedsState, clock: &TickClock, log: &EventLog, care: &CareEngine) -> Self {
        let run_hash = RunHash::compute(clock.seed, &pet.species.id.0, pet.evolution_stage, care.mistake_count());
        Self {
            seed: clock.seed,
            final_species: pet.species.name.clone(),
            evolution_stage: pet.evolution_stage,
            total_ticks: clock.current_tick().value(),
            care_mistakes: care.mistake_count(),
            final_happiness: needs.happiness,
            final_discipline: needs.discipline,
            final_hp: pet.hp,
            event_count: log.len(),
            run_hash,
        }
    }
}
