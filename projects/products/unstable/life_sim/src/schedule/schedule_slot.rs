use crate::actions::ActionKind;
use crate::time::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleSlot {
    pub start_tick: Tick,
    pub end_tick: Tick,
    pub activity: ActionKind,
}
