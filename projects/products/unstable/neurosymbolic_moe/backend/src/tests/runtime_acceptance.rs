mod v0 {
    use crate::aggregator::{AggregationStrategy, OutputAggregator};
    use crate::moe_core::{
        ExecutionContext, Expert, ExpertCapability, ExpertError, ExpertId, ExpertMetadata,
        ExpertOutput, ExpertStatus, ExpertType, Task, TaskType, TracePhase, TraceRecord,
    };
    use crate::orchestrator::MoePipelineBuilder;
    use crate::router::{HeuristicRouter, Router};
    use std::collections::HashMap;

    struct V0Expert {
        metadata: ExpertMetadata,
    }

    impl V0Expert {
        fn new(id: &str) -> Self {
            Self {
                metadata: ExpertMetadata {
                    id: ExpertId::new(id),
                    name: "v0-expert".to_string(),
                    version: "0.1.0".to_string(),
                    capabilities: vec![ExpertCapability::CodeGeneration],
                    status: ExpertStatus::Active,
                    expert_type: ExpertType::Deterministic,
                },
            }
        }
    }

    impl Expert for V0Expert {
        fn id(&self) -> &ExpertId {
            &self.metadata.id
        }

        fn metadata(&self) -> &ExpertMetadata {
            &self.metadata
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
                expert_id: self.metadata.id.clone(),
                content: format!("v0:{}", task.input()),
                confidence: 1.0,
                metadata: HashMap::new(),
                trace: Vec::new(),
            })
        }
    }

    #[test]
    fn v0_core_contracts_are_wired() {
        let expert_port: &dyn Expert = &V0Expert::new("expert-v0");
        let router = HeuristicRouter::default();
        let router_port: &dyn Router = &router;

        let _ = expert_port.id();
        let _ = router_port;
    }

    #[test]
    fn v0_models_and_aggregation_flow_work() {
        let task = Task::new("task-v0", TaskType::CodeGeneration, "hello");
        let output = ExpertOutput {
            expert_id: ExpertId::new("expert-v0"),
            content: "ok".to_string(),
            confidence: 0.8,
            metadata: HashMap::new(),
            trace: Vec::new(),
        };
        let aggregator = OutputAggregator::new(AggregationStrategy::First);
        let aggregated = aggregator
            .aggregate(vec![output])
            .expect("v0 aggregation should succeed");
        assert_eq!(task.id().as_str(), "task-v0");
        assert_eq!(aggregated.outputs.len(), 1);
        assert_eq!(aggregated.strategy, "first");
    }

    #[test]
    fn v0_trace_model_is_usable() {
        let trace = TraceRecord {
            trace_id: "trace-v0".to_string(),
            task_id: crate::moe_core::TaskId::new("task-v0"),
            timestamp: 1,
            expert_id: Some(ExpertId::new("expert-v0")),
            phase: TracePhase::Routing,
            detail: "routed".to_string(),
            metadata: HashMap::new(),
        };
        assert_eq!(trace.trace_id, "trace-v0");
        assert!(matches!(trace.phase, TracePhase::Routing));
    }

    #[test]
    fn v0_minimal_orchestration_flow_executes() {
        let mut pipeline = MoePipelineBuilder::new().build();
        pipeline
            .register_expert(Box::new(V0Expert::new("expert-v0")))
            .expect("registering v0 expert should succeed");

        let task = Task::new("task-v0", TaskType::CodeGeneration, "run");
        let result = pipeline
            .execute(task)
            .expect("v0 orchestration should succeed");
        assert!(result.selected_output.is_some());
        assert!(pipeline.trace_logger().count() > 0);
    }
}

mod v1 {
    use crate::aggregator::{AggregationStrategy, OutputAggregator};
    use crate::moe_core::{
        ExecutionContext, Expert, ExpertCapability, ExpertError, ExpertId, ExpertMetadata,
        ExpertOutput, ExpertStatus, ExpertType, MoeError, Task, TaskType, TracePhase,
    };
    use crate::orchestrator::MoePipelineBuilder;
    use crate::router::{HeuristicRouter, Router};
    use std::collections::HashMap;

    struct RoutingExpert {
        metadata: ExpertMetadata,
        confidence: f64,
        fail: bool,
    }

    impl RoutingExpert {
        fn new(id: &str, capabilities: Vec<ExpertCapability>, confidence: f64, fail: bool) -> Self {
            Self {
                metadata: ExpertMetadata {
                    id: ExpertId::new(id),
                    name: id.to_string(),
                    version: "1.0.0".to_string(),
                    capabilities,
                    status: ExpertStatus::Active,
                    expert_type: ExpertType::Deterministic,
                },
                confidence,
                fail,
            }
        }
    }

    impl Expert for RoutingExpert {
        fn id(&self) -> &ExpertId {
            &self.metadata.id
        }

        fn metadata(&self) -> &ExpertMetadata {
            &self.metadata
        }

        fn can_handle(&self, task: &Task) -> bool {
            matches!(task.task_type(), TaskType::CodeGeneration)
        }

        fn execute(
            &self,
            task: &Task,
            _context: &ExecutionContext,
        ) -> Result<ExpertOutput, ExpertError> {
            if self.fail {
                return Err(ExpertError::ExecutionFailed("forced failure".to_string()));
            }
            Ok(ExpertOutput {
                expert_id: self.metadata.id.clone(),
                content: format!("{}:{}", self.metadata.name, task.input()),
                confidence: self.confidence,
                metadata: HashMap::new(),
                trace: Vec::new(),
            })
        }
    }

    #[test]
    fn v1_heuristic_router_selects_matching_experts() {
        let mut pipeline = MoePipelineBuilder::new()
            .with_router(Box::new(HeuristicRouter::new(3)))
            .build();
        pipeline
            .register_expert(Box::new(RoutingExpert::new(
                "code-a",
                vec![ExpertCapability::CodeGeneration],
                0.9,
                false,
            )))
            .expect("registering first expert should succeed");
        pipeline
            .register_expert(Box::new(RoutingExpert::new(
                "code-b",
                vec![ExpertCapability::CodeGeneration],
                0.8,
                false,
            )))
            .expect("registering second expert should succeed");

        let task = Task::new("v1-router", TaskType::CodeGeneration, "build");
        let decision = HeuristicRouter::new(3)
            .route(&task, pipeline.registry())
            .expect("routing should succeed");
        assert!(!decision.selected_experts.is_empty());
    }

    #[test]
    fn v1_duplicate_expert_registration_returns_error() {
        let mut pipeline = MoePipelineBuilder::new().build();
        pipeline
            .register_expert(Box::new(RoutingExpert::new(
                "dup",
                vec![ExpertCapability::CodeGeneration],
                0.9,
                false,
            )))
            .expect("initial registration should succeed");
        let duplicate = pipeline.register_expert(Box::new(RoutingExpert::new(
            "dup",
            vec![ExpertCapability::CodeGeneration],
            0.9,
            false,
        )));
        assert!(duplicate.is_err());
    }

    #[test]
    fn v1_single_expert_execution_runs_successfully() {
        let mut pipeline = MoePipelineBuilder::new()
            .with_router(Box::new(HeuristicRouter::new(1)))
            .with_aggregation_strategy(AggregationStrategy::HighestConfidence)
            .build();
        pipeline
            .register_expert(Box::new(RoutingExpert::new(
                "single",
                vec![ExpertCapability::CodeGeneration],
                0.95,
                false,
            )))
            .expect("expert registration should succeed");

        let task = Task::new("v1-single", TaskType::CodeGeneration, "single run");
        let result = pipeline.execute(task).expect("pipeline should succeed");
        assert_eq!(result.outputs.len(), 1);
        let selected = result
            .selected_output
            .expect("selected output should be present");
        assert_eq!(selected.expert_id.as_str(), "single");
    }

    #[test]
    fn v1_multi_expert_execution_runs_successfully() {
        let mut pipeline = MoePipelineBuilder::new()
            .with_router(Box::new(HeuristicRouter::new(3)))
            .with_aggregation_strategy(AggregationStrategy::HighestConfidence)
            .build();
        pipeline
            .register_expert(Box::new(RoutingExpert::new(
                "multi-a",
                vec![ExpertCapability::CodeGeneration],
                0.7,
                false,
            )))
            .expect("first expert registration should succeed");
        pipeline
            .register_expert(Box::new(RoutingExpert::new(
                "multi-b",
                vec![ExpertCapability::CodeGeneration],
                0.9,
                false,
            )))
            .expect("second expert registration should succeed");

        let task = Task::new("v1-multi", TaskType::CodeGeneration, "multi run");
        let result = pipeline.execute(task).expect("pipeline should succeed");
        assert!(result.outputs.len() >= 2);
    }

    #[test]
    fn v1_routing_traces_are_recorded() {
        let mut pipeline = MoePipelineBuilder::new().build();
        pipeline
            .register_expert(Box::new(RoutingExpert::new(
                "trace-a",
                vec![ExpertCapability::CodeGeneration],
                0.9,
                false,
            )))
            .expect("expert registration should succeed");

        let task = Task::new("v1-trace", TaskType::CodeGeneration, "trace run");
        let _ = pipeline.execute(task).expect("pipeline should succeed");
        let routing_traces = pipeline.trace_logger().get_by_phase(&TracePhase::Routing);
        assert!(!routing_traces.is_empty());
    }

    #[test]
    fn v1_expert_failure_is_propagated() {
        let mut pipeline = MoePipelineBuilder::new().build();
        pipeline
            .register_expert(Box::new(RoutingExpert::new(
                "fail-a",
                vec![ExpertCapability::CodeGeneration],
                0.9,
                true,
            )))
            .expect("expert registration should succeed");

        let task = Task::new("v1-fail", TaskType::CodeGeneration, "fail run");
        let error = pipeline
            .execute(task)
            .expect_err("pipeline should return an expert failure");
        assert!(matches!(error, MoeError::ExpertError(_)));
    }

    #[test]
    fn v1_basic_aggregation_works() {
        let aggregator = OutputAggregator::new(AggregationStrategy::HighestConfidence);
        let outputs = vec![
            ExpertOutput {
                expert_id: ExpertId::new("a"),
                content: "a".to_string(),
                confidence: 0.6,
                metadata: HashMap::new(),
                trace: Vec::new(),
            },
            ExpertOutput {
                expert_id: ExpertId::new("b"),
                content: "b".to_string(),
                confidence: 0.9,
                metadata: HashMap::new(),
                trace: Vec::new(),
            },
        ];

        let aggregated = aggregator
            .aggregate(outputs)
            .expect("aggregation should succeed");
        let selected = aggregated
            .selected_output
            .expect("selected output should be present");
        assert_eq!(selected.expert_id.as_str(), "b");
    }
}

