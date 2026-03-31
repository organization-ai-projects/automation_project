//! projects/products/unstable/neurosymbolic_moe/backend/src/apps/impl_check.rs

use std::{collections::HashMap, str::FromStr};

use protocol::ProtocolId;
use threadpool::ThreadPool;

use crate::{
    aggregator::AggregationStrategy,
    apps::{
        DynError, run_concurrent_pipeline_checks, run_runtime_persistence_checks,
        run_training_and_cas_checks,
    },
    buffer_manager::{BufferEntry, BufferManager, BufferType, SessionBuffer, WorkingBuffer},
    dataset_engine::{
        ConcurrentDatasetStore, Correction, DatasetEntry, DatasetStore,
        DatasetTrainingBuildOptions, Outcome, TraceConverter,
    },
    evaluations::{EvaluationEngine, ExpertMetrics, RoutingMetrics},
    expert_registries::{ExpertRegistry, VersionEntry, VersionTracker},
    feedback_engine::{FeedbackEntry, FeedbackStore, FeedbackType},
    memory_engine::{
        LongTermMemory, MemoryEntry, MemoryQuery, MemoryStore, MemoryType, ShortTermMemory,
    },
    moe_core::{
        self, ExecutionContext, ExpertCapability, ExpertId, ExpertOutput, ExpertStatus, Task,
        TaskId, TaskPriority, TaskType, TracePhase, TraceRecord,
    },
    orchestrator::{MoePipeline, MoePipelineBuilder},
    policies_guard::{Policy, PolicyGuard, PolicyResult, PolicyType},
    retrieval_engine::{
        Chunk, Chunker, ChunkingStrategy, ContextAssembler, RetrievalQuery, RetrievalResult,
        Retriever, SimpleRetriever,
    },
    router::{HeuristicRouter, Router, RoutingDecision, RoutingStrategy, RoutingTrace},
    specialized_expert::SpecializedExpert,
    trace_logging::TraceLogger,
};

