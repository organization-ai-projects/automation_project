#![allow(dead_code)]
use crate::config::sim_config::SimConfig;
use crate::diagnostics::error::SimError;
use crate::events::event_log::EventLog;
use crate::replay::replay_file::ReplayFile;
use crate::report::run_report::RunReport;
use crate::scenario::scenario::Scenario;
use crate::scenario::scenario_loader::ScenarioLoader;
use crate::sim::sim_engine::SimEngine;
use crate::sim::sim_state::SimState;

/// Replays a saved replay file and validates the result matches.
pub struct ReplayEngine;

impl ReplayEngine {
    /// Run the replay and return (SimState, EventLog, RunReport) on success.
    pub fn replay(
        replay: &ReplayFile,
        config: &SimConfig,
    ) -> Result<(SimState, EventLog, RunReport), SimError> {
        let scenario: Scenario =
            ScenarioLoader::load_from_str(&replay.scenario_json)?;
        let (engine, mut state) = SimEngine::new(&scenario, replay.seed, config);
        let mut event_log = EventLog::new();

        while state.clock.tick.value() < replay.ticks {
            engine.tick(&mut state, &mut event_log);
        }

        let checksum = event_log.checksum();
        if checksum != replay.event_log_checksum {
            return Err(SimError::ReplayMismatch(format!(
                "event_log checksum mismatch: expected {} got {}",
                replay.event_log_checksum, checksum
            )));
        }

        let report = RunReport::build(&state, &event_log, replay.ticks);
        Ok((state, event_log, report))
    }
}
