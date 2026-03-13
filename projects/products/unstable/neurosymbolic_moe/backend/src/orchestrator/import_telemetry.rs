use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportTelemetry {
    pub governance_state_import_successes: u64,
    pub governance_state_import_rejections: u64,
    pub governance_bundle_import_successes: u64,
    pub governance_bundle_import_rejections: u64,
    pub runtime_bundle_import_successes: u64,
    pub runtime_bundle_import_rejections: u64,
    pub json_parse_failures: u64,
}

impl ImportTelemetry {
    pub fn record_governance_state_success(&mut self) {
        self.governance_state_import_successes += 1;
    }

    pub fn record_governance_state_rejection(&mut self) {
        self.governance_state_import_rejections += 1;
    }

    pub fn record_governance_bundle_success(&mut self) {
        self.governance_bundle_import_successes += 1;
    }

    pub fn record_governance_bundle_rejection(&mut self) {
        self.governance_bundle_import_rejections += 1;
    }

    pub fn record_runtime_bundle_success(&mut self) {
        self.runtime_bundle_import_successes += 1;
    }

    pub fn record_runtime_bundle_rejection(&mut self) {
        self.runtime_bundle_import_rejections += 1;
    }

    pub fn record_json_parse_failure(&mut self) {
        self.json_parse_failures += 1;
    }
}
