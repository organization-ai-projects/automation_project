// projects/products/unstable/digital_pet/backend/src/report/run_hash.rs
use sha2::{Digest, Sha256};

pub struct RunHash;

impl RunHash {
    pub fn compute(seed: u64, species: &str, evolution_stage: u32, mistakes: usize) -> String {
        let mut hasher = Sha256::new();
        hasher.update(seed.to_le_bytes());
        hasher.update(species.as_bytes());
        hasher.update(evolution_stage.to_le_bytes());
        hasher.update((mistakes as u64).to_le_bytes());
        hex::encode(hasher.finalize())
    }
}
