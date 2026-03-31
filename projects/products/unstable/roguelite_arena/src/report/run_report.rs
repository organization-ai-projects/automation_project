use crate::report::RunHash;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RunReport {
    pub(crate) ticks_run: u64,
    pub(crate) scenario_name: String,
    pub(crate) seed: u64,
    pub(crate) player_survived: bool,
    pub(crate) player_final_hp: u32,
    pub(crate) enemies_killed: u32,
    pub(crate) waves_cleared: u32,
    pub(crate) total_damage_dealt: u64,
    pub(crate) total_damage_taken: u64,
    pub(crate) items_collected: u32,
    pub(crate) event_count: usize,
    pub(crate) snapshot_hashes: BTreeMap<String, String>,
    pub(crate) run_hash: RunHash,
}

impl RunReport {
    pub(crate) fn compute_hash(&mut self) {
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
            "{{\"ticks_run\":{},\"scenario_name\":{},\"seed\":{},\"player_survived\":{},\"player_final_hp\":{},\"enemies_killed\":{},\"waves_cleared\":{},\"total_damage_dealt\":{},\"total_damage_taken\":{},\"items_collected\":{},\"event_count\":{},\"snapshot_hashes\":{{{}}}}}",
            self.ticks_run,
            common_json::to_json_string(&self.scenario_name).unwrap_or_else(|_| "\"\"".to_string()),
            self.seed,
            self.player_survived,
            self.player_final_hp,
            self.enemies_killed,
            self.waves_cleared,
            self.total_damage_dealt,
            self.total_damage_taken,
            self.items_collected,
            self.event_count,
            snapshots_json
        );

        self.run_hash = RunHash::compute(canonical.as_bytes());
    }
}
