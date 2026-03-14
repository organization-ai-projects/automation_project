//! projects/products/unstable/neurosymbolic_moe/backend/src/dataset_engine/dataset_store.rs
use std::collections::HashMap;

use crate::moe_core::MoeError;
use crate::moe_core::{ExpertId, TaskId};

use super::{
    Correction, DatasetEntry, DatasetQualityReport, DatasetTrainingBuildOptions,
    DatasetTrainingBundle, DatasetTrainingProvenance, DatasetTrainingSample, DatasetTrainingShard,
    Outcome,
};

#[derive(Debug, Clone)]
pub struct DatasetStore {
    entries: Vec<DatasetEntry>,
    corrections: HashMap<String, Vec<Correction>>,
}

impl DatasetStore {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            corrections: HashMap::new(),
        }
    }

    pub fn add_entry(&mut self, entry: DatasetEntry) {
        if self.has_entry_id(&entry.id) {
            self.upsert_entry(entry);
        } else {
            self.entries.push(entry);
        }
    }

    pub fn upsert_entry(&mut self, entry: DatasetEntry) {
        if let Some(existing) = self
            .entries
            .iter_mut()
            .find(|existing| existing.id == entry.id)
        {
            *existing = entry;
            return;
        }
        self.entries.push(entry);
    }

    pub fn add_correction(&mut self, correction: Correction) {
        self.corrections
            .entry(correction.entry_id.clone())
            .or_default()
            .push(correction);
    }

    pub fn get_by_task(&self, task_id: &TaskId) -> Vec<&DatasetEntry> {
        self.entries
            .iter()
            .filter(|e| e.task_id == *task_id)
            .collect()
    }

    pub fn get_by_expert(&self, expert_id: &ExpertId) -> Vec<&DatasetEntry> {
        self.entries
            .iter()
            .filter(|e| e.expert_id == *expert_id)
            .collect()
    }

    pub fn get_by_outcome(&self, outcome: &Outcome) -> Vec<&DatasetEntry> {
        self.entries
            .iter()
            .filter(|e| e.outcome == *outcome)
            .collect()
    }

    pub fn get_corrections(&self, entry_id: &str) -> Option<&Vec<Correction>> {
        self.corrections.get(entry_id)
    }

    pub fn entries_cloned(&self) -> Vec<DatasetEntry> {
        self.entries.clone()
    }

    pub fn corrections_cloned(&self) -> HashMap<String, Vec<Correction>> {
        self.corrections.clone()
    }

    pub fn replace_all(
        &mut self,
        entries: Vec<DatasetEntry>,
        corrections: HashMap<String, Vec<Correction>>,
    ) {
        self.entries = entries;
        self.corrections = corrections;
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }

    pub fn has_entry_id(&self, entry_id: &str) -> bool {
        self.entries.iter().any(|entry| entry.id == entry_id)
    }

    pub fn successful_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| e.outcome == Outcome::Success)
            .count()
    }

    pub fn failed_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| e.outcome == Outcome::Failure)
            .count()
    }

    pub fn average_score(&self) -> Option<f64> {
        let mut total = 0.0;
        let mut count = 0usize;

        for entry in &self.entries {
            if let Some(score) = entry.score {
                total += score;
                count += 1;
            }
        }

        if count == 0 {
            None
        } else {
            Some(total / count as f64)
        }
    }

    pub fn quality_report(&self, low_score_threshold: f64) -> DatasetQualityReport {
        let total_entries = self.entries.len();
        let scored_entries = self
            .entries
            .iter()
            .filter(|entry| entry.score.is_some())
            .count();
        let average_score = self.average_score();
        let low_score_entries = self
            .entries
            .iter()
            .filter(|entry| entry.score.is_some_and(|score| score < low_score_threshold))
            .count();

        let corrected_entries = self
            .entries
            .iter()
            .filter(|entry| {
                self.corrections
                    .get(&entry.id)
                    .is_some_and(|corrections| !corrections.is_empty())
            })
            .count();

        let correction_ratio = if total_entries == 0 {
            0.0
        } else {
            corrected_entries as f64 / total_entries as f64
        };

        let success_ratio = if total_entries == 0 {
            0.0
        } else {
            self.successful_count() as f64 / total_entries as f64
        };

        DatasetQualityReport {
            total_entries,
            scored_entries,
            average_score,
            low_score_entries,
            corrected_entries,
            correction_ratio,
            success_ratio,
        }
    }

    pub fn build_training_bundle(
        &self,
        options: &DatasetTrainingBuildOptions,
    ) -> Result<DatasetTrainingBundle, MoeError> {
        self.build_training_bundle_with_provenance(options, DatasetTrainingProvenance::default())
    }

    pub fn build_training_bundle_with_provenance(
        &self,
        options: &DatasetTrainingBuildOptions,
        provenance: DatasetTrainingProvenance,
    ) -> Result<DatasetTrainingBundle, MoeError> {
        if !(0.0..1.0).contains(&options.validation_ratio) {
            return Err(MoeError::DatasetError(format!(
                "invalid validation_ratio {} (must be in [0.0, 1.0))",
                options.validation_ratio
            )));
        }

        let mut filtered_low_score = 0usize;
        let mut filtered_outcome = 0usize;
        let mut filtered_missing_failure_correction = 0usize;
        let mut train_samples = Vec::new();
        let mut validation_samples = Vec::new();

        for entry in &self.entries {
            let latest_correction = self.corrections.get(&entry.id).and_then(|corrections| {
                corrections
                    .iter()
                    .max_by_key(|correction| correction.corrected_at)
            });

            if let Some(min_score) = options.min_score {
                let too_low = entry.score.is_none_or(|score| score < min_score);
                if too_low {
                    filtered_low_score += 1;
                    continue;
                }
            }

            let outcome_allowed = match entry.outcome {
                Outcome::Success => true,
                Outcome::Failure => options.include_failure_entries,
                Outcome::Partial => options.include_partial_entries,
                Outcome::Unknown => options.include_unknown_entries,
            };
            if !outcome_allowed {
                filtered_outcome += 1;
                continue;
            }

            if matches!(entry.outcome, Outcome::Failure)
                && options.require_correction_for_failure
                && latest_correction.is_none()
            {
                filtered_missing_failure_correction += 1;
                continue;
            }

            let (target_output, used_correction, correction_reason) = latest_correction
                .map(|correction| {
                    (
                        correction.corrected_output.clone(),
                        true,
                        Some(correction.reason.clone()),
                    )
                })
                .unwrap_or_else(|| (entry.output.clone(), false, None));

            let sample = DatasetTrainingSample {
                entry_id: entry.id.clone(),
                task_id: entry.task_id.as_str().to_string(),
                expert_id: entry.expert_id.as_str().to_string(),
                input: entry.input.clone(),
                target_output,
                source_output: entry.output.clone(),
                used_correction,
                correction_reason,
                score: entry.score,
                tags: entry.tags.clone(),
                metadata: entry.metadata.clone(),
            };

            let split_key = format!(
                "{}:{}:{}:{}",
                options.split_seed, sample.entry_id, sample.task_id, sample.expert_id
            );
            let bucket = fnv1a64(split_key.as_bytes()) % 10_000;
            let is_validation = (bucket as f64) < (options.validation_ratio * 10_000.0);
            if is_validation {
                validation_samples.push(sample);
            } else {
                train_samples.push(sample);
            }
        }

        let mut bundle = DatasetTrainingBundle {
            schema_version: DatasetTrainingBundle::schema_version(),
            bundle_checksum: String::new(),
            generated_at: options.generated_at,
            validation_ratio: options.validation_ratio,
            split_seed: options.split_seed,
            total_entries: self.entries.len(),
            included_entries: train_samples.len() + validation_samples.len(),
            filtered_low_score,
            filtered_outcome,
            filtered_missing_failure_correction,
            provenance,
            train_samples,
            validation_samples,
        };
        bundle.ensure_checksum();
        Self::validate_training_bundle(&bundle)?;
        Ok(bundle)
    }

    pub fn build_training_shards(
        &self,
        options: &DatasetTrainingBuildOptions,
        max_samples_per_shard: usize,
    ) -> Result<Vec<DatasetTrainingShard>, MoeError> {
        if max_samples_per_shard == 0 {
            return Err(MoeError::DatasetError(
                "max_samples_per_shard must be greater than zero".to_string(),
            ));
        }

        let bundle = self.build_training_bundle(options)?;
        Self::shard_training_bundle(&bundle, max_samples_per_shard)
    }

    pub fn shard_training_bundle(
        bundle: &DatasetTrainingBundle,
        max_samples_per_shard: usize,
    ) -> Result<Vec<DatasetTrainingShard>, MoeError> {
        if max_samples_per_shard == 0 {
            return Err(MoeError::DatasetError(
                "max_samples_per_shard must be greater than zero".to_string(),
            ));
        }
        if !bundle.has_supported_schema() {
            return Err(MoeError::DatasetError(format!(
                "unsupported training bundle schema version {}",
                bundle.schema_version
            )));
        }

        let train_shards = bundle.train_samples.len().div_ceil(max_samples_per_shard);
        let validation_shards = bundle
            .validation_samples
            .len()
            .div_ceil(max_samples_per_shard);
        let total_shards = train_shards.max(validation_shards).max(1);

        let mut shards = Vec::with_capacity(total_shards);
        for shard_index in 0..total_shards {
            let train_start = shard_index * max_samples_per_shard;
            let train_end =
                ((shard_index + 1) * max_samples_per_shard).min(bundle.train_samples.len());
            let validation_start = shard_index * max_samples_per_shard;
            let validation_end =
                ((shard_index + 1) * max_samples_per_shard).min(bundle.validation_samples.len());

            let train_samples = if train_start < train_end {
                bundle.train_samples[train_start..train_end].to_vec()
            } else {
                Vec::new()
            };
            let validation_samples = if validation_start < validation_end {
                bundle.validation_samples[validation_start..validation_end].to_vec()
            } else {
                Vec::new()
            };

            shards.push(DatasetTrainingShard {
                schema_version: bundle.schema_version,
                bundle_checksum: bundle.bundle_checksum.clone(),
                shard_checksum: String::new(),
                generated_at: bundle.generated_at,
                split_seed: bundle.split_seed,
                validation_ratio: bundle.validation_ratio,
                total_entries: bundle.total_entries,
                included_entries: bundle.included_entries,
                filtered_low_score: bundle.filtered_low_score,
                filtered_outcome: bundle.filtered_outcome,
                filtered_missing_failure_correction: bundle.filtered_missing_failure_correction,
                provenance: bundle.provenance.clone(),
                shard_index,
                total_shards,
                train_samples,
                validation_samples,
            });
            if let Some(last) = shards.last_mut() {
                last.ensure_checksum();
            }
        }

        Ok(shards)
    }

    pub fn rebuild_training_bundle_from_shards(
        shards: &[DatasetTrainingShard],
    ) -> Result<DatasetTrainingBundle, MoeError> {
        if shards.is_empty() {
            return Err(MoeError::DatasetError(
                "cannot rebuild training bundle from empty shards".to_string(),
            ));
        }

        let first = &shards[0];
        let total_shards = first.total_shards;
        if total_shards == 0 {
            return Err(MoeError::DatasetError(
                "invalid shard metadata: total_shards must be > 0".to_string(),
            ));
        }
        if shards.len() != total_shards {
            return Err(MoeError::DatasetError(format!(
                "invalid shard set: expected {} shards, got {}",
                total_shards,
                shards.len()
            )));
        }

        let mut index_set: std::collections::HashSet<usize> = std::collections::HashSet::new();
        for shard in shards {
            if !shard.verify_checksum() {
                return Err(MoeError::DatasetError(format!(
                    "invalid shard checksum at index {}",
                    shard.shard_index
                )));
            }
            if shard.total_shards != total_shards
                || shard.schema_version != first.schema_version
                || shard.bundle_checksum != first.bundle_checksum
                || shard.generated_at != first.generated_at
                || shard.split_seed != first.split_seed
                || (shard.validation_ratio - first.validation_ratio).abs() > f64::EPSILON
                || shard.total_entries != first.total_entries
                || shard.included_entries != first.included_entries
                || shard.filtered_low_score != first.filtered_low_score
                || shard.filtered_outcome != first.filtered_outcome
                || shard.filtered_missing_failure_correction
                    != first.filtered_missing_failure_correction
                || shard.provenance.generator != first.provenance.generator
                || shard.provenance.governance_state_version
                    != first.provenance.governance_state_version
                || shard.provenance.governance_state_checksum
                    != first.provenance.governance_state_checksum
                || shard.provenance.runtime_bundle_checksum
                    != first.provenance.runtime_bundle_checksum
                || shard.provenance.dataset_entry_count != first.provenance.dataset_entry_count
            {
                return Err(MoeError::DatasetError(
                    "inconsistent shard metadata across shard set".to_string(),
                ));
            }
            if !index_set.insert(shard.shard_index) {
                return Err(MoeError::DatasetError(format!(
                    "duplicate shard index {}",
                    shard.shard_index
                )));
            }
        }
        if (0..total_shards).any(|index| !index_set.contains(&index)) {
            return Err(MoeError::DatasetError(
                "shard set is missing one or more shard indexes".to_string(),
            ));
        }

        let mut ordered = shards.to_vec();
        ordered.sort_by_key(|shard| shard.shard_index);

        let mut train_samples = Vec::new();
        let mut validation_samples = Vec::new();
        for shard in &ordered {
            train_samples.extend(shard.train_samples.clone());
            validation_samples.extend(shard.validation_samples.clone());
        }

        let mut bundle = DatasetTrainingBundle {
            schema_version: first.schema_version,
            bundle_checksum: first.bundle_checksum.clone(),
            generated_at: first.generated_at,
            validation_ratio: first.validation_ratio,
            split_seed: first.split_seed,
            total_entries: first.total_entries,
            included_entries: first.included_entries,
            filtered_low_score: first.filtered_low_score,
            filtered_outcome: first.filtered_outcome,
            filtered_missing_failure_correction: first.filtered_missing_failure_correction,
            provenance: first.provenance.clone(),
            train_samples,
            validation_samples,
        };
        bundle.ensure_checksum();
        if !bundle.has_supported_schema() {
            return Err(MoeError::DatasetError(format!(
                "unsupported training bundle schema version {}",
                bundle.schema_version
            )));
        }
        if !bundle.verify_checksum() {
            return Err(MoeError::DatasetError(
                "rebuilt training bundle checksum verification failed".to_string(),
            ));
        }
        if bundle.bundle_checksum != first.bundle_checksum {
            return Err(MoeError::DatasetError(
                "rebuilt training bundle checksum mismatch against shard bundle checksum"
                    .to_string(),
            ));
        }
        if bundle.included_entries != bundle.train_samples.len() + bundle.validation_samples.len() {
            return Err(MoeError::DatasetError(
                "rebuilt training bundle included_entries does not match sample counts".to_string(),
            ));
        }
        Self::validate_training_bundle(&bundle)?;
        Ok(bundle)
    }

    pub fn validate_training_bundle(bundle: &DatasetTrainingBundle) -> Result<(), MoeError> {
        if !bundle.has_supported_schema() {
            return Err(MoeError::DatasetError(format!(
                "unsupported training bundle schema version {}",
                bundle.schema_version
            )));
        }
        if !(0.0..1.0).contains(&bundle.validation_ratio) {
            return Err(MoeError::DatasetError(format!(
                "invalid validation_ratio {} (must be in [0.0, 1.0))",
                bundle.validation_ratio
            )));
        }
        if !bundle.verify_checksum() {
            return Err(MoeError::DatasetError(
                "training bundle checksum verification failed".to_string(),
            ));
        }

        let included_from_samples = bundle.train_samples.len() + bundle.validation_samples.len();
        if bundle.included_entries != included_from_samples {
            return Err(MoeError::DatasetError(format!(
                "training bundle included_entries mismatch ({} != {})",
                bundle.included_entries, included_from_samples
            )));
        }
        if bundle.total_entries < bundle.included_entries {
            return Err(MoeError::DatasetError(format!(
                "training bundle total_entries ({}) is below included_entries ({})",
                bundle.total_entries, bundle.included_entries
            )));
        }

        let train_duplicates = Self::duplicate_sample_ids(&bundle.train_samples);
        if !train_duplicates.is_empty() {
            return Err(MoeError::DatasetError(format!(
                "training bundle has duplicate train sample ids: {}",
                train_duplicates.join(", ")
            )));
        }
        let validation_duplicates = Self::duplicate_sample_ids(&bundle.validation_samples);
        if !validation_duplicates.is_empty() {
            return Err(MoeError::DatasetError(format!(
                "training bundle has duplicate validation sample ids: {}",
                validation_duplicates.join(", ")
            )));
        }

        let train_ids: std::collections::HashSet<&str> = bundle
            .train_samples
            .iter()
            .map(|sample| sample.entry_id.as_str())
            .collect();
        let mut overlap_ids: Vec<&str> = bundle
            .validation_samples
            .iter()
            .map(|sample| sample.entry_id.as_str())
            .filter(|id| train_ids.contains(id))
            .collect();
        overlap_ids.sort_unstable();
        overlap_ids.dedup();
        if !overlap_ids.is_empty() {
            return Err(MoeError::DatasetError(format!(
                "training bundle has overlapping train/validation sample ids: {}",
                overlap_ids.join(", ")
            )));
        }

        if bundle.provenance.dataset_entry_count != 0
            && bundle.provenance.dataset_entry_count != bundle.total_entries
        {
            return Err(MoeError::DatasetError(format!(
                "training provenance dataset_entry_count mismatch ({} != {})",
                bundle.provenance.dataset_entry_count, bundle.total_entries
            )));
        }
        if bundle.provenance.generator.is_empty()
            && (!bundle.provenance.governance_state_checksum.is_empty()
                || !bundle.provenance.runtime_bundle_checksum.is_empty())
        {
            return Err(MoeError::DatasetError(
                "training provenance is inconsistent: generator missing while checksums are set"
                    .to_string(),
            ));
        }
        Ok(())
    }

    fn duplicate_sample_ids(samples: &[DatasetTrainingSample]) -> Vec<String> {
        let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
        let mut duplicates: Vec<String> = samples
            .iter()
            .map(|sample| sample.entry_id.as_str())
            .filter(|id| !seen.insert(*id))
            .map(ToString::to_string)
            .collect();
        duplicates.sort();
        duplicates.dedup();
        duplicates
    }
}

impl Default for DatasetStore {
    fn default() -> Self {
        Self::new()
    }
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
