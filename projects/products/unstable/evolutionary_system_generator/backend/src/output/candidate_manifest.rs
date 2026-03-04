// projects/products/unstable/evolutionary_system_generator/backend/src/output/candidate_manifest.rs
use crate::output::candidate::Candidate;
use crate::output::manifest_hash::ManifestHash;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandidateManifest {
    pub candidates: Vec<Candidate>,
    pub manifest_hash: ManifestHash,
    pub generation: u32,
    pub seed: u64,
}
