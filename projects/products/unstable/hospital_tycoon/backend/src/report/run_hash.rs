// projects/products/unstable/hospital_tycoon/backend/src/report/run_hash.rs
use sha2::{Digest, Sha256};

pub struct RunHash;

impl RunHash {
    pub fn compute(
        seed: u64,
        scenario_name: &str,
        total_ticks: u64,
        patients_treated: u32,
        patients_died: u32,
        final_budget: i64,
        final_reputation: u32,
        event_count: usize,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(seed.to_le_bytes());
        hasher.update(scenario_name.as_bytes());
        hasher.update(total_ticks.to_le_bytes());
        hasher.update(patients_treated.to_le_bytes());
        hasher.update(patients_died.to_le_bytes());
        hasher.update(final_budget.to_le_bytes());
        hasher.update(final_reputation.to_le_bytes());
        hasher.update((event_count as u64).to_le_bytes());
        hex::encode(hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_hash_deterministic() {
        let h1 = RunHash::compute(42, "test", 50, 10, 0, 15000, 60, 30);
        let h2 = RunHash::compute(42, "test", 50, 10, 0, 15000, 60, 30);
        assert_eq!(h1, h2);
        assert!(!h1.is_empty());
    }
}
