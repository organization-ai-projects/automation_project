//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/pipeline_moe/tests/governance_runtime.rs
use crate::orchestrator::{self, MoePipelineBuilder};

#[test]
fn governance_runtime_module_rejects_tampered_runtime_bundle_checksum() {
    let pipeline = MoePipelineBuilder::new().build();
    let payload = pipeline
        .export_runtime_bundle_json()
        .expect("runtime bundle should serialize");
    let mut bundle: orchestrator::RuntimePersistenceBundle =
        common_json::json::from_json_str(&payload).expect("bundle json should deserialize");
    bundle.runtime_checksum = "tampered-checksum".to_string();
    let tampered_payload = common_json::json::to_json_string_pretty(&bundle)
        .expect("tampered payload should serialize");

    let err = pipeline
        .preview_runtime_bundle_import_json(&tampered_payload)
        .expect_err("tampered checksum must be rejected");
    assert!(
        format!("{err}").contains("checksum"),
        "error should mention checksum verification"
    );
}

#[test]
fn governance_runtime_module_preview_governance_bundle_roundtrip_is_allowed() {
    let pipeline = MoePipelineBuilder::new().build();
    let payload = pipeline
        .export_governance_bundle_json()
        .expect("governance bundle should serialize");
    let decision = pipeline
        .preview_governance_bundle_import_json(&payload)
        .expect("preview should succeed");
    assert!(decision.allowed);
}
