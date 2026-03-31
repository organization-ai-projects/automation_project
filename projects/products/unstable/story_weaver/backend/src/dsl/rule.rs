use serde::{Deserialize, Serialize};

use crate::dsl::condition::Condition;
use crate::dsl::effect::Effect;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub description: String,
    pub conditions: Vec<Condition>,
    pub effects: Vec<Effect>,
    pub weight: u64,
}
