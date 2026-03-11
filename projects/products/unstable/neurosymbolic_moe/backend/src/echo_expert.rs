use std::collections::HashMap;

use crate::moe_core::{
    ExecutionContext, Expert, ExpertCapability, ExpertError, ExpertId, ExpertMetadata,
    ExpertOutput, ExpertStatus, ExpertType, Task,
};

pub struct EchoExpert {
    metadata: ExpertMetadata,
}

impl EchoExpert {
    pub fn new(id: &str, name: &str, capabilities: Vec<ExpertCapability>) -> Self {
        Self {
            metadata: ExpertMetadata {
                id: ExpertId::new(id),
                name: name.to_string(),
                version: "0.1.0".to_string(),
                capabilities,
                status: ExpertStatus::Active,
                expert_type: ExpertType::Deterministic,
            },
        }
    }
}

impl Expert for EchoExpert {
    fn id(&self) -> &ExpertId {
        &self.metadata.id
    }

    fn metadata(&self) -> &ExpertMetadata {
        &self.metadata
    }

    fn can_handle(&self, task: &Task) -> bool {
        !task.input().is_empty()
    }

    fn execute(
        &self,
        task: &Task,
        context: &ExecutionContext,
    ) -> Result<ExpertOutput, ExpertError> {
        Ok(ExpertOutput {
            expert_id: self.metadata.id.clone(),
            content: format!(
                "[{}] processed: {} (ctx:{} mem:{} buf:{})",
                self.metadata.name,
                task.input(),
                context.retrieved_context.len(),
                context.memory_entries.len(),
                context.buffer_data.len()
            ),
            confidence: 0.9,
            metadata: HashMap::new(),
            trace: vec![format!("Expert {} executed", self.metadata.name)],
        })
    }
}
