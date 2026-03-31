use super::replay_file::ReplayFile;
use crate::diagnostics::tactics_grid_error::TacticsGridError;
use crate::report::battle_report::BattleReport;
use crate::turn::turn_engine::TurnEngine;

pub struct ReplayEngine;

impl ReplayEngine {
    pub fn replay(replay: &ReplayFile) -> Result<BattleReport, TacticsGridError> {
        let (report, generated_draws) = TurnEngine::run_battle(&replay.scenario, replay.seed)?;

        if generated_draws != replay.rng_draws {
            return Err(TacticsGridError::ReplayMismatch(
                "rng draw sequence mismatch".to_string(),
            ));
        }

        Ok(report)
    }
}
