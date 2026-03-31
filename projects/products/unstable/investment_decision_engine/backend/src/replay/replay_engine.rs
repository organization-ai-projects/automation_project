use crate::config::{EngineConfig, FeatureGateConfig};
use crate::decision::{DecisionEngine, DecisionSummary};
use crate::replay::ReplayFile;

pub struct ReplayEngine;

impl ReplayEngine {
    pub fn execute(
        replay: &ReplayFile,
        _config: &EngineConfig,
        gate: &FeatureGateConfig,
    ) -> DecisionSummary {
        DecisionEngine::synthesize(
            &replay.risk_score,
            &replay.pnl,
            &replay.sentiment,
            gate,
        )
    }
}
