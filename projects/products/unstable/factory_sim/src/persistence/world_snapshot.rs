use crate::model::world::World;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

/// A deterministic snapshot of the world state at a given tick.
/// Suitable for checksum-based verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSnapshot {
    pub tick: u64,
    /// Sorted map: entity_id -> sorted map: item_name -> count.
    pub inventories: BTreeMap<u64, BTreeMap<String, u64>>,
}

impl WorldSnapshot {
    pub fn from_world(world: &World) -> Self {
        let mut inventories = BTreeMap::new();
        for id in world.machine_ids() {
            if let Some(machine) = world.get_machine(id) {
                let counts: BTreeMap<String, u64> = machine
                    .inventory
                    .counts()
                    .iter()
                    .filter(|(_, v)| **v > 0)
                    .map(|(k, v)| (k.clone(), *v))
                    .collect();
                inventories.insert(id.value(), counts);
            }
        }
        Self {
            tick: world.tick,
            inventories,
        }
    }

    /// Computes a stable SHA-256 checksum over the sorted snapshot representation.
    pub fn checksum(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.tick.to_le_bytes());
        for (entity_id, items) in &self.inventories {
            hasher.update(entity_id.to_le_bytes());
            for (name, count) in items {
                hasher.update(name.as_bytes());
                hasher.update(count.to_le_bytes());
            }
        }
        hex::encode(hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::entity_id::EntityId;
    use crate::model::machine::{Machine, MachineKind};

    #[test]
    fn checksum_is_stable() {
        let mut world = World::new();
        world.add_machine(Machine::new(EntityId::new(1), MachineKind::Sink));
        let s1 = WorldSnapshot::from_world(&world);
        let s2 = WorldSnapshot::from_world(&world);
        assert_eq!(s1.checksum(), s2.checksum());
    }

    #[test]
    fn checksum_changes_with_inventory() {
        let mut world = World::new();
        let m = Machine::new(EntityId::new(1), MachineKind::Sink);
        world.add_machine(m.clone());
        let s1 = WorldSnapshot::from_world(&world);

        world.get_machine_mut(EntityId::new(1)).unwrap()
            .inventory
            .add(&crate::model::item::Item::new("iron"), 5);
        let s2 = WorldSnapshot::from_world(&world);
        assert_ne!(s1.checksum(), s2.checksum());
    }
}
