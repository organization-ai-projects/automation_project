//! projects/products/unstable/neurosymbolic_moe/backend/src/apps/concurrency_checks.rs

use std::collections::HashMap;

use crate::{
    apps::DynError,
    echo_expert::EchoExpert,
    memory_engine::{MemoryEntry, MemoryType},
    moe_core::{ExpertCapability, Task, TaskType},
    orchestrator::{ConcurrentMoePipeline, MoePipelineBuilder},
};

pub(crate) fn run_concurrent_pipeline_checks() -> Result<(), DynError> {
    let concurrent_pipeline = ConcurrentMoePipeline::from_builder(MoePipelineBuilder::new());
    concurrent_pipeline.register_expert(Box::new(EchoExpert::new(
        "concurrent_runtime",
        "ConcurrentRuntime",
        vec![ExpertCapability::CodeGeneration],
    )))?;
    concurrent_pipeline.remember_short_term(MemoryEntry {
        id: "impl-concurrent-stm".to_string(),
        content: "concurrent recent".to_string(),
        tags: vec!["impl-check".to_string(), "concurrent".to_string()],
        created_at: 3,
        expires_at: None,
        memory_type: MemoryType::Short,
        relevance: 0.9,
        metadata: HashMap::new(),
    })?;
    concurrent_pipeline.remember_long_term(MemoryEntry {
        id: "impl-concurrent-ltm".to_string(),
        content: "concurrent historical".to_string(),
        tags: vec!["impl-check".to_string(), "concurrent".to_string()],
        created_at: 4,
        expires_at: None,
        memory_type: MemoryType::Long,
        relevance: 0.85,
        metadata: HashMap::new(),
    })?;
    let concurrent_result = concurrent_pipeline.execute(Task::new(
        "impl-concurrent-task",
        TaskType::CodeGeneration,
        "concurrent payload",
    ))?;
    let (
        concurrent_runtime_payload,
        concurrent_governance_bundle_payload,
        concurrent_governance_state_payload,
    ) = export_concurrent_payloads(&concurrent_pipeline)?;
    let concurrent_state_preview =
        concurrent_pipeline.preview_governance_import_json(&concurrent_governance_state_payload)?;
    let concurrent_bundle_preview = concurrent_pipeline
        .preview_governance_bundle_import_json(&concurrent_governance_bundle_payload)?;
    let concurrent_runtime_preview =
        concurrent_pipeline.preview_runtime_bundle_import_json(&concurrent_runtime_payload)?;
    roundtrip_import_concurrent_payloads(
        &concurrent_pipeline,
        &concurrent_runtime_payload,
        &concurrent_governance_bundle_payload,
        &concurrent_governance_state_payload,
    )?;

    let (
        concurrent_runtime_payload,
        concurrent_governance_bundle_payload,
        concurrent_governance_state_payload,
    ) = export_concurrent_payloads(&concurrent_pipeline)?;
    let concurrent_guard = concurrent_pipeline.governance_audit_trail()?;
    concurrent_pipeline.compare_and_import_runtime_bundle_json(
        concurrent_guard.current_version,
        &concurrent_runtime_payload,
    )?;
    concurrent_pipeline.compare_and_import_governance_bundle_json(
        concurrent_guard.current_version,
        &concurrent_governance_bundle_payload,
    )?;
    concurrent_pipeline.compare_and_import_governance_state_json(
        concurrent_guard.current_version,
        &concurrent_governance_state_payload,
    )?;

    let concurrent_guard = concurrent_pipeline.governance_audit_trail()?;
    let concurrent_checksum = governance_checksum_for_concurrent_pipeline(&concurrent_pipeline)?;
    concurrent_pipeline.compare_and_import_runtime_bundle_json_with_checksum(
        concurrent_guard.current_version,
        &concurrent_checksum,
        &concurrent_runtime_payload,
    )?;
    concurrent_pipeline.compare_and_import_governance_bundle_json_with_checksum(
        concurrent_guard.current_version,
        &concurrent_checksum,
        &concurrent_governance_bundle_payload,
    )?;
    concurrent_pipeline.compare_and_import_governance_state_json_with_checksum(
        concurrent_guard.current_version,
        &concurrent_checksum,
        &concurrent_governance_state_payload,
    )?;
    let concurrent_read_probe = concurrent_pipeline.with_read_timeout(3, |_| true)?;
    concurrent_pipeline.with_write_timeout(3, |_| Ok(()))?;
    let concurrent_write_probe = true;
    let concurrent_metrics = concurrent_pipeline.metrics();
    let lock_snapshot = concurrent_pipeline.metrics_snapshot();
    let slo_ok = concurrent_pipeline.is_within_lock_slo(1.0, 0.1);
    tracing::info!(
        "Concurrent checks: outputs={} state_allowed={} bundle_allowed={} runtime_allowed={} read_probe={} write_probe={} metrics={} lock_contention_rate={:.4} lock_timeout_rate={:.4} lock_slo_ok={}",
        concurrent_result.outputs.len(),
        concurrent_state_preview.allowed,
        concurrent_bundle_preview.allowed,
        concurrent_runtime_preview.allowed,
        concurrent_read_probe,
        concurrent_write_probe,
        concurrent_metrics.len(),
        lock_snapshot.contention_rate(),
        lock_snapshot.timeout_rate(),
        slo_ok,
    );
    Ok(())
}

fn governance_checksum_for_concurrent_pipeline(
    pipeline: &ConcurrentMoePipeline,
) -> Result<String, DynError> {
    let trail = pipeline.governance_audit_trail()?;
    if let Some(checksum) = trail.current_checksum {
        Ok(checksum)
    } else {
        Ok(pipeline.with_read(|inner| inner.export_governance_state().state_checksum)?)
    }
}

fn export_concurrent_payloads(
    pipeline: &ConcurrentMoePipeline,
) -> Result<(String, String, String), DynError> {
    Ok((
        pipeline.export_runtime_bundle_json()?,
        pipeline.export_governance_bundle_json()?,
        pipeline.export_governance_state_json()?,
    ))
}

fn roundtrip_import_concurrent_payloads(
    pipeline: &ConcurrentMoePipeline,
    runtime_payload: &str,
    governance_bundle_payload: &str,
    governance_state_payload: &str,
) -> Result<(), DynError> {
    pipeline.try_import_governance_state_json(governance_state_payload)?;
    pipeline.try_import_governance_bundle_json(governance_bundle_payload)?;
    pipeline.try_import_runtime_bundle_json(runtime_payload)?;
    pipeline.import_governance_state_json(governance_state_payload)?;
    pipeline.import_governance_bundle_json(governance_bundle_payload)?;
    pipeline.import_runtime_bundle_json(runtime_payload)?;
    Ok(())
}
