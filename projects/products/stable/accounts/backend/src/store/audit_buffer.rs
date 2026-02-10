// projects/products/stable/accounts/backend/src/store/audit_buffer.rs
use crate::store::account_store_error::AccountStoreError;
use crate::store::audit_buffer_config::AuditBufferConfig;
use crate::store::audit_entry::AuditEntry;
use common_json::to_string;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time::{Duration, interval};

/// Buffered audit log writer that batches writes to reduce I/O overhead
pub struct AuditBuffer {
    audit_path: PathBuf,
    buffer: Arc<Mutex<Vec<AuditEntry>>>,
    config: AuditBufferConfig,
    flush_task: JoinHandle<()>,
}

impl AuditBuffer {
    pub fn new(audit_path: PathBuf, config: AuditBufferConfig) -> Result<Self, AccountStoreError> {
        // Validate config
        if config.flush_interval_secs == 0 {
            return Err(AccountStoreError::InvalidConfig(
                "flush_interval_secs must be greater than 0".to_string(),
            ));
        }

        let buffer = Arc::new(Mutex::new(Vec::new()));

        // Start periodic flush task
        let buffer_clone = buffer.clone();
        let audit_path_clone = audit_path.clone();
        let flush_interval = config.flush_interval_secs;

        let flush_task = tokio::spawn(async move {
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

        Ok(Self {
            audit_path,
            buffer,
            config,
            flush_task,
        })
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
        // Drain buffered entries while holding the lock briefly.
        let entries = {
            let mut buffer = buffer.lock().await;
            if buffer.is_empty() {
                return Ok(());
            }
            std::mem::take(&mut *buffer)
        };
        // Lock is released here.

        // Serialize all entries without holding the lock.
        let payload = match Self::serialize_entries(&entries) {
            Ok(payload) => payload,
            Err(e) => {
                Self::requeue_entries(buffer, entries).await;
                return Err(e);
            }
        };

        // Write all entries in one operation without holding the lock.
        let mut file = match tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(audit_path)
            .await
        {
            Ok(file) => file,
            Err(e) => {
                Self::requeue_entries(buffer, entries).await;
                return Err(AccountStoreError::Io(e));
            }
        };

        if let Err(e) = file.write_all(payload.as_bytes()).await {
            Self::requeue_entries(buffer, entries).await;
            return Err(AccountStoreError::Io(e));
        }

        Ok(())
    }

    fn serialize_entries(entries: &[AuditEntry]) -> Result<String, AccountStoreError> {
        let mut payload = String::new();
        for entry in entries {
            let line = to_string(entry).map_err(|e| AccountStoreError::Json(e.to_string()))?;
            payload.push_str(&line);
            payload.push('\n');
        }
        Ok(payload)
    }

    async fn requeue_entries(buffer: &Arc<Mutex<Vec<AuditEntry>>>, mut entries: Vec<AuditEntry>) {
        let mut pending = buffer.lock().await;
        entries.append(&mut *pending);
        *pending = entries;
    }
}

impl Drop for AuditBuffer {
    fn drop(&mut self) {
        // Cancel the periodic flush task to prevent resource leaks
        self.flush_task.abort();

        // Note: Audit entries may be lost if AuditBuffer is dropped without an explicit
        // flush() call. For guaranteed durability on shutdown, call flush() before
        // dropping the AuditBuffer. The periodic flush task provides some protection
        // against data loss during normal operation.
    }
}
