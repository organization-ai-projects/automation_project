use crate::model::entity_id::EntityId;
use serde::{Deserialize, Serialize};

/// A conveyor segment connecting two entity nodes.
/// Items placed in a conveyor's inventory are forwarded downstream each tick.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conveyor {
    pub id: EntityId,
    pub from: EntityId,
    pub to: EntityId,
}

impl Conveyor {
    pub fn new(id: EntityId, from: EntityId, to: EntityId) -> Self {
        Self { id, from, to }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conveyor_stores_endpoints() {
        let c = Conveyor::new(EntityId::new(10), EntityId::new(1), EntityId::new(2));
        assert_eq!(c.from.value(), 1);
        assert_eq!(c.to.value(), 2);
    }
}
