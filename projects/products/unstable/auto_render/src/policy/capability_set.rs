use crate::plan::Capability;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitySet {
    pub capabilities: HashSet<Capability>,
}

impl CapabilitySet {
    pub fn new(capabilities: HashSet<Capability>) -> Self {
        Self { capabilities }
    }

    pub fn contains(&self, cap: &Capability) -> bool {
        self.capabilities.contains(cap)
    }
}
