use crate::events::event_log::EventLog;
use crate::model::inventory::Inventory;
use crate::model::party::Party;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayFile {
    pub seed: u64,
    pub step_count: u64,
    pub party: Party,
    pub inventory: Inventory,
    pub event_log: EventLog,
}
