use crate::rng::RngDraw;
use crate::rng::Seed;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ReplayFile {
    pub(crate) seed: Seed,
    pub(crate) ticks: u64,
    pub(crate) scenario_name: String,
    pub(crate) rng_draws: Vec<RngDraw>,
}
