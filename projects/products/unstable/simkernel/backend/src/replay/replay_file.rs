#![allow(dead_code)]
use crate::commands::command::Command;
use crate::determinism::rng_draw::RngDraw;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayFile {
    pub pack_id: String,
    pub pack_kind: String,
    pub scenario_hash: String,
    pub seed: u64,
    pub commands: Vec<Command>,
    pub rng_draws: Vec<RngDraw>,
    pub event_log_checksum: u64,
}
