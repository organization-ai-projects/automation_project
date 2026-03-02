use crate::id::runtime_id::RuntimeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Edge {
    pub from: RuntimeId,
    pub to: RuntimeId,
}

impl Edge {
    pub fn new(from: RuntimeId, to: RuntimeId) -> Self {
        Self { from, to }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stores_endpoints() {
        let from = RuntimeId::new(1);
        let to = RuntimeId::new(2);
        let edge = Edge::new(from, to);
        assert_eq!(edge.from, RuntimeId::new(1));
        assert_eq!(edge.to, RuntimeId::new(2));
    }
}
