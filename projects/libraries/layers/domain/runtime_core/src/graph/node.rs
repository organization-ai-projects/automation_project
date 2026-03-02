use crate::id::runtime_id::RuntimeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Node {
    pub id: RuntimeId,
    pub label: String,
}

impl Node {
    pub fn new(id: RuntimeId, label: impl Into<String>) -> Self {
        Self {
            id,
            label: label.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stores_fields() {
        let id = RuntimeId::new(1);
        let node = Node::new(id, "alpha");
        assert_eq!(node.id, RuntimeId::new(1));
        assert_eq!(node.label, "alpha");
    }
}
