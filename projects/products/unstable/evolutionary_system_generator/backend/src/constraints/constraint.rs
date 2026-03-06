// projects/products/unstable/evolutionary_system_generator/backend/src/constraints/constraint.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Constraint {
    MinActiveRules(usize),
    MaxTotalWeight(u32),
    RequiredRule(String),
}
