//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/tests/governance_state.rs
use crate::moe_core::MoeError;
use crate::orchestrator::{
    GovernanceImportPolicy, GovernancePersistenceBundle, GovernanceState, GovernanceStateSnapshot,
    MoePipelineBuilder,
};
use serde::Serialize;

#[derive(Serialize)]
struct LegacyGovernanceStatePayload {
    state_version: u64,
    continuous_governance_policy: Option<crate::orchestrator::ContinuousGovernancePolicy>,
    evaluation_baseline: Option<crate::evaluation_engine::EvaluationEngine>,
    last_continuous_improvement_report: Option<crate::orchestrator::ContinuousImprovementReport>,
}

#[derive(Serialize)]
struct LegacyGovernanceBundlePayload {
    state: LegacyGovernanceStatePayload,
}

fn malformed_payload_variants(payload: &str) -> Vec<String> {
    vec![
        payload.replacen('{', "[", 1).replacen('}', "]", 1),
        payload.replacen(':', ";", 1),
        format!("{payload} trailing-garbage"),
        payload
            .strip_suffix('}')
            .expect("json payload should end with object close")
            .to_string(),
    ]
}

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

#[test]
fn import_governance_state_json_rejects_malformed_payload_variants() {
    let source = MoePipelineBuilder::new().build();
    let payload = source
        .export_governance_state_json()
        .expect("governance state json export should succeed");

    for malformed_payload in malformed_payload_variants(&payload) {
        let mut target = MoePipelineBuilder::new().build();
        let err = target
            .import_governance_state_json(&malformed_payload)
            .expect_err("malformed governance state payload must be rejected");
        assert!(
            err.to_string().contains("deserialization failed")
                || err.to_string().contains("payload too large")
        );
    }
}

#[test]
fn import_governance_bundle_json_rejects_malformed_payload_variants() {
    let source = MoePipelineBuilder::new().build();
    let payload = source
        .export_governance_bundle_json()
        .expect("governance bundle json export should succeed");

    for malformed_payload in malformed_payload_variants(&payload) {
        let mut target = MoePipelineBuilder::new().build();
        let err = target
            .import_governance_bundle_json(&malformed_payload)
            .expect_err("malformed governance bundle payload must be rejected");
        assert!(
            err.to_string().contains("deserialization failed")
                || err.to_string().contains("payload too large")
        );
    }
}

#[test]
fn governance_state_and_bundle_json_reject_checksum_corruption_matrix() {
    let source = MoePipelineBuilder::new().build();

    let mut corrupted_state = source.export_governance_state();
    corrupted_state.state_checksum = "corrupted-state-checksum".to_string();
    let corrupted_state_payload = common_json::json::to_json_string_pretty(&corrupted_state)
        .expect("corrupted state payload serialization should succeed");

    let mut target_state = MoePipelineBuilder::new().build();
    let state_err = target_state
        .import_governance_state_json(&corrupted_state_payload)
        .expect_err("corrupted governance state checksum must be rejected");
    assert!(
        state_err
            .to_string()
            .contains("checksum verification failed")
    );

    let mut corrupted_bundle: GovernancePersistenceBundle = source.export_governance_bundle();
    corrupted_bundle.state.state_checksum = "corrupted-bundle-checksum".to_string();
    let corrupted_bundle_payload = common_json::json::to_json_string_pretty(&corrupted_bundle)
        .expect("corrupted bundle payload serialization should succeed");

    let mut target_bundle = MoePipelineBuilder::new().build();
    let bundle_err = target_bundle
        .import_governance_bundle_json(&corrupted_bundle_payload)
        .expect_err("corrupted governance bundle checksum must be rejected");
    assert!(
        bundle_err
            .to_string()
            .contains("latest audit checksum does not match state checksum")
            || bundle_err
                .to_string()
                .contains("checksum verification failed")
    );
}

#[test]
fn import_governance_state_json_accepts_legacy_payload_without_schema_or_checksum() {
    let payload = common_json::json::to_json_string_pretty(&LegacyGovernanceStatePayload {
        state_version: 0,
        continuous_governance_policy: None,
        evaluation_baseline: None,
        last_continuous_improvement_report: None,
    })
    .expect("legacy state payload serialization should succeed");

    let mut pipeline = MoePipelineBuilder::new().build();
    pipeline
        .import_governance_state_json(&payload)
        .expect("legacy state payload should be importable");
    assert!(pipeline.export_governance_state().verify_checksum());
}

