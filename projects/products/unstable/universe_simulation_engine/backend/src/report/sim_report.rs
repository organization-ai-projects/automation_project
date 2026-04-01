use crate::config::physics_config::PhysicsConfig;
use crate::report::run_hash::RunHash;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimReport {
    pub ticks_run: u64,
    pub seed: u64,
    pub final_era: String,
    pub final_cosmic_time_years: f64,
    pub total_particles: usize,
    pub total_stars: usize,
    pub total_galaxies: usize,
    pub filament_count: usize,
    pub void_count: usize,
    pub event_count: usize,
    pub snapshot_hashes: BTreeMap<String, String>,
    pub run_hash: RunHash,
    pub physics_config: PhysicsConfig,
}

impl SimReport {
    pub fn compute_hash(&mut self) {
        let snapshots_json = self
            .snapshot_hashes
            .iter()
            .map(|(k, v)| {
                format!(
                    "{}:{}",
                    common_json::to_json_string(k).unwrap_or_else(|_| "\"\"".to_string()),
                    common_json::to_json_string(v).unwrap_or_else(|_| "\"\"".to_string())
                )
            })
            .collect::<Vec<_>>()
            .join(",");

        let canonical = format!(
            "{{\"ticks_run\":{},\"seed\":{},\"final_era\":{},\"particles\":{},\"stars\":{},\"galaxies\":{},\"filaments\":{},\"voids\":{},\"events\":{},\"snapshots\":{{{}}}}}",
            self.ticks_run,
            self.seed,
            common_json::to_json_string(&self.final_era).unwrap_or_else(|_| "\"\"".to_string()),
            self.total_particles,
            self.total_stars,
            self.total_galaxies,
            self.filament_count,
            self.void_count,
            self.event_count,
            snapshots_json
        );

        self.run_hash = RunHash::compute(canonical.as_bytes());
    }
}
