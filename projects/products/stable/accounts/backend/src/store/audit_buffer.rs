// projects/products/stable/accounts/backend/src/store/audit_buffer.rs
use crate::store::account_store_error::AccountStoreError;
use crate::store::audit_buffer_config::AuditBufferConfig;
use crate::store::audit_entry::AuditEntry;
use common_json::to_string;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time::{Duration, interval};

/// Buffered audit log writer that batches writes to reduce I/O overhead
pub struct AuditBuffer {
    audit_path: PathBuf,
    buffer: Arc<Mutex<Vec<AuditEntry>>>,
    flush_lock: Arc<Mutex<()>>,
    pending_in_flight: Arc<AtomicUsize>,
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
        let flush_lock = Arc::new(Mutex::new(()));
        let pending_in_flight = Arc::new(AtomicUsize::new(0));

        // Start periodic flush task
        let buffer_clone = buffer.clone();
        let flush_lock_clone = flush_lock.clone();
        let pending_in_flight_clone = pending_in_flight.clone();
        let audit_path_clone = audit_path.clone();
        let flush_interval = config.flush_interval_secs;

        let flush_task = tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(flush_interval));
            // Skip the first immediate tick
            interval.tick().await;
            loop {
                interval.tick().await;
                if let Err(e) = Self::flush_internal(
                    &buffer_clone,
                    &flush_lock_clone,
                    &pending_in_flight_clone,
                    &audit_path_clone,
                )
                .await
                {
                    tracing::error!("Periodic audit flush failed: {}", e);
                }
            }
        });

        Ok(Self {
            audit_path,
            buffer,
            flush_lock,
            pending_in_flight,
            config,
            flush_task,
        })
    }

    /// Add an audit entry to the buffer
    pub async fn append(&self, entry: AuditEntry) -> Result<(), AccountStoreError> {
        let mut buffer = self.buffer.lock().await;
        let pending_total = buffer.len() + self.pending_in_flight.load(Ordering::Relaxed);
        if pending_total >= self.config.max_pending_entries {
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
        Self::flush_internal(
            &self.buffer,
            &self.flush_lock,
            &self.pending_in_flight,
            &self.audit_path,
        )
        .await
    }

    async fn flush_internal(
        buffer: &Arc<Mutex<Vec<AuditEntry>>>,
        flush_lock: &Arc<Mutex<()>>,
        pending_in_flight: &Arc<AtomicUsize>,
        audit_path: &PathBuf,
    ) -> Result<(), AccountStoreError> {
        // Serialize all flushes while keeping buffer lock free for append().
        let _flush_guard = flush_lock.lock().await;

        // Drain buffered entries while holding the lock briefly.
        let entries = {
            let mut buffer = buffer.lock().await;
            if buffer.is_empty() {
                return Ok(());
            }
            std::mem::take(&mut *buffer)
        };
        // Lock is released here.

        pending_in_flight.fetch_add(entries.len(), Ordering::Relaxed);
        let _in_flight_guard = InFlightGuard {
            counter: pending_in_flight.clone(),
            count: entries.len(),
        };

        let serialized_lines = match Self::serialize_entries(&entries) {
            Ok(lines) => lines,
            Err(e) => {
                Self::requeue_from_index(buffer, entries, 0).await;
                return Err(e);
            }
        };

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

        for (idx, line) in serialized_lines.iter().enumerate() {
            if let Err(e) = file.write_all(line.as_bytes()).await {
                Self::requeue_from_index(buffer, entries, idx).await;
                return Err(AccountStoreError::Io(e));
            }
        }

        Ok(())
    }

    fn serialize_entries(entries: &[AuditEntry]) -> Result<Vec<String>, AccountStoreError> {
        let mut lines = Vec::with_capacity(entries.len());
        for entry in entries {
            let mut line = to_string(entry).map_err(|e| AccountStoreError::Json(e.to_string()))?;
            line.push('\n');
            lines.push(line);
        }
        Ok(lines)
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

struct InFlightGuard {
    counter: Arc<AtomicUsize>,
    count: usize,
}

impl Drop for InFlightGuard {
    fn drop(&mut self) {
        self.counter.fetch_sub(self.count, Ordering::Relaxed);
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
