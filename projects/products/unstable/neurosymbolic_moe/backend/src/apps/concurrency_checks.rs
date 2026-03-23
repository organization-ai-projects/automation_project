//! projects/products/unstable/neurosymbolic_moe/backend/src/apps/concurrency_checks.rs

use std::collections::HashMap;

use protocol::ProtocolId;

use crate::{
    apps::DynError,
    echo_expert::EchoExpert,
    memory_engine::{MemoryEntry, MemoryType},
    moe_core::{ExpertCapability, Task, TaskType},
    orchestrator::{ConcurrentMoePipeline, ConcurrentOperationalReport, MoePipelineBuilder},
};

pub(crate) fn run_concurrent_pipeline_checks() -> Result<(), DynError> {
    run_concurrent_pipeline_checks_with_report().map(|_| ())
}

pub(crate) fn run_concurrent_pipeline_checks_with_report()
-> Result<ConcurrentOperationalReport, DynError> {
    let concurrent_pipeline = ConcurrentMoePipeline::from_builder(MoePipelineBuilder::new());
    let expert_id = ProtocolId::default();
    let task_id = ProtocolId::default();
    concurrent_pipeline.register_expert(Box::new(EchoExpert::new_with_id(
        expert_id,
        "concurrent_runtime",
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
    let concurrent_result = concurrent_pipeline.execute(Task::new_with_id(
        task_id,
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
        concurrent_guard.current_version.clone(),
        &concurrent_runtime_payload,
    )?;
    concurrent_pipeline.compare_and_import_governance_bundle_json(
        concurrent_guard.current_version.clone(),
        &concurrent_governance_bundle_payload,
    )?;
    concurrent_pipeline.compare_and_import_governance_state_json(
        concurrent_guard.current_version,
        &concurrent_governance_state_payload,
    )?;

    let concurrent_guard = concurrent_pipeline.governance_audit_trail()?;
    let concurrent_checksum = governance_checksum_for_concurrent_pipeline(&concurrent_pipeline)?;
    concurrent_pipeline.compare_and_import_runtime_bundle_json_with_checksum(
        &concurrent_guard.current_version,
        &concurrent_checksum,
        &concurrent_runtime_payload,
    )?;
    concurrent_pipeline.compare_and_import_governance_bundle_json_with_checksum(
        concurrent_guard.current_version.clone(),
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
    let import_telemetry = concurrent_pipeline.import_telemetry_snapshot()?;
    concurrent_pipeline.configure_write_guard(0.4, 4)?;
    let operational_report_json = concurrent_pipeline.export_operational_report_json()?;
    let operational_report = concurrent_pipeline.export_operational_report()?;
    let slo_status = operational_report.slo_status(1.0, 0.2, 1, 0, 0);
    let slo_violations = operational_report.slo_violations(1.0, 0.2, 1, 0, 0);
    let prometheus_text = operational_report.to_prometheus_text("moe_concurrency");
    let slo_ok = concurrent_pipeline.is_within_lock_slo(1.0, 0.1);
    if import_telemetry.governance_state_import_successes == 0
        || import_telemetry.governance_bundle_import_successes == 0
        || import_telemetry.runtime_bundle_import_successes == 0
    {
        return Err(std::io::Error::other(
            "concurrent checks expected non-zero import telemetry successes",
        )
        .into());
    }
    tracing::info!(
        "Concurrent checks: outputs={} state_allowed={} bundle_allowed={} runtime_allowed={} read_probe={} write_probe={} metrics={} lock_contention_rate={:.4} lock_timeout_rate={:.4} lock_slo_ok={} import_state_ok={} import_bundle_ok={} import_runtime_ok={} import_json_parse_failures={} operational_report_bytes={} slo_status={} slo_violations={} prometheus_bytes={} write_guard_rejections={}",
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
        import_telemetry.governance_state_import_successes,
        import_telemetry.governance_bundle_import_successes,
        import_telemetry.runtime_bundle_import_successes,
        import_telemetry.json_parse_failures,
        operational_report_json.len(),
        slo_status,
        slo_violations.len(),
        prometheus_text.len(),
        operational_report.write_guard_rejections,
    );
    if slo_status != "OK" {
        return Err(std::io::Error::other(format!(
            "concurrent SLO gate failed: {}",
            slo_violations.join("; ")
        ))
        .into());
    }
    Ok(operational_report)
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
