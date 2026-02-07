// projects/products/stable/accounts/backend/src/store/audit_buffer_config.rs

/// Configuration for audit buffer behavior
#[derive(Clone)]
pub struct AuditBufferConfig {
    /// Maximum number of entries to buffer before flushing
    pub max_batch_size: usize,
    /// Interval in seconds for periodic flush
    pub flush_interval_secs: u64,
}

impl Default for AuditBufferConfig {
    fn default() -> Self {
        Self {
            max_batch_size: 100,
            flush_interval_secs: 5,
        }
    }
}
