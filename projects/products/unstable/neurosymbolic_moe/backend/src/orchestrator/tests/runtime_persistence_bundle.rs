//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/tests/runtime_persistence_bundle.rs
use crate::memory_engine::{MemoryEntry, MemoryType};
use crate::moe_core::MoeError;
use crate::orchestrator::{
    GovernanceImportPolicy, GovernancePersistenceBundle, MoePipelineBuilder,
    RuntimePersistenceBundle, TrainerTriggerEvent, Version,
};
use crate::{buffer_manager::BufferManager, memory_engine::MemoryEntry as RuntimeMemoryEntry};
use protocol::ProtocolId;
use serde::Serialize;
use std::collections::HashMap;

fn test_memory_entry(id: &str, content: &str, memory_type: MemoryType) -> MemoryEntry {
    MemoryEntry {
        id: id.to_string(),
        content: content.to_string(),
        tags: vec!["persist".to_string()],
        created_at: 1,
        expires_at: None,
        memory_type,
        relevance: 0.9,
        metadata: HashMap::new(),
    }
}

fn memory_fingerprint(entries: &[MemoryEntry]) -> Vec<String> {
    let mut rows = entries
        .iter()
        .map(|entry| {
            format!(
                "{}|{}|{}|{:?}",
                entry.id, entry.content, entry.relevance, entry.memory_type
            )
        })
        .collect::<Vec<_>>();
    rows.sort();
    rows
}

fn assert_runtime_json_rejected(target: &mut crate::orchestrator::MoePipeline, payload: &str) {
    let import_err = target
        .import_runtime_bundle_json(payload)
        .expect_err("corrupted runtime payload must be rejected");
    assert!(
        import_err.to_string().contains("deserialization failed")
            || import_err
                .to_string()
                .contains("checksum verification failed")
            || import_err
                .to_string()
                .contains("governance bundle import rejected")
            || import_err
                .to_string()
                .contains("governance import rejected")
    );
}

#[derive(Serialize)]
struct LegacyRuntimePersistenceBundle {
    governance: GovernancePersistenceBundle,
    short_term_memory_entries: Vec<RuntimeMemoryEntry>,
    long_term_memory_entries: Vec<RuntimeMemoryEntry>,
    buffer_manager: BufferManager,
}

#[test]
fn runtime_bundle_roundtrip_restores_state_memory_and_buffers() {
    let mut source = MoePipelineBuilder::new().build();
    source
        .remember_short_term(test_memory_entry(
            "memory.short.incident_context",
            "incident context: timeout on retrieval endpoint",
            MemoryType::Short,
        ))
        .expect("short memory write should succeed");
    source
        .remember_long_term(test_memory_entry(
            "memory.long.postmortem_summary",
            "postmortem summary: add circuit breaker and retry budget",
            MemoryType::Long,
        ))
        .expect("long memory write should succeed");
    source.put_session_buffer(
        &ProtocolId::default(),
        "conversation.summary",
        "user requested runtime persistence replay validation",
    );

    let bundle = source.export_runtime_bundle();
    assert_eq!(
        bundle.schema_version,
        RuntimePersistenceBundle::schema_version()
    );
    assert!(bundle.verify_checksum());

    let mut target = MoePipelineBuilder::new().build();
    target
        .import_runtime_bundle(bundle.clone())
        .expect("runtime bundle import should succeed");

    let restored = target.export_runtime_bundle();
    assert!(restored.verify_checksum());
    assert_eq!(
        restored.governance.state.state_checksum,
        bundle.governance.state.state_checksum
    );
    assert_eq!(
        memory_fingerprint(&restored.short_term_memory_entries),
        memory_fingerprint(&bundle.short_term_memory_entries)
    );
    assert_eq!(
        memory_fingerprint(&restored.long_term_memory_entries),
        memory_fingerprint(&bundle.long_term_memory_entries)
    );
    assert_eq!(
        restored
            .buffer_manager
            .sessions()
            .values(&ProtocolId::default()),
        bundle
            .buffer_manager
            .sessions()
            .values(&ProtocolId::default())
    );
}

