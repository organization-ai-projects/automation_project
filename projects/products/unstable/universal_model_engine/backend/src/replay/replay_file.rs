use crate::events::event_log::EventLog;
use crate::model::state::State;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayFile {
    pub seed: u64,
    pub step_count: u64,
    pub state: State,
    pub event_log: EventLog,
}
