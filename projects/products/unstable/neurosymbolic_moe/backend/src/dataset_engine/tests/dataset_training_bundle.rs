use crate::dataset_engine::{
    Correction, DatasetEntry, DatasetStore, DatasetTrainingBuildOptions, Outcome,
};
use crate::moe_core::{ExpertId, TaskId};
use std::collections::HashMap;

fn make_entry(id: &str, expert: &str, outcome: Outcome, score: Option<f64>) -> DatasetEntry {
    DatasetEntry {
        id: id.to_string(),
        task_id: TaskId::new(format!("task-{id}")),
        expert_id: ExpertId::new(expert),
        input: format!("input for {id}"),
        output: format!("output for {id}"),
        outcome,
        score,
        tags: vec!["training".to_string()],
        created_at: 1,
        metadata: HashMap::new(),
    }
}

#[test]
fn training_bundle_uses_latest_correction_for_target_output() {
    let mut store = DatasetStore::new();
    store.add_entry(make_entry("e1", "expert-a", Outcome::Failure, Some(0.8)));
    store.add_correction(Correction {
        entry_id: "e1".to_string(),
        corrected_output: "first correction".to_string(),
        reason: "review-1".to_string(),
        corrected_at: 10,
    });
    store.add_correction(Correction {
        entry_id: "e1".to_string(),
        corrected_output: "latest correction".to_string(),
        reason: "review-2".to_string(),
        corrected_at: 20,
    });

    let options = DatasetTrainingBuildOptions {
        validation_ratio: 0.0,
        ..DatasetTrainingBuildOptions::default()
    };
    let bundle = store
        .build_training_bundle(&options)
        .expect("training bundle build should succeed");

    assert_eq!(bundle.included_entries, 1);
    assert_eq!(bundle.train_samples.len(), 1);
    assert!(bundle.verify_checksum());
    let sample = &bundle.train_samples[0];
    assert!(sample.used_correction);
    assert_eq!(sample.target_output, "latest correction");
    assert_eq!(sample.correction_reason.as_deref(), Some("review-2"));
}

#[test]
fn training_bundle_filters_by_score_and_outcome_requirements() {
    let mut store = DatasetStore::new();
    store.add_entry(make_entry(
        "success",
        "expert-a",
        Outcome::Success,
        Some(0.95),
    ));
    store.add_entry(make_entry(
        "low-score",
        "expert-a",
        Outcome::Success,
        Some(0.4),
    ));
    store.add_entry(make_entry(
        "failure-no-fix",
        "expert-a",
        Outcome::Failure,
        Some(0.9),
    ));
    store.add_entry(make_entry(
        "unknown",
        "expert-a",
        Outcome::Unknown,
        Some(0.9),
    ));
    store.add_correction(Correction {
        entry_id: "success".to_string(),
        corrected_output: "should not be used".to_string(),
        reason: "irrelevant".to_string(),
        corrected_at: 1,
    });

    let options = DatasetTrainingBuildOptions {
        validation_ratio: 0.0,
        min_score: Some(0.5),
        include_unknown_entries: false,
        ..DatasetTrainingBuildOptions::default()
    };
    let bundle = store
        .build_training_bundle(&options)
        .expect("training bundle build should succeed");

    assert_eq!(bundle.total_entries, 4);
    assert_eq!(bundle.included_entries, 1);
    assert_eq!(bundle.filtered_low_score, 1);
    assert_eq!(bundle.filtered_missing_failure_correction, 1);
    assert_eq!(bundle.filtered_outcome, 1);
    assert_eq!(bundle.train_samples[0].entry_id, "success");
}

#[test]
fn training_bundle_split_is_deterministic_for_same_seed() {
    let mut store = DatasetStore::new();
    for idx in 0..20_u32 {
        store.add_entry(make_entry(
            &format!("entry-{idx}"),
            "expert-a",
            Outcome::Success,
            Some(0.9),
        ));
    }

    let options = DatasetTrainingBuildOptions {
        split_seed: 42,
        validation_ratio: 0.2,
        ..DatasetTrainingBuildOptions::default()
    };
    let first = store
        .build_training_bundle(&options)
        .expect("first training bundle build should succeed");
    let second = store
        .build_training_bundle(&options)
        .expect("second training bundle build should succeed");

    let first_train_ids: Vec<&str> = first
        .train_samples
        .iter()
        .map(|sample| sample.entry_id.as_str())
        .collect();
    let second_train_ids: Vec<&str> = second
        .train_samples
        .iter()
        .map(|sample| sample.entry_id.as_str())
        .collect();
    let first_valid_ids: Vec<&str> = first
        .validation_samples
        .iter()
        .map(|sample| sample.entry_id.as_str())
        .collect();
    let second_valid_ids: Vec<&str> = second
        .validation_samples
        .iter()
        .map(|sample| sample.entry_id.as_str())
        .collect();

    assert_eq!(first_train_ids, second_train_ids);
    assert_eq!(first_valid_ids, second_valid_ids);
}

