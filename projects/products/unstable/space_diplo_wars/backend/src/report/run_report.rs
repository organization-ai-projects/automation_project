use serde::{Deserialize, Serialize};

use crate::diplomacy::treaty::Treaty;
use crate::war::battle_report::BattleReport;

use super::treaty_report::TreatyReport;
use super::turn_report::TurnReport;
use super::war_report::WarReport;

/// Canonical run report. JSON fields are serialized in struct definition order;
/// use JsonCodec (which sorts keys) for canonical bytes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunReport {
    pub game_id: String,
    pub seed: u64,
    pub turns_played: u64,
    pub turn_reports: Vec<TurnReport>,
    pub treaty_reports: Vec<TreatyReport>,
    pub war_report: WarReport,
    pub final_snapshot_hash: String,
}

impl RunReport {
    pub fn treaty_reports_from_treaties(
        treaties: &std::collections::BTreeMap<String, Treaty>,
    ) -> Vec<TreatyReport> {
        treaties
            .values()
            .map(|treaty| TreatyReport {
                treaty_id: treaty.id.0.clone(),
                kind: format!("{:?}", treaty.kind),
                parties: treaty
                    .parties
                    .iter()
                    .map(|empire| empire.0.clone())
                    .collect(),
                start_turn: treaty.start_turn,
                end_turn: treaty.end_turn,
            })
            .collect()
    }

    pub fn war_report_from_turn_reports(turn_reports: &[TurnReport]) -> WarReport {
        let battles: Vec<BattleReport> = turn_reports
            .iter()
            .flat_map(|turn_report| turn_report.battles.clone())
            .collect();
        WarReport {
            total_battles: battles.len(),
            battles,
        }
    }
}
