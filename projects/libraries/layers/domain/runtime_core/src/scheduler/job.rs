use crate::id::runtime_id::RuntimeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Job {
    pub id: RuntimeId,
    pub node_id: RuntimeId,
}

impl Job {
    pub fn new(id: RuntimeId, node_id: RuntimeId) -> Self {
        Self { id, node_id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stores_fields() {
        let job = Job::new(RuntimeId::new(10), RuntimeId::new(1));
        assert_eq!(job.id, RuntimeId::new(10));
        assert_eq!(job.node_id, RuntimeId::new(1));
    }
}
