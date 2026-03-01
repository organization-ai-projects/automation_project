use crate::model::colony_state::ColonyState;
use crate::time::tick::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    pub tick: Tick,
    pub state: ColonyState,
}

impl StateSnapshot {
    pub fn take(tick: Tick, state: &ColonyState) -> Self {
        Self { tick, state: state.clone() }
    }
}
