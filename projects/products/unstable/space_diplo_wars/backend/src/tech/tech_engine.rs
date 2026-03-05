use crate::model::sim_state::SimState;

pub struct TechEngine;

impl TechEngine {
    /// Apply one tick of tech research progress.
    #[allow(unused_variables)]
    pub fn tick(state: &mut SimState) {
        // Tech engine: research advancement is triggered by completed research queues.
    }
}
