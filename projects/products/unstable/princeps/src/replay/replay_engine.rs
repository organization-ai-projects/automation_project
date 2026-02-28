use crate::diagnostics::error::PrincepsError;
use crate::replay::replay_file::ReplayFile;
use crate::report::end_report::EndReport;
use crate::sim::sim_engine::SimEngine;

pub struct ReplayEngine;

impl ReplayEngine {
    pub fn replay(&self, replay_file: &ReplayFile) -> Result<EndReport, PrincepsError> {
        let mut engine = SimEngine::with_defaults(replay_file.seed);
        engine.run(replay_file.days)
    }
}
