use crate::combat::battle_report::BattleReport;
use crate::events::event_log::EventLog;
use crate::model::party::Party;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunReport {
    pub seed: u64,
    pub step_count: u64,
    pub party: Party,
    pub event_count: usize,
    pub battle_reports: Vec<BattleReport>,
}

impl RunReport {
    pub fn build(
        seed: u64,
        step_count: u64,
        party: &Party,
        event_log: &EventLog,
        battle_reports: &[BattleReport],
    ) -> Self {
        Self {
            seed,
            step_count,
            party: party.clone(),
            event_count: event_log.events.len(),
            battle_reports: battle_reports.to_vec(),
        }
    }
}
