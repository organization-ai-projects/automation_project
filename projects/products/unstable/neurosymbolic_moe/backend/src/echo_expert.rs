use std::collections::HashMap;

use protocol::ProtocolId;

use crate::moe_core::{
    ExecutionContext, Expert, ExpertCapability, ExpertError, ExpertId, ExpertMetadata,
    ExpertOutput, ExpertStatus, ExpertType, Task,
};
use crate::orchestrator::Version;

pub(crate) struct EchoExpert {
    metadata: ExpertMetadata,
}

impl EchoExpert {
    pub(crate) fn new_with_id(
        id: ProtocolId,
        name: &str,
        capabilities: Vec<ExpertCapability>,
    ) -> Self {
        Self {
            metadata: ExpertMetadata {
                id: ExpertId::from_protocol_id(id),
                name: name.to_string(),
                version: Version::new(1, 0, 0),
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
