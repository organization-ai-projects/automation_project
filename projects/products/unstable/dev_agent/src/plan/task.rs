use runtime_core::RuntimeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskKind {
    Scan,
    Plan,
    Patch,
    Verify,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub id: RuntimeId,
    pub label: String,
    pub kind: TaskKind,
}

impl Task {
    pub fn new(id: RuntimeId, label: impl Into<String>, kind: TaskKind) -> Self {
        Self {
            id,
            label: label.into(),
            kind,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stores_fields() {
        let id = RuntimeId::new(1);
        let task = Task::new(id, "scan", TaskKind::Scan);
        assert_eq!(task.id, RuntimeId::new(1));
        assert_eq!(task.label, "scan");
        assert_eq!(task.kind, TaskKind::Scan);
    }

    #[test]
    fn serializes_to_json() {
        let task = Task::new(RuntimeId::new(2), "plan", TaskKind::Plan);
        let json = serde_json::to_string(&task).unwrap();
        let restored: Task = serde_json::from_str(&json).unwrap();
        assert_eq!(task, restored);
    }
}
