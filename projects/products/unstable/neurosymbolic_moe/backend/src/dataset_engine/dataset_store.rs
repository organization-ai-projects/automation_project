use std::collections::HashMap;

use crate::moe_core::MoeError;
use crate::moe_core::{ExpertId, TaskId};

use super::{
    Correction, DatasetEntry, DatasetQualityReport, DatasetTrainingBuildOptions,
    DatasetTrainingBundle, DatasetTrainingSample, DatasetTrainingShard, Outcome,
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

    pub fn count(&self) -> usize {
        self.entries.len()
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

        Ok(DatasetTrainingBundle {
            schema_version: DatasetTrainingBundle::schema_version(),
            generated_at: options.generated_at,
            validation_ratio: options.validation_ratio,
            split_seed: options.split_seed,
            total_entries: self.entries.len(),
            included_entries: train_samples.len() + validation_samples.len(),
            filtered_low_score,
            filtered_outcome,
            filtered_missing_failure_correction,
            train_samples,
            validation_samples,
        })
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
                generated_at: bundle.generated_at,
                split_seed: bundle.split_seed,
                validation_ratio: bundle.validation_ratio,
                shard_index,
                total_shards,
                train_samples,
                validation_samples,
            });
        }

        Ok(shards)
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
