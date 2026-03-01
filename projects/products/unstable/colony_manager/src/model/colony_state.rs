use crate::jobs::job_queue::JobQueue;
use crate::map::colony_map::ColonyMap;
use crate::model::colonist::Colonist;
use crate::model::colonist_id::ColonistId;
use crate::model::inventory::Inventory;
use crate::time::tick_clock::TickClock;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColonyState {
    pub colonists: BTreeMap<ColonistId, Colonist>,
    pub inventory: Inventory,
    pub map: ColonyMap,
    pub job_queue: JobQueue,
    pub clock: TickClock,
}

impl ColonyState {
    pub fn new(map: ColonyMap) -> Self {
        Self {
            colonists: BTreeMap::new(),
            inventory: Inventory::default(),
            map,
            job_queue: JobQueue::default(),
            clock: TickClock::new(),
        }
    }
}
