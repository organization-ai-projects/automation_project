use crate::dataset_engine::DatasetTrainingShard;

#[test]
fn dataset_training_shard_model_is_constructible() {
    let mut shard = DatasetTrainingShard {
        schema_version: 1,
        bundle_checksum: "bundle-checksum".to_string(),
        shard_checksum: String::new(),
        generated_at: 0,
        split_seed: 0,
        validation_ratio: 0.1,
        total_entries: 0,
        included_entries: 0,
        filtered_low_score: 0,
        filtered_outcome: 0,
        filtered_missing_failure_correction: 0,
        shard_index: 0,
        total_shards: 1,
        train_samples: Vec::new(),
        validation_samples: Vec::new(),
    };
    shard.ensure_checksum();
    assert_eq!(shard.total_shards, 1);
    assert!(shard.train_samples.is_empty());
    assert!(shard.validation_samples.is_empty());
    assert!(shard.verify_checksum());
}
