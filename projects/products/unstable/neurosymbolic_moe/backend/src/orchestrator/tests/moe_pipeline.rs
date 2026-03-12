use crate::aggregator::AggregationStrategy;
use crate::dataset_engine::DatasetTrainingBuildOptions;
use crate::memory_engine::{MemoryEntry, MemoryType};
use crate::moe_core::{
    ExecutionContext, Expert, ExpertCapability, ExpertError, ExpertId, ExpertMetadata,
    ExpertOutput, ExpertStatus, ExpertType, MoeError, Task, TaskType,
};
use crate::orchestrator::MoePipelineBuilder;
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
    assert_eq!(rebuilt.included_entries, rebuilt_from_json.included_entries);
    assert!(rebuilt.verify_checksum());
    assert!(!shards_json.is_empty());
}