#[test]
fn runtime_bundle_roundtrip_restores_trainer_trigger_dead_letters() {
    let mut source = MoePipelineBuilder::new().build();
    source.trainer_trigger_queue.push(TrainerTriggerEvent {
        event_id: ProtocolId::default(),
        model_version: Version::new(2, 0, 0),
        training_bundle_checksum: "bundle-dead-letter-99".to_string(),
        included_entries: 120,
        train_samples: 96,
        validation_samples: 24,
        generated_at: 42,
        delivery_attempts: 8,
        last_attempted_at: Some(7),
    });
    assert!(
        source
            .lease_next_trainer_trigger_event_with_policy(100)
            .is_none()
    );
    assert_eq!(source.trainer_trigger_events_pending(), 0);
    assert_eq!(source.trainer_trigger_dead_letter_events_total(), 1);

    let bundle = source.export_runtime_bundle();
    assert_eq!(bundle.trainer_trigger_events.len(), 0);
    assert_eq!(bundle.trainer_trigger_dead_letter_events.len(), 1);
    assert_eq!(bundle.trainer_trigger_leased_event_ids.len(), 0);

    let mut target = MoePipelineBuilder::new().build();
    target
        .import_runtime_bundle(bundle.clone())
        .expect("runtime bundle import should succeed");

    let restored = target.export_runtime_bundle();
    assert_eq!(restored.trainer_trigger_events.len(), 0);
    assert_eq!(restored.trainer_trigger_dead_letter_events.len(), 1);
    assert_eq!(restored.trainer_trigger_leased_event_ids.len(), 0);
    assert_eq!(restored.recompute_checksum(), bundle.recompute_checksum());
}

#[test]
fn runtime_bundle_roundtrip_restores_trainer_trigger_leases() {
    let mut source = MoePipelineBuilder::new().build();
    source.trainer_trigger_queue.push(TrainerTriggerEvent {
        event_id: ProtocolId::default(),
        model_version: Version::new(4, 0, 0),
        training_bundle_checksum: "bundle-leased-111".to_string(),
        included_entries: 88,
        train_samples: 70,
        validation_samples: 18,
        generated_at: 50,
        delivery_attempts: 0,
        last_attempted_at: None,
    });
    let leased = source
        .lease_next_trainer_trigger_event_with_policy(u64::MAX - 10)
        .expect("event should lease before export");
    assert_eq!(leased.event_id, ProtocolId::default());

    let bundle = source.export_runtime_bundle();
    assert_eq!(bundle.trainer_trigger_events.len(), 1);
    assert_eq!(
        bundle.trainer_trigger_leased_event_ids,
        vec![ProtocolId::default()]
    );

    let mut target = MoePipelineBuilder::new().build();
    target
        .import_runtime_bundle(bundle.clone())
        .expect("runtime bundle import should preserve lease metadata");

    let report = target.export_operational_report();
    assert_eq!(report.trainer_trigger_events_pending, 1);
    assert_eq!(report.trainer_trigger_events_leased, 1);

    let restored_bundle = target.export_runtime_bundle();
    assert_eq!(
        restored_bundle.trainer_trigger_leased_event_ids,
        vec![ProtocolId::default()]
    );
    assert_eq!(
        restored_bundle.recompute_checksum(),
        bundle.recompute_checksum()
    );
}

