use crate::dataset_engine::{
    ConcurrentDatasetStore, Correction, DatasetEntry, DatasetStore, DatasetTrainingBuildOptions,
    Outcome,
};
use crate::moe_core::{ExpertId, TaskId};
use std::collections::HashMap;
use std::thread;

fn entry(id: &str, outcome: Outcome) -> DatasetEntry {
    DatasetEntry {
        id: id.to_string(),
        task_id: TaskId::new(format!("task-{id}")),
        expert_id: ExpertId::new("concurrent-expert"),
        input: format!("input-{id}"),
        output: format!("output-{id}"),
        outcome,
        score: Some(0.8),
        tags: vec!["concurrency".to_string()],
        created_at: 1,
        metadata: HashMap::new(),
    }
}

#[test]
fn concurrent_store_supports_parallel_writes_and_bundle_build() {
    let store = ConcurrentDatasetStore::new(DatasetStore::new());
    let mut handles = Vec::new();
    for worker in 0..4_u32 {
        let writer = store.clone();
        handles.push(thread::spawn(move || {
            for idx in 0..100_u32 {
                let id = format!("entry-{}-{}", worker, idx % 50);
                writer.add_entry(entry(&id, Outcome::Success));
            }
        }));
    }
    for handle in handles {
        handle.join().expect("worker thread should not panic");
    }

    // ids are upserted by id, so each worker contributes 50 unique ids.
    assert_eq!(store.count(), 200);
    let report = store.quality_report(0.5);
    assert_eq!(report.total_entries, 200);

    let options = DatasetTrainingBuildOptions {
        validation_ratio: 0.2,
        ..DatasetTrainingBuildOptions::default()
    };
    let bundle = store
        .build_training_bundle(&options)
        .expect("concurrent training bundle build should succeed");
    assert!(bundle.verify_checksum());
    let shards = store
        .build_training_shards(&options, 32)
        .expect("concurrent shard build should succeed");
    let rebuilt = store
        .rebuild_training_bundle_from_shards(&shards)
        .expect("rebuild from shards should succeed");
    assert_eq!(rebuilt.bundle_checksum, bundle.bundle_checksum);
}

#[test]
fn concurrent_store_supports_parallel_corrections() {
    let store = ConcurrentDatasetStore::new(DatasetStore::new());
    store.add_entry(entry("entry-correction", Outcome::Failure));

    let mut handles = Vec::new();
    for idx in 0..8_u32 {
        let writer = store.clone();
        handles.push(thread::spawn(move || {
            writer.add_correction(Correction {
                entry_id: "entry-correction".to_string(),
                corrected_output: format!("correction-{idx}"),
                reason: "review".to_string(),
                corrected_at: u64::from(idx),
            });
        }));
    }
    for handle in handles {
        handle.join().expect("correction thread should not panic");
    }

    let options = DatasetTrainingBuildOptions {
        validation_ratio: 0.0,
        ..DatasetTrainingBuildOptions::default()
    };
    let bundle = store
        .build_training_bundle(&options)
        .expect("bundle build with parallel corrections should succeed");
    assert_eq!(bundle.included_entries, 1);
    assert!(bundle.train_samples[0].used_correction);
}
