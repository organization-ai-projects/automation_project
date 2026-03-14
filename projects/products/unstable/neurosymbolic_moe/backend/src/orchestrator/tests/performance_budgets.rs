use crate::dataset_engine::{DatasetEntry, Outcome};
use crate::memory_engine::{MemoryEntry, MemoryType};
use crate::moe_core::{ExpertId, TaskId};
use crate::orchestrator::{ConcurrentMoePipeline, MoePipelineBuilder, TrainerTriggerEvent};
use common_time::current_timestamp_ms;
use std::collections::HashMap;
use std::thread;

fn perf_gate_enabled() -> bool {
    std::env::var("MOE_PERF_BUDGETS").is_ok_and(|value| value == "1")
}

fn env_usize(name: &str, default: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|raw| raw.parse::<usize>().ok())
        .unwrap_or(default)
}

fn env_u128(name: &str, default: u128) -> u128 {
    std::env::var(name)
        .ok()
        .and_then(|raw| raw.parse::<u128>().ok())
        .unwrap_or(default)
}

#[test]
fn perf_budget_runtime_bundle_roundtrip_import() {
    if !perf_gate_enabled() {
        return;
    }

    let iterations = env_usize("MOE_PERF_RUNTIME_ROUNDTRIP_ITERS", 50);
    let budget_ms = env_u128("MOE_PERF_RUNTIME_ROUNDTRIP_BUDGET_MS", 2_000);

    let mut source = MoePipelineBuilder::new().build();
    for idx in 0..256_u64 {
        source
            .remember_short_term(MemoryEntry {
                id: format!("perf-runtime-short-{idx}"),
                content: "perf".to_string(),
                tags: vec!["perf".to_string()],
                created_at: idx,
                expires_at: None,
                memory_type: MemoryType::Short,
                relevance: 0.8,
                metadata: HashMap::new(),
            })
            .expect("short-term insert should succeed");
    }
    let bundle = source.export_runtime_bundle();
    let mut target = MoePipelineBuilder::new().build();

    let started_ms = current_timestamp_ms();
    for _ in 0..iterations {
        target
            .import_runtime_bundle(bundle.clone())
            .expect("runtime bundle import should succeed");
    }
    let elapsed_ms = current_timestamp_ms().saturating_sub(started_ms) as u128;

    assert!(
        elapsed_ms <= budget_ms,
        "runtime roundtrip perf budget exceeded: {elapsed_ms}ms > {budget_ms}ms (iters={iterations})"
    );
}

#[test]
fn perf_budget_trainer_trigger_lease_ack_cycle() {
    if !perf_gate_enabled() {
        return;
    }

    let events = env_usize("MOE_PERF_TRIGGER_EVENTS", 5_000);
    let budget_ms = env_u128("MOE_PERF_TRIGGER_BUDGET_MS", 1_500);

    let mut pipeline = MoePipelineBuilder::new()
        .with_max_trainer_trigger_events(events)
        .build();
    for idx in 0..events {
        pipeline.trainer_trigger_queue.push(TrainerTriggerEvent {
            event_id: idx as u64,
            model_version: 1,
            training_bundle_checksum: format!("perf-bundle-{idx}"),
            included_entries: 10,
            train_samples: 8,
            validation_samples: 2,
            generated_at: idx as u64,
            delivery_attempts: 0,
            last_attempted_at: None,
        });
    }

    let started_ms = current_timestamp_ms();
    for tick in 0..events {
        let leased = pipeline
            .lease_next_trainer_trigger_event(tick as u64, 0)
            .expect("event should be leaseable");
        assert!(
            pipeline.acknowledge_trainer_trigger_event(leased.event_id),
            "leased event should be acknowledged"
        );
    }
    let elapsed_ms = current_timestamp_ms().saturating_sub(started_ms) as u128;

    assert_eq!(pipeline.trainer_trigger_events_pending(), 0);
    assert!(
        elapsed_ms <= budget_ms,
        "trainer trigger perf budget exceeded: {elapsed_ms}ms > {budget_ms}ms (events={events})"
    );
}

