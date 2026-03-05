use crate::rng::rng_draw::RngDraw;
use crate::rng::seed::Seed;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayFile {
    pub seed: Seed,
    pub ticks: u64,
    pub scenario_name: String,
    pub rng_draws: Vec<RngDraw>,
}
