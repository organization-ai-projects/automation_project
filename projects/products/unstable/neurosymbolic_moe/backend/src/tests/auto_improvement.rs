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
fn auto_improvement_policy_can_be_attached_via_builder() {
    let policy = AutoImprovementPolicy::default()
        .with_min_dataset_entries(2)
        .with_min_success_ratio(0.5);
    let pipeline = MoePipelineBuilder::new()
        .with_auto_improvement_policy(policy)
        .build();
    assert_eq!(pipeline.auto_improvement_status().runs_total, 0);
}

#[test]
fn execute_tracks_skip_reason_when_min_dataset_entries_is_not_reached() {
    let policy = AutoImprovementPolicy::default()
        .with_min_dataset_entries(10_000)
        .with_min_success_ratio(0.0)
        .with_min_average_score(None);
    let mut pipeline = MoePipelineBuilder::new()
        .with_auto_improvement_policy(policy)
        .build();
    pipeline
        .register_expert(Box::new(EchoExpert::new(
            "auto-skip-expert",
            "AutoSkipExpert",
            vec![ExpertCapability::CodeGeneration],
        )))
        .expect("register expert");

    pipeline
        .execute(Task::new(
            "auto-skip-task-1",
            TaskType::CodeGeneration,
            "skip me",
        ))
        .expect("execution should succeed");

    let status = pipeline.auto_improvement_status();
    assert_eq!(status.runs_total, 0);
    assert_eq!(status.skipped_min_dataset_entries_total, 1);
    assert_eq!(
        status.last_skip_reason.as_deref(),
        Some("dataset entries below min_dataset_entries")
    );
}

#[test]
fn trainer_trigger_event_queue_supports_pop_and_bounded_drain() {
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
            split_seed: 21,
        });
    let mut pipeline = MoePipelineBuilder::new()
        .with_router(Box::new(HeuristicRouter::new(2)))
        .with_aggregation_strategy(AggregationStrategy::HighestConfidence)
        .with_auto_improvement_policy(policy)
        .build();
    pipeline
        .register_expert(Box::new(EchoExpert::new(
            "auto-queue-expert",
            "AutoQueueExpert",
            vec![ExpertCapability::CodeGeneration],
        )))
        .expect("register expert");

    for idx in 0..3 {
        pipeline
            .execute(Task::new(
                format!("auto-queue-task-{idx}"),
                TaskType::CodeGeneration,
                "queue run",
            ))
            .expect("execution should succeed");
    }

    assert!(pipeline.trainer_trigger_events_pending() >= 3);
    assert!(pipeline.pop_next_trainer_trigger_event().is_some());
    let remaining = pipeline.trainer_trigger_events_pending();
    assert!(remaining >= 2);

    let drained = pipeline.drain_trainer_trigger_events(1);
    assert_eq!(drained.len(), 1);
    assert_eq!(pipeline.trainer_trigger_events_pending(), remaining - 1);
    assert!(!pipeline.trainer_trigger_events().is_empty());
}

#[test]
fn trainer_trigger_event_queue_supports_lease_fail_and_ack_flow() {
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
            split_seed: 31,
        });
    let mut pipeline = MoePipelineBuilder::new()
        .with_router(Box::new(HeuristicRouter::new(2)))
        .with_aggregation_strategy(AggregationStrategy::HighestConfidence)
        .with_auto_improvement_policy(policy)
        .build();
    pipeline
        .register_expert(Box::new(EchoExpert::new(
            "auto-lease-expert",
            "AutoLeaseExpert",
            vec![ExpertCapability::CodeGeneration],
        )))
        .expect("register expert");

    pipeline
        .execute(Task::new(
            "auto-lease-task-1",
            TaskType::CodeGeneration,
            "lease run",
        ))
        .expect("execution should succeed");
    assert_eq!(pipeline.trainer_trigger_events_pending(), 1);

    let leased = pipeline
        .lease_next_trainer_trigger_event(100, 30)
        .expect("expected a leased event");
    assert_eq!(leased.delivery_attempts, 1);
    assert_eq!(leased.last_attempted_at, Some(100));
    assert!(
        pipeline.lease_next_trainer_trigger_event(110, 30).is_none(),
        "event should not be leaseable before retry delay"
    );
    assert!(pipeline.mark_trainer_trigger_event_delivery_failed(leased.event_id, 120));
    let re_lease = pipeline
        .lease_next_trainer_trigger_event(151, 30)
        .expect("expected a re-leased event");
    assert_eq!(re_lease.delivery_attempts, 2);
    assert_eq!(re_lease.last_attempted_at, Some(151));
    assert!(pipeline.acknowledge_trainer_trigger_event(re_lease.event_id));
    assert_eq!(pipeline.trainer_trigger_events_pending(), 0);
    assert_eq!(
        pipeline
            .auto_improvement_status()
            .trainer_trigger_delivery_attempts_total,
        2
    );
    assert_eq!(
        pipeline
            .auto_improvement_status()
            .trainer_trigger_delivery_failures_total,
        1
    );
    assert_eq!(
        pipeline
            .auto_improvement_status()
            .trainer_trigger_acknowledged_total,
        1
    );
}
