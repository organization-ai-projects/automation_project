use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock, TryLockError};

use crate::memory_engine::MemoryEntry;
use crate::moe_core::{AggregatedOutput, Expert, MoeError, Task};
use crate::orchestrator::{
    GovernanceAuditTrail, GovernanceImportDecision, MoePipeline, MoePipelineBuilder,
};

const READ_LOCK_KIND: &str = "read";
const WRITE_LOCK_KIND: &str = "write";

#[derive(Clone)]
pub struct ConcurrentMoePipeline {
    inner: Arc<RwLock<MoePipeline>>,
    read_lock_acquisitions: Arc<AtomicU64>,
    write_lock_acquisitions: Arc<AtomicU64>,
    read_lock_contention: Arc<AtomicU64>,
    write_lock_contention: Arc<AtomicU64>,
    read_lock_timeouts: Arc<AtomicU64>,
    write_lock_timeouts: Arc<AtomicU64>,
    read_lock_spin_attempts_total: Arc<AtomicU64>,
    write_lock_spin_attempts_total: Arc<AtomicU64>,
}

impl ConcurrentMoePipeline {
    pub fn new(pipeline: MoePipeline) -> Self {
        Self {
            inner: Arc::new(RwLock::new(pipeline)),
            read_lock_acquisitions: Arc::new(AtomicU64::new(0)),
            write_lock_acquisitions: Arc::new(AtomicU64::new(0)),
            read_lock_contention: Arc::new(AtomicU64::new(0)),
            write_lock_contention: Arc::new(AtomicU64::new(0)),
            read_lock_timeouts: Arc::new(AtomicU64::new(0)),
            write_lock_timeouts: Arc::new(AtomicU64::new(0)),
            read_lock_spin_attempts_total: Arc::new(AtomicU64::new(0)),
            write_lock_spin_attempts_total: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn from_builder(builder: MoePipelineBuilder) -> Self {
        Self::new(builder.build())
    }

    pub fn with_read<T, F>(&self, f: F) -> Result<T, MoeError>
    where
        F: FnOnce(&MoePipeline) -> T,
    {
        match self.inner.try_read() {
            Ok(guard) => {
                self.record_read_acquisition(0);
                Ok(f(&guard))
            }
            Err(TryLockError::WouldBlock) => {
                self.read_lock_contention.fetch_add(1, Ordering::Relaxed);
                let guard = self
                    .inner
                    .read()
                    .map_err(|_| Self::lock_poisoned_error(READ_LOCK_KIND))?;
                self.record_read_acquisition(0);
                Ok(f(&guard))
            }
            Err(TryLockError::Poisoned(_)) => Err(Self::lock_poisoned_error(READ_LOCK_KIND)),
        }
    }

    pub fn with_write<T, F>(&self, f: F) -> Result<T, MoeError>
    where
        F: FnOnce(&mut MoePipeline) -> Result<T, MoeError>,
    {
        match self.inner.try_write() {
            Ok(mut guard) => {
                self.record_write_acquisition(0);
                f(&mut guard)
            }
            Err(TryLockError::WouldBlock) => {
                self.write_lock_contention.fetch_add(1, Ordering::Relaxed);
                let mut guard = self
                    .inner
                    .write()
                    .map_err(|_| Self::lock_poisoned_error(WRITE_LOCK_KIND))?;
                self.record_write_acquisition(0);
                f(&mut guard)
            }
            Err(TryLockError::Poisoned(_)) => Err(Self::lock_poisoned_error(WRITE_LOCK_KIND)),
        }
    }

    pub fn with_read_timeout<T, F>(&self, max_lock_attempts: u32, f: F) -> Result<T, MoeError>
    where
        F: FnOnce(&MoePipeline) -> T,
    {
        if let Ok(guard) = self.inner.try_read() {
            self.record_read_acquisition(0);
            return Ok(f(&guard));
        }

        self.read_lock_contention.fetch_add(1, Ordering::Relaxed);
        for attempts in 1..=max_lock_attempts {
            std::thread::yield_now();
            match self.inner.try_read() {
                Ok(guard) => {
                    self.record_read_acquisition(u64::from(attempts));
                    return Ok(f(&guard));
                }
                Err(TryLockError::WouldBlock) => {}
                Err(TryLockError::Poisoned(_)) => {
                    return Err(Self::lock_poisoned_error(READ_LOCK_KIND));
                }
            }
        }

        self.read_lock_timeouts.fetch_add(1, Ordering::Relaxed);
        Err(Self::lock_timeout_error(READ_LOCK_KIND, max_lock_attempts))
    }

    pub fn with_write_timeout<T, F>(&self, max_lock_attempts: u32, f: F) -> Result<T, MoeError>
    where
        F: FnOnce(&mut MoePipeline) -> Result<T, MoeError>,
    {
        if let Ok(mut guard) = self.inner.try_write() {
            self.record_write_acquisition(0);
            return f(&mut guard);
        }

        self.write_lock_contention.fetch_add(1, Ordering::Relaxed);
        for attempts in 1..=max_lock_attempts {
            std::thread::yield_now();
            match self.inner.try_write() {
                Ok(mut guard) => {
                    self.record_write_acquisition(u64::from(attempts));
                    return f(&mut guard);
                }
                Err(TryLockError::WouldBlock) => {}
                Err(TryLockError::Poisoned(_)) => {
                    return Err(Self::lock_poisoned_error(WRITE_LOCK_KIND));
                }
            }
        }

        self.write_lock_timeouts.fetch_add(1, Ordering::Relaxed);
        Err(Self::lock_timeout_error(WRITE_LOCK_KIND, max_lock_attempts))
    }

    pub fn register_expert(&self, expert: Box<dyn Expert>) -> Result<(), MoeError> {
        self.with_write(|pipeline| pipeline.register_expert(expert))
    }

    pub fn execute(&self, task: Task) -> Result<AggregatedOutput, MoeError> {
        self.with_write(|pipeline| pipeline.execute(task))
    }

    pub fn remember_short_term(&self, entry: MemoryEntry) -> Result<(), MoeError> {
        self.with_write(|pipeline| pipeline.remember_short_term(entry))
    }

    pub fn remember_long_term(&self, entry: MemoryEntry) -> Result<(), MoeError> {
        self.with_write(|pipeline| pipeline.remember_long_term(entry))
    }

    pub fn export_runtime_bundle_json(&self) -> Result<String, MoeError> {
        self.with_read(|pipeline| pipeline.export_runtime_bundle_json())?
    }

    pub fn export_governance_state_json(&self) -> Result<String, MoeError> {
        self.with_read(|pipeline| pipeline.export_governance_state_json())?
    }

    pub fn export_governance_bundle_json(&self) -> Result<String, MoeError> {
        self.with_read(|pipeline| pipeline.export_governance_bundle_json())?
    }

    pub fn governance_audit_trail(&self) -> Result<GovernanceAuditTrail, MoeError> {
        self.with_read(|pipeline| pipeline.governance_audit_trail())
    }

    pub fn compare_and_import_runtime_bundle_json_with_checksum(
        &self,
        expected_current_version: u64,
        expected_current_checksum: &str,
        payload: &str,
    ) -> Result<(), MoeError> {
        self.with_write(|pipeline| {
            pipeline.compare_and_import_runtime_bundle_json_with_checksum(
                expected_current_version,
                expected_current_checksum,
                payload,
            )
        })
    }

    pub fn import_runtime_bundle_json(&self, payload: &str) -> Result<(), MoeError> {
        self.with_write(|pipeline| pipeline.import_runtime_bundle_json(payload))
    }

    pub fn try_import_runtime_bundle_json(&self, payload: &str) -> Result<(), MoeError> {
        self.with_write(|pipeline| pipeline.try_import_runtime_bundle_json(payload))
    }

    pub fn preview_runtime_bundle_import_json(
        &self,
        payload: &str,
    ) -> Result<GovernanceImportDecision, MoeError> {
        self.with_read(|pipeline| pipeline.preview_runtime_bundle_import_json(payload))?
    }

    pub fn compare_and_import_runtime_bundle_json(
        &self,
        expected_current_version: u64,
        payload: &str,
    ) -> Result<(), MoeError> {
        self.with_write(|pipeline| {
            pipeline.compare_and_import_runtime_bundle_json(expected_current_version, payload)
        })
    }

    pub fn compare_and_import_governance_bundle_json_with_checksum(
        &self,
        expected_current_version: u64,
        expected_current_checksum: &str,
        payload: &str,
    ) -> Result<(), MoeError> {
        self.with_write(|pipeline| {
            pipeline.compare_and_import_governance_bundle_json_with_checksum(
                expected_current_version,
                expected_current_checksum,
                payload,
            )
        })
    }

    pub fn compare_and_import_governance_state_json_with_checksum(
        &self,
        expected_current_version: u64,
        expected_current_checksum: &str,
        payload: &str,
    ) -> Result<(), MoeError> {
        self.with_write(|pipeline| {
            pipeline.compare_and_import_governance_state_json_with_checksum(
                expected_current_version,
                expected_current_checksum,
                payload,
            )
        })
    }

    pub fn import_governance_bundle_json(&self, payload: &str) -> Result<(), MoeError> {
        self.with_write(|pipeline| pipeline.import_governance_bundle_json(payload))
    }

    pub fn try_import_governance_bundle_json(&self, payload: &str) -> Result<(), MoeError> {
        self.with_write(|pipeline| pipeline.try_import_governance_bundle_json(payload))
    }

    pub fn preview_governance_bundle_import_json(
        &self,
        payload: &str,
    ) -> Result<GovernanceImportDecision, MoeError> {
        self.with_read(|pipeline| pipeline.preview_governance_bundle_import_json(payload))?
    }

    pub fn compare_and_import_governance_bundle_json(
        &self,
        expected_current_version: u64,
        payload: &str,
    ) -> Result<(), MoeError> {
        self.with_write(|pipeline| {
            pipeline.compare_and_import_governance_bundle_json(expected_current_version, payload)
        })
    }

    pub fn import_governance_state_json(&self, payload: &str) -> Result<(), MoeError> {
        self.with_write(|pipeline| pipeline.import_governance_state_json(payload))
    }

    pub fn try_import_governance_state_json(&self, payload: &str) -> Result<(), MoeError> {
        self.with_write(|pipeline| pipeline.try_import_governance_state_json(payload))
    }

    pub fn preview_governance_import_json(
        &self,
        payload: &str,
    ) -> Result<GovernanceImportDecision, MoeError> {
        self.with_read(|pipeline| pipeline.preview_governance_import_json(payload))?
    }

    pub fn compare_and_import_governance_state_json(
        &self,
        expected_current_version: u64,
        payload: &str,
    ) -> Result<(), MoeError> {
        self.with_write(|pipeline| {
            pipeline.compare_and_import_governance_state_json(expected_current_version, payload)
        })
    }

    pub fn metrics(&self) -> BTreeMap<String, u64> {
        let mut map = BTreeMap::new();
        Self::insert_metric(
            &mut map,
            "read_lock_acquisitions",
            &self.read_lock_acquisitions,
        );
        Self::insert_metric(
            &mut map,
            "write_lock_acquisitions",
            &self.write_lock_acquisitions,
        );
        Self::insert_metric(&mut map, "read_lock_contention", &self.read_lock_contention);
        Self::insert_metric(
            &mut map,
            "write_lock_contention",
            &self.write_lock_contention,
        );
        Self::insert_metric(&mut map, "read_lock_timeouts", &self.read_lock_timeouts);
        Self::insert_metric(&mut map, "write_lock_timeouts", &self.write_lock_timeouts);
        Self::insert_metric(
            &mut map,
            "read_lock_spin_attempts_total",
            &self.read_lock_spin_attempts_total,
        );
        Self::insert_metric(
            &mut map,
            "write_lock_spin_attempts_total",
            &self.write_lock_spin_attempts_total,
        );
        map
    }

    fn record_read_acquisition(&self, attempts: u64) {
        self.read_lock_acquisitions.fetch_add(1, Ordering::Relaxed);
        self.read_lock_spin_attempts_total
            .fetch_add(attempts, Ordering::Relaxed);
    }

    fn record_write_acquisition(&self, attempts: u64) {
        self.write_lock_acquisitions.fetch_add(1, Ordering::Relaxed);
        self.write_lock_spin_attempts_total
            .fetch_add(attempts, Ordering::Relaxed);
    }

    fn lock_poisoned_error(lock_kind: &str) -> MoeError {
        MoeError::DatasetError(format!("concurrent pipeline {lock_kind} lock poisoned"))
    }

    fn lock_timeout_error(lock_kind: &str, max_lock_attempts: u32) -> MoeError {
        MoeError::DatasetError(format!(
            "concurrent pipeline {lock_kind} lock timeout after {max_lock_attempts} attempts"
        ))
    }

    fn insert_metric(map: &mut BTreeMap<String, u64>, key: &str, value: &Arc<AtomicU64>) {
        map.insert(key.to_string(), value.load(Ordering::Relaxed));
    }
}
