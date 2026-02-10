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
        if config.max_batch_size == 0 {
            return Err(AccountStoreError::InvalidConfig(
                "max_batch_size must be greater than 0".to_string(),
            ));
        }
        if config.flush_interval_secs == 0 {
            return Err(AccountStoreError::InvalidConfig(
                "flush_interval_secs must be greater than 0".to_string(),
            ));
        }
        if config.max_pending_entries == 0 {
            return Err(AccountStoreError::InvalidConfig(
                "max_pending_entries must be greater than 0".to_string(),
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
        if buffer.len() >= self.config.max_pending_entries {
            return Err(AccountStoreError::BufferFull {
                max_pending_entries: self.config.max_pending_entries,
            });
        }
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

        let mut file = match tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(audit_path)
            .await
        {
            Ok(file) => file,
            Err(e) => {
                Self::requeue_from_index(buffer, entries, 0).await;
                return Err(AccountStoreError::Io(e));
            }
        };

        for idx in 0..entries.len() {
            let mut line = match to_string(&entries[idx]) {
                Ok(line) => line,
                Err(e) => {
                    Self::requeue_from_index(buffer, entries, idx).await;
                    return Err(AccountStoreError::Json(e.to_string()));
                }
            };
            line.push('\n');

            if let Err(e) = file.write_all(line.as_bytes()).await {
                Self::requeue_from_index(buffer, entries, idx).await;
                return Err(AccountStoreError::Io(e));
            }
        }

        Ok(())
    }

    async fn requeue_from_index(
        buffer: &Arc<Mutex<Vec<AuditEntry>>>,
        mut entries: Vec<AuditEntry>,
        start_idx: usize,
    ) {
        let mut remaining = entries.split_off(start_idx);
        let mut pending = buffer.lock().await;
        remaining.append(&mut *pending);
        *pending = remaining;
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
