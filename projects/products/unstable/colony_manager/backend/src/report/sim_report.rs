use crate::report::colonist_report::ColonistReport;
use crate::report::run_hash::RunHash;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimReport {
    pub ticks_run: u64,
    pub scenario_name: String,
    pub seed: u64,
    pub colonist_reports: Vec<ColonistReport>,
    pub event_count: usize,
    pub snapshot_hashes: BTreeMap<String, String>,
    pub run_hash: RunHash,
}

impl SimReport {
    pub fn compute_hash(&mut self) {
        let mut colonists = self.colonist_reports.clone();
        colonists.sort_by_key(|c| c.id);

        let colonists_json = colonists
            .iter()
            .map(|c| {
                format!(
                    "{{\"id\":{},\"name\":{},\"final_mood\":\"{:.6}\",\"jobs_completed\":{}}}",
                    c.id.0,
                    common_json::to_json_string(&c.name).unwrap_or_else(|_| "\"\"".to_string()),
                    c.final_mood,
                    c.jobs_completed
                )
            })
            .collect::<Vec<_>>()
            .join(",");

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
            "{{\"ticks_run\":{},\"scenario_name\":{},\"seed\":{},\"colonist_reports\":[{}],\"event_count\":{},\"snapshot_hashes\":{{{}}}}}",
            self.ticks_run,
            common_json::to_json_string(&self.scenario_name).unwrap_or_else(|_| "\"\"".to_string()),
            self.seed,
            colonists_json,
            self.event_count,
            snapshots_json
        );

        self.run_hash = RunHash::compute(canonical.as_bytes());
    }
}
