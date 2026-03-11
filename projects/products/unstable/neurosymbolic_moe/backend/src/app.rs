//! projects/products/unstable/neurosymbolic_moe/backend/src/app.rs
use std::collections::HashMap;
use std::path::PathBuf;

use crate::aggregator::AggregationStrategy;
use crate::buffer_manager::{BufferEntry, BufferManager, BufferType, SessionBuffer, WorkingBuffer};
use crate::dataset_engine::{Correction, DatasetEntry, DatasetStore, Outcome, TraceConverter};
use crate::evaluation_engine::{EvaluationEngine, ExpertMetrics, RoutingMetrics};
use crate::expert_registry::{ExpertRegistry, VersionEntry, VersionTracker};
use crate::feedback_engine::{FeedbackEntry, FeedbackStore, FeedbackType};
use crate::memory_engine::{
    LongTermMemory, MemoryEntry, MemoryQuery, MemoryStore, MemoryType, ShortTermMemory,
};
use crate::moe_core::{
    ExecutionContext, Expert, ExpertCapability, ExpertError, ExpertId, ExpertMetadata,
    ExpertOutput, ExpertStatus, ExpertType, Task, TaskPriority, TaskType, TracePhase, TraceRecord,
};
use crate::orchestrator::{MoePipeline, MoePipelineBuilder};
use crate::policy_guard::{Policy, PolicyGuard, PolicyResult, PolicyType};
use crate::retrieval_engine::{
    Chunk, Chunker, ChunkingStrategy, ContextAssembler, RetrievalQuery, RetrievalResult, Retriever,
    SimpleRetriever,
};
use crate::router::{HeuristicRouter, Router, RoutingDecision, RoutingStrategy, RoutingTrace};
use crate::trace_logger::TraceLogger;

type DynError = Box<dyn std::error::Error>;

struct EchoExpert {
    metadata: ExpertMetadata,
}

impl EchoExpert {
    fn new(id: &str, name: &str, capabilities: Vec<ExpertCapability>) -> Self {
        Self {
            metadata: ExpertMetadata {
                id: ExpertId::new(id),
                name: name.to_string(),
                version: "0.1.0".to_string(),
                capabilities,
                status: ExpertStatus::Active,
                expert_type: ExpertType::Deterministic,
            },
        }
    }
}

impl Expert for EchoExpert {
    fn id(&self) -> &ExpertId {
        &self.metadata.id
    }

    fn metadata(&self) -> &ExpertMetadata {
        &self.metadata
    }

    fn can_handle(&self, _task: &Task) -> bool {
        true
    }

    fn execute(
        &self,
        task: &Task,
        context: &ExecutionContext,
    ) -> Result<ExpertOutput, ExpertError> {
        Ok(ExpertOutput {
            expert_id: self.metadata.id.clone(),
            content: format!(
                "[{}] processed: {} (ctx:{} mem:{} buf:{})",
                self.metadata.name,
                task.input(),
                context.retrieved_context.len(),
                context.memory_entries.len(),
                context.buffer_data.len()
            ),
            confidence: 0.9,
            metadata: HashMap::new(),
            trace: vec![format!("Expert {} executed", self.metadata.name)],
        })
    }
}

pub fn run() -> Result<(), DynError> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    match args[1].as_str() {
        "run" => cmd_run(&args[2..]),
        "status" => cmd_status(),
        "trace" => cmd_trace(&args[2..]),
        "impl-check" => cmd_impl_check(),
        other => {
            eprintln!("Unknown command: {other}");
            print_usage();
            Ok(())
        }
    }
}

