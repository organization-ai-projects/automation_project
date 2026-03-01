// projects/products/unstable/digital_pet/backend/src/snapshot/state_snapshot.rs
use crate::events::event_log::EventLog;
use crate::model::pet::Pet;
use crate::needs::needs_state::NeedsState;
use crate::snapshot::snapshot_hash::SnapshotHash;
use crate::time::tick::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    pub tick: u64,
    pub pet_name: String,
    pub species: String,
    pub evolution_stage: u32,
    pub hp: u32,
    pub hunger: u32,
    pub fatigue: u32,
    pub happiness: u32,
    pub discipline: u32,
    pub sick: bool,
    pub event_count: usize,
    pub hash: String,
}

impl StateSnapshot {
    pub fn capture(pet: &Pet, needs: &NeedsState, tick: Tick, log: &EventLog) -> Self {
        let hash = SnapshotHash::compute(pet, needs, tick);
        Self {
            tick: tick.value(),
            pet_name: pet.name.clone(),
            species: pet.species.name.clone(),
            evolution_stage: pet.evolution_stage,
            hp: pet.hp,
            hunger: needs.hunger,
            fatigue: needs.fatigue,
            happiness: needs.happiness,
            discipline: needs.discipline,
            sick: needs.sick,
            event_count: log.len(),
            hash,
        }
    }
}