#[test]
fn runtime_checksum_recompute_is_order_independent_for_leased_event_ids() {
    let mut source = MoePipelineBuilder::new().build();
    source.trainer_trigger_queue.push(TrainerTriggerEvent {
        event_id: ProtocolId::default(),
        model_version: Version::new(7, 0, 0),
        training_bundle_checksum: "bundle-leased-501".to_string(),
        included_entries: 64,
        train_samples: 51,
        validation_samples: 13,
        generated_at: 71,
        delivery_attempts: 0,
        last_attempted_at: None,
    });
    source.trainer_trigger_queue.push(TrainerTriggerEvent {
        event_id: ProtocolId::default(),
        model_version: Version::new(7, 0, 0),
        training_bundle_checksum: "bundle-leased-502".to_string(),
        included_entries: 65,
        train_samples: 52,
        validation_samples: 13,
        generated_at: 72,
        delivery_attempts: 0,
        last_attempted_at: None,
    });
    source
        .lease_next_trainer_trigger_event_with_policy(100)
        .expect("first event should lease");
    source
        .lease_next_trainer_trigger_event_with_policy(100)
        .expect("second event should lease");

    let mut bundle = source.export_runtime_bundle();
    assert_eq!(
        bundle.trainer_trigger_leased_event_ids,
        vec![ProtocolId::default(), ProtocolId::default()]
    );
    let expected_checksum = bundle.recompute_checksum();

    bundle.trainer_trigger_leased_event_ids.reverse();
    assert_eq!(
        bundle.recompute_checksum(),
        expected_checksum,
        "lease id ordering must not affect runtime checksum"
    );
}

#[test]
fn runtime_bundle_import_releases_expired_trainer_trigger_leases() {
    let mut source = MoePipelineBuilder::new().build();
    source.trainer_trigger_queue.push(TrainerTriggerEvent {
        event_id: ProtocolId::default(),
        model_version: Version::new(5, 0, 0),
        training_bundle_checksum: "bundle-leased-expired-222".to_string(),
        included_entries: 90,
        train_samples: 72,
        validation_samples: 18,
        generated_at: 60,
        delivery_attempts: 0,
        last_attempted_at: None,
    });
    let leased = source
        .lease_next_trainer_trigger_event_with_policy(1)
        .expect("event should lease before export");
    assert_eq!(leased.event_id, ProtocolId::default());

    let bundle = source.export_runtime_bundle();
    assert_eq!(
        bundle.trainer_trigger_leased_event_ids,
        vec![ProtocolId::default()]
    );

    let mut target = MoePipelineBuilder::new().build();
    target
        .import_runtime_bundle(bundle)
        .expect("runtime bundle import should succeed");

    let report = target.export_operational_report();
    assert_eq!(report.trainer_trigger_events_pending, 1);
    assert_eq!(
        report.trainer_trigger_events_leased, 0,
        "expired lease should be released on import"
    );
    assert!(report.runtime_last_import_at_epoch_seconds.is_some());
    assert!(report.runtime_last_import_released_expired_leases >= 1);
    assert_eq!(report.runtime_last_import_pending_events_after_import, 1);
    assert_eq!(report.runtime_last_import_leased_events_after_import, 0);
    assert!(
        report
            .import_telemetry
            .runtime_bundle_import_expired_leases_released_total
            >= 1
    );
    let runtime_import_report = target
        .last_runtime_import_report()
        .expect("runtime import report should be captured after import");
    assert!(runtime_import_report.released_expired_leases >= 1);
    assert!(
        target
            .lease_next_trainer_trigger_event_with_policy(100)
            .is_some(),
        "event should be leaseable immediately after expired lease release"
    );
}

#[test]
fn import_runtime_bundle_rejects_checksum_drift() {
    let mut source = MoePipelineBuilder::new().build();
    source
        .remember_short_term(test_memory_entry(
            "memory.short.audit_seed",
            "baseline payload for checksum validation",
            MemoryType::Short,
        ))
        .expect("short memory write should succeed");

    let mut tampered = source.export_runtime_bundle();
    tampered.short_term_memory_entries[0].content =
        "tampered payload for checksum validation".to_string();

    let mut target = MoePipelineBuilder::new().build();
    let err = target
        .import_runtime_bundle(tampered)
        .expect_err("checksum drift must be rejected");
    assert!(matches!(err, MoeError::PolicyRejected(_)));
    assert!(err.to_string().contains("checksum verification failed"));
}

