use crate::model::World;
use crate::sim::event_log::EventLog;
use crate::time::TickClock;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimState {
    pub world: World,
    pub clock: TickClock,
    pub event_log: EventLog,
    pub seed: u64,
}
