use serde::{Deserialize, Serialize};

use super::{DatasetTrainingProvenance, DatasetTrainingSample};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetTrainingShard {
    pub schema_version: u32,
    #[serde(default)]
    pub bundle_checksum: String,
    #[serde(default)]
    pub shard_checksum: String,
    pub generated_at: u64,
    pub split_seed: u64,
    pub validation_ratio: f64,
    pub total_entries: usize,
    pub included_entries: usize,
    pub filtered_low_score: usize,
    pub filtered_outcome: usize,
    pub filtered_missing_failure_correction: usize,
    #[serde(default)]
    pub provenance: DatasetTrainingProvenance,
    pub shard_index: usize,
    pub total_shards: usize,
    pub train_samples: Vec<DatasetTrainingSample>,
    pub validation_samples: Vec<DatasetTrainingSample>,
}

impl DatasetTrainingShard {
    pub fn ensure_checksum(&mut self) {
        if self.shard_checksum.is_empty() {
            self.shard_checksum = self.recompute_checksum();
        }
    }

    pub fn verify_checksum(&self) -> bool {
        !self.shard_checksum.is_empty() && self.shard_checksum == self.recompute_checksum()
    }

    pub fn recompute_checksum(&self) -> String {
        let train_fp = self
            .train_samples
            .iter()
            .map(sample_fingerprint)
            .collect::<Vec<_>>()
            .join("||");
        let validation_fp = self
            .validation_samples
            .iter()
            .map(sample_fingerprint)
            .collect::<Vec<_>>()
            .join("||");
        let material = format!(
            "{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{train_fp}::{validation_fp}",
            self.schema_version,
            self.bundle_checksum,
            self.generated_at,
            self.split_seed,
            self.validation_ratio,
            self.total_entries,
            self.included_entries,
            self.filtered_low_score,
            self.filtered_outcome,
            self.filtered_missing_failure_correction,
            self.provenance.generator,
            self.provenance.governance_state_version,
            self.provenance.governance_state_checksum,
            self.provenance.runtime_bundle_checksum,
            self.provenance.dataset_entry_count,
            self.shard_index,
            self.total_shards,
            self.train_samples.len()
        );
        format!("{:016x}", fnv1a64(material.as_bytes()))
    }
}

fn sample_fingerprint(sample: &DatasetTrainingSample) -> String {
    let mut tags = sample.tags.clone();
    tags.sort();
    let mut metadata: Vec<(&str, &str)> = sample
        .metadata
        .iter()
        .map(|(key, value)| (key.as_str(), value.as_str()))
        .collect();
    metadata.sort_by(|a, b| a.0.cmp(b.0));
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{:?}|{:?}|{:?}",
        sample.entry_id,
        sample.task_id,
        sample.expert_id,
        sample.input,
        sample.target_output,
        sample.source_output,
        sample.used_correction,
        sample.correction_reason.clone().unwrap_or_default(),
        sample.score,
        tags,
        metadata
    )
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
