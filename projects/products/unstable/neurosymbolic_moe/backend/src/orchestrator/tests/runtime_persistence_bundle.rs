use crate::memory_engine::{MemoryEntry, MemoryType};
use crate::moe_core::MoeError;
use crate::orchestrator::{
    GovernancePersistenceBundle, MoePipelineBuilder, RuntimePersistenceBundle,
};
use crate::{buffer_manager::BufferManager, memory_engine::MemoryEntry as RuntimeMemoryEntry};
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
        "session-incident-42",
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
            .values("session-incident-42"),
        bundle
            .buffer_manager
            .sessions()
            .values("session-incident-42")
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
        "session-runtime-json",
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
            .values("session-runtime-json"),
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
        "session-legacy-import",
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
            .values("session-legacy-import"),
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
            "session-bulk-load",
            format!("checkpoint.{idx}"),
            format!("state snapshot marker {idx}"),
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
            .values("session-bulk-load")
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
