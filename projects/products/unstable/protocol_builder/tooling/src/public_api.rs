// projects/products/unstable/protocol_builder/tooling/src/public_api.rs
use anyhow::Result;

use crate::validate::{EmittedManifest, EmittedValidator, GoldenTranscript, GoldenTranscriptValidator};

pub fn validate_emitted(manifest_path: &str) -> Result<()> {
    let content = std::fs::read_to_string(manifest_path)?;
    let manifest: EmittedManifest = common_json::from_json_str(&content)?;
    EmittedValidator::validate(&manifest)?;
    println!("emitted validation OK: {}", manifest.manifest_hash);
    Ok(())
}

pub fn validate_transcript(transcript_path: &str) -> Result<()> {
    let content = std::fs::read_to_string(transcript_path)?;
    let transcript: GoldenTranscript = common_json::from_json_str(&content)?;
    GoldenTranscriptValidator::validate(&transcript)?;
    println!("transcript validation OK: {}", transcript.final_manifest_hash);
    Ok(())
}
