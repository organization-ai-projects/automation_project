use crate::tooling::rule::Rule;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ruleset {
    pub name: String,
    pub rules: Vec<Rule>,
}
