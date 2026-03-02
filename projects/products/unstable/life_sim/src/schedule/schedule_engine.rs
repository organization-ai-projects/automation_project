use crate::actions::ActionKind;
use crate::schedule::schedule::Schedule;
use crate::time::Tick;

pub struct ScheduleEngine;

impl ScheduleEngine {
    pub fn scheduled_action(tick: Tick, schedule: &Schedule) -> Option<ActionKind> {
        schedule
            .slots
            .iter()
            .find(|s| tick >= s.start_tick && tick < s.end_tick)
            .map(|s| s.activity)
    }
}