mod v2 {
    use crate::memory_engine::{
        LongTermMemory, MemoryEntry, MemoryQuery, MemoryStore, MemoryType, ShortTermMemory,
    };
    use crate::moe_core::Task;
    use crate::retrieval_engine::{
        Chunk, Chunker, ChunkingStrategy, ContextAssembler, RetrievalQuery, Retriever,
        SimpleRetriever,
    };
    use std::collections::HashMap;

    fn memory_entry(
        id: &str,
        content: &str,
        tags: Vec<&str>,
        created_at: u64,
        expires_at: Option<u64>,
        memory_type: MemoryType,
        relevance: f64,
    ) -> MemoryEntry {
        MemoryEntry {
            id: id.to_string(),
            content: content.to_string(),
            tags: tags.into_iter().map(|tag| tag.to_string()).collect(),
            created_at,
            expires_at,
            memory_type,
            relevance,
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn v2_retriever_supports_filtering_and_ranking() {
        let mut retriever = SimpleRetriever::new();
        retriever.add_document(
            Chunk::new("c1", "rust rust deterministic systems", "src://a", 0, 30)
                .with_metadata("domain", "systems"),
        );
        retriever.add_document(
            Chunk::new("c2", "rust docs and markdown", "src://b", 0, 22)
                .with_metadata("domain", "docs"),
        );
        retriever.add_document(
            Chunk::new("c3", "python scripting utilities", "src://c", 0, 24)
                .with_metadata("domain", "systems"),
        );

        let query = RetrievalQuery::new("rust")
            .with_filter("domain", "systems")
            .with_min_relevance(0.05)
            .with_max_results(5);
        let results = retriever
            .retrieve(&query)
            .expect("retrieval should succeed");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].chunk_id, "c1");
        assert!(results[0].relevance_score > 0.0);
    }

    #[test]
    fn v2_chunking_strategies_generate_chunks() {
        let fixed = Chunker::new(ChunkingStrategy::FixedSize(4));
        let paragraph = Chunker::new(ChunkingStrategy::Paragraph);
        let semantic = Chunker::new(ChunkingStrategy::Semantic);

        let fixed_chunks = fixed.chunk("abcdefgh", "doc://fixed");
        let paragraph_chunks = paragraph.chunk("para one\n\npara two", "doc://paragraph");
        let semantic_chunks = semantic.chunk("Sentence one. Sentence two!", "doc://semantic");

        assert_eq!(fixed_chunks.len(), 2);
        assert_eq!(paragraph_chunks.len(), 2);
        assert!(!semantic_chunks.is_empty());
    }

    #[test]
    fn v2_context_assembly_is_budgeted_and_task_aware() {
        let results = vec![
            crate::retrieval_engine::RetrievalResult::new("c1", "AAAAAAAAAA", 0.9, "doc://a"),
            crate::retrieval_engine::RetrievalResult::new("c2", "BBBBBBBBBB", 0.8, "doc://b"),
        ];
        let assembler = ContextAssembler::new(12);
        let assembled = assembler.assemble(&results);
        let total_len: usize = assembled.iter().map(String::len).sum();
        assert!(total_len <= 12);

        let task = Task::new(
            "task-v2",
            crate::moe_core::TaskType::Retrieval,
            "lookup context",
        );
        let task_context = assembler.assemble_for_task(&results, &task);
        assert!(!task_context.is_empty());
        assert!(task_context[0].starts_with("[task:"));
        let task_total_len: usize = task_context.iter().map(String::len).sum();
        assert!(task_total_len <= 12);
    }

    #[test]
    fn v2_short_term_memory_supports_capacity_retrieve_and_expire() {
        let mut short = ShortTermMemory::new(2);
        short
            .store(memory_entry(
                "m1",
                "alpha",
                vec!["runtime"],
                1,
                Some(10),
                MemoryType::Short,
                0.9,
            ))
            .expect("storing m1 should succeed");
        short
            .store(memory_entry(
                "m2",
                "beta",
                vec!["runtime"],
                2,
                Some(20),
                MemoryType::Short,
                0.8,
            ))
            .expect("storing m2 should succeed");
        short
            .store(memory_entry(
                "m3",
                "gamma",
                vec!["runtime"],
                3,
                Some(30),
                MemoryType::Short,
                0.7,
            ))
            .expect("storing m3 should succeed");

        assert_eq!(short.count(), 2);
        assert!(short.remove("m1").is_none());

        let query = MemoryQuery {
            tags: Some(vec!["runtime".to_string()]),
            memory_type: Some(MemoryType::Short),
            min_relevance: Some(0.0),
            max_results: 10,
            include_expired: false,
            current_time: Some(15),
        };
        let retrieved = short
            .retrieve(&query)
            .expect("retrieval from short memory should succeed");
        assert!(!retrieved.is_empty());

        let expired = short.expire(25);
        assert!(expired >= 1);
    }

    #[test]
    fn v2_long_term_memory_supports_retrieve_and_remove() {
        let mut long = LongTermMemory::new();
        long.store(memory_entry(
            "l1",
            "knowledge",
            vec!["history"],
            1,
            None,
            MemoryType::Long,
            0.95,
        ))
        .expect("storing long-term entry should succeed");

        let query = MemoryQuery {
            tags: Some(vec!["history".to_string()]),
            memory_type: Some(MemoryType::Long),
            min_relevance: Some(0.5),
            max_results: 5,
            include_expired: true,
            current_time: Some(0),
        };
        let retrieved = long
            .retrieve(&query)
            .expect("retrieval from long memory should succeed");
        assert_eq!(retrieved.len(), 1);

        let removed = long.remove("l1");
        assert!(removed.is_some());
        assert_eq!(long.count(), 0);
    }
}

mod v3 {
    use crate::dataset_engine::{Correction, DatasetStore, Outcome, TraceConverter};
    use crate::feedback_engine::{FeedbackEntry, FeedbackStore, FeedbackType};
    use crate::moe_core::{
        ExecutionContext, Expert, ExpertCapability, ExpertError, ExpertId, ExpertMetadata,
        ExpertOutput, ExpertStatus, ExpertType, Task, TaskType, TracePhase,
    };
    use crate::orchestrator::MoePipelineBuilder;
    use crate::trace_logger::TraceLogger;
    use std::collections::HashMap;

    struct V3Expert {
        metadata: ExpertMetadata,
    }

    impl V3Expert {
        fn new(id: &str) -> Self {
            Self {
                metadata: ExpertMetadata {
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

    impl Expert for V3Expert {
        fn id(&self) -> &ExpertId {
            &self.metadata.id
        }

        fn metadata(&self) -> &ExpertMetadata {
            &self.metadata
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
                expert_id: self.metadata.id.clone(),
                content: format!("v3:{}", task.input()),
                confidence: 0.91,
                metadata: HashMap::new(),
                trace: Vec::new(),
            })
        }
    }

    #[test]
    fn v3_trace_to_dataset_conversion_and_corrections_work() {
        let mut logger = TraceLogger::new(32);
        let task_id = crate::moe_core::TaskId::new("v3-task");
        let expert_id = ExpertId::new("v3-expert");

        logger.log_phase(
            task_id.clone(),
            TracePhase::Routing,
            "route".to_string(),
            Some(expert_id.clone()),
        );
        logger.log_phase(
            task_id.clone(),
            TracePhase::ExpertExecution,
            "execute".to_string(),
            Some(expert_id.clone()),
        );

        let traces: Vec<crate::moe_core::TraceRecord> =
            logger.get_by_task(&task_id).into_iter().cloned().collect();
        let converter = TraceConverter::new();
        let entry = converter.convert(&traces, "input", "output", Outcome::Success);

        let mut store = DatasetStore::new();
        store.add_entry(entry.clone());
        store.add_correction(Correction {
            entry_id: entry.id.clone(),
            corrected_output: "output-fixed".to_string(),
            reason: "human-feedback".to_string(),
            corrected_at: 999,
        });

        assert_eq!(store.count(), 1);
        assert_eq!(store.successful_count(), 1);
        let corrections = store
            .get_corrections(&entry.id)
            .expect("corrections should exist for dataset entry");
        assert_eq!(corrections.len(), 1);
    }

    #[test]
    fn v3_feedback_store_supports_scoring_and_filters() {
        let mut feedback = FeedbackStore::new();
        let task_id = crate::moe_core::TaskId::new("v3-feedback-task");
        let expert_id = ExpertId::new("v3-feedback-expert");

        feedback.add(FeedbackEntry {
            id: "fb1".to_string(),
            task_id: task_id.clone(),
            expert_id: expert_id.clone(),
            feedback_type: FeedbackType::Positive,
            score: Some(0.8),
            comment: "good".to_string(),
            created_at: 1,
        });
        feedback.add(FeedbackEntry {
            id: "fb2".to_string(),
            task_id: task_id.clone(),
            expert_id: expert_id.clone(),
            feedback_type: FeedbackType::Suggestion,
            score: Some(0.6),
            comment: "improve".to_string(),
            created_at: 2,
        });

        assert_eq!(feedback.count(), 2);
        assert_eq!(feedback.get_by_task(&task_id).len(), 2);
        assert_eq!(feedback.get_by_expert(&expert_id).len(), 2);
        assert_eq!(feedback.get_by_type(&FeedbackType::Positive).len(), 1);
        let average = feedback
            .average_score_for_expert(&expert_id)
            .expect("average should exist for scored expert");
        assert!((average - 0.7).abs() < f64::EPSILON);
    }

    #[test]
    fn v3_pipeline_persists_dataset_and_accepts_feedback() {
        let mut pipeline = MoePipelineBuilder::new().build();
        pipeline
            .register_expert(Box::new(V3Expert::new("pipeline-v3-expert")))
            .expect("expert registration should succeed");

        let task = Task::new("v3-pipeline-task", TaskType::CodeGeneration, "ship");
        let result = pipeline
            .execute(task)
            .expect("pipeline execution should succeed");
        assert!(result.selected_output.is_some());
        assert!(pipeline.trace_logger().count() > 0);
        assert_eq!(pipeline.dataset_store().count(), 1);

        pipeline.add_feedback(FeedbackEntry {
            id: "pipeline-fb".to_string(),
            task_id: crate::moe_core::TaskId::new("v3-pipeline-task"),
            expert_id: ExpertId::new("pipeline-v3-expert"),
            feedback_type: FeedbackType::Correction,
            score: Some(0.9),
            comment: "accepted".to_string(),
            created_at: 10,
        });
        assert_eq!(pipeline.feedback_store().count(), 1);
    }
}

mod v4 {
    use crate::aggregator::AggregationStrategy;
    use crate::expert_registry::ExpertRegistry;
    use crate::moe_core::{
        ExecutionContext, Expert, ExpertCapability, ExpertError, ExpertId, ExpertMetadata,
        ExpertOutput, ExpertStatus, ExpertType, Task, TaskType,
    };
    use crate::orchestrator::{ArbitrationMode, MoePipelineBuilder};
    use crate::policy_guard::{Policy, PolicyType};
    use crate::router::{Router, RoutingDecision, RoutingStrategy};
    use std::collections::HashMap;

