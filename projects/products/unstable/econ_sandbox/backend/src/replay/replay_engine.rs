use crate::config::sim_config::SimConfig;
use crate::diagnostics::error::Error;
use crate::replay::replay_file::ReplayFile;
use crate::report::sim_report::SimReport;
use crate::sim::sim_engine::SimEngine;

pub struct ReplayEngine;

impl ReplayEngine {
    pub fn replay(replay: &ReplayFile) -> Result<SimReport, Error> {
        let config = SimConfig {
            seed: replay.seed,
            ticks: replay.ticks,
        };
        let (report, _) = SimEngine::run_sim(&config)?;
        Ok(report)
    }
}
