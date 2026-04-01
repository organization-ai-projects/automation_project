use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub name: String,
    pub rules: Vec<PolicyRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    pub kind: PolicyRuleKind,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyRuleKind {
    ForbidDependency { from: String, to: String },
    RequireDependency { from: String, to: String },
    MaxDependencies { crate_name: String, max: usize },
}
