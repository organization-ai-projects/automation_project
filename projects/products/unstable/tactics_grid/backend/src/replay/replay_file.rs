use crate::rng::rng_draw::RngDraw;
use crate::rng::seed::Seed;
use crate::scenario::scenario::Scenario;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReplayFile {
    pub seed: Seed,
    pub scenario: Scenario,
    pub rng_draws: Vec<RngDraw>,
}
