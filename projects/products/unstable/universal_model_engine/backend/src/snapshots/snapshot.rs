use crate::events::event_log::EventLog;
use crate::model::state::State;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub step_count: u64,
    pub state: State,
    pub events: EventLog,
}

impl Snapshot {
    pub fn from_state(step_count: u64, state: &State, events: &EventLog) -> Self {
        Self {
            step_count,
            state: state.clone(),
            events: events.clone(),
        }
    }
}
