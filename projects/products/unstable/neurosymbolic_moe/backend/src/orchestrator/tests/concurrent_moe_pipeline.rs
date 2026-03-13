//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/tests/concurrent_moe_pipeline.rs
use crate::memory_engine::{MemoryEntry, MemoryType};
use crate::moe_core::{
    ExecutionContext, Expert, ExpertCapability, ExpertError, ExpertId, ExpertMetadata,
    ExpertOutput, ExpertStatus, ExpertType, Task, TaskType,
};
use crate::orchestrator::{ConcurrentMoePipeline, MoePipelineBuilder};
use std::collections::HashMap;
use std::sync::mpsc;
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
