use serde::{Deserialize, Serialize};
use crate::genome::genome::Genome;
use crate::genome::genome_id::GenomeId;
use crate::seed::seed::Xorshift64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Mutation {
    TweakWeight { rule_idx: usize, delta: i32 },
    SwapWeights { idx_a: usize, idx_b: usize },
    ZeroRule { rule_idx: usize },
}

impl Mutation {
    pub fn apply(&self, genome: &mut Genome) {
        match self {
            Mutation::TweakWeight { rule_idx, delta } => {
                if *rule_idx < genome.rules.len() {
                    let w = genome.rules[*rule_idx].weight as i32 + delta;
                    genome.rules[*rule_idx].weight = w.max(0) as u32;
                }
            }
            Mutation::SwapWeights { idx_a, idx_b } => {
                if *idx_a < genome.rules.len() && *idx_b < genome.rules.len() {
                    let wa = genome.rules[*idx_a].weight;
                    genome.rules[*idx_a].weight = genome.rules[*idx_b].weight;
                    genome.rules[*idx_b].weight = wa;
                }
            }
            Mutation::ZeroRule { rule_idx } => {
                if *rule_idx < genome.rules.len() {
                    genome.rules[*rule_idx].weight = 0;
                }
            }
        }
    }

    pub fn random(rng: &mut Xorshift64, genome: &Genome) -> Self {
        let kind = rng.next_range(3);
        let n = genome.rules.len();
        match kind {
            0 => {
                let rule_idx = rng.next_range(n);
                let delta = (rng.next_range(11) as i32) - 5;
                Mutation::TweakWeight { rule_idx, delta }
            }
            1 => {
                let idx_a = rng.next_range(n);
                let idx_b = rng.next_range(n);
                Mutation::SwapWeights { idx_a, idx_b }
            }
            _ => {
                let rule_idx = rng.next_range(n);
                Mutation::ZeroRule { rule_idx }
            }
        }
    }
}
