#![allow(dead_code)]
use crate::ecs::world::World;
use crate::events::event_log::EventLog;
use crate::events::kernel_event::KernelEvent;
use crate::time::logical_clock::LogicalClock;

pub struct SystemContext<'a> {
    pub world: &'a mut World,
    pub clock: &'a LogicalClock,
    pub event_log: &'a mut EventLog,
}

impl<'a> SystemContext<'a> {
    pub fn new(world: &'a mut World, clock: &'a LogicalClock, event_log: &'a mut EventLog) -> Self {
        Self {
            world,
            clock,
            event_log,
        }
    }

    pub fn emit(&mut self, event: KernelEvent) {
        self.event_log.push(event);
    }
}