    struct FixedRouter {
        selected: Vec<ExpertId>,
        scores: HashMap<ExpertId, f64>,
    }

    impl Router for FixedRouter {
        fn route(
            &self,
            task: &Task,
            _registry: &ExpertRegistry,
        ) -> Result<RoutingDecision, crate::moe_core::MoeError> {
            Ok(RoutingDecision {
                task_id: task.id().clone(),
                selected_experts: self.selected.clone(),
                scores: self.scores.clone(),
                strategy: RoutingStrategy::MultiExpert,
                explanation: "fixed router decision".to_string(),
            })
        }
    }

    struct V4Expert {
        metadata: ExpertMetadata,
        confidence: f64,
        fail: bool,
        prefix: String,
    }

    impl V4Expert {
        fn new(
            id: &str,
            capabilities: Vec<ExpertCapability>,
            confidence: f64,
            fail: bool,
            prefix: &str,
        ) -> Self {
            Self {
                metadata: ExpertMetadata {
                    id: ExpertId::new(id),
                    name: id.to_string(),
                    version: "1.0.0".to_string(),
                    capabilities,
                    status: ExpertStatus::Active,
                    expert_type: ExpertType::Deterministic,
                },
                confidence,
                fail,
                prefix: prefix.to_string(),
            }
        }
    }

    impl Expert for V4Expert {
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
            _context: &ExecutionContext,
        ) -> Result<ExpertOutput, ExpertError> {
            if self.fail {
                return Err(ExpertError::ExecutionFailed("forced failure".to_string()));
            }
            Ok(ExpertOutput {
                expert_id: self.metadata.id.clone(),
                content: format!("{}{}", self.prefix, task.input()),
                confidence: self.confidence,
                metadata: HashMap::new(),
                trace: Vec::new(),
            })
        }
    }

    #[test]
    fn v4_router_weighted_arbitration_can_override_highest_confidence() {
        let low_conf_high_router = ExpertId::new("low-conf-high-router");
        let high_conf_low_router = ExpertId::new("high-conf-low-router");

        let mut scores = HashMap::new();
        scores.insert(low_conf_high_router.clone(), 1.0);
        scores.insert(high_conf_low_router.clone(), 0.1);

        let fixed_router = FixedRouter {
            selected: vec![low_conf_high_router.clone(), high_conf_low_router.clone()],
            scores,
        };

        let mut pipeline = MoePipelineBuilder::new()
            .with_router(Box::new(fixed_router))
            .with_aggregation_strategy(AggregationStrategy::HighestConfidence)
            .with_arbitration_mode(ArbitrationMode::RouterScoreWeighted)
            .build();

        pipeline
            .register_expert(Box::new(V4Expert::new(
                low_conf_high_router.as_str(),
                vec![ExpertCapability::CodeGeneration],
                0.5,
                false,
                "low:",
            )))
            .expect("registering low confidence expert should succeed");
        pipeline
            .register_expert(Box::new(V4Expert::new(
                high_conf_low_router.as_str(),
                vec![ExpertCapability::CodeGeneration],
                0.9,
                false,
                "high:",
            )))
            .expect("registering high confidence expert should succeed");

        let task = Task::new("v4-arb", TaskType::CodeGeneration, "input");
        let result = pipeline.execute(task).expect("pipeline should succeed");

        let selected = result
            .selected_output
            .expect("selected output should be present");
        assert_eq!(selected.expert_id.as_str(), low_conf_high_router.as_str());
        assert!(result.strategy.starts_with("router_score_weighted+"));
    }

    #[test]
    fn v4_fallback_continues_when_primary_expert_fails() {
        let primary = ExpertId::new("primary-fail");
        let fallback = ExpertId::new("fallback-ok");

        let mut scores = HashMap::new();
        scores.insert(primary.clone(), 1.0);
        scores.insert(fallback.clone(), 0.9);

        let fixed_router = FixedRouter {
            selected: vec![primary.clone(), fallback.clone()],
            scores,
        };

        let mut pipeline = MoePipelineBuilder::new()
            .with_router(Box::new(fixed_router))
            .with_fallback_on_expert_error(true)
            .with_aggregation_strategy(AggregationStrategy::First)
            .build();

        pipeline
            .register_expert(Box::new(V4Expert::new(
                primary.as_str(),
                vec![ExpertCapability::CodeGeneration],
                0.8,
                true,
                "primary:",
            )))
            .expect("registering failing primary expert should succeed");
        pipeline
            .register_expert(Box::new(V4Expert::new(
                fallback.as_str(),
                vec![ExpertCapability::CodeGeneration],
                0.7,
                false,
                "fallback:",
            )))
            .expect("registering fallback expert should succeed");

        let task = Task::new("v4-fallback", TaskType::CodeGeneration, "payload");
        let result = pipeline
            .execute(task)
            .expect("fallback execution should succeed");
        let selected = result
            .selected_output
            .expect("selected output should be present");
        assert_eq!(selected.expert_id.as_str(), fallback.as_str());
    }

    #[test]
    fn v4_task_metadata_chain_supports_planner_executor_flow() {
        let mut pipeline = MoePipelineBuilder::new()
            .with_task_metadata_chain(true)
            .with_aggregation_strategy(AggregationStrategy::HighestConfidence)
            .build();

        pipeline
            .register_expert(Box::new(V4Expert::new(
                "planner",
                vec![ExpertCapability::IssuePlanning],
                0.6,
                false,
                "plan::",
            )))
            .expect("registering planner expert should succeed");
        pipeline
            .register_expert(Box::new(V4Expert::new(
                "executor",
                vec![ExpertCapability::CodeGeneration],
                0.95,
                false,
                "exec::",
            )))
            .expect("registering executor expert should succeed");

        let task = Task::new("v4-chain", TaskType::Planning, "ship feature")
            .with_metadata("expert_chain", "planner>executor");
        let result = pipeline
            .execute(task)
            .expect("chain execution should succeed");

        assert_eq!(result.outputs.len(), 2);
        assert_eq!(result.outputs[0].expert_id.as_str(), "planner");
        assert_eq!(result.outputs[1].expert_id.as_str(), "executor");
        assert_eq!(result.outputs[1].content, "exec::plan::ship feature");
        let selected = result
            .selected_output
            .expect("selected output should be present");
        assert_eq!(selected.expert_id.as_str(), "executor");
    }

    #[test]
    fn v4_enforcer_blocks_unsafe_output_before_chain_propagation() {
        let mut pipeline = MoePipelineBuilder::new()
            .with_task_metadata_chain(true)
            .with_aggregation_strategy(AggregationStrategy::HighestConfidence)
            .build();

        pipeline.add_policy(Policy {
            id: "safety".to_string(),
            name: "safety".to_string(),
            description: "reject unsafe markers".to_string(),
            policy_type: PolicyType::SafetyCheck,
            active: true,
        });

        pipeline
            .register_expert(Box::new(V4Expert::new(
                "planner",
                vec![ExpertCapability::IssuePlanning],
                0.7,
                false,
                "<UNSAFE>::",
            )))
            .expect("registering planner expert should succeed");
        pipeline
            .register_expert(Box::new(V4Expert::new(
                "executor",
                vec![ExpertCapability::CodeGeneration],
                0.9,
                false,
                "exec::",
            )))
            .expect("registering executor expert should succeed");

        let task = Task::new("v4-chain-enforcer", TaskType::Planning, "ship feature")
            .with_metadata("expert_chain", "planner>executor");
        let result = pipeline.execute(task);
        assert!(matches!(
            result,
            Err(crate::moe_core::MoeError::PolicyRejected(_))
        ));
    }

    #[test]
    fn v4_enforcer_reselects_policy_compliant_output_when_available() {
        let unsafe_primary = ExpertId::new("unsafe-primary");
        let safe_secondary = ExpertId::new("safe-secondary");

        let mut scores = HashMap::new();
        scores.insert(unsafe_primary.clone(), 1.0);
        scores.insert(safe_secondary.clone(), 0.8);

        let fixed_router = FixedRouter {
            selected: vec![unsafe_primary.clone(), safe_secondary.clone()],
            scores,
        };

        let mut pipeline = MoePipelineBuilder::new()
            .with_router(Box::new(fixed_router))
            .with_aggregation_strategy(AggregationStrategy::HighestConfidence)
            .build();

        pipeline.add_policy(Policy {
            id: "safety".to_string(),
            name: "safety".to_string(),
            description: "reject unsafe markers".to_string(),
            policy_type: PolicyType::SafetyCheck,
            active: true,
        });

        pipeline
            .register_expert(Box::new(V4Expert::new(
                unsafe_primary.as_str(),
                vec![ExpertCapability::CodeGeneration],
                0.95,
                false,
                "<UNSAFE>::",
            )))
            .expect("registering unsafe expert should succeed");
        pipeline
            .register_expert(Box::new(V4Expert::new(
                safe_secondary.as_str(),
                vec![ExpertCapability::CodeGeneration],
                0.70,
                false,
                "safe::",
            )))
            .expect("registering safe expert should succeed");

        let task = Task::new("v4-enforcer", TaskType::CodeGeneration, "payload");
        let result = pipeline
            .execute(task)
            .expect("pipeline should fallback to policy-compliant output");

        let selected = result
            .selected_output
            .expect("selected output should be present");
        assert_eq!(selected.expert_id.as_str(), safe_secondary.as_str());
        assert!(selected.content.starts_with("safe::"));
        assert!(result.strategy.ends_with("+policy_fallback"));
    }

