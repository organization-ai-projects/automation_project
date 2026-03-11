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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::moe_core::{ExecutionContext, ExpertOutput, ExpertType, TaskType};
    use std::collections::HashMap;

    struct TestExpert {
        meta: ExpertMetadata,
    }

    impl TestExpert {
        fn new(id: &str, capabilities: Vec<ExpertCapability>) -> Self {
            Self {
                meta: ExpertMetadata {
                    id: ExpertId::new(id),
                    name: id.to_string(),
                    version: "1.0.0".to_string(),
                    capabilities,
                    status: ExpertStatus::Active,
                    expert_type: ExpertType::Deterministic,
                },
            }
        }
    }

    impl Expert for TestExpert {
        fn id(&self) -> &ExpertId {
            &self.meta.id
        }

        fn metadata(&self) -> &ExpertMetadata {
            &self.meta
        }

        fn can_handle(&self, _task: &Task) -> bool {
            true
        }

        fn execute(
            &self,
            _task: &Task,
            _context: &ExecutionContext,
        ) -> Result<ExpertOutput, ExpertError> {
            Ok(ExpertOutput {
                expert_id: self.meta.id.clone(),
                content: "test output".to_string(),
                confidence: 0.95,
                metadata: HashMap::new(),
                trace: Vec::new(),
            })
        }
    }

    #[test]
    fn register_and_get() {
        let mut reg = ExpertRegistry::new();
        let expert = TestExpert::new("e1", vec![ExpertCapability::CodeGeneration]);
        reg.register(Box::new(expert)).unwrap();
        assert!(reg.get(&ExpertId::new("e1")).is_some());
    }

    #[test]
    fn register_duplicate_returns_error() {
        let mut reg = ExpertRegistry::new();
        reg.register(Box::new(TestExpert::new("e1", vec![])))
            .unwrap();
        let result = reg.register(Box::new(TestExpert::new("e1", vec![])));
        assert!(result.is_err());
    }

    #[test]
    fn deregister_removes_expert() {
        let mut reg = ExpertRegistry::new();
        reg.register(Box::new(TestExpert::new("e1", vec![])))
            .unwrap();
        assert!(reg.deregister(&ExpertId::new("e1")).is_some());
        assert!(!reg.contains(&ExpertId::new("e1")));
    }

    #[test]
    fn find_by_capability() {
        let mut reg = ExpertRegistry::new();
        reg.register(Box::new(TestExpert::new(
            "e1",
            vec![ExpertCapability::CodeGeneration],
        )))
        .unwrap();
        reg.register(Box::new(TestExpert::new(
            "e2",
            vec![ExpertCapability::Retrieval],
        )))
        .unwrap();
        let found = reg.find_by_capability(&ExpertCapability::CodeGeneration);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].id().as_str(), "e1");
    }

    #[test]
    fn find_for_task() {
        let mut reg = ExpertRegistry::new();
        reg.register(Box::new(TestExpert::new("e1", vec![])))
            .unwrap();
        let task = Task::new("t1", TaskType::CodeGeneration, "gen code");
        let found = reg.find_for_task(&task);
        assert_eq!(found.len(), 1);
    }

    #[test]
    fn list_active() {
        let mut reg = ExpertRegistry::new();
        reg.register(Box::new(TestExpert::new("e1", vec![])))
            .unwrap();
        let active = reg.list_active();
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].id.as_str(), "e1");
    }

    #[test]
    fn count_and_contains() {
        let mut reg = ExpertRegistry::new();
        assert_eq!(reg.count(), 0);
        assert!(!reg.contains(&ExpertId::new("e1")));
        reg.register(Box::new(TestExpert::new("e1", vec![])))
            .unwrap();
        assert_eq!(reg.count(), 1);
        assert!(reg.contains(&ExpertId::new("e1")));
    }
}
