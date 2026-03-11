use std::collections::HashMap;

use crate::moe_core::{
    Expert, ExpertCapability, ExpertError, ExpertId, ExpertMetadata, ExpertStatus, MoeError, Task,
};

pub struct ExpertRegistry {
    experts: HashMap<ExpertId, Box<dyn Expert>>,
}

impl ExpertRegistry {
    pub fn new() -> Self {
        Self {
            experts: HashMap::new(),
        }
    }

    pub fn register(&mut self, expert: Box<dyn Expert>) -> Result<(), MoeError> {
        let id = expert.id().clone();
        if self.experts.contains_key(&id) {
            return Err(MoeError::ExpertError(ExpertError::InvalidInput(format!(
                "expert with id '{}' is already registered",
                id.as_str()
            ))));
        }
        self.experts.insert(id, expert);
        Ok(())
    }

    pub fn deregister(&mut self, id: &ExpertId) -> Option<Box<dyn Expert>> {
        self.experts.remove(id)
    }

    pub fn get(&self, id: &ExpertId) -> Option<&dyn Expert> {
        self.experts.get(id).map(|e| e.as_ref())
    }

    pub fn find_by_capability(&self, capability: &ExpertCapability) -> Vec<&dyn Expert> {
        self.experts
            .values()
            .filter(|expert| {
                expert
                    .metadata()
                    .capabilities
                    .iter()
                    .any(|c| c == capability)
            })
            .map(|e| e.as_ref())
            .collect()
    }

    pub fn find_for_task(&self, task: &Task) -> Vec<&dyn Expert> {
        self.experts
            .values()
            .filter(|expert| expert.can_handle(task))
            .map(|e| e.as_ref())
            .collect()
    }

    pub fn list_active(&self) -> Vec<&ExpertMetadata> {
        self.experts
            .values()
            .map(|e| e.metadata())
            .filter(|m| matches!(m.status, ExpertStatus::Active))
            .collect()
    }

    pub fn count(&self) -> usize {
        self.experts.len()
    }

    pub fn contains(&self, id: &ExpertId) -> bool {
        self.experts.contains_key(id)
    }
}

impl Default for ExpertRegistry {
    fn default() -> Self {
        Self::new()
    }
}