    #[test]
    fn v4_enforcer_custom_policy_can_block_non_compliant_output() {
        let mut pipeline = MoePipelineBuilder::new()
            .with_aggregation_strategy(AggregationStrategy::First)
            .build();

        pipeline.add_policy(Policy {
            id: "custom-require-safe".to_string(),
            name: "custom-require-safe".to_string(),
            description: "require SAFE marker in output".to_string(),
            policy_type: PolicyType::Custom("require:SAFE".to_string()),
            active: true,
        });

        pipeline
            .register_expert(Box::new(V4Expert::new(
                "unsafe-custom",
                vec![ExpertCapability::CodeGeneration],
                0.9,
                false,
                "plain::",
            )))
            .expect("registering expert should succeed");

        let task = Task::new("v4-custom-enforcer", TaskType::CodeGeneration, "payload");
        let result = pipeline.execute(task);
        assert!(matches!(
            result,
            Err(crate::moe_core::MoeError::PolicyRejected(_))
        ));
    }
}

mod v5 {
    use crate::dataset_engine::{Correction, DatasetEntry, DatasetStore, Outcome};
    use crate::evaluation_engine::EvaluationEngine;
    use crate::expert_registry::ExpertRegistry;
    use crate::memory_engine::{MemoryEntry, MemoryType};
    use crate::moe_core::{
        ExecutionContext, Expert, ExpertCapability, ExpertError, ExpertId, ExpertMetadata,
        ExpertOutput, ExpertStatus, ExpertType, Task, TaskId, TaskType,
    };
    use crate::orchestrator::{
        ContinuousGovernancePolicy, ContinuousImprovementReport, GovernanceImportPolicy,
        GovernancePersistenceBundle, GovernanceState, MoePipelineBuilder, RuntimePersistenceBundle,
    };
    use crate::router::{Router, RoutingDecision, RoutingStrategy};
    use std::collections::HashMap;

    fn dataset_entry(id: &str, expert: &str, outcome: Outcome, score: Option<f64>) -> DatasetEntry {
        DatasetEntry {
            id: id.to_string(),
            task_id: TaskId::new(format!("task-{id}")),
            expert_id: ExpertId::new(expert),
            input: "input".to_string(),
            output: "output".to_string(),
            outcome,
            score,
            tags: vec!["acceptance".to_string()],
            created_at: 1,
            metadata: HashMap::new(),
        }
    }

    fn memory_entry(
        id: &str,
        content: &str,
        memory_type: MemoryType,
        created_at: u64,
    ) -> MemoryEntry {
        MemoryEntry {
            id: id.to_string(),
            content: content.to_string(),
            tags: vec!["acceptance".to_string()],
            created_at,
            expires_at: None,
            memory_type,
            relevance: 0.9,
            metadata: HashMap::new(),
        }
    }

    struct SingleExpertRouter {
        expert_id: ExpertId,
    }

    impl Router for SingleExpertRouter {
        fn route(
            &self,
            task: &Task,
            _registry: &ExpertRegistry,
        ) -> Result<RoutingDecision, crate::moe_core::MoeError> {
            let mut scores = HashMap::new();
            scores.insert(self.expert_id.clone(), 1.0);
            Ok(RoutingDecision {
                task_id: task.id().clone(),
                selected_experts: vec![self.expert_id.clone()],
                scores,
                strategy: RoutingStrategy::SingleExpert,
                explanation: "single expert route".to_string(),
            })
        }
    }

    struct LocalFixedRouter {
        selected: Vec<ExpertId>,
        scores: HashMap<ExpertId, f64>,
    }

    impl Router for LocalFixedRouter {
        fn route(
            &self,
            task: &Task,
            _registry: &ExpertRegistry,
        ) -> Result<RoutingDecision, crate::moe_core::MoeError> {
            Ok(RoutingDecision {
                task_id: task.id().clone(),
                selected_experts: self.selected.clone(),
                scores: self.scores.clone(),
                strategy: RoutingStrategy::MultiExpert,
                explanation: "fixed route for v5".to_string(),
            })
        }
    }

    struct V5FlakyExpert {
        metadata: ExpertMetadata,
    }

