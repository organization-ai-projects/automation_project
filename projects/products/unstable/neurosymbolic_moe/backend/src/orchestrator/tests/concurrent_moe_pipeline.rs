//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/tests/concurrent_moe_pipeline.rs
use crate::memory_engine::{MemoryEntry, MemoryType};
use crate::moe_core::{
    ExecutionContext, Expert, ExpertCapability, ExpertError, ExpertId, ExpertMetadata,
    ExpertOutput, ExpertStatus, ExpertType, Task, TaskType,
};
use crate::orchestrator::{
    ConcurrentMoePipeline, ConcurrentOperationalReport, GovernanceState, MoePipelineBuilder,
};
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread;

struct ConcurrentTestExpert {
    meta: ExpertMetadata,
}

impl ConcurrentTestExpert {
    fn new(id: &str) -> Self {
        Self {
            meta: ExpertMetadata {
                id: ExpertId::new(id),
                name: id.to_string(),
                version: "1.0.0".to_string(),
                capabilities: vec![ExpertCapability::CodeGeneration],
                status: ExpertStatus::Active,
                expert_type: ExpertType::Deterministic,
            },
        }
    }
}

impl Expert for ConcurrentTestExpert {
    fn id(&self) -> &ExpertId {
        &self.meta.id
    }

    fn metadata(&self) -> &ExpertMetadata {
        &self.meta
    }

    fn can_handle(&self, task: &Task) -> bool {
        matches!(task.task_type(), TaskType::CodeGeneration)
    }

    fn execute(
        &self,
        task: &Task,
        _context: &ExecutionContext,
    ) -> Result<ExpertOutput, ExpertError> {
        Ok(ExpertOutput {
            expert_id: self.meta.id.clone(),
            content: task.input().to_string(),
            confidence: 0.9,
            metadata: HashMap::new(),
            trace: Vec::new(),
        })
    }
}

#[test]
fn concurrent_pipeline_supports_parallel_execution() {
    let pipeline = ConcurrentMoePipeline::from_builder(MoePipelineBuilder::new());
    pipeline
        .register_expert(Box::new(ConcurrentTestExpert::new("concurrent-expert")))
        .expect("expert registration should succeed");

    let mut handles = Vec::new();
    for worker in 0..6_u32 {
        let pipeline = pipeline.clone();
        handles.push(thread::spawn(move || {
            for idx in 0..20_u32 {
                let task = Task::new(
                    format!("concurrent-task-{worker}-{idx}"),
                    TaskType::CodeGeneration,
                    format!("payload-{worker}-{idx}"),
                );
                let result = pipeline.execute(task).expect("execution should succeed");
                assert!(result.selected_output.is_some());
            }
        }));
    }

    for handle in handles {
        handle.join().expect("worker should not panic");
    }

    let trace_count = pipeline
        .with_read(|inner| inner.trace_logger().count())
        .expect("trace count read should succeed");
    assert!(trace_count > 0);
}

