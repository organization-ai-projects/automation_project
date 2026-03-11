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
}

mod v5 {
    use crate::dataset_engine::{Correction, DatasetEntry, DatasetStore, Outcome};
    use crate::evaluation_engine::EvaluationEngine;
    use crate::expert_registry::ExpertRegistry;
    use crate::moe_core::{
        ExecutionContext, Expert, ExpertCapability, ExpertError, ExpertId, ExpertMetadata,
        ExpertOutput, ExpertStatus, ExpertType, Task, TaskId, TaskType,
    };
    use crate::orchestrator::{ContinuousImprovementReport, MoePipelineBuilder};
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
            if task.input().contains("fail") {
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
}
