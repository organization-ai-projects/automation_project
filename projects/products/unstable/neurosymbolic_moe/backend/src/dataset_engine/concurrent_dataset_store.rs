use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::moe_core::MoeError;

use super::{
    Correction, DatasetEntry, DatasetQualityReport, DatasetStore, DatasetTrainingBuildOptions,
    DatasetTrainingBundle, DatasetTrainingShard,
};

#[derive(Debug, Clone, Default)]
pub struct ConcurrentDatasetStore {
    inner: Arc<RwLock<DatasetStore>>,
}

impl ConcurrentDatasetStore {
    pub fn new(store: DatasetStore) -> Self {
        Self {
            inner: Arc::new(RwLock::new(store)),
        }
    }

    pub fn add_entry(&self, entry: DatasetEntry) {
        self.write_guard().add_entry(entry);
    }

    pub fn add_correction(&self, correction: Correction) {
        self.write_guard().add_correction(correction);
    }

    pub fn count(&self) -> usize {
        self.read_guard().count()
    }

    pub fn quality_report(&self, low_score_threshold: f64) -> DatasetQualityReport {
        self.read_guard().quality_report(low_score_threshold)
    }

    pub fn build_training_bundle(
        &self,
        options: &DatasetTrainingBuildOptions,
    ) -> Result<DatasetTrainingBundle, MoeError> {
        self.read_guard().build_training_bundle(options)
    }

    pub fn build_training_shards(
        &self,
        options: &DatasetTrainingBuildOptions,
        max_samples_per_shard: usize,
    ) -> Result<Vec<DatasetTrainingShard>, MoeError> {
        self.read_guard()
            .build_training_shards(options, max_samples_per_shard)
    }

    pub fn rebuild_training_bundle_from_shards(
        &self,
        shards: &[DatasetTrainingShard],
    ) -> Result<DatasetTrainingBundle, MoeError> {
        DatasetStore::rebuild_training_bundle_from_shards(shards)
    }

    fn read_guard(&self) -> RwLockReadGuard<'_, DatasetStore> {
        match self.inner.read() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    fn write_guard(&self) -> RwLockWriteGuard<'_, DatasetStore> {
        match self.inner.write() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }
}