pub(crate) fn cmd_impl_check() -> Result<(), DynError> {
    tracing::info!("Running implementation check...");

    let buffer_variants = [BufferType::Working, BufferType::Session];
    tracing::info!("Buffer variants wired: {}", buffer_variants.len());

    let task_id = moe_core::TaskId::new();
    let valid_protocol_id = ProtocolId::generate();
    let expert_id = ExpertId::new();

    let entry = BufferEntry::new("k", "v", 1)
        .with_task_id(task_id.clone())
        .with_session_id("s1");
    tracing::info!(
        "BufferEntry key={} created_at={}",
        entry.key,
        entry.created_at
    );

    let mut buffers = BufferManager::new(2);
    buffers
        .working_mut()
        .put("ctx", "value", Some(task_id.clone()));
    let working_get = buffers.working().get("ctx").map(|e| e.value.clone());
    let working_count = buffers.working().count();
    let working_keys_len = buffers.working().keys().len();
    let removed_working = buffers.working_mut().remove("ctx").is_some();
    tracing::info!(
        "Working buffer: count={} keys={} get={} removed={}",
        working_count,
        working_keys_len,
        working_get.unwrap_or_else(|| "none".to_string()),
        removed_working
    );

    buffers.sessions_mut().create_session(&valid_protocol_id);
    buffers
        .sessions_mut()
        .put(&valid_protocol_id, "profile", "alpha");
    let session_get = buffers
        .sessions()
        .get(&valid_protocol_id, "profile")
        .map(|e| e.value.clone())
        .unwrap_or_else(|| "none".to_string());
    let session_list_len = buffers.sessions().list_sessions().len();
    let session_count = buffers.sessions().session_count();
    let removed_session = buffers.sessions_mut().remove_session(&valid_protocol_id);
    tracing::info!(
        "Session buffer: get={} list={} count={} removed={}",
        session_get,
        session_list_len,
        session_count,
        removed_session
    );
    buffers.clear_all();

    let mut direct_working = WorkingBuffer::new(1);
    direct_working.put("direct", "value", None);
    let mut direct_sessions = SessionBuffer::new();
    let direct_session_id = ProtocolId::generate();
    direct_sessions.create_session(&direct_session_id);
    direct_sessions.put(&direct_session_id, "k", "v");
    tracing::info!(
        "Direct buffers: working={} sessions={}",
        direct_working.count(),
        direct_sessions.session_count()
    );

    let trace_converter = TraceConverter::new();
    let trace_record = TraceRecord {
        trace_id: "tr-1".to_string(),
        task_id: task_id.clone(),
        timestamp: 1,
        expert_id: Some(expert_id.clone()),
        phase: TracePhase::ExpertExecution,
        detail: "done".to_string(),
        metadata: HashMap::new(),
    };
    let dataset_entry = trace_converter.convert(
        std::slice::from_ref(&trace_record),
        "input",
        "output",
        Outcome::Success,
    );

    let mut dataset_store = DatasetStore::new();
    dataset_store.add_entry(dataset_entry.clone());
    let manual_entry = DatasetEntry {
        id: ProtocolId::generate(),
        task_id: task_id.clone(),
        expert_id: expert_id.clone(),
        input: "manual-input".to_string(),
        output: "manual-output".to_string(),
        outcome: Outcome::Partial,
        score: Some(0.5),
        tags: vec!["manual".to_string()],
        created_at: 2,
        metadata: HashMap::new(),
    };
    dataset_store.add_entry(manual_entry);
    dataset_store.add_correction(Correction {
        entry_id: dataset_entry.id,
        corrected_output: "output-v2".to_string(),
        reason: "improved".to_string(),
        corrected_at: 2,
    });
    let task_entries = dataset_store.get_by_task(&task_id).len();
    let expert_entries = dataset_store.get_by_expert(&expert_id).len();
    let success_entries = dataset_store.get_by_outcome(&Outcome::Success).len();
    let correction_entries = dataset_store
        .get_corrections(&dataset_entry.id)
        .map_or(0, std::vec::Vec::len);
    tracing::info!(
        "Dataset: count={} by_task={} by_expert={} success={} corrections={} ok={} ko={}",
        dataset_store.count(),
        task_entries,
        expert_entries,
        success_entries,
        correction_entries,
        dataset_store.successful_count(),
        dataset_store.failed_count()
    );

    let concurrent_store = ConcurrentDatasetStore::new(DatasetStore::new());
    let pool = ThreadPool::new(4);
    let (tx, rx) = std::sync::mpsc::channel();

    for worker in 0..4_u32 {
        let writer = concurrent_store.clone();
        let tx = tx.clone();
        pool.execute(move || {
            // Implementation-check path: exercise concurrent writes through the shared store.
            // With ProtocolId::default() in this product, these writes contend on the same
            // logical entry and validate concurrent upsert behavior rather than unique inserts.
            for idx in 0..32_u32 {
                let id = ProtocolId::default();
                writer.add_entry(DatasetEntry {
                    id,
                    task_id: TaskId::new(),
                    expert_id: ExpertId::new(),
                    input: format!("input-worker-{worker}-idx-{idx}"),
                    output: format!("output-worker-{worker}-idx-{idx}"),
                    outcome: Outcome::Success,
                    score: Some(0.8),
                    tags: vec!["concurrency".to_string()],
                    created_at: u64::from(worker * 32 + idx),
                    metadata: HashMap::from([("writer".to_string(), format!("worker-{worker}"))]),
                });
            }
            if let Err(e) = tx.send(()) {
                tracing::error!("Failed to send completion signal for worker {worker}: {e}");
            }
        });
    }

    // Wait for all tasks to complete
    for _ in 0..4 {
        if let Err(e) = rx.recv() {
            tracing::error!("Failed to receive completion signal: {e}");
        }
    }

    concurrent_store.add_correction(Correction {
        entry_id: ProtocolId::default(),
        corrected_output: "output-concurrent-corrected".to_string(),
        reason: "parallel-review".to_string(),
        corrected_at: 42,
    });
    let concurrent_report = concurrent_store.quality_report(0.5);
    let concurrent_bundle =
        concurrent_store.build_training_bundle(&DatasetTrainingBuildOptions {
            generated_at: 10,
            validation_ratio: 0.2,
            min_score: None,
            include_failure_entries: true,
            include_partial_entries: true,
            include_unknown_entries: false,
            require_correction_for_failure: false,
            split_seed: 99,
        })?;
    let concurrent_shards = concurrent_store.build_training_shards(
        &DatasetTrainingBuildOptions {
            generated_at: 11,
            validation_ratio: 0.2,
            min_score: None,
            include_failure_entries: true,
            include_partial_entries: true,
            include_unknown_entries: false,
            require_correction_for_failure: false,
            split_seed: 99,
        },
        16,
    )?;
    let concurrent_rebuilt =
        concurrent_store.rebuild_training_bundle_from_shards(&concurrent_shards)?;
    tracing::info!(
        "Concurrent dataset store: count={} quality_total={} bundle={} rebuilt={}",
        concurrent_store.count(),
        concurrent_report.total_entries,
        concurrent_bundle.included_entries,
        concurrent_rebuilt.included_entries
    );

    let mut evaluation = EvaluationEngine::new();
    evaluation.record_expert_execution(expert_id.clone(), true, 0.8, 10.0);
    evaluation.record_routing(1, false);
    let expert_rate = evaluation
        .get_expert_metrics(&expert_id)
        .map_or(0.0, |m| m.success_rate());
    let routing_accuracy = evaluation.get_routing_metrics().accuracy();
    let best = evaluation
        .best_performing_expert()
        .map(|m| m.expert_id.clone());
    let worst = evaluation
        .worst_performing_expert()
        .map(|m| m.expert_id.clone());
    tracing::info!(
        "Evaluation: expert_rate={expert_rate:.2} routing_accuracy={routing_accuracy:.2} best={:?} worst={:?}",
        best,
        worst
    );
    let mut manual_expert_metrics = ExpertMetrics::new(expert_id.clone());
    manual_expert_metrics.record_execution(true, 0.7, 8.0);
    let mut manual_routing_metrics = RoutingMetrics::new();
    manual_routing_metrics.record_routing(2, false);
    tracing::info!(
        "Manual metrics: expert_rate={:.2} routing_accuracy={:.2}",
        manual_expert_metrics.success_rate(),
        manual_routing_metrics.accuracy()
    );

    let mut versions = VersionTracker::new();
    versions.record_version(VersionEntry {
        expert_id: expert_id.clone(),
        version: "1.0.0".to_string(),
        registered_at: 1,
        status: ExpertStatus::Active,
    });
    let history_count = versions.get_history(&expert_id).map_or(0, |h| h.len());
    let latest = versions
        .latest_version(&expert_id)
        .map(|v| v.version.clone())
        .unwrap_or_else(|| "none".to_string());
    tracing::info!("Version tracker: history={history_count} latest={latest}");

    let mut feedback_store = FeedbackStore::new();
    feedback_store.add(FeedbackEntry {
        id: ProtocolId::generate(),
        task_id: task_id.clone(),
        expert_id: expert_id.clone(),
        feedback_type: FeedbackType::Positive,
        score: Some(0.9),
        comment: "good".to_string(),
        created_at: 2,
    });
    tracing::info!(
        "Feedback: count={} by_task={} by_expert={} by_type={} avg={:?}",
        feedback_store.count(),
        feedback_store.get_by_task(&task_id).len(),
        feedback_store.get_by_expert(&expert_id).len(),
        feedback_store.get_by_type(&FeedbackType::Positive).len(),
        feedback_store.average_score_for_expert(&expert_id)
    );

    let memory_short = MemoryEntry {
        id: "mem-short".to_string(),
        content: "recent".to_string(),
        tags: vec!["runtime".to_string()],
        created_at: 1,
        expires_at: Some(100),
        memory_type: MemoryType::Short,
        relevance: 0.9,
        metadata: HashMap::new(),
    };
    let memory_medium = MemoryEntry {
        id: "mem-medium".to_string(),
        content: "intermediate".to_string(),
        tags: vec!["runtime".to_string()],
        created_at: 2,
        expires_at: None,
        memory_type: MemoryType::Medium,
        relevance: 0.7,
        metadata: HashMap::new(),
    };
    let memory_long = MemoryEntry {
        id: "mem-long".to_string(),
        content: "archival".to_string(),
        tags: vec!["history".to_string()],
        created_at: 3,
        expires_at: None,
        memory_type: MemoryType::Long,
        relevance: 0.8,
        metadata: HashMap::new(),
    };
    let memory_query = MemoryQuery {
        tags: Some(vec!["runtime".to_string()]),
        memory_type: None,
        min_relevance: Some(0.5),
        max_results: 10,
        include_expired: true,
        current_time: Some(0),
    };

    let mut short_store = ShortTermMemory::new(4);
    short_store.store(memory_short)?;
    short_store.store(memory_medium)?;
    let short_found = short_store.retrieve(&memory_query)?.len();
    let short_expired = short_store.expire(101);
    let short_after = short_store.count();
    let removed_short = short_store.remove("mem-short").is_some();
    tracing::info!(
        "Short memory: found={} expired={} count={} removed={}",
        short_found,
        short_expired,
        short_after,
        removed_short
    );

    let mut long_store = LongTermMemory::new();
    long_store.store(memory_long)?;
    let long_found = long_store.retrieve(&memory_query)?.len();
    let long_count = long_store.count();
    let long_removed = long_store.remove("mem-long").is_some();
    tracing::info!(
        "Long memory: found={} count={} removed={}",
        long_found,
        long_count,
        long_removed
    );

    let mut chunk = Chunk::new(
        ProtocolId::generate(),
        "Rust systems programming",
        "doc://a",
        0,
        25,
    );
    chunk = chunk.with_metadata("domain", "systems");
    let chunker_fixed = Chunker::new(ChunkingStrategy::FixedSize(8));
    let chunker_paragraph = Chunker::new(ChunkingStrategy::Paragraph);
    let chunker_semantic = Chunker::new(ChunkingStrategy::Semantic);
    let fixed_chunks = chunker_fixed.chunk("abcdefgh12345678", "doc://fixed").len();
    let paragraph_chunks = chunker_paragraph
        .chunk("first paragraph.\n\nsecond paragraph.", "doc://para")
        .len();
    let semantic_chunks = chunker_semantic
        .chunk("Sentence one. Sentence two!", "doc://sem")
        .len();
    tracing::info!(
        "Chunking: fixed={} paragraph={} semantic={}",
        fixed_chunks,
        paragraph_chunks,
        semantic_chunks
    );

    let mut retriever = SimpleRetriever::new();
    retriever.add_document(chunk);
    retriever.add_document(
        Chunk::new(
            ProtocolId::generate(),
            "Rust async ecosystem",
            "doc://b",
            0,
            19,
        )
        .with_metadata("domain", "systems"),
    );

    let query = RetrievalQuery::new("rust")
        .with_task_id(task_id.clone())
        .with_expert_id(expert_id.clone())
        .with_max_results(5)
        .with_min_relevance(0.1)
        .with_filter("domain", "systems");
    let retriever_port: &dyn Retriever = &retriever;
    let retrieved = retriever_port.retrieve(&query)?;

    let synthetic =
        RetrievalResult::new(ProtocolId::generate(), "manual context", 0.4, "manual://ctx")
            .with_metadata("kind", "manual");
    let mut all_results = retrieved.clone();
    all_results.push(synthetic);

    let task_for_context = Task::new_with_id(
        ProtocolId::generate(),
        TaskType::Retrieval,
        "need retrieval context",
    )
    .with_context("contextual")
    .with_priority(TaskPriority::Critical)
    .with_metadata("intent", "demo");
    let assembler = ContextAssembler::new(120);
    let assembled = assembler.assemble(&all_results);
    let assembled_for_task = assembler.assemble_for_task(&all_results, &task_for_context);
    tracing::info!(
        "Retrieval: results={} assembled={} assembled_task={} task_type={:?} has_ctx={} priority={:?}",
        all_results.len(),
        assembled.len(),
        assembled_for_task.len(),
        task_for_context.task_type(),
        task_for_context.context().is_some(),
        task_for_context.priority()
    );

    let execution_context = ExecutionContext::new(task_id.clone())
        .with_retrieved_context(assembled_for_task.clone())
        .with_memory_entries(vec!["m1".to_string()])
        .with_buffer_data(vec!["b1".to_string()])
        .with_parameter("runtime", "demo");
    tracing::info!(
        "Execution context: retrieved={} memory={} buffer={} params={}",
        execution_context.retrieved_context.len(),
        execution_context.memory_entries.len(),
        execution_context.buffer_data.len(),
        execution_context.parameters.len()
    );

    let mut scores = HashMap::new();
    scores.insert(expert_id.clone(), 0.95);
    let decision = RoutingDecision {
        task_id: task_id.clone(),
        selected_experts: vec![expert_id.clone()],
        scores,
        strategy: RoutingStrategy::SingleExpert,
        explanation: "top score".to_string(),
    };
    let routing_trace = RoutingTrace::from_decision(&decision, 3);
    let extra_strategies = [RoutingStrategy::MultiExpert, RoutingStrategy::RoundRobin];
    tracing::info!(
        "Routing: selected={} evaluated={} strategy={:?} extra_strategies={}",
        routing_trace.selected.len(),
        routing_trace.candidates_evaluated,
        routing_trace.strategy,
        extra_strategies.len()
    );

    let mut logger = TraceLogger::new(16);
    logger.log(trace_record);
    logger.log_phase(
        task_id.clone(),
        TracePhase::Validation,
        "policy pass".to_string(),
        Some(expert_id.clone()),
    );
    logger.log_phase(
        task_id.clone(),
        TracePhase::Aggregation,
        "aggregation done".to_string(),
        Some(expert_id.clone()),
    );
    tracing::info!(
        "Trace logger stats: total={} task={} phase={} expert={} recent={}",
        logger.count(),
        logger.get_by_task(&task_id).len(),
        logger.get_by_phase(&TracePhase::Validation).len(),
        logger.get_by_expert(&expert_id).len(),
        logger.recent(2).len()
    );

    let mut guard = PolicyGuard::new();
    guard.add_policy(Policy {
        id: ProtocolId::from_str("safety").unwrap_or_else(|_| ProtocolId::generate()),
        name: "Safety".to_string(),
        description: "Unsafe marker check".to_string(),
        policy_type: PolicyType::SafetyCheck,
        active: true,
    });
    guard.add_policy(Policy {
        id: ProtocolId::from_str("format").unwrap_or_else(|_| ProtocolId::generate()),
        name: "Format".to_string(),
        description: "Format check".to_string(),
        policy_type: PolicyType::FormatValidation,
        active: true,
    });
    guard.add_policy(Policy {
        id: ProtocolId::from_str("content").unwrap_or_else(|_| ProtocolId::generate()),
        name: "Content".to_string(),
        description: "Content check".to_string(),
        policy_type: PolicyType::ContentFilter,
        active: true,
    });
    guard.add_policy(Policy {
        id: ProtocolId::from_str("custom").unwrap_or_else(|_| ProtocolId::generate()),
        name: "Custom".to_string(),
        description: "Custom check".to_string(),
        policy_type: PolicyType::Custom("always-pass".to_string()),
        active: true,
    });
    let output = ExpertOutput {
        expert_id: expert_id.clone(),
        content: "safe deterministic output".to_string(),
        confidence: 0.8,
        metadata: HashMap::new(),
        trace: vec!["ok".to_string()],
    };
    let policy_results = guard.validate(&output);
    let first_policy_result: Option<PolicyResult> = policy_results.first().cloned();
    guard.validate_strict(&output)?;
    let removed_policy = guard
        .remove_policy(&ProtocolId::from_str("custom").unwrap_or_else(|_| ProtocolId::generate()));
    tracing::info!(
        "Policy guard: results={} active={} removed_custom={}",
        policy_results.len(),
        guard.active_policy_count(),
        removed_policy
    );
    if let Some(sample) = first_policy_result {
        tracing::info!(
            "First policy result: {} => {}",
            sample.policy_id,
            sample.passed
        );
    }

    let mut registry = ExpertRegistry::new();
    let router_codegen_id = ProtocolId::generate();
    registry.register(Box::new(SpecializedExpert::code_generation_with_id(
        router_codegen_id,
        "router_codegen",
    )))?;
    let router_retrieval_id = ProtocolId::generate();
    registry.register(Box::new(SpecializedExpert::validation_with_id(
        router_retrieval_id,
        "router_validator",
    )))?;
    let route_task = Task::new_with_id(
        ProtocolId::generate(),
        TaskType::CodeGeneration,
        "build routing",
    );
    let capability_hits = registry
        .find_by_capability(&ExpertCapability::CodeGeneration)
        .len();
    let task_hits = registry.find_for_task(&route_task).len();
    let active = registry.list_active().len();
    let contains_router = registry.contains(&ExpertId::from_protocol_id(router_codegen_id));

    let removed = registry
        .deregister(&ExpertId::from_protocol_id(router_retrieval_id))
        .is_some();
    tracing::info!(
        "Registry: count={} active={} cap_hits={} task_hits={} contains={} removed={}",
        registry.count(),
        active,
        capability_hits,
        task_hits,
        contains_router,
        removed
    );

    let router_instance = HeuristicRouter::new(2);
    let routed = Router::route(&router_instance, &route_task, &registry)?;
    let fallback_variant = RoutingStrategy::Fallback;
    tracing::info!(
        "Router run: selected={} strategy={:?} fallback_variant={:?}",
        routed.selected_experts.len(),
        routed.strategy,
        fallback_variant
    );

    let mut pipeline: MoePipeline = MoePipelineBuilder::new()
        .with_router(Box::new(HeuristicRouter::new(2)))
        .with_aggregation_strategy(AggregationStrategy::WeightedAverage)
        .with_max_traces(128)
        .build();
    pipeline.register_expert(Box::new(SpecializedExpert::code_generation_with_id(
        ProtocolId::generate(),
        "pipeline_codegen",
    )))?;
    pipeline.add_policy(Policy {
        id: ProtocolId::generate(),
        name: "Length".to_string(),
        description: "max length".to_string(),
        policy_type: PolicyType::LengthLimit(10_000),
        active: true,
    });
    let pipeline_task = Task::new_with_id(
        ProtocolId::generate(),
        TaskType::CodeGeneration,
        "build component graph",
    );
    let pipeline_result = pipeline.execute(pipeline_task)?;
    pipeline.add_feedback(FeedbackEntry {
        id: ProtocolId::generate(),
        task_id: task_id.clone(),
        expert_id: expert_id.clone(),
        feedback_type: FeedbackType::Suggestion,
        score: Some(0.75),
        comment: "wire to central engine".to_string(),
        created_at: 3,
    });
    tracing::info!(
        "Pipeline: outputs={} eval_routings={} feedback={} traces={} dataset={}",
        pipeline_result.outputs.len(),
        pipeline.evaluation().get_routing_metrics().total_routings,
        pipeline.feedback_store().count(),
        pipeline.trace_logger().count(),
        pipeline.dataset_store().count()
    );

    let mut restore_pipeline = run_runtime_persistence_checks()?;
    run_training_and_cas_checks(&mut restore_pipeline)?;
    run_concurrent_pipeline_checks()?;

    tracing::info!("Implementation check completed.");
    Ok(())
}
