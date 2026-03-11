use serde::{Deserialize, Serialize};

use super::expert_output::ExpertOutput;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedOutput {
    pub outputs: Vec<ExpertOutput>,
    pub selected_output: Option<ExpertOutput>,
    pub strategy: String,
}
