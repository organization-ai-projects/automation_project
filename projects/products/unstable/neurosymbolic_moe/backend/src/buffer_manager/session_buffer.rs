use std::collections::HashMap;

use protocol::ProtocolId;
use serde::{Deserialize, Serialize};

use super::buffer_entry::BufferEntry;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionBuffer {
    sessions: HashMap<ProtocolId, HashMap<String, BufferEntry>>,
}

impl SessionBuffer {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    pub fn create_session(&mut self, session_id: &ProtocolId) {
        self.sessions.entry(*session_id).or_default();
    }

    pub fn put(
        &mut self,
        session_id: &ProtocolId,
        key: impl Into<String>,
        value: impl Into<String>,
    ) {
        let session = self.sessions.entry(*session_id).or_default();
        let key = key.into();
        let timestamp = session.values().map(|e| e.created_at).max().unwrap_or(0) + 1;

        let entry = BufferEntry {
            key: key.clone(),
            value: value.into(),
            created_at: timestamp,
            task_id: None,
            session_id: Some(session_id.to_string()),
        };
        session.insert(key, entry);
    }

    pub fn get(&self, session_id: &ProtocolId, key: &str) -> Option<&BufferEntry> {
        self.sessions.get(session_id)?.get(key)
    }

    pub fn values(&self, session_id: &ProtocolId) -> Vec<String> {
        self.values_ref(session_id)
    }

    pub fn values_ref(&self, session_id: &ProtocolId) -> Vec<String> {
        let mut entries: Vec<&BufferEntry> = self
            .sessions
            .get(session_id)
            .map(|session| session.values().collect())
            .unwrap_or_default();
        entries.sort_by(|a, b| a.key.cmp(&b.key));
        entries
            .into_iter()
            .map(|entry| entry.value.clone())
            .collect()
    }

    pub fn values_protocol_id(&self, session_id: &ProtocolId) -> Vec<ProtocolId> {
        let mut entries: Vec<&BufferEntry> = self
            .sessions
            .get(session_id)
            .map(|session| session.values().collect())
            .unwrap_or_default();
        entries.sort_by(|a, b| a.key.cmp(&b.key));
        entries
            .into_iter()
            .filter_map(|entry| entry.session_id.as_ref().and_then(|id| id.parse().ok()))
            .collect()
    }

    pub fn remove_session(&mut self, session_id: &ProtocolId) -> bool {
        self.sessions.remove(session_id).is_some()
    }

    pub fn list_sessions(&self) -> Vec<ProtocolId> {
        self.sessions.keys().cloned().collect()
    }

    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }
}

impl Default for SessionBuffer {
    fn default() -> Self {
        Self::new()
    }
}
