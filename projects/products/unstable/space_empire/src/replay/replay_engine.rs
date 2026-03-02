use crate::config::SimConfig;
use crate::diagnostics::SpaceEmpireError;
use crate::replay::ReplayFile;
use crate::report::SimReport;
use crate::scenario::{Scenario, ScenarioLoader};

pub struct ReplayEngine;

impl ReplayEngine {
    pub fn replay(
        replay_file: &ReplayFile,
        scenario: &Scenario,
        config: &SimConfig,
    ) -> Result<SimReport, SpaceEmpireError> {
        let scenario_hash = ScenarioLoader::scenario_hash(scenario);
        if scenario_hash != replay_file.scenario_hash {
            return Err(SpaceEmpireError::ReplayMismatch(format!(
                "Scenario hash mismatch: expected {}, got {}",
                replay_file.scenario_hash, scenario_hash
            )));
        }

        let replay_config = SimConfig {
            seed: replay_file.seed,
            ticks: replay_file.ticks_run,
            scenario_path: config.scenario_path.clone(),
        };

        let mut sim = crate::Sim::new(replay_config, scenario.clone());
        sim.run()
    }
}