#[test]
fn import_runtime_bundle_rejects_unsupported_schema() {
    let source = MoePipelineBuilder::new().build();
    let mut bundle = source.export_runtime_bundle();
    bundle.schema_version = RuntimePersistenceBundle::schema_version() + 1;

    let mut target = MoePipelineBuilder::new().build();
    let err = target
        .import_runtime_bundle(bundle)
        .expect_err("unsupported runtime schema must be rejected");
    assert!(matches!(err, MoeError::PolicyRejected(_)));
    assert!(err.to_string().contains("schema version"));
}

#[test]
fn import_runtime_bundle_allows_schema_change_when_policy_enables_it() {
    let source = MoePipelineBuilder::new().build();
    let mut bundle = source.export_runtime_bundle();
    bundle.schema_version = RuntimePersistenceBundle::schema_version() + 1;
    bundle.runtime_checksum = bundle.recompute_checksum();

    let mut target = MoePipelineBuilder::new()
        .with_governance_import_policy(GovernanceImportPolicy {
            allow_schema_change: true,
            ..GovernanceImportPolicy::strict()
        })
        .build();
    target
        .import_runtime_bundle(bundle)
        .expect("runtime schema change should be accepted when policy allows it");
}

#[test]
fn try_import_runtime_bundle_json_roundtrip_succeeds() {
    let mut source = MoePipelineBuilder::new().build();
    source
        .remember_short_term(test_memory_entry(
            "memory.short.runtime_json_seed",
            "runtime json payload for roundtrip",
            MemoryType::Short,
        ))
        .expect("short memory write should succeed");
    source.put_session_buffer(
        &ProtocolId::default(),
        "analysis.note",
        "roundtrip import should keep this value",
    );

    let payload = source
        .export_runtime_bundle_json()
        .expect("runtime bundle json export should succeed");

    let mut target = MoePipelineBuilder::new().build();
    target
        .try_import_runtime_bundle_json(&payload)
        .expect("runtime bundle json try-import should succeed");

    let restored = target.export_runtime_bundle();
    assert!(restored.verify_checksum());
    assert_eq!(
        restored
            .buffer_manager
            .sessions()
            .values(&ProtocolId::default()),
        vec!["roundtrip import should keep this value".to_string()]
    );
}

#[test]
fn preview_runtime_bundle_import_json_rejects_checksum_tampering() {
    let mut source = MoePipelineBuilder::new().build();
    source
        .remember_short_term(test_memory_entry(
            "memory.short.preview_seed",
            "baseline payload used by preview import",
            MemoryType::Short,
        ))
        .expect("short memory write should succeed");

    let payload = source
        .export_runtime_bundle_json()
        .expect("runtime bundle json export should succeed");
    let tampered_payload = payload.replace(
        "baseline payload used by preview import",
        "tampered payload used by preview import",
    );

    let target = MoePipelineBuilder::new().build();
    let err = target
        .preview_runtime_bundle_import_json(&tampered_payload)
        .expect_err("preview should reject checksum drift");
    assert!(matches!(err, MoeError::PolicyRejected(_)));
    assert!(err.to_string().contains("checksum verification failed"));
}

#[test]
fn preview_runtime_bundle_import_json_rejects_schema_drift() {
    let source = MoePipelineBuilder::new().build();
    let mut bundle = source.export_runtime_bundle();
    bundle.schema_version = RuntimePersistenceBundle::schema_version() + 1;
    let payload = common_json::json::to_json_string_pretty(&bundle)
        .expect("runtime bundle serialization should succeed");

    let target = MoePipelineBuilder::new().build();
    let err = target
        .preview_runtime_bundle_import_json(&payload)
        .expect_err("preview should reject unsupported schema");
    assert!(matches!(err, MoeError::PolicyRejected(_)));
    assert!(err.to_string().contains("schema version"));
}