#[test]
fn concurrent_pipeline_supports_parallel_memory_writes_and_export_reads() {
    let pipeline = ConcurrentMoePipeline::from_builder(MoePipelineBuilder::new());
    pipeline
        .remember_long_term(MemoryEntry {
            id: "memory.long.concurrent-seed".to_string(),
            content: "long-content-seed".to_string(),
            tags: vec!["concurrent".to_string()],
            created_at: 1,
            expires_at: None,
            memory_type: MemoryType::Long,
            relevance: 0.8,
            metadata: HashMap::new(),
        })
        .expect("long-term write should succeed");

    let mut handles = Vec::new();
    for worker in 0..4_u32 {
        let pipeline = pipeline.clone();
        handles.push(thread::spawn(move || {
            for idx in 0..32_u32 {
                let entry = MemoryEntry {
                    id: format!("memory.short.concurrent-{worker}-{idx}"),
                    content: format!("content-{worker}-{idx}"),
                    tags: vec!["concurrent".to_string()],
                    created_at: (worker as u64) * 1_000 + idx as u64,
                    expires_at: None,
                    memory_type: MemoryType::Short,
                    relevance: 0.7,
                    metadata: HashMap::new(),
                };
                pipeline
                    .remember_short_term(entry)
                    .expect("short-term write should succeed");
                let _ = pipeline
                    .export_governance_state_json()
                    .expect("state export read should succeed");
            }
        }));
    }

    for reader in 0..2_u32 {
        let pipeline = pipeline.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..48_u32 {
                let json = pipeline
                    .export_runtime_bundle_json()
                    .expect("runtime export read should succeed");
                assert!(!json.is_empty(), "reader {reader} should receive payload");
            }
        }));
    }

    for handle in handles {
        handle.join().expect("concurrent worker should not panic");
    }

    let short_count = pipeline
        .with_read(|inner| {
            inner
                .export_runtime_bundle()
                .short_term_memory_entries
                .len()
        })
        .expect("runtime snapshot read should succeed");
    assert!(short_count > 0);
    assert!(short_count <= 256);

    let trail = pipeline
        .governance_audit_trail()
        .expect("governance trail read should succeed");
    let expected_checksum = pipeline
        .with_read(|inner| {
            trail
                .current_checksum
                .clone()
                .unwrap_or_else(|| inner.export_governance_state().state_checksum)
        })
        .expect("governance checksum read should succeed");
    let payload = pipeline
        .export_runtime_bundle_json()
        .expect("runtime bundle export should succeed");
    pipeline
        .compare_and_import_runtime_bundle_json_with_checksum(
            trail.current_version,
            &expected_checksum,
            &payload,
        )
        .expect("runtime CAS import with checksum should succeed");

    let metrics = pipeline.metrics();
    let snapshot = pipeline.metrics_snapshot();
    assert!(
        metrics
            .get("read_lock_acquisitions")
            .copied()
            .unwrap_or_default()
            > 0
    );
    assert!(
        metrics
            .get("write_lock_acquisitions")
            .copied()
            .unwrap_or_default()
            > 0
    );
    assert!(snapshot.total_lock_acquisitions() > 0);
    assert!(snapshot.contention_rate() >= 0.0);
    assert!(snapshot.timeout_rate() >= 0.0);
    assert!(pipeline.is_within_lock_slo(2.0, 1.0));
}

#[test]
fn concurrent_pipeline_reports_lock_timeout_metrics_under_contention() {
    let pipeline = ConcurrentMoePipeline::from_builder(MoePipelineBuilder::new());
    let (tx, rx) = mpsc::channel::<()>();
    let locked = pipeline.clone();
    let holder = thread::spawn(move || {
        locked
            .with_write(|_| {
                tx.send(()).expect("lock acquisition signal should be sent");
                for _ in 0..100_000 {
                    std::thread::yield_now();
                }
                Ok(())
            })
            .expect("holding write lock should succeed");
    });

    rx.recv()
        .expect("write lock acquisition signal should be received");
    let err = pipeline
        .with_read_timeout(10, |_| 42_u32)
        .expect_err("read should timeout while write lock is held");
    assert!(err.to_string().contains("timeout"));

    holder.join().expect("holder thread should not panic");
    pipeline
        .with_write_timeout(10, |_| Ok(()))
        .expect("write lock timeout API should succeed when lock is available");
    let metrics = pipeline.metrics();
    let snapshot = pipeline.metrics_snapshot();
    assert!(
        metrics
            .get("read_lock_timeouts")
            .copied()
            .unwrap_or_default()
            >= 1
    );
    assert!(
        metrics
            .get("read_lock_contention")
            .copied()
            .unwrap_or_default()
            >= 1
    );
    assert!(
        metrics
            .get("write_lock_acquisitions")
            .copied()
            .unwrap_or_default()
            >= 1
    );
    assert!(snapshot.total_timeout_events() >= 1);
    assert!(!pipeline.is_within_lock_slo(1.0, 0.0));
}

#[test]
fn concurrent_pipeline_timeout_apis_normalize_zero_attempts() {
    let pipeline = ConcurrentMoePipeline::from_builder(MoePipelineBuilder::new());
    pipeline
        .with_read_timeout(0, |_| ())
        .expect("zero read attempts should be normalized to one");
    pipeline
        .with_write_timeout(0, |_| Ok(()))
        .expect("zero write attempts should be normalized to one");
    let metrics = pipeline.metrics();
    assert!(
        metrics
            .get("read_lock_spin_attempts_avg_milli")
            .copied()
            .unwrap_or_default()
            <= 1000
    );
    assert!(
        metrics
            .get("write_lock_spin_attempts_avg_milli")
            .copied()
            .unwrap_or_default()
            <= 1000
    );
}

