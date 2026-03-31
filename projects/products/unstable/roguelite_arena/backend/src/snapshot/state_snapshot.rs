use crate::model::ArenaState;
use crate::time::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct StateSnapshot {
    pub(crate) tick: Tick,
    pub(crate) state: ArenaState,
}

impl StateSnapshot {
    pub(crate) fn take(tick: Tick, state: &ArenaState) -> Self {
        Self {
            tick,
            state: state.clone(),
        }
    }
}