#[test]
fn import_runtime_bundle_json_accepts_legacy_payload_without_schema_or_checksum() {
    let mut source = MoePipelineBuilder::new().build();
    source
        .remember_short_term(test_memory_entry(
            "memory.short.legacy_seed",
            "legacy seed memory entry",
            MemoryType::Short,
        ))
        .expect("short memory write should succeed");
    source.put_session_buffer(
        &ProtocolId::default(),
        "conversation.note",
        "legacy payload should still be importable",
    );

    let bundle = source.export_runtime_bundle();
    let legacy = LegacyRuntimePersistenceBundle {
        governance: bundle.governance.clone(),
        short_term_memory_entries: bundle.short_term_memory_entries.clone(),
        long_term_memory_entries: bundle.long_term_memory_entries.clone(),
        buffer_manager: bundle.buffer_manager.clone(),
    };
    let legacy_payload = common_json::json::to_json_string_pretty(&legacy)
        .expect("legacy runtime bundle serialization should succeed");

    let mut target = MoePipelineBuilder::new().build();
    target
        .import_runtime_bundle_json(&legacy_payload)
        .expect("legacy runtime bundle json import should succeed");
    let restored = target.export_runtime_bundle();
    assert!(restored.verify_checksum());
    assert_eq!(
        restored
            .buffer_manager
            .sessions()
            .values(&ProtocolId::default()),
        vec!["legacy payload should still be importable".to_string()]
    );
}

#[test]
fn runtime_bundle_roundtrip_with_high_volume_preserves_counts() {
    let mut source = MoePipelineBuilder::new().build();
    for idx in 0..128_u32 {
        source
            .remember_short_term(test_memory_entry(
                &format!("memory.short.bulk.{idx}"),
                &format!("bulk short-term entry {idx} for load test"),
                MemoryType::Short,
            ))
            .expect("short memory write should succeed");
        source
            .remember_long_term(test_memory_entry(
                &format!("memory.long.bulk.{idx}"),
                &format!("bulk long-term entry {idx} for load test"),
                MemoryType::Long,
            ))
            .expect("long memory write should succeed");
        source.put_session_buffer(
            &ProtocolId::default(),
            &format!("checkpoint.{idx}"),
            &format!("state snapshot marker {idx}"),
        );
    }

    let bundle = source.export_runtime_bundle();
    let mut target = MoePipelineBuilder::new().build();
    target
        .import_runtime_bundle(bundle.clone())
        .expect("bulk runtime bundle import should succeed");
    let restored = target.export_runtime_bundle();

    assert_eq!(restored.short_term_memory_entries.len(), 128);
    assert_eq!(restored.long_term_memory_entries.len(), 128);
    assert_eq!(
        restored
            .buffer_manager
            .sessions()
            .values(&ProtocolId::default())
            .len(),
        128
    );
    assert_eq!(
        restored.recompute_checksum(),
        bundle.recompute_checksum(),
        "bulk roundtrip checksum should remain stable"
    );
}

#[test]
fn import_runtime_bundle_rejects_duplicate_short_term_memory_ids() {
    let source = MoePipelineBuilder::new().build();
    let mut bundle = source.export_runtime_bundle();
    let duplicated = test_memory_entry(
        "memory.short.duplicate.id",
        "first value",
        MemoryType::Short,
    );
    bundle.short_term_memory_entries.push(duplicated.clone());
    bundle.short_term_memory_entries.push(MemoryEntry {
        content: "second value".to_string(),
        ..duplicated
    });
    bundle.runtime_checksum = bundle.recompute_checksum();

    let mut target = MoePipelineBuilder::new().build();
    let err = target
        .import_runtime_bundle(bundle)
        .expect_err("duplicate short-term memory ids must be rejected");
    assert!(matches!(err, MoeError::PolicyRejected(_)));
    assert!(err.to_string().contains("duplicate short-term memory ids"));
}