#[test]
fn concurrent_pipeline_chaos_contention_recovers_after_lock_storm() {
    let pipeline = ConcurrentMoePipeline::from_builder(MoePipelineBuilder::new());
    pipeline
        .register_expert(Box::new(ConcurrentTestExpert::new(
            "concurrent-chaos-expert",
        )))
        .expect("expert registration should succeed");

    let stop_signal = Arc::new(AtomicBool::new(false));
    let (lock_tx, lock_rx) = mpsc::channel::<()>();
    let holder_pipeline = pipeline.clone();
    let holder_stop = Arc::clone(&stop_signal);
    let holder = thread::spawn(move || {
        holder_pipeline
            .with_write(|_| {
                lock_tx
                    .send(())
                    .expect("lock holder should signal acquisition");
                while !holder_stop.load(Ordering::Relaxed) {
                    std::thread::yield_now();
                }
                Ok(())
            })
            .expect("holder write lock should succeed");
    });

    lock_rx
        .recv()
        .expect("lock holder acquisition signal should be received");

    let mut handles = Vec::new();
    for worker in 0..4_u32 {
        let reader_pipeline = pipeline.clone();
        handles.push(thread::spawn(move || {
            let mut timeouts = 0_u32;
            for _ in 0..40_u32 {
                if reader_pipeline.with_read_timeout(3, |_| ()).is_err() {
                    timeouts += 1;
                }
            }
            timeouts
        }));

        let writer_pipeline = pipeline.clone();
        handles.push(thread::spawn(move || {
            let mut timeouts = 0_u32;
            for idx in 0..40_u32 {
                let entry = MemoryEntry {
                    id: format!("memory.short.chaos-{worker}-{idx}"),
                    content: format!("chaos-content-{worker}-{idx}"),
                    tags: vec!["chaos".to_string()],
                    created_at: u64::from(worker) * 1_000 + u64::from(idx),
                    expires_at: None,
                    memory_type: MemoryType::Short,
                    relevance: 0.6,
                    metadata: HashMap::new(),
                };
                if writer_pipeline
                    .with_write_timeout(3, |inner| inner.remember_short_term(entry))
                    .is_err()
                {
                    timeouts += 1;
                }
            }
            timeouts
        }));
    }

    let mut observed_worker_timeouts = 0_u32;
    for handle in handles {
        observed_worker_timeouts += handle.join().expect("worker should not panic");
    }
    assert!(observed_worker_timeouts > 0);

    stop_signal.store(true, Ordering::Relaxed);
    holder.join().expect("holder thread should not panic");

    pipeline
        .with_read_timeout(3, |_| ())
        .expect("read timeout API should recover after lock storm");
    pipeline
        .with_write_timeout(3, |_| Ok(()))
        .expect("write timeout API should recover after lock storm");
    let execute_result = pipeline
        .execute(Task::new(
            "chaos-recovery-task",
            TaskType::CodeGeneration,
            "post-chaos execution",
        ))
        .expect("pipeline should remain operable after contention storm");
    assert!(execute_result.selected_output.is_some());

    let snapshot = pipeline.metrics_snapshot();
    assert!(snapshot.read_lock_contention > 0);
    assert!(snapshot.write_lock_contention > 0);
    assert!(snapshot.read_lock_timeouts > 0);
    assert!(snapshot.write_lock_timeouts > 0);
    assert!(snapshot.total_lock_acquisitions() > 0);
    assert!(snapshot.avg_read_spin_attempts() >= 0.0);
    assert!(snapshot.avg_write_spin_attempts() >= 0.0);
    assert!(!pipeline.is_within_lock_slo(0.1, 0.01));
}

