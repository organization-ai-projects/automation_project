use crate::diagnostics::SpaceEmpireError;
use crate::io::JsonCodec;
use crate::model::SimState;
use crate::time::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    pub tick: Tick,
    pub state_json: String,
}

impl StateSnapshot {
    pub fn from_state(state: &SimState, tick: Tick) -> Result<StateSnapshot, SpaceEmpireError> {
        let state_json = JsonCodec::encode(state)?;
        Ok(StateSnapshot { tick, state_json })
    }
}
