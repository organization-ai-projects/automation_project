use serde::{Deserialize, Serialize};
use crate::constraints::constraint::Constraint;

fn default_top_n() -> usize {
    5
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Request {
    NewSearch {
        seed: u64,
        population_size: usize,
        max_generations: u32,
        rule_pool: Vec<String>,
        #[serde(default)]
        constraints: Vec<Constraint>,
    },
    StepGen,
    RunToEnd,
    GetCandidates {
        #[serde(default = "default_top_n")]
        top_n: usize,
    },
    SaveReplay {
        path: String,
    },
    LoadReplay {
        path: String,
        rule_pool: Vec<String>,
        #[serde(default)]
        constraints: Vec<Constraint>,
    },
    ReplayToEnd,
}
