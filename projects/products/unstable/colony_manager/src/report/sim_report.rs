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
        #[derive(Serialize)]
        struct Payload<'a> {
            ticks_run: u64,
            scenario_name: &'a str,
            seed: u64,
            colonist_reports: &'a Vec<ColonistReport>,
            event_count: usize,
            snapshot_hashes: &'a BTreeMap<String, String>,
        }
        let payload = Payload {
            ticks_run: self.ticks_run,
            scenario_name: &self.scenario_name,
            seed: self.seed,
            colonist_reports: &self.colonist_reports,
            event_count: self.event_count,
            snapshot_hashes: &self.snapshot_hashes,
        };
        let bytes = serde_json::to_vec(&payload).expect("sim report serializable");
        self.run_hash = RunHash::compute(&bytes);
    }
}
