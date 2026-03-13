//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/tests/governance_state.rs
use crate::moe_core::MoeError;
use crate::orchestrator::{GovernanceState, MoePipelineBuilder};

#[test]
fn ensure_checksum_populates_and_verifies() {
    let mut state = GovernanceState::from_components(3, None, None, None);
    state.state_checksum.clear();
    state.ensure_checksum();
    assert!(state.verify_checksum());
}

#[test]
fn tampered_checksum_fails_verification() {
    let mut state = GovernanceState::from_components(5, None, None, None);
    state.state_checksum = "tampered".to_string();
    assert!(!state.verify_checksum());
}

#[test]
fn compare_and_import_governance_state_rejects_version_mismatch() {
    let mut pipeline = MoePipelineBuilder::new().build();
    let state = GovernanceState::from_components(0, None, None, None);
    let err = pipeline
        .compare_and_import_governance_state(1, state)
        .expect_err("CAS import should reject mismatched version");
    assert!(matches!(err, MoeError::PolicyRejected(_)));
    assert!(err.to_string().contains("compare-and-import rejected"));
}

#[test]
fn compare_and_import_governance_state_succeeds_on_matching_version() {
    let source = MoePipelineBuilder::new().build();
    let state = source.export_governance_state();

    let mut target = MoePipelineBuilder::new().build();
    target
        .compare_and_import_governance_state(0, state)
        .expect("CAS import should succeed on matching version");
    assert!(target.governance_audit_trail().current_version > 0);
}

#[test]
fn compare_and_import_governance_state_with_checksum_rejects_checksum_mismatch() {
    let source = MoePipelineBuilder::new().build();
    let state = source.export_governance_state();

    let mut target = MoePipelineBuilder::new().build();
    let err = target
        .compare_and_import_governance_state_with_checksum(0, "bad-checksum", state)
        .expect_err("CAS import should reject mismatched checksum");
    assert!(matches!(err, MoeError::PolicyRejected(_)));
    assert!(err.to_string().contains("expected governance checksum"));
}

#[test]
fn compare_and_import_governance_state_with_checksum_succeeds_on_matching_checksum() {
    let source = MoePipelineBuilder::new().build();
    let state = source.export_governance_state();

    let mut target = MoePipelineBuilder::new().build();
    let expected_checksum = target.export_governance_state().state_checksum;
    target
        .compare_and_import_governance_state_with_checksum(0, &expected_checksum, state)
        .expect("CAS import should succeed on matching checksum");
    assert!(target.governance_audit_trail().current_version > 0);
}

#[test]
fn preview_governance_import_json_rejects_oversized_payload() {
    let pipeline = MoePipelineBuilder::new().build();
    let oversized_payload = "x".repeat((4 * 1024 * 1024) + 1);
    let err = pipeline
        .preview_governance_import_json(&oversized_payload)
        .expect_err("oversized governance state payload should be rejected");
    assert!(err.to_string().contains("payload too large"));
}

#[test]
fn preview_governance_bundle_import_json_rejects_oversized_payload() {
    let pipeline = MoePipelineBuilder::new().build();
    let oversized_payload = "x".repeat((16 * 1024 * 1024) + 1);
    let err = pipeline
        .preview_governance_bundle_import_json(&oversized_payload)
        .expect_err("oversized governance bundle payload should be rejected");
    assert!(err.to_string().contains("payload too large"));
}

#[test]
fn try_import_governance_state_json_rejects_oversized_payload() {
    let mut pipeline = MoePipelineBuilder::new().build();
    let oversized_payload = "x".repeat((4 * 1024 * 1024) + 1);
    let err = pipeline
        .try_import_governance_state_json(&oversized_payload)
        .expect_err("oversized governance state payload should be rejected");
    assert!(err.to_string().contains("payload too large"));
}

#[test]
fn compare_and_import_governance_bundle_json_rejects_oversized_payload() {
    let mut pipeline = MoePipelineBuilder::new().build();
    let oversized_payload = "x".repeat((16 * 1024 * 1024) + 1);
    let err = pipeline
        .compare_and_import_governance_bundle_json(0, &oversized_payload)
        .expect_err("oversized governance bundle payload should be rejected");
    assert!(err.to_string().contains("payload too large"));
}

#[test]
fn compare_and_import_governance_state_json_succeeds_on_matching_version() {
    let source = MoePipelineBuilder::new().build();
    let payload = source
        .export_governance_state_json()
        .expect("governance state json export should succeed");

    let mut target = MoePipelineBuilder::new().build();
    target
        .compare_and_import_governance_state_json(0, &payload)
        .expect("matching version should allow compare-and-import json");
    assert!(target.governance_audit_trail().current_version > 0);
}
