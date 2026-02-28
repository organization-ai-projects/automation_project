use crate::output::candidate::Candidate;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ManifestHash(pub String);

impl ManifestHash {
    pub fn compute(candidates: &[Candidate], generation: u32, seed: u64) -> Self {
        let mut hasher = Sha256::new();
        for c in candidates {
            hasher.update(c.genome_id.0.to_le_bytes());
            for rule in &c.genome.rules {
                hasher.update(rule.name.as_bytes());
                hasher.update(rule.weight.to_le_bytes());
            }
            let fitness_str = format!("{:.6}", c.fitness.0);
            hasher.update(fitness_str.as_bytes());
        }
        hasher.update(generation.to_le_bytes());
        hasher.update(seed.to_le_bytes());
        let result = hasher.finalize();
        ManifestHash(hex::encode(result))
    }
}
