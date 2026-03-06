use crate::economy::resource_kind::ResourceKind;
use crate::model::sim_state::SimState;
use crate::tech::tech_kind::TechKind;

pub struct TechEngine;

impl TechEngine {
    /// Apply one tick of tech research progress.
    pub fn tick(state: &mut SimState) {
        for empire in state.empires.values_mut() {
            // Passive fallback research path to guarantee tech progression even without queued items.
            let economics_level = empire.tech_tree.level(TechKind::Economics);
            if empire.resources.get(ResourceKind::Research) >= 10
                && economics_level < 10
                && empire.resources.spend(ResourceKind::Research, 10)
            {
                empire.tech_tree.advance(TechKind::Economics);
            }
        }
    }
}
