use crate::replay::replay_file::ReplayFile;
use crate::report::sim_report::SimReport;
use crate::scenario::scenario_loader::ScenarioLoader;
use crate::diagnostics::error::ColonyManagerError;

pub struct ReplayEngine;

impl ReplayEngine {
    pub fn replay(replay: &ReplayFile) -> Result<SimReport, ColonyManagerError> {
        use rand::SeedableRng;
        use rand::rngs::SmallRng;
        use crate::sim_engine::SimEngine;
        let scenario = ScenarioLoader::default_scenario(&replay.scenario_name);
        let mut rng = SmallRng::seed_from_u64(replay.seed.0);
        let report = SimEngine::run_with_rng(&scenario, replay.ticks, replay.seed.0, &mut rng)?;
        Ok(report)
    }
}
