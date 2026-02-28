#![allow(dead_code)]
use crate::ecs::world::World;
use crate::events::event_log::EventLog;
use crate::schedule::system::System;
use crate::schedule::system_context::SystemContext;
use crate::schedule::system_stage::SystemStage;
use crate::time::logical_clock::LogicalClock;

pub struct SystemScheduler {
    systems: Vec<Box<dyn System>>,
}

impl SystemScheduler {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    pub fn register(&mut self, system: Box<dyn System>) {
        self.systems.push(system);
        self.systems.sort_by_key(|s| (s.stage(), s.id()));
    }

    pub fn run_stage(
        &self,
        stage: SystemStage,
        world: &mut World,
        clock: &LogicalClock,
        event_log: &mut EventLog,
    ) {
        for system in self.systems.iter().filter(|s| s.stage() == stage) {
            let mut ctx = SystemContext::new(world, clock, event_log);
            system.run(&mut ctx);
        }
    }

    pub fn run_tick(&self, world: &mut World, clock: &LogicalClock, event_log: &mut EventLog) {
        for stage in [
            SystemStage::PreTick,
            SystemStage::Tick,
            SystemStage::PostTick,
        ] {
            self.run_stage(stage, world, clock, event_log);
        }
    }
}

impl Default for SystemScheduler {
    fn default() -> Self {
        Self::new()
    }
}
