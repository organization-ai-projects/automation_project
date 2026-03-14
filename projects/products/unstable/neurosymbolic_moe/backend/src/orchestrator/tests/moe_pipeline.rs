use crate::aggregator::AggregationStrategy;
use crate::dataset_engine::DatasetTrainingBuildOptions;
use crate::memory_engine::{MemoryEntry, MemoryType};
use crate::moe_core::{
    ExecutionContext, Expert, ExpertCapability, ExpertError, ExpertId, ExpertMetadata,
    ExpertOutput, ExpertStatus, ExpertType, MoeError, Task, TaskType,
};
use crate::orchestrator::{
    GovernanceStateSnapshot, MoePipelineBuilder, OperationalReport, TrainerTriggerEvent,
};
use crate::retrieval_engine::{RetrievalQuery, RetrievalResult, Retriever};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct TestExpert {
    meta: ExpertMetadata,
}

impl TestExpert {
    fn new(id: &str, capabilities: Vec<ExpertCapability>) -> Self {
        Self {
            meta: ExpertMetadata {
                id: ExpertId::new(id),
                name: id.to_string(),
                version: "1.0.0".to_string(),
                capabilities,
                status: ExpertStatus::Active,
                expert_type: ExpertType::Deterministic,
            },
        }
    }
}

struct StubRetriever;

impl Retriever for StubRetriever {
    fn retrieve(&self, _query: &RetrievalQuery) -> Result<Vec<RetrievalResult>, MoeError> {
        Ok(vec![RetrievalResult::new(
            "ctx-1",
            "retrieved rust context",
            0.9,
            "doc://ctx",
        )])
    }
}

struct RecordingRetriever {
    last_query: Arc<Mutex<Option<RetrievalQuery>>>,
}

impl Retriever for RecordingRetriever {
    fn retrieve(&self, query: &RetrievalQuery) -> Result<Vec<RetrievalResult>, MoeError> {
        *self
            .last_query
            .lock()
            .expect("query mutex should not be poisoned") = Some(query.clone());
        Ok(vec![RetrievalResult::new(
            "ctx-1",
            "metadata-aware context",
            0.95,
            "doc://recorded",
        )])
    }
}

impl Expert for TestExpert {
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
        context: &ExecutionContext,
    ) -> Result<ExpertOutput, ExpertError> {
        let trace_hint = context.retrieved_context.len() + context.memory_entries.len();
        Ok(ExpertOutput {
            expert_id: self.meta.id.clone(),
            content: format!("{}:{trace_hint}", task.input()),
            confidence: 0.95,
            metadata: HashMap::new(),
            trace: Vec::new(),
        })
    }
}

struct ContextStatsExpert {
    meta: ExpertMetadata,
}

impl ContextStatsExpert {
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

impl Expert for ContextStatsExpert {
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
        _task: &Task,
        context: &ExecutionContext,
    ) -> Result<ExpertOutput, ExpertError> {
        Ok(ExpertOutput {
            expert_id: self.meta.id.clone(),
            content: format!(
                "r{}-m{}-b{}",
                context.retrieved_context.len(),
                context.memory_entries.len(),
                context.buffer_data.len()
            ),
            confidence: 1.0,
            metadata: HashMap::new(),
            trace: Vec::new(),
        })
    }
}

#[test]
fn builder_builds_pipeline() {
    let pipeline = MoePipelineBuilder::new()
        .with_aggregation_strategy(AggregationStrategy::First)
        .with_max_traces(500)
        .build();
    assert_eq!(pipeline.registry().count(), 0);
}

#[test]
fn register_expert() {
    let mut pipeline = MoePipelineBuilder::new().build();
    let expert = TestExpert::new("e1", vec![ExpertCapability::CodeGeneration]);
    pipeline
        .register_expert(Box::new(expert))
        .expect("expert registration should succeed");
    assert_eq!(pipeline.registry().count(), 1);
}

