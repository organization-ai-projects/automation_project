//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/tests/import_telemetry.rs
use crate::orchestrator::ImportTelemetry;

#[test]
fn import_telemetry_counters_increment_independently() {
    let mut telemetry = ImportTelemetry::default();
    telemetry.record_governance_state_success();
    telemetry.record_governance_state_rejection();
    telemetry.record_governance_bundle_success();
    telemetry.record_governance_bundle_rejection();
    telemetry.record_runtime_bundle_success();
    telemetry.record_runtime_bundle_rejection();
    telemetry.record_json_parse_failure();

    assert_eq!(telemetry.governance_state_import_successes, 1);
    assert_eq!(telemetry.governance_state_import_rejections, 1);
    assert_eq!(telemetry.governance_bundle_import_successes, 1);
    assert_eq!(telemetry.governance_bundle_import_rejections, 1);
    assert_eq!(telemetry.runtime_bundle_import_successes, 1);
    assert_eq!(telemetry.runtime_bundle_import_rejections, 1);
    assert_eq!(telemetry.json_parse_failures, 1);
}
