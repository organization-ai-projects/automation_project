use crate::memory_engine::{MemoryEntry, MemoryType};
use crate::moe_core::MoeError;
use crate::orchestrator::{MoePipelineBuilder, RuntimePersistenceBundle};
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
                "{}|{}|{}|{}",
                entry.id,
                entry.content,
                entry.relevance,
                format!("{:?}", entry.memory_type)
            )
        })
        .collect::<Vec<_>>();
    rows.sort();
    rows
}

#[test]
fn runtime_bundle_roundtrip_restores_state_memory_and_buffers() {
    let mut source = MoePipelineBuilder::new().build();
    source
        .remember_short_term(test_memory_entry(
            "short-1",
            "short memory payload",
            MemoryType::Short,
        ))
        .expect("short memory write should succeed");
    source
        .remember_long_term(test_memory_entry(
            "long-1",
            "long memory payload",
            MemoryType::Long,
        ))
        .expect("long memory write should succeed");
    source.put_session_buffer("session-1", "note", "value-1");

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
        restored.buffer_manager.sessions().values("session-1"),
        bundle.buffer_manager.sessions().values("session-1")
    );
}

#[test]
fn import_runtime_bundle_rejects_checksum_drift() {
    let mut source = MoePipelineBuilder::new().build();
    source
        .remember_short_term(test_memory_entry(
            "short-1",
            "baseline payload",
            MemoryType::Short,
        ))
        .expect("short memory write should succeed");

    let mut tampered = source.export_runtime_bundle();
    tampered.short_term_memory_entries[0].content = "tampered payload".to_string();

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
            "short-json-1",
            "json short payload",
            MemoryType::Short,
        ))
        .expect("short memory write should succeed");
    source.put_session_buffer("session-json", "k", "v");

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
        restored.buffer_manager.sessions().values("session-json"),
        vec!["v".to_string()]
    );
}

#[test]
fn preview_runtime_bundle_import_json_rejects_checksum_tampering() {
    let mut source = MoePipelineBuilder::new().build();
    source
        .remember_short_term(test_memory_entry(
            "short-preview-1",
            "baseline payload",
            MemoryType::Short,
        ))
        .expect("short memory write should succeed");

    let payload = source
        .export_runtime_bundle_json()
        .expect("runtime bundle json export should succeed");
    let tampered_payload = payload.replace("baseline payload", "tampered payload");

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
