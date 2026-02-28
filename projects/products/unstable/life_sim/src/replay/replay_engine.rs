use crate::diagnostics::LifeSimError;
use crate::replay::replay_file::ReplayFile;
use crate::report::RunReport;
use crate::sim::event_log::EventLog;
use crate::sim::sim_engine::SimEngine;
use crate::sim::sim_state::SimState;
use crate::time::{Tick, TickClock};

pub struct ReplayEngine;

impl ReplayEngine {
    /// Reconstruct SimState from a ReplayFile and produce an identical RunReport.
    pub fn replay(replay: &ReplayFile) -> Result<RunReport, LifeSimError> {
        let mut state = SimState {
            world: replay.initial_world.clone(),
            clock: TickClock::new(Tick(0)),
            event_log: EventLog::default(),
            seed: replay.seed,
        };

        let engine = SimEngine::new(replay.config.clone());
        engine.run(&mut state)
    }
}