#[test]
fn full_execute_pipeline() {
    let mut pipeline = MoePipelineBuilder::new().build();
    let expert = TestExpert::new("codegen", vec![ExpertCapability::CodeGeneration]);
    pipeline
        .register_expert(Box::new(expert))
        .expect("expert registration should succeed");

    let task = Task::new("t1", TaskType::CodeGeneration, "write code");
    let result = pipeline
        .execute(task)
        .expect("pipeline execution should succeed");
    assert!(result.selected_output.is_some());
    let selected = result
        .selected_output
        .expect("selected output should be present");
    assert_eq!(selected.expert_id.as_str(), "codegen");
    assert!(pipeline.trace_logger().count() > 0);
}

#[test]
fn execute_enriches_context_with_retrieval_memory_and_buffer() {
    let mut pipeline = MoePipelineBuilder::new()
        .with_retriever(Box::new(StubRetriever))
        .with_context_max_length(256)
        .build();
    let expert = TestExpert::new("codegen", vec![ExpertCapability::CodeGeneration]);
    pipeline
        .register_expert(Box::new(expert))
        .expect("expert registration should succeed");

    pipeline
        .remember_short_term(MemoryEntry {
            id: "mem-1".to_string(),
            content: "recent memory".to_string(),
            tags: vec!["runtime".to_string()],
            created_at: 1,
            expires_at: None,
            memory_type: MemoryType::Short,
            relevance: 0.8,
            metadata: HashMap::new(),
        })
        .expect("short-term memory write should succeed");

    let task = Task::new("t-enriched", TaskType::CodeGeneration, "write rust code");
    let result = pipeline
        .execute(task)
        .expect("pipeline execution should succeed");
    let selected = result
        .selected_output
        .expect("selected output should be present");

    // TestExpert emits "<input>:<retrieved+memory>"; task-aware assembly adds a header segment.
    assert!(selected.content.ends_with(":3"));

    let retrieval_traces = pipeline
        .trace_logger()
        .get_by_phase(&crate::moe_core::TracePhase::Retrieval);
    let memory_traces = pipeline
        .trace_logger()
        .get_by_phase(&crate::moe_core::TracePhase::MemoryQuery);
    assert!(!retrieval_traces.is_empty());
    assert!(!memory_traces.is_empty());
}

#[test]
fn execute_respects_metadata_for_retrieval_memory_and_session_buffer() {
    let recorded = Arc::new(Mutex::new(None));
    let retriever = RecordingRetriever {
        last_query: Arc::clone(&recorded),
    };
    let mut pipeline = MoePipelineBuilder::new()
        .with_retriever(Box::new(retriever))
        .with_context_max_length(256)
        .build();
    pipeline
        .register_expert(Box::new(ContextStatsExpert::new("stats")))
        .expect("expert registration should succeed");

    pipeline
        .remember_short_term(MemoryEntry {
            id: "mem-critical".to_string(),
            content: "critical memory".to_string(),
            tags: vec!["critical".to_string()],
            created_at: 1,
            expires_at: None,
            memory_type: MemoryType::Short,
            relevance: 0.9,
            metadata: HashMap::new(),
        })
        .expect("short memory write should succeed");
    pipeline
        .remember_short_term(MemoryEntry {
            id: "mem-other".to_string(),
            content: "other memory".to_string(),
            tags: vec!["other".to_string()],
            created_at: 2,
            expires_at: None,
            memory_type: MemoryType::Short,
            relevance: 0.95,
            metadata: HashMap::new(),
        })
        .expect("short memory write should succeed");
    pipeline
        .remember_long_term(MemoryEntry {
            id: "mem-long".to_string(),
            content: "long memory".to_string(),
            tags: vec!["critical".to_string()],
            created_at: 3,
            expires_at: None,
            memory_type: MemoryType::Long,
            relevance: 0.8,
            metadata: HashMap::new(),
        })
        .expect("long memory write should succeed");
    pipeline.put_session_buffer("session-a", "note", "session-value");

    let task = Task::new("t-metadata", TaskType::CodeGeneration, "generate")
        .with_metadata("retrieval.max_results", "3")
        .with_metadata("retrieval.min_relevance", "0.4")
        .with_metadata("retrieval.filter.domain", "systems")
        .with_metadata("memory.tags", "critical")
        .with_metadata("memory.min_relevance", "0.85")
        .with_metadata("memory.max_results", "4")
        .with_metadata("session_id", "session-a");

    let result = pipeline.execute(task).expect("execution should succeed");
    let selected = result
        .selected_output
        .expect("selected output should be present");
    assert_eq!(selected.content, "r2-m1-b2");

    let query = recorded
        .lock()
        .expect("query mutex should not be poisoned")
        .clone()
        .expect("retrieval query should be recorded");
    assert_eq!(query.max_results, 3);
    assert!((query.min_relevance - 0.4).abs() < f64::EPSILON);
    assert_eq!(
        query.filters.get("domain").map(String::as_str),
        Some("systems")
    );
}

