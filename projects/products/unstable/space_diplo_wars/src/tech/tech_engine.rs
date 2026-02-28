use crate::model::sim_state::SimState;

pub struct TechEngine;

impl TechEngine {
    /// Apply one tick of tech research progress.
    pub fn tick(_state: &mut SimState) {
        // Tech engine: research advancement is triggered by completed research queues.
    }
}
