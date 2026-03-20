use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoutingStrategy {
    SingleExpert,
    MultiExpert,
    Fallback,
    RoundRobin,
}