    impl V5FlakyExpert {
        fn new(id: &str) -> Self {
            Self {
                metadata: ExpertMetadata {
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

    impl Expert for V5FlakyExpert {
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
            _context: &ExecutionContext,
        ) -> Result<ExpertOutput, ExpertError> {
            if task.input().contains("fail")
                && (self.metadata.id.as_str().contains("primary")
                    || self.metadata.id.as_str().contains("flaky"))
            {
                return Err(ExpertError::ExecutionFailed(
                    "intentional failure".to_string(),
                ));
            }
            Ok(ExpertOutput {
                expert_id: self.metadata.id.clone(),
                content: format!("ok::{}", task.input()),
                confidence: 0.9,
                metadata: HashMap::new(),
                trace: Vec::new(),
            })
        }
    }

    #[test]
    fn v5_regression_tracking_detects_quality_drop() {
        let expert = ExpertId::new("v5-expert");

        let mut baseline = EvaluationEngine::new();
        baseline.record_expert_execution(expert.clone(), true, 0.9, 10.0);
        baseline.record_expert_execution(expert.clone(), true, 0.8, 11.0);
        baseline.record_routing(1, false);
        baseline.record_routing(2, false);

        let mut current = EvaluationEngine::new();
        current.record_expert_execution(expert.clone(), false, 0.2, 12.0);
        current.record_expert_execution(expert.clone(), true, 0.7, 11.0);
        current.record_routing(1, true);
        current.record_routing(1, true);

        let expert_regressions = current.detect_expert_regressions(&baseline, 0.2);
        assert_eq!(expert_regressions.len(), 1);
        assert_eq!(expert_regressions[0].expert_id.as_str(), expert.as_str());
        assert!(
            expert_regressions[0].previous_success_rate
                > expert_regressions[0].current_success_rate
        );

        let routing_regression = current
            .detect_routing_regression(&baseline, 0.2)
            .expect("routing regression should be present");
        assert!(routing_regression.delta < 0.0);
        assert!(routing_regression.previous_accuracy > routing_regression.current_accuracy);
    }

    #[test]
    fn v5_governance_report_blocks_promotion_when_thresholds_fail() {
        let mut engine = EvaluationEngine::new();
        let expert_bad = ExpertId::new("v5-bad");

        engine.record_expert_execution(expert_bad.clone(), false, 0.1, 20.0);
        engine.record_routing(1, true);

        let report = engine.governance_report(0.8, 0.9);
        assert!(!report.ready_for_promotion);
        assert!(report.routing_accuracy_below_threshold);
        assert_eq!(report.underperforming_experts, vec![expert_bad]);
        assert!((report.min_expert_success_rate - 0.8).abs() < f64::EPSILON);
        assert!((report.min_routing_accuracy - 0.9).abs() < f64::EPSILON);
    }

    #[test]
    fn v5_dataset_quality_report_captures_low_quality_and_corrections() {
        let mut store = DatasetStore::new();
        store.add_entry(dataset_entry(
            "good",
            "v5-expert",
            Outcome::Success,
            Some(0.9),
        ));
        store.add_entry(dataset_entry(
            "bad",
            "v5-expert",
            Outcome::Failure,
            Some(0.2),
        ));

        store.add_correction(Correction {
            entry_id: "bad".to_string(),
            corrected_output: "fixed-output".to_string(),
            reason: "human-feedback".to_string(),
            corrected_at: 2,
        });

        let report = store.quality_report(0.5);
        assert_eq!(report.total_entries, 2);
        assert_eq!(report.low_score_entries, 1);
        assert_eq!(report.corrected_entries, 1);
        assert!((report.success_ratio - 0.5).abs() < f64::EPSILON);
        assert!(report.average_score.expect("average score should exist") < 0.9);
    }

    #[test]
    fn v5_continuous_improvement_report_flags_regression_after_baseline() {
        let expert_id = ExpertId::new("v5-flaky");
        let router = SingleExpertRouter {
            expert_id: expert_id.clone(),
        };

        let mut pipeline = MoePipelineBuilder::new()
            .with_router(Box::new(router))
            .build();
        pipeline
            .register_expert(Box::new(V5FlakyExpert::new(expert_id.as_str())))
            .expect("expert registration should succeed");

        let warmup = Task::new("v5-warmup", TaskType::CodeGeneration, "clean");
        pipeline
            .execute(warmup)
            .expect("warmup execution should succeed");
        pipeline.capture_evaluation_baseline();

        let failing = Task::new("v5-failing", TaskType::CodeGeneration, "fail-now");
        let failure = pipeline.execute(failing);
        assert!(failure.is_err());

        let report: ContinuousImprovementReport =
            pipeline.continuous_improvement_report(0.8, 0.9, 0.5, 0.1);
        assert!(report.requires_human_review);
        assert!(!report.expert_regressions.is_empty());
        assert!(!report.governance.ready_for_promotion);
        assert_eq!(report.dataset_quality.total_entries, 1);
        assert!(report.routing_regression.is_none());
    }

    #[test]
    fn v5_continuous_governance_gate_can_block_outputs() {
        let primary = ExpertId::new("v5-primary-fail");
        let fallback = ExpertId::new("v5-fallback-ok");

        let mut scores = HashMap::new();
        scores.insert(primary.clone(), 1.0);
        scores.insert(fallback.clone(), 0.9);

        let fixed_router = LocalFixedRouter {
            selected: vec![primary.clone(), fallback.clone()],
            scores,
        };

        let policy = ContinuousGovernancePolicy::new(0.8, 0.9, 0.5, 0.1, true);

        let mut pipeline = MoePipelineBuilder::new()
            .with_router(Box::new(fixed_router))
            .with_fallback_on_expert_error(true)
            .with_continuous_governance_policy(policy)
            .build();

        let primary_expert = V5FlakyExpert::new(primary.as_str());
        let fallback_expert = V5FlakyExpert::new(fallback.as_str());
        pipeline
            .register_expert(Box::new(primary_expert))
            .expect("registering primary expert should succeed");
        pipeline
            .register_expert(Box::new(fallback_expert))
            .expect("registering fallback expert should succeed");

        // Primary fails via input marker, fallback succeeds because it does not
        // use the marker after fallback routing.
        let task = Task::new("v5-gate-block", TaskType::CodeGeneration, "fail-primary");
        let result = pipeline.execute(task);
        assert!(matches!(
            result,
            Err(crate::moe_core::MoeError::PolicyRejected(_))
        ));
        assert!(pipeline.last_continuous_improvement_report().is_some());
    }

    #[test]
    fn v5_continuous_governance_gate_non_blocking_keeps_output() {
        let policy = ContinuousGovernancePolicy::new(1.1, 0.99, 0.5, 0.1, false);
        let router = SingleExpertRouter {
            expert_id: ExpertId::new("v5-non-block"),
        };

        let mut pipeline = MoePipelineBuilder::new()
            .with_router(Box::new(router))
            .with_continuous_governance_policy(policy)
            .build();
        pipeline
            .register_expert(Box::new(V5FlakyExpert::new("v5-non-block")))
            .expect("expert registration should succeed");

        let task = Task::new("v5-gate-non-block", TaskType::CodeGeneration, "clean");
        let result = pipeline.execute(task);
        assert!(result.is_ok());
        assert!(pipeline.last_continuous_improvement_report().is_some());
        assert!(
            pipeline
                .last_continuous_improvement_report()
                .expect("report should exist")
                .requires_human_review
        );
    }

    #[test]
    fn v5_auto_promote_on_pass_captures_new_baseline() {
        let policy = ContinuousGovernancePolicy::new(0.1, 0.1, 0.5, 0.1, false)
            .with_auto_promote_on_pass(true);
        let router = SingleExpertRouter {
            expert_id: ExpertId::new("v5-auto-promote"),
        };

        let mut pipeline = MoePipelineBuilder::new()
            .with_router(Box::new(router))
            .with_continuous_governance_policy(policy)
            .build();
        pipeline
            .register_expert(Box::new(V5FlakyExpert::new("v5-auto-promote")))
            .expect("expert registration should succeed");

        assert!(!pipeline.has_evaluation_baseline());
        let task = Task::new("v5-auto-promote-task", TaskType::CodeGeneration, "clean");
        let result = pipeline.execute(task);
        assert!(result.is_ok());
        assert!(pipeline.has_evaluation_baseline());
    }

    #[test]
    fn v5_human_approval_hook_promotes_pending_review() {
        let policy = ContinuousGovernancePolicy::new(1.1, 0.99, 0.5, 0.1, true);
        let router = SingleExpertRouter {
            expert_id: ExpertId::new("v5-approval"),
        };

        let mut pipeline = MoePipelineBuilder::new()
            .with_router(Box::new(router))
            .with_continuous_governance_policy(policy)
            .build();
        pipeline
            .register_expert(Box::new(V5FlakyExpert::new("v5-approval")))
            .expect("expert registration should succeed");

        let task = Task::new("v5-approval-task", TaskType::CodeGeneration, "clean");
        let result = pipeline.execute(task);
        assert!(matches!(
            result,
            Err(crate::moe_core::MoeError::PolicyRejected(_))
        ));
        assert!(
            pipeline
                .last_continuous_improvement_report()
                .expect("report should exist")
                .requires_human_review
        );

        let approved = pipeline.approve_pending_human_review_and_promote();
        assert!(approved);
        assert!(pipeline.has_evaluation_baseline());
        assert!(
            !pipeline
                .last_continuous_improvement_report()
                .expect("report should exist")
                .requires_human_review
        );
    }

    #[test]
    fn v5_governance_state_roundtrip_supports_replay_between_pipelines() {
        let policy = ContinuousGovernancePolicy::new(1.1, 0.99, 0.5, 0.1, false);
        let router_a = SingleExpertRouter {
            expert_id: ExpertId::new("v5-state-a"),
        };

        let mut pipeline_a = MoePipelineBuilder::new()
            .with_router(Box::new(router_a))
            .with_continuous_governance_policy(policy)
            .build();
        pipeline_a
            .register_expert(Box::new(V5FlakyExpert::new("v5-state-a")))
            .expect("expert registration should succeed");

        let task = Task::new("v5-state-task-a", TaskType::CodeGeneration, "clean");
        let _ = pipeline_a.execute(task).expect("execution should succeed");
        assert!(pipeline_a.last_continuous_improvement_report().is_some());

        let state = pipeline_a.export_governance_state();
        assert!(state.verify_checksum());
        let state_json = pipeline_a
            .export_governance_state_json()
            .expect("state json export should succeed");

        let router_b = SingleExpertRouter {
            expert_id: ExpertId::new("v5-state-b"),
        };
        let mut pipeline_b = MoePipelineBuilder::new()
            .with_router(Box::new(router_b))
            .build();
        pipeline_b
            .register_expert(Box::new(V5FlakyExpert::new("v5-state-b")))
            .expect("expert registration should succeed");

        pipeline_b.import_governance_state(state);
        assert!(pipeline_b.last_continuous_improvement_report().is_some());
        assert!(!pipeline_b.governance_audit_trail().entries.is_empty());

        let mut pipeline_c = MoePipelineBuilder::new().build();
        pipeline_c
            .import_governance_state_json(&state_json)
            .expect("state json import should succeed");
        assert!(pipeline_c.last_continuous_improvement_report().is_some());
    }

    #[test]
    fn v5_governance_state_checksum_blocks_tampered_json_import() {
        let policy = ContinuousGovernancePolicy::new(1.1, 0.99, 0.5, 0.1, false);
        let router = SingleExpertRouter {
            expert_id: ExpertId::new("v5-checksum"),
        };

        let mut pipeline = MoePipelineBuilder::new()
            .with_router(Box::new(router))
            .with_continuous_governance_policy(policy)
            .build();
        pipeline
            .register_expert(Box::new(V5FlakyExpert::new("v5-checksum")))
            .expect("expert registration should succeed");
        let task = Task::new("v5-checksum-task", TaskType::CodeGeneration, "clean");
        let _ = pipeline.execute(task).expect("execution should succeed");

        let state_json = pipeline
            .export_governance_state_json()
            .expect("state json export should succeed");
        let mut parsed: GovernanceState =
            common_json::json::from_json_str(&state_json).expect("json parse should succeed");
        parsed.state_checksum = "deadbeef".to_string();
        let tampered = common_json::json::to_json_string_pretty(&parsed)
            .expect("tampered json serialization should succeed");

        let mut new_pipeline = MoePipelineBuilder::new().build();
        let result = new_pipeline.import_governance_state_json(&tampered);
        assert!(matches!(
            result,
            Err(crate::moe_core::MoeError::PolicyRejected(_))
        ));
    }

    #[test]
    fn v5_governance_audit_trail_respects_max_entries() {
        let policy = ContinuousGovernancePolicy::new(1.1, 0.99, 0.5, 0.1, false);
        let router = SingleExpertRouter {
            expert_id: ExpertId::new("v5-audit"),
        };

        let mut pipeline = MoePipelineBuilder::new()
            .with_router(Box::new(router))
            .with_continuous_governance_policy(policy)
            .with_max_governance_audit_entries(2)
            .build();
        pipeline
            .register_expert(Box::new(V5FlakyExpert::new("v5-audit")))
            .expect("expert registration should succeed");

        let task1 = Task::new("v5-audit-1", TaskType::CodeGeneration, "clean");
        let _ = pipeline.execute(task1).expect("execution should succeed");
        let task2 = Task::new("v5-audit-2", TaskType::CodeGeneration, "clean");
        let _ = pipeline.execute(task2).expect("execution should succeed");
        let task3 = Task::new("v5-audit-3", TaskType::CodeGeneration, "clean");
        let _ = pipeline.execute(task3).expect("execution should succeed");

        let trail = pipeline.governance_audit_trail();
        assert_eq!(trail.entries.len(), 2);
        assert!(trail.current_version >= 3);
    }

    #[test]
    fn v5_governance_state_diff_detects_policy_and_checksum_drift() {
        let policy_a = ContinuousGovernancePolicy::new(0.8, 0.9, 0.5, 0.1, false);
        let policy_b = ContinuousGovernancePolicy::new(0.9, 0.9, 0.5, 0.1, false);

        let pipeline = MoePipelineBuilder::new()
            .with_continuous_governance_policy(policy_a)
            .build();
        let mut target = GovernanceState::from_components(42, Some(policy_b), None, None);
        assert!(target.verify_checksum());

        let diff = pipeline.diff_governance_state(&target);
        assert!(diff.has_drift);
        assert!(diff.policy_changed);
        assert!(diff.checksum_changed);
        assert!(diff.version_delta != 0);

        target.state_checksum = "0000".to_string();
        let tampered_diff = pipeline.diff_governance_state(&target);
        assert!(tampered_diff.has_drift);
        assert!(tampered_diff.checksum_changed);
    }

    #[test]
    fn v5_import_policy_blocks_version_regression_from_json() {
        let policy = ContinuousGovernancePolicy::new(1.1, 0.99, 0.5, 0.1, false);
        let import_policy = GovernanceImportPolicy {
            allow_schema_change: false,
            allow_version_regression: false,
            max_version_regression: Some(0),
            require_policy_match: false,
        };

        let router = SingleExpertRouter {
            expert_id: ExpertId::new("v5-import-regression"),
        };
        let mut pipeline = MoePipelineBuilder::new()
            .with_router(Box::new(router))
            .with_continuous_governance_policy(policy)
            .with_governance_import_policy(import_policy)
            .build();
        pipeline
            .register_expert(Box::new(V5FlakyExpert::new("v5-import-regression")))
            .expect("expert registration should succeed");

        let _ = pipeline
            .execute(Task::new(
                "v5-import-regression-task",
                TaskType::CodeGeneration,
                "clean",
            ))
            .expect("execution should succeed");

        let mut state = pipeline.export_governance_state();
        state.state_version = 0;
        state.state_checksum = state.recompute_checksum();
        let payload = common_json::json::to_json_string_pretty(&state)
            .expect("state serialization should succeed");

        let result = pipeline.import_governance_state_json(&payload);
        assert!(matches!(
            result,
            Err(crate::moe_core::MoeError::PolicyRejected(_))
        ));
    }

    #[test]
    fn v5_import_policy_can_require_policy_match() {
        let policy_a = ContinuousGovernancePolicy::new(0.8, 0.9, 0.5, 0.1, false);
        let policy_b = ContinuousGovernancePolicy::new(0.9, 0.9, 0.5, 0.1, false);
        let import_policy = GovernanceImportPolicy {
            allow_schema_change: false,
            allow_version_regression: true,
            max_version_regression: None,
            require_policy_match: true,
        };

        let mut pipeline = MoePipelineBuilder::new()
            .with_continuous_governance_policy(policy_a)
            .with_governance_import_policy(import_policy)
            .build();
        let incoming = GovernanceState::from_components(1, Some(policy_b), None, None);
        let payload = common_json::json::to_json_string_pretty(&incoming)
            .expect("state serialization should succeed");

        let result = pipeline.import_governance_state_json(&payload);
        assert!(matches!(
            result,
            Err(crate::moe_core::MoeError::PolicyRejected(_))
        ));
    }

    #[test]
    fn v5_import_preview_reports_rejection_reasons() {
        let current_policy = ContinuousGovernancePolicy::new(0.8, 0.9, 0.5, 0.1, false);
        let incoming_policy = ContinuousGovernancePolicy::new(0.9, 0.9, 0.5, 0.1, false);

        let import_policy = GovernanceImportPolicy {
            allow_schema_change: false,
            allow_version_regression: false,
            max_version_regression: Some(0),
            require_policy_match: true,
        };

        let pipeline = MoePipelineBuilder::new()
            .with_continuous_governance_policy(current_policy)
            .with_governance_import_policy(import_policy)
            .build();

        let incoming = GovernanceState::from_components(0, Some(incoming_policy), None, None);
        let payload = common_json::json::to_json_string_pretty(&incoming)
            .expect("state serialization should succeed");
        let decision = pipeline
            .preview_governance_import_json(&payload)
            .expect("preview should succeed");

        assert!(!decision.allowed);
        assert!(decision.diff.policy_changed);
        assert!(!decision.reasons.is_empty());
    }

    #[test]
    fn v5_try_import_json_returns_explicit_policy_rejection_message() {
        let policy = ContinuousGovernancePolicy::new(0.8, 0.9, 0.5, 0.1, false);
        let import_policy = GovernanceImportPolicy {
            allow_schema_change: false,
            allow_version_regression: false,
            max_version_regression: Some(0),
            require_policy_match: false,
        };

        let mut pipeline = MoePipelineBuilder::new()
            .with_continuous_governance_policy(policy)
            .with_governance_import_policy(import_policy)
            .build();

        // Move local governance version forward so importing version 0 is a regression.
        pipeline
            .register_expert(Box::new(V5FlakyExpert::new("v5-import-explicit")))
            .expect("expert registration should succeed");
        let _ = pipeline
            .execute(Task::new(
                "v5-import-explicit-task",
                TaskType::CodeGeneration,
                "clean",
            ))
            .expect("execution should succeed");

        let incoming = GovernanceState::from_components(0, None, None, None);
        let payload = common_json::json::to_json_string_pretty(&incoming)
            .expect("state serialization should succeed");
        let result = pipeline.try_import_governance_state_json(&payload);

        assert!(matches!(
            result,
            Err(crate::moe_core::MoeError::PolicyRejected(_))
        ));
        if let Err(crate::moe_core::MoeError::PolicyRejected(message)) = result {
            assert!(message.contains("governance import rejected"));
        }
    }

    #[test]
    fn v5_bundle_import_preview_reports_rejection_reasons() {
        let current_policy = ContinuousGovernancePolicy::new(0.8, 0.9, 0.5, 0.1, false);
        let incoming_policy = ContinuousGovernancePolicy::new(0.9, 0.9, 0.5, 0.1, false);
        let import_policy = GovernanceImportPolicy {
            allow_schema_change: false,
            allow_version_regression: false,
            max_version_regression: Some(0),
            require_policy_match: true,
        };

        let pipeline = MoePipelineBuilder::new()
            .with_continuous_governance_policy(current_policy)
            .with_governance_import_policy(import_policy)
            .build();

        let incoming = GovernanceState::from_components(0, Some(incoming_policy), None, None);
        let bundle = GovernancePersistenceBundle {
            state: incoming,
            audit_entries: Vec::new(),
            snapshots: Vec::new(),
        };
        let payload = common_json::json::to_json_string_pretty(&bundle)
            .expect("bundle serialization should succeed");

        let decision = pipeline
            .preview_governance_bundle_import_json(&payload)
            .expect("bundle preview should succeed");
        assert!(!decision.allowed);
        assert!(decision.diff.policy_changed);
        assert!(!decision.reasons.is_empty());
    }

    #[test]
    fn v5_try_bundle_import_json_returns_explicit_policy_rejection_message() {
        let policy = ContinuousGovernancePolicy::new(0.8, 0.9, 0.5, 0.1, false);
        let import_policy = GovernanceImportPolicy {
            allow_schema_change: false,
            allow_version_regression: false,
            max_version_regression: Some(0),
            require_policy_match: false,
        };

        let mut pipeline = MoePipelineBuilder::new()
            .with_continuous_governance_policy(policy)
            .with_governance_import_policy(import_policy)
            .build();

        // Move local governance version forward so importing version 0 is a regression.
        pipeline
            .register_expert(Box::new(V5FlakyExpert::new("v5-bundle-import-explicit")))
            .expect("expert registration should succeed");
        let _ = pipeline
            .execute(Task::new(
                "v5-bundle-import-explicit-task",
                TaskType::CodeGeneration,
                "clean",
            ))
            .expect("execution should succeed");

        let incoming = GovernancePersistenceBundle {
            state: GovernanceState::from_components(0, None, None, None),
            audit_entries: Vec::new(),
            snapshots: Vec::new(),
        };
        let payload = common_json::json::to_json_string_pretty(&incoming)
            .expect("bundle serialization should succeed");
        let result = pipeline.try_import_governance_bundle_json(&payload);

        assert!(matches!(
            result,
            Err(crate::moe_core::MoeError::PolicyRejected(_))
        ));
        if let Err(crate::moe_core::MoeError::PolicyRejected(message)) = result {
            assert!(message.contains("governance bundle import rejected"));
        }
    }

    #[test]
    fn v5_runtime_bundle_import_preview_reports_rejection_reasons() {
        let policy = ContinuousGovernancePolicy::new(0.8, 0.9, 0.5, 0.1, false);
        let import_policy = GovernanceImportPolicy {
            allow_schema_change: false,
            allow_version_regression: false,
            max_version_regression: Some(0),
            require_policy_match: false,
        };

        let mut pipeline = MoePipelineBuilder::new()
            .with_continuous_governance_policy(policy)
            .with_governance_import_policy(import_policy)
            .build();
        pipeline
            .register_expert(Box::new(V5FlakyExpert::new("v5-runtime-preview-explicit")))
            .expect("expert registration should succeed");
        let _ = pipeline
            .execute(Task::new(
                "v5-runtime-preview-explicit-task",
                TaskType::CodeGeneration,
                "clean",
            ))
            .expect("execution should succeed");

        let source = MoePipelineBuilder::new().build();
        let incoming = source.export_runtime_bundle();
        let payload = common_json::json::to_json_string_pretty(&incoming)
            .expect("runtime bundle serialization should succeed");

        let decision = pipeline
            .preview_runtime_bundle_import_json(&payload)
            .expect("runtime bundle preview should succeed");
        assert!(!decision.allowed);
        assert!(decision.diff.version_delta < 0);
        assert!(!decision.reasons.is_empty());
    }

    #[test]
    fn v5_try_runtime_bundle_import_json_returns_explicit_policy_rejection_message() {
        let policy = ContinuousGovernancePolicy::new(0.8, 0.9, 0.5, 0.1, false);
        let import_policy = GovernanceImportPolicy {
            allow_schema_change: false,
            allow_version_regression: false,
            max_version_regression: Some(0),
            require_policy_match: false,
        };

        let mut pipeline = MoePipelineBuilder::new()
            .with_continuous_governance_policy(policy)
            .with_governance_import_policy(import_policy)
            .build();
        pipeline
            .register_expert(Box::new(V5FlakyExpert::new("v5-runtime-import-explicit")))
            .expect("expert registration should succeed");
        let _ = pipeline
            .execute(Task::new(
                "v5-runtime-import-explicit-task",
                TaskType::CodeGeneration,
                "clean",
            ))
            .expect("execution should succeed");

        let source = MoePipelineBuilder::new().build();
        let incoming = source.export_runtime_bundle();
        let payload = common_json::json::to_json_string_pretty(&incoming)
            .expect("runtime bundle serialization should succeed");
        let result = pipeline.try_import_runtime_bundle_json(&payload);

        assert!(matches!(
            result,
            Err(crate::moe_core::MoeError::PolicyRejected(_))
        ));
        if let Err(crate::moe_core::MoeError::PolicyRejected(message)) = result {
            assert!(message.contains("runtime bundle import rejected"));
        }
    }

    #[test]
    fn v5_runtime_bundle_json_import_rejects_tampered_checksum() {
        let mut source = MoePipelineBuilder::new().build();
        source
            .remember_short_term(memory_entry(
                "v5-runtime-checksum-stm",
                "runtime payload baseline",
                MemoryType::Short,
                1,
            ))
            .expect("short-term memory write should succeed");

        let payload = source
            .export_runtime_bundle_json()
            .expect("runtime bundle json export should succeed");
        let mut parsed: RuntimePersistenceBundle = common_json::json::from_json_str(&payload)
            .expect("runtime bundle parse should succeed");
        parsed.short_term_memory_entries[0].content = "runtime payload tampered".to_string();
        let tampered_payload = common_json::json::to_json_string_pretty(&parsed)
            .expect("tampered runtime bundle serialization should succeed");

        let mut target = MoePipelineBuilder::new().build();
        let result = target.import_runtime_bundle_json(&tampered_payload);
        assert!(matches!(
            result,
            Err(crate::moe_core::MoeError::PolicyRejected(_))
        ));
    }

    #[test]
    fn v5_runtime_bundle_preview_rejects_unsupported_schema() {
        let source = MoePipelineBuilder::new().build();
        let mut bundle = source.export_runtime_bundle();
        bundle.schema_version = RuntimePersistenceBundle::schema_version() + 1;
        let payload = common_json::json::to_json_string_pretty(&bundle)
            .expect("runtime bundle serialization should succeed");

        let target = MoePipelineBuilder::new().build();
        let result = target.preview_runtime_bundle_import_json(&payload);
        assert!(matches!(
            result,
            Err(crate::moe_core::MoeError::PolicyRejected(_))
        ));
    }

    #[test]
    fn v5_rollback_restores_previous_governance_snapshot() {
        let policy = ContinuousGovernancePolicy::new(1.1, 0.99, 0.5, 0.1, false);
        let router = SingleExpertRouter {
            expert_id: ExpertId::new("v5-rollback"),
        };

        let mut pipeline = MoePipelineBuilder::new()
            .with_router(Box::new(router))
            .with_continuous_governance_policy(policy)
            .with_max_governance_state_snapshots(8)
            .build();
        pipeline
            .register_expert(Box::new(V5FlakyExpert::new("v5-rollback")))
            .expect("expert registration should succeed");

        let _ = pipeline
            .execute(Task::new(
                "v5-rollback-1",
                TaskType::CodeGeneration,
                "clean",
            ))
            .expect("first execution should succeed");
        let first_snapshot_version = pipeline
            .governance_state_snapshots()
            .last()
            .expect("snapshot should exist")
            .version;

        let _ = pipeline
            .execute(Task::new(
                "v5-rollback-2",
                TaskType::CodeGeneration,
                "clean",
            ))
            .expect("second execution should succeed");

        pipeline
            .rollback_governance_state_to_version(first_snapshot_version)
            .expect("rollback should succeed");
        let last_reason = pipeline
            .governance_audit_trail()
            .entries
            .last()
            .expect("audit entry should exist after rollback")
            .reason
            .clone();
        assert!(last_reason.contains("rollback"));
    }

    #[test]
    fn v5_governance_snapshot_limit_is_enforced() {
        let policy = ContinuousGovernancePolicy::new(1.1, 0.99, 0.5, 0.1, false);
        let router = SingleExpertRouter {
            expert_id: ExpertId::new("v5-snapshot-limit"),
        };

        let mut pipeline = MoePipelineBuilder::new()
            .with_router(Box::new(router))
            .with_continuous_governance_policy(policy)
            .with_max_governance_state_snapshots(2)
            .build();
        pipeline
            .register_expert(Box::new(V5FlakyExpert::new("v5-snapshot-limit")))
            .expect("expert registration should succeed");

        for idx in 0..4 {
            let _ = pipeline
                .execute(Task::new(
                    format!("v5-snapshot-limit-{idx}"),
                    TaskType::CodeGeneration,
                    "clean",
                ))
                .expect("execution should succeed");
        }

        assert_eq!(pipeline.governance_state_snapshots().len(), 2);
    }

    #[test]
    fn v5_governance_retention_keeps_snapshots_aligned_with_audit_versions() {
        let policy = ContinuousGovernancePolicy::new(1.1, 0.99, 0.5, 0.1, false);
        let router = SingleExpertRouter {
            expert_id: ExpertId::new("v5-retention-align"),
        };

        let mut pipeline = MoePipelineBuilder::new()
            .with_router(Box::new(router))
            .with_continuous_governance_policy(policy)
            .with_max_governance_audit_entries(1)
            .with_max_governance_state_snapshots(8)
            .build();
        pipeline
            .register_expert(Box::new(V5FlakyExpert::new("v5-retention-align")))
            .expect("expert registration should succeed");

        let _ = pipeline
            .execute(Task::new(
                "v5-retention-align-1",
                TaskType::CodeGeneration,
                "clean",
            ))
            .expect("first execution should succeed");
        let _ = pipeline
            .execute(Task::new(
                "v5-retention-align-2",
                TaskType::CodeGeneration,
                "clean",
            ))
            .expect("second execution should succeed");

        let trail = pipeline.governance_audit_trail();
        assert_eq!(trail.entries.len(), 1);
        assert_eq!(pipeline.governance_state_snapshots().len(), 1);
        assert_eq!(
            pipeline.governance_state_snapshots()[0].version,
            trail.entries[0].version
        );
    }

    #[test]
    fn v5_governance_bundle_roundtrip_restores_audit_and_snapshots() {
        let policy = ContinuousGovernancePolicy::new(1.1, 0.99, 0.5, 0.1, false);
        let router = SingleExpertRouter {
            expert_id: ExpertId::new("v5-bundle"),
        };

        let mut source = MoePipelineBuilder::new()
            .with_router(Box::new(router))
            .with_continuous_governance_policy(policy)
            .build();
        source
            .register_expert(Box::new(V5FlakyExpert::new("v5-bundle")))
            .expect("expert registration should succeed");
        let _ = source
            .execute(Task::new(
                "v5-bundle-task",
                TaskType::CodeGeneration,
                "clean",
            ))
            .expect("execution should succeed");

        let bundle = source.export_governance_bundle();
        let bundle_json = source
            .export_governance_bundle_json()
            .expect("bundle json export should succeed");
        assert!(!bundle.audit_entries.is_empty());
        assert!(!bundle.snapshots.is_empty());

        let mut target = MoePipelineBuilder::new().build();
        target
            .import_governance_bundle(bundle.clone())
            .expect("bundle import should succeed");
        assert_eq!(
            target.export_governance_state().state_version,
            bundle.state.state_version
        );
        assert_eq!(
            target.governance_audit_trail().current_version,
            bundle.state.state_version
        );
        assert_eq!(
            target.governance_audit_trail().entries.len(),
            bundle.audit_entries.len()
        );
        assert_eq!(
            target.governance_state_snapshots().len(),
            bundle.snapshots.len()
        );

        let mut target_json = MoePipelineBuilder::new().build();
        target_json
            .import_governance_bundle_json(&bundle_json)
            .expect("bundle json import should succeed");
        assert_eq!(
            target_json.export_governance_state().state_version,
            bundle.state.state_version
        );
        assert_eq!(
            target_json.governance_audit_trail().current_version,
            bundle.state.state_version
        );
        assert_eq!(
            target_json.governance_audit_trail().entries.len(),
            bundle.audit_entries.len()
        );
    }

    #[test]
    fn v5_runtime_bundle_roundtrip_restores_memory_and_buffers() {
        let policy = ContinuousGovernancePolicy::new(1.1, 0.99, 0.5, 0.1, false);
        let router = SingleExpertRouter {
            expert_id: ExpertId::new("v5-runtime-bundle"),
        };

        let mut source = MoePipelineBuilder::new()
            .with_router(Box::new(router))
            .with_continuous_governance_policy(policy)
            .build();
        source
            .register_expert(Box::new(V5FlakyExpert::new("v5-runtime-bundle")))
            .expect("expert registration should succeed");
        source
            .remember_short_term(memory_entry(
                "stm-1",
                "recent runtime memory",
                MemoryType::Short,
                1,
            ))
            .expect("short-term memory write should succeed");
        source
            .remember_long_term(memory_entry(
                "ltm-1",
                "archival runtime memory",
                MemoryType::Long,
                2,
            ))
            .expect("long-term memory write should succeed");
        let _ = source
            .execute(Task::new(
                "v5-runtime-bundle-task",
                TaskType::CodeGeneration,
                "clean",
            ))
            .expect("execution should succeed");

        let runtime_bundle = source.export_runtime_bundle();
        let runtime_bundle_json = source
            .export_runtime_bundle_json()
            .expect("runtime bundle json export should succeed");
        assert!(!runtime_bundle.short_term_memory_entries.is_empty());
        assert!(!runtime_bundle.long_term_memory_entries.is_empty());
        assert!(runtime_bundle.buffer_manager.working().count() > 0);

        let mut target = MoePipelineBuilder::new().build();
        target
            .import_runtime_bundle(runtime_bundle.clone())
            .expect("runtime bundle import should succeed");
        let restored = target.export_runtime_bundle();
        assert_eq!(
            restored.short_term_memory_entries.len(),
            runtime_bundle.short_term_memory_entries.len()
        );
        assert_eq!(
            restored.long_term_memory_entries.len(),
            runtime_bundle.long_term_memory_entries.len()
        );
        assert_eq!(
            restored.buffer_manager.working().count(),
            runtime_bundle.buffer_manager.working().count()
        );

        let mut target_json = MoePipelineBuilder::new().build();
        target_json
            .import_runtime_bundle_json(&runtime_bundle_json)
            .expect("runtime bundle json import should succeed");
        let restored_json = target_json.export_runtime_bundle();
        assert_eq!(
            restored_json.short_term_memory_entries.len(),
            runtime_bundle.short_term_memory_entries.len()
        );
        assert_eq!(
            restored_json.long_term_memory_entries.len(),
            runtime_bundle.long_term_memory_entries.len()
        );
        assert_eq!(
            restored_json.buffer_manager.working().count(),
            runtime_bundle.buffer_manager.working().count()
        );
    }

    #[test]
    fn v5_governance_bundle_rejects_tampered_snapshot_checksums() {
        let policy = ContinuousGovernancePolicy::new(1.1, 0.99, 0.5, 0.1, false);
        let router = SingleExpertRouter {
            expert_id: ExpertId::new("v5-bundle-tamper"),
        };

        let mut source = MoePipelineBuilder::new()
            .with_router(Box::new(router))
            .with_continuous_governance_policy(policy)
            .build();
        source
            .register_expert(Box::new(V5FlakyExpert::new("v5-bundle-tamper")))
            .expect("expert registration should succeed");
        let _ = source
            .execute(Task::new(
                "v5-bundle-tamper-task",
                TaskType::CodeGeneration,
                "clean",
            ))
            .expect("execution should succeed");

        let bundle_json = source
            .export_governance_bundle_json()
            .expect("bundle json export should succeed");
        let mut parsed: GovernancePersistenceBundle =
            common_json::json::from_json_str(&bundle_json).expect("bundle parse should succeed");
        parsed.snapshots[0].state.state_checksum = "tampered".to_string();
        let tampered_json = common_json::json::to_json_string_pretty(&parsed)
            .expect("tampered bundle serialization should succeed");

        let mut target = MoePipelineBuilder::new().build();
        let result = target.import_governance_bundle_json(&tampered_json);
        assert!(matches!(
            result,
            Err(crate::moe_core::MoeError::PolicyRejected(_))
        ));
    }

    #[test]
    fn v5_governance_bundle_rejects_mismatched_latest_audit_version() {
        let policy = ContinuousGovernancePolicy::new(1.1, 0.99, 0.5, 0.1, false);
        let router = SingleExpertRouter {
            expert_id: ExpertId::new("v5-bundle-audit-mismatch"),
        };

        let mut source = MoePipelineBuilder::new()
            .with_router(Box::new(router))
            .with_continuous_governance_policy(policy)
            .build();
        source
            .register_expert(Box::new(V5FlakyExpert::new("v5-bundle-audit-mismatch")))
            .expect("expert registration should succeed");
        let _ = source
            .execute(Task::new(
                "v5-bundle-audit-mismatch-task",
                TaskType::CodeGeneration,
                "clean",
            ))
            .expect("execution should succeed");

        let bundle_json = source
            .export_governance_bundle_json()
            .expect("bundle json export should succeed");
        let mut parsed: GovernancePersistenceBundle =
            common_json::json::from_json_str(&bundle_json).expect("bundle parse should succeed");
        let last = parsed
            .audit_entries
            .last_mut()
            .expect("audit entries should not be empty");
        last.version = last.version.saturating_add(1);

        let tampered_json = common_json::json::to_json_string_pretty(&parsed)
            .expect("tampered bundle serialization should succeed");
        let mut target = MoePipelineBuilder::new().build();
        let result = target.import_governance_bundle_json(&tampered_json);
        assert!(matches!(
            result,
            Err(crate::moe_core::MoeError::PolicyRejected(_))
        ));
    }

    #[test]
    fn v5_governance_bundle_rejects_snapshot_version_state_mismatch() {
        let policy = ContinuousGovernancePolicy::new(1.1, 0.99, 0.5, 0.1, false);
        let router = SingleExpertRouter {
            expert_id: ExpertId::new("v5-bundle-snapshot-version-mismatch"),
        };

        let mut source = MoePipelineBuilder::new()
            .with_router(Box::new(router))
            .with_continuous_governance_policy(policy)
            .build();
        source
            .register_expert(Box::new(V5FlakyExpert::new(
                "v5-bundle-snapshot-version-mismatch",
            )))
            .expect("expert registration should succeed");
        let _ = source
            .execute(Task::new(
                "v5-bundle-snapshot-version-mismatch-task",
                TaskType::CodeGeneration,
                "clean",
            ))
            .expect("execution should succeed");

        let bundle_json = source
            .export_governance_bundle_json()
            .expect("bundle json export should succeed");
        let mut parsed: GovernancePersistenceBundle =
            common_json::json::from_json_str(&bundle_json).expect("bundle parse should succeed");
        parsed.snapshots[0].version = parsed.snapshots[0].version.saturating_add(1);

        let tampered_json = common_json::json::to_json_string_pretty(&parsed)
            .expect("tampered bundle serialization should succeed");
        let mut target = MoePipelineBuilder::new().build();
        let result = target.import_governance_bundle_json(&tampered_json);
        assert!(matches!(
            result,
            Err(crate::moe_core::MoeError::PolicyRejected(_))
        ));
    }

    #[test]
    fn v5_governance_bundle_rejects_non_monotonic_audit_versions() {
        let policy = ContinuousGovernancePolicy::new(1.1, 0.99, 0.5, 0.1, false);
        let router = SingleExpertRouter {
            expert_id: ExpertId::new("v5-bundle-audit-order"),
        };

        let mut source = MoePipelineBuilder::new()
            .with_router(Box::new(router))
            .with_continuous_governance_policy(policy)
            .build();
        source
            .register_expert(Box::new(V5FlakyExpert::new("v5-bundle-audit-order")))
            .expect("expert registration should succeed");

        let _ = source
            .execute(Task::new(
                "v5-bundle-audit-order-1",
                TaskType::CodeGeneration,
                "clean",
            ))
            .expect("first execution should succeed");
        let _ = source
            .execute(Task::new(
                "v5-bundle-audit-order-2",
                TaskType::CodeGeneration,
                "clean",
            ))
            .expect("second execution should succeed");

        let bundle_json = source
            .export_governance_bundle_json()
            .expect("bundle json export should succeed");
        let mut parsed: GovernancePersistenceBundle =
            common_json::json::from_json_str(&bundle_json).expect("bundle parse should succeed");
        parsed.audit_entries.swap(0, 1);

        let tampered_json = common_json::json::to_json_string_pretty(&parsed)
            .expect("tampered bundle serialization should succeed");
        let mut target = MoePipelineBuilder::new().build();
        let result = target.import_governance_bundle_json(&tampered_json);
        assert!(matches!(
            result,
            Err(crate::moe_core::MoeError::PolicyRejected(_))
        ));
    }

    #[test]
    fn v5_governance_bundle_rejects_audit_snapshot_checksum_divergence_for_same_version() {
        let policy = ContinuousGovernancePolicy::new(1.1, 0.99, 0.5, 0.1, false);
        let router = SingleExpertRouter {
            expert_id: ExpertId::new("v5-bundle-checksum-divergence"),
        };

        let mut source = MoePipelineBuilder::new()
            .with_router(Box::new(router))
            .with_continuous_governance_policy(policy)
            .build();
        source
            .register_expert(Box::new(V5FlakyExpert::new(
                "v5-bundle-checksum-divergence",
            )))
            .expect("expert registration should succeed");
        let _ = source
            .execute(Task::new(
                "v5-bundle-checksum-divergence-task",
                TaskType::CodeGeneration,
                "clean",
            ))
            .expect("execution should succeed");

        let bundle_json = source
            .export_governance_bundle_json()
            .expect("bundle json export should succeed");
        let mut parsed: GovernancePersistenceBundle =
            common_json::json::from_json_str(&bundle_json).expect("bundle parse should succeed");

        let version = parsed
            .snapshots
            .first()
            .expect("snapshot should be present")
            .version;
        let audit = parsed
            .audit_entries
            .iter_mut()
            .find(|entry| entry.version == version)
            .expect("matching audit version should exist");
        audit.checksum = "audit-diverged".to_string();

        let tampered_json = common_json::json::to_json_string_pretty(&parsed)
            .expect("tampered bundle serialization should succeed");
        let mut target = MoePipelineBuilder::new().build();
        let result = target.import_governance_bundle_json(&tampered_json);
        assert!(matches!(
            result,
            Err(crate::moe_core::MoeError::PolicyRejected(_))
        ));
    }

    #[test]
    fn v5_governance_bundle_rejects_snapshot_without_matching_audit_entry() {
        let policy = ContinuousGovernancePolicy::new(1.1, 0.99, 0.5, 0.1, false);
        let router = SingleExpertRouter {
            expert_id: ExpertId::new("v5-bundle-orphan-snapshot"),
        };

        let mut source = MoePipelineBuilder::new()
            .with_router(Box::new(router))
            .with_continuous_governance_policy(policy)
            .build();
        source
            .register_expert(Box::new(V5FlakyExpert::new("v5-bundle-orphan-snapshot")))
            .expect("expert registration should succeed");
        let _ = source
            .execute(Task::new(
                "v5-bundle-orphan-snapshot-task",
                TaskType::CodeGeneration,
                "clean",
            ))
            .expect("execution should succeed");

        let bundle_json = source
            .export_governance_bundle_json()
            .expect("bundle json export should succeed");
        let mut parsed: GovernancePersistenceBundle =
            common_json::json::from_json_str(&bundle_json).expect("bundle parse should succeed");

        let snapshot_version = parsed
            .snapshots
            .first()
            .expect("snapshot should exist")
            .version;
        parsed
            .audit_entries
            .retain(|entry| entry.version != snapshot_version);

        let tampered_json = common_json::json::to_json_string_pretty(&parsed)
            .expect("tampered bundle serialization should succeed");
        let mut target = MoePipelineBuilder::new().build();
        let result = target.import_governance_bundle_json(&tampered_json);
        assert!(matches!(
            result,
            Err(crate::moe_core::MoeError::PolicyRejected(_))
        ));
    }

    #[test]
    fn v5_governance_bundle_restore_supports_rollback_from_imported_snapshots() {
        let policy = ContinuousGovernancePolicy::new(1.1, 0.99, 0.5, 0.1, false);
        let router = SingleExpertRouter {
            expert_id: ExpertId::new("v5-bundle-rollback-imported"),
        };

        let mut source = MoePipelineBuilder::new()
            .with_router(Box::new(router))
            .with_continuous_governance_policy(policy)
            .with_max_governance_state_snapshots(8)
            .build();
        source
            .register_expert(Box::new(V5FlakyExpert::new("v5-bundle-rollback-imported")))
            .expect("expert registration should succeed");

        let _ = source
            .execute(Task::new(
                "v5-bundle-rollback-imported-1",
                TaskType::CodeGeneration,
                "clean",
            ))
            .expect("first execution should succeed");
        let _ = source
            .execute(Task::new(
                "v5-bundle-rollback-imported-2",
                TaskType::CodeGeneration,
                "clean",
            ))
            .expect("second execution should succeed");

        let bundle = source.export_governance_bundle();
        assert!(
            bundle.snapshots.len() >= 2,
            "source should carry at least two snapshots"
        );
        let rollback_target_version = bundle
            .snapshots
            .first()
            .expect("snapshot should exist")
            .version;

        let mut target = MoePipelineBuilder::new()
            .with_max_governance_state_snapshots(8)
            .build();
        target
            .import_governance_bundle(bundle)
            .expect("bundle import should succeed");

        let before_len = target.governance_audit_trail().entries.len();
        target
            .rollback_governance_state_to_version(rollback_target_version)
            .expect("rollback on imported snapshot should succeed");
        let after = target.governance_audit_trail();
        assert_eq!(after.entries.len(), before_len + 1);
        assert!(
            after
                .entries
                .last()
                .expect("audit entry should exist after rollback")
                .reason
                .contains("rollback"),
            "rollback reason should be recorded after restore"
        );
    }

    #[test]
    fn v5_governance_bundle_import_retention_keeps_snapshots_aligned_with_audit_versions() {
        let policy = ContinuousGovernancePolicy::new(1.1, 0.99, 0.5, 0.1, false);
        let router = SingleExpertRouter {
            expert_id: ExpertId::new("v5-import-retention-align"),
        };

        let mut source = MoePipelineBuilder::new()
            .with_router(Box::new(router))
            .with_continuous_governance_policy(policy)
            .with_max_governance_audit_entries(8)
            .with_max_governance_state_snapshots(8)
            .build();
        source
            .register_expert(Box::new(V5FlakyExpert::new("v5-import-retention-align")))
            .expect("expert registration should succeed");
        let _ = source
            .execute(Task::new(
                "v5-import-retention-align-1",
                TaskType::CodeGeneration,
                "clean",
            ))
            .expect("first execution should succeed");
        let _ = source
            .execute(Task::new(
                "v5-import-retention-align-2",
                TaskType::CodeGeneration,
                "clean",
            ))
            .expect("second execution should succeed");

        let bundle = source.export_governance_bundle();
        assert!(bundle.audit_entries.len() >= 2);
        assert!(bundle.snapshots.len() >= 2);

        let mut target = MoePipelineBuilder::new()
            .with_max_governance_audit_entries(1)
            .with_max_governance_state_snapshots(8)
            .build();
        target
            .import_governance_bundle(bundle)
            .expect("bundle import should succeed");

        let trail = target.governance_audit_trail();
        assert_eq!(trail.entries.len(), 1);
        assert_eq!(target.governance_state_snapshots().len(), 1);
        assert_eq!(
            target.governance_state_snapshots()[0].version,
            trail.entries[0].version
        );
    }
}
