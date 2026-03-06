use crate::model::sim_state::SimState;

/// Applies per-tick economy updates to all empires in the state.
pub struct EconomyEngine;

impl EconomyEngine {
    /// Apply one economy tick: add base production to each empire's wallet.
    pub fn tick(state: &mut SimState) {
        for empire in state.empires.values_mut() {
            // Base per-tick production (1 of each resource per tick)
            use crate::economy::resource_kind::ResourceKind;
            empire.resources.add(ResourceKind::Metal, 1);
            empire.resources.add(ResourceKind::Energy, 1);
            empire.resources.add(ResourceKind::Food, 1);
            empire.resources.add(ResourceKind::Research, 1);
        }
    }
}
