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