#[test]
fn import_telemetry_tracks_success_rejection_and_parse_failures() {
    let mut source = MoePipelineBuilder::new().build();
    source
        .remember_short_term(MemoryEntry {
            id: "telemetry.short.seed".to_string(),
            content: "telemetry seed".to_string(),
            tags: vec!["telemetry".to_string()],
            created_at: 1,
            expires_at: None,
            memory_type: MemoryType::Short,
            relevance: 0.8,
            metadata: HashMap::new(),
        })
        .expect("short memory write should succeed");
    let runtime_bundle = source.export_runtime_bundle();
    let runtime_payload = source
        .export_runtime_bundle_json()
        .expect("runtime payload export should succeed");
    let governance_state_payload = source
        .export_governance_state_json()
        .expect("governance state payload export should succeed");

    let mut target = MoePipelineBuilder::new().build();
    let malformed_runtime_payload = format!("{runtime_payload} trailing-garbage");
    target
        .import_runtime_bundle_json(&malformed_runtime_payload)
        .expect_err("malformed runtime payload should be rejected");
    let after_parse_failure = target.import_telemetry_snapshot();
    assert_eq!(after_parse_failure.json_parse_failures, 1);

    target
        .import_runtime_bundle(runtime_bundle)
        .expect("runtime bundle import should succeed");
    let after_runtime_success = target.import_telemetry_snapshot();
    assert_eq!(after_runtime_success.runtime_bundle_import_successes, 1);
    target
        .import_runtime_bundle_json(&runtime_payload)
        .expect("first runtime json import should succeed");
    target
        .import_runtime_bundle_json(&runtime_payload)
        .expect("second runtime json import should deduplicate");
    assert!(target.import_journal_deduplicated_replays_total() >= 1);
    assert!(target.import_journal_events_total() >= 3);

    let unsupported_state_payload =
        governance_state_payload.replace("\"schema_version\": 1", "\"schema_version\": 999");
    target
        .import_governance_state_json(&unsupported_state_payload)
        .expect_err("unsupported schema should be rejected in strict mode");
    let after_state_rejection = target.import_telemetry_snapshot();
    assert!(after_state_rejection.governance_state_import_rejections >= 1);

    target
        .import_governance_state_json(&governance_state_payload)
        .expect("valid governance state should import");
    let final_snapshot = target.import_telemetry_snapshot();
    assert!(final_snapshot.governance_state_import_successes >= 1);
}

