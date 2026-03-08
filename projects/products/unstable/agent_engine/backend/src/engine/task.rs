// projects/products/unstable/agent_engine/backend/src/engine/task.rs
use std::collections::BTreeMap;

use crate::engine::step_spec;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct Task {
    pub id: String,
    pub label: String,
    pub metadata: BTreeMap<String, String>,
    pub input: BTreeMap<String, String>,
    pub steps: Vec<step_spec::StepSpec>,
}
