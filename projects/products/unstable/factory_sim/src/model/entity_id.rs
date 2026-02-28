use serde::{Deserialize, Serialize};

/// Unique identifier for a simulation entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct EntityId(u64);

impl EntityId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EntityId({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stores_value() {
        let id = EntityId::new(42);
        assert_eq!(id.value(), 42);
    }

    #[test]
    fn ordering_holds() {
        let a = EntityId::new(1);
        let b = EntityId::new(2);
        assert!(a < b);
    }
}
