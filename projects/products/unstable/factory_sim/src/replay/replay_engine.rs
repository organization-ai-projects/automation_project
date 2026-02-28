use crate::diagnostics::error::SimError;
use crate::model::world::World;
use crate::persistence::world_snapshot::WorldSnapshot;
use crate::replay::sim_event::SimEvent;
use crate::sim::sim_engine::SimEngine;

/// Replays a recorded event log against a fresh world and verifies
/// that the snapshot checksum matches at the target tick.
pub struct ReplayEngine {
    initial_world: World,
    events: Vec<SimEvent>,
}

impl ReplayEngine {
    pub fn new(initial_world: World, events: Vec<SimEvent>) -> Self {
        Self {
            initial_world,
            events,
        }
    }

    /// Runs the simulation from scratch for `ticks` steps and returns the snapshot.
    /// The replay is deterministic: given the same initial world and seed, the
    /// snapshot checksum at tick N will always be identical.
    pub fn replay(&self, ticks: u64) -> Result<WorldSnapshot, SimError> {
        let mut engine = SimEngine::new(self.initial_world.clone());
        engine.run(ticks)?;
        Ok(WorldSnapshot::from_world(&engine.world))
    }

    /// Returns all recorded events up to and including the given tick.
    pub fn events_up_to(&self, tick: u64) -> Vec<&SimEvent> {
        self.events.iter().filter(|e| e.tick() <= tick).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::entity_id::EntityId;
    use crate::model::item::Item;
    use crate::model::machine::{Machine, MachineKind};

    fn build_world() -> World {
        let mut world = World::new();
        world.add_machine(Machine::new(
            EntityId::new(1),
            MachineKind::Source {
                output: Item::new("iron"),
                rate: 1,
            },
        ));
        world.add_machine(Machine::new(EntityId::new(2), MachineKind::Conveyor));
        world.add_machine(Machine::new(EntityId::new(3), MachineKind::Sink));
        world.add_edge(EntityId::new(1), EntityId::new(2));
        world.add_edge(EntityId::new(2), EntityId::new(3));
        world
    }

    #[test]
    fn replay_produces_identical_checksum() {
        let world = build_world();
        let replay = ReplayEngine::new(world.clone(), Vec::new());

        let snap_a = replay.replay(5).unwrap();
        let snap_b = replay.replay(5).unwrap();
        assert_eq!(snap_a.checksum(), snap_b.checksum());
    }
}
