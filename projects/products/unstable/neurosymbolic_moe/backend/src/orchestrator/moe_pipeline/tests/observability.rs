//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/moe_pipeline/tests/observability.rs
use crate::orchestrator::{MoePipelineBuilder, OperationalReport};

#[test]
fn observability_module_exports_operational_report_json() {
    let pipeline = MoePipelineBuilder::new().build();
    let report = pipeline.export_operational_report();
    assert_eq!(
        report.governance_current_version,
        pipeline.export_governance_state().state_version
    );

    let report_json = pipeline
        .export_operational_report_json()
        .expect("operational report json should serialize");
    let parsed: OperationalReport =
        common_json::json::from_json_str(&report_json).expect("operational report json parse");
    assert_eq!(
        parsed.governance_current_version,
        report.governance_current_version
    );
}
