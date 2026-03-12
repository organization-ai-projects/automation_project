use crate::dataset_engine::DatasetTrainingShard;

#[test]
fn dataset_training_shard_model_is_constructible() {
    let shard = DatasetTrainingShard {
        schema_version: 1,
        generated_at: 0,
        split_seed: 0,
        validation_ratio: 0.1,
        shard_index: 0,
        total_shards: 1,
        train_samples: Vec::new(),
        validation_samples: Vec::new(),
    };
    assert_eq!(shard.total_shards, 1);
    assert!(shard.train_samples.is_empty());
    assert!(shard.validation_samples.is_empty());
}
