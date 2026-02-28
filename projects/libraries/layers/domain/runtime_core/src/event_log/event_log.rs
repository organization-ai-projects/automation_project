use crate::event_log::event::Event;
use crate::scheduler::job::Job;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventLog {
    events: Vec<Event>,
}

impl EventLog {
    pub fn new() -> Self {
        Self::default()
    }

    /// Records an event for the given job execution.
    pub fn record(&mut self, job: &Job) {
        let sequence = self.events.len() as u64;
        self.events.push(Event::new(sequence, job.id, job.node_id));
    }

    pub fn events(&self) -> &[Event] {
        &self.events
    }

    /// Serializes the log to JSON bytes.
    pub fn serialize(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }

    /// Deserializes a log from JSON bytes.
    pub fn deserialize(data: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(data)
    }

    /// Replays the log and returns the ordered sequence of node ids,
    /// asserting the event sequence is intact.
    pub fn replay(&self) -> Vec<crate::id::runtime_id::RuntimeId> {
        let mut ordered = self.events.clone();
        ordered.sort_by_key(|e| e.sequence);
        ordered.into_iter().map(|e| e.node_id).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::id::runtime_id::RuntimeId;
    use crate::scheduler::job::Job;

    #[test]
    fn record_and_query() {
        let mut log = EventLog::new();
        let job = Job::new(RuntimeId::new(0), RuntimeId::new(1));
        log.record(&job);
        assert_eq!(log.events().len(), 1);
        assert_eq!(log.events()[0].node_id, RuntimeId::new(1));
    }

    #[test]
    fn serialize_deserialize_roundtrip() {
        let mut log = EventLog::new();
        log.record(&Job::new(RuntimeId::new(0), RuntimeId::new(10)));
        log.record(&Job::new(RuntimeId::new(1), RuntimeId::new(20)));
        let bytes = log.serialize().unwrap();
        let restored = EventLog::deserialize(&bytes).unwrap();
        assert_eq!(restored.events(), log.events());
    }

    #[test]
    fn replay_returns_node_ids_in_order() {
        let mut log = EventLog::new();
        log.record(&Job::new(RuntimeId::new(0), RuntimeId::new(5)));
        log.record(&Job::new(RuntimeId::new(1), RuntimeId::new(6)));
        let replayed = log.replay();
        assert_eq!(replayed, vec![RuntimeId::new(5), RuntimeId::new(6)]);
    }
}