#[test]
fn concurrent_pipeline_compare_and_import_with_checksum_rejects_mismatch_for_all_payloads() {
    let pipeline = ConcurrentMoePipeline::from_builder(MoePipelineBuilder::new());
    let runtime_payload = pipeline
        .export_runtime_bundle_json()
        .expect("runtime payload export should succeed");
    let governance_bundle_payload = pipeline
        .export_governance_bundle_json()
        .expect("governance bundle payload export should succeed");
    let governance_state_payload = pipeline
        .export_governance_state_json()
        .expect("governance state payload export should succeed");
    let version = pipeline
        .governance_audit_trail()
        .expect("governance trail read should succeed")
        .current_version;

    let runtime_err = pipeline
        .compare_and_import_runtime_bundle_json_with_checksum(version, "deadbeef", &runtime_payload)
        .expect_err("runtime CAS import should reject checksum mismatch");
    assert!(
        runtime_err
            .to_string()
            .contains("expected governance checksum")
    );

    let bundle_err = pipeline
        .compare_and_import_governance_bundle_json_with_checksum(
            version,
            "deadbeef",
            &governance_bundle_payload,
        )
        .expect_err("bundle CAS import should reject checksum mismatch");
    assert!(
        bundle_err
            .to_string()
            .contains("expected governance checksum")
    );

    let state_err = pipeline
        .compare_and_import_governance_state_json_with_checksum(
            version,
            "deadbeef",
            &governance_state_payload,
        )
        .expect_err("state CAS import should reject checksum mismatch");
    assert!(
        state_err
            .to_string()
            .contains("expected governance checksum")
    );
}

#[test]
fn concurrent_pipeline_compare_and_import_with_checksum_accepts_match_for_all_payloads() {
    let runtime_pipeline = ConcurrentMoePipeline::from_builder(MoePipelineBuilder::new());
    let runtime_payload = runtime_pipeline
        .export_runtime_bundle_json()
        .expect("runtime payload export should succeed");
    let runtime_trail = runtime_pipeline
        .governance_audit_trail()
        .expect("runtime trail read should succeed");
    let runtime_checksum = runtime_trail.current_checksum.clone().unwrap_or_else(|| {
        runtime_pipeline
            .with_read(|inner| inner.export_governance_state().state_checksum)
            .expect("runtime checksum fallback read should succeed")
    });
    runtime_pipeline
        .compare_and_import_runtime_bundle_json_with_checksum(
            runtime_trail.current_version,
            &runtime_checksum,
            &runtime_payload,
        )
        .expect("runtime CAS import should accept checksum match");

    let bundle_pipeline = ConcurrentMoePipeline::from_builder(MoePipelineBuilder::new());
    let bundle_payload = bundle_pipeline
        .export_governance_bundle_json()
        .expect("bundle payload export should succeed");
    let bundle_trail = bundle_pipeline
        .governance_audit_trail()
        .expect("bundle trail read should succeed");
    let bundle_checksum = bundle_trail.current_checksum.clone().unwrap_or_else(|| {
        bundle_pipeline
            .with_read(|inner| inner.export_governance_state().state_checksum)
            .expect("bundle checksum fallback read should succeed")
    });
    bundle_pipeline
        .compare_and_import_governance_bundle_json_with_checksum(
            bundle_trail.current_version,
            &bundle_checksum,
            &bundle_payload,
        )
        .expect("bundle CAS import should accept checksum match");

    let state_pipeline = ConcurrentMoePipeline::from_builder(MoePipelineBuilder::new());
    let state_payload = state_pipeline
        .export_governance_state_json()
        .expect("state payload export should succeed");
    let state_trail = state_pipeline
        .governance_audit_trail()
        .expect("state trail read should succeed");
    let state_checksum = state_trail.current_checksum.clone().unwrap_or_else(|| {
        state_pipeline
            .with_read(|inner| inner.export_governance_state().state_checksum)
            .expect("state checksum fallback read should succeed")
    });
    state_pipeline
        .compare_and_import_governance_state_json_with_checksum(
            state_trail.current_version,
            &state_checksum,
            &state_payload,
        )
        .expect("state CAS import should accept checksum match");
}

