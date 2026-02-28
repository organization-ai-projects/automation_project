use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Constraint {
    MinActiveRules(usize),
    MaxTotalWeight(u32),
    RequiredRule(String),
}
