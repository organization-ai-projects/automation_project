// projects/products/unstable/code_forge_engine/backend/src/contract/rule_spec.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleSpec {
    pub id: String,
    pub description: String,
    pub enforced: bool,
}
