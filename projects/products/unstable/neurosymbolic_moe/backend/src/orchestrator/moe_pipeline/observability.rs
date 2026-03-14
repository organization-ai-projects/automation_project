//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/moe_pipeline/observability.rs
use crate::memory_engine::MemoryStore;
use crate::moe_core::MoeError;
use crate::orchestrator::{MoePipeline, OperationalReport};

impl MoePipeline {
    pub fn export_operational_report(&self) -> OperationalReport {
        let audit_trail = self.governance_audit_trail();
        let runtime_bundle = self.export_runtime_bundle();
        let model_registry = self.model_registry();
        let sessions = self.buffer_manager.sessions().list_sessions();
        let session_buffer_values = sessions
            .iter()
            .map(|session| self.buffer_manager.sessions().values_ref(session).len())
            .sum();

        OperationalReport {
            governance_current_version: audit_trail.current_version,
            governance_current_checksum: audit_trail.current_checksum,
            governance_audit_entries: audit_trail.entries.len(),
            governance_state_snapshots: self.governance_state_snapshots.len(),
            runtime_bundle_checksum: runtime_bundle.runtime_checksum,
            short_term_memory_entries: self.short_term_memory.count(),
            long_term_memory_entries: self.long_term_memory.count(),
            working_buffer_entries: self.buffer_manager.working().count(),
            session_buffer_sessions: sessions.len(),
            session_buffer_values,
            trace_entries: self.trace_logger.count(),
            dataset_entries: self.dataset_store.count(),
            feedback_entries: self.feedback_store.count(),
            import_telemetry: self.import_telemetry_snapshot(),
            import_journal_events_total: self.import_journal_events_total(),
            import_journal_parse_failures_total: self.import_journal_parse_failures_total(),
            import_journal_rejections_total: self.import_journal_rejections_total(),
            import_journal_successful_imports_total: self.import_journal_successful_imports_total(),
            import_journal_deduplicated_replays_total: self
                .import_journal_deduplicated_replays_total(),
            import_journal_tracked_fingerprints: self.import_journal.tracked_fingerprint_count(),
            auto_improvement_runs_total: self.auto_improvement_status().runs_total,
            auto_improvement_bootstrap_entries_total: self
                .auto_improvement_status()
                .bootstrap_entries_total,
            auto_improvement_last_included_entries: self
                .auto_improvement_status()
                .last_included_entries,
            model_registry_entries: model_registry.entry_count(),
            model_registry_active_version: model_registry.active_version.unwrap_or(0),
            model_registry_latest_version: model_registry.latest_version().unwrap_or(0),
            trainer_trigger_events_pending: self.trainer_trigger_events_pending(),
        }
    }

    pub fn export_operational_report_json(&self) -> Result<String, MoeError> {
        common_json::json::to_json_string_pretty(&self.export_operational_report()).map_err(|err| {
            MoeError::DatasetError(format!("operational report serialization failed: {err}"))
        })
    }
}
