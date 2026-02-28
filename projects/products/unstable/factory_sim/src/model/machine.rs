use crate::model::entity_id::EntityId;
use crate::model::inventory::Inventory;
use crate::model::item::Item;
use serde::{Deserialize, Serialize};

/// The role a machine plays in the factory graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MachineKind {
    /// Produces items unconditionally each tick.
    Source { output: Item, rate: u64 },
    /// Moves items along edges (no transformation).
    Conveyor,
    /// Consumes items arriving in its inventory.
    Sink,
    /// Transforms inputs into outputs according to a recipe.
    Transformer {
        input: Item,
        input_count: u64,
        output: Item,
        output_count: u64,
    },
}

/// A simulation entity representing any node in the factory flow graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Machine {
    pub id: EntityId,
    pub kind: MachineKind,
    pub inventory: Inventory,
}

impl Machine {
    pub fn new(id: EntityId, kind: MachineKind) -> Self {
        Self {
            id,
            kind,
            inventory: Inventory::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_machine_created() {
        let m = Machine::new(
            EntityId::new(1),
            MachineKind::Source {
                output: Item::new("iron"),
                rate: 2,
            },
        );
        assert_eq!(m.id.value(), 1);
        assert_eq!(m.inventory.total(), 0);
    }

    #[test]
    fn sink_machine_created() {
        let m = Machine::new(EntityId::new(2), MachineKind::Sink);
        assert_eq!(m.id.value(), 2);
    }
}