fn cmd_run(args: &[String]) -> Result<(), DynError> {
    let input = if args.is_empty() {
        "default task input".to_string()
    } else {
        args.join(" ")
    };

    let router = HeuristicRouter::new(3);
    let mut pipeline = MoePipelineBuilder::new()
        .with_router(Box::new(router))
        .with_aggregation_strategy(AggregationStrategy::HighestConfidence)
        .with_max_traces(1000)
        .build();

    pipeline.register_expert(Box::new(EchoExpert::new(
        "code_gen",
        "CodeGenerationExpert",
        vec![ExpertCapability::CodeGeneration],
    )))?;
    pipeline.register_expert(Box::new(EchoExpert::new(
        "code_transform",
        "CodeTransformExpert",
        vec![ExpertCapability::CodeTransformation],
    )))?;
    pipeline.register_expert(Box::new(EchoExpert::new(
        "validator",
        "ValidationExpert",
        vec![ExpertCapability::Validation],
    )))?;

    pipeline.add_policy(Policy {
        id: "length_check".to_string(),
        name: "Output Length Check".to_string(),
        description: "Ensures output is not too long".to_string(),
        policy_type: PolicyType::LengthLimit(10000),
        active: true,
    });

    let task = Task::new("task-001", TaskType::CodeGeneration, input)
        .with_context("runtime")
        .with_priority(TaskPriority::High)
        .with_metadata("source", "cli");

    let task_kind = format!("{:?}", task.task_type());
    let task_priority = format!("{:?}", task.priority());
    let has_context = task.context().is_some();

    match pipeline.execute(task) {
        Ok(result) => {
            println!("Pipeline execution successful");
            if let Some(selected) = &result.selected_output {
                println!("Selected expert: {}", selected.expert_id.as_str());
                println!("Confidence: {:.2}", selected.confidence);
                println!("Output: {}", selected.content);
            }
            println!("Total outputs: {}", result.outputs.len());
            println!("Strategy: {}", result.strategy);
            println!("Task kind: {task_kind}, priority: {task_priority}, context: {has_context}");
        }
        Err(e) => {
            eprintln!("Pipeline execution failed: {e}");
        }
    }

    println!(
        "\nExpert registry: {} experts registered",
        pipeline.registry().count()
    );
    println!("Trace log: {} entries", pipeline.trace_logger().count());
    println!(
        "Dataset store: {} entries",
        pipeline.dataset_store().count()
    );

    Ok(())
}

fn cmd_status() -> Result<(), DynError> {
    println!("neurosymbolic_moe platform v0.1.0");
    println!();
    println!("Components:");
    println!("  moe_core          - Expert trait, Task model, ExecutionContext");
    println!("  expert_registry   - Pluggable expert registration");
    println!("  router            - Heuristic task routing");
    println!("  retrieval_engine  - RAG retrieval abstraction");
    println!("  memory_engine     - Short-term and long-term memory");
    println!("  buffer_manager    - Working and session buffers");
    println!("  dataset_engine    - Incremental trace-to-dataset pipeline");
    println!("  evaluation_engine - Expert and routing metrics");
    println!("  feedback_engine   - Execution feedback and corrections");
    println!("  aggregator        - Multi-expert output aggregation");
    println!("  policy_guard      - Output validation and policy checks");
    println!("  trace_logger      - Execution traces and telemetry");
    println!("  orchestrator      - Main orchestration pipeline");
    println!();
    println!("Use `impl-check` to run the full component wiring smoke test.");
    Ok(())
}

fn cmd_trace(args: &[String]) -> Result<(), DynError> {
    let trace_path = if args.is_empty() {
        None
    } else {
        Some(PathBuf::from(&args[0]))
    };

    if let Some(path) = trace_path {
        println!("Trace output path: {}", path.display());
    }

    let task_id = crate::moe_core::TaskId::new("trace-demo");
    let expert_id = ExpertId::new("trace-expert");
    let mut logger = TraceLogger::new(8);

    logger.log_phase(
        task_id.clone(),
        TracePhase::Routing,
        "route trace command".to_string(),
        Some(expert_id.clone()),
    );
    logger.log_phase(
        task_id.clone(),
        TracePhase::Aggregation,
        "aggregate trace command".to_string(),
        Some(expert_id.clone()),
    );

    let by_task = logger.get_by_task(&task_id);
    let by_phase = logger.get_by_phase(&TracePhase::Routing);
    let by_expert = logger.get_by_expert(&expert_id);
    let recent = logger.recent(1);

    println!(
        "Trace stats: total={} by_task={} by_phase={} by_expert={} recent={}",
        logger.count(),
        by_task.len(),
        by_phase.len(),
        by_expert.len(),
        recent.len()
    );

    logger.clear();
    println!("Trace logger cleared: {}", logger.count());
    Ok(())
}

