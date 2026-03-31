use crate::combat::CombatEngine;
use crate::diagnostics::RogueliteArenaError;
use crate::replay::ReplayFile;
use crate::report::RunReport;
use crate::scenarios::ScenarioLoader;

pub(crate) struct ReplayEngine;

impl ReplayEngine {
    pub(crate) fn replay(replay: &ReplayFile) -> Result<RunReport, RogueliteArenaError> {
        let scenario = ScenarioLoader::default_scenario(&replay.scenario_name);
        let (report, generated_draws) = CombatEngine::run(&scenario, replay.ticks, replay.seed.0)?;
        if generated_draws != replay.rng_draws {
            return Err(RogueliteArenaError::ReplayMismatch(
                "rng draw sequence mismatch".to_string(),
            ));
        }
        Ok(report)
    }
}
