//! projects/products/unstable/neurosymbolic_moe/backend/src/tests/auto_improvement.rs
use std::collections::HashMap;

use crate::aggregator::AggregationStrategy;
use crate::dataset_engine::{
    DatasetTrainingBuildOptions, DatasetTrainingBundle, DatasetTrainingProvenance,
    DatasetTrainingSample,
};
use crate::echo_expert::EchoExpert;
use crate::moe_core::{ExpertCapability, Task, TaskPriority, TaskType};
use crate::orchestrator::{AutoImprovementPolicy, MoePipelineBuilder};
use crate::router::HeuristicRouter;

#[test]
fn bootstrap_initial_dataset_from_training_bundle_json_seeds_entries() {
    let mut pipeline = MoePipelineBuilder::new().build();
    let sample = DatasetTrainingSample {
        entry_id: "seed-1".to_string(),
        task_id: "task-seed-1".to_string(),
        expert_id: "expert-seed-1".to_string(),
        input: "seed input".to_string(),
        target_output: "seed output".to_string(),
        source_output: "seed source".to_string(),
        used_correction: false,
        correction_reason: None,
        score: Some(0.9),
        tags: vec!["seed".to_string()],
        metadata: HashMap::new(),
    };
    let mut bundle = DatasetTrainingBundle {
        schema_version: DatasetTrainingBundle::schema_version(),
        bundle_checksum: String::new(),
        generated_at: 42,
        validation_ratio: 0.0,
        split_seed: 7,
        total_entries: 1,
        included_entries: 1,
        filtered_low_score: 0,
        filtered_outcome: 0,
        filtered_missing_failure_correction: 0,
        provenance: DatasetTrainingProvenance::default(),
        train_samples: vec![sample],
        validation_samples: Vec::new(),
    };
    bundle.ensure_checksum();
    let payload = common_json::json::to_json_string_pretty(&bundle).expect("serialize bundle");

    let seeded = pipeline
        .bootstrap_initial_dataset_from_training_bundle_json(&payload)
        .expect("bootstrap should succeed");
    let second_seed = pipeline
        .bootstrap_initial_dataset_from_training_bundle_json(&payload)
        .expect("bootstrap replay should succeed");

    assert_eq!(seeded, 1);
    assert_eq!(second_seed, 0);
    assert_eq!(pipeline.dataset_store().count(), 1);
    assert_eq!(
        pipeline.auto_improvement_status().bootstrap_entries_total,
        1
    );
}

#[test]
fn execute_triggers_auto_improvement_when_policy_thresholds_are_met() {
    let policy = AutoImprovementPolicy::default()
        .with_min_dataset_entries(1)
        .with_min_success_ratio(0.0)
        .with_min_average_score(None)
        .with_training_build_options(DatasetTrainingBuildOptions {
            generated_at: 1,
            validation_ratio: 0.2,
            min_score: None,
            include_failure_entries: true,
            include_partial_entries: true,
            include_unknown_entries: false,
            require_correction_for_failure: false,
            split_seed: 11,
        });
    let mut pipeline = MoePipelineBuilder::new()
        .with_router(Box::new(HeuristicRouter::new(2)))
        .with_aggregation_strategy(AggregationStrategy::HighestConfidence)
        .with_auto_improvement_policy(policy)
        .build();
    pipeline
        .register_expert(Box::new(EchoExpert::new(
            "auto-expert",
            "AutoExpert",
            vec![ExpertCapability::CodeGeneration],
        )))
        .expect("register expert");

    let task = Task::new("auto-task-1", TaskType::CodeGeneration, "build feature")
        .with_priority(TaskPriority::High)
        .with_context("auto-improvement-test");
    let result = pipeline.execute(task);

    assert!(result.is_ok());
    assert!(pipeline.auto_improvement_status().runs_total >= 1);
    assert!(
        pipeline
            .auto_improvement_status()
            .last_bundle_checksum
            .as_ref()
            .is_some()
    );
}

#[test]
fn auto_improvement_policy_can_be_configured_and_cleared() {
    let mut pipeline = MoePipelineBuilder::new().build();
    pipeline.configure_auto_improvement_policy(
        AutoImprovementPolicy::default()
            .with_min_dataset_entries(2)
            .with_min_success_ratio(0.5),
    );
    pipeline.clear_auto_improvement_policy();
    assert_eq!(pipeline.auto_improvement_status().runs_total, 0);
}