#[test]
fn perf_budget_concurrent_runtime_soak() {
    if !perf_gate_enabled() {
        return;
    }

    let workers = env_usize("MOE_PERF_SOAK_WORKERS", 8);
    let ops_per_worker = env_usize("MOE_PERF_SOAK_OPS_PER_WORKER", 300);
    let budget_ms = env_u128("MOE_PERF_SOAK_BUDGET_MS", 4_000);

    let pipeline = ConcurrentMoePipeline::from_builder(MoePipelineBuilder::new());
    let started_ms = current_timestamp_ms();

    let mut handles = Vec::new();
    for worker in 0..workers {
        let pipeline = pipeline.clone();
        handles.push(thread::spawn(move || {
            for op in 0..ops_per_worker {
                if op % 2 == 0 {
                    let _ = pipeline.remember_short_term(MemoryEntry {
                        id: format!("perf-soak-{worker}-{op}"),
                        content: "soak".to_string(),
                        tags: vec!["perf".to_string()],
                        created_at: ((worker * ops_per_worker) + op) as u64,
                        expires_at: None,
                        memory_type: MemoryType::Short,
                        relevance: 0.5,
                        metadata: HashMap::new(),
                    });
                } else {
                    let _ = pipeline.export_runtime_bundle_json();
                }
            }
        }));
    }
    for handle in handles {
        handle.join().expect("worker thread should not panic");
    }

    let elapsed_ms = current_timestamp_ms().saturating_sub(started_ms) as u128;
    let snapshot = pipeline.metrics_snapshot();
    assert!(
        snapshot.total_lock_acquisitions() > 0,
        "soak test should acquire locks"
    );
    assert!(
        elapsed_ms <= budget_ms,
        "concurrent soak perf budget exceeded: {elapsed_ms}ms > {budget_ms}ms (workers={workers}, ops={ops_per_worker})"
    );
}

#[test]
fn perf_budget_runtime_checksum_recompute() {
    if !perf_gate_enabled() {
        return;
    }

    let entries = env_usize("MOE_PERF_CHECKSUM_ENTRIES", 2_000);
    let iterations = env_usize("MOE_PERF_CHECKSUM_ITERS", 100);
    let budget_ms = env_u128("MOE_PERF_CHECKSUM_BUDGET_MS", 2_000);

    let mut pipeline = MoePipelineBuilder::new()
        .with_max_trainer_trigger_events(entries)
        .build();

    for idx in 0..entries {
        pipeline
            .remember_short_term(MemoryEntry {
                id: format!("perf-checksum-short-{idx}"),
                content: "checksum".to_string(),
                tags: vec!["perf".to_string()],
                created_at: idx as u64,
                expires_at: None,
                memory_type: MemoryType::Short,
                relevance: 0.7,
                metadata: HashMap::new(),
            })
            .expect("short-term insert should succeed");

        pipeline
            .remember_long_term(MemoryEntry {
                id: format!("perf-checksum-long-{idx}"),
                content: "checksum".to_string(),
                tags: vec!["perf".to_string()],
                created_at: idx as u64,
                expires_at: None,
                memory_type: MemoryType::Long,
                relevance: 0.6,
                metadata: HashMap::new(),
            })
            .expect("long-term insert should succeed");

        pipeline
            .training_runtime_state
            .dataset_store
            .upsert_entry(DatasetEntry {
                id: format!("perf-checksum-dataset-{idx}"),
                task_id: TaskId::new(format!("t-{idx}")),
                expert_id: ExpertId::new("perf-expert"),
                input: "in".to_string(),
                output: "out".to_string(),
                outcome: Outcome::Success,
                score: Some(0.9),
                tags: vec!["perf".to_string()],
                created_at: idx as u64,
                metadata: HashMap::new(),
            });

        pipeline.trainer_trigger_queue.push(TrainerTriggerEvent {
            event_id: idx as u64,
            model_version: 1,
            training_bundle_checksum: format!("perf-checksum-bundle-{idx}"),
            included_entries: 10,
            train_samples: 8,
            validation_samples: 2,
            generated_at: idx as u64,
            delivery_attempts: 0,
            last_attempted_at: None,
        });
    }

    let started_ms = current_timestamp_ms();
    for _ in 0..iterations {
        let checksum = pipeline.runtime_bundle_checksum();
        assert!(!checksum.is_empty(), "checksum must not be empty");
    }
    let elapsed_ms = current_timestamp_ms().saturating_sub(started_ms) as u128;

    assert!(
        elapsed_ms <= budget_ms,
        "runtime checksum perf budget exceeded: {elapsed_ms}ms > {budget_ms}ms (entries={entries}, iterations={iterations})"
    );
}
