// projects/products/stable/accounts/backend/src/store/audit_buffer.rs
use crate::store::audit_entry::AuditEntry;
use crate::store::account_store_error::AccountStoreError;
use common_json::to_string;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};

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

/// Buffered audit log writer that batches writes to reduce I/O overhead
pub struct AuditBuffer {
    audit_path: PathBuf,
    buffer: Arc<Mutex<Vec<AuditEntry>>>,
    config: AuditBufferConfig,
}

impl AuditBuffer {
    pub fn new(audit_path: PathBuf, config: AuditBufferConfig) -> Self {
        let buffer = Arc::new(Mutex::new(Vec::new()));
        
        // Start periodic flush task
        let buffer_clone = buffer.clone();
        let audit_path_clone = audit_path.clone();
        let flush_interval = config.flush_interval_secs;
        
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(flush_interval));
            // Skip the first immediate tick
            interval.tick().await;
            loop {
                interval.tick().await;
                if let Err(e) = Self::flush_internal(&buffer_clone, &audit_path_clone).await {
                    tracing::error!("Periodic audit flush failed: {}", e);
                }
            }
        });
        
        Self {
            audit_path,
            buffer,
            config,
        }
    }

    /// Add an audit entry to the buffer
    pub async fn append(&self, entry: AuditEntry) -> Result<(), AccountStoreError> {
        let mut buffer = self.buffer.lock().await;
        buffer.push(entry);
        let should_flush = buffer.len() >= self.config.max_batch_size;
        drop(buffer);

        if should_flush {
            self.flush().await?;
        }

        Ok(())
    }

    /// Flush all buffered entries to disk
    pub async fn flush(&self) -> Result<(), AccountStoreError> {
        Self::flush_internal(&self.buffer, &self.audit_path).await
    }

    async fn flush_internal(
        buffer: &Arc<Mutex<Vec<AuditEntry>>>,
        audit_path: &PathBuf,
    ) -> Result<(), AccountStoreError> {
        let mut buffer = buffer.lock().await;
        
        if buffer.is_empty() {
            return Ok(());
        }

        // Serialize all entries
        let mut payload = String::new();
        for entry in buffer.iter() {
            let line = to_string(entry).map_err(|e| AccountStoreError::Json(e.to_string()))?;
            payload.push_str(&line);
            payload.push('\n');
        }

        // Write all entries in one operation
        tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(audit_path)
            .await?
            .write_all(payload.as_bytes())
            .await?;

        // Clear buffer after successful write
        buffer.clear();

        Ok(())
    }
}

impl Drop for AuditBuffer {
    fn drop(&mut self) {
        // Best effort flush on drop
        // We can't use async in Drop, so we use blocking approach
        let buffer = self.buffer.clone();
        let audit_path = self.audit_path.clone();
        
        std::thread::spawn(move || {
            let rt = tokio::runtime::Handle::try_current();
            if let Ok(handle) = rt {
                handle.block_on(async move {
                    if let Err(e) = Self::flush_internal(&buffer, &audit_path).await {
                        eprintln!("Failed to flush audit buffer on drop: {}", e);
                    }
                });
            }
        });
    }
}
