//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/import_telemetry.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportTelemetry {
    pub governance_state_import_successes: u64,
    pub governance_state_import_rejections: u64,
    pub governance_bundle_import_successes: u64,
    pub governance_bundle_import_rejections: u64,
    pub runtime_bundle_import_successes: u64,
    pub runtime_bundle_import_rejections: u64,
    pub runtime_bundle_import_expired_leases_released_total: u64,
    pub runtime_bundle_import_dead_letter_events_observed_total: u64,
    pub json_parse_failures: u64,
}

impl ImportTelemetry {
    pub fn record_governance_state_success(&mut self) {
        self.governance_state_import_successes =
            self.governance_state_import_successes.saturating_add(1);
    }

    pub fn record_governance_state_rejection(&mut self) {
        self.governance_state_import_rejections =
            self.governance_state_import_rejections.saturating_add(1);
    }

    pub fn record_governance_bundle_success(&mut self) {
        self.governance_bundle_import_successes =
            self.governance_bundle_import_successes.saturating_add(1);
    }

    pub fn record_governance_bundle_rejection(&mut self) {
        self.governance_bundle_import_rejections =
            self.governance_bundle_import_rejections.saturating_add(1);
    }

    pub fn record_runtime_bundle_success(&mut self) {
        self.runtime_bundle_import_successes =
            self.runtime_bundle_import_successes.saturating_add(1);
    }

    pub fn record_runtime_bundle_rejection(&mut self) {
        self.runtime_bundle_import_rejections =
            self.runtime_bundle_import_rejections.saturating_add(1);
    }

    pub fn record_runtime_bundle_import_released_expired_leases(&mut self, released: u64) {
        self.runtime_bundle_import_expired_leases_released_total = self
            .runtime_bundle_import_expired_leases_released_total
            .saturating_add(released);
    }

    pub fn record_runtime_bundle_import_dead_letter_events_observed(&mut self, observed: u64) {
        self.runtime_bundle_import_dead_letter_events_observed_total = self
            .runtime_bundle_import_dead_letter_events_observed_total
            .saturating_add(observed);
    }

    pub fn record_json_parse_failure(&mut self) {
        self.json_parse_failures = self.json_parse_failures.saturating_add(1);
    }
}
