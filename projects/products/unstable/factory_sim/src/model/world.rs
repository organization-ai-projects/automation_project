use crate::model::entity_id::EntityId;
use crate::model::machine::Machine;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The complete simulation world: all machines, indexed by EntityId.
/// BTreeMap ensures deterministic iteration order.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct World {
    machines: BTreeMap<u64, Machine>,
    /// Directed edges: from -> list of to.
    edges: BTreeMap<u64, Vec<u64>>,
    pub tick: u64,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_machine(&mut self, machine: Machine) {
        self.machines.insert(machine.id.value(), machine);
    }

    pub fn add_edge(&mut self, from: EntityId, to: EntityId) {
        self.edges.entry(from.value()).or_default().push(to.value());
    }

    pub fn get_machine(&self, id: EntityId) -> Option<&Machine> {
        self.machines.get(&id.value())
    }

    pub fn get_machine_mut(&mut self, id: EntityId) -> Option<&mut Machine> {
        self.machines.get_mut(&id.value())
    }

    /// Returns machine ids in deterministic (sorted) order.
    pub fn machine_ids(&self) -> Vec<EntityId> {
        self.machines.keys().copied().map(EntityId::new).collect()
    }

    /// Returns downstream neighbour ids for `from`.
    pub fn neighbors(&self, from: EntityId) -> Vec<EntityId> {
        self.edges
            .get(&from.value())
            .cloned()
            .unwrap_or_default()
            .iter()
            .copied()
            .map(EntityId::new)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::machine::MachineKind;

    #[test]
    fn add_and_get_machine() {
        let mut world = World::new();
        let m = Machine::new(EntityId::new(1), MachineKind::Sink);
        world.add_machine(m);
        assert!(world.get_machine(EntityId::new(1)).is_some());
    }

    #[test]
    fn machine_ids_are_sorted() {
        let mut world = World::new();
        world.add_machine(Machine::new(EntityId::new(3), MachineKind::Sink));
        world.add_machine(Machine::new(EntityId::new(1), MachineKind::Sink));
        world.add_machine(Machine::new(EntityId::new(2), MachineKind::Sink));
        let ids: Vec<u64> = world.machine_ids().iter().map(|id| id.value()).collect();
        assert_eq!(ids, vec![1, 2, 3]);
    }

    #[test]
    fn neighbors_returns_correct_ids() {
        let mut world = World::new();
        world.add_machine(Machine::new(EntityId::new(1), MachineKind::Sink));
        world.add_machine(Machine::new(EntityId::new(2), MachineKind::Sink));
        world.add_edge(EntityId::new(1), EntityId::new(2));
        let neighbors = world.neighbors(EntityId::new(1));
        assert_eq!(neighbors.len(), 1);
        assert_eq!(neighbors[0].value(), 2);
    }
}