#[test]
fn import_runtime_bundle_failure_rolls_back_runtime_state_and_runtime_import_observability() {
    let mut source = MoePipelineBuilder::new().build();
    source
        .remember_short_term(test_memory_entry(
            "memory.short.rollback_seed",
            "rollback baseline entry",
            MemoryType::Short,
        ))
        .expect("short memory write should succeed");
    source.trainer_trigger_queue.push(TrainerTriggerEvent {
        event_id: ProtocolId::default(),
        model_version: Version::new(1, 0, 0),
        training_bundle_checksum: "bundle-rollback-333".to_string(),
        included_entries: 32,
        train_samples: 26,
        validation_samples: 6,
        generated_at: 77,
        delivery_attempts: 0,
        last_attempted_at: None,
    });

    let baseline_bundle = source.export_runtime_bundle();
    let mut target = MoePipelineBuilder::new().build();
    target
        .import_runtime_bundle(baseline_bundle.clone())
        .expect("baseline runtime bundle import should succeed");
    let telemetry_before = target.import_telemetry_snapshot();
    let runtime_checksum_before = target.export_runtime_bundle().runtime_checksum;
    let report_checksum_before = target
        .last_runtime_import_report()
        .expect("baseline import should capture runtime import report")
        .runtime_checksum_after_import
        .clone();

    let mut invalid_bundle = baseline_bundle.clone();
    invalid_bundle.trainer_trigger_leased_event_ids = vec![ProtocolId::default()];
    invalid_bundle.runtime_checksum = invalid_bundle.recompute_checksum();

    let err = target
        .import_runtime_bundle(invalid_bundle)
        .expect_err("invalid leased ids should fail invariant validation");
    assert!(
        err.to_string()
            .contains("runtime bundle import failed and was rolled back")
    );

    let telemetry_after = target.import_telemetry_snapshot();
    assert_eq!(
        telemetry_after.runtime_bundle_import_successes,
        telemetry_before.runtime_bundle_import_successes
    );
    assert_eq!(
        telemetry_after.runtime_bundle_import_rejections,
        telemetry_before.runtime_bundle_import_rejections + 1
    );
    assert_eq!(
        telemetry_after.governance_bundle_import_successes,
        telemetry_before.governance_bundle_import_successes
    );
    assert_eq!(
        telemetry_after.runtime_bundle_import_expired_leases_released_total,
        telemetry_before.runtime_bundle_import_expired_leases_released_total
    );
    assert_eq!(
        telemetry_after.runtime_bundle_import_dead_letter_events_observed_total,
        telemetry_before.runtime_bundle_import_dead_letter_events_observed_total
    );

    let runtime_checksum_after = target.export_runtime_bundle().runtime_checksum;
    assert_eq!(runtime_checksum_after, runtime_checksum_before);
    let report_checksum_after = target
        .last_runtime_import_report()
        .expect("failed import should preserve previous runtime import report")
        .runtime_checksum_after_import
        .clone();
    assert_eq!(report_checksum_after, report_checksum_before);
}

#[test]
fn import_runtime_bundle_rejects_memory_id_overlap_between_tiers() {
    let source = MoePipelineBuilder::new().build();
    let mut bundle = source.export_runtime_bundle();
    let shared_id = "memory.shared.id";
    bundle.short_term_memory_entries.push(test_memory_entry(
        shared_id,
        "short tier value",
        MemoryType::Short,
    ));
    bundle.long_term_memory_entries.push(test_memory_entry(
        shared_id,
        "long tier value",
        MemoryType::Long,
    ));
    bundle.runtime_checksum = bundle.recompute_checksum();

    let mut target = MoePipelineBuilder::new().build();
    let err = target
        .import_runtime_bundle(bundle)
        .expect_err("memory id overlap must be rejected");
    assert!(matches!(err, MoeError::PolicyRejected(_)));
    assert!(
        err.to_string()
            .contains("overlap between short and long term")
    );
}

#[test]
fn preview_runtime_bundle_import_json_rejects_oversized_payload() {
    let oversized_payload = "x".repeat((16 * 1024 * 1024) + 1);
    let target = MoePipelineBuilder::new().build();
    let err = target
        .preview_runtime_bundle_import_json(&oversized_payload)
        .expect_err("oversized payload must be rejected before deserialization");
    assert!(matches!(err, MoeError::PolicyRejected(_)));
    assert!(err.to_string().contains("payload too large"));
}

