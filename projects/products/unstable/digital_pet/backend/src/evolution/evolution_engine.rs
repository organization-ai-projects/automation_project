// projects/products/unstable/digital_pet/backend/src/evolution/evolution_engine.rs
use crate::care::care_engine::CareEngine;
use crate::events::event_log::EventLog;
use crate::events::sim_event::SimEvent;
use crate::evolution::evolution_tree::EvolutionTree;
use crate::model::pet::Pet;
use crate::needs::needs_state::NeedsState;
use crate::time::tick::Tick;

pub struct EvolutionEngine {
    tree: EvolutionTree,
}

impl EvolutionEngine {
    pub fn new(tree: EvolutionTree) -> Self { Self { tree } }

    pub fn evaluate(&mut self, pet: &mut Pet, needs: &NeedsState, care: &CareEngine, tick: Tick, log: &mut EventLog) {
        let species_id = pet.species.id.0.clone();
        let mistakes = care.mistake_count();
        let ticks = tick.value();
        if let Some(new_species) = self.tree.find_evolution(&species_id, mistakes, ticks, needs.happiness, needs.discipline) {
            log.push(SimEvent::evolved(tick, species_id, new_species.id.0.clone()));
            pet.evolve_to(new_species);
            pet.age_ticks = ticks;
        }
    }
}
