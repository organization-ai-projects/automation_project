use crate::contract::module_spec::ModuleSpec;
use crate::contract::rule_spec::RuleSpec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub modules: Vec<ModuleSpec>,
    #[serde(default)]
    pub rules: Vec<RuleSpec>,
}
