use super::run_hash::RunHash;
use crate::turn::action_entry::ActionEntry;
use crate::unit::team::Team;
use crate::unit::unit_id::UnitId;
use std::collections::BTreeMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnitSummary {
    pub id: UnitId,
    pub name: String,
    pub team: Team,
    pub alive: bool,
    pub hp: i32,
    pub max_hp: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BattleReport {
    pub scenario_name: String,
    pub seed: u64,
    pub turns_played: u32,
    pub winner: Option<String>,
    pub actions: Vec<ActionEntry>,
    pub snapshot_hashes: BTreeMap<String, String>,
    pub unit_summaries: Vec<UnitSummary>,
    pub run_hash: RunHash,
}

impl BattleReport {
    pub fn compute_hash(&mut self) {
        let canonical = format!(
            "scenario_name:{},seed:{},turns_played:{},winner:{},actions_count:{},snapshot_count:{}",
            self.scenario_name,
            self.seed,
            self.turns_played,
            self.winner.as_deref().unwrap_or("none"),
            self.actions.len(),
            self.snapshot_hashes.len(),
        );
        let mut full_canonical = canonical;
        for (key, value) in &self.snapshot_hashes {
            full_canonical.push_str(&format!(",{key}:{value}"));
        }
        self.run_hash = RunHash::compute(full_canonical.as_bytes());
    }
}
