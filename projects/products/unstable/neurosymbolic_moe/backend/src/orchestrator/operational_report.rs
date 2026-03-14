//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/operational_report.rs
use serde::{Deserialize, Serialize};

use crate::orchestrator::ImportTelemetry;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationalReport {
    pub governance_current_version: u64,
    pub governance_current_checksum: Option<String>,
    pub governance_audit_entries: usize,
    pub governance_state_snapshots: usize,
    pub runtime_bundle_checksum: String,
    pub short_term_memory_entries: usize,
    pub long_term_memory_entries: usize,
    pub working_buffer_entries: usize,
    pub session_buffer_sessions: usize,
    pub session_buffer_values: usize,
    pub trace_entries: usize,
    pub dataset_entries: usize,
    pub feedback_entries: usize,
    pub import_telemetry: ImportTelemetry,
    pub import_journal_events_total: u64,
    pub import_journal_parse_failures_total: u64,
    pub import_journal_rejections_total: u64,
    pub import_journal_successful_imports_total: u64,
    pub import_journal_deduplicated_replays_total: u64,
    pub import_journal_tracked_fingerprints: usize,
    pub auto_improvement_runs_total: u64,
    pub auto_improvement_bootstrap_entries_total: usize,
    pub auto_improvement_last_included_entries: usize,
}

impl OperationalReport {
    pub fn slo_violations(
        &self,
        min_runtime_import_successes: u64,
        max_total_import_rejections: u64,
        max_json_parse_failures: u64,
    ) -> Vec<String> {
        let mut violations = Vec::new();
        if self.import_telemetry.runtime_bundle_import_successes < min_runtime_import_successes {
            violations.push(format!(
                "runtime import successes {} below minimum {}",
                self.import_telemetry.runtime_bundle_import_successes, min_runtime_import_successes
            ));
        }
        let total_rejections = self.import_telemetry.governance_state_import_rejections
            + self.import_telemetry.governance_bundle_import_rejections
            + self.import_telemetry.runtime_bundle_import_rejections;
        if total_rejections > max_total_import_rejections {
            violations.push(format!(
                "total import rejections {} above maximum {}",
                total_rejections, max_total_import_rejections
            ));
        }
        if self.import_telemetry.json_parse_failures > max_json_parse_failures {
            violations.push(format!(
                "json parse failures {} above maximum {}",
                self.import_telemetry.json_parse_failures, max_json_parse_failures
            ));
        }
        violations
    }

    pub fn slo_status(
        &self,
        min_runtime_import_successes: u64,
        max_total_import_rejections: u64,
        max_json_parse_failures: u64,
    ) -> &'static str {
        if self
            .slo_violations(
                min_runtime_import_successes,
                max_total_import_rejections,
                max_json_parse_failures,
            )
            .is_empty()
        {
            "OK"
        } else {
            "FAIL"
        }
    }

    pub fn to_prometheus_text(&self, prefix: &str) -> String {
        let p = if prefix.is_empty() {
            "moe_pipeline".to_string()
        } else {
            prefix.to_string()
        };
        format!(
            "{p}_governance_current_version {}\n{p}_governance_audit_entries {}\n{p}_governance_state_snapshots {}\n{p}_short_term_memory_entries {}\n{p}_long_term_memory_entries {}\n{p}_working_buffer_entries {}\n{p}_session_buffer_sessions {}\n{p}_session_buffer_values {}\n{p}_trace_entries {}\n{p}_dataset_entries {}\n{p}_feedback_entries {}\n{p}_import_runtime_successes {}\n{p}_import_runtime_rejections {}\n{p}_import_governance_state_successes {}\n{p}_import_governance_state_rejections {}\n{p}_import_governance_bundle_successes {}\n{p}_import_governance_bundle_rejections {}\n{p}_import_json_parse_failures {}\n{p}_import_journal_events_total {}\n{p}_import_journal_parse_failures_total {}\n{p}_import_journal_rejections_total {}\n{p}_import_journal_successful_imports_total {}\n{p}_import_journal_deduplicated_replays_total {}\n{p}_import_journal_tracked_fingerprints {}\n{p}_auto_improvement_runs_total {}\n{p}_auto_improvement_bootstrap_entries_total {}\n{p}_auto_improvement_last_included_entries {}\n",
            self.governance_current_version,
            self.governance_audit_entries,
            self.governance_state_snapshots,
            self.short_term_memory_entries,
            self.long_term_memory_entries,
            self.working_buffer_entries,
            self.session_buffer_sessions,
            self.session_buffer_values,
            self.trace_entries,
            self.dataset_entries,
            self.feedback_entries,
            self.import_telemetry.runtime_bundle_import_successes,
            self.import_telemetry.runtime_bundle_import_rejections,
            self.import_telemetry.governance_state_import_successes,
            self.import_telemetry.governance_state_import_rejections,
            self.import_telemetry.governance_bundle_import_successes,
            self.import_telemetry.governance_bundle_import_rejections,
            self.import_telemetry.json_parse_failures,
            self.import_journal_events_total,
            self.import_journal_parse_failures_total,
            self.import_journal_rejections_total,
            self.import_journal_successful_imports_total,
            self.import_journal_deduplicated_replays_total,
            self.import_journal_tracked_fingerprints,
            self.auto_improvement_runs_total,
            self.auto_improvement_bootstrap_entries_total,
            self.auto_improvement_last_included_entries,
        )
    }
}
