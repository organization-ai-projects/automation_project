use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::dsl::rule::Rule;
use crate::state::StateValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    pub title: String,
    pub initial_state: BTreeMap<String, StateValue>,
    pub rules: Vec<Rule>,
    pub max_steps: u64,
}
