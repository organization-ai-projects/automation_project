use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::time::turn::Turn;

use super::battle_report::BattleReport;

/// Log of all battles, keyed by turn for deterministic ordering.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WarLog {
    pub conflicts: BTreeMap<Turn, Vec<BattleReport>>,
}

impl WarLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, turn: Turn, report: BattleReport) {
        self.conflicts.entry(turn).or_default().push(report);
    }
}