fn cmd_impl_check() -> Result<(), DynError> {
    println!("Running implementation check...");

    let buffer_variants = [BufferType::Working, BufferType::Session];
    println!("Buffer variants wired: {}", buffer_variants.len());

    let task_id = crate::moe_core::TaskId::new("impl-task");
    let expert_id = ExpertId::new("impl-expert");

    let entry = BufferEntry::new("k", "v", 1)
        .with_task_id(task_id.clone())
        .with_session_id("s1");
    println!(
        "BufferEntry key={} created_at={}",
        entry.key, entry.created_at
    );

    let mut buffers = BufferManager::new(2);
    buffers
        .working_mut()
        .put("ctx", "value", Some(task_id.clone()));
    let working_get = buffers.working().get("ctx").map(|e| e.value.clone());
    let working_count = buffers.working().count();
    let working_keys_len = buffers.working().keys().len();
    let removed_working = buffers.working_mut().remove("ctx").is_some();
    println!(
        "Working buffer: count={} keys={} get={} removed={}",
        working_count,
        working_keys_len,
        working_get.unwrap_or_else(|| "none".to_string()),
        removed_working
    );

    buffers.sessions_mut().create_session("s1");
    buffers.sessions_mut().put("s1", "profile", "alpha");
    let session_get = buffers
        .sessions()
        .get("s1", "profile")
        .map(|e| e.value.clone())
        .unwrap_or_else(|| "none".to_string());
    let session_list_len = buffers.sessions().list_sessions().len();
    let session_count = buffers.sessions().session_count();
    let removed_session = buffers.sessions_mut().remove_session("s1");
    println!(
        "Session buffer: get={} list={} count={} removed={}",
        session_get, session_list_len, session_count, removed_session
    );
    buffers.clear_all();

    let mut direct_working = WorkingBuffer::new(1);
    direct_working.put("direct", "value", None);
    let mut direct_sessions = SessionBuffer::new();
    direct_sessions.create_session("direct-s");
    direct_sessions.put("direct-s", "k", "v");
    println!(
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
        id: "manual-ds".to_string(),
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
        entry_id: dataset_entry.id.clone(),
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
    println!(
        "Dataset: count={} by_task={} by_expert={} success={} corrections={} ok={} ko={}",
        dataset_store.count(),
        task_entries,
        expert_entries,
        success_entries,
        correction_entries,
        dataset_store.successful_count(),
        dataset_store.failed_count()
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
        .map(|m| m.expert_id.as_str().to_string());
    let worst = evaluation
        .worst_performing_expert()
        .map(|m| m.expert_id.as_str().to_string());
    println!(
        "Evaluation: expert_rate={expert_rate:.2} routing_accuracy={routing_accuracy:.2} best={:?} worst={:?}",
        best, worst
    );
    let mut manual_expert_metrics = ExpertMetrics::new(expert_id.clone());
    manual_expert_metrics.record_execution(true, 0.7, 8.0);
    let mut manual_routing_metrics = RoutingMetrics::new();
    manual_routing_metrics.record_routing(2, false);
    println!(
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
    println!("Version tracker: history={history_count} latest={latest}");

    let mut feedback_store = FeedbackStore::new();
    feedback_store.add(FeedbackEntry {
        id: "fb1".to_string(),
        task_id: task_id.clone(),
        expert_id: expert_id.clone(),
        feedback_type: FeedbackType::Positive,
        score: Some(0.9),
        comment: "good".to_string(),
        created_at: 2,
    });
    println!(
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
    };

    let mut short_store = ShortTermMemory::new(4);
    short_store.store(memory_short)?;
    short_store.store(memory_medium)?;
    let short_found = short_store.retrieve(&memory_query)?.len();
    let short_expired = short_store.expire(101);
    let short_after = short_store.count();
    let removed_short = short_store.remove("mem-short").is_some();
    println!(
        "Short memory: found={} expired={} count={} removed={}",
        short_found, short_expired, short_after, removed_short
    );

    let mut long_store = LongTermMemory::new();
    long_store.store(memory_long)?;
    let long_found = long_store.retrieve(&memory_query)?.len();
    let long_count = long_store.count();
    let long_removed = long_store.remove("mem-long").is_some();
    println!(
        "Long memory: found={} count={} removed={}",
        long_found, long_count, long_removed
    );

    let mut chunk = Chunk::new("c0", "Rust systems programming", "doc://a", 0, 25);
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
    println!(
        "Chunking: fixed={} paragraph={} semantic={}",
        fixed_chunks, paragraph_chunks, semantic_chunks
    );

    let mut retriever = SimpleRetriever::new();
    retriever.add_document(chunk);
    retriever.add_document(
        Chunk::new("c1", "Rust async ecosystem", "doc://b", 0, 19)
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

    let synthetic = RetrievalResult::new("manual-1", "manual context", 0.4, "manual://ctx")
        .with_metadata("kind", "manual");
    let mut all_results = retrieved.clone();
    all_results.push(synthetic);

    let task_for_context = Task::new("ctx-task", TaskType::Retrieval, "need retrieval context")
        .with_context("contextual")
        .with_priority(TaskPriority::Critical)
        .with_metadata("intent", "demo");
    let assembler = ContextAssembler::new(120);
    let assembled = assembler.assemble(&all_results);
    let assembled_for_task = assembler.assemble_for_task(&all_results, &task_for_context);
    println!(
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
    println!(
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
    println!(
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
    println!(
        "Trace logger stats: total={} task={} phase={} expert={} recent={}",
        logger.count(),
        logger.get_by_task(&task_id).len(),
        logger.get_by_phase(&TracePhase::Validation).len(),
        logger.get_by_expert(&expert_id).len(),
        logger.recent(2).len()
    );

    let mut guard = PolicyGuard::new();
    guard.add_policy(Policy {
        id: "safety".to_string(),
        name: "Safety".to_string(),
        description: "Unsafe marker check".to_string(),
        policy_type: PolicyType::SafetyCheck,
        active: true,
    });
    guard.add_policy(Policy {
        id: "format".to_string(),
        name: "Format".to_string(),
        description: "Format check".to_string(),
        policy_type: PolicyType::FormatValidation,
        active: true,
    });
    guard.add_policy(Policy {
        id: "content".to_string(),
        name: "Content".to_string(),
        description: "Content check".to_string(),
        policy_type: PolicyType::ContentFilter,
        active: true,
    });
    guard.add_policy(Policy {
        id: "custom".to_string(),
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
    let removed_policy = guard.remove_policy("custom");
    println!(
        "Policy guard: results={} active={} removed_custom={}",
        policy_results.len(),
        guard.active_policy_count(),
        removed_policy
    );
    if let Some(sample) = first_policy_result {
        println!(
            "First policy result: {} => {}",
            sample.policy_id, sample.passed
        );
    }

    let mut registry = ExpertRegistry::new();
    registry.register(Box::new(EchoExpert::new(
        "router_codegen",
        "RouterCodegen",
        vec![ExpertCapability::CodeGeneration],
    )))?;
    registry.register(Box::new(EchoExpert::new(
        "router_retrieval",
        "RouterRetrieval",
        vec![ExpertCapability::Retrieval],
    )))?;
    let route_task = Task::new("route-task", TaskType::CodeGeneration, "build routing");
    let capability_hits = registry
        .find_by_capability(&ExpertCapability::CodeGeneration)
        .len();
    let task_hits = registry.find_for_task(&route_task).len();
    let active = registry.list_active().len();
    let contains_router = registry.contains(&ExpertId::new("router_codegen"));
    let removed = registry
        .deregister(&ExpertId::new("router_retrieval"))
        .is_some();
    println!(
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
    println!(
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
    pipeline.register_expert(Box::new(EchoExpert::new(
        "pipeline_codegen",
        "PipelineCodegen",
        vec![ExpertCapability::CodeGeneration],
    )))?;
    pipeline.add_policy(Policy {
        id: "len".to_string(),
        name: "Length".to_string(),
        description: "max length".to_string(),
        policy_type: PolicyType::LengthLimit(10_000),
        active: true,
    });
    let pipeline_task = Task::new(
        "pipeline-task",
        TaskType::CodeGeneration,
        "build component graph",
    );
    let pipeline_result = pipeline.execute(pipeline_task)?;
    pipeline.add_feedback(FeedbackEntry {
        id: "fb-pipeline".to_string(),
        task_id: task_id.clone(),
        expert_id: expert_id.clone(),
        feedback_type: FeedbackType::Suggestion,
        score: Some(0.75),
        comment: "wire to central engine".to_string(),
        created_at: 3,
    });
    println!(
        "Pipeline: outputs={} eval_routings={} feedback={} traces={} dataset={}",
        pipeline_result.outputs.len(),
        pipeline.evaluation().get_routing_metrics().total_routings,
        pipeline.feedback_store().count(),
        pipeline.trace_logger().count(),
        pipeline.dataset_store().count()
    );

    println!("Implementation check completed.");
    Ok(())
}

fn print_usage() {
    println!("neurosymbolic_moe - advanced modular MoE platform");
    println!();
    println!("Commands:");
    println!("  run [input...]     Execute a task through the MoE pipeline");
    println!("  status             Show platform component status");
    println!("  trace [path]       Inspect execution traces");
    println!("  impl-check         Execute full component wiring check");
}
