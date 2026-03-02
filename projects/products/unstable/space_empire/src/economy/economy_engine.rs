use crate::economy::production::ProductionOutput;
use crate::events::SimEvent;
use crate::model::SimState;

pub struct EconomyEngine;

impl EconomyEngine {
    pub fn tick(state: &mut SimState, events: &mut Vec<SimEvent>) {
        let tick = state.tick;
        for empire in state.empires.values_mut() {
            let mut total_amounts = std::collections::BTreeMap::new();
            for planet in empire.planets.values() {
                let output = ProductionOutput::compute_for_planet(planet);
                for (resource, amount) in &output.per_tick {
                    *total_amounts.entry(*resource).or_insert(0u64) += amount;
                }
            }
            for (&resource, &amount) in &total_amounts {
                empire.wallet.add(resource, amount);
            }
            if !total_amounts.is_empty() {
                events.push(SimEvent::ResourcesProduced {
                    tick,
                    empire_id: empire.empire_id,
                    amounts: total_amounts,
                });
            }
        }
    }
}
