use crate::sim::sim_state::SimState;
use crate::time::tick::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    pub tick: Tick,
    pub state: SimState,
}

impl StateSnapshot {
    pub fn take(tick: Tick, state: &SimState) -> Self {
        Self {
            tick,
            state: state.clone(),
        }
    }
}
