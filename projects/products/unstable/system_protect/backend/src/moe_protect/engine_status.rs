use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineStatus {
    pub expert_count: usize,
    pub firewall_rule_count: usize,
    pub signature_count: usize,
    pub events_analyzed: u64,
    pub threats_blocked: u64,
}
