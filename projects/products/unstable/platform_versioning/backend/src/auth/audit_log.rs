// projects/products/unstable/platform_versioning/backend/src/auth/audit_log.rs
use std::sync::{Arc, Mutex};

use crate::auth::AuditEntry;

/// An in-memory audit log for sensitive actions.
///
/// In production this would be flushed to persistent storage. The in-memory
/// implementation is sufficient for the initial version; persistence can be
/// added without changing the API.
#[derive(Clone)]
pub struct AuditLog {
    entries: Arc<Mutex<Vec<AuditEntry>>>,
}

impl AuditLog {
    /// Creates an empty audit log.
    pub fn new() -> Self {
        Self {
            entries: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Records an entry.
    pub fn record(&self, entry: AuditEntry) {
        if let Ok(mut entries) = self.entries.lock() {
            entries.push(entry);
        }
    }

    /// Returns a snapshot of all recorded entries.
    pub fn snapshot(&self) -> Vec<AuditEntry> {
        self.entries.lock().map(|e| e.clone()).unwrap_or_default()
    }
}

impl Default for AuditLog {
    fn default() -> Self {
        Self::new()
    }
}
