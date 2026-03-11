use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::buffer_entry::BufferEntry;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionBuffer {
    sessions: HashMap<String, HashMap<String, BufferEntry>>,
}

impl SessionBuffer {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    pub fn create_session(&mut self, session_id: impl Into<String>) {
        self.sessions.entry(session_id.into()).or_default();
    }

    pub fn put(&mut self, session_id: &str, key: impl Into<String>, value: impl Into<String>) {
        let session = self.sessions.entry(session_id.to_string()).or_default();
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

    pub fn get(&self, session_id: &str, key: &str) -> Option<&BufferEntry> {
        self.sessions.get(session_id)?.get(key)
    }

    pub fn remove_session(&mut self, session_id: &str) -> bool {
        self.sessions.remove(session_id).is_some()
    }

    pub fn list_sessions(&self) -> Vec<&str> {
        self.sessions.keys().map(|k| k.as_str()).collect()
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
