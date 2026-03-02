#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct KernelConfig {
    pub max_ticks: u64,
    pub max_turns: u64,
    pub default_ticks_per_turn: u64,
}

impl Default for KernelConfig {
    fn default() -> Self {
        Self {
            max_ticks: 100_000,
            max_turns: 10_000,
            default_ticks_per_turn: 10,
        }
    }
}