#[test]
fn import_governance_bundle_json_accepts_legacy_payload_without_audit_or_snapshots() {
    let payload = common_json::json::to_json_string_pretty(&LegacyGovernanceBundlePayload {
        state: LegacyGovernanceStatePayload {
            state_version: 0,
            continuous_governance_policy: None,
            evaluation_baseline: None,
            last_continuous_improvement_report: None,
        },
    })
    .expect("legacy bundle payload serialization should succeed");

    let mut pipeline = MoePipelineBuilder::new().build();
    pipeline
        .import_governance_bundle_json(&payload)
        .expect("legacy bundle payload should be importable");
    assert!(pipeline.export_governance_state().verify_checksum());
}

#[test]
fn import_governance_state_json_rejects_unsupported_schema_version() {
    let source = MoePipelineBuilder::new().build();
    let mut state = source.export_governance_state();
    state.schema_version = GovernanceState::schema_version() + 1;
    state.state_checksum = state.recompute_checksum();
    let payload = common_json::json::to_json_string_pretty(&state)
        .expect("governance state serialization should succeed");

    let mut target = MoePipelineBuilder::new().build();
    let err = target
        .import_governance_state_json(&payload)
        .expect_err("unsupported governance schema should be rejected");
    assert!(matches!(err, MoeError::PolicyRejected(_)));
    assert!(err.to_string().contains("schema"));
}

#[test]
fn import_governance_bundle_json_rejects_unsupported_state_schema_version() {
    let source = MoePipelineBuilder::new().build();
    let mut bundle = source.export_governance_bundle();
    bundle.state.schema_version = GovernanceState::schema_version() + 1;
    bundle.state.state_checksum = bundle.state.recompute_checksum();
    let payload = common_json::json::to_json_string_pretty(&bundle)
        .expect("governance bundle serialization should succeed");

    let mut target = MoePipelineBuilder::new().build();
    let err = target
        .import_governance_bundle_json(&payload)
        .expect_err("unsupported governance state schema in bundle should be rejected");
    assert!(matches!(err, MoeError::PolicyRejected(_)));
    assert!(err.to_string().contains("schema"));
}

#[test]
fn import_governance_bundle_json_rejects_unsupported_snapshot_schema_version() {
    let source = MoePipelineBuilder::new().build();
    let mut bundle = source.export_governance_bundle();
    if bundle.snapshots.is_empty() {
        bundle.snapshots.push(GovernanceStateSnapshot {
            version: bundle.state.state_version,
            reason: "synthetic snapshot for schema validation test".to_string(),
            state: bundle.state.clone(),
        });
    }
    let snapshot = bundle
        .snapshots
        .first_mut()
        .expect("bundle snapshot list should be non-empty");
    snapshot.state.schema_version = GovernanceState::schema_version() + 1;
    snapshot.state.state_checksum = snapshot.state.recompute_checksum();
    let payload = common_json::json::to_json_string_pretty(&bundle)
        .expect("governance bundle serialization should succeed");

    let mut target = MoePipelineBuilder::new().build();
    let err = target
        .import_governance_bundle_json(&payload)
        .expect_err("unsupported governance snapshot schema should be rejected");
    assert!(matches!(err, MoeError::PolicyRejected(_)));
    assert!(err.to_string().contains("schema"));
}

#[test]
fn import_governance_state_json_allows_schema_change_when_policy_enables_it() {
    let source = MoePipelineBuilder::new().build();
    let mut state = source.export_governance_state();
    state.schema_version = GovernanceState::schema_version() + 1;
    state.state_checksum = state.recompute_checksum();
    let payload = common_json::json::to_json_string_pretty(&state)
        .expect("governance state serialization should succeed");

    let mut target = MoePipelineBuilder::new()
        .with_governance_import_policy(GovernanceImportPolicy {
            allow_schema_change: true,
            ..GovernanceImportPolicy::strict()
        })
        .build();
    target
        .import_governance_state_json(&payload)
        .expect("schema change should be accepted when policy allows it");
}

#[test]
fn import_governance_bundle_json_allows_schema_change_when_policy_enables_it() {
    let source = MoePipelineBuilder::new().build();
    let mut bundle = source.export_governance_bundle();
    bundle.state.schema_version = GovernanceState::schema_version() + 1;
    bundle.state.state_checksum = bundle.state.recompute_checksum();
    let payload = common_json::json::to_json_string_pretty(&bundle)
        .expect("governance bundle serialization should succeed");

    let mut target = MoePipelineBuilder::new()
        .with_governance_import_policy(GovernanceImportPolicy {
            allow_schema_change: true,
            ..GovernanceImportPolicy::strict()
        })
        .build();
    target
        .import_governance_bundle_json(&payload)
        .expect("schema change should be accepted when policy allows it");
}
