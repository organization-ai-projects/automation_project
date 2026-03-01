#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct RunConfig {
    pub pack_kind: String,
    pub seed: u64,
    pub ticks: u64,
    pub turns: u64,
    pub ticks_per_turn: u64,
}