#[test]
fn export_operational_report_includes_runtime_governance_and_import_telemetry() {
    let mut pipeline = MoePipelineBuilder::new().build();
    pipeline
        .register_expert(Box::new(TestExpert::new(
            "ops-report-expert",
            vec![ExpertCapability::CodeGeneration],
        )))
        .expect("expert registration should succeed");
    pipeline
        .remember_short_term(MemoryEntry {
            id: "ops-short".to_string(),
            content: "short".to_string(),
            tags: vec!["ops".to_string()],
            created_at: 1,
            expires_at: None,
            memory_type: MemoryType::Short,
            relevance: 0.7,
            metadata: HashMap::new(),
        })
        .expect("short-term memory write should succeed");
    pipeline
        .remember_long_term(MemoryEntry {
            id: "ops-long".to_string(),
            content: "long".to_string(),
            tags: vec!["ops".to_string()],
            created_at: 2,
            expires_at: None,
            memory_type: MemoryType::Long,
            relevance: 0.8,
            metadata: HashMap::new(),
        })
        .expect("long-term memory write should succeed");
    pipeline.put_session_buffer("ops-session", "k1", "v1");
    pipeline.put_session_buffer("ops-session", "k2", "v2");
    let _ = pipeline
        .execute(Task::new(
            "ops-report-task",
            TaskType::CodeGeneration,
            "operational report",
        ))
        .expect("execution should succeed");
    let runtime_payload = pipeline
        .export_runtime_bundle_json()
        .expect("runtime payload export should succeed");
    pipeline
        .import_runtime_bundle_json(&runtime_payload)
        .expect("runtime import should succeed");

    let report = pipeline.export_operational_report();
    assert_eq!(
        report.governance_current_version,
        pipeline.export_governance_state().state_version
    );
    assert_eq!(report.short_term_memory_entries, 1);
    assert_eq!(report.long_term_memory_entries, 1);
    assert_eq!(report.session_buffer_sessions, 1);
    assert_eq!(report.session_buffer_values, 2);
    assert!(report.trace_entries > 0);
    assert!(report.runtime_bundle_checksum.len() >= 8);
    assert!(report.import_telemetry.runtime_bundle_import_successes >= 1);

    let report_json = pipeline
        .export_operational_report_json()
        .expect("operational report json should serialize");
    let parsed: OperationalReport =
        common_json::json::from_json_str(&report_json).expect("operational report should parse");
    assert_eq!(parsed.session_buffer_values, report.session_buffer_values);
    assert_eq!(
        parsed.governance_current_version,
        report.governance_current_version
    );
    assert!(report.import_journal_events_total >= 1);
    assert!(!report.to_prometheus_text("moe_test").is_empty());
    assert_eq!(report.slo_status(1, 0, 0), "OK");
}

#[test]
fn export_training_dataset_bundle_json_from_pipeline() {
    let mut pipeline = MoePipelineBuilder::new().build();
    let expert = TestExpert::new("training-export", vec![ExpertCapability::CodeGeneration]);
    pipeline
        .register_expert(Box::new(expert))
        .expect("expert registration should succeed");

    let task = Task::new(
        "t-training-dataset",
        TaskType::CodeGeneration,
        "build dataset candidate",
    );
    let _ = pipeline
        .execute(task)
        .expect("pipeline execution should succeed");

    let options = DatasetTrainingBuildOptions {
        generated_at: 123,
        validation_ratio: 0.2,
        min_score: None,
        include_failure_entries: true,
        include_partial_entries: true,
        include_unknown_entries: false,
        require_correction_for_failure: false,
        split_seed: 5,
    };
    let bundle = pipeline
        .export_training_dataset_bundle(&options)
        .expect("training dataset bundle should export");
    let json = pipeline
        .export_training_dataset_bundle_json(&options)
        .expect("training dataset bundle json should export");

    assert!(bundle.included_entries > 0);
    assert_eq!(
        bundle.provenance.generator,
        "neurosymbolic_moe_backend.orchestrator"
    );
    assert!(bundle.verify_checksum());
    assert!(!json.is_empty());
}

