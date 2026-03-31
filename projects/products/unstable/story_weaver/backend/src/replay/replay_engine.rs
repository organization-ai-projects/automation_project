use crate::config::StoryConfig;
use crate::diagnostics::Error;
use crate::engine::NarrativeEngine;
use crate::replay::ReplayFile;
use crate::report::StoryReport;

pub struct ReplayEngine;

impl ReplayEngine {
    pub fn replay(replay: &ReplayFile) -> Result<StoryReport, Error> {
        let config = StoryConfig {
            seed: replay.seed,
            max_steps: replay.script.max_steps,
        };
        let (report, _) = NarrativeEngine::run(&replay.script, &config)?;
        Ok(report)
    }
}
