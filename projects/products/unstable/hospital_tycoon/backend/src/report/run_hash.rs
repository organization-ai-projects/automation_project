// projects/products/unstable/hospital_tycoon/backend/src/report/run_hash.rs
use crate::report::run_hash_input::RunHashInput;
use sha2::{Digest, Sha256};

pub struct RunHash;

impl RunHash {
    pub fn compute(input: &RunHashInput<'_>) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.seed.to_le_bytes());
        hasher.update(input.scenario_name.as_bytes());
        hasher.update(input.total_ticks.to_le_bytes());
        hasher.update(input.patients_treated.to_le_bytes());
        hasher.update(input.patients_died.to_le_bytes());
        hasher.update(input.final_budget.to_le_bytes());
        hasher.update(input.final_reputation.to_le_bytes());
        hasher.update((input.event_count as u64).to_le_bytes());
        hex::encode(hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_hash_deterministic() {
        let input = RunHashInput {
            seed: 42,
            scenario_name: "test",
            total_ticks: 50,
            patients_treated: 10,
            patients_died: 0,
            final_budget: 15000,
            final_reputation: 60,
            event_count: 30,
        };
        let h1 = RunHash::compute(&input);
        let h2 = RunHash::compute(&input);
        assert_eq!(h1, h2);
        assert!(!h1.is_empty());
    }
}
