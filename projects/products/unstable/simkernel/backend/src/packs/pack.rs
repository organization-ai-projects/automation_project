#![allow(dead_code)]
use crate::determinism::seed::Seed;
use crate::ecs::world::World;
use crate::events::event_log::EventLog;
use crate::packs::pack_id::PackId;
use crate::packs::pack_kind::PackKind;
use crate::time::logical_clock::LogicalClock;

pub trait Pack: Send + Sync {
    fn id(&self) -> PackId;
    fn kind(&self) -> PackKind;
    fn initialize(&self, world: &mut World, seed: Seed);
    fn tick(&self, world: &mut World, clock: &LogicalClock, event_log: &mut EventLog);
    fn name(&self) -> &str;
}