#[test]
fn compare_and_import_runtime_bundle_rejects_version_mismatch() {
    let mut source = MoePipelineBuilder::new().build();
    source
        .remember_short_term(test_memory_entry(
            "memory.short.compare_and_import",
            "compare and import runtime payload",
            MemoryType::Short,
        ))
        .expect("short memory write should succeed");
    let bundle = source.export_runtime_bundle();

    let mut target = MoePipelineBuilder::new().build();
    let err = target
        .compare_and_import_runtime_bundle(Version::new(1, 0, 0), bundle)
        .expect_err("version mismatch must be rejected");
    assert!(matches!(err, MoeError::PolicyRejected(_)));
    assert!(err.to_string().contains("compare-and-import rejected"));
}

#[test]
fn compare_and_import_runtime_bundle_succeeds_on_matching_version() {
    let mut source = MoePipelineBuilder::new().build();
    source
        .remember_short_term(test_memory_entry(
            "memory.short.compare_and_import_ok",
            "compare and import runtime payload ok",
            MemoryType::Short,
        ))
        .expect("short memory write should succeed");
    let bundle = source.export_runtime_bundle();

    let mut target = MoePipelineBuilder::new().build();
    target
        .compare_and_import_runtime_bundle(Version::new(1, 0, 0), bundle)
        .expect("matching version should allow compare-and-import");
    let restored = target.export_runtime_bundle();
    assert!(restored.verify_checksum());
}

#[test]
fn compare_and_import_runtime_bundle_with_checksum_rejects_checksum_mismatch() {
    let mut source = MoePipelineBuilder::new().build();
    source
        .remember_short_term(test_memory_entry(
            "memory.short.compare_and_import_checksum",
            "compare and import runtime payload checksum",
            MemoryType::Short,
        ))
        .expect("short memory write should succeed");
    let bundle = source.export_runtime_bundle();

    let mut target = MoePipelineBuilder::new().build();
    let err = target
        .compare_and_import_runtime_bundle_with_checksum(Version::new(1, 0, 0), "deadbeef", bundle)
        .expect_err("checksum mismatch must be rejected");
    assert!(matches!(err, MoeError::PolicyRejected(_)));
    assert!(err.to_string().contains("expected governance checksum"));
}

#[test]
fn compare_and_import_runtime_bundle_with_checksum_succeeds_on_match() {
    let mut source = MoePipelineBuilder::new().build();
    source
        .remember_short_term(test_memory_entry(
            "memory.short.compare_and_import_checksum_ok",
            "compare and import runtime payload checksum ok",
            MemoryType::Short,
        ))
        .expect("short memory write should succeed");
    let bundle = source.export_runtime_bundle();

    let mut target = MoePipelineBuilder::new().build();
    let expected_checksum = target.export_governance_state().state_checksum;
    target
        .compare_and_import_runtime_bundle_with_checksum(
            Version::new(1, 0, 0),
            &expected_checksum,
            bundle,
        )
        .expect("matching checksum should allow compare-and-import");
    let restored = target.export_runtime_bundle();
    assert!(restored.verify_checksum());
}

#[test]
fn compare_and_import_runtime_bundle_json_succeeds_on_matching_version() {
    let mut source = MoePipelineBuilder::new().build();
    source
        .remember_short_term(test_memory_entry(
            "memory.short.compare_and_import_json_ok",
            "compare and import runtime payload json ok",
            MemoryType::Short,
        ))
        .expect("short memory write should succeed");
    let payload = source
        .export_runtime_bundle_json()
        .expect("runtime bundle json export should succeed");

    let mut target = MoePipelineBuilder::new().build();
    target
        .compare_and_import_runtime_bundle_json(Version::new(1, 0, 0), &payload)
        .expect("matching version should allow compare-and-import json");
    let restored = target.export_runtime_bundle();
    assert!(restored.verify_checksum());
}