#[test]
fn export_training_dataset_shards_from_pipeline() {
    let mut pipeline = MoePipelineBuilder::new().build();
    let expert = TestExpert::new(
        "training-shard-export",
        vec![ExpertCapability::CodeGeneration],
    );
    pipeline
        .register_expert(Box::new(expert))
        .expect("expert registration should succeed");

    for idx in 0..6_u32 {
        let task = Task::new(
            format!("t-training-shard-{idx}"),
            TaskType::CodeGeneration,
            format!("build shard dataset candidate {idx}"),
        );
        let _ = pipeline
            .execute(task)
            .expect("pipeline execution should succeed");
    }

    let options = DatasetTrainingBuildOptions {
        generated_at: 123,
        validation_ratio: 0.2,
        min_score: None,
        include_failure_entries: true,
        include_partial_entries: true,
        include_unknown_entries: false,
        require_correction_for_failure: false,
        split_seed: 5,
    };
    let shards = pipeline
        .export_training_dataset_shards(&options, 2)
        .expect("training dataset shards should export");
    let shards_json = pipeline
        .export_training_dataset_shards_json(&options, 2)
        .expect("training dataset shards json should export");
    let rebuilt = pipeline
        .rebuild_training_dataset_bundle_from_shards(&shards)
        .expect("training dataset bundle should rebuild from shards");
    let rebuilt_from_json = pipeline
        .rebuild_training_dataset_bundle_from_shards_json(&shards_json)
        .expect("training dataset bundle should rebuild from shards json");

    assert!(!shards.is_empty());
    assert!(
        shards
            .iter()
            .all(|shard| shard.total_shards == shards.len())
    );
    assert!(
        shards
            .iter()
            .all(|shard| shard.provenance.generator == "neurosymbolic_moe_backend.orchestrator")
    );
    assert_eq!(rebuilt.included_entries, rebuilt_from_json.included_entries);
    assert_eq!(
        rebuilt.provenance.generator,
        "neurosymbolic_moe_backend.orchestrator"
    );
    assert!(rebuilt.verify_checksum());
    assert!(!shards_json.is_empty());
}

#[test]
fn preview_training_dataset_bundle_json_validates_payload() {
    let mut pipeline = MoePipelineBuilder::new().build();
    pipeline
        .register_expert(Box::new(TestExpert::new(
            "training-preview",
            vec![ExpertCapability::CodeGeneration],
        )))
        .expect("expert registration should succeed");
    let _ = pipeline
        .execute(Task::new(
            "t-training-preview",
            TaskType::CodeGeneration,
            "build preview dataset candidate",
        ))
        .expect("pipeline execution should succeed");

    let options = DatasetTrainingBuildOptions {
        generated_at: 456,
        validation_ratio: 0.3,
        min_score: None,
        include_failure_entries: true,
        include_partial_entries: true,
        include_unknown_entries: false,
        require_correction_for_failure: false,
        split_seed: 9,
    };
    let payload = pipeline
        .export_training_dataset_bundle_json(&options)
        .expect("training dataset bundle json should export");
    let preview = pipeline
        .preview_training_dataset_bundle_json(&payload)
        .expect("training dataset bundle preview should succeed");
    assert!(preview.verify_checksum());
}

#[test]
fn preview_training_dataset_shards_json_validates_payload() {
    let mut pipeline = MoePipelineBuilder::new().build();
    pipeline
        .register_expert(Box::new(TestExpert::new(
            "training-shards-preview",
            vec![ExpertCapability::CodeGeneration],
        )))
        .expect("expert registration should succeed");
    for idx in 0..4_u32 {
        let _ = pipeline
            .execute(Task::new(
                format!("t-training-shards-preview-{idx}"),
                TaskType::CodeGeneration,
                "build shard preview dataset candidate",
            ))
            .expect("pipeline execution should succeed");
    }

    let options = DatasetTrainingBuildOptions {
        generated_at: 789,
        validation_ratio: 0.25,
        min_score: None,
        include_failure_entries: true,
        include_partial_entries: true,
        include_unknown_entries: false,
        require_correction_for_failure: false,
        split_seed: 12,
    };
    let payload = pipeline
        .export_training_dataset_shards_json(&options, 2)
        .expect("training dataset shards json should export");
    let preview = pipeline
        .preview_training_dataset_shards_json(&payload)
        .expect("training dataset shards preview should succeed");
    assert!(preview.verify_checksum());
}

