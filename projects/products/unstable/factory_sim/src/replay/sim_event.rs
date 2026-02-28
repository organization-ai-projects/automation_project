use crate::model::entity_id::EntityId;
use serde::{Deserialize, Serialize};

/// An event representing a state change in the simulation.
/// All world-state mutations must be representable as a SimEvent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimEvent {
    /// A source produced items into its inventory.
    Produced {
        tick: u64,
        entity: EntityId,
        item: String,
        amount: u64,
    },
    /// Items were transferred between two entities.
    Transferred {
        tick: u64,
        from: EntityId,
        to: EntityId,
        item: String,
        amount: u64,
    },
    /// A sink consumed items from its inventory.
    Consumed {
        tick: u64,
        entity: EntityId,
        item: String,
        amount: u64,
    },
}

impl SimEvent {
    pub fn tick(&self) -> u64 {
        match self {
            Self::Produced { tick, .. } => *tick,
            Self::Transferred { tick, .. } => *tick,
            Self::Consumed { tick, .. } => *tick,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produced_tick() {
        let e = SimEvent::Produced {
            tick: 5,
            entity: EntityId::new(1),
            item: "iron".into(),
            amount: 2,
        };
        assert_eq!(e.tick(), 5);
    }
}
