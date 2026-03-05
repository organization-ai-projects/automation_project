use crate::diagnostics::colony_manager_error::ColonyManagerError;
use crate::replay::replay_file::ReplayFile;
use crate::report::sim_report::SimReport;
use crate::scenarios::scenario_loader::ScenarioLoader;

pub struct ReplayEngine;

impl ReplayEngine {
    pub fn replay(replay: &ReplayFile) -> Result<SimReport, ColonyManagerError> {
        let scenario = ScenarioLoader::default_scenario(&replay.scenario_name);
        let (report, generated_draws) =
            crate::sim_engine::SimEngine::run(&scenario, replay.ticks, replay.seed.0)?;
        if generated_draws != replay.rng_draws {
            return Err(ColonyManagerError::ReplayMismatch(
                "rng draw sequence mismatch".to_string(),
            ));
        }
        Ok(report)
    }
}
