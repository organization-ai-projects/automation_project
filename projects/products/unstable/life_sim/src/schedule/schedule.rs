use crate::schedule::schedule_slot::ScheduleSlot;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Schedule {
    pub slots: Vec<ScheduleSlot>,
}

impl Schedule {
    #[allow(dead_code)]
    pub fn new(mut slots: Vec<ScheduleSlot>) -> Self {
        slots.sort_by_key(|s| s.start_tick);
        Self { slots }
    }
}
