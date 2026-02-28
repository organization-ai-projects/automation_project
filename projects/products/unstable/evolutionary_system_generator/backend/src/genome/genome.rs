use crate::genome::genome_id::GenomeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RuleEntry {
    pub name: String,
    pub weight: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Genome {
    pub id: GenomeId,
    pub rules: Vec<RuleEntry>,
}
