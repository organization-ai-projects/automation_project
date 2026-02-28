// projects/products/unstable/protocol_builder/tooling/src/validate/golden_transcript_validator.rs
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptEntry {
    pub request: String,
    pub response: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenTranscript {
    pub schema_name: String,
    pub entries: Vec<TranscriptEntry>,
    pub final_manifest_hash: String,
    pub artifacts: BTreeMap<String, String>,
}

pub struct GoldenTranscriptValidator;

impl GoldenTranscriptValidator {
    /// Validates that the golden transcript is internally consistent.
    pub fn validate(transcript: &GoldenTranscript) -> Result<()> {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        for content in transcript.artifacts.values() {
            hasher.update(content.as_bytes());
        }
        let computed = hex::encode(hasher.finalize());
        if computed != transcript.final_manifest_hash {
            anyhow::bail!(
                "transcript manifest_hash mismatch: expected {} got {}",
                transcript.final_manifest_hash,
                computed
            );
        }
        tracing::info!(
            schema_name = %transcript.schema_name,
            entries = transcript.entries.len(),
            manifest_hash = %computed,
            "golden transcript validation passed"
        );
        Ok(())
    }
}