#[test]
fn training_shards_cover_all_samples_without_overlap_loss() {
    let mut store = DatasetStore::new();
    for idx in 0..17_u32 {
        store.add_entry(make_entry(
            &format!("entry-{idx}"),
            "expert-a",
            Outcome::Success,
            Some(0.9),
        ));
    }

    let options = DatasetTrainingBuildOptions {
        split_seed: 11,
        validation_ratio: 0.3,
        ..DatasetTrainingBuildOptions::default()
    };
    let bundle = store
        .build_training_bundle(&options)
        .expect("training bundle build should succeed");
    let shards = store
        .build_training_shards(&options, 4)
        .expect("training shards build should succeed");
    let rebuilt = DatasetStore::rebuild_training_bundle_from_shards(&shards)
        .expect("rebuild from shards should succeed");

    let sharded_train: usize = shards.iter().map(|shard| shard.train_samples.len()).sum();
    let sharded_valid: usize = shards
        .iter()
        .map(|shard| shard.validation_samples.len())
        .sum();

    assert_eq!(sharded_train, bundle.train_samples.len());
    assert_eq!(sharded_valid, bundle.validation_samples.len());
    assert!(
        shards
            .iter()
            .all(|shard| shard.total_shards == shards.len())
    );
    assert_eq!(rebuilt.bundle_checksum, bundle.bundle_checksum);
}

#[test]
fn training_shards_reject_zero_max_samples() {
    let store = DatasetStore::new();
    let options = DatasetTrainingBuildOptions::default();
    let err = store
        .build_training_shards(&options, 0)
        .expect_err("zero max_samples_per_shard should fail");
    assert!(err.to_string().contains("max_samples_per_shard"));
}

#[test]
fn rebuild_training_bundle_from_shards_rejects_tampered_shard_checksum() {
    let mut store = DatasetStore::new();
    for idx in 0..8_u32 {
        store.add_entry(make_entry(
            &format!("entry-{idx}"),
            "expert-a",
            Outcome::Success,
            Some(0.9),
        ));
    }

    let options = DatasetTrainingBuildOptions {
        split_seed: 33,
        validation_ratio: 0.25,
        ..DatasetTrainingBuildOptions::default()
    };
    let mut shards = store
        .build_training_shards(&options, 3)
        .expect("training shards build should succeed");
    shards[0].train_samples[0].target_output = "tampered target".to_string();

    let err = DatasetStore::rebuild_training_bundle_from_shards(&shards)
        .expect_err("tampered shard content should fail checksum validation");
    assert!(err.to_string().contains("invalid shard checksum"));
}

#[test]
fn validate_training_bundle_rejects_overlapping_train_and_validation_ids() {
    let mut store = DatasetStore::new();
    for idx in 0..6_u32 {
        store.add_entry(make_entry(
            &format!("entry-{idx}"),
            "expert-a",
            Outcome::Success,
            Some(0.9),
        ));
    }

    let options = DatasetTrainingBuildOptions {
        split_seed: 1,
        validation_ratio: 0.5,
        ..DatasetTrainingBuildOptions::default()
    };
    let mut bundle = store
        .build_training_bundle(&options)
        .expect("training bundle build should succeed");
    if bundle.train_samples.is_empty() && !bundle.validation_samples.is_empty() {
        bundle
            .train_samples
            .push(bundle.validation_samples[0].clone());
    }
    if bundle.validation_samples.is_empty() && !bundle.train_samples.is_empty() {
        bundle
            .validation_samples
            .push(bundle.train_samples[0].clone());
    }
    let shared_id = bundle.train_samples[0].entry_id.clone();
    bundle.validation_samples[0].entry_id = shared_id;
    bundle.included_entries = bundle.train_samples.len() + bundle.validation_samples.len();
    bundle.bundle_checksum.clear();
    bundle.ensure_checksum();

    let err = DatasetStore::validate_training_bundle(&bundle)
        .expect_err("overlapping ids should fail validation");
    assert!(
        err.to_string()
            .contains("overlapping train/validation sample ids")
    );
}