#[test]
#[ignore = "perf smoke test; run manually when profiling checksum throughput"]
fn runtime_bundle_checksum_perf_smoke_large_payload() {
    let mut source = MoePipelineBuilder::new().build();
    for idx in 0..5_000_u32 {
        source
            .remember_short_term(test_memory_entry(
                &format!("memory.short.perf.{idx}"),
                &format!("perf short-term entry {idx}"),
                MemoryType::Short,
            ))
            .expect("short memory write should succeed");
        source
            .remember_long_term(test_memory_entry(
                &format!("memory.long.perf.{idx}"),
                &format!("perf long-term entry {idx}"),
                MemoryType::Long,
            ))
            .expect("long memory write should succeed");
        source.put_session_buffer(
            &ProtocolId::default(),
            format!("checkpoint.{idx}"),
            format!("perf snapshot {idx}"),
        );
    }

    let bundle = source.export_runtime_bundle();
    let baseline_checksum = bundle.recompute_checksum();
    assert!(!baseline_checksum.is_empty());
    for _ in 0..5 {
        let next_checksum = bundle.recompute_checksum();
        assert_eq!(
            next_checksum, baseline_checksum,
            "checksum recomputation should remain stable under repeated calls"
        );
    }
}

#[test]
fn import_runtime_bundle_json_rejects_malformed_payload_variants() {
    let mut source = MoePipelineBuilder::new().build();
    source
        .remember_short_term(test_memory_entry(
            "memory.short.malformed-seed",
            "seed for malformed payload checks",
            MemoryType::Short,
        ))
        .expect("short memory write should succeed");
    let payload = source
        .export_runtime_bundle_json()
        .expect("runtime bundle json export should succeed");

    let malformed_payloads = vec![
        payload.replacen('{', "[", 1).replacen('}', "]", 1),
        payload.replacen(':', ";", 1),
        format!("{payload} trailing-garbage"),
        payload
            .strip_suffix('}')
            .expect("payload should end with object close")
            .to_string(),
    ];

    for malformed in malformed_payloads {
        let mut target = MoePipelineBuilder::new().build();
        assert_runtime_json_rejected(&mut target, &malformed);
    }
}

#[test]
fn import_runtime_bundle_json_rejects_corruption_matrix_with_stale_checksum() {
    let mut source = MoePipelineBuilder::new().build();
    source
        .remember_short_term(test_memory_entry(
            "memory.short.corruption-matrix",
            "baseline short-term content",
            MemoryType::Short,
        ))
        .expect("short memory write should succeed");
    source
        .remember_long_term(test_memory_entry(
            "memory.long.corruption-matrix",
            "baseline long-term content",
            MemoryType::Long,
        ))
        .expect("long memory write should succeed");
    source.put_session_buffer(
        &ProtocolId::default(),
        "checkpoint",
        "baseline session payload",
    );

    let base_bundle = source.export_runtime_bundle();
    let mut variants = Vec::new();

    let mut short_mutated = base_bundle.clone();
    short_mutated.short_term_memory_entries[0].content = "tampered short-term content".to_string();
    variants.push(short_mutated);

    let mut long_mutated = base_bundle.clone();
    long_mutated.long_term_memory_entries[0].content = "tampered long-term content".to_string();
    variants.push(long_mutated);

    let mut session_mutated = base_bundle.clone();
    session_mutated.buffer_manager.sessions_mut().put(
        &ProtocolId::default(),
        "checkpoint.extra",
        "tampered extra checkpoint",
    );
    variants.push(session_mutated);

    let mut governance_mutated = base_bundle.clone();
    governance_mutated.governance.state.state_checksum = "deadbeef".to_string();
    variants.push(governance_mutated);

    for variant in variants {
        let payload = common_json::json::to_json_string_pretty(&variant)
            .expect("variant payload serialization should succeed");
        let mut target = MoePipelineBuilder::new().build();
        assert_runtime_json_rejected(&mut target, &payload);
    }
}