#[test]
fn concurrent_pipeline_exposes_import_telemetry_for_parse_failures_rejections_and_successes() {
    let pipeline = ConcurrentMoePipeline::from_builder(MoePipelineBuilder::new());

    let _ = pipeline.import_runtime_bundle_json("{invalid json");

    let runtime_payload = pipeline
        .export_runtime_bundle_json()
        .expect("runtime export should succeed");

    let governance_state_payload = pipeline
        .export_governance_state_json()
        .expect("governance state export should succeed");
    let mut upgraded_state: GovernanceState =
        common_json::json::from_json_str(&governance_state_payload)
            .expect("governance state payload should deserialize");
    upgraded_state.state_version += 1;
    upgraded_state.state_checksum.clear();
    let upgraded_state_payload = common_json::json::to_json_string_pretty(&upgraded_state)
        .expect("upgraded governance state payload should serialize");
    pipeline
        .import_governance_state_json(&upgraded_state_payload)
        .expect("governance state import should advance version");

    pipeline
        .import_runtime_bundle_json(&runtime_payload)
        .expect_err("stale runtime payload should be rejected");

    let refreshed_runtime_payload = pipeline
        .export_runtime_bundle_json()
        .expect("runtime export after governance upgrade should succeed");
    pipeline
        .import_runtime_bundle_json(&refreshed_runtime_payload)
        .expect("runtime import should succeed");

    let telemetry = pipeline
        .import_telemetry_snapshot()
        .expect("import telemetry should be readable");
    assert!(telemetry.json_parse_failures >= 1);
    assert!(telemetry.runtime_bundle_import_rejections >= 1);
    assert!(telemetry.runtime_bundle_import_successes >= 1);

    let metrics = pipeline.metrics();
    assert!(
        metrics
            .get("json_parse_failures")
            .copied()
            .unwrap_or_default()
            >= 1
    );
    assert!(
        metrics
            .get("runtime_bundle_import_rejections")
            .copied()
            .unwrap_or_default()
            >= 1
    );
    assert!(
        metrics
            .get("runtime_bundle_import_successes")
            .copied()
            .unwrap_or_default()
            >= 1
    );
}

#[test]
fn concurrent_pipeline_exports_operational_report_with_lock_and_import_telemetry() {
    let pipeline = ConcurrentMoePipeline::from_builder(MoePipelineBuilder::new());
    pipeline
        .remember_short_term(MemoryEntry {
            id: "ops.concurrent.short".to_string(),
            content: "short".to_string(),
            tags: vec!["ops".to_string()],
            created_at: 1,
            expires_at: None,
            memory_type: MemoryType::Short,
            relevance: 0.7,
            metadata: HashMap::new(),
        })
        .expect("short memory write should succeed");
    pipeline
        .remember_long_term(MemoryEntry {
            id: "ops.concurrent.long".to_string(),
            content: "long".to_string(),
            tags: vec!["ops".to_string()],
            created_at: 2,
            expires_at: None,
            memory_type: MemoryType::Long,
            relevance: 0.8,
            metadata: HashMap::new(),
        })
        .expect("long memory write should succeed");
    let runtime_payload = pipeline
        .export_runtime_bundle_json()
        .expect("runtime export should succeed");
    pipeline
        .import_runtime_bundle_json(&runtime_payload)
        .expect("runtime import should succeed");
    pipeline
        .with_read_timeout(2, |_| ())
        .expect("read timeout api should succeed");
    pipeline
        .with_write_timeout(2, |_| Ok(()))
        .expect("write timeout api should succeed");

    let report = pipeline
        .export_operational_report()
        .expect("concurrent operational report should export");
    assert_eq!(report.pipeline.short_term_memory_entries, 1);
    assert_eq!(report.pipeline.long_term_memory_entries, 1);
    assert!(
        report
            .pipeline
            .import_telemetry
            .runtime_bundle_import_successes
            >= 1
    );
    assert!(report.lock_metrics.total_lock_acquisitions() > 0);
    assert!(report.lock_contention_rate >= 0.0);
    assert!(report.lock_timeout_rate >= 0.0);

    let report_json = pipeline
        .export_operational_report_json()
        .expect("concurrent operational report json should serialize");
    let parsed: ConcurrentOperationalReport =
        common_json::json::from_json_str(&report_json).expect("report json should parse");
    assert_eq!(
        parsed.pipeline.runtime_bundle_checksum,
        report.pipeline.runtime_bundle_checksum
    );
    assert!(parsed.lock_metrics.total_lock_acquisitions() > 0);
    assert!(parsed.lock_contention_rate >= 0.0);
    assert!(parsed.lock_timeout_rate >= 0.0);
}