#[test]
fn preview_training_dataset_bundle_json_rejects_oversized_payload() {
    let pipeline = MoePipelineBuilder::new().build();
    let oversized_payload = "x".repeat((64 * 1024 * 1024) + 1);
    let err = pipeline
        .preview_training_dataset_bundle_json(&oversized_payload)
        .expect_err("oversized training bundle payload should be rejected");
    assert!(err.to_string().contains("payload too large"));
}

#[test]
fn preview_training_dataset_shards_json_rejects_oversized_payload() {
    let pipeline = MoePipelineBuilder::new().build();
    let oversized_payload = "x".repeat((128 * 1024 * 1024) + 1);
    let err = pipeline
        .preview_training_dataset_shards_json(&oversized_payload)
        .expect_err("oversized training shard payload should be rejected");
    assert!(err.to_string().contains("payload too large"));
}

#[test]
fn runtime_bundle_checksum_matches_exported_bundle_checksum() {
    let mut pipeline = MoePipelineBuilder::new().build();
    pipeline
        .register_expert(Box::new(TestExpert::new(
            "checksum-expert",
            vec![ExpertCapability::CodeGeneration],
        )))
        .expect("expert registration should succeed");

    pipeline
        .remember_short_term(MemoryEntry {
            id: "checksum-short".to_string(),
            content: "short".to_string(),
            tags: vec!["checksum".to_string()],
            created_at: 1,
            expires_at: None,
            memory_type: MemoryType::Short,
            relevance: 0.9,
            metadata: HashMap::new(),
        })
        .expect("short-term memory should store");
    pipeline
        .remember_long_term(MemoryEntry {
            id: "checksum-long".to_string(),
            content: "long".to_string(),
            tags: vec!["checksum".to_string()],
            created_at: 2,
            expires_at: None,
            memory_type: MemoryType::Long,
            relevance: 0.8,
            metadata: HashMap::new(),
        })
        .expect("long-term memory should store");

    let task = Task::new("checksum-task", TaskType::CodeGeneration, "checksum");
    let _ = pipeline.execute(task).expect("execution should succeed");

    let fast = pipeline.runtime_bundle_checksum();
    let exported = pipeline.export_runtime_bundle().runtime_checksum;
    assert_eq!(fast, exported);
}

#[test]
fn runtime_invariants_reject_duplicate_trainer_trigger_event_ids() {
    let mut pipeline = MoePipelineBuilder::new().build();
    pipeline.trainer_trigger_queue.push(TrainerTriggerEvent {
        event_id: 7,
        model_version: 1,
        training_bundle_checksum: "bundle-a".to_string(),
        included_entries: 10,
        train_samples: 8,
        validation_samples: 2,
        generated_at: 1,
        delivery_attempts: 0,
        last_attempted_at: None,
    });
    pipeline.trainer_trigger_queue.push(TrainerTriggerEvent {
        event_id: 7,
        model_version: 2,
        training_bundle_checksum: "bundle-b".to_string(),
        included_entries: 11,
        train_samples: 9,
        validation_samples: 2,
        generated_at: 2,
        delivery_attempts: 0,
        last_attempted_at: None,
    });

    let err = pipeline
        .validate_runtime_invariants()
        .expect_err("duplicate trigger ids should violate invariants");
    assert!(err.to_string().contains("duplicate event_id"));
}

