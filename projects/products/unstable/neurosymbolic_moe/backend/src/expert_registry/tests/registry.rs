use crate::expert_registry::ExpertRegistry;
use crate::moe_core::{
    ExecutionContext, Expert, ExpertCapability, ExpertError, ExpertId, ExpertMetadata,
    ExpertOutput, ExpertStatus, ExpertType, Task, TaskType,
};
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

    fn can_handle(&self, task: &Task) -> bool {
        !task.input().is_empty()
    }

    fn execute(
        &self,
        task: &Task,
        context: &ExecutionContext,
    ) -> Result<ExpertOutput, ExpertError> {
        let context_size = context.retrieved_context.len() + context.memory_entries.len();
        Ok(ExpertOutput {
            expert_id: self.meta.id.clone(),
            content: format!("{}:{context_size}", task.input()),
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
    reg.register(Box::new(expert))
        .expect("register should succeed");
    assert!(reg.get(&ExpertId::new("e1")).is_some());
}

#[test]
fn register_duplicate_returns_error() {
    let mut reg = ExpertRegistry::new();
    reg.register(Box::new(TestExpert::new("e1", vec![])))
        .expect("first register should succeed");
    let result = reg.register(Box::new(TestExpert::new("e1", vec![])));
    assert!(result.is_err());
}

#[test]
fn deregister_removes_expert() {
    let mut reg = ExpertRegistry::new();
    reg.register(Box::new(TestExpert::new("e1", vec![])))
        .expect("register should succeed");
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
    .expect("register e1 should succeed");
    reg.register(Box::new(TestExpert::new(
        "e2",
        vec![ExpertCapability::Retrieval],
    )))
    .expect("register e2 should succeed");
    let found = reg.find_by_capability(&ExpertCapability::CodeGeneration);
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].id().as_str(), "e1");
}

#[test]
fn find_for_task() {
    let mut reg = ExpertRegistry::new();
    reg.register(Box::new(TestExpert::new("e1", vec![])))
        .expect("register should succeed");
    let task = Task::new("t1", TaskType::CodeGeneration, "gen code");
    let found = reg.find_for_task(&task);
    assert_eq!(found.len(), 1);
}

#[test]
fn list_active() {
    let mut reg = ExpertRegistry::new();
    reg.register(Box::new(TestExpert::new("e1", vec![])))
        .expect("register should succeed");
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
        .expect("register should succeed");
    assert_eq!(reg.count(), 1);
    assert!(reg.contains(&ExpertId::new("e1")));
}
