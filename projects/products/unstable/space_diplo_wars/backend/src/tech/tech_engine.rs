use crate::economy::resource_kind::ResourceKind;
use crate::model::sim_state::SimState;
use crate::tech::tech_kind::TechKind;

pub struct TechEngine;

impl TechEngine {
    /// Apply one tick of tech research progress.
    pub fn tick(state: &mut SimState) {
        for empire in state.empires.values_mut() {
            // Passive fallback research path to guarantee tech progression even without queued items.
            if empire.resources.get(ResourceKind::Research) >= 10
                && empire.resources.spend(ResourceKind::Research, 10)
            {
                empire.tech_tree.advance(TechKind::Economics);
            }
        }
    }
}
