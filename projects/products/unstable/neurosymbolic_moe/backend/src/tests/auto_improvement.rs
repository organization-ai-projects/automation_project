//! projects/products/unstable/neurosymbolic_moe/backend/src/tests/auto_improvement.rs
use std::collections::HashMap;

use protocol::ProtocolId;
use std::str::FromStr;

use crate::aggregator::AggregationStrategy;
use crate::dataset_engine::{
    DatasetTrainingBuildOptions, DatasetTrainingBundle, DatasetTrainingProvenance,
    DatasetTrainingSample,
};
use crate::echo_expert::EchoExpert;
use crate::moe_core::{ExpertCapability, ExpertId, Task, TaskId, TaskPriority, TaskType};
use crate::orchestrator::{AutoImprovementPolicy, MoePipelineBuilder};
use crate::router::HeuristicRouter;

fn protocol_id(byte: u8) -> ProtocolId {
    ProtocolId::from_str(&format!("{:032x}", byte.max(1)))
        .expect("test protocol id should be valid fixed hex")
}

#[test]
fn bootstrap_initial_dataset_from_training_bundle_json_seeds_entries() {
    let mut pipeline = MoePipelineBuilder::new().build();
    let sample = DatasetTrainingSample {
        entry_id: ProtocolId::default(),
        task_id: TaskId::new(),
        expert_id: ExpertId::new(),
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
        pipeline
            .auto_improvement_status()
            .global_counters
            .bootstrap_entries_total,
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
        .register_expert(Box::new(EchoExpert::new_with_id(
            protocol_id(1),
            "AutoExpert",
            vec![ExpertCapability::CodeGeneration],
        )))
        .expect("register expert");

    let task = Task::new(TaskType::CodeGeneration, "build feature")
        .with_priority(TaskPriority::High)
        .with_context("auto-improvement-test");
    let result = pipeline.execute(task);

    assert!(result.is_ok());
    assert!(
        pipeline
            .auto_improvement_status()
            .global_counters
            .runs_total
            >= 1
    );
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
    assert_eq!(
        pipeline
            .auto_improvement_status()
            .global_counters
            .runs_total,
        0
    );
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
        .register_expert(Box::new(EchoExpert::new_with_id(
            protocol_id(2),
            "AutoSkipExpert",
            vec![ExpertCapability::CodeGeneration],
        )))
        .expect("register expert");

    pipeline
        .execute(Task::new(TaskType::CodeGeneration, "skip me"))
        .expect("execution should succeed");

    let status = pipeline.auto_improvement_status();
    assert_eq!(status.global_counters.runs_total, 0);
    assert_eq!(status.skip_counters.min_dataset_entries_total, 1);
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
        .register_expert(Box::new(EchoExpert::new_with_id(
            protocol_id(3),
            "AutoQueueExpert",
            vec![ExpertCapability::CodeGeneration],
        )))
        .expect("register expert");

    for _ in 0..3 {
        pipeline
            .execute(Task::new(TaskType::CodeGeneration, "queue run"))
            .expect("execution should succeed");
    }

    assert!(pipeline.trainer_trigger_events_pending() >= 3);
    assert!(pipeline.pop_next_trainer_trigger_event().is_some());
    let remaining = pipeline.trainer_trigger_events_pending();
    assert!(remaining >= 2);

    assert!(pipeline.pop_next_trainer_trigger_event().is_some());
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
        .register_expert(Box::new(EchoExpert::new_with_id(
            protocol_id(4),
            "auto-lease-expert",
            vec![ExpertCapability::CodeGeneration],
        )))
        .expect("register expert");

    pipeline
        .execute(Task::new(TaskType::CodeGeneration, "lease run"))
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
            .delivery_stats
            .delivery_attempts_total,
        2
    );
    assert_eq!(
        pipeline
            .auto_improvement_status()
            .delivery_stats
            .delivery_failures_total,
        1
    );
    assert_eq!(
        pipeline
            .auto_improvement_status()
            .delivery_stats
            .acknowledged_total,
        1
    );
    assert_eq!(
        pipeline
            .auto_improvement_status()
            .delivery_stats
            .dead_letter_total,
        0
    );
}

#[test]
fn trainer_trigger_event_queue_dead_letters_after_max_attempts_policy() {
    let policy = AutoImprovementPolicy::default()
        .with_min_dataset_entries(1)
        .with_min_success_ratio(0.0)
        .with_min_average_score(None)
        .with_trainer_trigger_min_retry_delay_seconds(0)
        .with_trainer_trigger_max_delivery_attempts_before_dead_letter(2)
        .with_training_build_options(DatasetTrainingBuildOptions {
            generated_at: 1,
            validation_ratio: 0.2,
            min_score: None,
            include_failure_entries: true,
            include_partial_entries: true,
            include_unknown_entries: false,
            require_correction_for_failure: false,
            split_seed: 37,
        });
    let mut pipeline = MoePipelineBuilder::new()
        .with_router(Box::new(HeuristicRouter::new(2)))
        .with_aggregation_strategy(AggregationStrategy::HighestConfidence)
        .with_auto_improvement_policy(policy)
        .build();
    pipeline
        .register_expert(Box::new(EchoExpert::new_with_id(
            protocol_id(5),
            "auto-dead-letter-expert",
            vec![ExpertCapability::CodeGeneration],
        )))
        .expect("register expert");

    pipeline
        .execute(Task::new(TaskType::CodeGeneration, "dead-letter run"))
        .expect("execution should succeed");

    let first = pipeline
        .lease_next_trainer_trigger_event_with_policy(1)
        .expect("expected first lease");
    assert!(pipeline.mark_trainer_trigger_event_delivery_failed(first.event_id, 2));
    let second = pipeline
        .lease_next_trainer_trigger_event_with_policy(3)
        .expect("expected second lease");
    assert!(pipeline.mark_trainer_trigger_event_delivery_failed(second.event_id, 4));

    assert!(
        pipeline
            .lease_next_trainer_trigger_event_with_policy(5)
            .is_none(),
        "event should be dead-lettered after max attempts"
    );
    assert_eq!(pipeline.trainer_trigger_events_pending(), 0);
    assert_eq!(pipeline.trainer_trigger_dead_letter_events_total(), 1);
    assert_eq!(
        pipeline
            .auto_improvement_status()
            .delivery_stats
            .dead_letter_total,
        1
    );
}
