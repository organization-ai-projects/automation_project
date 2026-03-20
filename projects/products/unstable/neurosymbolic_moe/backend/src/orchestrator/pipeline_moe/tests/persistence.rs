//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/pipeline_moe/tests/persistence.rs
use crate::orchestrator::MoePipelineBuilder;

#[test]
fn persistence_module_governance_json_roundtrip_stays_allowed() {
    let mut pipeline = MoePipelineBuilder::new().build();

    let state_payload = pipeline
        .export_governance_state_json()
        .expect("governance state should serialize");
    let preview = pipeline
        .preview_governance_import_json(&state_payload)
        .expect("preview should succeed");
    assert!(preview.allowed);
    pipeline
        .try_import_governance_state_json(&state_payload)
        .expect("state import should succeed");
}

#[test]
fn persistence_module_runtime_bundle_json_roundtrip_stays_allowed() {
    let mut pipeline = MoePipelineBuilder::new().build();

    let runtime_payload = pipeline
        .export_runtime_bundle_json()
        .expect("runtime bundle should serialize");
    let preview = pipeline
        .preview_runtime_bundle_import_json(&runtime_payload)
        .expect("runtime preview should succeed");
    assert!(preview.allowed);
    pipeline
        .try_import_runtime_bundle_json(&runtime_payload)
        .expect("runtime import should succeed");
}
