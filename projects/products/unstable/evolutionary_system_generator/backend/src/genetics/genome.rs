// projects/products/unstable/evolutionary_system_generator/backend/src/genetics/genome.rs
use crate::genetics::genome_id::GenomeId;
use crate::genetics::rule_entry::RuleEntry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Genome {
    pub id: GenomeId,
    pub rules: Vec<RuleEntry>,
}
