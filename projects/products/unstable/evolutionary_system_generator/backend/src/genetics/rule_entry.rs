// projects/products/unstable/evolutionary_system_generator/backend/src/genetics/rule_entry.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RuleEntry {
    pub name: String,
    pub weight: u32,
}
