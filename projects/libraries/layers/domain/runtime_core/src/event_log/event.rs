use crate::id::runtime_id::RuntimeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    pub sequence: u64,
    pub job_id: RuntimeId,
    pub node_id: RuntimeId,
}

impl Event {
    pub fn new(sequence: u64, job_id: RuntimeId, node_id: RuntimeId) -> Self {
        Self {
            sequence,
            job_id,
            node_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stores_fields() {
        let e = Event::new(0, RuntimeId::new(0), RuntimeId::new(5));
        assert_eq!(e.sequence, 0);
        assert_eq!(e.job_id, RuntimeId::new(0));
        assert_eq!(e.node_id, RuntimeId::new(5));
    }
}