#[test]
fn trainer_trigger_ack_requires_active_lease() {
    let mut pipeline = MoePipelineBuilder::new().build();
    pipeline.trainer_trigger_queue.push(TrainerTriggerEvent {
        event_id: 42,
        model_version: 1,
        training_bundle_checksum: "bundle-42".to_string(),
        included_entries: 10,
        train_samples: 8,
        validation_samples: 2,
        generated_at: 1,
        delivery_attempts: 0,
        last_attempted_at: None,
    });

    assert!(!pipeline.acknowledge_trainer_trigger_event(42));
    let leased = pipeline
        .lease_next_trainer_trigger_event(10, 0)
        .expect("event should lease");
    assert_eq!(leased.event_id, 42);
    assert!(pipeline.acknowledge_trainer_trigger_event(42));
    assert_eq!(pipeline.trainer_trigger_events_pending(), 0);
}

#[test]
fn trainer_trigger_mark_failed_requires_active_lease() {
    let mut pipeline = MoePipelineBuilder::new().build();
    pipeline.trainer_trigger_queue.push(TrainerTriggerEvent {
        event_id: 43,
        model_version: 1,
        training_bundle_checksum: "bundle-43".to_string(),
        included_entries: 10,
        train_samples: 8,
        validation_samples: 2,
        generated_at: 1,
        delivery_attempts: 0,
        last_attempted_at: None,
    });

    assert!(!pipeline.mark_trainer_trigger_event_delivery_failed(43, 11));
    let leased = pipeline
        .lease_next_trainer_trigger_event(12, 0)
        .expect("event should lease");
    assert_eq!(leased.event_id, 43);
    assert!(pipeline.mark_trainer_trigger_event_delivery_failed(43, 13));
    let released = pipeline
        .lease_next_trainer_trigger_event(13, 0)
        .expect("failed event should be leaseable again");
    assert_eq!(released.event_id, 43);
}

#[test]
fn trainer_trigger_lease_recovers_after_retry_window_without_explicit_failure() {
    let mut pipeline = MoePipelineBuilder::new().build();
    pipeline.trainer_trigger_queue.push(TrainerTriggerEvent {
        event_id: 44,
        model_version: 1,
        training_bundle_checksum: "bundle-44".to_string(),
        included_entries: 10,
        train_samples: 8,
        validation_samples: 2,
        generated_at: 1,
        delivery_attempts: 0,
        last_attempted_at: None,
    });

    let first = pipeline
        .lease_next_trainer_trigger_event(100, 60)
        .expect("event should lease at first attempt");
    assert_eq!(first.event_id, 44);
    assert_eq!(first.delivery_attempts, 1);

    assert!(
        pipeline.lease_next_trainer_trigger_event(120, 60).is_none(),
        "event should remain in-flight before retry window elapses"
    );

    let retried = pipeline
        .lease_next_trainer_trigger_event(161, 60)
        .expect("event should be re-leaseable after retry window");
    assert_eq!(retried.event_id, 44);
    assert_eq!(retried.delivery_attempts, 2);
}

#[test]
fn runtime_invariants_reject_missing_active_model_registry_version() {
    let mut pipeline = MoePipelineBuilder::new().build();
    pipeline
        .training_runtime_state
        .model_registry
        .active_version = Some(999);

    let err = pipeline
        .validate_runtime_invariants()
        .expect_err("missing active model version should violate invariants");
    assert!(err.to_string().contains("active model version"));
}

#[test]
fn runtime_invariants_reject_governance_snapshot_version_mismatch() {
    let mut pipeline = MoePipelineBuilder::new().build();
    let state = pipeline.export_governance_state();
    pipeline
        .governance_runtime_state
        .governance_state_snapshots
        .push(GovernanceStateSnapshot {
            version: state.state_version.saturating_add(1),
            reason: "test mismatch".to_string(),
            state,
        });

    let err = pipeline
        .validate_runtime_invariants()
        .expect_err("snapshot/state version mismatch should violate invariants");
    assert!(err.to_string().contains("snapshot version"));
}
