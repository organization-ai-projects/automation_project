//! projects/products/unstable/neurosymbolic_moe/backend/src/router/router.rs
use crate::expert_registry::ExpertRegistry;
use crate::moe_core::{MoeError, Task};

use super::routing_decision::RoutingDecision;

pub trait Router: Send + Sync {
    fn route(&self, task: &Task, registry: &ExpertRegistry) -> Result<RoutingDecision, MoeError>;
}
